mod args;

use args::{InspectionCommand, TopLevelInspection};
use local_ip_address::{list_afinet_netifas, local_ip};

fn main() {
  let command: TopLevelInspection = argh::from_env();

  match command.subcommand {
    InspectionCommand::InIp(option) => {
      if option.detailed {
        let network_interfaces = list_afinet_netifas().unwrap();

        for (name, ip) in network_interfaces.iter() {
          println!("{}:\t{:?}", name, ip);
        }
      } else {
        let my_local_ip = local_ip().unwrap();
        println!("{}", my_local_ip);
      }
    }
  }
}
