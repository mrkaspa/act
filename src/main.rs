extern crate actix;

use actix::prelude::*;

struct Ping(Addr<ActPingPong>);

impl Message for Ping {
    type Result = ();
}
struct Pong(Addr<ActPingPong>);

impl Message for Pong {
    type Result = ();
}

struct ActPingPong {
    counter: i32,
}

impl ActPingPong {
    fn new() -> Self {
        ActPingPong { counter: 0 }
    }
}

impl Actor for ActPingPong {
    type Context = Context<Self>;
}

impl Handler<Ping> for ActPingPong {
    type Result = ();

    fn handle(&mut self, msg: Ping, ctx: &mut Context<Self>) -> Self::Result {
        println!("PING {}", self.counter);
        self.counter += 1;
        let sent = msg.0.send(Pong(ctx.address()));
        Arbiter::spawn(sent.map(|_| ()).map_err(|_| ()));
    }
}

impl Handler<Pong> for ActPingPong {
    type Result = ();

    fn handle(&mut self, msg: Pong, ctx: &mut Context<Self>) -> Self::Result {
        println!("PONG {}", self.counter);
        self.counter += 1;
        let sent = msg.0.send(Ping(ctx.address()));
        Arbiter::spawn(sent.map(|_| ()).map_err(|_| ()));
    }
}

fn main() -> std::result::Result<(), std::io::Error> {
    let system = System::new("sys");
    // start new actor
    let addr1 = ActPingPong::new().start();
    let addr2 = ActPingPong::new().start();
    let _res = addr1.send(Ping(addr2));

    system.run()
}
