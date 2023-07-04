# Crypto samples

## OpenSSL

    let txt = "Hello world!";
    let hash = sha256(txt.as_bytes());
    let digest = hex::encode(hash);    
    println!("\nOpenSSL: SHA256 hash for {}\n{}\n", txt, digest);
