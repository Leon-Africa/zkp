use num_bigint::BigUint;
use tonic::{transport::Server, Code, Request, Response, Status};

pub mod zkp_auth {
    include!("./zkp_auth.rs");
}

use zkp_auth::{auth_server::{Auth, AuthServer}, RegisterRequest, RegisterResponse, AuthenticationChallengeRequest, AuthenticationAnswerResponse, AuthenticationAnswerRequest, AuthenticationChallengeResponse};

#[derive(Debug, Default)]
struct  AuthImpl {}

#[derive(Debug, Default)]
pub struct UserInfo {
    pub user_name: String,

    //registration
    pub y1: BigUint,
    pub y2: BigUint,

    //authorization
    pub r1: BigUint,
    pub r2: BigUint,

    //verification
    pub c: BigUint,
    pub s: BigUint,
    pub session_id: BigUint

}
#[tonic::async_trait]
impl Auth for AuthImpl {
    async fn register(&self, request: Request<RegisterRequest>) -> Result<Response<RegisterResponse>, Status> {

        //extract request
        let request = request.into_inner();

        //get request information
        let username = request.user;
        let y1 = BigUint::from_bytes_be(&request.y1);
        let y2 = BigUint::from_bytes_be(&request.y2);

        //intialize structure
        let mut user_info = UserInfo::default();
        user_info.user_name = username;
        user_info.y1 = y1;
        user_info.y2 = y2;

        Ok(Response::new(RegisterResponse {  })) //defined in protobuf
    }
    
    async fn create_authentication_challenge(&self, request: Request<AuthenticationChallengeRequest>) -> Result<Response<AuthenticationChallengeResponse>, Status> {
        todo!()
    }

    async fn verify_authentication(&self, request: Request<AuthenticationAnswerRequest>) -> Result<Response<AuthenticationAnswerResponse>, Status> {
        todo!()
    }
}

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:777".to_string();

    println!("Running the server in {}", addr);

    let auth_impl = AuthImpl::default();

    Server::builder().add_service(AuthServer::new(auth_impl)).serve(addr.parse().expect("could not convert addr")).await.unwrap();
}
