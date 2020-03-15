use crate::message::{self, ClientRequest, QueryResult, ClientResponse, Query};
use std::net::UdpSocket;
use std::error::Error;

pub struct DraftClient {
    client_id: String,
    leader_id: String,
    servers: Vec<String>,
}
impl DraftClient {
    pub fn new(client_id: String, servers: Vec<String>) -> Result<DraftClient, Box<dyn Error>> {
        let mut leader= None;

        for server in &servers {
            match Self::get_leader(client_id.clone(), server.clone()) {
                Some(value) => {
                    leader = Some(value);
                    break;
                },
                None => continue,
            }
        }

        match leader {
            Some(leader_id) => Ok(DraftClient{
                client_id,
                leader_id,
                servers
            }),
            None => Err("Could not find the leader")?
        }
    }

    fn client_id(&self) -> String {
        self.client_id.clone()
    }

    fn leader_id(&self) -> String {
        self.leader_id.clone()
    }

    pub fn save_value(&self, key: String, value: String) {
        let query = Query {
            action: message::Action::Save,
            key: key,
            value: Some(value)
        };

        println!("Query={:?}", query);
        let result = self.execute_query(query);
        println!("Result={:?}", result);
    }

    pub fn get_value(&self, key: String) -> Option<String> {
        let query = Query {
            action: message::Action::Get,
            key: key,
            value: None
        };

        println!("Query={:?}", query);
        let result = self.execute_query(query);
        println!("Result={:?}", result);

        result.value
    }

    pub fn delete_value(&self, key: String) {
        let query = Query {
            action: message::Action::Delete,
            key: key,
            value: None
        };

        println!("Query={:?}", query);
        let result = self.execute_query(query);
        println!("Result={:?}", result);
    }

    /// Query the servers to find the new leader.
    ///
    /// To be used in case the current leader crashes.
    fn search_leader(&mut self) {
        let mut leader= None;

        for server in &self.servers {
            match Self::get_leader(self.client_id(), server.clone()) {
                Some(value) => {
                    leader = Some(value);
                    break;
                },
                None => continue,
            }
        }

        match leader {
            Some(server) => self.leader_id = server,
            None => panic!("Could not find the leader")
        }
    }

    /// Send a dummy query to a server. If returns is OK we got the leader,
    /// otherwise the address of the leader.
    fn get_leader(client_id: String, server: String) -> Option<String> {
        // Empty query
        let query = Query{
            action: message::Action::Get,
            key: "test".to_string(),
            value: None
        };

        let result = Self::execute_query_internal(query, client_id, server.clone(), "0".to_string());

        match result.error {
            message::QUERY_RESULT_SUCCESS => {
                Some(server)
            },
            message::QUERY_RESULT_REDIRECT => {
                Some(result.value.unwrap())
            },
            _ => {
                eprintln!("{}: {:?}", result.message, result.value);
                None
            }
        }
    }


    fn execute_query(&self, query: Query) -> QueryResult {
        DraftClient::execute_query_internal(query,self.client_id(), self.leader_id(), "1".to_string())
    }

    fn execute_query_internal(query: Query, client_id: String, leader_id: String, request_id: String) -> QueryResult {

        // Create socket
        let socket = UdpSocket::bind(format!("{}:{}", client_id, 0)).unwrap();

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
        let _ = socket.send_to(&data, leader_id).unwrap();

        let response = DraftClient::receive_response(socket);

        response.result
    }

    fn receive_response(socket: UdpSocket) -> ClientResponse {
        // Receive buffer
        let mut buf = [0u8; 65535];

        let (amount, _src) = socket.recv_from(&mut buf).unwrap();
        let json: String = String::from_utf8_lossy(&buf[1..amount]).to_string();

        let response = message::deserialize(&json).unwrap();

        response
    }
}

