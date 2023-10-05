use std::collections::HashMap;

use num_bigint::BigUint;
use num_traits::{FromPrimitive, ToPrimitive};
use std::sync::Mutex;
use tonic::{transport::Server, Code, Request, Response, Status};

use zkp::ZKP;

pub mod zkp_auth {
    include!("./zkp_auth.rs");
}

use zkp_auth::{
    auth_server::{Auth, AuthServer},
    AuthenticationAnswerRequest, AuthenticationAnswerResponse, AuthenticationChallengeRequest,
    AuthenticationChallengeResponse, RegisterRequest, RegisterResponse,
};

#[derive(Debug, Default)]
pub struct AuthImpl {
    pub user_info: Mutex<HashMap<String, UserInfo>>,
    pub auth_id_to_user: Mutex<HashMap<String, String>>,
}

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
    pub session_id: BigUint,
}
#[tonic::async_trait]
impl Auth for AuthImpl {
    async fn register(
        &self,
        request: Request<RegisterRequest>,
    ) -> Result<Response<RegisterResponse>, Status> {
        println!("Processing Register: {:?} ", request);
        //extract request
        let request = request.into_inner();

        //get request information
        let username = request.user;
        let y1 = BigUint::from_bytes_be(&request.y1);
        let y2 = BigUint::from_bytes_be(&request.y2);

        //intialize structure
        let mut user_info = UserInfo::default();

        user_info.user_name = username.clone();
        user_info.y1 = y1;
        user_info.y2 = y2;

        let mut user_info_hashmap = &mut self.user_info.lock().unwrap(); //async ensure no other read or process can read from memory
        user_info_hashmap.insert(username.clone(), user_info);

        Ok(Response::new(RegisterResponse {})) //defined in protobuf
    }

    async fn create_authentication_challenge(
        &self,
        request: Request<AuthenticationChallengeRequest>,
    ) -> Result<Response<AuthenticationChallengeResponse>, Status> {
        println!("Processing Challenge request: {:?} ", request);
        //extract request
        let request = request.into_inner();

        //get request information
        let username = request.user;

        let mut user_info_hashmap = &mut self.user_info.lock().unwrap(); //async ensure no other read or process can read from memory

        //has the user registered
        if let Some(user_info) = user_info_hashmap.get_mut(&username) {
            user_info.r1 = BigUint::from_bytes_be(&request.r1);
            user_info.r2 = BigUint::from_bytes_be(&request.r2);

            let (_, _, _, q) = ZKP::get_constants();

            let c = ZKP::gen_ran_below(&q);
            let auth_id = ZKP::gen_ran_str(1000);

            println!("✅ Successful Challenge Request username: {:?}", username);

            //store auth id
            let mut auth_id_to_user = &mut self.auth_id_to_user.lock().unwrap();
            auth_id_to_user.insert(auth_id.clone(), username);

            Ok(Response::new(AuthenticationChallengeResponse {
                auth_id,
                c: c.to_bytes_be(),
            }))
        } else {
            Err(Status::new(
                Code::NotFound,
                format!("User: {} not found in database", username),
            ))
        }
    }

    async fn verify_authentication(
        &self,
        request: Request<AuthenticationAnswerRequest>,
    ) -> Result<Response<AuthenticationAnswerResponse>, Status> {
        todo!()
    }
}

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:7770".to_string();

    println!("Running the server in {}", addr);

    let auth_impl = AuthImpl::default();

    Server::builder()
        .add_service(AuthServer::new(auth_impl))
        .serve(addr.parse().expect("could not convert addr"))
        .await
        .unwrap();
}
