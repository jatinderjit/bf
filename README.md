# BF

[BF](https://en.wikipedia.org/wiki/Brainfuck) Interpreter.

Usage:

```terminal
cargo run -- <path/to/source>
```

## Introduction

```brainfuck
+     Increment the current byte by 1
-     Decrement the current byte by 1
>     Move the pointer to the right by 1 byte
<     Move the pointer to the left by 1 byte
[     Enter the loop if the current byte is nonzero;
      Else advance to the instruction after the matching bracket
]     Go the loop start
,     Read into the current byte
.     Output the current byte (ascii value)

Everything else is a comment
```

### Common Constructs

- Set current cell to zero: `[-]`
- Cat: `.[.,]`
- Find the next zero: `[>]` (or `--[++>--]++` for the next 2)
- Move the contents of a cell from one cell to another: `[->+<]`
- Add two bytes: `[>+<-]` (destructive)
- To persist the original value, copy then add: `[->>+<<]>>[-<+<+>>]`
- Subtract one cell from another: `>>[-]<< [>+<-]>[<+<->>-]`

#### Multiplication (Annotated)

Multiplication is implemented as repeated addition.

```brainfuck
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
```

The equivalent C program would look like this:

```c
#import "stdio.h"

int main(int argc, char *argv[]) {

  int x = 7;
  int y = 5;

  int i = 0, j = 0, k = 0;

  while (y > 0) {
    y -= 1;
    while (x > 0) {
      x -= 1;
      i += 1;
      j += 1;
    }
    while (j > 0) {
      j -= 1;
      x += 1;
    }
    k += 1;
  }
  while (k > 0) {
    y += 1;
    k -= 1;
  }
  printf("%c", i);
  return 0;
}
```

## References

- [Compiling Brainfuck Code](https://rodrigodd.github.io/2022/10/21/bf_compiler-part1.html).
