#include "organController.h"

organController::organController(int powerpin, int organMedistart, int organMedistop) {
  this->powerpin = powerpin;
  this->organMedistart = organMedistart;
  this->organMedistop = organMedistop;
  

  SPI.begin();
  SPI.setClockDivider(SPI_CLOCK_DIV8); 
  SPI.setDataMode(SPI_MODE0);
  SPI.setBitOrder(MSBFIRST);
}

void organController::start() {
  pinMode(latchPin, OUTPUT);
  digitalWrite(latchPin, LOW);   
  clear();
  pinMode(powerpin, OUTPUT);
  digitalWrite(powerpin, HIGH);
}

void organController::load(uint8_t bit) {
  digitalWrite(latchPin, LOW);
  SPI.transfer(bit);
}

void organController::set() {
  digitalWrite(latchPin, HIGH);
}


void organController::clear() {
  load(clearConst);
  set();
}