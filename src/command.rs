use std::path::Path;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Command {
    path: String,
    origin: String,
    destination: String,
}

impl Command {
    pub fn from_args() -> Self {
        StructOpt::from_args()
    }

    pub fn path(&self) -> &Path {
        Path::new(&self.path)
    }

    pub fn origin(&self) -> &str {
        &self.origin
    }

    pub fn destination(&self) -> &str {
        &self.destination
    }
}
