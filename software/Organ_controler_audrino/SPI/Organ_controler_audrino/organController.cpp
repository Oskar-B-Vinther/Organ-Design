#include "organController.h"





SceduledEvent::SceduledEvent(){
      for (int i = 0;i<4;i++) {
         config[i] = {0x00}; 
      }
      Deltatime = 0; 
      freash = true;

      newtempo = 0;
      changeTempo = false;  
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
  fullConst = 0; 
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
  digitalWrite(powerPin, HIGH);
}

SceduledEvent readNextEvent(){
byte infobyte = Serial.read();
switch (infobyte){
    case 0x01: // ping
    Serial.write(0x01); //  respons back. 
      break; 

    case 0x80: // Medi on

      break; 

    case 0x90: // Medi off

      break; 

    case 0x2F: // medi all clear

      break; 

    case 0x51: // medi set tempo

      break;

    default: // FatalSerial error as the type of message was not requrenized. 
    Serial.write(0x02);
      return SceduledEvent();
      break; 
      }


  Serial.write(0x02); // undefined result
  return SceduledEvent();
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
/*
void organController::set(long triggertime){
  Timer1.initialize(triggertime); 
  Timer1.attachInterrupt(fast_latch); 
}
*/

// Set the whole config to "all off" sate
void organController::clear() {
  for (int i = 0; i<4;i++){
   config[i] = clearConst;
  }
}

//------------// error codes
/*
1: Out of range: Note to high
2: Out of range: Note to low
*/
//-------------//
int organController::Set_Medi_Note(int note, bool state){ // note is medi 0-128. and state is "on" or "off" EI. 1 or 0.  

// error handling tet of our of range
if (note >organMedistop ){
return 1; 
} 
if (note < organMedistart ){
return 2; 
} 
//----------//
note = note-organMedistart; // shift the range down so its from 0-32
byte ByteN = note / 8; // whole number division to get the byte in which the desirreed change is in. 

byte part = config[ByteN]; 
if (state){ // checks if the not should be set to on or off. 
part = bitSet(part,note % 8); // here the modulo function is used to get the excat possistion of the bit in the specific byte. 
} else {
part = bitClear(part,note % 8);
}
config[ByteN] = part; // applys the change to the organ config. 
return 0; 
}


// functions to maanges the idex of the event buffer.
void organController::nextReadIndex() {
readIndex = (readIndex+1) %32;
}

void organController::nextWriteIndex() {
writeIndex = (writeIndex+1) %32;
}






