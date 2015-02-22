#![macro_use]

use std::collections::BitVec;

macro_rules! for_flags {
    ($f:expr, $($p:path => $e:expr),+) => {
        for &f in [$($p),+].iter() {
            if $f.contains(f) {
                match f {
                    $($p => $e,)+
                    _ => unreachable!()
                }
            }
        }
    }
}

pub trait SliceSubsetsExt {
    type Item;

    fn subsets(&self) -> Subsets<Self::Item>;
}

impl<T> SliceSubsetsExt for [T] {
    type Item = T;

    fn subsets(&self) -> Subsets<T> {
        Subsets {
            slice: self,
            current: BitVec::from_elem(self.len(), false),
            produced_empty: false
        }
    }
}

pub struct Subsets<'a, T: 'a> {
    slice: &'a [T],
    current: BitVec,
    produced_empty: bool
}

impl<'a, T: 'a> Subsets<'a, T> {
    #[inline]
    fn snapshot(&self) -> Subset<'a, T> {
        Subset {
            slice: self.slice,
            selected: self.current.clone(),
            current: 0
        }
    }
}

impl<'a, T: 'a> Iterator for Subsets<'a, T> {
    type Item = Subset<'a, T>;

    fn next(&mut self) -> Option<Subset<'a, T>> {
        if !self.produced_empty {
            self.produced_empty = true;
            return Some(self.snapshot());
        }

        let mut i = 0;
        while i < self.current.len() {
            if !self.current[i] {
                self.current.set(i, true);
                break;
            } else {
                self.current.set(i, false);
                i += 1;
            }
        }

        if i == self.current.len() {
            None
        } else {
            Some(self.snapshot())
        }
    }
}

pub struct Subset<'a, T: 'a> {
    slice: &'a [T],
    selected: BitVec,
    current: usize
}

impl<'a, T: 'a> Iterator for Subset<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        let mut r = None;
        while self.current < self.selected.len() {
            if self.selected[self.current] {
                r = Some(unsafe { self.slice.get_unchecked(self.current) });
                self.current += 1;
                break;
            }
            self.current += 1;
        }
        r
    }
}

pub mod precision {
    use std::num::Float;

    pub fn digits_to_bits(digits: u32) -> u32 {
        const LOG2_10: f64 = 3.3219280948873624;
        (digits as f64 * LOG2_10).ceil() as u32
    }

    pub fn bits_to_digits(bits: u32) -> u32 {
        const LOG10_2: f64 = 0.30102999566398119;
        (bits as f64 * LOG10_2).floor() as u32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[inline(always)]
    pub fn deref<T: Copy>(x: &T) -> T { *x }

    #[test]
    fn test_empty_slice_subsets() {
        static S: &'static [u8] = &[];

        let it = S.subsets();

        let subsets = it.collect::<Vec<_>>();
        assert_eq!(1, subsets.len());

        assert_eq!(0, subsets.into_iter().next().unwrap().count());
    }

    #[test]
    fn test_subsets() {
        static S: &'static [u8] = &[1, 2, 3];

        let r: Vec<Vec<u8>> = S.subsets().map(|s| s.map(deref).collect()).collect();

        assert_eq!(8, r.len());

        assert_eq!(
            r,
            vec![
                vec![],
                vec![1],
                vec![2],
                vec![1, 2],
                vec![3],
                vec![1, 3],
                vec![2, 3],
                vec![1, 2, 3]
            ]
        );
    }
}
