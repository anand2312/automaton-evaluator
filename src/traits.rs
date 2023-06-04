pub trait Acceptor {
    /// All acceptor automata will implement this trait.
    fn test_string(&self, s: String) -> bool;
}
