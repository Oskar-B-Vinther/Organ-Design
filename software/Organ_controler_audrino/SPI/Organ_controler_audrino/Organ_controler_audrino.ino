

#include "organController.h"

// this class uses spi. so it takes up pin 10,11, and 13. 
organController pipeOrgan(5/*power ping*/, 36 /*possiple medi start*/, 96 /*possiple medi stop*/);
int comport = 3;


void setup() {
  pipeOrgan.start();
}

void loop() {
 //------------// change the medi config
test ();
/*
pipeOrgan.load(0x00);
pipeOrgan.set();
delay(2000);

pipeOrgan.load(0xFF);
pipeOrgan.set();
delay(2000);
*/
 //------------// here ^-

}
void test (){
 pinMode(10, OUTPUT);
 pinMode(11, OUTPUT);
 pinMode(13, OUTPUT);

digitalWrite(10, LOW);   
digitalWrite(11, LOW);   

for (int i = 0; i<8; i++){
digitalWrite(13, LOW);   
delay(2);
digitalWrite(13, HIGH);   
delay(2);
}

digitalWrite(10, HIGH);   
}


