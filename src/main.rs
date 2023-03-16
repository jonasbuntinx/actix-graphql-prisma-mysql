use actix_web::{guard, web, web::Data, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};

mod prisma;
mod graphql;

use graphql::schema::{build_schema, AppSchema};

async fn graphql_handler(schema: web::Data<AppSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn graphql_playground() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(GraphQLPlaygroundConfig::new("/")))
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let schema = build_schema().await;

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(schema.clone()))
            .service(web::resource("/").guard(guard::Post()).to(graphql_handler))
            .service(web::resource("/").guard(guard::Get()).to(graphql_playground))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}