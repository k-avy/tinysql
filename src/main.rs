// mod threadpooling;
// // mod server_db_pool;
// mod tidensled;
// mod tiny_sqlx;
// mod db_insert;
// mod tide_db;
// mod tide_qury;
// mod tiny_host;
mod tide_host;

// use threadpooling::tiny_fn;
// use server_db_pool::start_server;
// use tiny_sqlx::tinysqlserver;
// use tidensled::tidendb;
// use db_insert::tiny_fn;
// use tide_db::tide_fn;
// use tide_qury::tide_fn;
// use tiny_host::tiny_fn;
use tide_host::tide_fn;
fn main() {

    // tide_fn();
    // tiny_fn();
   async_std::task::block_on(tide_fn());

//    start_server();
    
//    async_std::task::block_on(tinysqlserver()).unwrap();
    // async_std::task::block_on(tidendb()).unwrap();

}
