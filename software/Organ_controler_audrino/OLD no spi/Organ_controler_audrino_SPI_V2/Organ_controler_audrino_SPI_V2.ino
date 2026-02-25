
class organController {
bool config[128] = {false};  // there are medinotes,   0-127.  

// controler pins
int setPin, shiftPin, pushPin;
int powerpin;

// organ config
int organMedistart, organMedistop; 

 public: organController (
int setPin,
int shiftPin,
int pushPin, 
int powerpin, 
int organMedistart, 
int organMedistop

 ){ // constructor
    this->setPin = setPin;
    this->shiftPin = shiftPin;
    this->pushPin = pushPin;
    this->powerpin = powerpin;
    this->organMedistart = organMedistart;
    this->organMedistop = organMedistop;
  }

 public: void start() {
  clear();
  pinMode(powerpin,OUTPUT);
  digitalWrite(powerpin, HIGH);
  } 

  public: void Update() {
    pinMode(setPin,OUTPUT);





    for (int i = organMedistart; i<organMedistop;i++) {
      digitalWrite(setPin, config[i]);
      pulsePin(shiftPin);
    }
    pulsePin(pushPin);
  }

  public: void clear(){
  pinMode(setPin,OUTPUT);
  
    for (int i = organMedistart; i<organMedistop;i++) {
      digitalWrite(setPin, false);
      pulsePin(shiftPin);   
      }
      pulsePin(pushPin);
  }

  private: void pulsePin(int pin){
    pinMode(pin,OUTPUT);
    digitalWrite(pin, HIGH);
    delay(1);
    digitalWrite(pin, LOW);
  }

};




organController pipeOrgan(2, 3, 4, 5, 36, 96);
int comport = 3;


void setup() {
  pipeOrgan.start();

}

void loop() {
 //------------// change the medi config


 //------------// here ^-

pipeOrgan.Update();

}



