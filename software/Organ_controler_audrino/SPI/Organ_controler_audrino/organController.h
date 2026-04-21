
#ifndef ORGANCONTROLLER_H
#define ORGANCONTROLLER_H

#include <Arduino.h>
#include <SPI.h>
#include <TimerOne.h>

class SceduledEvent {
     public: 
      byte config[4];
      int Deltatime; 
      bool freash; 
      bool changeTempo;
      int newtempo; 
      SceduledEvent();
};

class organController {
 public:
   byte config[4] = {0x00};  // there are medinotes, which are aviliable on my organ. I only have 32 values EI. 8 x 4 = 32. 

   byte writeIndex;
   byte readIndex;
   SceduledEvent events[32];

   byte clearConst = 0x00;        // All pins LOW (0)
   byte fullConst  = 0xFF;        // All pins HIGH (255)
   
// controler pins  // hardcoded 
   int latchPin;
   int powerPin;

// organ config
   int organMedistart, organMedistop;
   int nextTime;

// Serialport
   int baudSpeed;

   // constuctor
    organController(int powerPin,int latchPin, int organMedistart, int organMedistop);
    //fucntions
    void start(); 
    void nextReadIndex();
    void nextWriteIndex();
    SceduledEvent readNextEvent();


    void load();
    void set();
    void set(long triggertime);
    //static void fast_latch(); // latch wich use hardware manipulation to quicly flip the pin
    void clear(); // changes all notes to off same as setting all medi 0-128 to false. 
    int Set_Medi_Note(int note, bool state);

};


#endif // end of heaeer files
