use super::models::Channel;
use std::sync::Mutex;

pub struct AppState {
    pub health_check_response: String,
    pub visit_count: Mutex<u32>,
    pub channels: Mutex<Vec<Channel>>,
}
