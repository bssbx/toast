use once_cell::sync::Lazy;
use std::sync::Mutex;

use mysql::prelude::*;
use mysql::*;

use crate::config::*;
use crate::user::UserInfo;

//配置数据库连接
static CONN: Lazy<Mutex<PooledConn>> = Lazy::new(|| {
    let pool_temp = Pool::new(Opts::from_url(DB_URI).unwrap()).unwrap();
    let conn_temp = pool_temp.get_conn().unwrap();
    Mutex::new(conn_temp)
});

#[derive(Debug)]
pub struct DB {}

impl DB {
    pub fn init_db_conn() {
        println!("{:#?}", CONN);
    }
}
impl DB {
    //根据网址查询用户数据
    // pub fn query_with_domain(domain: String) -> Option<UserModel> {
    //     println!("db domain -> {}", &domain);
    //     let _model = CONN
    //         .lock()
    //         .unwrap()
    //         .query_first(format!("select * from userInfo where domain = {}", domain))
    //         .map(|row| {
    //             row.map(
    //                 |(id, domain, panel_type, client_type, begin_date, end_date)| {
    //                     UserModel::new(id, domain, panel_type, begin_date, end_date, client_type)
    //                 },
    //             )
    //         });
    //     _model.unwrap()
    // }
}
