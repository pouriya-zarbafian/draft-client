# draft-client
Rust client for draft: https://github.com/pouriya-zarbafian/draft

# Cluster setup
Setup and start a cluster of draft nodes (see https://github.com/pouriya-zarbafian/draft).

# Client setup
In `main.rs` set the value of the variable `client_id` to the IP of client host, and set the value of the variable `servers` to the list of servers in the cluster.
```
// Client id requires IP as it is used to bind sockets
let client_id = String::from("127.0.0.1");

// List of all servers
let servers = vec![String::from("127.0.0.1:7777"), String::from("127.0.0.1:8888"), String::from("127.0.0.1:9999")];
```

# Run
```
cd draft-config
cargo run
```

# Note
The leader is initialized when calling `DraftClient::new()`. Currently if the leader crashes, the client is not able to recover the new leader dynamically.

