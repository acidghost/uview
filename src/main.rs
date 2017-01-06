extern crate pcap;
extern crate clap;
extern crate serial;

mod uviewpacket;
use uviewpacket::{UViewPacket, ValueType};

use pcap::Device;
use clap::{App, SubCommand};
use serial::prelude::*;
use serial::PortSettings;

use std::time::SystemTime;



fn network_bandwidth(interval: ValueType, port: &mut SerialPort, bpf: Option<&str>, max_bandwidth: ValueType) {
    let mut cap = Device::lookup().unwrap().open().unwrap();
    let mut timer = SystemTime::now();
    let mut bytes = UViewPacket::new();

    if let Some(filter) = bpf {
        cap.filter(filter).unwrap();
    }

    while let Ok(packet) = cap.next() {
        bytes += packet.header.caplen as ValueType;
        if let Ok(duration) = SystemTime::now().duration_since(timer) {
            if duration.as_secs() >= interval {
                timer = SystemTime::now();
                let scaled = bytes.scale(0, max_bandwidth);
                println!("{:?}", scaled);
                port.write(scaled.to_string().as_bytes()).unwrap();
                bytes = UViewPacket::new();
            }
        }
    }
}


fn main() {
    let matches =
        App::new("µview")
            .version("1.0")
            .author("acidghost")
            .about("Monitors stuff and reports to the MicroView")
            .args_from_usage(
                "[interval] -i, --interval=[interval] 'Interval of time between updates'
                 <port> -p, --port=[port] 'MicroView serial port'")
            .subcommand(SubCommand::with_name("network")
                .about("Network monitoring")
                .args_from_usage("[filter] -f, --filter=[filter] 'Berkeley Packet Filter to use'")
                .subcommand(SubCommand::with_name("bandwidth")
                    .about("Network bandwidth monitoring")
                    .args_from_usage("<max> -m, --max=[max] 'Max bandwidth'")))
            .get_matches();

    let interval: u64 = matches.value_of("interval").unwrap_or("1").parse().unwrap();
    let port = matches.value_of("port").unwrap();

    let mut serial_port = serial::open(port).unwrap();
    let mut serial_settings = PortSettings::default();
    serial_settings.set_baud_rate(serial::Baud115200).unwrap();
    serial_port.configure(&mut serial_settings).unwrap();

    if let Some(network_matches) = matches.subcommand_matches("network") {
        let bpf = network_matches.value_of("filter");
        if let Some(bandwidth_matches) = network_matches.subcommand_matches("bandwidth") {
            let max_bandwidth = bandwidth_matches.value_of("max").unwrap().parse().unwrap();
            network_bandwidth(interval, &mut serial_port, bpf, max_bandwidth);
        }
    }

    println!("Bye-Bye!");
}
