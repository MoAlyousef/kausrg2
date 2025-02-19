use crate::utils;
use tokio::sync::OnceCell;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Record {
    pub id: i64,
    pub namen: Option<String>,
    pub namar: Option<String>,
    pub nat: Option<i64>,
    pub rank: Option<i64>,
    pub div: Option<i64>,
    pub spec: Option<String>,
    pub specr: Option<String>,
    pub qual: Option<String>,
    pub qlar: Option<String>,
    pub email: Option<String>,
    pub image: Option<String>,
}

pub static STAFF: OnceCell<Vec<Record>> = OnceCell::const_new();

pub struct RecordEn {
    pub id: i64,
    pub name: String,
    pub nationality: String,
    pub rank: String,
    pub division: String,
}

impl RecordEn {
    pub fn from_record(staff: &Record) -> Self {
        RecordEn {
            id: staff.id,
            name: staff.namen.clone().unwrap_or(String::new()),
            nationality: utils::nationality(staff.nat.unwrap_or(1)),
            rank: utils::rank(staff.rank.unwrap_or(3)),
            division: utils::division(staff.div.unwrap_or(1)),
        }
    }
}
