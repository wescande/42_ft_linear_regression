// extern crate gnuplot;
extern crate clap;
extern crate indicatif;
extern crate console;

use clap::{Arg, App};
// use gnuplot::{Figure, Caption, Color};
use indicatif::{ProgressBar, ProgressStyle};
use std::{thread, time, io};
use console::Term;
// use console::Term::Write; // <--- bring the trait into scope
use std::io::Write; // <--- bring the trait into scope


// use cli::*;

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
        self.term.write(" ".as_bytes())
        // io::stdout().flush();
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

fn main_linear_regression() -> Result<i32, io::Error>{
    let mut cli = Cli::new()?;

    loop {
        match cli.read() {
            Ok(buf) => println!("GOT command [{}]", buf),
            Err(e) => return Err(e),
        }


        // let mut buf = String::new();
        // match stdin.read_line(&mut buf) {
        //         Ok(0) => break,
        //         Ok(_) => println!("I just read {} from len {}", buf, buf.len()),
        //         Err(_) => break,
        // }

//         let toto : String = read!("{}\n");
//         println!("I just read {} from len {}", toto, toto.len());
    }
}

fn main() -> () {
    // let matches = App::new("Linear regression")
    App::new("Linear regression")
        .version("0.1.0")
        .author("William Escande <wescande@student.42.fr")
        .about("An introduction to machine learning")
        .arg(Arg::with_name("URL")
             .takes_value(true)
             .index(1)
             .help("url to download"))
        .get_matches();
    //TODO welcome message
    if let Err(e) = main_linear_regression() {
        println!("{}", e);
        return ;
    }
    //TODO goodby message
    // prompt("");
    let bar = create_progress_bar("Hello", 50);
    for _ in 0..50 {
        thread::sleep(time::Duration::from_millis(100));
        bar.inc(1);
    }
    bar.finish();
    //TODO parse args
    // let x = [0u32, 1, 2];
    // let y = [3u32, 4, 5];
    // let mut fg = Figure::new();
    // // fg.set_terminal("wxt", "Hello from here");
    // fg.axes2d()
    //     .lines(&x, &y, &[Caption("A line"), Color("black")]);
    // fg.show();
    // loop{};

    return ;
}
