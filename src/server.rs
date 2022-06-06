use std::collections::HashMap;
use tokio::sync::mpsc;
use std::{pin::Pin};
use tokio::sync::broadcast;
use futures_core::Stream;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{transport::Server, Request, Response, Status};
use chat_app::{ChatMessage, User, UserList, JoinResponse, 
    Empty, chatter_server::{Chatter, ChatterServer}};

pub mod chat_app {
    tonic::include_proto!("chat"); //the string specified here must match the proto package name
}

#[derive(Debug, Default)]
pub struct MyChatter {}

// When a new user connects, we will create a pair of mpsc channel.
// Add the users and its related senders will be saved in below shared struct
#[derive(Debug)]
struct MasterRoom {
    rooms:  HashMap<String, mpsc::Sender<ChatMessage>>,
}

impl MasterRoom {
    fn new() -> MasterRoom {
        MasterRoom { rooms: HashMap::new() }
    }
}

#[tonic::async_trait]
impl Chatter for MyChatter {
    async fn join(&self, request: Request<User> //need to add these users to a room
    ) -> Result<Response<JoinResponse>, Status> {
        let res = chat_app::JoinResponse {
            error: 0, msg: format!("Hello {}!", request.into_inner().name).into()
        };
        Ok(Response::new(res))
    }

    async fn send_msg(&self, request:tonic::Request<ChatMessage>,
    )->Result<tonic::Response<Empty>,tonic::Status> {
        let reqMsg = request.into_inner();

        Ok(Response::new(chat_app::Empty {}))
    }

    type ReceiveMsgStream = ReceiverStream<Result<ChatMessage, Status>>;

    async fn receive_msg(&self,request:Request<Empty>,
    )->Result<Response<Self::ReceiveMsgStream>, Status> {
        //creating a queue or channel
        let (mut tx, rx) = mpsc::channel(4);
        //creating a new task 
        tokio::spawn(async move {
            //looping and sending our response using stream
            for _ in 0..4 {
                //sending response to our channel
                tx.send(Ok(ChatMessage { 
                    from:String::from("me"), msg: format!("hello"), time:String::from(""),
                })).await.unwrap();
            }
        });
        //returning our receiver so that tonic can listen on receiver and send the response to the client
        Ok(Response::new(ReceiverStream::new(rx)))
    }

    async fn get_all_users(&self,request:Request<Empty>,
    )->Result<tonic::Response<UserList>,Status> {
        todo!()
    }

}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let user = MyChatter::default();
    let channelMaps : MasterRoom = MasterRoom::new();

    Server::builder()
        .add_service(ChatterServer::new(user))
        .serve(addr)
        .await?;
    
    Ok(())
}
