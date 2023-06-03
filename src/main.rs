use std::io::{BufWriter, Write};
use std::ops::BitOr;
use std::sync::{Mutex, Arc} ;
use std::sync::atomic::AtomicU8;
use std::thread::JoinHandle;
use std::time::Instant;
use std::{thread, vec};
use std::fs::File;
use rand::prelude::*;
use rsa::{Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey};

const N: u32 = 100000000;

fn generate_random_coins(x64: &Arc<Mutex<u64>>, y64: &Arc<Mutex<u64>>, x: Arc<AtomicU8>){
    let mut handles: Vec<JoinHandle<()>> = vec![]; 
    let finished: Arc<AtomicU8> = Arc::new(AtomicU8::new(0));

    let x_c = Arc::clone(&x);
    let f_c = Arc::clone(&finished);
    let handle = thread::spawn(move||{
        for i in 0..N {
            x_c.store((i % 2) as u8, std::sync::atomic::Ordering::SeqCst);
            if i % 50 == 0 {
                if f_c.load(std::sync::atomic::Ordering::SeqCst) == 2 {
                    break;
                }
            }
        }
    });
    handles.push(handle);

    let x_c = Arc::clone(&x);
    let f_c = Arc::clone(&finished);
    let handle = thread::spawn(move ||{
        for i in N..0 {
            x_c.store((i % 2) as u8  , std::sync::atomic::Ordering::SeqCst);
            if i % 50 == 0 {
                if f_c.load(std::sync::atomic::Ordering::SeqCst) == 2 {
                   break;
                }
            }
        }
    });
    handles.push(handle);

    let x64_c = Arc::clone(&x64);
    let x_c = Arc::clone(&x);
    let f_c = Arc::clone(&finished);
    let handle = thread::spawn(move ||{
        let mut x64_outcome: u64 = 0;
        for _ in 0..64 {
            let loc = x_c.load(std::sync::atomic::Ordering::SeqCst);
            x64_outcome <<= 1;
            x64_outcome = x64_outcome.bitor(loc as u64).into();
        }
        let mut copy = x64_c.lock().unwrap(); 
        f_c.store(f_c.load(std::sync::atomic::Ordering::SeqCst) + 1, std::sync::atomic::Ordering::SeqCst);
        *copy = x64_outcome;
    });
    handles.push(handle);

    let y64_c = Arc::clone(&y64);
    let x_c = Arc::clone(&x);
    let f_c = Arc::clone(&finished);
    let handle = thread::spawn(move ||{
        let mut y64_outcome: u64 = 0;
        for _ in 0..64 {
            let loc = x_c.load(std::sync::atomic::Ordering::SeqCst);
            y64_outcome <<= 1;
            y64_outcome = y64_outcome.bitor(loc as u64).into();
        }
        let mut copy = y64_c.lock().unwrap(); 
        f_c.store(f_c.load(std::sync::atomic::Ordering::SeqCst) + 1, std::sync::atomic::Ordering::SeqCst);
        *copy = y64_outcome;
    });
    handles.push(handle);

    for handle in handles {
        handle.join().unwrap();
    }
}

fn generate_64_bit() -> u64{
    let x64 = Arc::new(Mutex::new(0 as u64));
    let y64 = Arc::new(Mutex::new(0 as u64));

    let x: Arc<AtomicU8> = Arc::new(AtomicU8::new(0));

    let save_to_file = false;

    let file = File::options()
            .write(true)
            .append(true)
            .open("src/output.txt").expect("Couldn't open file");

    let mut buf_writer = BufWriter::new(file);

    let mut iter = 0;
    let mut temp: u64 = 0;
    loop {
        generate_random_coins(&x64, &y64, x.clone());

        let mut mask:u64 = 0x00000001;

        let outputx64= *x64.lock().unwrap() as u64;
        let outputy64 = *y64.lock().unwrap() as u64;

        let mut outputx = outputx64 & mask;
        let mut outputy = outputy64 & mask;
        for i in 1..64 {
            mask <<= 1;
            outputx = outputx ^ ((outputx64 & mask) >> i);
            outputy = outputy ^ ((outputy64 & mask) >> i);
        }
        
        if outputx == outputy {
            continue;
        } else {
            temp <<= 1;
            temp += outputx;
            iter += 1;
            
            if iter == 64 {
                break;
            }
        }
    }
    if save_to_file {
        buf_writer.write(temp.to_string().as_bytes()).expect("Couldn't write to file!");
        buf_writer.write(b"\n").expect("Couldn't write to file!");

    }
    temp
}

