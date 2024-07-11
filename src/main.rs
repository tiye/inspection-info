mod args;
mod show_file_size;

use sysinfo::System;

use bytesize::ByteSize;
use cli_clipboard;

use args::{InspectionCommand, TopLevelInspection};
use local_ip_address::{list_afinet_netifas, local_ip};

fn main() -> Result<(), String> {
  let command: TopLevelInspection = argh::from_env();

  use InspectionCommand::*;

  match command.subcommand {
    InIp(options) => {
      if options.detailed {
        let network_interfaces = list_afinet_netifas().unwrap();

        for (name, ip) in network_interfaces.iter() {
          println!("{}:\t{:?}", name, ip);
        }
      } else {
        let my_local_ip = local_ip().unwrap();
        println!("{}", my_local_ip);
      }
    }
    CopyFile(options) => {
      // check if option.src exists
      if !std::path::Path::new(&options.file).exists() {
        eprintln!("File {} does not exist", options.file);
        std::process::exit(1);
      }
      let content = std::fs::read_to_string(&options.file).expect("read file");
      cli_clipboard::set_contents(content.to_owned()).expect("write to clipboard");
      println!("Copiled {} characters to clipboard", content.chars().count());
    }
    ShowMemory(_) => {
      let mut sys = System::new_all();

      // First we update all information of our `System` struct.
      sys.refresh_all();

      println!("System:\n");
      // RAM and swap information:
      println!("total memory: {} bytes", ByteSize(sys.total_memory()));
      println!(" used memory: {} bytes", ByteSize(sys.used_memory()));
      println!("total swap  : {} bytes", ByteSize(sys.total_swap()));
      println!(" used swap  : {} bytes", ByteSize(sys.used_swap()));
    }
    ShowProcesses(_) => {
      let mut sys = System::new_all();

      // First we update all information of our `System` struct.
      sys.refresh_all();

      for (pid, process) in sys.processes() {
        println!("{}\t#{pid}", process.name());
        if let Some(v) = process.cwd() {
          print!("\t{:?}", v);
        }
        if let Some(v) = process.user_id() {
          print!("\t{:?}", v);
        }
        print!("\n");
        // println!("    {}", process.cmd().join(" "));
      }
    }
    ShowWorkingDirectory(_) => {
      let cwd = std::env::current_dir().expect("get current working directory");
      let dir = cwd.display().to_string();
      cli_clipboard::set_contents(dir.to_owned()).expect("write to clipboard");
      println!("{}\t\t(copied to clipboard)", dir);
    }
    ListFileSize(options) => {
      show_file_size::show_file_size(options)?;
    }
  }

  Ok(())
}
