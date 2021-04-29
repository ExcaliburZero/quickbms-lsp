use std::io::{stdin, stdout, BufReader};

use quickbms_lsp::server::server;

fn main() {
    server::run(BufReader::new(stdin()), stdout());
}
