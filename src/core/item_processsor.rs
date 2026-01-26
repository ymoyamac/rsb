pub trait Process<T> {
    type Item;
    fn process(data: &mut Vec<Self::Item>);
}