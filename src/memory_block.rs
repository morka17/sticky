use std::ptr::NonNull;
use std::alloc::{alloc, dealloc, Layout};


/// Crate interface 
/// 
/// The block of memory as a base address and a size, 

pub struct Block {
    ptr: BlockPtr,
    size: BlockSize,
}



/// `[BlockPtr] and `[BlockSize]` are defined as 
pub type BlockPtr = NonNull<u8>;
pub type BlockSize = usize;


impl Block {
    /// To obtain a `[Block]` use the `[Block::new()]` function
    pub fn new(&mut self, size: BlockSize) -> Result<Block, BlockError>{
        if !size.is_power_of_two() {
            return Err(BlockError::BadRequest);
        }

        Ok(Block{
            ptr: Self::alloc_block(size)?,
            size,
        })
    }

    pub fn alloc_block(size: BlockSize) -> Result<BlockPtr, BlockError>{
        unsafe{
            let layout = Layout::from_size_align_unchecked(size, size);

            let ptr = alloc(layout);
            if ptr.is_null (){
                Err(BlockError::OOM)
            }else {
                Ok(NonNull::new_unchecked(ptr))
            }
        }
    }

    pub fn as_ptr(&self) -> *const u8{
        self.ptr.as_ptr()
    }

    pub fn dealloc_block(ptr: BlockPtr, size: BlockSize) {
        unsafe {
            let layout = Layout::from_size_align_unchecked(size, size);
            dealloc(ptr.as_ptr(),  layout);
        }
    }
}



/// Errors take one of two forms, an invalid block-size or out-of-memory,
/// both of which may be returned by `[Block::new()]`
pub enum BlockError{
    /// Usually means requested block size, and therefore alignment, wasn't a 
    /// power of two
    BadRequest,
    /// Insufficient memory, could'nt allocate a block
    OOM,
}





/// An interface that satisfies the interior mutability property, by borrowing
/// the allocator instance immutability,
trait AllocRaw{
    fn alloc<T>(&self, object: T) -> *const T;
}




#[cfg(test)]
mod test {
    #[test]
    fn allocate_its_works() {
        // the block address bitwise AND the alignment bits (size -1) should
        // be mutually exclusive set of bits.
        // let mask = size - 1;
        // assert!((block.ptr.as_ptr() as usize & mask) ^ mask == mask)
    }
}



// let alignment = size_of::<usize>() * a;
// // mask out the least significant bits that correspond to the alignment - 1
// // then add the full alignment 
// let size = (size_of::<T>() & !(alignment -1)) + alignment;