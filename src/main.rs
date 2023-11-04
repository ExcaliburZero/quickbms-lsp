use std::io::{stdin, stdout, BufReader};

use quickbms_lsp::server::server;

fn main() {
    let exit_code = server::run(BufReader::new(stdin()), stdout());
    std::process::exit(exit_code);
}
