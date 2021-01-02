use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "anstool")]
pub struct Arguments {
    /// Input file
    #[structopt(name = "FILE", parse(from_os_str))]
    input: PathBuf,
}

impl Arguments {
    pub fn get_input(&self) -> &PathBuf {
        &self.input
    }
}
