mod code_gen;
mod enums;
mod lexer;
mod memory;
mod optimizer;
pub mod vm;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
