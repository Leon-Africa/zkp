use num_bigint::BigUint;
use std::io::stdin;
pub mod zkp_auth {
    include!("./zkp_auth.rs");
}

use zkp_auth::{
    auth_client::AuthClient, AuthenticationAnswerRequest, AuthenticationChallengeRequest,
    RegisterRequest,
};

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
    bufffer.clear();

    //get password
    println!("Please provide password:");
    stdin()
        .read_line(&mut bufffer)
        .expect("Could not get password from stdin");
    let password = BigUint::from_bytes_be(bufffer.trim().as_bytes());
    bufffer.clear();

    //Register Request
    let y1 = ZKP::mod_exp(&alpha, &password, &p);
    let y2 = ZKP::mod_exp(&beta, &password, &p);
    let request = RegisterRequest {
        user: username.clone(),
        y1: y1.to_bytes_be(),
        y2: y2.to_bytes_be(),
    };

    let response = client
        .register(request)
        .await
        .expect("Could not register in server");
    println!("{:?}", response);

    //login using registered password
    println!("Please provide password to login:");
    stdin()
        .read_line(&mut bufffer)
        .expect("Could not get password from stdin");
    let password = BigUint::from_bytes_be(bufffer.trim().as_bytes());
    bufffer.clear();

    //Authentication Challenge
    let k = ZKP::gen_ran_below(&q);
    let r1 = ZKP::mod_exp(&alpha, &k, &p);
    let r2 = ZKP::mod_exp(&beta, &k, &p);

    let request = AuthenticationChallengeRequest {
        user: username,
        r1: r1.to_bytes_be(),
        r2: r2.to_bytes_be(),
    };

    let response = client
        .create_authentication_challenge(request)
        .await
        .expect("Could not request challenge to server")
        .into_inner();
    println!("{:?}", response);

    //Authentication Answer Request
    let auth_id = response.auth_id;
    let c = BigUint::from_bytes_be(&response.c);
    let s = zkp.solve(&k, &c, &password);

    let request = AuthenticationAnswerRequest {
        auth_id: auth_id,
        s: s.to_bytes_be(),
    };

    let response = client
        .verify_authentication(request)
        .await
        .expect("Could not verify authentication")
        .into_inner();
    println!("{:?}", response);

    //success
    println!("Login successful with session_id: {}", response.session_id);
}
