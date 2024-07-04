// mod threadpooling;
// mod server_db_pool;

mod tiny_sqlx;

// use threadpooling::tiny_fn;
// use server_db_pool::start_server;
use tiny_sqlx::tinysqlserver;
fn main() {
//    tiny_fn();
//    start_server();
   async_std::task::block_on(tinysqlserver()).unwrap();
}
