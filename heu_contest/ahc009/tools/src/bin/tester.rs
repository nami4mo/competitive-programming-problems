use std::process::Stdio;
use std::io::prelude::*;
use tools::*;

const JUDGE: bool = false;

fn exec(command: &str, args: &Vec<String>) {
	let mut input = String::new();
	std::io::stdin().read_to_string(&mut input).unwrap();
	let input = parse_input(&input);
	let mut p = std::process::Command::new(command)
		.args(args)
		.stdin(Stdio::piped())
		.stdout(Stdio::piped())
		.spawn().unwrap_or_else(|e| { eprintln!("failed to execute the command"); eprintln!("{}", e); std::process::exit(1) } );
	let mut stdin = std::io::BufWriter::new(p.stdin.take().unwrap());
	let mut stdout = std::io::BufReader::new(p.stdout.take().unwrap());
	write!(stdin, "{}", input).unwrap();
	stdin.flush().unwrap();
	let mut sim = Sim::new(&input);
	for t in 1..=T {
		loop {
			let mut out = String::new();
			stdout.read_line(&mut out).unwrap_or_else(|_| {
				eprintln!("Your program has terminated unexpectedly on turn {}", t);
				eprintln!("Score = 0");
				std::process::exit(1)
			});
			let out = out.trim();
			if !JUDGE {
				println!("{}", out);
			}
			if out.starts_with("#") {
				continue;
			}
			if let Err(err) = sim.human_move(&out) {
				eprintln!("{}", err);
				eprintln!("Score = 0");
				if JUDGE {
					println!("WA");
					println!("0");
				}
				std::process::exit(0);
			}
			let pet_move = sim.pet_move();
			let _ = writeln!(stdin, "{}", pet_move);
			stdin.flush().unwrap();
			break;
		}
	}
	let score = sim.compute_score();
	eprintln!("Score = {}", score);
	if JUDGE {
		println!("AC");
		println!("{}", score);
	}
}

fn main() {
	if std::env::args().len() < 2 {
		eprintln!("Usage: {} <command> [<args>...]", std::env::args().nth(0).unwrap());
		return;
	}
	let command = std::env::args().nth(1).unwrap();
	let args = std::env::args().skip(2).collect::<Vec<_>>();
	exec(&command, &args);
}
