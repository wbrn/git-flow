mod error;
mod utils;
mod gitc;
mod gf;
mod cli;

use crate::cli::cli_run;
use crate::error::Error;

fn main() {
    // the main is a handler to all final results
    match cli_run() {
        Ok(s) => println!("{}", s),
        Err(e) => {
            match e {
                Error::Generic(generr) => eprintln!("generic: {}", generr),
                Error::NoHead => eprintln!("Git2: no head"),
                Error::Git(giterr) => eprintln!("Git2 {}", giterr),
                Error::Io(ioerr) => eprintln!("Io {}", ioerr),
            }
        }
    }
}
