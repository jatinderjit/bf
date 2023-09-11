Multiply cell 0 and cell 1
+++++++    set x=7 (or input the numbers)
> +++++    set y=5

helper cells:
  cell 2: restore x

Execute the loop y times (add x to cell 2 in each iteration)
[
  - <
  [ -    >>+    >+    <<< ]  sets x to 0 and increments cell 2 and 3 by x
  >>>                        pointer at cell 3
  [ -    <<<    +    >>> ]   restore x (move value from cell 3 to cell 0)
  >+                         increment cell 4
  <<<                        pointer at cell 1
]
>>>                          pointer at cell 4
[-<<<+>>>]                   restore y (move value from cell 4 to cell 1)
<<.                          print cell 2 which is now x*y
