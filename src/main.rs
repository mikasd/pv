use std::env;
use std::io::{self, Read, Write};

const BUFFER_SIZE: usize = 16 * 1024;

fn main() {
    // bool that checks if env var exists
    let silent = !env::var("PV_SILENT").unwrap_or_default().is_empty();
    let mut total_bytes = 0;

    loop {
        let mut buffer = [0; BUFFER_SIZE];
        let num_read = match io::stdin().read(&mut buffer) {
            Ok(0) => break,
            Ok(x) => x,
            Err(_) => break,
        };
        total_bytes += num_read;
        io::stdout().write_all(&buffer[..num_read]).unwrap();
    }
    if !silent {
        eprintln!("total bytes: {}", total_bytes);
    }
}
