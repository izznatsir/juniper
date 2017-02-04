extern crate iron;
extern crate mount;
extern crate logger;
extern crate rustc_serialize;
#[macro_use] extern crate juniper;

use std::env;

use mount::Mount;
use logger::Logger;
use iron::prelude::*;
use juniper::EmptyMutation;
use juniper::iron_handlers::{GraphQLHandler, GraphiQLHandler};
use juniper::tests::model::Database;

fn context_factory(_: &mut Request) -> Database {
    Database::new()
}

fn main() {
    let mut mount = Mount::new();

    let graphql_endpoint = GraphQLHandler::new(
        context_factory,
        Database::new(),
        EmptyMutation::<Database>::new(),
    );
    let graphiql_endpoint = GraphiQLHandler::new("/graphql");

    mount.mount("/", graphiql_endpoint);
    mount.mount("/graphql", graphql_endpoint);

    let mut chain = Chain::new(mount);

    setup_logger(&mut chain);

    let host = env::var("LISTEN").unwrap_or("0.0.0.0:8080".to_owned());
    println!("GraphQL server started on {}", host);
    Iron::new(chain).http(host.as_str()).unwrap();
}

// Temporary fix - Iron's logger middleware does not work on Windows
#[cfg(not(windows))]
fn setup_logger(chain: &mut Chain) {
    let (logger_before, logger_after) = Logger::new(None);
    chain.link_before(logger_before);
    chain.link_after(logger_after);
}

#[cfg(windows)]
fn setup_logger(_: &mut Chain) {
}
