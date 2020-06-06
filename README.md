# bf_compile
Optimizing interpreter for Brainfuck.

## Supported optimizations
* Replacing arbitrary number of `>` with `move pointer n cells forward`
* Replacing arbitrary number of `<` with `move pointer n cells backward`
* Replacing arbitrary number of `+` with `add n to current cell`
* Replacing arbitrary number of `-` with `substract n from current cell`
* Replacing `[-]` with `set value of current cell to 0`
* Replacing `[>>>]` with `move pointer forward n cells at a time, until cell with value=0 found`
* Replacing `[<<<]` with `move pointer backward n cells at a time, until cell with value=0 found`

## Performance before and after
Interpreter was benchmarked before and after implementing optimizations using [the following BF-app](http://esoteric.sange.fi/brainfuck/bf-source/prog/mandelbrot.b).

Results on my laptop (Core i7-6700HQ @ 2.60 GHz):
|             | Time (ms)  |
| ----------- |:----------:|
| Before      | 25122.0416 |
| After       | 8109.403   |
