use crate::db::{establish_connection, model_series::Serie, model_episodes::Episode};

#[test]
fn create_episode_with_id_serie_and_description() {
    let conn = establish_connection();

    let title_serie = "Rust serie";
    let description_serie = None;
    let image_url_serie = None;
    let link_serie = "https://www.rust_serie.com";
    let status = "A";
    let serie = Serie::create(title_serie, description_serie, image_url_serie, link_serie, status, &conn).unwrap();

    let title = "Rust episode";
    let description = Some("Rust episode description");
    let interviewed = Some("Shane Macgowan");
    let publication_date = chrono::NaiveDate::from_ymd(2020, 3, 4);
    let image_url = None;
    let link = "https://www.rust_episode.com";
    let duration_seconds = 564;
    let episode = Episode::create(Some(&serie.id), title_serie, title, description, interviewed, Some(&publication_date), image_url, link, duration_seconds, &conn).unwrap();

    assert_eq!(episode.title.as_str(), title);
    assert_eq!(episode.description.unwrap().as_str(), description.unwrap());
    assert_eq!(episode.link.as_str(), link);
    assert_eq!(episode.duration_seconds, duration_seconds);
    assert_eq!(episode.id_serie.unwrap().as_str(), serie.id);
}

/*
#[test]
fn create_episode_fails_with_id_serie_not_found() {
    let conn = establish_connection();

    let id_serie = "ABCDEF";
    let title_serie = "TitleSerie";

    let title = "Rust episode";
    let description = Some("Rust episode description");
    let interviewed = Some("Shane Macgowan");
    let publication_date = chrono::NaiveDate::from_ymd(2020, 3, 4);
    let image_url = None;
    let link = "https://www.rust_episode.com";
    let duration_seconds = 564;
    let episode = Episode::create(Some(&id_serie), title_serie, title, description, interviewed, Some(&publication_date), image_url, link, duration_seconds, &conn).unwrap();

    assert_eq!(episode.title.as_str(), title);
    assert_eq!(episode.description.unwrap().as_str(), description.unwrap());
    assert_eq!(episode.link.as_str(), link);
    assert_eq!(episode.duration_seconds, duration_seconds);
    assert_eq!(episode.id_serie.unwrap().as_str(), id_serie);
} */

#[test]
fn create_user_with_existing_title() {
    let conn = establish_connection();
    let title_serie = "TitleSerie";
    let title = "Rust episode";
    let description = Some("Rust episode description");
    let interviewed = Some("Shane Macgowan");
    let publication_date = chrono::NaiveDate::from_ymd(2020, 3, 4);
    let image_url = None;
    let link = "https://www.rust_episode.com";
    let duration_seconds = 564;

    let episode = Episode::create(None, title_serie, title, description, interviewed, Some(&publication_date), image_url, link, duration_seconds, &conn).unwrap();
    let existing_episode = Episode::create(None, title_serie, title, description, interviewed, Some(&publication_date), image_url, link, duration_seconds, &conn).unwrap();
    
    assert_eq!(episode.id, existing_episode.id);
}

#[test]
fn list_episodes() {
    let conn = establish_connection();

    let title_serie = "TitleSerie";
    let title = "Rust episode";
    let description = Some("Rust episode description");
    let interviewed = Some("Shane Macgowan");
    let publication_date = chrono::NaiveDate::from_ymd(2020, 3, 4);
    let image_url = None;
    let link = "https://www.rust_episode.com";
    let duration_seconds = 564;
    let episode = Episode::create(None, title_serie, title, description, interviewed, Some(&publication_date), image_url, link, duration_seconds, &conn).unwrap();

    let existing_episodes = Episode::list(&conn);
    assert_eq!(1, existing_episodes.len());
    assert_eq!(episode.id, existing_episodes[0].id);
}
#[test]
fn get_episode_by_title() {
    let conn = establish_connection();

    let title_serie = "TitleSerie";
    let title = "Rust episode";
    let description = Some("Rust episode description");
    let interviewed = Some("Shane Macgowan");
    let publication_date = chrono::NaiveDate::from_ymd(2020, 3, 4);
    let image_url = None;
    let link = "https://www.rust_episode.com";
    let duration_seconds = 564;
    let episode = Episode::create(None, title_serie, title, description, interviewed, Some(&publication_date), image_url, link, duration_seconds, &conn).unwrap();

    let existing_episode = Episode::by_title(&title, &conn).unwrap();
    assert_eq!(episode.id, existing_episode.id);
}
#[test]
fn get_episode_by_id() {
    let conn = establish_connection();

    let title_serie = "TitleSerie";
    let title = "Rust episode";
    let description = Some("Rust episode description");
    let interviewed = Some("Shane Macgowan");
    let publication_date = chrono::NaiveDate::from_ymd(2020, 3, 4);
    let image_url = None;
    let link = "https://www.rust_episode.com";
    let duration_seconds = 564;
    let episode = Episode::create(None, title_serie, title, description, interviewed, Some(&publication_date), image_url, link, duration_seconds, &conn).unwrap();

    let existing_episode = Episode::by_id(&episode.id, &conn).unwrap();
    assert_eq!(episode.id, existing_episode.id);
}
