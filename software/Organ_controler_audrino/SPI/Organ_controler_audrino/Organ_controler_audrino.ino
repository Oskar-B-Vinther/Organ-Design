
// this class uses spi. so it takes up pin 10,11, and 13. 
organController pipeOrgan(5/*power ping*/, 36 /*possiple medi start*/, 96 /*possiple medi stop*/);
int comport = 3;


void setup() {
  pipeOrgan.start();
}

void loop() {
 //------------// change the medi config


 //------------// here ^-

pipeOrgan.Update();

}



