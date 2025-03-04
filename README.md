# Learning Rust with a BBS system
Working on a bbs in rust to learn rust
 

## Notes
- rules of ownership have different implications depending on whether our data is stored on the stack or the heap.
- If we assign a variable to an existing variable with a stack-based type such as i32, it will make a computationally inexpensive copy of that value.
- When working with datatypes that utilize the heap, such as String, we cannot copy values from one variable to another since heap-based types do not implement the Copy trait. Instead of copying, Rust will instead move the value out of the original variable and into the new one.
- Anytime a variable is declared with `let` it is stored in memory