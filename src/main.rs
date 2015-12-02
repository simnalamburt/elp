extern crate rustc_serialize;
extern crate docopt;
#[macro_use]
extern crate aws_abacus;
extern crate walkdir;
extern crate chrono;

use docopt::Docopt;
use std::path;
use aws_abacus::elb_log_files;
use aws_abacus::RuntimeContext;
use chrono::{UTC};

fn main() {
    let args: Args = Docopt::new(USAGE)
                            .and_then(|d| d.decode())
                            .unwrap_or_else(|e| e.exit());

    let runtime_context = RuntimeContext {
        debug: args.flag_debug,
    };
    let debug = runtime_context.debug;
    let log_location = &path::Path::new(&args.arg_log_location);
    debug!(debug, "Running summary on {}.", log_location.to_str().unwrap());

    let start = UTC::now(); //TODO put this behind a cli arg
    let mut filenames = Vec::new();
    match elb_log_files::file_list(log_location, &mut filenames) {
        Ok(num_files) => {
            debug!(debug, "Found {} files.", num_files);
            let total_record_count = elb_log_files::process_files(&runtime_context, filenames);
            debug!(debug, "Processed {} records in {} files.", total_record_count, num_files);
        },
        Err(e) => {
            println!("ERROR: {}", e);
        },
    };

    let end = UTC::now(); //TODO put this behind a cli arg

    let time = end - start; //TODO put this behind a cli arg

    println!("TIME: {}", time);  //TODO put this behind a cli arg
}

const USAGE: &'static str = "
aws-abacus

Usage:
  aws-abacus <log-location>
  aws-abacus (-d | --debug) <log-location>
  aws-abacus (-h | --help)

Options:
  -h --help     Show this screen.
  -d --debug    Turn on debug output
";

#[derive(Debug, RustcDecodable)]
struct Args {
    arg_log_location: String,
    flag_debug: bool,
}
