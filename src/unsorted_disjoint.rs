// !!!cmk make the names consistent, start/lower vs end/upper/end/...
// !!!cmk replace OptionRange with Option<RangeInclusive<T>>

use crate::{Integer, SortedDisjoint, SortedStarts};
use itertools::Itertools;
use num_traits::Zero;
use std::{
    cmp::{max, min},
    ops::RangeInclusive,
};

#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct UnsortedDisjoint<T, I>
where
    T: Integer,
    I: Iterator<Item = RangeInclusive<T>>,
{
    iter: I,
    option_range_inclusive: Option<RangeInclusive<T>>,
    min_value_plus_2: T,
    two: T,
}

impl<T, I> From<I> for UnsortedDisjoint<T, I::IntoIter>
where
    T: Integer,
    I: IntoIterator<Item = RangeInclusive<T>>, // Any iterator is fine
{
    fn from(into_iter: I) -> Self {
        UnsortedDisjoint {
            iter: into_iter.into_iter(),
            option_range_inclusive: None,
            min_value_plus_2: T::min_value() + T::one() + T::one(),
            two: T::one() + T::one(),
        }
    }
}

impl<T, I> Iterator for UnsortedDisjoint<T, I>
where
    T: Integer,
    I: Iterator<Item = RangeInclusive<T>>,
{
    type Item = RangeInclusive<T>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(range_inclusive) = self.iter.next() {
                let (next_start, next_end) = range_inclusive.into_inner();
                if next_start > next_end {
                    continue;
                }
                assert!(next_end <= T::max_value2()); // !!!cmk0 raise error on panic?
                if let Some(self_range_inclusive) = self.option_range_inclusive.clone() {
                    let (self_start, self_end) = self_range_inclusive.into_inner();
                    if (next_start >= self.min_value_plus_2 && self_end <= next_start - self.two)
                        || (self_start >= self.min_value_plus_2
                            && next_end <= self_start - self.two)
                    {
                        let result = Some(self_start..=self_end);
                        self.option_range_inclusive = Some(next_start..=next_end);
                        return result;
                    } else {
                        self.option_range_inclusive =
                            Some(min(self_start, next_start)..=max(self_end, next_end));
                        continue;
                    }
                } else {
                    self.option_range_inclusive = Some(next_start..=next_end);
                    continue;
                }
            } else if let Some(range_inclusive) = self.option_range_inclusive.clone() {
                self.option_range_inclusive = None;
                return Some(range_inclusive);
            } else {
                return None;
            }
        }
    }

    // As few as one (or zero if iter is empty) and as many as iter.len()
    // There could be one extra if option_range_inclusive is Some.
    fn size_hint(&self) -> (usize, Option<usize>) {
        let (lower, upper) = self.iter.size_hint();
        let lower = if lower == 0 { 0 } else { 1 };
        if self.option_range_inclusive.is_some() {
            (lower, upper.map(|x| x + 1))
        } else {
            (lower, upper)
        }
    }
}

// Can't implement fmt::Display fmt must take ownership
impl<T, I> UnsortedDisjoint<T, I>
where
    T: Integer,
    I: Iterator<Item = RangeInclusive<T>>,
{
    #[allow(clippy::inherent_to_string)]
    pub fn to_string(self) -> String {
        self.map(|range_inclusive| {
            let (start, end) = range_inclusive.into_inner();
            format!("{start}..={end}") // cmk could we format RangeInclusive directly?
        })
        .join(", ")
    }
}

#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct SortedDisjointWithLenSoFar<T, I>
where
    T: Integer,
    I: Iterator<Item = RangeInclusive<T>> + SortedDisjoint,
{
    iter: I,
    len: <T as Integer>::SafeLen,
}

// cmk Rule there is no reason From's should be into iterators
impl<T: Integer, I> From<I> for SortedDisjointWithLenSoFar<T, I::IntoIter>
where
    I: IntoIterator<Item = RangeInclusive<T>>,
    I::IntoIter: SortedDisjoint,
{
    fn from(into_iter: I) -> Self {
        SortedDisjointWithLenSoFar {
            iter: into_iter.into_iter(),
            len: <T as Integer>::SafeLen::zero(),
        }
    }
}

impl<T: Integer, I> SortedDisjointWithLenSoFar<T, I>
where
    I: Iterator<Item = RangeInclusive<T>> + SortedDisjoint,
{
    pub fn len_so_far(&self) -> <T as Integer>::SafeLen {
        self.len
    }
}

impl<T: Integer, I> Iterator for SortedDisjointWithLenSoFar<T, I>
where
    I: Iterator<Item = RangeInclusive<T>> + SortedDisjoint,
{
    type Item = (T, T);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(range_inclusive) = self.iter.next() {
            let (start, end) = range_inclusive.clone().into_inner();
            debug_assert!(start <= end && end <= T::max_value2());
            self.len += T::safe_inclusive_len(&range_inclusive);
            Some((start, end))
        } else {
            None
        }
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}
impl<T: Integer, I> SortedDisjoint for SortedDisjointWithLenSoFar<T, I> where
    I: Iterator<Item = RangeInclusive<T>> + SortedDisjoint
{
}
impl<T: Integer, I> SortedStarts for SortedDisjointWithLenSoFar<T, I> where
    I: Iterator<Item = RangeInclusive<T>> + SortedDisjoint
{
}

#[derive(Clone)]
#[must_use = "iterators are lazy and do nothing unless consumed"]

/// A wrapper around an iterator of ranges that
/// assumes that the ranges are sorted by start, but not necessarily by end.
///
/// It implements the [`SortedStarts`] trait which is required on inputs to
/// `SortedDisjointIter` iterator.
// cmk0000 bad link
pub struct AssumeSortedStarts<T, I>
where
    T: Integer,
    I: Iterator<Item = RangeInclusive<T>>,
{
    pub(crate) iter: I,
}

impl<T: Integer, I> SortedStarts for AssumeSortedStarts<T, I> where
    I: Iterator<Item = RangeInclusive<T>>
{
}

impl<T, I> AssumeSortedStarts<T, I>
where
    T: Integer,
    I: Iterator<Item = RangeInclusive<T>>,
{
    pub fn new(iter: I) -> Self {
        AssumeSortedStarts { iter }
    }
}

impl<T, I> Iterator for AssumeSortedStarts<T, I>
where
    T: Integer,
    I: Iterator<Item = RangeInclusive<T>>,
{
    type Item = RangeInclusive<T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }

    // !!!cmk rule add a size hint, but think about if it is correct with respect to other fields
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}
