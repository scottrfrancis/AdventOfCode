# Day 04

Seems like it should be a backtracking puzzle, but brute force should solve it just fine.
Having a good datastructure and traversal mechanism would be helpful.

### approach

* seems like having a 'good grid' implementation is always a stumble in AOC w/ Rust. So maybe try to fix that.
* then traverse l -> r , t -> b, starting from 'X' and then search the 8 adjancent directions for a string of 'MAS' in that direction.
  - build strings of 3 chars in each of the 8 directions
  - direction isn't 3 deep? skip it
  - test each of the (up to 8) strings to == 'MAS'
* total.  move to next 'X'