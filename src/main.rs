extern crate iron;
extern crate urlencoded;

use iron::prelude::*;
use iron::status;
use urlencoded::UrlEncodedBody;

fn main() {
    fn hello_world(req: &mut Request) -> IronResult<Response> {
      match req.get_ref::<UrlEncodedBody>() {
        Ok(ref hashmap) => println!("Parsed POST request body:\n {:?}", hashmap),
        Err(ref e) => println!("{:?}", e)
      };

      Ok(Response::with((status::Created, "Hello!")))
    }

    Iron::new(hello_world).http("localhost:3000").unwrap();
    println!("On 3000");
}
