mod tests;

use itertools::Itertools;
use std::cmp::max;
use std::collections::BTreeMap;
use std::convert::From;
use std::fmt;
use std::ops::BitOr;

pub fn fmt(items: &BTreeMap<u128, u128>) -> String {
    items
        .iter()
        .map(|(start, end)| format!("{start}..{end}"))
        .join(",")
}

fn len_slow(items: &BTreeMap<u128, u128>) -> u128 {
    items.iter().map(|(start, end)| end - start).sum()
}

pub fn internal_add(items: &mut BTreeMap<u128, u128>, len: &mut u128, start: u128, end: u128) {
    assert!(start < end); // !!!cmk check that length is not zero
                          // !!! cmk would be nice to have a partition_point function that returns two iterators
    let mut before = items.range_mut(..=start).rev();
    if let Some((start_before, end_before)) = before.next() {
        if *end_before < start {
            insert(items, len, start, end);
            *len += end - start;
        } else if *end_before < end {
            *len += end - *end_before;
            *end_before = end;
            let start_before = *start_before;
            delete_extra(items, len, start_before, end);
        } else {
            // completely contained, so do nothing
        }
    } else {
        insert(items, len, start, end);
        *len += end - start;
    }
}

fn delete_extra(items: &mut BTreeMap<u128, u128>, len: &mut u128, start: u128, end: u128) {
    let mut after = items.range_mut(start..);
    let (start_after, start_end) = after.next().unwrap(); // !!! cmk assert that there is a next
    assert!(start == *start_after && end == *start_end); // !!! cmk real assert
                                                         // !!!cmk would be nice to have a delete_range function
    let mut end_new = end;
    let delete_list = after
        .map_while(|(start_delete, end_delete)| {
            if *start_delete <= end {
                end_new = max(end_new, *end_delete);
                *len -= *end_delete - *start_delete;
                Some(*start_delete)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    if end_new > end {
        *len += end_new - end;
        *start_end = end_new;
    }
    for start in delete_list {
        items.remove(&start);
    }
}
fn insert(items: &mut BTreeMap<u128, u128>, len: &mut u128, start: u128, end: u128) {
    let was_there = items.insert(start, end);
    assert!(was_there.is_none());
    // !!!cmk real assert
    delete_extra(items, len, start, end);
}

// !!!cmk can I use a Rust range?
// !!!cmk allow negatives and any size

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct RangeSetInt {
    len: u128,
    items: BTreeMap<u128, u128>, // !!!cmk usize?
}

// !!!cmk support =, and single numbers
// !!!cmk error to use -
// !!!cmk are the unwraps OK?
// !!!cmk what about bad input?
impl From<&str> for RangeSetInt {
    fn from(s: &str) -> Self {
        let mut result = RangeSetInt::new();
        for range in s.split(',') {
            let mut range = range.split("..");
            let start = range.next().unwrap().parse::<u128>().unwrap();
            let end = range.next().unwrap().parse::<u128>().unwrap();
            result.internal_add(start, end);
        }
        result
    }
}

impl fmt::Debug for RangeSetInt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", fmt(&self.items))
    }
}

impl fmt::Display for RangeSetInt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", fmt(&self.items))
    }
}

impl RangeSetInt {
    pub fn new() -> RangeSetInt {
        RangeSetInt {
            items: BTreeMap::new(),
            len: 0,
        }
    }

    pub fn clear(&mut self) {
        self.items.clear();
        self.len = 0;
    }

    // !!!cmk keep this in a field
    pub fn len(&self) -> u128 {
        self.len
    }

    fn len_slow(&self) -> u128 {
        len_slow(&self.items)
    }

    /// Moves all elements from `other` into `self`, leaving `other` empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use rangeset_int::RangeSetInt;
    ///
    /// let mut a = RangeSetInt::from("1..4");
    /// let mut b = RangeSetInt::from("3..6");
    ///
    /// a.append(&mut b);
    ///
    /// assert_eq!(a.len(), 5);
    /// assert_eq!(b.len(), 0);
    ///
    /// assert!(a.contains(1));
    /// assert!(a.contains(2));
    /// assert!(a.contains(3));
    /// assert!(a.contains(4));
    /// assert!(a.contains(5));
    /// ```
    pub fn append(&mut self, other: &mut Self) {
        for (start, end) in other.items.iter() {
            self.internal_add(*start, *end);
        }
        other.clear();
    }

    /// Returns `true` if the set contains an element equal to the value.
    ///
    /// # Examples
    ///
    /// ```
    /// use rangeset_int::RangeSetInt;
    ///
    /// let set = RangeSetInt::from([1, 2, 3]);
    /// assert_eq!(set.contains(1), true);
    /// assert_eq!(set.contains(4), false);
    /// ```
    pub fn contains(&self, value: u128) -> bool {
        self.items
            .range(..=value)
            .next_back()
            .map_or(false, |(_, end)| value < *end)
    }

