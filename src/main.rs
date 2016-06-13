#[macro_use]
extern crate nickel;

use nickel::{Nickel, HttpRouter, Router};

struct ServerData;

fn main() {
    let mut server = Nickel::with_data(ServerData);

    server.utilize(explicit_router());

    server.listen("0.0.0.0:0");
}

fn explicit_router() -> Router<ServerData> {
    let mut router = Nickel::router();

    router.get("/",
               middleware! { |_, res| <ServerData>
                 // if you change this to just:
                 //
                 // res.send("foo")
                 //
                 // it will not compile
                return res.send("foo")
            });

    router
}
