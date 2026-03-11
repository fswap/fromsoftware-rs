use std::{
    alloc::{GlobalAlloc, Layout, handle_alloc_error},
    ops::{Deref, DerefMut},
    ptr::NonNull,
    sync::atomic::{AtomicU32, Ordering},
};

use vtable_rs::VPtr;

use crate::dlkr::DLHeapAllocator;

#[repr(transparent)]
/// A reference counted pointer to an object that implements `DLReferenceCountObject`
///
/// Source of name: RTTI
pub struct DLReferencePointer<T: DLReferenceCountObject>(NonNull<T>);

impl<T: DLReferenceCountObject> DLReferencePointer<T> {
    /// Creates a new pointer to a reference-counted object with value `T`.
    ///
    /// This uses [`DLHeapAllocator::runtime`], which is a safe default for all
    /// known reference-counted types.
    pub fn new(data: T) -> Self {
        let layout = Layout::new::<T>();

        // Some `DLReferenceCountObject`s use `DLAllocator::GetHeapAllocatorOf`
        // to deallocate their data, but others just use
        // `::GetSystemAllocatorOf`. Since `GetHeapAllocator` supports system
        // allocators but `GetSystemAllocator` doesn't support dynamic heap
        // allocators, we always allocate with the runtime (system) allocator.
        match NonNull::new(unsafe { DLHeapAllocator::runtime().alloc(layout) }.cast::<T>()) {
            Some(new) => {
                unsafe { new.write(data) };
                Self(new)
            }
            None => handle_alloc_error(layout),
        }
    }
}

impl<T: DLReferenceCountObject> Deref for DLReferencePointer<T> {
    type Target = T;

    fn deref(&self) -> &T {
        // Safety: the data constructed by `Self::new` is guaranteed to be a valid reference, and
        // it is only invalidated when the last reference is dropped
        unsafe { self.0.as_ref() }
    }
}

impl<T: DLReferenceCountObject> DerefMut for DLReferencePointer<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // Safety: the data constructed by `Self::new` is guaranteed to be a valid reference, and
        // it is only invalidated when the last reference is dropped
        unsafe { self.0.as_mut() }
    }
}

impl<T: DLReferenceCountObject> Clone for DLReferencePointer<T> {
    fn clone(&self) -> Self {
        // Increment the reference count and return a clone that points to the same string
        self.reference_count().fetch_add(1, Ordering::Relaxed);
        Self(self.0)
    }
}

impl<T: DLReferenceCountObject> Drop for DLReferencePointer<T> {
    fn drop(&mut self) {
        // Decrement the reference count and free the shared string if this is the last reference
        if self.reference_count().fetch_sub(1, Ordering::Relaxed) == 1 {
            (self.vtable().destroy)(self);
        }
    }
}

#[vtable_rs::vtable]
pub trait DLReferenceCountObjectVmt {
    /// Deletes `this`, running the destructor and freeing the memory. This is run automatically
    /// when the last reference is dropped.
    fn destroy(&mut self);

    fn destructor(&mut self);
}

/// Trait implemented by objects managed by a `DLReferencePointer`
///
/// Source of name: RTTI
pub trait DLReferenceCountObject: Sized + 'static {
    fn vtable(&self) -> VPtr<dyn DLReferenceCountObjectVmt, Self>;

    fn reference_count(&self) -> &AtomicU32;
}
