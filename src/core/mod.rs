pub mod item_reader;
pub mod item_processsor;
pub mod item_writer;
pub mod step;

pub use item_reader::{ItemReader, Reader};
pub use item_processsor::Process;

pub trait Read: Sized {
    fn read_file(headers: &[&str], values: &[&str]) -> Option<Self>;
}