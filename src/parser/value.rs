#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Int(i64),
    String(String),
    Ident(String),
}
