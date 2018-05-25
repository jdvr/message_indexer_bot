extern crate futures;
extern crate telegram_bot;
extern crate tokio_core;

use std::env;

use futures::Stream;
use tokio_core::reactor::Core;
use telegram_bot::*;

fn main() {
    let mut core = Core::new().unwrap();

    let token = env::var("TELEGRAM_BOT_TOKEN").unwrap();
    let api = match Api::configure(token).build(core.handle()) {
			Ok(api) => api,
			Err(e) => panic!(e),
	      };

    // Fetch new updates via long poll method
    let future = api.stream().for_each(|update| {
        println!("{:?}", update);
        // If the received update contains a new message...
        if let UpdateKind::Message(message) = update.kind {
            
            if let MessageKind::Text {ref data, ..} = message.kind {
                let index = data;
                &message.reply_to_message.map(|message_or_post| {
                    if let MessageOrChannelPost::Message(target_message) = *message_or_post {
                        if let MessageKind::Text {ref data, ..} = target_message.kind {
                            api.spawn(message.text_reply(
                                format!("I am going to index \n{}\n in '{}'", data, index)
                            ));
                        }
                    }
                }).unwrap_or_else(|| api.spawn(message.text_reply(format!("Not target message found"))));
            }
        }

        Ok(())
    });

    core.run(future).unwrap();
}
