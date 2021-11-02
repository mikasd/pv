use std::io::{self, Read, Write};

const BUFFER_SIZE: usize = 16 * 1024;

fn main() {
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
    eprintln!("total bytes: {}", total_bytes);
}
