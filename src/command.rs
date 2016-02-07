use std::path::Path;

pub struct Command {
    path: String,
    origin: String,
    destination: String,
}

#[derive(Debug)]
pub enum CommandParseError {
    InvalidArgCount,
    InvalidPath,
}

impl Command {
    pub fn from_args<I: IntoIterator<Item = String>>(args: I) -> Result<Command, CommandParseError> {
        let mut args = args.into_iter();
        Ok(Command {
            path: try!(read_path(args.next())),
            origin: try!(args.next().ok_or(CommandParseError::InvalidArgCount)),
            destination: try!(args.next().ok_or(CommandParseError::InvalidArgCount))
        })
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

fn read_path<T>(path: Option<T>) -> Result<String, CommandParseError>
    where T: AsRef<Path> + Into<String>
{
    match path {
        None => Err(CommandParseError::InvalidArgCount),
        Some(path) => if path.as_ref().exists() {
            Ok(path.into())
        } else {
            Err(CommandParseError::InvalidPath)
        }
    }
}
