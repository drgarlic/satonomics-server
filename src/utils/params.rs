pub struct Params {
    pub path: String,
    pub index: Option<usize>,
    pub body: Option<String>,
}

impl Params {
    pub fn new(path: &str, index: Option<usize>) -> Self {
        Self {
            path: path.to_string(),
            body: None,
            index,
        }
    }

    pub fn to_key(&self) -> String {
        if let Some(index) = self.index {
            format!("{}-{}", self.path, index)
        } else {
            self.path.to_string()
        }
    }
}
