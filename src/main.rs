#[macro_use]
extern crate clap;
use clap::App;
use rusoto_core::credential::*;
use rusoto_core::region::Region;
use rusoto_sqs::*;
use rusoto_core::request::HttpClient;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let chain_provider = if matches.is_present("profile") {
		let profile = matches.value_of("profile").expect("This will always work!");

    	let mut profile_provider = ProfileProvider::new().expect("So reliable");
    	profile_provider.set_profile(profile);

        ChainProvider::with_profile_provider(profile_provider)
    } else {
        ChainProvider::new()
    };

    let client = HttpClient::new().unwrap();

    let sqs_client = if matches.is_present("region") {
    	let region_string = matches.value_of("region").expect("This will always be present");
    	let region = region_string.parse::<Region>().expect("This will always, always work");

        SqsClient::new_with(client, chain_provider, region)
    } else {
        SqsClient::new_with(client, chain_provider, Region::default())
    };

    match matches.subcommand() {
    	("list", Some(_sub_args)) => {

    		let request = ListQueuesRequest {
    			..Default::default()
    		};

    		let result = sqs_client.list_queues(request).sync().unwrap().queue_urls.unwrap();
    		print!("{:?}", result);
    		 }
    	,
    	_ => {}
    }
}
