mod variables;
mod control_statements;
mod loops;
mod switch;
mod enums;
mod structure;
mod tuples;
mod ownerships;
mod implementation;
mod pointers;
mod data_structures;
mod strings;
mod io;
mod traits;
mod concurrency;
mod println;
mod constants;
mod functions;
mod closures;

use futures::executor::block_on;

#[tokio::main]
async fn main() {
    // block_on(concurrency::asyn_await::run());

    io::run();
    concurrency::threads::run()
}
