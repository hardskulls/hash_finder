# hash_finder

## 📖 About
`hash_finder` is a program that finds `F` results of natural number hashes 
with `N` zeroes at the end of the hash.

⚠️ Note that when `N` is 7 or greater, the algorithm significantly slows down.

## Usage
Use `cargo run -r -- -F <RESULTS> -N <ZEROS>` to run the algorithm.  
`cargo run -r` part is for running program in optimized mode (⚠️ this is important).  
`--` here means "pass the following to the generated program".