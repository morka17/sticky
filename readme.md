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

__Secondly with the Bump Allocation__
>The bump allocation - we have a pointer, the bump pointer, which points at the space in the block after the last object that was written. when the next object is written, the bump pointer is incremented to point to the space after that object
_the block size will be 32K. This size can be any power ot two though and different use cases may show different optimal sizes._
```rust
pub const BLOCK_SIZE_BITS: usize = 15;
pub const BLOCK_SIZE: usize = 1 << BLOCK_SIZE_BITS;
```