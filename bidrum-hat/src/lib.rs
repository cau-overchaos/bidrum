use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;
use std::{f32, thread};

use btleplug::api::{Central, CentralEvent, Manager as _, Peripheral, ScanFilter};
use btleplug::platform::Manager;
use futures::stream::StreamExt;
use futures::{executor, FutureExt};
use uuid::Uuid;

pub struct BidrumHat {
    spinning: Arc<AtomicBool>,
    dropping: Arc<AtomicBool>,
}

async fn get_data(spinning: Arc<AtomicBool>, dropping: Arc<AtomicBool>) {
    let service_uuid = Uuid::parse_str("8e191920-dda8-4f40-b2bc-f8f99f680c94").unwrap();
    let characteristic_uuid = Uuid::parse_str("2a89fe67-88ff-4bac-8e42-d122d6995ad1").unwrap();

    let manager = Manager::new().await.expect("Failed to get manager");
    let adapter_list = manager
        .adapters()
        .await
        .expect("Failed to get adapter list");
    if adapter_list.is_empty() {
        eprintln!("No Bluetooth adapters found");
    }

    'adapter_loop: for adapter in adapter_list.iter() {
        // println!("Starting scan...");
        let mut events = adapter
            .events()
            .await
            .expect("Failed to get adapter event stream");

        adapter
            .start_scan(ScanFilter::default())
            .await
            .expect("Can't scan BLE adapter for connected devices...");

        let mut event_join_handle = events.next();
        loop {
            let event_option = loop {
                if dropping.load(Ordering::Relaxed) {
                    break 'adapter_loop;
                }
                if let Some(v) = (&mut event_join_handle).now_or_never() {
                    break v;
                }
            };

            if let Some(event) = event_option {
                match event {
                    CentralEvent::DeviceDiscovered(id) | CentralEvent::DeviceUpdated(id) => {
                        let peripheral = {
                            let peripheral = adapter.peripheral(&id).await;
                            if let Ok(peripheral) = peripheral {
                                peripheral
                            } else {
                                break;
                            }
                        };

                        let local_name = peripheral.properties().await.unwrap().unwrap().local_name;

                        if local_name.is_some_and(|x| x.contains("bidrum-hat")) {
                            if !peripheral.is_connected().await.unwrap_or(false) {
                                let connect_result = peripheral.connect().await;
                                if connect_result.is_err() {
                                    break;
                                }
                            }

                            peripheral
                                .discover_services()
                                .await
                                .expect("Failed to discover services");

                            if let Some(characteristic) =
                                peripheral.characteristics().iter().find(|x| {
                                    x.uuid == characteristic_uuid && x.service_uuid == service_uuid
                                })
                            {
                                peripheral
                                    .subscribe(characteristic)
                                    .await
                                    .expect("Failed to subscribe characteristic");

                                let mut notification_stream = peripheral
                                    .notifications()
                                    .await
                                    .expect("Failed to make notification stream");

                                // Process while the BLE connection is not broken or stopped.
                                loop {
                                    let notification = notification_stream.next();
                                    let timeouted_notification = tokio::time::timeout(
                                        Duration::from_millis(100),
                                        notification,
                                    )
                                    .await;

                                    if !peripheral.is_connected().await.unwrap_or(false) {
                                        break;
                                    }

                                    if let Ok(unwrapped) = timeouted_notification {
                                        if let Some(data) = &unwrapped {
                                            let norm = std::str::from_utf8(data.value.as_slice())
                                                .unwrap_or("0.0")
                                                .parse::<f32>()
                                                .unwrap();

                                            spinning.store(norm > 1.0, Ordering::Relaxed);
                                            println!("spin norm: {:#}", norm);
                                        }
                                    }
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}

impl BidrumHat {
    pub fn new() -> BidrumHat {
        let spinning = Arc::new(AtomicBool::new(false));
        let spinning_for_thread = spinning.clone();
        let dropping = Arc::new(AtomicBool::new(false));
        let dropping_for_thread = dropping.clone();

        thread::spawn(move || {
            let handle = tokio::runtime::Runtime::new().unwrap();
            let _guard = handle.enter();
            executor::block_on(get_data(spinning_for_thread, dropping_for_thread));
        });

        let result = BidrumHat {
            spinning: spinning,
            dropping: dropping,
        };

        return result;
    }
    pub fn spinning(&self) -> bool {
        self.spinning.load(Ordering::Relaxed)
    }
}

impl Drop for BidrumHat {
    fn drop(&mut self) {
        self.dropping.store(true, Ordering::Relaxed);
    }
}

// "-- --nocapture" is needed when running test
#[test]
fn test() {
    let hat = BidrumHat::new();

    println!("Running test");
    loop {
        if hat.spinning() {
            println!("spinning");
        } else {
            println!("not spinning");
        }
    }
}
