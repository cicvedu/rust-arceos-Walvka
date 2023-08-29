//! Simple memory allocation.
//!
//! TODO: more efficient

use core::alloc::Layout;
use core::num::NonZeroUsize;

use crate::{AllocResult, BaseAllocator, ByteAllocator};

pub struct SimpleByteAllocator {
    start: usize,
    end: usize,
    allocated: usize,
    new_pointer: usize,
}

impl SimpleByteAllocator {
    pub const fn new() -> Self {
        Self {
            start: 0,
            end: 0,
            allocated: 0,
            new_pointer: 0,
        }
    }
}

impl BaseAllocator for SimpleByteAllocator {
    fn init(&mut self, _start: usize, _size: usize) {
        self.start = _start;
        self.end = self.start + _size;
        self.new_pointer = self.start;
        self.allocated = 0;
    }

    fn add_memory(&mut self, _start: usize, _size: usize) -> AllocResult {
        todo!();
    }
}

impl ByteAllocator for SimpleByteAllocator {
    fn alloc(&mut self, _layout: Layout) -> AllocResult<NonZeroUsize> {
        let size = _layout.size();
        let result = self.new_pointer;
        if self.new_pointer + size > self.end {
            Err(crate::AllocError::NoMemory)
        } else {
            self.allocated += 1;
            self.new_pointer += size;
            Ok(NonZeroUsize::new(result).unwrap())
        }
    }

    fn dealloc(&mut self, _pos: NonZeroUsize, _layout: Layout) {
        if self.allocated > 0 {
            self.allocated -= 1;
        } else {
            self.new_pointer = self.start;
        }
    }

    fn total_bytes(&self) -> usize {
        self.end - self.start
    }

    fn used_bytes(&self) -> usize {
        self.new_pointer - self.start
    }

    fn available_bytes(&self) -> usize {
        self.end - self.new_pointer
    }
}