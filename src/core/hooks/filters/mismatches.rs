use std::ops::Range;

use derive_getters::Getters;
use derive_more::Constructor;
use itertools::zip;

use crate::core::mismatches::roi::ROIMismatchesPreview;
use crate::core::mismatches::site::SiteMismatchesPreview;

use super::MismatchesPreFilter;

#[derive(Constructor, Getters, Debug, PartialEq, Copy, Clone)]
pub struct ByMismatches {
    minmismatches: u32,
    minfreq: f32,
    mincov: u32,
}

impl ByMismatches {
    #[inline]
    fn ok_roi(&self, x: &ROIMismatchesPreview) -> bool {
        let (cov, mismatch) = (x.coverage(), x.mismatches());
        x.coverage() >= self.mincov && mismatch >= self.minmismatches && mismatch as f32 / cov as f32 >= self.minfreq
    }

    fn ok_site(&self, x: &SiteMismatchesPreview) -> bool {
        let cov = x.1.coverage();
        let mismatch = x.1.mismatches(x.0);
        cov >= self.mincov && mismatch >= self.minmismatches && mismatch as f32 / cov as f32 >= self.minfreq
    }

    // #[inline]
    // fn ok_ranges(&self, x: &impl IntervalMismatches) -> Vec<Range<usize>> {
    //     let mut result = Vec::new();
    //
    //     let mut prevind: Option<usize> = None;
    //     for (ind, (seqnc, refnc)) in zip(x.ncounts(), x.refnuc()).enumerate() {
    //         let cov = seqnc.coverage();
    //         let mismatch = seqnc.mismatches(refnc);
    //         let isok =
    //             cov >= self.mincov && mismatch >= self.minmismatches && mismatch as f32 / cov as f32 >= self.minfreq;
    //
    //         match (prevind, isok) {
    //             // Faulty range continues
    //             (None, false) => {}
    //             // Faulty range finished
    //             (None, true) => prevind = Some(ind),
    //             // Ok range continues
    //             (Some(_), true) => {}
    //             // Ok range finished
    //             (Some(prev), false) => {
    //                 result.push(prev..ind);
    //                 prevind = None;
    //             }
    //         }
    //     }
    //     if let Some(prev) = prevind {
    //         result.push(prev..x.ncounts().len());
    //     }
    //
    //     result
    // }
}

impl MismatchesPreFilter<ROIMismatchesPreview> for ByMismatches {
    #[inline]
    fn is_ok(&self, preview: &ROIMismatchesPreview) -> bool {
        self.ok_roi(&preview)
    }
}

impl MismatchesPreFilter<SiteMismatchesPreview> for ByMismatches {
    #[inline]
    fn is_ok(&self, preview: &SiteMismatchesPreview) -> bool {
        self.ok_site(preview)
    }
}

#[cfg(test)]
mod tests {
    use bio_types::genome::Interval;
    use bio_types::strand::Strand;

    use crate::core::dna::{NucCounts, Nucleotide};
    use crate::core::mismatches::roi::MismatchesSummary;
    use crate::core::workload::ROI;

    use super::*;

    #[test]
    fn ok_roi() {
        let mut dummy: MismatchesSummary = Default::default();

        dummy.A.C = 1;
        dummy.A.A = 4;

        dummy.C.G = 1;
        dummy.C.C = 2;

        dummy.G.T = 1;
        dummy.G.G = 5;

        dummy.T.A = 10;
        dummy.T.T = 3;
        // dummy coverage = 27, mismatches = 13, freq = 0.48148

        for (expected, minmismatches, minfreq, mincov) in [
            (false, 14, 0f32, 0),
            (true, 13, 0f32, 0),
            (true, 12, 0f32, 0),
            (true, 13, 0.48f32, 0),
            (false, 13, 0.5f32, 0),
            (true, 13, 0.48f32, 11),
            (true, 13, 0.48f32, 27),
            (false, 13, 0.48f32, 30),
            (false, 1, 0.48f32, 42),
        ] {
            let filter = ByMismatches::new(minmismatches, minfreq, mincov);
            assert_eq!(filter.ok_roi(&dummy), expected, "{} {} {}", minmismatches, minfreq, mincov);
        }
    }

    // #[test]
    // fn ok_interval() {
    //     let refnuc = [
    //         Nucleotide::A,
    //         Nucleotide::T,
    //         Nucleotide::G,
    //         Nucleotide::C,
    //         Nucleotide::A,
    //         Nucleotide::Unknown,
    //         Nucleotide::Unknown,
    //     ];
    //     let sequenced = [NucCounts { A: 1, C: 2, G: 3, T: 4 }].repeat(7);
    //
    //     let dummy = RefIntervalMismatches::new(Interval::new("1".into(), 10..17), Strand::Forward, &refnuc, &sequenced);
    //     for (expected, minmismatches, minfreq, mincov) in [
    //         (vec![0..7], 0, 0f32, 0),
    //         (vec![0..1, 3..7], 8, 0f32, 0),
    //         (vec![0..1, 4..7], 9, 0.9f32, 9),
    //         (vec![5..7], 10, 0.95f32, 10),
    //     ] {
    //         let filter = ByMismatches::new(minmismatches, minfreq, mincov);
    //         assert_eq!(filter.ok_ranges(&dummy), expected);
    //     }
    // }

    #[test]
    fn ok_site() {
        let mut dummy: SiteMismatchesPreview = (Nucleotide::A, NucCounts { A: 1, C: 2, G: 3, T: 4 });

        for (expected, minmismatches, minfreq, mincov) in [
            (false, 10, 0f32, 0),
            (true, 9, 0f32, 5),
            (true, 8, 0f32, 8),
            (true, 9, 0.85f32, 9),
            (false, 9, 0.95f32, 10),
            (true, 9, 0.85f32, 10),
            (false, 9, 0.85f32, 11),
        ] {
            let filter = ByMismatches { minmismatches, minfreq, mincov };
            assert_eq!(filter.is_ok(&dummy), expected);
        }

        dummy.0 = Nucleotide::Unknown;
        for (expected, minmismatches, minfreq, mincov) in [
            (true, 10, 0f32, 0),
            (true, 9, 0f32, 0),
            (false, 11, 0f32, 0),
            (true, 10, 1f32, 0),
            (false, 11, 1f32, 0),
            (true, 10, 1f32, 10),
            (false, 10, 1f32, 11),
        ] {
            let filter = ByMismatches { minmismatches, minfreq, mincov };
            assert_eq!(filter.is_ok(&dummy), expected);
        }
    }
}
