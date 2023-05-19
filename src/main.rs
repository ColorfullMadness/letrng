use std::io::{BufWriter, Write};
use std::ops::BitOr;
use std::sync::{Mutex, Arc} ;
use std::sync::atomic::AtomicU8;
use std::thread::JoinHandle;
use std::{thread, vec};
use std::fs::File;
use rsa::{RsaPrivateKey};
use rand::prelude::*;

const N: u32 = 100000000;

pub struct LETRNG{

}

impl Default for LETRNG{
    fn default() -> Self {
        return LETRNG {  };
    }
}

impl CryptoRng for LETRNG {}

pub fn generate_random_coins(x64: &Arc<Mutex<u64>>, y64: &Arc<Mutex<u64>>, x: Arc<AtomicU8>){
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
        print!(" x64 ={:21?}",*copy);
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
        print!(" y64 ={:21?}",*copy);
    });
    handles.push(handle);

    for handle in handles {
        handle.join().unwrap();
    }
}

fn generate_64_bit() -> u8{ //marmolada
    let x64 = Arc::new(Mutex::new(0 as u64));
    let y64 = Arc::new(Mutex::new(0 as u64));

    let x: Arc<AtomicU8> = Arc::new(AtomicU8::new(0));

    //let file = File::create("src/output.txt").expect("Couldn't open file nooob");
    let file = File::options()
            .write(true)
            .append(true)
            .open("src/output.txt").expect("Couldn't open file");

    let mut bufWriter = BufWriter::new(file);

    let mut iter = 0;
    let mut temp: u8 = 0;
    loop {
        generate_random_coins(&x64, &y64, x.clone());



        let mut mask:u8 = 0x00000001;

        //marmolada
        let outputx64: u8 = *x64.lock().unwrap() as u8;
        let outputy64: u8 = *y64.lock().unwrap() as u8;


        bufWriter.write(outputx64.to_string().as_bytes()).expect("Couldn't write to file!");
        bufWriter.write(b"\n").expect("Couldn't write to file!");
        bufWriter.write(outputy64.to_string().as_bytes()).expect("Couldn't write to file!");
        bufWriter.write(b"\n").expect("Couldn't write to file!");
        let mut outputx = outputx64 & mask;
        let mut outputy = outputy64 & mask;
        for i in 1..8 {//marmolada
            mask <<= 1;
            outputx = outputx ^ ((outputx64 & mask) >> i);
            outputy = outputy ^ ((outputy64 & mask) >> i);
        }

        println!("\t coin_a:{} coin_b:{}", outputx, outputy);
        
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
        println!("RAND: {}\n",temp);
        
    }
    // bufWriter.write(temp.to_string().as_bytes()).expect("Couldn't write to file!");
    // bufWriter.write(b"\n").expect("Couldn't write to file!");

    temp
}


fn main() {
    for _ in 0..100000{
        generate_64_bit();
    }

    // let mut rng = rand::thread_rng();

    // let mut letrng = ThreadRng::default();
    
    // let priv_key = RsaPrivateKey::new(&mut letrng,2048);

}