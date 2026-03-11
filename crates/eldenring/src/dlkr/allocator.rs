use std::{
    alloc::{GlobalAlloc, Layout},
    mem::transmute,
    ptr::NonNull,
};

use pelite::pe64::Pe;
use shared::*;
use vtable_rs::VPtr;

use crate::rva;

#[vtable_rs::vtable]
pub trait DLAllocatorVmt {
    fn destructor(&mut self, param_2: bool);

    /// Getter for the allocator ID.
    fn allocator_id(&self) -> u32;

    fn unk10(&self);

    fn heap_flags(&self) -> &u64;

    fn heap_capacity(&self) -> usize;

    fn heap_size(&self) -> usize;

    fn backing_heap_capacity(&self) -> usize;

    fn heap_allocation_count(&self) -> usize;

    /// Retrieves allocation size for a specific allocation.
    fn allocation_size(&self, allocation: *const u8) -> usize;

    fn allocate(&mut self, size: usize) -> *const u8;

    fn allocate_aligned(&mut self, size: usize, alignment: usize) -> *const u8;

    fn reallocate(&mut self, allocation: *const u8, size: usize) -> *const u8;

    fn reallocate_aligned(
        &mut self,
        allocation: *const u8,
        size: usize,
        alignment: usize,
    ) -> *const u8;

    fn deallocate(&mut self, allocation: *const u8);

    fn allocate_second(&mut self, size: usize) -> *const u8;

    fn allocate_aligned_second(&mut self, size: usize, alignment: usize) -> *const u8;

    fn reallocate_second(&mut self, allocation: *const u8, size: usize) -> *const u8;

    fn reallocate_aligned_second(
        &mut self,
        allocation: *const u8,
        size: usize,
        alignment: usize,
    ) -> *const u8;

    fn deallocate_second(&mut self, allocation: *const u8);

    fn unka0(&self) -> bool;

    fn allocation_belongs_to_first_allocator(&mut self, allocation: *const u8) -> bool;

    fn allocation_belongs_to_second_allocator(&mut self, allocation: *const u8) -> bool;

    fn lock(&mut self);

    fn unlock(&mut self);

    fn get_memory_block_for_allocation(&mut self, allocation: *const u8) -> *const u8;
}

pub struct DLAllocatorBase {
    pub vftable: VPtr<dyn DLAllocatorVmt, Self>,
}

#[repr(transparent)]
#[derive(Clone)]
pub struct DLAllocatorRef(NonNull<DLAllocatorBase>);

unsafe impl GlobalAlloc for DLAllocatorRef {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let allocator = self.0.as_ptr();
        unsafe { ((*allocator).vftable.allocate)(&mut *allocator, layout.size()) as *mut u8 }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        let allocator = self.0.as_ptr();
        unsafe {
            ((*allocator).vftable.deallocate)(&mut *allocator, ptr);
        }
    }
}

impl From<NonNull<DLAllocatorBase>> for DLAllocatorRef {
    fn from(ptr: NonNull<DLAllocatorBase>) -> Self {
        Self(ptr)
    }
}

impl DLAllocatorVmt for DLAllocatorBase {
    extern "C" fn destructor(&mut self, _param_2: bool) {
        todo!()
    }

    extern "C" fn allocator_id(&self) -> u32 {
        todo!()
    }

    extern "C" fn unk10(&self) {
        todo!()
    }

    extern "C" fn heap_flags(&self) -> &u64 {
        todo!()
    }

    extern "C" fn heap_capacity(&self) -> usize {
        todo!()
    }

    extern "C" fn heap_size(&self) -> usize {
        todo!()
    }

    extern "C" fn backing_heap_capacity(&self) -> usize {
        todo!()
    }

    extern "C" fn heap_allocation_count(&self) -> usize {
        todo!()
    }

    extern "C" fn allocation_size(&self, _allocation: *const u8) -> usize {
        todo!()
    }

