#include <Arduino.h>

const int LEFT_LED_OUT = 2;
const int RIGHT_LED_OUT = 3;
const int LEFT_LED_IN = 4;
const int RIGHT_LED_IN = 5;

void setup() {
  pinMode(LEFT_LED_OUT, OUTPUT);
  pinMode(RIGHT_LED_OUT, OUTPUT);
  pinMode(LEFT_LED_IN, INPUT);
  pinMode(RIGHT_LED_IN, INPUT);

  digitalWrite(LEFT_LED_OUT, HIGH);
  digitalWrite(RIGHT_LED_OUT, HIGH);

  Serial.begin(9600);
  while(!Serial) {}
  Serial.write("\n\nREADY\n\n");
}

int lastLED = LEFT_LED_OUT;

bool leftLast = false;
bool rightLast = false;

void loop() {
  int left = digitalRead(LEFT_LED_IN);
  int right = digitalRead(RIGHT_LED_IN);

  //This is a heartbeat message, just so the client won't time out.
  Serial.write(".");

  if(left == LOW) {
    if(!leftLast) {
      Serial.write("{");
      leftLast = true;
    }
  } else {
    if(leftLast) {
      Serial.write("[");
    }

    leftLast = false;
  }

  if(right == LOW) {
    if(!rightLast) {
      Serial.write("}");
      rightLast = true;
    }
  } else {
    if(rightLast) {
      Serial.write("]");
    }

    rightLast = false;
  }

  delay(20);
}
