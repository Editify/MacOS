use discord_rpc_client::{Client, Event};

pub fn connect() {
    let mut client = Client::new(1209416659874357258);

    client.on_event(Event::Ready, move |_ctx| {
        println!("Ready")
    });

    client.start();


        let _ = client.set_activity(|act| {
            act.state("Idle")
        });
    }