use serialport::available_ports;

fn main() {
    let ports = available_ports().expect("No ports found!");
    for p in ports {
        println!("{}", p.port_name);
    }
}
