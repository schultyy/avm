#[derive(Clone)]
pub struct Language {
    pub name: &'static str
}

pub fn nodejs() -> Language {
    Language {
        name: "node"
    }
}

pub fn ruby() -> Language {
    Language {
        name: "ruby"
    }
}
