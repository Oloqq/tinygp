mod params;
mod tinygp;
use structopt::StructOpt;
use tinygp::TinyGP;
use std::fs::metadata;
use std::fs::File;
use std::io::{self, Write};

#[derive(StructOpt, Debug)]
struct Args {
    #[structopt(short, long)]
    seed: Option<u64>,

    #[structopt(short, long, default_value = "100")]
    generations: usize,

    #[structopt(short, long)]
    output: Option<String>,

    problempath: String,
}

fn main() {
    let args = Args::from_args();

    let md = metadata(&args.problempath).expect("Incorrect PROBLEMPATH");
    if md.is_file() {
        let writer: Box<dyn Write> = match args.output {
            Some(output) => Box::new(File::create(output).expect("Could not create file")),
            None => Box::new(io::stdout())
        };

        let mut tgp = TinyGP::from_problem(&args.problempath, args.seed, writer).unwrap();
        tgp.evolve(args.generations);
    } else if md.is_dir() {
        let base_path = args.output.expect("Output path must be specified for a problem suite");
    } else {
        panic!("PROBLEMPATH is not a dir, not a file, what is it?");
    }

}
