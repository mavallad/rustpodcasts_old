use crate::db::{establish_connection, model_series::Serie};

#[test]
fn create_serie_with_description() {
    let conn = establish_connection();
    let title = "Rust Serie";
    let description = Some("Rust Serie description");
    let image_url = None;
    let link = "https://www.rust_serie.com";
    let status = "A";
    let serie = Serie::create(title, description, image_url, link, status, &conn).unwrap();
    assert_eq!(serie.title.as_str(), title);
    assert_eq!(serie.description.unwrap().as_str(), description.unwrap());
    assert_eq!(serie.link.as_str(), link);
    assert_eq!(serie.status.as_str(), status);
}

#[test]
fn create_serie_with_existing_title() {
    let conn = establish_connection();
    let title = "Rust Serie";
    let description = None;
    let image_url = None;
    let link = "https://www.rust_serie.com";
    let status = "A";
    let serie = Serie::create(title, description, image_url, link, status, &conn).unwrap();
    let existing_serie = Serie::create(title, description, image_url, link, status, &conn).unwrap();
    assert_eq!(serie.id, existing_serie.id);
}

#[test]
fn list_series() {
    let conn = establish_connection();
    let title = "Rust Serie";
    let description = None;
    let image_url = None;
    let link = "https://www.rust_serie.com";
    let status = "A";
    let serie = Serie::create(title, description, image_url, link, status, &conn).unwrap();
    let existing_series = Serie::list(&conn);
    assert_eq!(1, existing_series.len());
    assert_eq!(serie.id, existing_series[0].id);
}
#[test]
fn get_serie_by_title() {
    let conn = establish_connection();
    let title = "Rust Serie";
    let description = None;
    let image_url = None;
    let link = "https://www.rust_serie.com";
    let status = "A";
    let serie = Serie::create(title, description, image_url, link, status, &conn).unwrap();
    let existing_serie = Serie::by_title(&title, &conn).unwrap();
    assert_eq!(serie.id, existing_serie.id);
}
#[test]
fn get_serie_by_id() {
    let conn = establish_connection();
    let title = "Rust Serie";
    let description = None;
    let image_url = None;
    let link = "https://www.rust_serie.com";
    let status = "A";
    let serie = Serie::create(title, description, image_url, link, status, &conn).unwrap();
    let existing_serie = Serie::by_id(&serie.id, &conn).unwrap();
    assert_eq!(serie.id, existing_serie.id);
}
