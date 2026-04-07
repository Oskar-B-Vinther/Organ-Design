

#include "organController.h"

// this class uses spi. so it takes up pin 10,11, and 13. 
//pin 13 SCLK (shift pin)
// pin 11 (bit out)
// pin 10 latcth
organController pipeOrgan(5/*power ping*/, 36 /*possiple medi start*/, 96 /*possiple medi stop*/);
int comport = 3;

  const int dataPin  = 11;   // DS
  const int latchPin = 10;   // STCP
  const int clockPin = 13;  // SHCP



void setup() {
//pipeOrgan.start();

  pinMode(dataPin, OUTPUT);
  pinMode(latchPin, OUTPUT);
  pinMode(clockPin, OUTPUT);
}

void loop() {
 //------------// change the medi config

//sendToShiftRegister((byte)0);
//delay(2000);

sendToShiftRegister((byte)255);
delay(2000);


/*
pipeOrgan.load(0x00);
pipeOrgan.set();
delay(2000);

pipeOrgan.load(0xFF);
pipeOrgan.set();
delay(2000);

 //------------// here ^-

 */

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






void sendToShiftRegister(byte data) {
  shiftOut(dataPin, clockPin, MSBFIRST, data);
     digitalWrite(dataPin, LOW);   
      digitalWrite(latchPin, HIGH);    // Prepare to load data
      digitalWrite(latchPin, LOW);          // Update outputs
    
}









