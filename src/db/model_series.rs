use uuid::Uuid;
use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use super::schema::series;
use super::schema::series::dsl::series as serie_dsl;

#[derive(Debug, Deserialize, Serialize, Queryable, Insertable)]
#[table_name = "series"]
pub struct Serie {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub image_url: Option<String>,
    pub link: String,
    pub status: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}
impl Serie {
    pub fn list(conn: &SqliteConnection) -> Vec<Self> {
        serie_dsl.load::<Serie>(conn).expect("Error loading series")
    }
    pub fn by_id(id: &str, conn: &SqliteConnection) -> Option<Self> {
        if let Ok(record) = serie_dsl.find(id).get_result::<Serie>(conn) {
            Some(record)
        } else {
            None
        }
    }
    pub fn by_title(title_str: &str, conn: &SqliteConnection) -> Option<Self> {
        use super::schema::series::dsl::title;
        if let Ok(record) = serie_dsl.filter(title.eq(title_str)).first::<Serie>(conn) {
            Some(record)
        } else {
            None
        }
    }
    pub fn create(title: &str, description: Option<&str>, image_url: Option<&str>, link: &str, status: &str, conn: &SqliteConnection) -> Option<Self> {
        let new_id = Uuid::new_v4().to_hyphenated().to_string();
        
        if let Some(serie) = Self::by_title(&title, conn) {
            return Some(serie)
        }
        
        let new_serie = Self::new_serie_struct(&new_id, title, description, image_url, link, status);
        diesel::insert_into(serie_dsl)
            .values(&new_serie)
            .execute(conn)
            .expect("Error saving new serie");
        Self::by_id(&new_id, conn)
    }
    fn new_serie_struct(id: &str, title: &str, description: Option<&str>, image_url: Option<&str>, link: &str, status: &str) -> Self {
        Serie {
            id: id.into(),
            title: title.into(),
            description: description.map(Into::into),
            image_url: image_url.map(Into::into),
            link: link.into(),
            status: status.into(),
            created_at: chrono::Local::now().naive_local(),
            updated_at: chrono::Local::now().naive_local(),
        }
    }
}

#[cfg(test)]
mod serie_test;