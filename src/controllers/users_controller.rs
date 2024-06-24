use actix_web::web::Path;
use actix_web::{get, post, put, web, HttpResponse, Scope};
use diesel::prelude::*;
use diesel::result::Error;
use diesel::QueryDsl;

use crate::models::UserModel;
use crate::types::orders::{OrderDTO, OrderedProductDTO};
use crate::AppData;

use crate::schema::users::dsl::*;

pub fn users_scope() -> Scope {
    Scope::new("users")
        .service(get_by_telegram_id)
        .service(change_phone_number)
        .service(register)
}

#[get("/{telegram_id}")]
async fn get_by_telegram_id(app_data: web::Data<AppData>, tg_id: web::Path<i64>) -> HttpResponse {
    match users
        .filter(activated.eq(true))
        .find(*tg_id)
        .select(UserModel::as_select())
        .first(&mut app_data.db_pool.get().unwrap()) as Result<UserModel, Error>
    {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

#[post("/register")]
async fn register(app_data: web::Data<AppData>, new_user: web::Json<UserModel>) -> HttpResponse {
    match diesel::insert_into(users)
        .values(&new_user.0)
        .execute(&mut app_data.db_pool.get().unwrap())
    {
        Ok(_) => HttpResponse::Created().json(new_user.0),
        Err(_) => HttpResponse::Forbidden().finish(),
    }
}

#[put("/{tg_id}/activate/{secret_code}")]
async fn activate(
    _app_data: web::Data<AppData>,
    _path_data: web::Path<(i64, String)>,
) -> HttpResponse {
    todo!()
}

#[put("/{tg_id}/change/phone_number/{phone_nuber}")]
async fn change_phone_number(
    app_data: web::Data<AppData>,
    path_data: web::Path<(i64, String)>,
) -> HttpResponse {
    let (tg_id, new_phone_number) = (*path_data).clone();
    match diesel::update(users.find(tg_id))
        .set(phone_number.eq(new_phone_number.clone()))
        .execute(&mut app_data.db_pool.get().unwrap())
    {
        Ok(_) => HttpResponse::Ok().json(new_phone_number.clone()),
        Err(_) => HttpResponse::Forbidden().finish(),
    }
}

#[post("/{tg_id}/make_order")]
async fn make_order(
    app_data: web::Data<AppData>,
    products: web::Json<Vec<OrderedProductDTO>>,
    tg_id: Path<i64>,
) -> HttpResponse {
    let store = OrderDTO {
        customer_telegram_id: *tg_id,
        products: products.0,
    }
    .store(&mut app_data.db_pool.get().unwrap());
    match store {
        Ok(order) => HttpResponse::Created().json(order),
        Err(_) => HttpResponse::BadRequest().finish(),
    }
}
