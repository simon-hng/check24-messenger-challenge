use std::collections::HashMap;

use crate::actor_message::NotifyMessage;
use crate::chat::actor_message::{Connect, Disconnect, Notification};
use actix::prelude::*;
use actix_web::cookie::Key;

pub struct MessageServer {
    sessions: HashMap<i32, Recipient<Notification>>,
    _key: Key,
}

impl MessageServer {
    pub fn new(key: Key) -> MessageServer {
        MessageServer {
            _key: key,
            sessions: HashMap::new(),
        }
    }
}

impl Actor for MessageServer {
    type Context = Context<Self>;
}

impl Handler<Connect> for MessageServer {
    type Result = i32;

    fn handle(&mut self, msg: Connect, _ctx: &mut Self::Context) -> Self::Result {
        /*
        TODO: I wanted to check the cookie but this is getting out of hand so we just
              believe the client that it is what they say they are
         */

        log::info!("Added {} to the sessions", msg.id);
        self.sessions.insert(msg.id.to_owned(), msg.addr);

        msg.id
    }
}

impl Handler<Disconnect> for MessageServer {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _ctx: &mut Self::Context) -> Self::Result {
        self.sessions.remove(&msg.id);
    }
}

impl Handler<Notification> for MessageServer {
    type Result = ();

    fn handle(&mut self, msg: Notification, ctx: &mut Self::Context) -> Self::Result {
        log::info!("Forwarding message {:?}", msg);

        match msg {
            Notification::Message(NotifyMessage { recipient_id, .. }) => {
                let recipient = self
                    .sessions
                    .get(&recipient_id)
                    .ok_or("Recipient not found")
                    .expect("TODO return error");

                recipient
                    .send(msg)
                    .into_actor(self)
                    .then(|_res, _act, _ctx| fut::ready(()))
                    .wait(ctx);
            }
            _ => {}
        }
    }
}
