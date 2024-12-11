use std::u32;

pub fn countdown(n: u32) -> Vec<u32> {
    let mut rs:Vec<u32> = Vec::new();
    let mut it:u32 = n;

    while it != 0 {
        rs.push(it);
        it-= 1;
    }

    rs.push(0);
    return rs;
}
