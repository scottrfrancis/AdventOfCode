# Day 02

check to see if a series derivative exceeds a rate

### approach

1. parse each line into a Polars Series
2. compute the derivative of the series as a pair-wise difference
3. filter the derivative on the threshold (> 3 in the puzzle)
4. if the filter has ANY trues, the level is unsafe
5. count the number of SAFE levels

(Could also apply the derivate function 3x to see if it fully zeros)