mod tinygp;
mod benchmark;
mod params;

#[allow(unused)]
use std::fs::{self, metadata, File};
#[allow(unused)]
use std::io::{self, Write};
use structopt::StructOpt;
#[allow(unused)]
use tinygp::TinyGP;

use crate::benchmark::run_benchmark;

#[derive(StructOpt, Debug)]
pub struct Args {
    #[structopt(short, long)]
    seed: Option<u64>,

    #[structopt(short, long, default_value = "100")]
    generations: usize,

    #[structopt(long,short="u")]
    suite: Option<String>,

    #[structopt(long,short="d")]
    stdout: bool,

    #[structopt(short, long)]
    fresh: bool,

    // #[structopt(short, long)]
    // output: Option<String>,

    // problempath: String,
}

fn main() {
    // env_logger::init();
    env_logger::Builder::from_default_env()
        .format(|buf, record| {
            writeln!(buf, "{}: {}", record.level(), record.args())
        })
        .init();

    // logging: set environment variable RUST_LOG to one of the levels
    // log::trace!("This is a trace message");
    // log::debug!("This is a debug message");
    // log::info!("This is an info message");
    // log::warn!("This is a warning message");
    // log::error!("This is an error message");

    let args = Args::from_args();
    if let Some(suite) = &args.suite {
        println!("Selected suite: {suite}");
        run_benchmark(&suite, &args);
    }
    else {
        unimplemented!();
        // let md = metadata(&args.problempath).expect("Incorrect PROBLEMPATH");
        // if md.is_file() {
        //     let writer: Box<dyn Write> = match args.output {
        //         Some(output) => Box::new(File::create(output).expect("Could not create file")),
        //         None => Box::new(io::stdout()),
        //     };

        //     let mut tgp = TinyGP::from_problem(&args.problempath, args.seed, writer).unwrap();
        //     tgp.evolve(args.generations);
        // } else if md.is_dir() {
        //     let base_path = &args
        //         .output
        //         .expect("Output path must be specified for a problem suite");
        //     let md = metadata(&base_path).expect("Output path does not exist");
        //     if !md.is_dir() {
        //         panic!("Output path is not a directory")
        //     }
        //     for entry in fs::read_dir(&args.problempath).expect("Cannot read directory at PROBLEMPATH")
        //     {
        //         let entry = entry.expect("wtf");
        //         let input = entry.path();
        //         let output = format!("{}{}", base_path, entry.file_name().to_str().unwrap());
        //         println!("{output}");
        //         if entry.path().is_file() {
        //             let writer: Box<dyn Write> =
        //                 Box::new(File::create(output).expect("Could not create file"));
        //             let mut tgp =
        //                 TinyGP::from_problem(input.to_str().unwrap(), args.seed, writer).unwrap();
        //             tgp.evolve(args.generations);
        //         }
        //     }
        // } else {
        //     panic!("PROBLEMPATH is not a dir, not a file, what is it?");
        // }
    }
}