mod ansiblefile;
mod arguments;

use self::{ansiblefile::AnsibleFile, arguments::Arguments};
use std::error::Error;
use structopt::StructOpt;

fn main() {
    let args = Arguments::from_args();
    let ansiblefile = AnsibleFile::from_file(args.get_input());

    match ansiblefile {
        Ok(file) => {
            println!("{:#?}", file);
        }
        Err(err) => {
            eprintln!("Error: {}", err);

            let mut dyn_err: &dyn Error = &err;
            while let Some(source) = dyn_err.source() {
                eprintln!("    caused by: {}", source);
                dyn_err = source;
            }
        }
    }
}
