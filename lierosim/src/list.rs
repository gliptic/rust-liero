pub struct List<T> {
    storage: Vec<T>
}

impl<T> List<T> {
    fn new() -> List<T> {
        List { storage: Vec::new() }
    }
}