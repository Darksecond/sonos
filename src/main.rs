extern crate sonos;
extern crate clap;

use clap::{App, Arg, SubCommand};

//TODO use enum for property

fn main() {
    let matches = App::new("sonos")
        .version("0.1")
        .author("Tim Peters <mail@darksecond.nl")
        .about("Control sonos devices via the command line")
        .subcommand(SubCommand::with_name("list")
            .about("List devices"))
        .subcommand(SubCommand::with_name("info")
            .about("Get current properties")
            .arg(Arg::with_name("device")
                .required(true)
                .help("Name of device")))
        .subcommand(SubCommand::with_name("set")
            .about("Set property")
            .arg(Arg::with_name("PROPERTY")
                .required(true)
                .index(1))
            .arg(Arg::with_name("VALUE")
                .required(true)
                .index(2))
            .arg(Arg::with_name("device")
                .takes_value(true)
                .short("d")
                .long("device")
                .required(true)
                .help("Name of device")))
        .get_matches();

    if let Some(_matches) = matches.subcommand_matches("list") {
        let devices = sonos::discover().unwrap();
        for ref device in devices.iter() {
            println!("{}", device.name);
        }
    }

    if let Some(matches) = matches.subcommand_matches("info") {
        let name = matches.value_of("device").unwrap();
        let devices = sonos::discover().unwrap();
        let device = devices.iter().find(|d| d.name == name).expect("Could not find device");
        println!("{}% Volume", device.volume().unwrap());   
    }

    if let Some(matches) = matches.subcommand_matches("set") {
        let property = matches.value_of("PROPERTY").unwrap();
        let value = matches.value_of("VALUE").unwrap();
        let name = matches.value_of("device").unwrap();
        let devices = sonos::discover().unwrap();
        let device = devices.iter().find(|d| d.name == name).expect("Could not find device");
        match property {
            "volume" => {
                let value = value.parse().expect("Not a number");
                device.set_volume(value).unwrap();
                println!("Set volume to {}%", value)
            }
            _ => panic!("Not a valid property")
        }
    }
}
