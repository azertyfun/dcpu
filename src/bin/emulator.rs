extern crate byteorder;
extern crate dcpu;
extern crate docopt;
#[macro_use]
extern crate log;
extern crate rustc_serialize;
extern crate simplelog;

#[macro_use]
mod utils;

use docopt::Docopt;

use dcpu::cpu::Cpu;
use dcpu::computer::Computer;

const USAGE: &'static str = "
Usage:
  emulator [(-d <device>)...] [<file>]
  emulator (--help | --version)

Options:
  <file>             The binary file to execute.
  -d, --device       Des super devices.
  <file>             File to use instead of stdin.
  -h, --help         Show this message.
  --version          Show the version of disassembler.
";

#[derive(Debug, RustcDecodable)]
struct Args {
    arg_device: Option<Vec<String>>,
    arg_file: Option<String>,
}

fn main() {
    simplelog::TermLogger::init(simplelog::LogLevelFilter::Info).unwrap();

    let args: Args = Docopt::new(USAGE)
                            .and_then(|d| d.decode())
                            .unwrap_or_else(|e| e.exit());

    let rom = {
        let input = utils::get_input(args.arg_file);
        let mut rom = Vec::new();
        rom.extend(utils::IterU16{input: input});
        rom
    };

    let mut cpu = Cpu::default();
    cpu.load(&rom, 0);

    let mut computer = Computer::new(cpu);

    loop {
        match computer.tick() {
            Ok(_) => (),
            Err(e) => {
                println!("{}", e);
                break;
            }
        }
    }
}
