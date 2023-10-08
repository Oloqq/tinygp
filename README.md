Program is a list of opcodes (integers, in java implementation used interchangeably with chars).

Some opcodes denote an arithmetic operation like
```
ADD = 110
SUB = 111
MUL = 112
DIV = 113
```
The next 2 opcodes are argument of the operation

Values below 110 are references to actual values. \
Let N = number of variables in the problem. \
`[0:N)` - those are placeholders to be filled with input values of each training case \
`[N:110)` - those are numerical constants for the problem. Those are only generated once at the start, and not mutated further. \
Values above 113 are not permitted.

This is an exemplary program along it's internal representation
```
(X1  + (-4.025456902691228 / -4.025456902691228))
[110, 0, 113, 37, 37]
```