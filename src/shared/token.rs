#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Token {
    Literal(u8),
    Match(u16, u16),
}
