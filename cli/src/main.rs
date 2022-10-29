mod arg;

use arg::Arguments;
use clap::Parser;

fn main() {
    let args = Arguments::parse();
    env_logger::Builder::new()
        .filter_level(args.verbosity.log_level_filter())
        .init();
}
