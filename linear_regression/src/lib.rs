#[macro_use]
extern crate serde_derive;
extern crate bincode;
extern crate indicatif;
extern crate csv;
extern crate colored;

use std::error::Error;
use std::fs::File;
use std::fmt;
use indicatif::{ProgressBar, ProgressStyle};
use bincode::{serialize_into, deserialize_from};
use colored::*;
use std::process;

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
                       .template("{msg} {spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}/{len} eta: {eta}")
                       .progress_chars("=> ")),
    };
    bar
}
pub fn get_user_mileage() -> f64 {
    let mut input_text = String::new();
    std::io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");

    let trimmed = input_text.trim();
    match trimmed.parse::<f64>() {
        Ok(i) => return i,
        Err(_) => (),
    };
    println!("{}", format!("ERROR: {}: Not a valid float value. We will use 0. instead.", trimmed).red());
    return 0.;
}

#[derive(Deserialize)]
pub struct RealPrice {
    pub km: f64,
    pub price: f64,
}

pub struct Data {
    pub x: Vec<f64>,
    pub y: Vec<f64>,
    len: usize,
}

impl Data {
    pub fn new() -> Data {
        Data {
            x: Vec::new(),
            y: Vec::new(),
            len: 0,
        }
    }
}

pub fn fill_vec_from_csv(filename:&str) -> Result<Data, Box<Error>> {
    let mut data: Data = Data::new();
    let file = File::open(filename)?;
    let mut rdr = csv::Reader::from_reader(file);
    for result in rdr.records() {
        let row: RealPrice = result?.deserialize(None)?;
        data.x.push(row.km);
        data.y.push(row.price);
        data.len += 1;
    }
    Ok(data)
}

pub struct Convert {
    min: f64,
    ratio: f64,
}

fn convert_vec(vec: &Vec<f64>) -> (Vec<f64>, Convert) {
    let max:f64 = vec.iter().cloned().fold(0./0., f64::max);
    let min:f64 = vec.iter().cloned().fold(0./0., f64::min);
    let ratio:f64 = {
        if max == min {
            1.
        }
        else {
            max - min
        }};
    let convert: Convert = Convert {min: min, ratio: ratio};
    let result: Vec<f64> = vec.iter().map(|val| { (val - min) / ratio }).collect::<Vec<_>>();
    (result, convert)
}

pub fn convert_csv(data: &Data) -> (Data, Convert, Convert){
    let (x, xconvert) = convert_vec(&data.x);
    let (y, yconvert) = convert_vec(&data.y);
    let data_convert = Data {
        x: x,
        y: y,
        len: data.len
    };
    (data_convert, xconvert, yconvert)
}

#[derive(Serialize, Deserialize)]
pub struct Gradient {
    a: f64,
    b: f64,
}

impl Gradient {
    pub fn new() -> Gradient {
        match Gradient::load() {
            Ok(y) => y,
            Err(_) => Gradient {a: 0.0, b: 0.0},
        }
    }
    pub fn new_long(long_regression: bool, xconv: &Convert, yconv: &Convert) -> Gradient {
        match Gradient::load() {
            Ok(mut y) => if long_regression {y.convert_in(xconv, yconv); y}else{y},
            Err(_) => Gradient {a: 0.0, b: -yconv.min / yconv.ratio},
        }
    }
    pub fn cloned(&self) -> Gradient {
        Gradient {
            a: self.a,
            b: self.b,
        }
    }
    pub fn set(&mut self, a: f64, b: f64) {
        self.a = a;
        self.b = b;
    }
    pub fn clear(&mut self) {
        self.set(0., 0.);
    }
    pub fn clear_long(&mut self, yconv: &Convert) {
        self.set(0., -yconv.min / yconv.ratio);
    }
    pub fn convert_in(&mut self, x: &Convert, y: &Convert) {
        let a = self.a * x.ratio / y.ratio;
        let b = (self.b - y.min + self.a * x.min) / y.ratio;
        self.set(a, b);
    }
    pub fn convert_out(&mut self, x: &Convert, y: &Convert) {
        let a = self.a * y.ratio / x.ratio;
        let b = self.b * y.ratio + y.min - a * x.min;
        self.set(a, b);
    }
    fn load() -> std::io::Result<Gradient> {
        let mut file = File::open("gradient.bin")?;
        Ok(deserialize_from(&mut file).unwrap())
    }
    pub fn save(&self) {
        let mut file = match File::create("gradient.bin") {
            Ok(o) => o,
            Err(e) => {println!("{}", format!("{}", e).red()); process::exit(0);},
        };
        serialize_into(&mut file, &self).unwrap();
    }
    pub fn estimate_price(&self, mileage: f64) -> f64 {
        mileage * self.a + self.b
    }
    pub fn calc_error(&self, data: &Data) -> f64 {
        let mut error: f64 = 0.;
        let mut index: usize = 0;
        while index < data.len {
            error += (self.estimate_price(data.x[index]) - data.y[index]).abs();
            index += 1;
        }
        error
    }
    pub fn descent(&mut self, data: &Data, learning_rate: f64) {
        let mut a: f64 = 0.;
        let mut b: f64 = 0.;
        let mut index: usize = 0;
        while index < data.len {
            let error: f64 = self.estimate_price(data.x[index]) - data.y[index];
            b += error;
            a += error * data.x[index];
            index += 1;
        }
        self.a -= learning_rate * (a / data.len as f64);
        self.b -= learning_rate * (b / data.len as f64);
    }
}

