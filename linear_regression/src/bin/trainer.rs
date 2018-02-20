#[macro_use]
extern crate clap;
extern crate colored;
extern crate linear_regression;

use linear_regression::*;
use clap::{Arg, App};
use colored::*;
use std::process;

fn main() -> () {
    let opt = App::new("Linear regression - Predict")
        .version("0.1.0")
        .author("William Escande <wescande@student.42.fr")
        .about("Return a price in euro of car value, estimated from current data model")
        .arg(Arg::with_name("iteration").short("i").long("iteration").default_value("500").takes_value(true).help("Number of iteration for gradient descent"))
        .arg(Arg::with_name("reuse").short("r").long("reuse").help("If set, use current data model as init point"))
        .arg(Arg::with_name("long").short("L").long("long").help("If set, will set values treat in a [0 - 1] range"))
        .arg(Arg::with_name("learningrate").short("l").long("learningRate").default_value("0.1").takes_value(true).help("Set learning coeficient ] 0. - 1. ["))
        .arg(Arg::with_name("csvfile").short("f").long("file").default_value("data.csv").takes_value(true).help("csv file to be parsed"))
        .get_matches();
    let learning_rate = value_t!(opt.value_of("learningrate"), f64).unwrap_or_else(|e| e.exit());
    let iteration = value_t!(opt.value_of("iteration"), u64).unwrap_or_else(|e| e.exit());
    let mut descent = match Descent::new(
        opt.value_of("csvfile").unwrap(),
        opt.is_present("long")
        ) {
        Ok(o) => o,
        Err(e) => {println!("{}", format!("{}", e).red()); process::exit(0);},
    };
    if !opt.is_present("reuse") {
        descent.clear();
    }
    let gradient = descent.descent(iteration, learning_rate);
    gradient.save();
}
