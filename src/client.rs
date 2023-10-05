use num_bigint::BigUint;
use std::io::stdin;
pub mod zkp_auth {
    include!("./zkp_auth.rs");
}

use tonic::Request;
use zkp_auth::{auth_client::AuthClient, RegisterRequest};

use zkp::ZKP;

#[tokio::main]
async fn main() {
    let mut bufffer = String::new();
    let (alpha, beta, p, q) = ZKP::get_constants();
    let zkp = ZKP {
        alpha: alpha.clone(),
        beta: beta.clone(),
        p: p.clone(),
        q: q.clone(),
    };

    let mut client = AuthClient::connect("http://127.0.0.1:7770")
        .await
        .expect("Could not connect to the server");

    println!("Connected to server");

    //get username
    println!("Please provide username:");
    stdin()
        .read_line(&mut bufffer)
        .expect("Could not get username from stdin");
    let username = bufffer.trim().to_string();

    //get password
    println!("Please provide password:");
    stdin()
        .read_line(&mut bufffer)
        .expect("Could not get password from stdin");
    let password = BigUint::from_bytes_be(bufffer.trim().as_bytes());

    let y1 = ZKP::mod_exp(&alpha, &password, &p);
    let y2 = ZKP::mod_exp(&alpha, &password, &p);

    let request = RegisterRequest {
        user: username,
        y1: y1.to_bytes_be(),
        y2: y2.to_bytes_be(),
    };

    let response = client
        .register(request)
        .await
        .expect("Could not register in server");
    println!("{:?}", response)
}
