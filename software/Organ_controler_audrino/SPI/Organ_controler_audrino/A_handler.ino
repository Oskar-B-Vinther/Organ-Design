/*
#include <SPI.h>

class organController {
bool config[128] = {false};  // there are medinotes,   0-127.  

long Bits = 0; 
long bit =4294967295;

// controler pins  // hardcoded 
int setPin = 11;
int shiftPin = 13;
int pushPin = 10;
int powerpin;

// organ config
int organMedistart, organMedistop; 

 public: organController (
int powerpin, 
int organMedistart, 
int organMedistop

 ){ // constructor
    this->powerpin = powerpin;
    this->organMedistart = organMedistart;
    this->organMedistop = organMedistop;
    SPI.begin();
  }

 public: void start() {
  clear();
  pinMode(powerpin,OUTPUT);
  digitalWrite(powerpin, HIGH);
  } 

  public: void Update() {
 SPI.transfer(bit) ;
  }

  public: void clear(){
   SPI.transfer(Bits);
  }

};

*/