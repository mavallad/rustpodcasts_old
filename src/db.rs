pub mod model_series;
pub mod model_episodes;
pub mod schema;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

embed_migrations!();
pub fn establish_connection() -> SqliteConnection {
    if cfg!(test) {
        let conn = SqliteConnection::establish(":memory:")
          .unwrap_or_else(|_| panic!("Error creating test database"));
        
        let _result = diesel_migrations::run_pending_migrations(&conn);
        
//        diesel::sql_query("PRAGMA foreign_keys = ON").execute(&conn).unwrap();
        
        conn
    } else {
        println!("NO ES TEST");
        dotenv().ok();
    
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    
        SqliteConnection::establish(&database_url)
          .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
    }
}
