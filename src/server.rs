use tonic::{transport::Server, Code, Request, Response, Status};

pub mod zkp_auth {
    include!("./zkp_auth.rs");
}

use zkp_auth::{auth_server::{Auth, AuthServer}, RegisterRequest, RegisterResponse, AuthenticationChallengeRequest, AuthenticationAnswerResponse, AuthenticationAnswerRequest, AuthenticationChallengeResponse};

#[derive(Debug, Default)]

struct  AuthImpl {}

#[tonic::async_trait]
impl Auth for AuthImpl {
    async fn register(&self, request: Request<RegisterRequest>) -> Result<Response<RegisterResponse>, Status> {
        todo!()
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
