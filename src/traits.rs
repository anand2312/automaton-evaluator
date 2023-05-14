pub trait Acceptor {
    fn test_string(&self, s: String) -> bool;
}
