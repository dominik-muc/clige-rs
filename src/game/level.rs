use super::object::Object;

pub struct Level<'a> {
    name: &'a str,
    content: Vec<&'a dyn Object>,
}

impl Level<'_> {
    pub fn get_name(&self) -> &str {
        self.name
    }
    pub fn get_contents(&self) -> Vec<Box<dyn Object>> {
        Vec::new()
    }
}
