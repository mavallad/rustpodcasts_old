use super::handlers::*;
use actix_web::web;

pub fn general_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(health_check_handler));
}

pub fn channel_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/channels")
            .route("/", web::post().to(new_channel_handler))
            .route("/{channel_id}", web::get().to(channel_handler))
            .route("/", web::get().to(all_channels_handler)));
}
