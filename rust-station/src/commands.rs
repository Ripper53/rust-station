#[derive(Debug)]
pub enum WorldCommand {
    DamageTrainCart(TrainCartID),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct TrainCartID(usize);

impl TrainCartID {
    pub fn new(index: usize) -> Self {
        TrainCartID(index)
    }
    pub fn index(&self) -> usize {
        self.0
    }
}
