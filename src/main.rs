#[macro_use]
extern crate clap;
use clap::App;
use rusoto_core::credential::*;
use rusoto_core::region::Region;
use rusoto_sqs::*;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let chain_provider = if matches.is_present("profile") {
        ChainProvider::with_profile_provider(ProfileProvider::with_configuration("",matches.value_of("profile").expect("This will always work!")))
    } else {
        ChainProvider::new()
    };

    let sqs_client = if matches.is_present("region") {
        SqsClient::new(Region::from_str(matches.value_of("region").expect("This will always be present")).expect("You've only gone and blown the bloody doors off"))
    } else {
        SqsClient::new
    };


    match matches.subcommand() {
    	("list", Some(_sub_args)) => println!("listing queues..."),
    	_ => {}
    }
}
