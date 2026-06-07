#include <Arduino.h>
#include <SPI.h>
#include <TimerOne.h>

int power = 5; 
int latch = 6; 

byte off[4] = {0x00,0x00,0x00,0x00};
byte on[4]  = {0xff, 0xff, 0xff, 0xff};
byte dyn[4]  = {0xff, 0xff, 0xff, 0xff};
// spi 11 and 13 
int index = 0; 
bool bits[32] = {0};

void setup(){
pinMode(latch, OUTPUT);
 SPI.begin();
 SPI.beginTransaction(SPISettings(4000000, LSBFIRST, SPI_MODE0));
 SPITransfer(on);
}

void loop(){
/*
SPITransfer(on);
delay(1000);
SPITransfer(off);
delay(1000);
*/


  for (int i =0; i<32;i++){
    bits[i] = 0; 
  }

  bits[index] = 1; 
  tobyte(bits);
  SPITransfer(dyn);
  index = (index+1) % 32;

  delay(100);

  //SPITransfer(on);
}



void tobyte(bool ID[]){

  for (int i = 0;i<4;i++){
    dyn[i] = 0x00;
  }
  
  for (int j = 0;j<4;j++){
    for(int i=0; i<8; i++){
      dyn[j] |= ID[j*8+i] << i;
    }
  }

}



void SPITransfer(byte bytes[]){
   digitalWrite(latch, LOW);
  for (int i = 0; i<4;i++){
    SPI.transfer(bytes[i]);
  }
 
  digitalWrite(latch, HIGH);
  digitalWrite(latch, LOW);
}
