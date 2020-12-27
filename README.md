# Star light

## Objective

You have a row of `N` stars, represented by a string of `0` or `1`, with a total of `N` characters in the string:
* `0` means that the star is **off**.
* `1` means that the star is **on**.

The rules are:
* The leftmost character on the left is star `1`.
* The right-most character is the star `N`.
* `N` must be 1 <= N <= 25.
* Each star has an independent switch that allows you to turn it on or off.
* To switch a star on or off, there are two rules:  
    1. You can change the state of star i if i+1 is on AND, if i < n - 1, i+2, i+3, ... `N` are off.
    2. Rule 1 does not apply to star `N`, which can be switched on or off whenever you want.

The game starts with a given lighting scheme start.  
You will also have a target lighting pattern target.

The objective is to find **the minimum number of switches** needed to change the pattern from `start` to `target`.

## Example

From `1101` to `0100`: 2 minimum switches.
From `101010` to `010101`: 26 minimum switches.

## Notes

Nothing really complex here - I implemented the tests first and the algorithm using a VecDeque "only".  
I added as parameter a user limit for my tests, in order to stop the internal loop (when computing the "next light to switch"), and I let it here just in case...

I also implemented the examples in a unit test.

## Run the tests && compile the project

```rust
cargo test && cargo build --release
```

## Run the project

```rust
./target/release/star_light
```
