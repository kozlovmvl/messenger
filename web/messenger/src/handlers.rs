use actix_web::{get, post, web, HttpResponse, Error};
use crate::{db::PgPool, schema::{users, messages}, models::{User, Message, NewMessage}};
use diesel::prelude::*;

#[get("/users/list")]
async fn list_users(pool: web::Data<PgPool>) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("Failed to get connection from pool.");
    let results = web::block(move || {
        users::table
            .select((users::id, users::username))
            .load::<User>(&conn)
    }).await?.map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(results))
}

#[get("/messages/inner/{recipient_id}")]
async fn inner_messages(pool: web::Data<PgPool>, path: web::Path<(i32,)>) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("Failed to get connection from pool.");
    let results = web::block(move || {
        messages::table
            .filter(messages::recipient_id.eq(path.into_inner().0))
            .load::<Message>(&conn)
    }).await?.map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(results))
}

#[post("/messages/create")]
async fn create_message(pool: web::Data<PgPool>, data: web::Json<NewMessage>) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("Failed to get connection from pool.");
    let new_message = NewMessage{
        author_id: data.author_id,
        recipient_id: data.recipient_id,
        text: data.text.clone(),
        date: data.date
    };
    diesel::insert_into(messages::table)
        .values(&new_message)
        .execute(&conn)
        .expect("Error saving new message");

    Ok(HttpResponse::Ok().json(data))
}

