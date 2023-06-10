use crate::hash::HashImpl;

mod hash;

fn main() {

    let r = HashImpl::check("./rstool.exe");
    println!("r: {}", r);
}
