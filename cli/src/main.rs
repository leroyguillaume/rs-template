mod arg;

use arg::Arguments;
use clap::Parser;
use simple_logger::SimpleLogger;

fn main() {
    let args = Arguments::parse();
    SimpleLogger::new()
        .with_level(args.verbosity.log_level_filter())
        .init()
        .unwrap();
}
