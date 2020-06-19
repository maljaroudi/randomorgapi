use serde_json::json;
use clap::{App,Arg};
use nanoid::nanoid;
use ureq;
pub struct GenerateIntegers{
    api_key: String,
    n: u16,
    min: i32,
    max: i32,
}

fn main() {
    let url = String::from("https://api.random.org/json-rpc/2/invoke");

    let args = App::new("Random.org API")
        .version("0.5")
        .about("Yet Another Random Number Generator using random.org api implementation")
        .arg(Arg::with_name("api")
        .help("The number of random integers you want")
        .takes_value(true)
        .required(true))
        .arg(Arg::with_name("n")
        .help("The number of random integers you want")
        .takes_value(true)
        .required(true))
        .arg(Arg::with_name("min")
        .help("The minimum number 0-1e9")
        .takes_value(true)
        .required(true))
        .arg(Arg::with_name("max")
        .help("The maximum number 0-1e9")
        .takes_value(true)
        .required(true))
        .get_matches();
    let api_values = GenerateIntegers {
        api_key: args.value_of("api").unwrap().to_owned().to_string(),
        n: args.value_of("n").unwrap().trim().parse().expect("problem with your number of integers"),
        min: args.value_of("min").unwrap().trim().parse().expect("problem with your minimum"),
        max: args.value_of("max").unwrap().trim().parse().expect("problem with your maximum"),
    };

    let value = call_random(api_values, url);
     println!("{}",value)

}

fn call_random(api_vals: GenerateIntegers, url: String) -> String {
    let data = json!({
        "jsonrpc": "2.0",
        "method": "generateIntegers",
        "id": nanoid!(),
        "params": {
        "apiKey": api_vals.api_key.as_str(),
        "n": api_vals.n,
        "min": api_vals.min,
        "max": api_vals.max
        }
    });
    let result= ureq::post(&url).send_json(data).into_string().unwrap();
    let start_bytes = result.find(r#""data":"#).unwrap_or(0);
    let end_bytes = result.find(r#","completionTime""#).unwrap_or(result.len());
    let resultf = &result[start_bytes+7..end_bytes];
    resultf.to_owned()
}


