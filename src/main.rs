use std::env;
use std::io::{self, ErrorKind, Read, Result, Write};

const BUFFER_SIZE: usize = 16 * 1024;

fn main() -> Result<()> {
    // bool that checks if env var exists
    let silent = !env::var("PV_SILENT").unwrap_or_default().is_empty();
    let mut total_bytes = 0;
    let mut buffer = [0; BUFFER_SIZE];

    loop {
        let num_read = match io::stdin().read(&mut buffer) {
            Ok(0) => break,
            Ok(x) => x,
            Err(_) => break,
        };
        total_bytes += num_read;
        if !silent {
            eprint!("\r{}", total_bytes);
        }
        if let Err(e) = io::stdout().write_all(&buffer[..num_read]) {
            if e.kind() == ErrorKind::BrokenPipe {
                break;
            }
            return Err(e);
        }
    }

    Ok(())
}
