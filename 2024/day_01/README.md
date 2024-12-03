# Day 01 - Historian Hysteria

## sample input

is given as two lists in columns that need to be compared by taking the differnces of the lowest number of each list, then sum the differences.

### approach

1. parse the input into a polars data frame (overkill, but this may be a handy reusable component).  Extract the columns.
2. for each column, create a min-heap
3. pop the bottom of the two min-heaps and take the distance as abs value into a distances vector
4. sum the vector

## Part II

that approach worked well for Part I. It could be optimized and avoid putting everything in and out of a DataFrame, but at least that separated the input parsing and helped me learn some more Polars. I tried to read the input as CSV, etc. but it was getting too ugly, so settled for using RegEx, another crate i need to learn to use better/more.

For Part II, we need to do 'different' stats. The "left" list (or `x` in my code) is the 'key' and the right (`y`) can be used to build a histogram.  The goal now is for each x, scale it by the frequency of that x in the y list.  Total the scaled sums. 

### approach

1. parse as before
2. no need for min-heaps. Instead, compute a histogram (really a term-frequency matrix) for the "right" column / `y` values
3. for each x, find the frequency of x in y (f) and `sum += f(x)*x`, where f(x) is the frequency of the value x in the y list

to vectorize, I'll make a series for `f` and then do a dot product with `x` and sum the product.