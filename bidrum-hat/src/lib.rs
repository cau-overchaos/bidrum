use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::{f32, thread};

use btleplug::api::{Central, CentralEvent, Manager as _, Peripheral, ScanFilter};
use btleplug::platform::Manager;
use futures::executor;
use futures::stream::StreamExt;
use uuid::Uuid;

pub struct BidrumHat {
    spinning: Arc<AtomicBool>,
}

async fn get_data(spinning: Arc<AtomicBool>) {
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

    for adapter in adapter_list.iter() {
        // println!("Starting scan...");
        let mut events = adapter
            .events()
            .await
            .expect("Failed to get adapter event stream");

        adapter
            .start_scan(ScanFilter::default())
            .await
            .expect("Can't scan BLE adapter for connected devices...");

        while let Some(event) = events.next().await {
            match event {
                CentralEvent::DeviceDiscovered(id) => {
                    let peripheral = adapter.peripheral(&id).await.unwrap();
                    let local_name = peripheral.properties().await.unwrap().unwrap().local_name;

                    if local_name.is_some_and(|x| x.contains("bidrum-hat")) {
                        if !peripheral.is_connected().await.unwrap_or(false) {
                            peripheral.connect().await.expect("Failed to connect");
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
                            while let Some(data) = notification_stream.next().await {
                                let norm = std::str::from_utf8(data.value.as_slice())
                                    .unwrap_or("0.0")
                                    .parse::<f32>()
                                    .unwrap();

                                spinning.store(norm > 1.0, Ordering::Relaxed);
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    }
}

impl BidrumHat {
    pub fn new() -> BidrumHat {
        let spinning = Arc::new(AtomicBool::new(false));
        let spinning_for_thread = spinning.clone();

        thread::spawn(move || {
            let handle = tokio::runtime::Runtime::new().unwrap();
            let _guard = handle.enter();
            executor::block_on(get_data(spinning_for_thread));
        });

        let result = BidrumHat { spinning: spinning };

        return result;
    }
    pub fn spinning(&self) -> bool {
        self.spinning.load(Ordering::Relaxed)
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
