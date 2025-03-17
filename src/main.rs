use warp;

mod driver;
mod handler;
mod model;
mod routes;
mod libs;
mod common;
mod middleware;

use driver::db;

#[tokio::main]
async fn main() {
    let db = db::connect_db().await;
    // let customer_routes = routes::customer_routes(db);
    // handle 2 routes
    let routes = routes::routes(db);

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3000))
        .await;
}