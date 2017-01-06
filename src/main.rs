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
use std::thread;
use std::sync::mpsc;



fn network_bandwidth(tx: mpsc::Sender<UViewPacket<ValueType>>, bpf: Option<String>, max_bandwidth: ValueType) {
    let mut cap = Device::lookup().unwrap().open().unwrap();

    if let Some(filter) = bpf {
        cap.filter(filter.as_str()).unwrap();
    }

    while let Ok(packet) = cap.next() {
        let bytes = UViewPacket::new(packet.header.caplen as ValueType).scale(0, max_bandwidth);
        tx.send(bytes).unwrap();
    }
}


fn uview_sender(rx: mpsc::Receiver<UViewPacket<ValueType>>, serial_port: &mut SerialPort, interval: u64) {
    let mut timer = SystemTime::now();
    let mut accumulator = UViewPacket::new(0 as ValueType);
    loop {
        accumulator += match rx.try_recv() {
            Ok(val) => val,
            Err(_) => UViewPacket::new(0 as ValueType)
        };
        if let Ok(duration) = SystemTime::now().duration_since(timer) {
            if duration.as_secs() >= interval {
                println!("{:?}", accumulator);
                serial_port.write(accumulator.to_string().as_bytes()).unwrap();
                accumulator = UViewPacket::new(0 as ValueType);
                timer = SystemTime::now();
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

    let (tx, rx) = mpsc::channel();
    let do_sender = || { uview_sender(rx, &mut serial_port, interval) };

    if let Some(network_matches) = matches.subcommand_matches("network") {
        let bpf = network_matches.value_of("filter").map(|f| f.to_string());
        if let Some(bandwidth_matches) = network_matches.subcommand_matches("bandwidth") {
            let max_bandwidth = bandwidth_matches.value_of("max").unwrap().parse().unwrap();
            thread::spawn(move || network_bandwidth(tx, bpf, max_bandwidth));
            do_sender();
        }
    }

    println!("Bye-Bye!");
}
