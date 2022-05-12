use diesel::{r2d2::{Pool, ConnectionManager}, pg::PgConnection};

pub type PgPool = Pool<ConnectionManager<PgConnection>>;

pub fn get_connection_pool(db_host: String) -> PgPool {
    let manager = ConnectionManager::<PgConnection>::new(db_host);
    Pool::builder()
        .build(manager)
        .expect("Failed create pool.")
}

