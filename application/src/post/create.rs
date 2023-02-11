use domain::models::{Post, NewPost};
use shared::response_models::{Response, ResponseBody};
use infrastructure::establish_connection;
use diesel::prelude::*;
use rocket::response::status::Created;
use rocket::serde::json::Json;

pub fn create_post(post: Json<NewPost>) -> Created<String> {
    use domain::schema::posts;
    use domain::schema::posts::id;

    let post = post.into_inner();

    diesel::insert_into(posts::table).values(&post).execute(&mut establish_connection()).expect("Error saving post.");

    let result = posts::table.order(id.desc()).first::<Post>(&mut establish_connection()).unwrap();
    let response = Response { body: ResponseBody::Post(result) };

    Created::new("").tagged_body(serde_json::to_string(&response).unwrap())
}