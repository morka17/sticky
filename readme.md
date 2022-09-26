# Sticky interpreter 

__Starting with the alignment__

_The allocated size of an object is calculated by_
```rust
let alignment = size_of::<usize>() * 2;
let size = (size_of::<T>() & !(alignment - 1)) + alignment; 
```

>  The memory block as a base address and a size.
```rust
pub type blockPtr = NonNull<u8>;
pub type BlockSize = usize
```   

