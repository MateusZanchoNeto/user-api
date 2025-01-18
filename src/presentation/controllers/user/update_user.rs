use crate::application::dto::update_user_input_dto::UpdateUserInputDto;
use crate::application::use_cases::update_user::UpdateUserUseCase;
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
struct ErrorResponse {
    message: String,
}

#[derive(Debug, Serialize)]
struct Response {
    id: i32,
    name: String,
    email: String,
}

#[actix_web::put("/user/{id}")]
async fn update_user(
    data: web::Data<Settings>,
    id: web::Path<i32>,
    dto: web::Json<Body>,
) -> impl Responder {
    let repository = user_repository_factory(&data.connection_pool);
    let service = Box::new(UserServiceImpl::new(repository));
    let update_user_use_case = UpdateUserUseCase::new(service);
    let user = dto.into_inner();
    match update_user_use_case.execute(UpdateUserInputDto {
        id: *id,
        name: user.name,
        email: user.email,
    }) {
        Ok(updated_user) => HttpResponse::Ok().json(Response {
            id: updated_user.id,
            name: updated_user.name,
            email: updated_user.email,
        }),
        Err(e) => HttpResponse::NotFound().json(ErrorResponse { message: e }),
    }
}
