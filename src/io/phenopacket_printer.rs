use prost::Message;
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;

/// A trait for serializing Phenopacket (or similar) Protobuf messages to an output stream.
pub trait PhenopacketPrinter {
    /// Print the Protobuf message to a writer (e.g. file, stdout) in JSON or YAML.
    fn print<W: Write>(&self, message: &dyn Message, writer: &mut W) -> io::Result<()>;

    /// Convenience method to print to a file path.
    fn print_to_path(&self, message: &dyn Message, path: &Path) -> io::Result<()> {
        let mut file = File::create(path)?;
        self.print(message, &mut file)
    }
}
