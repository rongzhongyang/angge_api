use rbatis::rbatis::RBatis;
use rbatis::rbdc::pool::{ConnectionManager, Pool};
use rbdc_mysql::MysqlDriver;
use rbdc_sqlite::SqliteDriver;
use rbdc_pool_fast::FastPool;

pub async fn init_db(url: &str) -> RBatis {
    let rb = RBatis::new();

    if url.starts_with("mysql://") {
        let manager = ConnectionManager::new(MysqlDriver {}, url).expect("create mysql manager error");
        let pool = FastPool::new(manager).expect("create mysql pool error");
        rb.init_pool(pool).expect("init mysql pool error");
    } else if url.starts_with("sqlite://") {
        let manager = ConnectionManager::new(SqliteDriver {}, url).expect("create sqlite manager error");
        let pool = FastPool::new(manager).expect("create sqlite pool error");
        rb.init_pool(pool).expect("init sqlite pool error");
    } else {
        panic!("Unsupported database driver. URL must start with 'mysql://' or 'sqlite://'");
    }
    rb
}
