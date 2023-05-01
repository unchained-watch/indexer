use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
    Error, Surreal,
};

pub async fn get_instance_db() -> Result<Surreal<Client>, Error> {
    // Connect to the server
    let db = Surreal::new::<Ws>("127.0.0.1:8000").await?;

    // Signin as a namespace, database, or root user
    db.signin(Root {
        username: "user",
        password: "password",
    })
    .await?;

    db.use_ns("bc-index").use_db("bc-index").await?;

    Ok(db)
}
