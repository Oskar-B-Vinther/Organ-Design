

#include "organController.h"

// this class uses spi. so it takes up pin 10,11, and 13. 
//pin 13 SCLK (shift pin)
// pin 11 (bit out)
// pin 6 latcth

organController pipeOrgan(
      5 /*power ping*/,
      6 /*latch pin*/,  
      57 /*possiple medi start*/, 
      89 /*possiple medi stop*/
          );
  
void setup() {
pipeOrgan.start();


// Set a "test" config. 

  
 pipeOrgan.StartRead = false; 



}

void loop() {
 //------------// change the medi config
//pipeOrgan.readNextEvent();

    delay(1000);
    for(int i =0; i<4;i++){
    pipeOrgan.config[i] = pipeOrgan.fullConst;
   }
   pipeOrgan.loadManual();
   pipeOrgan.set();


    delay(1000);
 //  pipeOrgan.clear();
   pipeOrgan.loadManual();
   pipeOrgan.set();


    if ( pipeOrgan.StartRead ){
    pipeOrgan.StartRead = false;

    pipeOrgan.load();
    fast_latch();
    }

}

// needs to be as the function need to be static while still haveing acsess to pipeorgan metods- 
static void fast_latch(){
   // PORTB |= (1 << PORTB6);
   // PORTB &= ~(1 << PORTB6);
    pipeOrgan.set();
    pipeOrgan.nextReadIndex();
    pipeOrgan.load();
    Timer1.initialize(pipeOrgan.events[pipeOrgan.readIndex].Deltatime);
    Timer1.attachInterrupt(fast_latch);
  }