    extern "C" fn allocate(&mut self, _size: usize) -> *const u8 {
        todo!()
    }

    extern "C" fn allocate_aligned(&mut self, size: usize, alignment: usize) -> *const u8 {
        (self.vftable.allocate_aligned)(self, size, alignment)
    }

    extern "C" fn reallocate(&mut self, _allocation: *const u8, _size: usize) -> *const u8 {
        todo!()
    }

    extern "C" fn reallocate_aligned(
        &mut self,
        _allocation: *const u8,
        _size: usize,
        _alignment: usize,
    ) -> *const u8 {
        todo!()
    }

    extern "C" fn deallocate(&mut self, _allocation: *const u8) {
        todo!()
    }

    extern "C" fn allocate_second(&mut self, _size: usize) -> *const u8 {
        todo!()
    }

    extern "C" fn allocate_aligned_second(&mut self, _size: usize, _alignment: usize) -> *const u8 {
        todo!()
    }

    extern "C" fn reallocate_second(&mut self, _allocation: *const u8, _size: usize) -> *const u8 {
        todo!()
    }

    extern "C" fn reallocate_aligned_second(
        &mut self,
        _allocation: *const u8,
        _size: usize,
        _alignment: usize,
    ) -> *const u8 {
        todo!()
    }

    extern "C" fn deallocate_second(&mut self, _allocation: *const u8) {
        todo!()
    }

    extern "C" fn unka0(&self) -> bool {
        todo!()
    }

    extern "C" fn allocation_belongs_to_first_allocator(&mut self, _allocation: *const u8) -> bool {
        todo!()
    }

    extern "C" fn allocation_belongs_to_second_allocator(
        &mut self,
        _allocation: *const u8,
    ) -> bool {
        todo!()
    }

    extern "C" fn lock(&mut self) {
        todo!()
    }

    extern "C" fn unlock(&mut self) {
        todo!()
    }

    extern "C" fn get_memory_block_for_allocation(&mut self, _allocation: *const u8) -> *const u8 {
        todo!()
    }
}

/// A zero-typed struct representing the set of allocators used by the game that
/// allocate onto the main heap.
///
/// All `HeapAllocator<DLKR::DLDynamicHeap<...>>` allocators should be
/// compatible with this, as well as
/// `HeapAllocator<DLKR::Win32RuntimeHeapImpl>`.
pub struct DLHeapAllocator;

impl DLHeapAllocator {
    /// Returns the game's main dynamic heap allocator.
    pub fn main() -> DLAllocatorRef {
        unsafe {
            transmute::<u64, DLAllocatorRef>(
                Program::current()
                    .rva_to_va(rva::get().main_heap_allocator_ptr)
                    .unwrap(),
            )
        }
    }

    /// Returns the global instance of [`DLAllocatorBase`] that uses the
    /// standard MSVC `malloc()`/`free()` implementation for heap management.
    pub fn runtime() -> DLAllocatorRef {
        unsafe {
            transmute::<u64, DLAllocatorRef>(
                Program::current()
                    .rva_to_va(rva::get().runtime_heap_allocator)
                    .unwrap(),
            )
        }
    }
}

impl GameAllocator for DLHeapAllocator {
    fn allocate(layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        match NonNull::new(unsafe { DLHeapAllocator::main().alloc(layout) }) {
            Some(ptr) => Ok(NonNull::slice_from_raw_parts(ptr, layout.size())),
            None => Err(AllocError),
        }
    }

    unsafe fn deallocate(ptr: NonNull<u8>, layout: Layout) {
        unsafe {
            let call = transmute::<u64, extern "C" fn(NonNull<u8>) -> DLAllocatorRef>(
                Program::current()
                    .rva_to_va(rva::get().dlallocator_get_heap_allocator_of)
                    .unwrap(),
            );
            let allocator = call(ptr);
            allocator.dealloc(ptr.as_ptr(), layout);
        }
    }
}