    // https://stackoverflow.com/questions/49599833/how-to-find-next-smaller-key-in-btreemap-btreeset
    // https://stackoverflow.com/questions/35663342/how-to-modify-partially-remove-a-range-from-a-btreemap
    fn internal_add(&mut self, start: u128, end: u128) {
        internal_add(&mut self.items, &mut self.len, start, end);
    }

    // fn _internal_add(&mut self, start: u128, length: u128) {
    //     // !!!cmk put this shortcut back?
    //     // if self._items.len() == 0 {
    //     //     self._items.insert(start, length);
    //     //     return;
    //     // }

    //     // !!!cmk rename index to "range"
    //     let range = self._items.range(..start);
    //     let mut peekable_forward = range.clone().peekable();
    //     let peek_forward = peekable_forward.peek();
    //     let mut peekable_backwards = range.rev().peekable();
    //     let peek_backwards = peekable_backwards.peek();
    //     if let Some(peek_forward) = peek_forward {
    //         let mut peek_forward = *peek_forward;
    //         if *peek_forward.0 == start {
    //             if length > *peek_forward.1 {
    //                 peek_forward.1 = &length;
    //                 // previous_range = peek_forward;
    //                 // peek_forward = peekable_forward.next(); // index should point to the following range for the remainder of this method
    //                 todo!()
    //             } else {
    //                 todo!();
    //             }
    //         }
    //     } else {
    //         println!("self._items.insert(start, length);");
    //         if let Some(previous_range) = peek_backwards {
    //             // nothing
    //         } else {
    //             return;
    //         }
    //     }

    //     todo!();
    //     //             return;
    //     //         }
    //     //     } else if index == 0 {
    //     //         self._items.insert(index, RangeX { start, length });
    //     //         previous_index = index;
    //     //         index += 1 // index_of_miss should point to the following range for the remainder of this method
    //     //     } else {
    //     //         previous_index = index - 1;
    //     //         let previous_range: &mut RangeX = &mut self._items[previous_index];

    //     //         if previous_range.end() >= start {
    //     //             let new_length = start + length - previous_range.start;
    //     //             if new_length <= previous_range.length {
    //     //                 return;
    //     //             } else {
    //     //                 previous_range.length = new_length;
    //     //             }
    //     //         } else {
    //     //             // after previous range, not contiguous with previous range
    //     //             self._items.insert(index, RangeX { start, length });
    //     //             previous_index = index;
    //     //             index += 1;
    //     //         }
    //     //     }
    //     // }

    //     // let previous_range: &RangeX = &self._items[previous_index];
    //     // let previous_end = previous_range.end();
    //     // while index < self._items.len() {
    //     //     let range: &RangeX = &self._items[index];
    //     //     if previous_end < range.start {
    //     //         break;
    //     //     }
    //     //     let range_end = range.end();
    //     //     if previous_end < range_end {
    //     //         self._items[previous_index].length = range_end - previous_range.start;
    //     //         index += 1;
    //     //         break;
    //     //     }
    //     //     index += 1;
    //     // }
    //     // self._items.drain(previous_index + 1..index);
    // }
}

impl BitOr<&RangeSetInt> for &RangeSetInt {
    type Output = RangeSetInt;

    /// Returns the union of `self` and `rhs` as a new `RangeSetInt`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rangeset_int::RangeSetInt;
    ///
    /// let a = RangeSetInt::from([1, 2, 3]);
    /// let b = RangeSetInt::from([3, 4, 5]);
    ///
    /// let result = &a | &b;
    /// assert_eq!(result, RangeSetInt::from([1, 2, 3, 4, 5]));
    /// ```
    fn bitor(self, rhs: &RangeSetInt) -> RangeSetInt {
        let mut result = self.clone();
        for (start, end) in rhs.items.iter() {
            result.internal_add(*start, *end);
        }
        result
    }
}

impl<const N: usize> From<[u128; N]> for RangeSetInt {
    fn from(arr: [u128; N]) -> Self {
        let mut result = RangeSetInt::new();
        for value in arr.iter() {
            result.internal_add(*value, *value + 1);
        }
        result
    }
}

impl IntoIterator for RangeSetInt {
    type Item = u128;
    type IntoIter = IntoIter;

    /// Gets an iterator for moving out the `RangeSetInt`'s contents.
    ///
    /// # Examples
    ///
    /// ```
    /// use rangeset_int::RangeSetInt;
    ///
    /// let set = RangeSetInt::from([1, 2, 3, 4]);
    ///
    /// let v: Vec<_> = set.into_iter().collect();
    /// assert_eq!(v, [1, 2, 3, 4]);
    /// ```
    fn into_iter(self) -> IntoIter {
        IntoIter {
            item_iter: 0..0,
            range_iter: self.items.into_iter(),
        }
    }
}

pub struct IntoIter {
    item_iter: core::ops::Range<u128>,
    range_iter: std::collections::btree_map::IntoIter<u128, u128>,
}

impl Iterator for IntoIter {
    type Item = u128;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(item) = self.item_iter.next() {
            return Some(item);
        }
        if let Some((start, end)) = self.range_iter.next() {
            self.item_iter = start..end;
            return self.next();
        }
        None
    }
}
