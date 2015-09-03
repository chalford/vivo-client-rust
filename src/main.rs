extern crate iron;
extern crate urlencoded;
extern crate amqp;

use amqp::session::Session;
use amqp::table;
use amqp::protocol;
use amqp::basic::Basic;

use iron::prelude::*;
use iron::status;
use urlencoded::UrlEncodedBody;

fn fire_event() {
    println!("FIRING EVENT");

    let mut session = Session::open_url("amqp://dhalfxhj:TJ-8jR4Z9ikYuFVy2mULh75CjnisKSmL@bunny.cloudamqp.com/dhalfxhj").unwrap();
    let mut channel = session.open_channel(1).unwrap();
    
    let routing_key = "training-microservices-routing";
    
    channel.basic_publish("training-microservices", routing_key, true, false,
        protocol::basic::BasicProperties{ content_type: Some("text".to_string()), ..Default::default()}, (b"Hello from rust!").to_vec());

}

fn main() {
    fn hello_world(req: &mut Request) -> IronResult<Response> {
      match req.get_ref::<UrlEncodedBody>() {
        Ok(ref hashmap) => {
            println!("Parsed POST request body:\n {:?}", hashmap);
            fire_event();
        },
        Err(ref e) => println!("{:?}", e)
      };

      Ok(Response::with((status::Created, "Hello!")))
    }

    Iron::new(hello_world).http("localhost:3000").unwrap();
    println!("On 3000");
}
