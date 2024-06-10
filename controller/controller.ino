#define BILL_PIN 2      // 지폐기
#define COIN_PIN 3      // 코인기
#define INPUT_PIN_1 4   // 열편
#define OUTPUT_PIN_1 5  // 열채
#define INPUT_PIN_2 6   // 궁편
#define OUTPUT_PIN_2 7  // 궁채
#define RELAY_DELAY 50  // 딜레이 50μs

int step;
int pin1ConnectedTo, pin2ConnectedTo;
unsigned int lastTimestamp;
unsigned int coin_cnt;

void setup()
{
  step = 0;
  coin_cnt = 0;
  pinMode(OUTPUT_PIN_1, OUTPUT);
  pinMode(OUTPUT_PIN_2, OUTPUT);
  pinMode(INPUT_PIN_1, INPUT_PULLUP);
  pinMode(INPUT_PIN_2, INPUT_PULLUP);
  
  // 열채, 궁채
  digitalWrite(OUTPUT_PIN_1, LOW);
  digitalWrite(OUTPUT_PIN_2, LOW);
  
  // 지폐, 코인기 인터럽트
  attachInterrupt(digitalPinToInterrupt(BILL_PIN), bill, FALLING);
  attachInterrupt(digitalPinToInterrupt(COIN_PIN), coin, FALLING);

  // 시리얼 셋
  Serial.begin(9600);
  while (!Serial) {
    ; // wait for serial port to connect. Needed for native USB
  }
}

void bill()
{
  coin_cnt++;
}

void coin()
{
  coin_cnt++;
}

void loop()
{
  unsigned int timestamp = micros();
  if (timestamp - lastTimestamp > 100) {
   	step = (step + 1) % 3;
    lastTimestamp = timestamp;
  }
  
  switch(step) {
   	case 0:
    	digitalWrite(OUTPUT_PIN_1, LOW);
    	digitalWrite(OUTPUT_PIN_2, HIGH);
      delayMicroseconds(RELAY_DELAY);
        if(digitalRead(INPUT_PIN_1) == HIGH) {
			pin2ConnectedTo = INPUT_PIN_1;
        } else if (digitalRead(INPUT_PIN_2) == HIGH) {
			pin2ConnectedTo = INPUT_PIN_2;
        } else {
			pin2ConnectedTo = 0;
        }
    break;
    case 1:
    	digitalWrite(OUTPUT_PIN_2, LOW);
    	digitalWrite(OUTPUT_PIN_1, HIGH);
      delayMicroseconds(RELAY_DELAY);
        if(digitalRead(INPUT_PIN_1) == HIGH) {
			pin1ConnectedTo = INPUT_PIN_1;
        } else if (digitalRead(INPUT_PIN_2) == HIGH) {
			pin1ConnectedTo = INPUT_PIN_2;
        } else {
			pin1ConnectedTo = 0;
        }
    break;
    case 2:
    	digitalWrite(OUTPUT_PIN_1, LOW);
    	digitalWrite(OUTPUT_PIN_2, LOW);
      delayMicroseconds(RELAY_DELAY);
      uint8_t bits = 0;
      if (pin1ConnectedTo == INPUT_PIN_1)
        bits |= (uint8_t)1;
      if (pin1ConnectedTo == INPUT_PIN_2)
        bits |= (uint8_t)2;
      if (pin2ConnectedTo == INPUT_PIN_1)
        bits |= (uint8_t)4;
      if (pin2ConnectedTo == INPUT_PIN_2)
        bits |= (uint8_t)8;
      if (coin_cnt>0){
        bits |= (uint8_t)16;
        coin_cnt--;
      }
      Serial.write(bits);
    break;
  }
}
