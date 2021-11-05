use crate::BUFFER_SIZE;

use crossbeam::channel::Sender;
use std::io::{BufReader, Read, Result};
use encoding::{DecoderTrap, Encoding, all::WINDOWS_1252};

pub fn read_loop<R: Read>(reader: R, stats_tx: Sender<usize>, write_tx: Sender<Vec<u8>>) -> Result<()> {
    let mut reader = BufReader::new(reader);

    let mut buffer = [0; BUFFER_SIZE];
    loop {
        let num_read = match reader.read(&mut buffer) {
            Ok(0) => break,
            Ok(x) => x,
            Err(_) => break,
        };

        let decoded_bytes = WINDOWS_1252.decode(&buffer, DecoderTrap::Replace);

        if let Ok(bytes) = decoded_bytes {
            let _ = stats_tx.send(num_read);
            if write_tx.send(bytes.into_bytes()).is_err() {
                break;
            }
        } else {
            eprintln!("error during decoding, {:?}", decoded_bytes);
            break;
        }
    }
    
    let _ = stats_tx.send(0);
    let _ = write_tx.send(Vec::new());
    Ok(())
}
