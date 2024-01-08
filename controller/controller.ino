#define OUTPUT_PIN_1 4
#define OUTPUT_PIN_2 5
#define INPUT_PIN_1 8
#define INPUT_PIN_2 9
#define RELAY_DELAY 5
int step;
int pin1ConnectedTo, pin2ConnectedTo;
unsigned int lastTimestamp;
void setup()
{
  step = 0;
  pinMode(OUTPUT_PIN_1, OUTPUT);
  pinMode(OUTPUT_PIN_2, OUTPUT);
  pinMode(INPUT_PIN_1, INPUT_PULLUP);
  pinMode(INPUT_PIN_2, INPUT_PULLUP);
  
  digitalWrite(OUTPUT_PIN_1, LOW);
  digitalWrite(OUTPUT_PIN_2, LOW);
  
  Serial.begin(9600);
  while (!Serial) {
    ; // wait for serial port to connect. Needed for native USB
  }
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
    	delay(RELAY_DELAY);
        if(digitalRead(INPUT_PIN_1) == LOW) {
			pin2ConnectedTo = INPUT_PIN_1;
        } else if (digitalRead(INPUT_PIN_2) == LOW) {
			pin2ConnectedTo = INPUT_PIN_2;
        } else {
			pin2ConnectedTo = 0;
        }
    break;
    case 1:
    	digitalWrite(OUTPUT_PIN_2, LOW);
    	digitalWrite(OUTPUT_PIN_1, HIGH);
    	delay(RELAY_DELAY);
        if(digitalRead(INPUT_PIN_1) == LOW) {
			pin1ConnectedTo = INPUT_PIN_1;
        } else if (digitalRead(INPUT_PIN_2) == LOW) {
			pin1ConnectedTo = INPUT_PIN_2;
        } else {
			pin1ConnectedTo = 0;
        }
    break;
    case 2:
    	digitalWrite(OUTPUT_PIN_1, LOW);
    	digitalWrite(OUTPUT_PIN_2, LOW);
      //delay(RELAY_DELAY);
    	char message[40];
    	sprintf(message, "%d to %d, %d to %d", OUTPUT_PIN_1, pin1ConnectedTo, OUTPUT_PIN_2, pin2ConnectedTo);
    	Serial.println(message);
    break;
  }
}