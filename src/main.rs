use serde::{Serialize, Deserialize};
use jsonwebtoken::*;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    exp: u64,
}

fn main() {
    let secret = "123".as_bytes();
    let encoding = EncodingKey::from_secret(&secret);
    let decoding = DecodingKey::from_secret(&secret);
    let header = Header::new(Algorithm::HS512);
    let now = 60 + std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let claims = Claims { exp: now };
    let token = encode(&header, &claims, &encoding).unwrap();
    println!("{:?}", token);
    let validation = Validation::new(Algorithm::HS512);
    let xclaims = decode::<Claims>(&token, &decoding, &validation).unwrap();
    println!("{:?}", xclaims);
}
