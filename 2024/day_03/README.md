# Day 03

parse out the garbage and run a virt machine

### approach

1. use RegEx to extract the well-formed instructions
2. run (and sum) them in a virtual machine (one instruction - multiply)
3. return sum

## Part 2

added a couple op codes -- do() and don't()

* need to augment the regex parsing to capture AND ORDER these
* modify the arg parsing for these new opcodes (as there are no args)
* modify the vm to enable/disable processing (i multiplied by 0/1)