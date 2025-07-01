pub struct Upload {
    pub name: String,
    pub teacher: String,
    pub subject: String,
    pub year: String,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct Test {
    pub id: i64,
    pub name: String,
    pub teacher: String,
    pub subject: String,
    pub year: i64,
    pub files: i64,
}

#[derive(serde::Deserialize)]
pub struct Search {
    pub name: Option<String>,
    pub teacher: Option<String>,
    pub subject: Option<String>,
    pub year: Option<i64>,
}