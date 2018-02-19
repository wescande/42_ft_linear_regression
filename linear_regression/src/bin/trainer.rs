#[macro_use]
extern crate clap;
extern crate indicatif;
extern crate linear_regression;
extern crate colored;

use colored::*;
use linear_regression::*;
use indicatif::{ProgressBar, ProgressStyle};
use std::process;
use clap::{Arg, App};

fn create_progress_bar(msg: &str, length: u64) -> ProgressBar {
    let bar = match length {
        0 => ProgressBar::new_spinner(),
        _ => ProgressBar::new(length),
    };
    bar.set_message(msg);
    match length {
        0 => bar.set_style(ProgressStyle::default_spinner()),
        _ => bar
            .set_style(ProgressStyle::default_bar()
                       .template("{msg} {spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} eta: {eta}")
                       .progress_chars("=> ")),
    };
    bar
}

fn main() -> () {
    let opt = App::new("Linear regression - Predict")
        .version("0.1.0")
        .author("William Escande <wescande@student.42.fr")
        .about("Return a price in euro of car value, estimated from current data model")
        .arg(Arg::with_name("iteration").short("i").long("iteration").default_value("500").takes_value(true).help("Number of iteration for gradient descent"))
        .arg(Arg::with_name("reuse").short("r").long("reuse").help("If set, use current data model as init point"))
        .arg(Arg::with_name("learningrate").short("l").long("learningRate").default_value("0.2").takes_value(true).help("Set learning coeficient ] 0. - 1. ["))
        .arg(Arg::with_name("csvfile").short("f").long("file").default_value("data.csv").takes_value(true).help("csv file to be parsed"))
        // .arg(Arg::with_name("error").short("e").long("error").help("If set, will print the error evolution of data model"))//TODO
        .get_matches();
    let mut gradient = Gradient::new();
    if !opt.is_present("reuse") {
        gradient.clear();
    }
    let learning_rate = value_t!(opt.value_of("learningrate"), f64).unwrap_or_else(|e| e.exit());
    // if learning_rate <= 0. || learning_rate >= 1. {
    //     println!("{}", format!("ERROR: {}: Not a valid learning coefficient rate. It must be in range ] 0. - 1. [\nWe will use default value (0.2) instead.", learning_rate).red());
    // }
    let iteration = value_t!(opt.value_of("iteration"), u64).unwrap_or_else(|e| e.exit());
    let price_vec = match fill_vec_from_csv(opt.value_of("csvfile").unwrap()) {
        Ok(o) => o,
        Err(err) => {println!("{}", format!("{}: Failed to parse csv file: {}", opt.value_of("csvfile").unwrap(), err).red());
            process::exit(1)}
    };
    let bar = create_progress_bar("Descente de gradient", iteration);
    for _ in 0..iteration {
        bar.inc(1);
        gradient.descent(&price_vec, learning_rate);
    }
    bar.finish();
    println!("Got learning_rate {} | iteration {}", learning_rate, iteration);
    gradient.save();
}
