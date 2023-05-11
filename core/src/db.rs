use serde::{Deserialize, Serialize};
use std::env;
use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
    Error, Surreal,
};

#[derive(Serialize, Deserialize, Debug)]
struct ConfigDatabase {
    host: String,
    port: u16,
    user: String,
    password: String,
    namespace: String,
    name: String,
}

impl ConfigDatabase {
    pub fn new() -> Self {
        let host = env::var("DATABASE_HOST").expect("DATABASE_PORT must be set");

        let port: u16 = env::var("DATABASE_PORT")
            .expect("DATABASE_PORT must be set")
            .parse()
            .unwrap();

        let user = env::var("DATABASE_USER").expect("DATABASE_USER must be set");

        let password = env::var("DATABASE_PASSWORD").expect("DATABASE_PASSWORD must be set");

        let namespace = env::var("DATABASE_NAMESPACE").expect("DATABASE_NAMESPACE must be set");

        let name = env::var("DATABASE_NAME").expect("DATABASE_NAME must be set");

        ConfigDatabase {
            host,
            port,
            user,
            password,
            namespace,
            name,
        }
    }
}

#[tracing::instrument]
pub async fn get_instance_db() -> Result<Surreal<Client>, Error> {
    let db_config = ConfigDatabase::new();

    // Connect to the database using websocket
    let db = Surreal::new::<Ws>(format!("{}:{}", db_config.host, db_config.port)).await?;

    db.signin(Root {
        username: &db_config.user,
        password: &db_config.password,
    })
    .await?;

    db.use_ns(db_config.namespace)
        .use_db(db_config.name)
        .await?;

    Ok(db)
}
