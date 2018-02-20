#[macro_use]
extern crate clap;
extern crate linear_regression;
extern crate colored;
use colored::*;
use linear_regression::*;
use std::io::Write;
use std::io::stdout;
use clap::{Arg, App};

fn main() -> () {
    let opt = App::new("Linear regression - Predict")
        .version("0.1.0")
        .author("William Escande <wescande@student.42.fr")
        .about("Return a price in euro of car value, estimated from current data model")
        .arg(Arg::with_name("mileage")
             .short("m")
             .long("mileage")
             .takes_value(true)
             .help("Car mileage in km"))
        .get_matches();
    let mileage = {
        if opt.is_present("mileage") {
            value_t!(opt.value_of("mileage"), f64).unwrap_or_else(|e| e.exit())
        }
        else {
            print!("{}\n{}","Please enter a mileage:".yellow(), " ".purple());
            stdout().flush().unwrap();
            get_user_mileage()
        }
    };
    let gradient = Gradient::new();
    println!("{}",  format!("A {} mileage car worth {:.2} euros", mileage, gradient.estimate_price(mileage)).green());
}
