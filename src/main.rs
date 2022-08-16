use core::panic;
use std::env::{self};
use std::io::{BufReader, BufRead};
use std::process::exit;
use std::fs::{File, self};

const BUFFER_SIZE: usize = 2048;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        do_stdout()
    }

    for i in 1..args.len() {
        let line_cnt = do_wcl(&args[i]);
        let word_cnt = do_wcw(&args[i]);
        let bytes_cnt = do_wcb(&args[i]);
        println!(" {} {} {} {}", line_cnt, word_cnt, bytes_cnt, &args[i])
    }

    exit(0);
}

fn do_stdout() {
    unsafe {
        let buf: *mut libc::c_void = libc::malloc(BUFFER_SIZE);
        let mut n: libc::ssize_t;
        loop {
            n = libc::read(
                libc::STDOUT_FILENO,
                buf,
                std::mem::size_of::<libc::c_void>(),
            );
            if libc::write(libc::STDOUT_FILENO, buf, n as usize) < 0 {
                panic!("write_error");
            }
        }
    }
}

fn do_wcl(file_path: &str) -> usize {
    let f = File::open(file_path).unwrap();
    let reader = BufReader::new(f);
    let mut line_cnt = 0;
    for _ in reader.lines() {
        line_cnt += 1;
    }

    line_cnt 
}

fn do_wcw(file_path: &str) -> usize {
    let f = File::open(file_path).unwrap();
    let reader = BufReader::new(f);
    let mut word_cnt = 0;
    for line in reader.lines() {
        let line_str: String = line.unwrap();
        let v: Vec<&str> = line_str.trim().split_whitespace().collect();
        word_cnt += v.len();
    }

    word_cnt
        
}

fn do_wcb(file_path: &str) -> u64 {
    let x = fs::metadata(file_path).unwrap();
    x.len()
}
