#[macro_use]
extern crate nickel;

use nickel::{Nickel, HttpRouter};

struct ServerData;

fn main() {
    let mut server = Nickel::with_data(ServerData);

    server.get("/",
               middleware! { |_, res| <ServerData>
                return res.send("foo")
    });

    server.listen("0.0.0.0:0");
}
