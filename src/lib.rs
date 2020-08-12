mod board;
mod position;

pub use board::Board;
pub use position::Position;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
