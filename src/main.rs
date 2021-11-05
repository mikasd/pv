use crossbeam::channel::{bounded, unbounded};
use pipev::{args::Args, read, stats, write};
use std::fs::File;
use std::io::{self, Result};
use std::thread;

fn main() -> Result<()> {
    let args = Args::parse();
    let Args {
        infile,
        outfile,
        silent,
    } = args;

    let (stats_tx, stats_rx) = unbounded();
    let (write_tx, write_rx) = bounded(1024);

    let read_handle = thread::spawn(move || {
        if infile.is_empty() {
            read::read_loop(io::stdin(), stats_tx, write_tx)
        } else {
            read::read_loop(File::open(infile)?, stats_tx, write_tx)
        }
    });
    let stats_handle = thread::spawn(move || stats::stats_loop(silent, stats_rx));
    let write_handle = thread::spawn(move || write::write_loop(&outfile, write_rx));

    let read_io_result = read_handle.join().unwrap();
    let stats_io_result = stats_handle.join().unwrap();
    let write_io_result = write_handle.join().unwrap();

    read_io_result?;
    stats_io_result?;
    write_io_result?;

    Ok(())
}
