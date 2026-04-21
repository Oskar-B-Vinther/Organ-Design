
#set document(
  title: "how to audrino wil comunitcate with the computer",
)
#title();
#outline();
#pagebreak();

= Compunitcation protocall
Each message start with a byte wich shows which messaage is sent. By knowsing this I will have a standard of the messages sendt.
For this projekt i will have the following messages:
This program will only use one channel and treat all event as if on the same channel.

- Ping
- Medievent: note on
- Medievent: note off
- All clear
- Set Tempo
- FatalSerialError

\

== Message type event type
- Ping
  - 0x01

- Medievent: note on:
  - 0x80

- Medievent: note off:
  - 0x90

- All clear:
  - 0x2F

- Set Tempo:
  - 0x51

- FatalSerialError
  - 0x02


inspired by: https://breno.sarmen.to/midi_documentation/list.html


== Medinote
Here it will just be repecented as a byte

== Tine and set time
Each note on, note off and All clear will de repecented as a 2 byte delta time, aka 2 byte repecenting the time between the last event and this event.

== Combined compoosison:
a message will start with an id byte, then following will be the data bytes.
Example:

Ping:
- ([0x01]), ....

Medievent: note on:
- ([0x80], [medinote], [deltaTime1], [deltaTime2]),....


All clear:
- ([0x2F], [deltaTime1], [deltaTime2])

