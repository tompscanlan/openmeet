use bb8::{Pool, PooledConnection};
use cassandra_cpp::{Cluster, Session};
use rocket::request::{self, FromRequest};
use rocket::{Request, State};
use rocket::outcome::Outcome;
use std::ops::{Deref, DerefMut};
use rocket::async_trait;
use rocket::http::Status;

pub struct CassandraPool(Pool<CassandraConnectionManager>);

impl CassandraPool {
    pub async fn new(contact_points: &str) -> Result<Self, bb8::RunError<cassandra_cpp::Error>> {
        let manager = CassandraConnectionManager::new(contact_points);
        let pool = Pool::builder().build(manager).await?;
        Ok(CassandraPool(pool))
    }
    pub async fn get_connection(&self) -> Result<PooledConnection<'_, CassandraConnectionManager>, Status> {
        self.0.get().await.map_err(|e| {
            eprintln!("Failed to get connection: {}", e);
            Status::InternalServerError
        })
    }
}

pub struct CassandraConnection(pub PooledConnection<'static, CassandraConnectionManager>);

impl Deref for CassandraConnection {
    type Target = Session;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for CassandraConnection {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub struct CassandraConnectionManager {
    contact_points: String,
}

impl CassandraConnectionManager {
    pub fn new(contact_points: &str) -> Self {
        CassandraConnectionManager {
            contact_points: contact_points.to_string(),
        }
    }
}

#[async_trait]
impl bb8::ManageConnection for CassandraConnectionManager {
    type Connection = Session;
    type Error = cassandra_cpp::Error;

    async fn connect(&self) -> Result<Self::Connection, Self::Error> {
        let mut cluster = Cluster::default();
        cluster.set_contact_points(&self.contact_points)?;
        cluster.connect().await
    }

    async fn is_valid(&self, conn: &mut Self::Connection) -> Result<(), Self::Error> {
        let statement = conn.statement("SELECT now() FROM system.local");
        let result = match statement.execute().await {
            Ok(result) => result,
            Err(e) => return Err(e),
        };
        
        if result.row_count() == 1 {
            return Ok(());
        } else {
            return Err("Invalid result".to_string().into());
        }
    }

    fn has_broken(&self, _: &mut Self::Connection) -> bool {
        false
    }
}


use std::cell::RefCell;

pub struct CassandraConn<'r>(RefCell<Option<PooledConnection<'static, CassandraConnectionManager>>>, &'r Request<'r>);

impl<'r> CassandraConn<'r> {
    pub fn get(&self) -> Result<std::cell::Ref<'_, Session>, Status> {
        self.0.borrow().as_ref().map(|conn| std::cell::Ref::map(conn, |c| c.deref()))
            .ok_or(Status::InternalServerError)
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for CassandraConn<'r> {
    type Error = Status;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let pool = request.rocket().state::<CassandraPool>().ok_or(Status::InternalServerError).unwrap();
        
        match pool.get_connection().await {
            Ok(conn) => Outcome::Success(CassandraConn(RefCell::new(Some(conn)), request)),
            Err(_) => Outcome::Error((Status::ServiceUnavailable, Status::ServiceUnavailable)),
        }
    }
}