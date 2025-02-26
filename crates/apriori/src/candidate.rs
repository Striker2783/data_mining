use std::ops::{Deref, DerefMut};

type CandidateType = Vec<Vec<usize>>;

#[derive(Debug, Default, Hash, Clone)]
pub struct Candidates(CandidateType);
impl Candidates {
    pub fn new(v: CandidateType) -> Self {
        Self(v)
    }
    pub fn to_vec(self) -> CandidateType {
        self.0
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
