use regex::Regex;
use std::fs;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opts {
    #[structopt(short="f", long="file", default_value = "data.txt")]
    file: String,
}
fn main() {
    let opts = Opts::from_args();
    println!("{:?}", opts);
    let file = opts.file;

    let re = Regex::new(r"(\d{4})-(\d{2})-(\d{2})").unwrap();

    let text = fs::read_to_string(file)
        .expect("Something went wrong reading the file");

    for cap in re.captures_iter(&text) {
        println!("Month: {} Day: {} Year: {}", &cap[2], &cap[3], &cap[1]);
    }
}
