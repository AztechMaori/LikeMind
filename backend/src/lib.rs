

mod routes;
use routes::create_routes;
use sea_orm::Database;


pub async fn run(db_uri: &str){
// connect SEAORM and DB 
let database:Result<sea_orm::prelude::DatabaseConnection, sea_orm::prelude::DbErr> = Database::connect(db_uri).await;








// run server 
    let app =  create_routes(database);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}




