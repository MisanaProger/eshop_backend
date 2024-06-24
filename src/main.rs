use actix_web::{web, App, HttpServer};
use controllers::users_controller::users_scope;
use diesel::{
    r2d2::{self, ConnectionManager},
    PgConnection,
};
use redis::Client;

mod controllers;
mod models;
mod schema;
mod types;

type DBPool = r2d2::Pool<ConnectionManager<PgConnection>>;
type DBConnection = r2d2::PooledConnection<ConnectionManager<PgConnection>>;

#[derive(Clone)]
struct AppData {
    db_pool: DBPool,
    _redis_client: redis::Client,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_data = AppData {
        db_pool: establish_db_connection(),
        _redis_client: establish_redis_connection(),
    };

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_data.clone()))
            .service(users_scope())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

fn establish_db_connection() -> DBPool {
    let manager = r2d2::ConnectionManager::<PgConnection>::new(
        dotenvy::var("DATABASE_URL").expect("DATABASE_URL mustbe set") as String,
    );
    r2d2::Pool::builder()
        .build(manager)
        .expect("database URL should be valid path to db")
}

fn establish_redis_connection() -> Client {
    Client::open(dotenvy::var("REDIS_URL").expect("REDIS_URL mustbe set"))
        .expect("redis URL should be valid path to redis")
}
