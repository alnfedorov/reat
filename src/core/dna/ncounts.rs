// pub use inner::*;

// Workaround to disable snake_case warning for the struct.
// Annotating struct/fields didn't work for some reasons
// mod inner {
//     #![allow(non_snake_case)]

use std::cmp::max;

use derive_more::{Add, AddAssign, Mul};

use crate::core::dna::{Nucleotide, ReqNucleotide};

#[derive(Clone, Copy, Eq, PartialEq, Debug, Add, AddAssign, Mul, Default)]
#[allow(non_snake_case)]
pub struct NucCounts {
    pub A: u32,
    pub C: u32,
    pub G: u32,
    pub T: u32,
}

impl NucCounts {
    #[allow(non_snake_case)]
    pub fn new(A: u32, C: u32, G: u32, T: u32) -> Self {
        Self { A, C, G, T }
    }

    pub fn increment(&mut self, sequence: &[Nucleotide]) {
        for nuc in sequence {
            match nuc {
                Nucleotide::A => self.A += 1,
                Nucleotide::C => self.C += 1,
                Nucleotide::G => self.G += 1,
                Nucleotide::T => self.T += 1,
                Nucleotide::Unknown => {}
            }
        }
    }

    #[inline]
    pub const fn zeros() -> NucCounts {
        NucCounts { A: 0, T: 0, G: 0, C: 0 }
    }

    #[inline]
    #[allow(non_snake_case)]
    pub const fn A(A: u32) -> NucCounts {
        NucCounts { A, T: 0, G: 0, C: 0 }
    }

    #[inline]
    #[allow(non_snake_case)]
    pub const fn T(T: u32) -> NucCounts {
        NucCounts { A: 0, T, G: 0, C: 0 }
    }

    #[inline]
    #[allow(non_snake_case)]
    pub const fn G(G: u32) -> NucCounts {
        NucCounts { A: 0, T: 0, G, C: 0 }
    }

    #[inline]
    #[allow(non_snake_case)]
    pub const fn C(C: u32) -> NucCounts {
        NucCounts { A: 0, T: 0, G: 0, C }
    }

    #[inline]
    pub const fn coverage(&self) -> u32 {
        self.A + self.T + self.G + self.C
    }

    #[inline]
    pub fn mismatches(&self, reference: Nucleotide) -> u32 {
        match reference {
            Nucleotide::A => self.C + self.G + self.T,
            Nucleotide::C => self.A + self.G + self.T,
            Nucleotide::G => self.A + self.C + self.T,
            Nucleotide::T => self.A + self.C + self.G,
            Nucleotide::Unknown => self.A + self.C + self.G + self.T,
        }
    }

    #[inline]
    pub fn mostfreq(&self) -> (ReqNucleotide, &u32) {
        let maximum = max(max(self.A, self.T), max(self.G, self.C));

        if self.A == maximum {
            (ReqNucleotide::A, &self.A)
        } else if self.C == maximum {
            (ReqNucleotide::C, &self.C)
        } else if self.G == maximum {
            (ReqNucleotide::G, &self.G)
        } else {
            (ReqNucleotide::T, &self.T)
        }
    }

    #[inline]
    pub fn complementary(&self) -> Self {
        Self { A: self.T, C: self.G, G: self.C, T: self.A }
    }
}
// }

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use crate::core::dna::{Nucleotide, ReqNucleotide};

    use super::*;

    #[test]
    fn from_sequence() {
        let mut counts = NucCounts { A: 1, C: 2, G: 3, T: 5 };

        let sequence = [
            [Nucleotide::A].repeat(7),
            [Nucleotide::C].repeat(10),
            [Nucleotide::Unknown].repeat(5),
            [Nucleotide::G].repeat(3),
            [Nucleotide::T].repeat(2),
            [Nucleotide::Unknown].repeat(5),
        ]
        .iter()
        .flatten()
        .copied()
        .collect_vec();
        counts.increment(&sequence);

        let expected = NucCounts { A: 8, C: 12, G: 6, T: 7 };
        assert_eq!(expected, counts);
    }

    #[test]
    fn coverage() {
        let dummy = NucCounts { A: 1, C: 2, G: 3, T: 0 };
        assert_eq!(dummy.coverage(), 6);
        assert_eq!(NucCounts::zeros().coverage(), 0);
    }

    #[test]
    fn mismatches() {
        let dummy = NucCounts { A: 1, C: 2, G: 3, T: 4 };
        assert_eq!(dummy.mismatches(Nucleotide::A), 9);
        assert_eq!(dummy.mismatches(Nucleotide::C), 8);
        assert_eq!(dummy.mismatches(Nucleotide::G), 7);
        assert_eq!(dummy.mismatches(Nucleotide::T), 6);
        assert_eq!(dummy.mismatches(Nucleotide::Unknown), 10);
    }

    #[test]
    fn mostfreq_maximum() {
        let mut dummy = NucCounts { A: 10, C: 2, G: 3, T: 5 };
        assert_eq!(dummy.mostfreq(), (ReqNucleotide::A, &10));
        dummy.A = 1;
        assert_eq!(dummy.mostfreq(), (ReqNucleotide::T, &5));
        dummy.T = 1;
        assert_eq!(dummy.mostfreq(), (ReqNucleotide::G, &3));
        dummy.G = 1;
        assert_eq!(dummy.mostfreq(), (ReqNucleotide::C, &2));
        dummy.C = 1;
    }

    #[test]
    fn mostfreq_compet_maximum() {
        let mut dummy = NucCounts { A: 1, C: 1, G: 1, T: 1 };
        // ordered when ncounters are equal
        assert_eq!(dummy.mostfreq(), (ReqNucleotide::A, &1));
        dummy.A = 0;
        assert_eq!(dummy.mostfreq(), (ReqNucleotide::C, &1));
        dummy.C = 0;
        assert_eq!(dummy.mostfreq(), (ReqNucleotide::G, &1));
        dummy.G = 0;
        assert_eq!(dummy.mostfreq(), (ReqNucleotide::T, &1));
    }

    #[test]
    fn add() {
        let mut a = NucCounts { A: 0, C: 1, G: 2, T: 3 };
        let b = NucCounts { A: 1, C: 2, G: 3, T: 4 };
        let result = NucCounts { A: 1, C: 3, G: 5, T: 7 };
        assert_eq!(a + b, result);
        a += b;
        assert_eq!(a, result);
    }
}
