use crate::application::dto::delete_user_input_dto::DeleteUserInputDto;
use crate::application::use_cases::delete_user::RemoveUserUseCase;
use crate::config::settings::Settings;
use crate::core::services::user_service_impl::UserServiceImpl;
use crate::infrastructure::database::factories::user_repository_factory::user_repository_factory;
use actix_web::{web, HttpResponse, Responder};
use serde::Serialize;

#[derive(Debug, Serialize)]
struct ErrorResponse {
    message: String,
}

#[actix_web::delete("/user/{id}")]
async fn delete_user(data: web::Data<Settings>, id: web::Path<i32>) -> impl Responder {
    let repository = user_repository_factory(&data.connection_pool);
    let service = Box::new(UserServiceImpl::new(repository));
    let remove_user_use_case = RemoveUserUseCase::new(service);
    let dto = DeleteUserInputDto { id: *id };
    match remove_user_use_case.execute(dto) {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => HttpResponse::NotFound().json(ErrorResponse { message: e }),
    }
}
