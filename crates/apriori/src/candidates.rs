use std::{collections::{HashMap, HashSet}, ops::{Deref, DerefMut}};

pub type CandidateType = HashSet<Vec<usize>>;
pub type CandidateCounter = HashMap<Vec<usize>, u64>;

#[derive(Debug, Default, Clone)]
pub struct Candidates(CandidateType);
impl Candidates {
    pub fn new(v: CandidateType) -> Self {
        Self(v)
    }
}
impl Deref for Candidates {
    type Target = CandidateType;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for Candidates {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<CandidateType> for Candidates {
    fn from(value: CandidateType) -> Self {
        Self::new(value)
    }
}