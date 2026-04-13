use anyhow::{Context, Result};
use serialport::SerialPort;
use std::io::{Read, Write};
use std::time::Duration;
 
/* =========================
   STATIC CONFIG (edit these)
   ========================= */
 
const PORT_NAME: &str = "/dev/ttyACM0"; // Windows example: "COM3"
const BAUD_RATE: u32 = 115_200;         // Must match Arduino Serial.begin(...)
const TIMEOUT_MS: u64 = 200;            // Read timeout
 
const READ_REPLY: bool = false;         // true if Arduino echoes/replies
const SEND_DELAY_MS: u64 = 50;          // Optional delay between messages
 
fn main() -> Result<()> {
    // 1) Open the serial port
    let mut port = open_port(PORT_NAME, BAUD_RATE, TIMEOUT_MS)?;
 
    // 2) Send some raw bytes (example)
    let raw: [u8; 4] = [0x01, 0x02, 0xFF, 0x10];
    send_bytes(&mutport, &raw)?;
    if READ_REPLY {
        read_some(&mut port)?;
    }
 
    std:🧵:sleep(Duration::from_millis(SEND_DELAY_MS));
 
    // 3) Send MIDI-style messages (Note On then Note Off)
    // Middle C = 60 (0x3C)
    let note_on = midi_note_on(1, 60, 127)?;
    send_bytes(&mutport, &note_on)?;
    if READ_REPLY {
        read_some(&mut port)?;
    }
 
    std:🧵:sleep(Duration::from_millis(250));
 
    let note_off = midi_note_off(1, 60, 0)?;
    send_bytes(&mutport, &note_off)?;
    if READ_REPLY {
        read_some(&mut port)?;
    }
 
    Ok(())
}
 
/* =========================
   Serial helpers
   ========================= */
 
fn open_port(port_name: &str, baud: u32, timeout_ms: u64) -> Result<Box<dyn SerialPort>> {
    let timeout = Duration::from_millis(timeout_ms);
 
    let port = serialport::new(port_name, baud)
        .timeout(timeout)
        .open()
        .with_context(|| format!("Failed to open serial port '{}'", port_name))?;
 
    println!("Opened serial port '{}' at {} baud", port_name, baud);
    Ok(port)
}
 
fn send_bytes(port: &mut dyn SerialPort, bytes: &[u8]) -> Result<()> {
port.writeall(bytes).context("Failed to write bytes to serial port")?;
 
    // On some platforms flush is a no-op; ignoring flush error is usually fine.
    let  = port.flush();
 
    println!("Sent {} bytes: {}", bytes.len(), format_hex(bytes));
    Ok(())
}
 
fn read_some(port: &mut dyn SerialPort) -> Result<()> {
    let mut buf = [0u8; 256];
    match port.read(&mut buf) {
        Ok(n) if n > 0 => {
            println!("Received {} bytes: {}", n, formathex(&buf[..n]));
        }
        Ok() => {
            println!("Received 0 bytes");
        }
        Err(e) => {
            // Timeouts are expected if Arduino didn't send anything back.
            println!("Read error (often timeout): {e}");
        }
    }
    Ok(())
}
 
fn format_hex(bytes: &[u8]) -> String {
    let mut out = String::new();
    for (i, b) in bytes.iter().enumerate() {
        if i > 0 {
            out.push(' ');
        }
        out.push_str(&format!("{:02X}", b));
    }
    out
}
 
/* =========================
   MIDI helpers (pure bytes)
   =========================
   MIDI messages are just bytes:
   - Status byte: command + channel
   - Data bytes: 0..=127 (7-bit)
 
   Note On : 0x90 | (channel-1), note, velocity
   Note Off: 0x80 | (channel-1), note, velocity
*/
 
fn midi_note_on(channel: u8, note: u8, velocity: u8) -> Result<[u8; 3]> {
    validate_midi(channel, note, velocity)?;
    let status = 0x90 | ((channel - 1) & 0x0F);
    Ok([status, note & 0x7F, velocity & 0x7F])
}
 
fn midi_note_off(channel: u8, note: u8, velocity: u8) -> Result<[u8; 3]> {
    validate_midi(channel, note, velocity)?;
    let status = 0x80 | ((channel - 1) & 0x0F);
    Ok([status, note & 0x7F, velocity & 0x7F])
}
 
fn validate_midi(channel: u8, note: u8, velocity: u8) -> Result<()> {
    anyhow::ensure!((1..=16).contains(&channel), "MIDI channel must be 1..=16");
    anyhow::ensure!(note <= 127, "MIDI note must be 0..=127");
anyhow::ensure!(velocity <= 127, "MIDI velocity must be 0..=127");
    Ok(())
}
``