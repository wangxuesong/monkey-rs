#[derive(Debug)]
pub enum Object {
    Int(i64),
}

impl Object {
    pub fn inspect(&self) -> String {
        match self {
            Object::Int(i) => i.to_string(),
        }
    }
}
