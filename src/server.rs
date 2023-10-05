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
#[derive(Debug, Default, Hash)]
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
    pub session_id: String,
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
            let (_, _, _, q) = ZKP::get_constants();
            let c = ZKP::gen_ran_below(&q);
            let auth_id = ZKP::gen_ran_str(1000);

            user_info.c = c.clone();
            user_info.r1 = BigUint::from_bytes_be(&request.r1);
            user_info.r2 = BigUint::from_bytes_be(&request.r2);

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
        println!("Processing Verification request: {:?} ", request);
        //extract request
        let request = request.into_inner();

        //get request information
        let auth_id = request.auth_id;

        let auth_id_to_user_hashmap = &mut self.auth_id_to_user.lock().unwrap(); //async ensure no other read or process can read from memory

        //has the user registered
        if let Some(user_name) = auth_id_to_user_hashmap.get(&auth_id) {
            let mut user_info_hashmap = &mut self.user_info.lock().unwrap(); //async ensure no other read or process can read from memory
            let user_info = user_info_hashmap
                .get_mut(user_name)
                .expect("AUthId: {} not found on hashmap");

            let s = &user_info.s;
            let r1 = &user_info.r1;
            let r2 = &user_info.r2;
            let y1 = &user_info.y1;
            let y2 = &user_info.y2;
            let c = &user_info.c;

            let (alpha, beta, p, q) = ZKP::get_constants();
            let zkp = ZKP { alpha, beta, p, q };

            let verification = zkp.verify(r1, r2, y1, y2, c, s);

            if verification {
                let session_id = ZKP::gen_ran_str(1000);

                println!("Correct Challenge Solution username: {:?}", user_name);

                Ok(Response::new(AuthenticationAnswerResponse { session_id }))
            } else {
                println!("Wrong Challenge Solution username: {:?}", user_name);

                Err(Status::new(
                    Code::PermissionDenied,
                    format!("AuthId: {} bad solution to the challenge", auth_id),
                ))
            }
        } else {
            Err(Status::new(
                Code::NotFound,
                format!("AuthId: {} not found in database", auth_id),
            ))
        }
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
