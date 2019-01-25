#[derive(Debug)]
pub struct Prob {}

impl Prob {
    pub fn probability(desired: i8) -> bool {
        rand::random::<i8>() % 100 < desired
    }
}