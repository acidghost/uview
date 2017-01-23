extern crate pcap;
extern crate clap;
extern crate serial;

mod uviewpacket;
use uviewpacket::{UViewPacket, ValueType, DisplayMode, scale};

use pcap::Device;
use clap::{App, SubCommand};
use serial::prelude::*;
use serial::PortSettings;

use std::time::{Duration, SystemTime};
use std::thread;
use std::sync::mpsc;



#[repr(C)]
struct MemInfo {
    pub total: u64,
    pub free: u64,
    pub buffers: u64,
    pub cached: u64,
    pub swap_total: u64,
    pub swap_free: u64
}


extern "C" {
    fn get_mem_info() -> MemInfo;
}



fn memory_free(tx: mpsc::Sender<ValueType>, interval: u64, percentage: bool) {
    loop {
        tx.send(unsafe {
            let mi = get_mem_info();
            if percentage {
                let free = mi.free as f64;
                let total = mi.total as f64;
                ((free / total) * 100_f64) as ValueType
            } else {
                // return it in MB
                mi.free / 1024 / 1024
            }
        }).unwrap();
        thread::sleep(Duration::new(interval, 0));
    }
}


fn network_bandwidth(tx: mpsc::Sender<ValueType>, bpf: Option<String>, max_bandwidth: ValueType) {
    let mut cap = Device::lookup().unwrap().open().unwrap();

    if let Some(filter) = bpf {
        cap.filter(filter.as_str()).unwrap();
    }

    while let Ok(packet) = cap.next() {
        let bytes = scale(packet.header.caplen as ValueType, 0, max_bandwidth);
        tx.send(bytes).unwrap();
    }
}


fn uview_sender(rx: mpsc::Receiver<ValueType>, serial_port: &mut SerialPort,
                interval: u64, display_mode: DisplayMode) {

    let mut timer = SystemTime::now();
    let mut accumulator = UViewPacket::new(0 as ValueType, display_mode);
    loop {
        accumulator += match rx.try_recv() {
            Ok(val) => val,
            Err(_) => 0 as ValueType
        };
        if let Ok(duration) = SystemTime::now().duration_since(timer) {
            if duration.as_secs() >= interval {
                println!("{:?}", accumulator);
                if let DisplayMode::Chart = display_mode {
                    accumulator.scale(0, 64);
                }
                serial_port.write((accumulator.to_string() + "\n").as_bytes()).unwrap();
                accumulator.zero();
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
                 <port> -p, --port=[port] 'MicroView serial port'
                 [mode-font] -f, --font 'Display in font mode'
                 [percentage] -x, --percentage 'Display percentages'")
            .subcommand(SubCommand::with_name("network")
                .about("Network monitoring")
                .args_from_usage("[filter] -f, --filter=[filter] 'Berkeley Packet Filter to use'")
                .subcommand(SubCommand::with_name("bandwidth")
                    .about("Network bandwidth monitoring")
                    .args_from_usage("<max> -m, --max=[max] 'Max bandwidth'")))
            .subcommand(SubCommand::with_name("memory")
                .about("Memory monitoring")
                .subcommand(SubCommand::with_name("free")
                    .about("Free RAM monitoring")))
            .get_matches();

    let interval: u64 = matches.value_of("interval").unwrap_or("1").parse().unwrap();
    let port = matches.value_of("port").unwrap();
    let display_mode = if matches.is_present("mode-font") {
        DisplayMode::Font
    } else {
        DisplayMode::Chart
    };
    let percentage = matches.is_present("percentage");

    let mut serial_port = serial::open(port).unwrap();
    let mut serial_settings = PortSettings::default();
    serial_settings.set_baud_rate(serial::Baud115200).unwrap();
    serial_port.configure(&mut serial_settings).unwrap();

    let (tx, rx) = mpsc::channel();
    let start_sender = || { uview_sender(rx, &mut serial_port, interval, display_mode) };

    if let Some(network_matches) = matches.subcommand_matches("network") {
        let bpf = network_matches.value_of("filter").map(|f| f.to_string());
        if let Some(bandwidth_matches) = network_matches.subcommand_matches("bandwidth") {
            let max_bandwidth = bandwidth_matches.value_of("max").unwrap().parse().unwrap();
            thread::spawn(move || network_bandwidth(tx, bpf, max_bandwidth));
            start_sender();
        }
    } else if let Some(memory_matches) = matches.subcommand_matches("memory") {
        if let Some(_) = memory_matches.subcommand_matches("free") {
            thread::spawn(move ||
                memory_free(tx, interval, percentage || display_mode == DisplayMode::Chart));
            start_sender();
        }
    }

    println!("Bye-Bye!");
}
