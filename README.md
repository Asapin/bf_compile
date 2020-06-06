# bf_compile
Optimizing interpreter for Brainfuck.

Currently supported optimizations:
* Replacing arbitrary number of `>` with `move pointer n cells forward`
* Replacing arbitrary number of `<` with `move pointer n cells backward`
* Replacing arbitrary number of `+` with `add n to current cell`
* Replacing arbitrary number of `-` with `substract n from current cell`
* Replacing `[-]` with `set value of current cell to 0`
* Replacing `[>>>]` with `move pointer forward n cells at a time, until cell with value=0 found`
* Replacing `[<<<]` with `move pointer backward n cells at a time, until cell with value=0 found`
