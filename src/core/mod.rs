pub mod item_reader;
pub mod item_processsor;
pub mod step;

pub trait Read: Sized {
    fn read_file(headers: &[&str], values: &[&str]) -> Option<Self>;
}