mod args;

use cli_clipboard;

use args::{InspectionCommand, TopLevelInspection};
use local_ip_address::{list_afinet_netifas, local_ip};

fn main() {
  let command: TopLevelInspection = argh::from_env();

  match command.subcommand {
    InspectionCommand::InIp(options) => {
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
    InspectionCommand::CopyFile(options) => {
      // check if option.src exists
      if !std::path::Path::new(&options.file).exists() {
        eprintln!("File {} does not exist", options.file);
        std::process::exit(1);
      }
      let content = std::fs::read_to_string(&options.file).expect("read file");
      cli_clipboard::set_contents(content.to_owned()).expect("write to clipboard");
      println!("Copiled {} characters to clipboard", content.chars().count());
    }
  }
}
