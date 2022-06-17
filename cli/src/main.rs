mod lib;

use lib::execute;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    execute(args);
}