impl fmt::Display for Gradient {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Gradient is: {}x + {}", self.a, self.b)
    }
}

pub struct Descent {
    long_regression: bool,
    gradient: Gradient,
    error: f64,
    error_history: Vec<f64>,
    data: Data,
    data_convert: Data,
    x: Convert,
    y: Convert,
}

impl Descent {
    pub fn new(filename:&str, long_regression:bool) -> Result<Descent, Box<Error>> {
        let data = fill_vec_from_csv(filename)?;
        let (data_convert, x, y) = convert_csv(&data);
        let gradient = Gradient::new_long(long_regression, &x, &y);
        // if long_regression {
        //     gradient.convert_in(&x, &y);
        // }
        Ok(Descent {
            long_regression: long_regression,
            error: if long_regression {
                gradient.calc_error(&data_convert)
            } else {
                gradient.calc_error(&data)
            },
            error_history: Vec::new(),
            data: data,
            data_convert: data_convert,
            gradient: gradient,
            x: x,
            y: y,
        })
    }
    pub fn get_gradient(&self) -> Gradient {
        let mut gradient = self.gradient.cloned();
        if self.long_regression {
            gradient.convert_out(&self.x, &self.y);
        }
        gradient
    }
    pub fn get_data(&self) -> &Data {
            &self.data
    }
    fn get_cur_data(&self) -> &Data {
        if self.long_regression {
            &self.data_convert
        }
        else {
            &self.data
        }
    }
    pub fn calc_error(&self) -> f64 {
        self.gradient.calc_error(self.get_cur_data())
    }
    pub fn set_error(&mut self) {
        self.error = self.calc_error();
    }
    pub fn clear(&mut self) {
        if self.long_regression {
            self.gradient.clear_long(&self.y);
        }
        else {
            self.gradient.clear();
        }
        self.set_error();
    }
    pub fn descent_end(&mut self) -> Gradient {
        self.get_gradient()
        // if self.long_regression {
        //     self.gradient.convert_out(&self.x, &self.y);
        // }
        // self.gradient.cloned()
    }
    pub fn descent(&mut self, iteration:u64, learning_rate:f64) -> Gradient {
        let bar = create_progress_bar("Gradient descent algorithm", iteration);
        for i in 0..iteration {
            self.error_history.push(self.error);
            bar.inc(1);
            let mut gradient = self.gradient.cloned();
            gradient.descent(self.get_cur_data(), learning_rate);
            let error = self.calc_error();
            if error > self.error {
                bar.finish_and_clear();
                println!("{}", format!("We passed the minimal convergence point at iteration {} on {}. End of training", i, iteration).red());
                return self.descent_end();
            }
            self.error = error;
            self.gradient = gradient;
        }
        bar.finish();
        self.descent_end()
    }
    pub fn get_error_history(&self) -> Vec<f64> {
        self.error_history.clone()
    }
}
