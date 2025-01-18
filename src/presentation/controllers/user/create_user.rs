use crate::application::dto::create_user_input_dto::CreateUserInputDto;
use crate::application::use_cases::create_user::CreateUserUseCase;
use crate::config::settings::Settings;
use crate::core::services::user_service_impl::UserServiceImpl;
use crate::infrastructure::database::factories::user_repository_factory::user_repository_factory;
use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
struct Body {
    name: String,
    email: String,
}

#[derive(Debug, Serialize)]
struct Response {
    id: i32,
    name: String,
    email: String,
}

#[derive(Debug, Serialize)]
struct ErrorResponse {
    message: String,
}

#[actix_web::post("/user")]
async fn create_user(data: web::Data<Settings>, dto: web::Json<Body>) -> impl Responder {
    let repository = user_repository_factory(&data.connection_pool);
    let service = Box::new(UserServiceImpl::new(repository));
    let create_user_use_case = CreateUserUseCase::new(service);
    let user = dto.into_inner();
    match create_user_use_case.execute(CreateUserInputDto {
        name: user.name,
        email: user.email,
    }) {
        Ok(created_user) => HttpResponse::Created().json(Response {
            id: created_user.id,
            name: created_user.name,
            email: created_user.email,
        }),
        Err(e) => HttpResponse::BadRequest().json(ErrorResponse { message: e }),
    }
}
