mod params;
mod tinygp;
use structopt::StructOpt;
use tinygp::TinyGP;

#[derive(StructOpt, Debug)]
struct Args {
    #[structopt(short, long)]
    seed: Option<u64>,

    problemfile: String,
}

fn main() {
    let args = Args::from_args();

    let mut tgp = TinyGP::from_problem(&args.problemfile, args.seed).unwrap();
    tgp.evolve(100);
}
