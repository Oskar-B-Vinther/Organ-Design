

int shift = 10; 
int latch = 11; 
int data = 12; 

int test[8] = {1,1,1,1,1,1,1,1};


void setup() {
pinMode(shift,OUTPUT);
pinMode(latch,OUTPUT);
pinMode(data,OUTPUT);

}

void loop() {
  // put your main code here, to run repeatedly:





}



void shiftData(int byte[], int Delaytime) {

for (int i = 0; i<8;i++){
  // data either high or low
  if (byte[i] == 1){
    digitalWrite(data, HIGH);
  } else {
    digitalWrite(data, LOW);
  }
    delay(Delaytime*2); // So data can be messured. 


    // shift the data once outways. 
    digitalWrite(shift, LOW);
    digitalWrite(shift, HIGH);
    delay(Delaytime); // so shift can be messured. 
    digitalWrite(shift, LOW);

  }
  digitalWrite(data, LOW); // data pin becomes low after each shiftout, so it's now high unless it needs to. 

  // the latch
  digitalWrite(data, LOW);
  digitalWrite(data, HIGH);
  delay(Delaytime); // so shift can be messured. 
  digitalWrite(data, LOW);

}










