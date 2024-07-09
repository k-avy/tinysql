mod tinyhttp_db_pooled_rayon;
mod tide_db_embeded;
mod tide_db_pooled_r2d2;
mod tinyhttp_routes_crud;
mod tide_routes_crud;
mod tinyhttp_db_hosted;
mod tinyhttp_rayon_db_pooled_r2d2;

use std::{io::{self, Write}};

use async_std::task;
use tinyhttp_db_hosted::tiny_db_hosted;
use tinyhttp_db_pooled_rayon::tiny_pooled;
use tide_db_embeded::tide_embedded;
use tide_db_pooled_r2d2::tide_pooled_db;
use tinyhttp_routes_crud::tinyhttp_crud;
use tide_routes_crud::tide_crud;
use tinyhttp_rayon_db_pooled_r2d2::server_db_pooled;

fn call_function(name: &str) {
    match name {
        "tiny_db_hosted" => tiny_db_hosted(),
        "tiny_pooled" => tiny_pooled(),
        "tide_embedded" => task::block_on(tide_embedded()),
        "tide_pooled_db" => task::block_on(tide_pooled_db()),
        "tinyhttp_crud" => tinyhttp_crud(),
        "tide_crud" => task::block_on(tide_crud()).unwrap(),
        "server_db_pooled" => server_db_pooled(),
        _ => println!("Function not found"),
    }
}
fn main() {

    let mut input = String::new();

    println!("Enter the function name to call:");

    // Flush stdout to ensure the prompt is shown before input
    io::stdout().flush().unwrap();

    // Read the input from the user
    io::stdin().read_line(&mut input).expect("Failed to read input");

    // Trim the input to remove any leading/trailing whitespace
    let input = input.trim();

       
    // Call the corresponding function
    call_function(input);

}

