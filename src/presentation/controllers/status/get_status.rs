// use crate::application::dto::read_user_input_dto::ReadUserInputDto;
// use crate::application::dto::read_user_output_dto::ReadUserOutputDto;
// use crate::application::use_cases::read_user::GetUserUseCase;
// use crate::config::settings::Settings;
// use crate::core::services::user_service_impl::UserServiceImpl;
// use crate::infrastructure::database::factories::user_repository_factory::user_repository_factory;
// use actix_web::{web, HttpResponse, Responder};
// use serde::Serialize;
//
// #[derive(Debug, Serialize)]
// struct Body {
//     id: i32,
//     name: String,
//     email: String,
// }
//
// impl From<ReadUserOutputDto> for Body {
//     fn from(dto: ReadUserOutputDto) -> Self {
//         Body {
//             id: dto.id,
//             name: dto.name,
//             email: dto.email,
//         }
//     }
// }
//
// #[actix_web::get("/user/{id}")]
// async fn get_user(data: web::Data<Settings>, id: web::Path<i32>) -> impl Responder {
//     let repository = user_repository_factory(&data.connection_pool);
//     let service = Box::new(UserServiceImpl::new(repository));
//     let get_user_use_case = GetUserUseCase::new(service);
//     let dto = ReadUserInputDto { id: *id };
//     match get_user_use_case.execute(dto) {
//         Some(user) => HttpResponse::Ok().json(Body::from(user)),
//         None => HttpResponse::NotFound().finish(),
//     }
// }

use crate::config::settings::Settings;
use crate::core::services::status_service_impl::StatusServiceImpl;
use crate::infrastructure::database::factories::status_repository_factory::status_repository_factory;
use actix_web::{web, Responder};

#[actix_web::get("/status")]
async fn get_status(data: web::Data<Settings>) -> impl Responder {
    let repository = status_repository_factory(&data.connection_pool);
    let service = Box::new(StatusServiceImpl::new(repository));
    let status = service.get_status(&data.database_config.database).unwrap();
    let get_status_use_case = GetStatusUseCase::new(service);
}
