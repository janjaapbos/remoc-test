#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ListItem {
    pub index: usize,
    pub item: String,
}

impl ListItem {
    pub fn new(i: usize, s: String) -> Self {
        ListItem { index: i, item: s }
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Data {
    pub field1: u32,
    pub field2: String,
    pub list: Vec<ListItem>,
}
