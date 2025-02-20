use crate::utils;
use tokio::sync::OnceCell;

#[allow(dead_code)]
#[derive(Debug, sqlx::FromRow)]
pub struct Data {
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

pub static STAFF: OnceCell<Vec<Data>> = OnceCell::const_new();

pub struct Record {
    pub id: i64,
    pub name: String,
    pub nationality: String,
    pub rank: String,
    pub division: String,
}

impl Record {
    pub fn from_data(staff: &Data, lang: utils::Language) -> Self {
        Record {
            id: staff.id,
            name: if lang == utils::Language::English {
                staff.namen.clone().unwrap_or_default()
            } else {
                staff.namar.clone().unwrap_or_default()
            },
            nationality: utils::nationality(staff.nat.unwrap_or(1), lang),
            rank: utils::rank(staff.rank.unwrap_or(3), lang),
            division: utils::division(staff.div.unwrap_or(1), lang),
        }
    }
}
