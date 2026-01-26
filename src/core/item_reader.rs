use crate::core::Read;

pub trait Reader<'a> {
    type Item;
    fn read(&self, path: &'a str, delimiter: &'a str) -> Option<Vec<Self::Item>>;
}

#[derive(Debug)]
pub struct ItemReader<T> {
    items: Vec<T>
}

impl<T> ItemReader<T> {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn items_mut(&mut self) -> &mut Vec<T> {
        &mut self.items
    }
}

impl<'a, T> Reader<'a> for ItemReader<T> 
where
    T: Read
{

    type Item = T;

    fn read(&self, path: &'a str, delimiter: &'a str) -> Option<Vec<Self::Item>> {
        
        let content = std::fs::read_to_string(path).ok()?;
        let lines: Vec<&str> = content.lines().collect();

        let headers: Vec<&str> = lines[0].split(delimiter).collect();
        let mut items: Vec<Self::Item> = Vec::new();

        for line in lines.iter().skip(1) {
            if line.trim().is_empty() {
                continue;
            }

            let values: Vec<&str> = line.split(delimiter).collect();

            if let Some(item) = T::read_file(&headers, &values) {
                items.push(item);
            }
        }
        Some(items)
    }
}

#[cfg(test)]
mod tests {

    use crate::core::item_reader::{ItemReader, Read, Reader};
    use tof::Deserialize;

    #[derive(Debug, Deserialize)]
    struct User {
        name: String,
        email: String,
        age: u8,
    }

    #[test]
    fn item_reader() {
        let item_reader: ItemReader<User> = ItemReader::new();

        let users = item_reader.read("db.txt", ";");

        if let Some(users) = &users {
            let first = users.first().unwrap();
            assert_eq!("John Doe", first.name);
            assert_eq!("john.doe@email.com", first.email);
            assert_eq!(33, first.age);
        }
        dbg!(&users);
        
    }
}