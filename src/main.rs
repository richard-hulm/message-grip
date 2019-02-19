#[macro_use]
extern crate clap;
use clap::App;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    match matches.subcommand() {
    	("list", Some(_sub_args)) => println!("listing queues..."),
    	_ => panic!("Unexpected command")
    }
}
