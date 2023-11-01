use std::f32::consts::E;

use async_trait::async_trait;
use databend_driver::{Client, Connection, Error};
use mobc::{Manager, Pool};
use once_cell::sync::OnceCell;

#[derive(Clone, Debug)]
pub struct DatabendConnectionManager {
    pub dns: String,
}

impl DatabendConnectionManager {
    pub fn create(dns: String) -> Self {
        DatabendConnectionManager { dns }
    }
}

#[async_trait]
impl Manager for DatabendConnectionManager {
    type Connection = Box<dyn Connection>;
    type Error = Error;
    async fn connect(&self) -> Result<Self::Connection, Self::Error> {
        let client = Client::new(self.dns.clone());
        client.get_conn().await
    }

    async fn check(&self, conn: Self::Connection) -> Result<Self::Connection, Self::Error> {
        conn.version().await?;
        Ok(conn)
    }
}
fn DATABEND_CONNECTION_POOL() -> &'static Pool<DatabendConnectionManager> {
    static INSTANCE: OnceCell<Pool<DatabendConnectionManager>> = OnceCell::new();
    INSTANCE.get_or_init(|| {
        let dns = option_env!("DATABEND_CONNECTION_STRING")
            .unwrap_or("databend://admin:1234Admin!@localhost:8000/default?sslmode=disable")
            .to_string();

        let pool_size = option_env!("DATABEND_POOL_SIZE")
            .unwrap_or("20")
            .parse::<usize>()
            .unwrap_or(20);
        let manager = DatabendConnectionManager::create(dns);
        Pool::builder().max_open(20).build(manager)
    })
}

pub async fn get_databend_connection() -> Result<Box<dyn Connection>, mobc::Error<Error>> {
    match DATABEND_CONNECTION_POOL().get().await {
        Ok(conn) => Ok(conn.into_inner()),
        Err(err) => {
            tracing::error!("Error: {:?}", err);
            Err(err)
        }
    }
}
