use std::fmt::Debug;

use crate::core::{Read, item_reader::ItemReader, item_processsor::Process};

#[derive(Debug)]
pub struct Step<'a, T: Debug + Read + Process<T, Item = T>> {
    step_id: u8,
    step_name: &'a str,
    path: &'a str,
    delimiter: &'a str,
    data: Option<Vec<T>>,
    status: Status,
    item_reader: Option<ItemReader>,
    item_processor: bool,
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
            data: None,
            status: Status::Starting,
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

    pub fn set_item_reader(&mut self, item_reader: ItemReader) {
        self.item_reader = Some(item_reader);
    }

    pub fn run(&mut self) {
        use crate::core::item_reader::Reader;

        if let Some(item_reader) = &self.item_reader {
            self.status = Status::Reading;
            self.data = item_reader.read::<T>(self.path, self.delimiter);
            dbg!(&self);
        }

        if self.item_processor {
            if let Some(data) = &mut self.data {
                self.status = Status::Processing;
                T::process(data);
            }
        }
        dbg!(&self);


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
                user.name = user.name.to_uppercase()+ &" ".to_string() + &user.age.to_string();
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
        let item_reader: ItemReader = ItemReader::new();
        let mut step: Step<'_, User> = Step::new(1, "insert_users", "db.txt", ";");
        step.set_item_reader(item_reader);
        step.item_processor = true;
        step.run();
    }
}
