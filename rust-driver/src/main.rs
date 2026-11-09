// ////////////////////////////////////////////////////////////////////
//   // FILE       :: main.rs
//   // AUTHOR     :: rowsred
//   // DATE       :: 2026-06-20
//   // HOST_OS    :: Linux
//   // DESC       :: just keep , code
// ////////////////////////////////////////////////////////////////////
//
//
//
use std::io;
use std::io::Write;
use std::thread;
use std::time::Duration;
const Q_PID: u16 = 0x05C6;
const Q_VID: u16 = 0x9008;
fn search_port() -> Option<String> {
    let ports = serialport::available_ports().ok()?;
    for p in ports {
        if let serialport::SerialPortType::UsbPort(usb_info) = p.port_type {
            if usb_info.vid == Q_VID && usb_info.pid == Q_PID {
                return Some(p.port_name);
            }
        }
    }
    None
}
fn main() {
    println!("Waiting conection...");
    let port_name = loop {
        if let Some(name) = search_port() {
            println!("Conected {}", name);
            break name;
        }
    };

    println!("Try hold port : {}", port_name);
    let port_builder = serialport::new(&port_name, 115_200)
        .timeout(Duration::from_millis(1000))
        .data_bits(serialport::DataBits::Eight)
        .parity(serialport::Parity::None)
        .stop_bits(serialport::StopBits::One);

    match port_builder.open() {
        Ok(mut _stream) => {
            println!("Berhasil hold : {}", port_name);
            loop {
                thread::sleep(Duration::from_secs(1));
            }
        }
        Err(e) => {
            eprintln!("gagal:{}", e)
        }
    }
}
