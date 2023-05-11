use common::{config::Config, environment::is_development};
use dotenv::dotenv;
use std::env;

use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
    Error, Surreal,
};

#[tracing::instrument]
pub async fn get_instance_db() -> Result<Surreal<Client>, Error> {
    let config: Config = Config::new();

    if is_development() {
        dotenv::from_filename(config.dotenv_path).ok();
    } else {
        dotenv().ok();
    }

    let host = env::var("DATABASE_HOST").expect("DATABASE_PORT must be set");
    let port: u16 = env::var("DATABASE_PORT")
        .expect("DATABASE_PORT must be set")
        .parse()
        .unwrap();
    let user = env::var("DATABASE_USER").expect("DATABASE_USER must be set");
    let password = env::var("DATABASE_PASSWORD").expect("DATABASE_PASSWORD must be set");
    let namespace = env::var("DATABASE_NAMESPACE").expect("DATABASE_NAMESPACE must be set");
    let name = env::var("DATABASE_NAME").expect("DATABASE_NAME must be set");

    // Connect to the server
    let db = Surreal::new::<Ws>(format!("{}:{}", host, port)).await?;

    // Signin as a namespace, database, or root user
    db.signin(Root {
        username: &user,
        password: &password,
    })
    .await?;

    db.use_ns(namespace).use_db(name).await?;

    Ok(db)
}
