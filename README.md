Solutions to all 500 puzzles from Advent of Code 2015-2024: https://adventofcode.com/

Almost all of this code was written as quickly as possible (about 6-10 hours per year),
and never cleaned up after getting the correct answer out of it,
so it's pretty horrible.

This was done in Rust, written almost entirely from scratch without the use of
third-party libraries (because it's more fun to figure out and understand the algorithms myself),
except for a few standard features that would be too hard or too boring to reimplement:

* Commonly used:
  * `itertools` for `collect_vec`, `permutations`, etc
  * `peg` for parsing the more complicated syntaxes (I also tried `lalrpop` and `pest`, but `peg` worked the best)
  * `rayon` for easily parallelising some computations, because optimised brute force can be quicker than finding a cleverer algorithm
  * `std` obviously
* Rarely used:
  * `crossbeam` for some multithreading
  * `fancy-regex` for regexes with backrefs
  * `md-5`, `hex` for puzzles that require MD5
  * `nalgebra` for some 2D/3D vectors, when I couldn't be bothered to do all the maths explicitly
  * `num` for `bigint` and `rational`, when brute-forcing and finding `i128` isn't big enough
  * `serde_json` for parsing JSON subsets
  * `thiserror` for writing a reasonably-generic pathfinding library that was reused in many puzzles
