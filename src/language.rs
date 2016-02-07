#[derive(Clone)]
pub struct Language {
    pub name: &'static str
}

pub fn supported() -> Vec<&'static str> {
    vec!("node")
}

pub fn nodejs() -> Language {
    Language {
        name: "node"
    }
}
