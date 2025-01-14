use crate::application::dto::read_user_output_dto::ReadUserOutputDto;
use crate::application::use_cases::read_all_users::GetAllUsersUseCase;
use crate::config::settings::Settings;
use crate::core::services::user_service_impl::UserServiceImpl;
use crate::infrastructure::database::factories::user_repository_factory::user_repository_factory;
use actix_web::{web, HttpResponse, Responder};
use serde::Serialize;

#[derive(Debug, Serialize)]
struct Body {
    id: i32,
    name: String,
    email: String,
}

impl From<ReadUserOutputDto> for Body {
    fn from(dto: ReadUserOutputDto) -> Self {
        Body {
            id: dto.id,
            name: dto.name,
            email: dto.email,
        }
    }
}

#[actix_web::get("/users")]
async fn get_users(data: web::Data<Settings>) -> impl Responder {
    let repository = user_repository_factory(&data.connection_pool);
    let service = Box::new(UserServiceImpl::new(repository));
    let get_all_users_use_case = GetAllUsersUseCase::new(service);
    HttpResponse::Ok().json(
        get_all_users_use_case
            .execute()
            .into_iter()
            .map(Body::from)
            .collect::<Vec<Body>>(),
    )
}
