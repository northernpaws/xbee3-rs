use tokio;

#[tokio::main]
fn main() {
    let ports = serialport::available_ports()
        .expect("No ports found!");
    
    let port = ports[0];

    // We can initialize the Tokio transport with an instance of the serial port
    // because the serial port implements the std::io::{Write,Read} traits.
    let device = xbee_rs::tokio::open(port);
}
