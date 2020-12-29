use uuid::Uuid;
use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use super::schema::episodes;
use super::schema::episodes::dsl::episodes as episode_dsl;

#[derive(Debug, Deserialize, Serialize, Queryable, Insertable)]
#[table_name = "episodes"]
pub struct Episode {
    pub id: String,
    pub id_serie: Option<String>,
    pub serie_title: String,
    pub title: String,
    pub description: Option<String>,
    pub interviewed: Option<String>,
    pub publication_date: Option<chrono::NaiveDate>,
    pub image_url: Option<String>,
    pub link: String,
    pub duration_seconds: i32,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}
impl Episode {
    pub fn list(conn: &SqliteConnection) -> Vec<Self> {
        episode_dsl.load::<Episode>(conn).expect("Error loading episodes")
    }
    pub fn by_id(id: &str, conn: &SqliteConnection) -> Option<Self> {
        if let Ok(record) = episode_dsl.find(id).get_result::<Episode>(conn) {
            Some(record)
        } else {
            None
        }
    }
    pub fn by_title(title_str: &str, conn: &SqliteConnection) -> Option<Self> {
        use super::schema::episodes::dsl::title;
        if let Ok(record) = episode_dsl.filter(title.eq(title_str)).first::<Episode>(conn) {
            Some(record)
        } else {
            None
        }
    }
    pub fn create(id_serie: Option<&str>, serie_title: &str, title: &str, description: Option<&str>, interviewed: Option<&str>, publication_date: Option<&chrono::NaiveDate>,
            image_url: Option<&str>, link: &str, duration_seconds: i32, conn: &SqliteConnection) -> Option<Self> {
        let new_id = Uuid::new_v4().to_hyphenated().to_string();
        
        if let Some(episode) = Self::by_title(&title, conn) {
            return Some(episode)
        }
        
        let new_episode = Self::new_episode_struct(&new_id, id_serie, serie_title, title, description, interviewed, publication_date, image_url, link, duration_seconds);
        diesel::insert_into(episode_dsl)
            .values(&new_episode)
            .execute(conn)
            .expect("Error saving new episode");
        Self::by_id(&new_id, conn)
    }
    fn new_episode_struct(id: &str, id_serie: Option<&str>, serie_title: &str, title: &str, description: Option<&str>, interviewed: Option<&str>,
            publication_date: Option<&chrono::NaiveDate>, image_url: Option<&str>, link: &str, duration_seconds: i32) -> Self {
        Episode {
            id: id.into(),
            id_serie: id_serie.map(Into::into),
            serie_title: serie_title.into(),
            title: title.into(),
            description: description.map(Into::into),
            interviewed: interviewed.map(Into::into),
            publication_date: publication_date.map(Clone::clone),
            image_url: image_url.map(Into::into),
            link: link.into(),
            duration_seconds: duration_seconds.into(),
            created_at: chrono::Local::now().naive_local(),
            updated_at: chrono::Local::now().naive_local(),
        }
    }
}

#[cfg(test)]
mod episode_test;