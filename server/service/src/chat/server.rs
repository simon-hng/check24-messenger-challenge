use std::collections::HashMap;

use crate::chat::actor_message::*;
use actix::prelude::*;
use actix_web::cookie::Key;
use sea_orm::prelude::Uuid;

pub struct NotificationServer {
    sessions: HashMap<Uuid, Recipient<Notification>>,
    _key: Key,
}

impl NotificationServer {
    pub fn new(key: Key) -> NotificationServer {
        NotificationServer {
            _key: key,
            sessions: HashMap::new(),
        }
    }
}

impl Actor for NotificationServer {
    type Context = Context<Self>;
}

impl Handler<Connect> for NotificationServer {
    type Result = ();

    fn handle(&mut self, msg: Connect, _ctx: &mut Self::Context) -> Self::Result {
        /*
        TODO: I wanted to check the cookie but this is getting out of hand so we just
              believe the client that it is what they say they are
         */

        log::info!("Added {} to the sessions", msg.id);
        self.sessions.insert(msg.id.to_owned(), msg.addr);
    }
}

impl Handler<Disconnect> for NotificationServer {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _ctx: &mut Self::Context) -> Self::Result {
        self.sessions.remove(&msg.id);
    }
}

impl Handler<Notification> for NotificationServer {
    type Result = ();

    fn handle(&mut self, msg: Notification, _ctx: &mut Self::Context) -> Self::Result {
        log::info!("Forwarding message {:?}", msg);

        // TODO: this looks like it could be drier. Perhaps separate the enums?
        let recipient_id = match msg {
            Notification::Message(NotifyMessage { recipient_id, .. }) => recipient_id,
            Notification::Read(NotifyRead {
                sender_id: recipient_id,
                ..
            }) => recipient_id,
            _ => {
                panic!("This should not happen")
            }
        };

        let recipient = self.sessions.get(&recipient_id);
        if let Some(recipient) = recipient {
            recipient.do_send(msg)
        }
    }
}
