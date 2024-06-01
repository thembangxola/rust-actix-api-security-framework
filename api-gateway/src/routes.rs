use actix_web::{web, HttpResponse, Responder};
use mongodb::{Client, Database};
use serde::{Deserialize, Serialize};
use futures::stream::StreamExt;
use mongodb::bson::doc;

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub username: String,
    pub password: String,
    pub email: String,
}

pub async fn get_users(db: web::Data<Database>) -> impl Responder {
    let collection = db.collection::<User>("users");
    let mut cursor = collection.find(None, None).await.unwrap();
    let mut users = Vec::new();

    while let Some(user) = cursor.next().await {
        users.push(user.unwrap());
    }

    HttpResponse::Ok().json(users)
}

pub async fn create_user(db: web::Data<Database>, user: web::Json<User>) -> impl Responder {
    let collection = db.collection::<User>("users");
    let new_user = User {
        username: user.username.clone(),
        password: user.password.clone(),
        email: user.email.clone(),
    };

    let result = collection.insert_one(new_user, None).await;

    match result {
        Ok(_) => HttpResponse::Created().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn update_user(db: web::Data<Database>, user: web::Json<User>) -> impl Responder {
    let collection = db.collection::<User>("users");
    let filter = doc! { "username": &user.username };
    let update = doc! { "$set": { "email": &user.email, "password": &user.password } };

    let result = collection.update_one(filter, update, None).await;

    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
pub async fn delete_user(db: web::Data<Database>, username: web::Path<String>) -> impl Responder {
    let collection = db.collection::<User>("users");
    let filter = doc! { "username": username.into_inner() };

    let result = collection.delete_one(filter, None).await;

    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .route("", web::get().to(get_users))
            .route("", web::post().to(create_user))
            .route("", web::put().to(update_user))
            .route("/{username}", web::delete().to(delete_user))
    );
}

pub async fn not_found() -> impl Responder {
    HttpResponse::NotFound().body("Not Found")
}

