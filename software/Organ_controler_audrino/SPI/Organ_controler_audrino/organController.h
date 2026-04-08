
#ifndef ORGANCONTROLLER_H
#define ORGANCONTROLLER_H

#include <Arduino.h>
#include <SPI.h>
#include <TimerOne.h>

class organController {

 public:
   byte config[4] = {0x00};  // there are medinotes, which are aviliable on my organ. I only have 32 values EI. 8 x 4 = 32. 
   byte clearConst = 0x00;        // All pins LOW (0)
   byte fullConst  = 0xFF;        // All pins HIGH (255)
   
// controler pins  // hardcoded 
   int latchPin;
   int powerPin;

// organ config
   int organMedistart, organMedistop;
   int nextTime;

   // constuctor
    organController(int powerPin,int latchPin, int organMedistart, int organMedistop);

    //fucntions
    void start();
    void load();
    void set();
    void set(long triggertime);
    static void fast_latch();
    void clear();
    int Set_Medi_Note(int note, bool state);
};

#endif // end of heaeer files
