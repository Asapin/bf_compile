include!(concat!(env!("OUT_DIR"), "/result.rs"));

fn main() {
    println!("{}", result());
}
