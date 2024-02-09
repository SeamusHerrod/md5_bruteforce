extern crate md5;
use std::time::Instant;
use std::fs;
use std::cell::Cell;
fn main() {
    let mut found = Cell::new(false);
    let mut task = 3;
    let start_time = Instant::now();
    let test_pass = "00000000000019";
    let test_hash = md5::compute(test_pass);
    let test_hash = format!("{:x}", test_hash);

    println!("test hash: {}", test_hash);
    if task == 0 {
        crack_1(&found, "".to_string(), 14, &mut |s| {
            let comp_hash = md5::compute(s);
            let comp_hash = format!("{:x}", comp_hash);
            println!("current attempt: {}\ncurrent attempt hash: {}", s, comp_hash);
            if comp_hash == test_hash {
                println!("password0: {}", s);
                found.set(true);
                let duration = start_time.elapsed();
                println!("Time elapsed: {:?}", duration);
                let result = format!("password0: {}\nTime elapsed: {:?}", s, duration);
                fs::write("password0.txt", result).expect("Unable to write file");
                task = 2;
            }
        });

    }
    found = false.into();
    let start_time = Instant::now();
    if task == 1 {
        crack_1(&found, "".to_string(), 14, &mut |s| {
            let comp_hash = md5::compute(s);
            let comp_hash = format!("{:x}", comp_hash);
            println!("current attempt: {}\ncurrent attempt hash: {}", s, comp_hash);
            if comp_hash == HASH1 {
                println!("password1: {}", s);
                found.set(true);
                let duration = start_time.elapsed();
                let result = format!("password1: {}\nTime elapsed: {:?}", s, duration);
                fs::write("password1.txt", result).expect("Unable to write file");
                task = 2;
            }
        });
    }
    found = false.into();
    let start_time = Instant::now();
    if task == 2 {
        crack_2(&found, "".to_string(), 11, &mut |s| {
            let comp_hash = md5::compute(s);
            let comp_hash = format!("{:x}", comp_hash);
            println!("current attempt: {}\ncurrent attempt hash: {}", s, comp_hash);
            if comp_hash == HASH2 {
                found.set(true);
                let duration = start_time.elapsed();
                let result = format!("password2: {}\nTime elapsed: {:?}", s, duration);
                fs::write("password2.txt", result).expect("Unable to write file");
                task = 3;
            }
        });
    }
    found = false.into();
    let start_time = Instant::now();
    if task == 3 {
        crack_3(&found, "", 11, &mut |s| {
            let comp_hash = md5::compute(s);
            let comp_hash = format!("{:x}", comp_hash);
            println!("current attempt: {}\ncurrent attempt hash: {}", s, comp_hash);
            if comp_hash == HASH3 {
                found.set(true);
                let duration = start_time.elapsed();
                let result = format!("password3: {}\nTime elapsed: {:?}", s, duration);
                fs::write("password3.txt", result).expect("Unable to write file");
            }
        });
    }
}

// pasword length: 14 
// format: A-Z, a-z, or 0-9
static HASH1: &'static str = "94d9e03c11395841301a7aee967864ec";

// password length: 11
// format: <4 UPPER CASE A-Z><3 digit number><4 lower case letter> 
static HASH2: &'static str = "f593def02f37f3a6d57bcbc9480a3316";


fn crack_1 (found: &Cell<bool>, curr: String, len: usize, f: &mut dyn FnMut(&str)) {
    if found.get() {
        return;
    }
    if curr.len() == len {
        f(&curr)
    }
    else {
        for x in (b'0'..=b'9').chain(b'A'..=b'Z').chain(b'a'..=b'z').map(char::from) {
            crack_1(&found, curr.clone() + &x.to_string(), len, f);
        }
    }
    
}

fn crack_2 (found: &Cell<bool>, curr: String, len: usize, f: &mut dyn FnMut(&str)) {
    if found.get() {
        return;
    }
    if curr.len() == len {
        f(&curr)
    }
    else {
        if curr.len() < 4 {
            for x in (b'A'..=b'Z').map(char::from) {
                crack_2(&found, curr.clone() + &x.to_string(), len, f);
            }
        }
        else if curr.len() < 7 {
            for x in (b'0'..=b'9').map(char::from) {
                crack_2(&found, curr.clone() + &x.to_string(), len, f);
            }
        }
        else {
            for x in (b'a'..=b'z').map(char::from) {
                crack_2(&found, curr.clone() + &x.to_string(), len, f);
            }
        }
    }
}

// password length: 11
// format: 7 lower case letters and the number "1234" but you don't know the 
//         position of "1234"
static HASH3: &'static str = "bfb2c12706757b8324368de6a365338b";
fn crack_3(found: &Cell<bool>, curr: &str, len: usize, f: &mut dyn FnMut(&str)){
    if found.get() {
        return;
    }
    if curr.len() == 7 {
        for i in 0..=7 {
            let mut s = String::new();
            s.push_str(&curr[0..i]);
            s.push_str("1234");
            s.push_str(&curr[i..]);
            f(&s);
        }
    } else {
        for c in b'a'..=b'z' {
            let mut next = curr.to_string();
            next.push(c as char);
            if next.len() <= len {
                crack_3(&found, &next, len, f);
            }
        }
    }
}
