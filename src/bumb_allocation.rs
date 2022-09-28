use crate::Block;

use std::ptr::write;

/// Block size will be 32K, a resonably optimal size arrived at in the
/// original Immix paper
pub const BLOCK_SIZE_BITS: usize = 15;
pub const BLOCK_SIZE: usize = 1 << BLOCK_SIZE_BITS;
pub const LINE_SIZE_BITS: usize = 7;
pub const LINE_SIZE: usize = 1 << LINE_SIZE_BITS;
pub const LINE_COUNT: usize = BLOCK_SIZE / LINE_SIZE;

/// Struct that wraps the block with a bump pointer and other metadata.
pub struct BumpBlock {
    cursor: usize,
    limit: usize,
    block: Block,
    meta: Box<BlockMeta>,
}
// cursor is the index into the block where the next object can be written.
// block: this the block itself in which objects will be written.

impl BumpBlock {
    pub fn inner_alloc(&mut self, alloc_size: usize) -> Option<*const u8> {
        let next_bump = self.cursor + alloc_size;

        if next_bump > BLOCK_SIZE {
            None
        } else {
            let offset = self.cursor;
            self.cursor = next_bump;
            unsafe { Some(self.block.as_ptr().add(offset) as *const u8) }
        }
    }

    unsafe fn write<T>(dest: *const u8, object: T) {
        write(dest as *mut T, object);
    }
}

/// `[line_mark]` is an array of boolean flags, one for each line in block,
/// to indicate whether it has been marked or not.
/// `[block_mark]` simply says whether the entire block has marked objects in it.
/// if this is ever `false`, the entire block can be deallocated.
pub struct BlockMeta {
    line_mark: [bool; LINE_COUNT],
    block_mark: bool,
}

impl BlockMeta {
    pub fn find_next_available_hole(&self, starting_at: usize) -> Option<(usize, usize)> {
        let mut count = 0;
        let mut start: Option<usize> = None;
        let mut stop: usize = 0;

        let starting_line = starting_at / LINE_SIZE;

        for (index, marked) in self.line_mark[starting_line..].iter().enumerate() {
            let abs_index = starting_line + index;

            // count unmarked lines
            if !*marked {
                count += 1;
                // if this is the first line in a hole (and not the zeroth line), consider it
                // conservatively marked and skip to the next line
                if count == 1 && abs_index > 0 {
                    continue;
                }

                // record the first hole index
                if start.is_none() {
                    start = Some(abs_index);
                }

                // stop is now at the end of this line
                stop = abs_index + 1;
            }

            // when reached a marked line or the end of the block, there is a valid hole to work with
            if count > 0 && (*marked || stop >= LINE_COUNT) {
                if let Some(start) = start{
                    let cursor = start * LINE_SIZE;
                    let limit = stop * LINE_SIZE;

                    return Some((cursor, limit))
                }
            }

            // if this line is marked and did'nt return a new cursor/limit pair by now,
            // reset the hole state
            if *marked {
                count = 0;
                start = None;
            }
        }

        None
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn bump_allocate_it_works() {
        
    }
}
