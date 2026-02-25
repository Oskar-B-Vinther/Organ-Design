

#ifndef ORGANCONTROLLER_H
#define ORGANCONTROLLER_H

#include <Arduino.h>
#include <SPI.h>

class organController {

 public:
   bool config[128] = {false};  // there are medinotes,   0-127.  
   uint8_t clearConst = 0x00;        // All pins LOW (0)
   uint8_t fullConst  = 0xFF;        // All pins HIGH (255)

// controler pins  // hardcoded 
   //int setPin = 11;
  // int shiftPin = 13;
   int latchPin = 10;
   int powerpin;

// organ config
   int organMedistart, organMedistop;
    organController(int powerpin, int organMedistart, int organMedistop);
    void start();
    void load(uint8_t bit);
    void set();
    void clear();
};

#endif // end of heaeer files
