extern crate futures;
extern crate telegram_bot;
extern crate tokio_core;

use std::env;

use futures::Stream;
use tokio_core::reactor::Core;
use telegram_bot::*;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

struct MessageHasher {
    message: Message
}

impl Hash for  MessageHasher {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.message.id.hash(state);
        self.message.date.hash(state);
    }
}

fn to_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}


fn main() {
    let mut core = Core::new().unwrap();

    let token = env::var("TELEGRAM_BOT_TOKEN").unwrap();
    let api = match Api::configure(token).build(core.handle()) {
			Ok(api) => api,
			Err(e) => panic!(e),
	      };

    // Fetch new updates via long poll method
    let future = api.stream().for_each(|update| {
        // If the received update contains a new message...
        if let UpdateKind::Message(message) = update.kind {
            
            if let MessageKind::Text {ref data, ..} = message.kind {
                let index = data;
                let response = match message.clone().reply_to_message {
                    Some(message_or_post) => {
                        let mut r = String::from("No target message found");
                        if let MessageOrChannelPost::Message(target_message) = *message_or_post {    
                            if let MessageKind::Text {ref data, ..} = target_message.kind {
                                let message_id = to_hash(&MessageHasher { message: target_message.clone() });
                                let path = format!("/data/message_index/{}/{}/{}.text", &target_message.from.first_name, index, message_id);

                                r = format!("I am going to index \n{}\n in '{}'\n with id: {} in {}", data, index, message_id, path);
                            }
                        }
                        r
                    }
                    None => String::from("Not target message found"),
                };
                api.spawn(message.text_reply(response));
            }
        }

        Ok(())
    });

    core.run(future).unwrap();
}
