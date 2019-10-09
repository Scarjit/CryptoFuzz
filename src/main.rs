#[macro_use]
extern crate afl;
extern crate rsa;
extern crate rand;
use rsa::{PublicKey, RSAPrivateKey, PaddingScheme};
use rand::rngs::OsRng;
use std::sync::{Arc, Mutex};
use std::fs;

fn main() {

    let rng = Arc::new(Mutex::new(OsRng::new().expect("no secure randomness available")));
    let bits = 2048;

    let key = RSAPrivateKey::new(&mut *rng.lock().unwrap(), bits).expect("failed to generate a key");

    fuzz!(|data: &[u8]| {
        if data.len() < 254 {
            let enc_data = key.encrypt(&mut *rng.lock().unwrap(), PaddingScheme::PKCS1v15, &data[..]).unwrap();
            let dec_data = key.decrypt(PaddingScheme::PKCS1v15, &enc_data).unwrap();
            assert_eq!(&data[..], &dec_data[..]);
        }
    });

    /*
    // Decrypt
    let dec_data = key.decrypt(PaddingScheme::PKCS1v15, &enc_data).expect("failed to decrypt");
    assert_eq!(&data[..], &dec_data[..]);
    */
}


#[test]
fn test_in_data(){
    println!("Generating private rng");
    let rng = Arc::new(Mutex::new(OsRng::new().expect("no secure randomness available")));
    let bits = 2048;
    println!("Generating private key");
    let key = RSAPrivateKey::new(&mut *rng.lock().unwrap(), bits).expect("failed to generate a key");
    let paths = fs::read_dir("./in").unwrap();


    for path in paths {
        let pa = path.unwrap().path();
        println!("Testing {:?}", &pa);
        let data = fs::read(&pa).unwrap();

        if data.len() < 254 { //PKCS#1 max message len
            let enc_data = key.encrypt(&mut *rng.lock().unwrap(), PaddingScheme::PKCS1v15, &data[..]).unwrap();
            let dec_data = key.decrypt(PaddingScheme::PKCS1v15, &enc_data).unwrap();
            assert_eq!(&data[..], &dec_data[..]);
        }else{
            println!("Invalid test size ({:?})", &pa);
        }
    }
}

#[test]
fn evalute_crashes(){
    println!("Generating private rng");
    let rng = Arc::new(Mutex::new(OsRng::new().expect("no secure randomness available")));
    let bits = 2048;
    println!("Generating private key");
    let key = RSAPrivateKey::new(&mut *rng.lock().unwrap(), bits).expect("failed to generate a key");

    let dirs = [
        "./out/master_fuzzer/crashes/",
        "./out/slave_1/crashes/",
        "./out/slave_2/crashes/",
        "./out/slave_3/crashes/",
        "./out/slave_4/crashes/",
        "./out/slave_5/crashes/",
        "./out/slave_6/crashes/",
        "./out/slave_7/crashes/",
        "./out/slave_8/crashes/",
        "./out/slave_9/crashes/",
    ];

    for dir in &dirs {
        println!("{:?}", dir);
        let paths = fs::read_dir(&dir).unwrap();
        for path in paths {
            let pa = path.unwrap().path();
            println!("Testing {:?}", &pa);
            let data = fs::read(&pa).unwrap();
            if data.len() < 254 {
                let enc_data = key.encrypt(&mut *rng.lock().unwrap(), PaddingScheme::PKCS1v15, &data[..]).unwrap();
                let dec_data = key.decrypt(PaddingScheme::PKCS1v15, &enc_data).unwrap();
                assert_eq!(&data[..], &dec_data[..]);
            }else{
                println!("Invalid test size ({:?})", &pa);
            }

        }
    }
}