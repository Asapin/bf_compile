# bf_compile
Learning Rust by writing optimizing Brainfuck interpreter first and then trying to rewrite it as a compile-time interpreter.

## Optimizing interpreter
Optimizing interpreter works by parsing source code into a vector of [IrCommand](./src/enums.rs) first, and then by converting it into a vector of [Command](./src/enums.rs),
while applying different optimizations to make resulting code more compact and faster.
Also, as it is not possible to read user input during compilation, `,` in BF programs (reading user input) is ignored, which means that not all BF programs are supported.

### Supported optimizations
* Replacing arbitrary number of `>` with `move pointer n cells forward`
* Replacing arbitrary number of `<` with `move pointer n cells backward`
* Replacing arbitrary number of `+` with `add n to current cell`
* Replacing arbitrary number of `-` with `substract n from current cell`
* Replacing `[-]` with `set value of current cell to 0`
* Replacing `[>>>]` with `move pointer forward n cells at a time, until cell with value=0 found`
* Replacing `[<<<]` with `move pointer backward n cells at a time, until cell with value=0 found`

### Performance before and after
Interpreter was benchmarked before and after implementing optimizations using [the following BF-app](http://esoteric.sange.fi/brainfuck/bf-source/prog/mandelbrot.b).

Results on my laptop (Core i7-6700HQ @ 2.60 GHz):
|             | Time (ms)  |
| ----------- |:----------:|
| Before      | 25122.0416 |
| After       | 8109.403   |

## Compile time optimizer

Currently compile-time is implemented using build script.

### Build.rs

All code responsible for parsing and executing BF programs is separated into an `interpreter` library. That library is then used as a build dependency.
Build.rs loads source code written in BF and calls `VM::run` method, which optimizes and executes it. The result is then returned back to build.rs and is used to generate `result.rs` with a single method:

```
pub fn result() -> &'static str {
    "<BF output here>"
}
```


That file is then included into `main.rs`, so we can call `result()` and print it's value:

```
include!(concat!(env!("OUT_DIR"), "/result.rs"));

fn main() {
    println!("{}", result());
}
```

Resulting source code is available in the `lib.rs` branch
