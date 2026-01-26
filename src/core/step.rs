
pub trait FromFile: Sized {
    fn from_file_row(headers: &[&str], values: &[&str]) -> Option<Self>;
}

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
    T: FromFile
{

    type Item = T;

    fn read(&self, path: &'a str, delimiter: &'a str) -> Option<Vec<Self::Item>> {
        
        let content = std::fs::read_to_string(path).ok()?;
        let lines: Vec<&str> = content.lines().collect();

        let headers: Vec<&str> = lines[0].split(delimiter).collect();
        let mut items: Vec<Self::Item> = Vec::new();

        for line in lines.iter().skip(1) {
            dbg!(&line);
            if line.trim().is_empty() {
                continue;
            }

            let values: Vec<&str> = line.split(delimiter).collect();

            if let Some(item) = T::from_file_row(&headers, &values) {
                items.push(item);
            }
        }
        Some(items)
    }
}

#[derive(Debug)]
pub struct Step<'a, T> {
    step_id: u8,
    step_name: &'a str,
    path: &'a str,
    delimiter: &'a str,
    data: T,
    item_reader: Option<ItemReader<T>>
}

#[cfg(test)]
mod tests {

    use super::*;
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

        dbg!(users);
        
    }
}
