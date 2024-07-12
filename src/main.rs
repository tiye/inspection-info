mod args;
mod dir_marks;
mod show_file_size;

use sysinfo::System;

use bytesize::ByteSize;
use cli_clipboard;

use args::{DirMarkCommand, InspectionCommand, TopLevelInspection};
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

    DirMark(options) => match options.subcommand {
      DirMarkCommand::Add(options) => {
        let wd = std::env::current_dir().map_err(|e| e.to_string())?;
        let path = wd.to_str().expect("convert to string");
        let mut marks = dir_marks::DirMarks::load().expect("load marks");
        marks.add(options.kwd, path, options.desc.unwrap_or_default());
        marks.save_and_write()?;
      }
      DirMarkCommand::Remove(options) => {
        let mut marks = dir_marks::DirMarks::load().expect("load marks");
        if options.by_path {
          marks.remove_by_path(&options.kwd);
        } else {
          marks.remove(&options.kwd);
        }
        marks.save_and_write()?;
      }
      DirMarkCommand::Search(options) => {
        let marks = dir_marks::DirMarks::load().expect("load marks");
        marks.list_all(options.query.as_deref());
      }
      DirMarkCommand::Jump(options) => {
        let mut marks = dir_marks::DirMarks::load().expect("load marks");
        marks.jump(&options.kwd)?;
      }
    },
  }

  Ok(())
}
