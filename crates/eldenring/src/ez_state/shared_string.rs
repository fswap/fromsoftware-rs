use std::{
    mem::transmute,
    ops::{Deref, DerefMut},
    sync::atomic::AtomicU32,
};

use pelite::pe64::Pe;
use shared::Program;
use vtable_rs::VPtr;

use crate::{
    dltx::DLString,
    dlut::{DLReferenceCountObject, DLReferenceCountObjectVmt, DLReferencePointer},
    rva,
};

#[repr(C)]
/// A referenced-counted pointer to a DLString used to pass data to and from EzState scripts
///
/// Source of name: RTTI
pub struct EzStateSharedString {
    vftable: VPtr<dyn DLReferenceCountObjectVmt, Self>,
    reference_count: AtomicU32,
    string: DLString,
}

impl<S: AsRef<str>> From<S> for DLReferencePointer<EzStateSharedString> {
    fn from(value: S) -> Self {
        DLReferencePointer::new(EzStateSharedString {
            vftable: unsafe {
                transmute::<u64, VPtr<dyn DLReferenceCountObjectVmt, EzStateSharedString>>(
                    Program::current()
                        .rva_to_va(rva::get().ez_state_shared_string_vmt)
                        .unwrap(),
                )
            },
            reference_count: AtomicU32::new(1),
            string: value.into(),
        })
    }
}

impl Deref for EzStateSharedString {
    type Target = DLString;

    fn deref(&self) -> &Self::Target {
        &self.string
    }
}

impl DerefMut for EzStateSharedString {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.string
    }
}

impl DLReferenceCountObject for EzStateSharedString {
    fn vtable(&self) -> VPtr<dyn DLReferenceCountObjectVmt, Self> {
        self.vftable
    }

    fn reference_count(&self) -> &AtomicU32 {
        &self.reference_count
    }
}
