use color_eyre::eyre::Result;
use std::env;
use std::io::{self, Write};
use std::net::{IpAddr, TcpStream};
use std::process;
use std::str::FromStr;
use std::sync::mpsc::{channel, Sender};
use std::thread;

const MAX: u16 = 65535;

struct Arguments {
	ipaddr: IpAddr,
	threads: u16,
}

impl Arguments {
	fn new(args: &[String]) -> Result<Arguments, &'static str> {
		if args.len() < 2 || args.len() > 4 {
			return Err("help");
		}

		let flag = args[1].clone();

		if let Ok(ipaddr) = IpAddr::from_str(&flag) {
			return Ok(Arguments { ipaddr, threads: 4 });
		} else {
			if flag.contains("-h") || flag.contains("--help") {
				return Err("help");
			} else if flag.contains("-j") || flag.contains("--threads") {
				let ipaddr = match IpAddr::from_str(&args[3]) {
					Ok(s) => s,
					Err(_) => return Err("Not a valid IP Address"),
				};
				let threads = match args[2].parse::<u16>() {
					Ok(s) => s,
					Err(_) => return Err("failed to parse thread number"),
				};
				return Ok(Arguments { ipaddr, threads });
			} else {
				return Err("invalid syntax");
			}
		}
	}
}

fn print_help() {
	println!(
		"Usage: port-sniffer [options] ip\n\n\t(-j|--threads) <num>\tset the number of threads (default: 4)\n\n(-h|--help) to print this help menu"
	);
}

fn scan(tx: Sender<u16>, start_port: u16, addr: IpAddr, num_threads: u16) {
	let mut port: u16 = start_port + 1;
	loop {
		match TcpStream::connect((addr, port)) {
			Ok(_) => {
				print!(".");
				io::stdout().flush().unwrap();
				tx.send(port).unwrap();
			}
			Err(_) => {}
		}

		if (MAX - port) <= num_threads {
			break;
		}
		port += num_threads;
	}
}

fn main() -> Result<()> {
	color_eyre::install()?;

	let args: Vec<String> = env::args().collect(); // first element is the executable
	let program = args[0].clone();
	let arguments = Arguments::new(&args).unwrap_or_else(|err| {
		if err.contains("help") {
			print_help();
			process::exit(0);
		} else {
			eprintln!("{} problem parsing arguments: {}", program, err);
			process::exit(0);
		}
	});

	let num_threads = arguments.threads;
	let addr = arguments.ipaddr;
	let (tx, rx) = channel();
	for i in 0..num_threads {
		let tx = tx.clone();

		thread::spawn(move || {
			scan(tx, i, addr, num_threads);
		});
	}

	let mut out = vec![];
	drop(tx);

	for p in rx {
		out.push(p);
	}

	println!("");
	out.sort();
	for v in out {
		println!("{} is open", v);
	}

	Ok(())
}
