mod message;
mod client;

use crate::client::DraftClient;

fn main() {

    // Client id requires IP as it is used to bind sockets
    let client_id = String::from("127.0.0.1");

    // List of all servers
    let servers = vec![String::from("127.0.0.1:7777"), String::from("127.0.0.1:8888"), String::from("127.0.0.1:9999")];

    // Create client
    let client = DraftClient::new(client_id, servers).unwrap();

    let key = String::from("the key");
    let initial_value = String::from("the initial value");

    // Insert a value
    client.save_value(key.clone(), initial_value.clone());

    // Retrieve inserted value
    let initial_result = client.get_value(key.clone()).unwrap();

    assert_eq!(initial_value, initial_result);

    let updated_value = String::from("the updated value");

    // Update value
    client.save_value(key.clone(), updated_value.clone());

    // Retrieve updated value
    let updated_result = client.get_value(key.clone()).unwrap();

    assert_eq!(updated_value, updated_result);

    // Delete value
    client.delete_value(key.clone());

    let deleted_value = client.get_value(key.clone());

    assert_eq!(deleted_value, None);
}
