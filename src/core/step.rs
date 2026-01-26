use std::fmt::Debug;

use crate::core::{Read, item_reader::ItemReader};

pub trait Process<T> {
    type Item;
    fn process(data: &mut Vec<Self::Item>);
}

#[derive(Debug)]
pub struct Step<'a, T: Debug + Read> {
    step_id: u8,
    step_name: &'a str,
    path: &'a str,
    delimiter: &'a str,
    item_reader: Option<ItemReader<T>>,
    item_processor: bool,
}

#[derive(Debug)]
pub struct StepExecutor<T: Debug + Read> {
    step_status: Status,
    data: Option<Vec<T>>
}

#[derive(Debug)]
pub enum Status {
    Starting,
    Reading,
    Processing,
    Writing,
    OK,
    KO
}

impl<'a, T> Step<'a, T>
where
    T: Debug + Read + Process<T, Item = T>
{
    pub fn new(step_id: u8, step_name: &'a str, path: &'a str, delimiter: &'a str) -> Step<'a, T> {
        Self {
            step_id,
            step_name,
            path,
            delimiter,
            item_reader: None,
            item_processor: false
        }
    }

    pub fn step_id(&self) -> &u8 {
        &self.step_id
    }

    pub fn step_name(&self) -> &'a str {
        self.step_name
    }

    pub fn path(&self) -> &'a str {
        self.path
    }

    pub fn delimiter(&self) -> &'a str {
        self.delimiter
    }

    pub fn item_reader(&self) -> Option<&ItemReader<T>> {
        self.item_reader.as_ref()
    }

    pub fn set_item_reader(&mut self, item_reader: ItemReader<T>) {
        self.item_reader = Some(item_reader);
    }

    pub fn run(&mut self) {
        use crate::core::item_reader::Reader;

        let mut executor: StepExecutor<T> = StepExecutor {
            step_status: Status::Starting,
            data: Some(Vec::new())
        };

        if let Some(item_reader) = self.item_reader() {
            executor.step_status = Status::Reading;
            executor.data = item_reader.read(self.path, self.delimiter);
            dbg!(&executor);
        }

        if self.item_processor {
            if let Some(data) = &mut executor.data {
                executor.step_status = Status::Processing;
                T::process(data);
            }
        }
        dbg!(&executor);


    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use tof::Deserialize;

    #[derive(Debug, Deserialize)]
    pub struct User {
        pub name: String,
        pub email: String,
        pub age: u8,
    }

    impl<T> Process<T> for User {
        type Item = User;
    
        fn process(data: &mut Vec<Self::Item>) {
            data.iter_mut().for_each(|user| {
                user.name = user.name.to_uppercase();
                user.email = user.email.to_ascii_uppercase();
            });
        }
    }

    #[test]
    fn step() {
        let step: Step<'_, User> = Step::new(1, "insert_users", "db.txt", ";");
        assert_eq!("insert_users", step.step_name());
        assert_eq!("db.txt", step.path());
        assert_eq!(";", step.delimiter());
        
    }

    #[test]
    fn run_step() {
        let item_reader: ItemReader<User> = ItemReader::new();
        let mut step: Step<'_, User> = Step::new(1, "insert_users", "db.txt", ";");
        step.set_item_reader(item_reader);
        step.item_processor = true;
        step.run();
    }
}
