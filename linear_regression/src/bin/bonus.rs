extern crate linear_regression;
extern crate colored;
extern crate clap;
extern crate gnuplot;
extern crate console;

use colored::*;
use linear_regression::*;

use clap::{Arg, App};
use gnuplot::{Figure, Caption, Color};
use std::io;
use std::process;
use console::Term;
// use console::Term::Write; // <--- bring the trait into scope
use std::io::Write; // <--- bring the trait into scope
use std::io::stdout;


pub struct Cli {
    term: Term,
    // command: String,
    // history: Vec<String>,
}

impl Cli {
    pub fn new() -> io::Result<Cli> {
        let term = Term::stdout();
        if !term.is_term() {
            return Err(io::Error::new(io::ErrorKind::BrokenPipe, "Cannot init terminal"))
        }
        Ok(Cli {
            term: term
            // command: String::new(),
            // history: Vec::new(),
        })
    }

    fn prompt(&mut self) -> io::Result<usize>{
        // self.term.write();

        let write = self.term.write(" ".as_bytes());
        io::stdout().flush().unwrap();
        write
    }

    fn read(&mut self) -> io::Result<String>{
        self.prompt()?;
        let mut command = String::new();
        // self.command.clear();
        match io::stdin().read_line(&mut command) {
                Ok(0) => return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "End of file")),
                Ok(_) => (),
                Err(e) => return Err(e)
        }
        match command.pop() {
            Some('\n') => (),
            Some(e) => command.push(e),
            None => return Err(io::Error::new(io::ErrorKind::InvalidInput, "No char in command")),
        }
        Ok(command)
    }
}
fn usage() {
    println!("{}", format!("Following command are available:
    quit | exit | q     => quit the sub shell
    help | h            => display this help
    train               => train algo with standard values
    show                => print the graph of data and gradient values
    regression | error  => display error regression history (if train was called before)
    save                => save current gradient couple
    clear               => clear gradient
    price               => get price of a car
    ").yellow());
}

fn show(descent: &Descent, fg: &mut Figure) {
    let data = descent.get_data();
    let gradient = descent.get_gradient();
    let x2 = [0., data.x.iter().cloned().fold(0./0., f64::max)];
    let y2 = [gradient.estimate_price(x2[0]), gradient.estimate_price(x2[1])];
    fg.clear_axes();
    fg.axes2d()
        .points(&data.x, &data.y, &[Caption("Points"), Color("red")])
        .lines(&x2, &y2, &[Caption("A line"), Color("black")]);
    fg.show();
}
fn train(descent: &mut Descent) {
    descent.descent(500, 0.1);
}

fn regression(descent: &Descent, fg: &mut Figure) {
    let errors = descent.get_error_history();
    if errors.len() == 0 {
        println!("{}", "There is no history yet. Please run `Train` to init a gradient descent.".red());
        return;
    }
    let mut xerrors: Vec<f64> = Vec::new();
    let mut i = 0;
    for _ in &errors {
        xerrors.push(i as f64);
        i += 1;
    }
    fg.clear_axes();
    fg.axes2d()
        .lines(&xerrors, &errors, &[Caption("Error History"), Color("blue")]);
    fg.show();
}

fn print_gradient(descent: &Descent) {
    let gradient = descent.get_gradient();
    println!("{}", gradient);
}
fn save(descent: &Descent) {
    let gradient = descent.get_gradient();
    gradient.save();
}
fn price(descent: &Descent){
    print!("{}\n{}","Please enter a mileage:".yellow(), " ".purple());
    stdout().flush().unwrap();
    let mileage = get_user_mileage();
    let gradient = descent.get_gradient();
    println!("{}",  format!("A {} mileage car worth {:.2} euros", mileage, gradient.estimate_price(mileage)).green());
}

fn analyze_command(buf: String, descent: &mut Descent, fg0: &mut Figure, fg1: &mut Figure) {
    let res: Vec<String> = buf.split_whitespace().map(|s| s.to_string()).collect();
    if res.len() == 0 {
        return;
    }
    match res[0].as_str() {
        "quit" | "exit" | "q" => process::exit(0),
        "help" | "h" => usage(),
        "train" => train(descent),
        "show" => show(&descent, fg0),
        "regression" | "error" => regression(&descent, fg1),
        "save" => save(&descent),
        "gradient" => print_gradient(&descent),
        "clear" => descent.clear(),
        "price" => price(&descent),
        cmd => {println!("{}", format!("{}: Command unknown", cmd).red()); usage();},
    }
}

fn main_linear_regression(filename: &str) -> Result<i32, io::Error>{
    let mut cli = Cli::new()?;
    let mut descent = Descent::new(
        filename,
        true
        ).unwrap();
    let mut fg0: Figure= Figure::new();
    let mut fg1: Figure= Figure::new();
    loop {
        match cli.read() {
            Ok(buf) => if buf.len() > 0 {analyze_command(buf, &mut descent,  &mut fg0, &mut fg1);},//println!("GOT command [{}]", buf),
            Err(e) => return Err(e),
        }
    }
}

fn main() -> () {
    // let matches = App::new("Linear regression")
    let opt = App::new("Linear regression")
        .version("0.1.0")
        .author("William Escande <wescande@student.42.fr")
        .about("An introduction to machine learning")
        .arg(Arg::with_name("csvfile").short("f").long("file").default_value("data.csv").takes_value(true).help("csv file to be parsed"))
        .get_matches();
    if let Err(e) = main_linear_regression(opt.value_of("csvfile").unwrap()) {
        println!("Cli as catch an error: {}", e);
    }
    // return;
    // let mut descent = Descent::new(
    //     opt.value_of("csvfile").unwrap(),
    //     true
    //     ).unwrap();
    // if !opt.is_present("reuse") {
    //     descent.clear();
    // }
    // let gradient = descent.descent(500, 0.1);
    // gradient.save();
    //TODO welcome message
    // let data:Data = match fill_vec_from_csv(opt.value_of("csvfile").unwrap()) {
    //     Ok(o) => o,
    //     Err(err) => {println!("{}", format!("{}: Failed to parse csv file: {}", opt.value_of("csvfile").unwrap(), err).red());
    //         process::exit(1)}
    // };
    // let errors = descent.get_error_history();
    // let mut xerrors: Vec<f64> = Vec::new();
    // let mut i = 0;
    // for _ in &errors {
    //     xerrors.push(i as f64);
    //     i += 1;
    // }
    // let (data, x, y) = convert_csv(data);
    // let mut x: Vec<f64> = Vec::new();
    // let mut y: Vec<f64> = Vec::new();
    // for price in price_vec {
    //     x.push(price.km);
    //     y.push(price.price);
    // }

    // let gradient = Gradient::new();
    // let x2 = [0., data.x.iter().cloned().fold(0./0., f64::max)];
    // let y2 = [gradient.estimate_price(x2[0]), gradient.estimate_price(x2[1])];
    // if let Err(e) = main_linear_regression() {
    //     println!("{}", e);
    //     return ;
    // }
    //TODO goodby message
    // prompt("");
    // let bar = create_progress_bar("Hello", 50);
    // for _ in 0..50 {
    //     thread::sleep(time::Duration::from_millis(100));
    //     bar.inc(1);
    // }
    // bar.finish();
    //TODO parse args
    // let x = [0u32, 1, 2];
    // let y = [3u32, 4, 5];
    // let mut fg = Figure::new();
    // // fg.set_terminal("wxt", "Hello from here");
    // fg.axes2d()
    //     .points(&data.x, &data.y, &[Caption("Points"), Color("red")])
    //     .lines(&x2, &y2, &[Caption("A line"), Color("black")]);
    // let mut fgerr = Figure::new();
    // fgerr.axes2d()
    //     .lines(&xerrors, &errors, &[Caption("Error History"), Color("blue")]);
    // // fg.show();
    // fgerr.show();
    // loop{};

    // return ;
}
