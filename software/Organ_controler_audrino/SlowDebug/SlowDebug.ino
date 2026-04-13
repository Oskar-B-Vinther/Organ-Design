

int shift = 13; 
int latch = 6; 
int data = 11; 

int test[8] = {1,1,1,1,1,1,1,1};
int test1[8] = {0,0,0,0,0,0,0,0};

void setup() {
pinMode(shift,OUTPUT);
pinMode(latch,OUTPUT);
pinMode(data,OUTPUT);
}

void loop() {
  // put your main code here, to run repeatedly:

shiftData(test,500);
shiftData(test1,500);
}

void shiftData(int byte[], int Delaytime) {

for (int i = 0; i<8;i++){
  // data either high or low
  if (byte[i] == 1){
    digitalWrite(data, HIGH);
  } else {
    digitalWrite(data, LOW);
  }
    // delay(Delaytime*2); // So data can be messured. 

    // shift the data once outways. 
    digitalWrite(shift, LOW);
    digitalWrite(shift, HIGH);
    //delay(Delaytime); // so shift can be messured. 
    digitalWrite(shift, LOW);

/*
    // the latch
     digitalWrite(latch, LOW);
     digitalWrite(latch, HIGH);
     delay(Delaytime); // so shift can be messured. 
     digitalWrite(latch, LOW);
*/
  }
  digitalWrite(data, LOW); // data pin becomes low after each shiftout, so it's now high unless it needs to. 

  // the latch
  digitalWrite(latch, LOW);
  digitalWrite(latch, HIGH);
  delay(Delaytime); // so shift can be messured. 
  digitalWrite(latch, LOW);
}










