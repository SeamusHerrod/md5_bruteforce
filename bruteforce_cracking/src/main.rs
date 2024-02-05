extern crate md5;

fn main() {
    let mut found = false;

    crack_1("".to_string(), 14, &mut |s| {
        let comp_hash = md5::compute(s);
        let comp_hash = format!("{:x}", comp_hash);
        println!("current attempt: {}\ncurrent attempt hash: {}", s, comp_hash);
        if comp_hash == HASH1 {
            println!("password1: {}", s);
            found = true;
        }
    });
}

// pasword length: 14 
// format: A-Z, a-z, or 0-9
static HASH1: &'static str = "94d9e03c11395841301a7aee967864ec";

// password length: 14
// format: <4 UPPER CASE A-Z><3 digit number><7 lower case number> 
static HASH2: &'static str = "a5a410227370171ad4754213b9be8cbe";

// password length: 14
// format: 10 upper case letters and the number "1234" but you don't know the 
//         position of "1234"
static HASH3: &'static str = "e42b28ac0eb64cbc34f0030e0821f435";

fn crack_1 (curr: String, len: usize, f: &mut dyn FnMut(&str)) {
    if curr.len() == len {
        f(&curr)
    }
    else {
        for x in (b'0'..=b'9').chain(b'A'..=b'Z').chain(b'a'..=b'z').map(char::from) {
            crack_1(curr.clone() + &x.to_string(), len, f);
        }
    }

}
