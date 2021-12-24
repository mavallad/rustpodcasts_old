
use super::state::AppState;
use actix_web::{web, HttpResponse};
use super::models::Channel;

pub async fn health_check_handler(app_state: web::Data<AppState>) -> HttpResponse {
    let health_check_response = &app_state.health_check_response;
    let mut visit_count = app_state.visit_count.lock().unwrap();
    let response = format!("{} {} times", health_check_response, visit_count);
    *visit_count += 1;
    HttpResponse::Ok().json(&response)
}

pub async fn new_channel_handler(
    new_channel: web::Json<Channel>,
    app_state: web::Data<AppState>
) -> HttpResponse {
    println!("Received new channel");
    let channels_count = app_state
        .channels
        .lock()
        .unwrap()
        .clone()
        .len();

    let channel = Channel {
        id: Some(channels_count + 1),
        name: new_channel.name.clone(),
        description: new_channel.description.clone(),
        url: new_channel.url.clone(),
        icon_path: new_channel.icon_path.clone(),
        language: new_channel.language.clone(),
        active: new_channel.active
    };
    app_state.channels.lock().unwrap().push(channel);
    HttpResponse::Ok().json("Added channel")
}

pub async fn all_channels_handler(
    app_state: web::Data<AppState>
) -> HttpResponse {
    let channels = app_state
    .channels
    .lock()
    .unwrap()
    .clone();

    HttpResponse::Ok().json(channels)
}

pub async fn channel_handler(
    app_state: web::Data<AppState>,
    params: web::Path<(usize)>,
) -> HttpResponse {
    let channel_id: usize = params.0;
    let filtered_channel = app_state
        .channels
        .lock()
        .unwrap()
        .clone()
        .into_iter()
        .find(|c| c.id.is_some() && c.id.unwrap() == channel_id);

    if filtered_channel.is_some() {
        HttpResponse::Ok().json(filtered_channel.unwrap())
    } else {
        HttpResponse::Ok().json("No channels found with id".to_string())
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::http::StatusCode;
    use std::sync::{Mutex, mpsc::channel};

    #[actix_rt::test]
    async fn post_channel_test() {
        let channel = web::Json(Channel {
            id: None,
            name: "First Channel".into(),
            description: Some("Description of first channel".into()),
            url: "http://testurl.com".into(),
            icon_path: "/icon_localtion.png".into(),
            language: "en".into(),
            active: true
        });
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
        visit_count: Mutex::new(0),
        channels: Mutex::new(vec![]),
        });
        let resp = new_channel_handler(channel, app_state).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn get_one_course_success() {
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            channels: Mutex::new(vec![]),
        });
        let params: web::Path<usize> = web::Path::from(1);
        let resp = channel_handler(app_state, params).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }
}
