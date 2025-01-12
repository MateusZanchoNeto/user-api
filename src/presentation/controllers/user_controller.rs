use crate::application::dto::create_user_dto::CreateUserDto;
use crate::application::use_cases::create_user::CreateUserUseCase;
use crate::application::use_cases::get_all_users::GetAllUsersUseCase;
use crate::application::use_cases::get_user::GetUserUseCase;
use crate::application::use_cases::remove_user::RemoveUserUseCase;
use crate::config::settings::Settings;
use crate::core::services::user_service_impl::UserServiceImpl;
use crate::infrastructure::database::factories::user_repository_factory::user_repository_factory;
use actix_web::{web, HttpResponse, Responder};

#[actix_web::get("/users")]
async fn get_users(data: web::Data<Settings>) -> impl Responder {
    let repository = user_repository_factory(&data.database_type);
    let service = Box::new(UserServiceImpl::new(repository));
    let get_all_users_use_case = GetAllUsersUseCase::new(service);
    HttpResponse::Ok().json(get_all_users_use_case.execute())
}

#[actix_web::get("/user/{id}")]
async fn get_user(data: web::Data<Settings>, id: web::Path<u32>) -> impl Responder {
    let repository = user_repository_factory(&data.database_type);
    let service = Box::new(UserServiceImpl::new(repository));
    let get_user_use_case = GetUserUseCase::new(service);
    match get_user_use_case.execute(*id) {
        Some(user) => HttpResponse::Ok().json(user),
        None => HttpResponse::NotFound().finish(),
    }
}

#[actix_web::post("/user")]
async fn create_user(data: web::Data<Settings>, dto: web::Json<CreateUserDto>) -> impl Responder {
    let repository = user_repository_factory(&data.database_type);
    let service = Box::new(UserServiceImpl::new(repository));
    let create_user_use_case = CreateUserUseCase::new(service);
    match create_user_use_case.execute(dto.into_inner()) {
        Ok(_) => HttpResponse::Created().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[actix_web::delete("/user/{id}")]
async fn delete_user(data: web::Data<Settings>, id: web::Path<u32>) -> impl Responder {
    let repository = user_repository_factory(&data.database_type);
    let service = Box::new(UserServiceImpl::new(repository));
    let remove_user_use_case = RemoveUserUseCase::new(service);
    match remove_user_use_case.execute(*id) {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub fn configure_user_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_user);
    cfg.service(create_user);
    cfg.service(delete_user);
    cfg.service(get_users);
}
