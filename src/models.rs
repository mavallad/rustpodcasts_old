use actix_web::web;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Channel {
    pub id: Option<usize>,
    pub name: String,
    pub description: Option<String>,
    pub url: String,
    pub icon_path: String,
    pub language: String,
    pub active: bool
}

impl From<web::Json<Channel>> for Channel {
    fn from(channel: web::Json<Channel>) -> Self {
        Channel {
            id: channel.id,
            name: channel.name.clone(),
            description: channel.description.clone(),
            url: channel.url.clone(),
            icon_path: channel.icon_path.clone(),
            language: channel.language.clone(),
            active: channel.active
        }
    }
}

/*
curl -X POST localhost:3000/courses/ -H "Content-Type: application/json" -d '{
"name":"Channel1", "description": "My first channel", "url": "http://myfirstchannel.com", "icon_path": "no icon", "active": true }'
*/
