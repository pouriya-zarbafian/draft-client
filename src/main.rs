mod message;
mod client;

use rand::Rng;

use crate::message::{ClientRequest, ClientResponse, Query};
use crate::client::DraftClient;
use std::sync::Arc;

fn main() {

    // Client id requires IP as it is used to bind sockets
    let client_id = String::from("127.0.0.1");

    // Leader id is used to find leader
    // TODO: handle follower redirection to leader
    let leader_id = String::from("127.0.0.1:8888");

    // List of all servers
    let servers = vec![String::from("127.0.0.1:7777"), String::from("127.0.0.1:8888"), String::from("127.0.0.1:9999")];

    // Create client
    let client = DraftClient::new(client_id, leader_id, servers);

    let client = Arc::new(client);

    // First insert values
    execute_inserts(client, 1, 6);
    //execute_inserts(client, 6, 11);
    //execute_inserts(client, 11, 16);
    //execute_inserts(client, 16, 22);

    // Then retrieve inserted values
    // execute_gets(client, 1, 22);

}

fn execute_inserts(client: Arc<DraftClient>, from: i32, to: i32) {
    let mut handles = Vec::new();

    for i in from..to {
        let client = Arc::clone(&client);

        let handle = std::thread::Builder::new()
            .name(format!("thread {}", i))
            .spawn(move || {
                let query = Query {
                    action: message::Action::Save,
                    key: i.to_string(),
                    value: i.to_string().repeat(2)
                };

                let result = client.execute_query(query);
                println!("Query={} -> Result={}", i, result.message);

            })
            .unwrap();

        handles.push(handle);
    }

    for h in handles {
        h.join().unwrap();
    }
}

fn execute_gets(client: Arc<DraftClient>, from: i32, to: i32) {
    let mut handles = Vec::new();

    for i in from..to {
        let client = Arc::clone(&client);

        let handle = std::thread::Builder::new()
            .name(format!("thread {}", i))
            .spawn(move || {
                let query = Query {
                    action: message::Action::Get,
                    key: i.to_string(),
                    value: "".into()
                };

                let result = client.execute_query(query);
                println!("Query={} -> Result={:?}", i, result);

            })
            .unwrap();

        handles.push(handle);
    }

    for h in handles {
        h.join().unwrap();
    }
}
