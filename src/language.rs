pub struct Language {
    pub name: &'static str
}

pub fn nodejs() -> Language {
    Language {
        name: "node"
    }
}
