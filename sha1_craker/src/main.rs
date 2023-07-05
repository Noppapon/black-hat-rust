use sha1::Digest;
use std::{
    env,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

//In real-world scenario, we may want to use optimized hash crackers such as "hashcat" or "John the Ripper"
//they, among other things, may use the GPU to significantly speed up the craking.

const SHA1_HEX_STRING_LENGTH: usize = 40;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("Usage:");
        println!("sha1_craker: <wordlist.txt> <sha1_hash>");
        return Ok(());
    }

    let hash_to_crack = args[2].trim();
    if hash_to_crack.len() != SHA1_HEX_STRING_LENGTH {
        return Err("sha1 hash not valid".into());
    }

    let wordlist_file = File::open(&args[1])?;
    let reader = BufReader::new(&wordlist_file);

    for line in reader.lines() {
        let line = line?.trim().to_string();
        println!("{}", line);
    }

    let wordlist_file = File::open(&args[1])?;
    let reader = BufReader::new(&wordlist_file);

    //To optimize, we could load the wordlist in memory before performing the computations
    for line in reader.lines() {
        let line = line?;
        let common_password = line.trim();
        if hash_to_crack == hex::encode(sha1::Sha1::digest(common_password.as_bytes())) {
            println!("Password found: {}", &common_password);
            return Ok(());
        }
    }
    // We opened the wordlist file, but never closed it. RAII: Resource Acquisition Is Initialization
    // Automatically closed when goes out of scope

    println!("password not found in wordlist :(");

    Ok(())
}
