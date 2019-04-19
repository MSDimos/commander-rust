# why this ?
For a long time, developing cli in `Rust` is difficult.
Since Rust is a static language, the compiler needs to know all the details at compile time. 
It conflicts with the dynamics of the CLI.
The community offers a wide range of solutions. Yes, they're excellent, but they're not very simple.

Inspired by [commander.js](https://github.com/tj/commander.js) & [rocket.rs](https://rocket.rs), 
I developed this crate.

# limit
If you want to use this crate, please guarantee that you have follow rules below:
+ using `Rust 2018`
+  
