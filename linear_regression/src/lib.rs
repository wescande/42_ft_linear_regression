#[macro_use]
extern crate serde_derive;
extern crate bincode;
extern crate csv;

use std::error::Error;
use std::fs::File;
use std::fmt;
use bincode::{serialize_into, deserialize_from};

#[derive(Deserialize)]
pub struct RealPrice {
    km: f64,
    price: f64,
}

pub fn fill_vec_from_csv(filename:&str) -> Result<Vec<RealPrice>, Box<Error>> {
    let mut data_vec: Vec<RealPrice> = Vec::new();
    let file = File::open(filename)?;
    let mut rdr = csv::Reader::from_reader(file);
    for result in rdr.records() {
        let row: RealPrice = result?.deserialize(None)?;
        data_vec.push(row);
    }
    Ok(data_vec)
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
    pub fn set(&mut self, a: f64, b: f64) {
        self.a = a;
        self.b = b;
    }
    pub fn clear(&mut self) {
        self.set(0., 0.);
    }
    fn load() -> std::io::Result<Gradient> {
        let mut file = File::open("gradient.bin")?;
        Ok(deserialize_from(&mut file).unwrap())
    }
    pub fn save(&self) {
        let mut file = File::create("gradient.bin").unwrap();
        serialize_into(&mut file, &self).unwrap();
    }
    pub fn estimate_price(&self, mileage: f64) -> f64 {
        mileage * self.a + self.b
    }
    pub fn descent(&mut self, price_vec: &Vec<RealPrice>, learning_rate: f64) {
        let tmp = Gradient {a: self.a, b: self.b};
        let mut a: f64 = 0.;
        let mut b: f64 = 0.;
        let len: f64 = price_vec.len() as f64;
        for price in price_vec{
            let error:f64 = tmp.estimate_price(price.km) - price.price;
            b += error;
            a += error * price.km;
        }
        println!("{}", self);
        self.a -= learning_rate * (a / len);
        self.b -= learning_rate * (b / len);
    }
}
impl fmt::Display for Gradient {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Gradient is: {}x + {}", self.a, self.b)
    }
}

pub struct Descent {
    gradient: Gradient,
    error: f64,
}
impl Descent {
    pub fn new(gradient: Gradient) -> Descent {
        Descent {
            gradient: gradient,
            error: 0.,
        }
    }
    pub fn descent() -> Result<(), &'static str> {
        Ok()
    }
}
