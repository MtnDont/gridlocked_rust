# GriDLocked Map Algorithm
Built entirely using Rust std libraries. Custom Random struct implements Xorshift algorithm to provide random values within ranges.

Method for choosing new blocks could be improved by tracking active blocks bordering non-active blocks.

## Building
```
rustc main.rs
```

## Example Output
```
Please input a seed: 
0
Seed: 1684731575
0 0 0 0 0 0 0 0 0 0 
0 0 0 0 0 1 0 0 0 0 
0 0 0 0 0 1 1 1 0 0 
0 1 1 1 1 1 1 1 0 0 
0 1 1 1 1 1 1 1 0 0 
0 1 1 1 1 1 1 1 1 0 
0 0 1 1 1 1 1 1 0 0 
0 0 0 1 1 1 0 0 0 0 
0 0 0 1 1 0 0 0 0 0 
0 0 0 0 0 0 0 0 0 0 

Time to Execute: 65.39µs
```