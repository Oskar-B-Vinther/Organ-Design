

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
}

void loop() {
 //------------// change the medi config

  for (int i = 57; i<89;i++){
  pipeOrgan.Set_Medi_Note(i,true);
  }
  pipeOrgan.load();
  pipeOrgan.set();
  delay(1000);

  
  pipeOrgan.clear();
  pipeOrgan.load();
  pipeOrgan.set();
  delay(1000);
}















void test (){
 int shift = 13;
 int data = 11;
 int latch = 10;
 
 pinMode(latch, OUTPUT); // latch
 pinMode(data, OUTPUT); // data 
 pinMode(shift, OUTPUT); // shift

digitalWrite(data, HIGH);   

for (int i = 0; i<8; i++){
digitalWrite(shift, LOW);   
delay(2);
digitalWrite(shift, HIGH);   
delay(2);
}

digitalWrite(shift, LOW);   

digitalWrite(latch, HIGH);   
delay(2);
digitalWrite(latch, LOW);   

}
/*
void sendToShiftRegister(byte data) {
  shiftOut(dataPin, clockPin, MSBFIRST, data);
     digitalWrite(dataPin, LOW);   
>>>>>>> 039881788416562258159568d9d208abdcba79c1
      digitalWrite(latchPin, HIGH);    // Prepare to load data
     //digitalWrite(latchPin, LOW);          // Update outputs

      digitalWrite(dataPin, LOW);  
}
*/
