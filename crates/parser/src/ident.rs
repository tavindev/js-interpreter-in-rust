#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Ident(String);

impl Ident {
    pub fn new<S: Into<String>>(ident: S) -> Self {
        Self(ident.into())
    }

    pub fn value(&self) -> String {
        self.0.clone()
    }
}

impl Into<String> for Ident {
    fn into(self) -> String {
        self.0
    }
}