fn generate_32_bit() -> u32{
    let x64 = Arc::new(Mutex::new(0 as u64));
    let y64 = Arc::new(Mutex::new(0 as u64));

    let x: Arc<AtomicU8> = Arc::new(AtomicU8::new(0));

    let mut iter = 0;
    let mut temp: u32 = 0;
    loop {
        generate_random_coins(&x64, &y64, x.clone());

        let mut mask:u32 = 0x00000001;

        let outputx64= *x64.lock().unwrap() as u32;
        let outputy64 = *y64.lock().unwrap() as u32;

        let mut outputx = outputx64 & mask;
        let mut outputy = outputy64 & mask;
        for i in 1..32 {
            mask <<= 1;
            outputx = outputx ^ ((outputx64 & mask) >> i);
            outputy = outputy ^ ((outputy64 & mask) >> i);
        }
        
        if outputx == outputy {
            continue;
        } else {
            temp <<= 1;
            temp += outputx;
            iter += 1;
            
            if iter == 32 {
                break;
            }
        }
    }
    temp
}


fn generate_8_bit() -> u8{
    let x64 = Arc::new(Mutex::new(0 as u64));
    let y64 = Arc::new(Mutex::new(0 as u64));

    let x: Arc<AtomicU8> = Arc::new(AtomicU8::new(0));


    let mut iter = 0;
    let mut temp: u8 = 0;
    loop {
        generate_random_coins(&x64, &y64, x.clone());

        let mut mask:u8 = 0x00000001;

        let outputx64= *x64.lock().unwrap() as u8;
        let outputy64 = *y64.lock().unwrap() as u8;

        let mut outputx = outputx64 & mask;
        let mut outputy = outputy64 & mask;
        for i in 1..8 {
            mask <<= 1;
            outputx = outputx ^ ((outputx64 & mask) >> i);
            outputy = outputy ^ ((outputy64 & mask) >> i);
        }
        
        if outputx == outputy {
            continue;
        } else {
            temp <<= 1;
            temp += outputx;
            iter += 1;
            
            if iter == 8 {
                break;
            }
        }
    }
    temp
}

pub struct LETRNG{

}

impl Default for LETRNG{
    fn default() -> Self {
        return LETRNG {  };
    }
}

impl CryptoRng for LETRNG {}

impl RngCore for LETRNG{
    fn fill_bytes(&mut self, dest: &mut [u8]) {
        dest.iter_mut().for_each(|x| *x = generate_8_bit());
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand::Error> {
        dest.iter_mut().for_each(|x| *x = generate_8_bit());
        Ok(())
    }

    fn next_u32(&mut self) -> u32 {
        generate_32_bit()
    }

    fn next_u64(&mut self) -> u64 {
        generate_64_bit()
    }
}

fn main() {
    let now = Instant::now();
    let mut letrng = LETRNG::default();

    let priv_key = RsaPrivateKey::new(&mut letrng, 2048).expect("Failed to generate key.");
    println!("Elapsed priv key generation:   {:.2?}",now.elapsed());

    let pub_key = RsaPublicKey::from(&priv_key);
    println!("Elapsed public key generation: {:.2?}",now.elapsed());

    let data = b"Marmolada jest pyszna";
    let enc_data = pub_key.encrypt(&mut letrng, Pkcs1v15Encrypt, &data[..]).expect("Problem with encrypting data.");
    println!("Elapsed encrypting:            {:.2?}",now.elapsed());

    let data_2 = b"Marmolada nie jest pyszna";
    let enc_data_2 = pub_key.encrypt(&mut letrng, Pkcs1v15Encrypt, &data_2[..]).expect("Problem with encrypting data.");
    assert_ne!(enc_data, enc_data_2);

    let dec_data = priv_key.decrypt(Pkcs1v15Encrypt, &enc_data).expect("Problems with decoding data.");
    println!("Elapsed decrypting:            {:.2?}",now.elapsed());
    
    println!("Priv key:   {:?}",priv_key);
    println!("Public key: {:?}",pub_key);

    println!("Enc data: {:?}",enc_data);
    println!("Dec data: {}",String::from_utf8(dec_data).expect("Problem with casting"));


    let priv_key = RsaPrivateKey::new(&mut letrng, 2048).expect("Failed to generate key.");

    let dec_data2 = priv_key.decrypt(Pkcs1v15Encrypt, &enc_data).expect("Problems with decoding data.");
    println!("Dec data 2: {}",String::from_utf8(dec_data2).expect("Problem with casting"));
    
}