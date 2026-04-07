#include "organController.h"

organController::organController(int powerpin,int latchpin, int organMedistart, int organMedistop) {
  this->powerpin = powerpin;
  this->latchpin = latchpin;
  this->organMedistart = organMedistart;
  this->organMedistop = organMedistop;
 
  
}

void organController::start() {

//-----// define pins
  // define SPI: 
 SPI.begin();
  SPI.setClockDivider(SPI_CLOCK_DIV8); 
  SPI.setDataMode(SPI_MODE0);
  SPI.setBitOrder(MSBFIRST);

// define lacth pin: 
  pinMode(latchPin, OUTPUT);

// define power pin to solinodes: 
  pinMode(powerpin, OUTPUT);

//-----// reset system
  clear();
  load();
  set();

// Turn on power to the system
  digitalWrite(powerpin, HIGH);
}

// pushes whatever is in the config out to the organs internal memory. 
void organController::load() {
 for (int i = 0; i<4;i++){
   SPI.transfer(config[i]);
  }
}


// uses the latch pin to to change the external pins on the shifitng register to the internal values. 
void organController::set() {
  digitalWrite(latchPin, LOW);
  delay(1);
  digitalWrite(latchPin, HIGH);
}

// Set the whole config to "all off" sate
void organController::clear() {
  for (int i = 0; i<4;i++){
   config[i] = clearConst;
  }
}

void organController::define_note_at(){




}


