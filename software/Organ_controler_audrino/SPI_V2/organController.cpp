#include "organController.h"

SceduledEvent::SceduledEvent(){
      for (int i = 0;i<4;i++) {
         config[i] = {0xFF}; 
      }
    freash =false;
      // not used
      Deltatime = 1000; 
}

organController::organController(int powerPin,int latchPin, int organMedistart, int organMedistop) {
  this->powerPin = powerPin;
  this->latchPin = latchPin;
  this->organMedistart = organMedistart;
  this->organMedistop = organMedistop;
 
  for (int i = 0; i<32; i++){
  events[i] = SceduledEvent();
  }

  writeIndex = 0; 
}

void organController::start() {
 Serial.begin(9600);
  //-----// define pins
  // define SPI: 
  SPI.begin();
  SPI.beginTransaction(SPISettings(4000000, LSBFIRST, SPI_MODE0));

  // define lacth pin: 
  pinMode(latchPin, OUTPUT);

  // define power pin to solinodes: 
  pinMode(powerPin, OUTPUT);

  //-----// reset system
  clear();
  load();
  set();

  // Turn on power to the system
  digitalWrite(powerPin, HIGH); // power safty to reasure power to mosfets are tuned off during startup
}

 void organController::update(){
  readNextEvent();

    if (!IsBufferFilled()){
      Serial.write(0x04); // gives signal to the server to send new config
    }
}


void organController::readNextEvent(){
  byte infobyte = Serial.read();

  switch (infobyte){
      case 0x01: // ping
        Serial.write(0x86); //  respons back. 
        break;

      case 0x52: // new config ariwed. 

      break;

      case 0x51: // medi set tempo and start
        clearBuffer();
        TimingEvent();
        break;

      default: // FatalSerial error as the type of message was not requrenized. 
      //Serial.write(0x02);
        break; 
        }
}


// pushes whatever is in the config out to the organs internal memory. 
void organController::load() {
  SceduledEvent Event = events[readIndex];
  for (int i = 0; i<4;i++){
    SPI.transfer(Event.config[i]);
    }
}
// uses the latch pin to to change the external pins on the shifitng register to the internal values. 
void organController::set() {
  digitalWrite(latchPin, LOW);
  digitalWrite(latchPin, HIGH);
}
// Set the whole config to "all off" sate
void organController::clear() {
  for (int i = 0; i<4;i++){
    SPI.transfer(0x00);
    }
  set();
}

// functions to maanges the idex of the event buffer.
void organController::nextReadIndex() {
  events[readIndex].freash = true;
  readIndex = (readIndex+1) % 32;
}

void organController::nextWriteIndex() {
  writeIndex = (writeIndex+1) %32;
}


void organController::newConfig(){
  for (i = 0; i<4, i++) {
  events[writeIndex].config[i]
  }

  byte data[4] = {0x00, 0x00, 0x00, 0x00};
  for (int i = 0; i<4; i++){
    data[i] = Serial.read();
  }

// Conbines the bytes into a single Deltatine int.  
  events[writeIndex].Deltatime = 
                       ((uint32_t)data[0] << 24) |
                       ((uint32_t)data[1] << 16) |
                       ((uint32_t)data[2] << 8)  |
                       ((uint32_t)data[3]);

}

bool organController::IsBufferFilled(){
  bool filled = true;
  for (int i = 0; i<32;i++){
    if (events[i].freash) {
      filled = false;
    }
  }
  return filled;
}

void organController::TimingEvent() {
  byte time_one = Serial.read();
  byte time_two = Serial.read();
  int time = (int)(time_one << 8) | time_two;
  bpm = time; 
  StartRead = true; 
  }

  void organController::printState(){
    for (int i = 0;i<32;i++){
        Serial.print(events[i].config[0]);
        Serial.print(" , ");
        Serial.print(events[i].config[1]);
        Serial.print(" , ");
        Serial.print(events[i].config[2]);
        Serial.print(" , ");
        Serial.print(events[i].config[3]);
        Serial.print("\n");
    }
    Serial.print("\n");
    Serial.print("\n");
    Serial.print("\n");
}

void organController::clearBuffer(){
  for (int i = 0;i<32;i++){
    events[i].freash = false;

    for (int j = 0; j<4;j++){
      events[i].config[j] = 0x00;
    }
  }
}






