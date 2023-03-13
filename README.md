range-set-int
==========

cmk
[![github](https://img.shields.io/badge/github-anyinput-8da0cb?style=flat&labelColor=555555&logo=github)](https://github.com/CarlKCarlK/anyinput)
[![crates.io](https://img.shields.io/crates/v/anyinput.svg?flat&color=fc8d62&logo=rust")](https://crates.io/crates/anyinput)
[![docs.rs](https://img.shields.io/badge/docs.rs-anyinput-66c2a5?flat&labelColor=555555&logoColor=white&logo=core:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K)](https://docs.rs/anyinput)[![CI](https://github.com/CarlKCarlK/anyinput/actions/workflows/ci.yml/badge.svg)](https://github.com/CarlKCarlK/anyinput/actions/workflows/ci.yml)

A crate for efficiently manipulating ranges of integers (including negatives and up to u128) using set operations such as `union()`, `intersection()`, and `difference()`.

The crate differs from sets in the standard library (such as `BTreeSet` and `HashSet`) because it does not need to store every element in the set, only for every contiguous range of elements. It differs from other interval libraries (that we know of) by being specialized and optimized for integer elements.

Example 1
---------

Here we take the union (operator “|”) of two RangeSetInt's:

![Example 1](doc/example1.png "Example 1")

```rust
use range_set_int::RangeSetInt;

 // a is the set of integers from 100 to 499 (inclusive) and 501 to 1000 (inclusive)
let a = RangeSetInt::from([100..=499,501..=999]);
 // b is the set of integers -20 and the range 400 to 599 (inclusive)
let b = RangeSetInt::from([-20..=-20,400..=599]);
// c is the union of a and b, namely -20 and 100 to 999 (inclusive)
let c = a | b;
assert_eq!(c, RangeSetInt::from([-20..=-20,100..=999]));
```

Example 2
---------

In biology, suppose we want to find the intron regions of a gene but we are given only the transcription region and the exon regions.

![Example 2](doc/example2.png "Example 2")

We create a `RangeSetInt` for the transcription region and a `RangeSetInt` for all the exon regions.
Then we take the difference of the transcription region and exon regions to find the intron regions.

```rust
use range_set_int::RangeSetInt;

let line = "chr15   29370   37380   29370,32358,36715   30817,32561,37380";

// split the line on white space
let mut iter = line.split_whitespace();
let chr = iter.next().unwrap();

// Parse the start and end of the transcription region into a RangeSetInt
let trans_start: i32 = iter.next().unwrap().parse().unwrap();
let trans_end: i32 = iter.next().unwrap().parse().unwrap();
let trans = RangeSetInt::from([trans_start..=trans_end]);
assert_eq!(trans, RangeSetInt::from([29370..=37380]));

// Parse the start and end of the exons into a RangeSetInt
let exon_starts = iter.next().unwrap().split(',').map(|s| s.parse::<i32>());
let exon_ends = iter.next().unwrap().split(',').map(|s| s.parse::<i32>());
let exon_ranges = exon_starts
    .zip(exon_ends)
    .map(|(s, e)| s.unwrap()..=e.unwrap());
let exons = RangeSetInt::from_iter(exon_ranges);
assert_eq!(
    exons,
    RangeSetInt::from([29370..=30817, 32358..=32561, 36715..=37380])
);

// Use 'set subtraction' to find the introns
let intron = trans - exons;
assert_eq!(intron, RangeSetInt::from([30818..=32357, 32562..=36714]));
for range in intron.ranges() {
    let (start, end) = range.into_inner();
    println!("{chr}\t{start}\t{end}");
}
```
