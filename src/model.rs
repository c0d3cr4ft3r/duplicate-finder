use tabled::Tabled;

#[derive(Tabled, Clone)]
pub struct DuplicateRow {
    pub hash: String,
    pub size: u64,
    pub path: String,
}
