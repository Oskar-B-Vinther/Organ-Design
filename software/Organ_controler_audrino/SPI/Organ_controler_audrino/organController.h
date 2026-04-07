
#ifndef ORGANCONTROLLER_H
#define ORGANCONTROLLER_H

#include <Arduino.h>
#include <SPI.h>

class organController {

 public:
   byte config[4] = {0x00};  // there are medinotes, which are aviliable on my organ. I only have 32 values EI. 8 x 4 = 32. 
   uint8_t clearConst = 0x00;        // All pins LOW (0)
   uint8_t fullConst  = 0xFF;        // All pins HIGH (255)

// controler pins  // hardcoded 
   int latchPin;
   int powerpin;

// organ config
   int organMedistart, organMedistop;
    organController(int powerpin,int latchpin, int organMedistart, int organMedistop);
    void start();
    void load();
    void set();
    void clear();
    void define_note_at();
};

#endif // end of heaeer files
