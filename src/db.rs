pub mod models;
pub mod schema;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

use models::*;
use schema::posts;
use schema::posts::dsl::*;

fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}",
                                   database_url))
}

pub fn get_posts() -> Vec<Post> {
    let connection = establish_connection();
    posts
        .filter(published.eq(true))
        .limit(5)
        .load::<Post>(&connection)
        .expect("Error loading posts")
}

use uuid::Uuid;

pub fn create_post(t: &str, b: &str) -> String {
    let connection = establish_connection();

    let uuid = Uuid::new_v4().to_hyphenated().to_string();

    let new_post = NewPost { id: &uuid,  title: t, body: b };

    diesel::insert_into(posts::table)
        .values(&new_post)
        .execute(&connection)
        .expect("Error saving new post");

    uuid
}

pub fn publish_post(key: String) -> usize {
    let connection = establish_connection();

    diesel::update(posts.find(key))
        .set(published.eq(true))
        .execute(&connection)
        .expect("Unable to find post")
}



