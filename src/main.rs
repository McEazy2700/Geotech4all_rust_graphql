use actix_web::{get, post, HttpResponse, Result, web, web::Data, HttpServer, App};
use async_graphql::http::GraphiQLSource;
use async_graphql_actix_web::{GraphQLResponse, GraphQLRequest};
use dotenv::dotenv;
use migration::{Migrator, MigratorTrait};
use std::env::var;

pub mod graphql;
pub mod db_manager;
pub mod middleware;

use crate::{
    db_manager::perform_migrations_from_entities,
    graphql::{schema::{AppSchema, build_schema}, context::AppContext},
    middleware::jwt_middleware::AuthUser
};

#[get("/graphql")]
async fn playground() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(GraphiQLSource::build().endpoint("/graphql")
        .finish())
    )
}

#[post("/graphql")]
async fn execute(schema: web::Data<AppSchema>, req: GraphQLRequest, auth_token: AuthUser) -> GraphQLResponse {
    schema.execute(req.into_inner().data(auth_token.clone())).await.into()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    
    let conn = match migration::db::DB::init().connect().await {
        Ok(conn) => conn,
        Err(err) => panic!("DB_CONNECTION_ERR: {err}"),
    };
    
    match Migrator::up(&conn,None).await {
        Ok(_) => {},
        Err(err) => panic!("MIGRATION_ERR: {err}"),
    };

    perform_migrations_from_entities(&conn).await;
    
    let schema = build_schema();
    
    
    let debug = var("DEBUG").unwrap_or(String::from("true"));
    let port = var("PORT").unwrap_or(String::from("8080"));
    
    let addr = if debug == "true" {
        format!("127.0.0.1:{port}")
    } else {
        format!("0.0.0.0:{port}")
    };
    
    println!("GraphIQL playground: {addr}");
    HttpServer::new(move ||{
        App::new()
            .app_data(Data::new(schema.clone()))
            .app_data(Data::new(AppContext{ db: conn.clone() }))
            .service(playground)
            .service(execute)
    })
    .bind(addr)?
    .run()
    .await
}
