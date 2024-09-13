use zero2prod::{configuration::get_configuration, startup::run};
use sqlx::PgPool;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // Panic if we can't read configuration
    let configuration = get_configuration().expect("Failed to read configuration !!!");
    let connection_pool =PgPool::connect(
            &configuration.database.connection_string()
        )
        .await
        .expect("Failed to connect to Postgres.");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    run(listener, connection_pool)?.await
}


/*

@ close of ch. 3...

Re: the connection pool::
See how 
- it's est here with the sqlx call using outr config stuff
- and then passed to Run where it wind sup in AppData
- from which it can be pulled into diff. spots as if it were DI .
 
 */