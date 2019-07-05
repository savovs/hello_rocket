extern crate diesel;
extern crate hello_rocket;

use rocket_contrib::json::{Json};

use self::db::models::posts::*;
use self::diesel::prelude::*;
use self::hello_rocket::*;

#[get("/posts")]
pub fn get_posts() -> QueryResult<Json<Vec<Post>>> {
    use hello_rocket::db::schema::posts::dsl::*;

    let connection = establish_connection();

    posts.filter(published.eq(true))
        .limit(5)
        .load::<Post>(&connection)
        .map(|result| Json(result))
}
