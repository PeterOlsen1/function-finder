pub struct Call {
    pub filename: String,
    pub content: String,
    pub idx: u16
}

pub struct Definition {
    pub content: String,
    pub name: String,
    pub idx: u16,
    pub params: Vec<String>,
    pub filename: String
}