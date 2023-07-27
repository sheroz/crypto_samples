use openssl::sha::sha256;
use openssl::hash::{Hasher, MessageDigest};


fn main() {
    println!("\nOpenSSL samples\n");

    let txt = "abc";
    let hash = sha256(txt.as_bytes());
    let digest = hex::encode(hash);    
    assert_eq!(digest, "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad");
    println!("SHA-256 for {}\n{}\n", txt, digest);

    let mut hasher = Hasher::new(MessageDigest::sha3_256()).unwrap();
    hasher.update(txt.as_bytes()).unwrap();
    let hash = hasher.finish().unwrap();
    let digest = hex::encode(hash);    
    assert_eq!(digest, "3a985da74fe225b2045c172d6bd390bd855f086e3e9d525b46bfe24511431532");
    println!("SHA-3-256 hash for {}\n{}\n", txt, digest);
}