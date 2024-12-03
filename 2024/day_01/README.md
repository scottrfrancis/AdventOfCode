# Day 01 - Historian Hysteria

## sample input

is given as two lists in columns that need to be compared by taking the differnces of the lowest number of each list, then sum the differences.

### approach

1. parse the input into a polars data frame (overkill, but this may be a handy reusable component).  Extract the columns.
2. for each column, create a min-heap
3. pop the bottom of the two min-heaps and take the distance as abs value into a distances vector
4. sum the vector 