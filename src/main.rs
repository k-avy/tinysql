mod threadpooling;
// mod server_db_pool;
mod tidensled;
mod tiny_sqlx;
mod db_insert;
// use threadpooling::tiny_fn;
// use server_db_pool::start_server;
// use tiny_sqlx::tinysqlserver;
// use tidensled::tidendb;
use db_insert::tiny_fn;
fn main() {
   tiny_fn();
//    start_server();
    
//    async_std::task::block_on(tinysqlserver()).unwrap();
    // async_std::task::block_on(tidendb()).unwrap();

}
