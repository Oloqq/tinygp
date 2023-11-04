mod params;
mod tinygp;
use structopt::StructOpt;
use tinygp::TinyGP;

#[derive(StructOpt, Debug)]
struct Args {
    #[structopt(short, long)]
    seed: Option<u64>,

    #[structopt(short, long, default_value = "100")]
    generations: usize,

    problemfile: String,
}

fn main() {
    let args = Args::from_args();

    let mut tgp = TinyGP::from_problem(&args.problemfile, args.seed).unwrap();
    tgp.evolve(args.generations);
}
