
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
  SceduledEvent();
};
//-------------------------------------------------------------------//
class organController {
public:
  // cycle buffer variables.
  bool StartRead;
  byte writeIndex;
  byte readIndex;
  SceduledEvent events[32];

  int state = 0;
  // controler pins  // hardcoded
  int latchPin;
  int powerPin;

  // organ config
  int organMedistart, organMedistop;
  int nextTime;
  int bpm;
  // Serialport
  int baudSpeed;

  // constuctor
  organController(int powerPin, int latchPin, int organMedistart, int organMedistop);
  //fucntions
  void start();
  void update();
  void readNextEvent();

  void nextReadIndex();
  void nextWriteIndex();

  void clearBuffer();
  bool IsBufferFilled();

  void TimingEvent();
  void newConfig();
  void load();
  void set();
  void clear();
  //static void fast_latch(); // latch wich use hardware manipulation to quicly flip the pin
  //debug
  void printState();
};

#endif  // end of heaeer files
