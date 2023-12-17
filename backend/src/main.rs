    use dotenvy_macro::dotenv; 
    use dotenvy::dotenv; 
    use backend::run;
    
    #[tokio::main]
    async fn main() {
        // build our application with a single route
        dotenv().ok(); 
        let db_uri = dotenv!("DATABASE_URL");
        run(db_uri).await;


    }

