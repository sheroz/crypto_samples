# Crypto samples

## OpenSSL

For setting up the openssl dependencies please look here:
[https://docs.rs/openssl/latest/openssl/](https://docs.rs/openssl/latest/openssl/)

Ubuntu:

    sudo apt-get install pkg-config libssl-dev

sha256 sample:

    let txt = "Hello world!";
    let hash = sha256(txt.as_bytes());
    let digest = hex::encode(hash);    
    println!("\nOpenSSL: SHA256 hash for {}\n{}\n", txt, digest);
