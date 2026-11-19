//! Development interface for a single rank-axis equality constraint.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dimension {
    Known(usize),
    Symbol { binder: usize, axis: usize },
    Runtime { value: usize, axis: usize },
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Equality {
    Proven,
    Contradiction,
    Guard(Dimension, Dimension),
}
pub fn require_equal(left: Dimension, right: Dimension) -> Equality {
    use Dimension::Known;
    match (left, right) {
        (Known(0), _) | (_, Known(0)) => Equality::Contradiction,
        _ if left == right => Equality::Proven,
        (Known(_), Known(_)) => Equality::Contradiction,
        _ => Equality::Guard(left, right),
    }
}
