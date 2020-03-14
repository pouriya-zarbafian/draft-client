use crate::message::{self, ClientRequest, QueryResult, ClientResponse, Query};
use std::net::UdpSocket;

pub struct DraftClient {
    client_id: String,
    leader_id: String,
    servers: Vec<String>,
}
impl DraftClient {
    pub fn new(client_id: String, leader_id: String, servers: Vec<String>) -> DraftClient {
        DraftClient{
            client_id,
            leader_id,
            servers
        }
    }

    pub fn client_id(&self) -> String {
        self.client_id.clone()
    }

    pub fn leader_id(&self) -> String {
        self.leader_id.clone()
    }

    pub fn execute_query(&self, query: Query) -> QueryResult {

        // Create socket
        let socket = UdpSocket::bind(format!("{}:{}", self.client_id, 0)).unwrap();

        // Reuest id
        let request_id = String::from("TODO");

        // Build request
        let request = ClientRequest {
            client_id: socket.local_addr().unwrap().to_string(),
            request_id,
            query
        };

        // Serialize message to JSON
        let json = message::serialize(&request).unwrap();

        // Concatenate message type and json
        let data = [&[message::MESSAGE_TYPE_CLIENT_REQUEST], json.as_bytes()].concat();


        // Send data
        let _ = socket.send_to(&data, self.leader_id()).unwrap();

        let response = self.receive_response(socket);

        response.result
    }

    fn receive_response(&self, socket: UdpSocket) -> ClientResponse {
        // Receive buffer
        let mut buf = [0u8; 65535];

        let (amount, _src) = socket.recv_from(&mut buf).unwrap();
        let json: String = String::from_utf8_lossy(&buf[1..amount]).to_string();

        let response = message::deserialize(&json).unwrap();

        response
    }
}

