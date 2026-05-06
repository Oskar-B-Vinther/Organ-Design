

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
for (byte i = 0; i<32;i++){
    for (byte j = 0;j<4;j++) {
        pipeOrgan.events[i].config[j] = {i};
        pipeOrgan.events[i].Deltatime = 500000; 
     }
}
  
 pipeOrgan.StartRead = true; 



}

void loop() {
 //------------// change the medi config
//pipeOrgan.readNextEvent();

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
