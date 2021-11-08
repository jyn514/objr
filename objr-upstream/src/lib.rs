#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2018::*;
#[macro_use]
extern crate std;
extern crate self as objr;
pub mod macros {
    #[macro_export]
    macro_rules! objc_enum {
        ($(#[$attribute : meta]) * $pub : vis struct $enum : ident < $type :
         ty > ; impl $ignore : ident { $($a : ident = $b : expr), * }) =>
        ($(#[$attribute]) * $pub struct $enum($type) ;
         #[allow(non_upper_case_globals)] impl $enum
         {
             $($pub const $a : $enum = $enum($b) ;) * $pub const fn
             field(& self) -> $type { self.0 }
         })
    }
}
mod class {
    use std::ffi::{c_void, CStr};
    use super::performselector::PerformablePointer;
    use super::bindings::*;
    use std::os::raw::c_char;
    use core::marker::PhantomData;
    use std::fmt::Formatter;
    #[link(name = "objc", kind = "dylib")]
    extern "C" {
        fn objc_lookUpClass(name: *const c_char)
        -> *mut c_void;
    }
    #[repr(transparent)]
    pub struct AnyClass(c_void);
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for AnyClass {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            loop { }
        }
    }
    impl PartialEq for AnyClass {
        fn eq(&self, other: &Self) -> bool { loop { } }
    }
    pub trait ObjcClass: ObjcInstance + Sized {
        fn class()
        -> &'static Class<Self>;
    }
    #[repr(transparent)]
    pub struct Class<T: ObjcClass>(c_void, PhantomData<T>);
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl <T: ::core::fmt::Debug + ObjcClass> ::core::fmt::Debug for Class<T> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            loop { }
        }
    }
    unsafe impl <T: ObjcClass> PerformablePointer for Class<T> { }
    impl <T: ObjcClass> PartialEq for Class<T> {
        fn eq(&self, other: &Self) -> bool { loop { } }
    }
    impl <T: ObjcClass> Class<T> {
        pub unsafe fn from_str(cstr: &CStr) -> &'static Self { loop { } }
        pub fn as_anyclass(&self) -> &'static AnyClass { loop { } }
    }
    impl <T: ObjcClass> Class<T> {
        pub fn alloc_init(&self, pool: &ActiveAutoreleasePool)
         -> StrongCell<T> {
            loop { }
        }
        pub unsafe fn alloc(&self, pool: &ActiveAutoreleasePool) -> *mut T {
            loop { }
        }
        pub unsafe fn assume_nonmut_perform(&self) -> *mut Self { loop { } }
    }
    impl <T: ObjcClass> std::fmt::Display for Class<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result { loop { } }
    }
    #[macro_export]
    macro_rules! objc_class {
        ($(#[$attribute : meta]) * $pub : vis struct $objctype : ident
         { @ class($objcname : ident) }) =>
        {
            :: objr :: bindings :: objc_instance!
            { $(#[$attribute]) * $pub struct $objctype ; } :: objr :: bindings
            :: __objc_implement_class! { $objctype, $objcname }
        } ;
    }
    #[macro_export]
    macro_rules! objc_class_newtype {
        ($(#[$attribute : meta]) * $pub : vis struct $newtype : ident
         $(< $($T : ident), + >) ? : $oldtype : ident ;) =>
        {
            :: objr :: bindings :: objc_instance_newtype!
            {
                $(#[$attribute]) * $pub struct $newtype $(< $($T), + >) ? :
                $oldtype ;
            } impl $(< $($T), + >) ? objr :: bindings :: ObjcClass for
            $newtype $(< $($T), + >) ?
            {
                fn class() -> & 'static Class < Self >
                { unsafe { std :: mem :: transmute($oldtype :: class()) } }
            }
        }
    }
}
mod objectpointers {
    /*! object pointer types

For safe types:

1.  AutoreleasedCell - part of an autorelease pool
2.  StrongCell - Compiler emits retain/release calls.

Mutable variants:

1.  AutoreleasedMutCell - like [AutoreleasedCell] but mutable
2.  StrongMutCell - like [StrongCell] but mutable

Lifetime variants:
1.  StrongLifetimeCell - like [StrongCell] but tracks some explicit lifetime.  Often used for objects that borrow Rust storage.


See documentation for particular cells.
*/
    use core::ffi::{c_void};
    use crate::bindings::{ActiveAutoreleasePool, ObjcInstance};
    use std::marker::PhantomData;
    use crate::objcinstance::NonNullImmutable;
    use std::ptr::NonNull;
    use std::fmt::{Debug};
    use std::hash::{Hash, Hasher};
    extern "C" {
        fn objc_autoreleaseReturnValue(object: *const c_void)
        -> *const c_void;
    }
    const DEBUG_MEMORY: bool = false;
    #[link(name = "objc", kind = "dylib")]
    extern "C" {
        fn objc_retain(ptr: *const c_void)
        -> *const c_void;
        fn objc_release(ptr: *const c_void);
        fn objc_autorelease(ptr: *const c_void);
    }
    /**
An objc object that is part of an autorelease pool

The pool is used to lexically scope the lifetime of the pointer.
*/
    pub struct AutoreleasedCell<'a, T> {
        ptr: NonNullImmutable<T>,
        marker: PhantomData<&'a T>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl <'a, T: ::core::fmt::Debug> ::core::fmt::Debug for
     AutoreleasedCell<'a, T> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            loop { }
        }
    }
    impl <'a, T: ObjcInstance> AutoreleasedCell<'a, T> {
        pub fn autoreleasing(cell: &T, _pool: &'a ActiveAutoreleasePool)
         -> Self {
            loop { }
        }
        pub unsafe fn assume_autoreleased(ptr: &T,
                                          _pool: &'a ActiveAutoreleasePool)
         -> Self {
            loop { }
        }
        pub unsafe fn assume_mut(self) -> AutoreleasedMutCell<'a, T> {
            loop { }
        }
    }
    impl <'a, T: ObjcInstance> std::ops::Deref for AutoreleasedCell<'a, T> {
        type Target = T;
        #[inline]
        fn deref(&self) -> &T { loop { } }
    }
    impl <'a, T: ObjcInstance> std::fmt::Display for AutoreleasedCell<'a, T>
     where T: std::fmt::Display {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            loop { }
        }
    }
    impl <'a, T: PartialEq + ObjcInstance> PartialEq for
     AutoreleasedCell<'a, T> {
        fn eq(&self, other: &Self) -> bool { loop { } }
    }
    impl <'a, T: Eq + ObjcInstance> Eq for AutoreleasedCell<'a, T> { }
    impl <'a, T: Hash + ObjcInstance> Hash for AutoreleasedCell<'a, T> {
        fn hash<H: Hasher>(&self, state: &mut H) { loop { } }
    }
    /**
An objc object that is part of an autorelease pool

The pool is used to lexically scope the lifetime of the pointer.
 */
    pub struct AutoreleasedMutCell<'a, T> {
        ptr: NonNull<T>,
        marker: PhantomData<&'a T>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl <'a, T: ::core::fmt::Debug> ::core::fmt::Debug for
     AutoreleasedMutCell<'a, T> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            loop { }
        }
    }
    impl <'a, T: ObjcInstance> AutoreleasedMutCell<'a, T> {
        pub fn autoreleasing(cell: &mut T, _pool: &'a ActiveAutoreleasePool)
         -> Self {
            loop { }
        }
        pub unsafe fn assume_autoreleased(ptr: &mut T,
                                          _pool: &'a ActiveAutoreleasePool)
         -> Self {
            loop { }
        }
    }
    impl <'a, T: ObjcInstance> std::ops::Deref for AutoreleasedMutCell<'a, T>
     {
        type Target = T;
        #[inline]
        fn deref(&self) -> &T { loop { } }
    }
    impl <'a, T: ObjcInstance> std::ops::DerefMut for
     AutoreleasedMutCell<'a, T> {
        #[inline]
        fn deref_mut(&mut self) -> &mut T { loop { } }
    }
    impl <'a, T: ObjcInstance> std::fmt::Display for
     AutoreleasedMutCell<'a, T> where T: std::fmt::Display {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            loop { }
        }
    }
    impl <'a, T: PartialEq + ObjcInstance> PartialEq for
     AutoreleasedMutCell<'a, T> {
        fn eq(&self, other: &Self) -> bool { loop { } }
    }
    impl <'a, T: Eq + ObjcInstance> Eq for AutoreleasedMutCell<'a, T> { }
    impl <'a, T: Hash + ObjcInstance> Hash for AutoreleasedMutCell<'a, T> {
        fn hash<H: Hasher>(&self, state: &mut H) { loop { } }
    }
    /**
A strong pointer to an objc object.

This is often the type you want as the return
type when implementing an ObjC binding.

When this type is created, we will `retain` (unless using an unsafe [StrongCell::assume_retained()] constructor)
When the obj is dropped, we will `release`.

In ObjC, the compiler tries to elide retain/release but it
may not be possible due to lack of global knowledge, in which
case it inserts `retain` as a precaution.

In Rust we have global knowledge of lifetimes so we can
elide more perfectly.  However this requires splitting up
objc `strong` into an explicit typesystem.

This type emits `retain`/`release` unconditionally.  Therefore
you can think of it like the "worst case" of objc `strong`, the
case where the compiler cannot elide anything.  You can also think of
it as a "lifetime eraser", that is we erase knowledge of the object lifetime,
so we assume we need to retain.

This is often used at the border of an objc binding.

For an elided 'best case' version, see `RefCell`.
*/
    pub struct StrongCell<T: ObjcInstance>(NonNullImmutable<T>);
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl <T: ::core::fmt::Debug + ObjcInstance> ::core::fmt::Debug for
     StrongCell<T> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            loop { }
        }
    }
    impl <T: ObjcInstance> StrongCell<T> {
        pub fn retaining(cell: &T) -> Self { loop { } }
        pub fn autoreleasing<'a>(cell: &Self, pool: &'a ActiveAutoreleasePool)
         -> AutoreleasedCell<'a, T> {
            loop { }
        }
        pub unsafe fn assume_retained(reference: &T) -> Self { loop { } }
        pub unsafe fn assume_mut(self) -> StrongMutCell<T> { loop { } }
        #[inline(always)]
        pub fn return_autoreleased(self) -> *const T { loop { } }
    }
    impl <T: ObjcInstance> Clone for StrongCell<T> {
        fn clone(&self) -> Self { loop { } }
    }
    impl <T: ObjcInstance> Drop for StrongCell<T> {
        fn drop(&mut self) { loop { } }
    }
    impl <T: ObjcInstance> std::ops::Deref for StrongCell<T> {
        type Target = T;
        #[inline]
        fn deref(&self) -> &T { loop { } }
    }
    impl <'a, T: ObjcInstance> std::fmt::Display for StrongCell<T> where
     T: std::fmt::Display {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            loop { }
        }
    }
    impl <T: PartialEq + ObjcInstance> PartialEq for StrongCell<T> {
        fn eq(&self, other: &Self) -> bool { loop { } }
    }
    impl <T: Eq + ObjcInstance> Eq for StrongCell<T> { }
    impl <T: Hash + ObjcInstance> Hash for StrongCell<T> {
        fn hash<H: Hasher>(&self, state: &mut H) { loop { } }
    }
    unsafe impl <T: ObjcInstance> Send for StrongCell<T> { }
    unsafe impl <T: ObjcInstance + Sync> Sync for StrongCell<T> { }
    pub struct StrongLifetimeCell<'a,
                                  T: ObjcInstance>(NonNullImmutable<T>,
                                                   PhantomData<&'a ()>);
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl <'a, T: ::core::fmt::Debug + ObjcInstance> ::core::fmt::Debug for
     StrongLifetimeCell<'a, T> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            loop { }
        }
    }
    impl <'a, T: ObjcInstance> StrongLifetimeCell<'a, T> {
        pub fn retaining(cell: &'a T) -> Self { loop { } }
        pub fn autoreleasing<'b: 'a>(cell: &'a Self,
                                     pool: &'b ActiveAutoreleasePool)
         -> AutoreleasedCell<'b, T> {
            loop { }
        }
        pub unsafe fn assume_retained_limited(reference: &'a T) -> Self {
            loop { }
        }
    }
    impl <'a, T: ObjcInstance> Drop for StrongLifetimeCell<'a, T> {
        fn drop(&mut self) { loop { } }
    }
    impl <'a, T: ObjcInstance> std::ops::Deref for StrongLifetimeCell<'a, T> {
        type Target = T;
        #[inline]
        fn deref(&self) -> &T { loop { } }
    }
    impl <'a, T: ObjcInstance> std::fmt::Display for StrongLifetimeCell<'a, T>
     where T: std::fmt::Display {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            loop { }
        }
    }
    impl <'a, T: PartialEq + ObjcInstance> PartialEq for
     StrongLifetimeCell<'a, T> {
        fn eq(&self, other: &Self) -> bool { loop { } }
    }
    impl <'a, T: Eq + ObjcInstance> Eq for StrongLifetimeCell<'a, T> { }
    impl <'a, T: Hash + ObjcInstance> Hash for StrongLifetimeCell<'a, T> {
        fn hash<H: Hasher>(&self, state: &mut H) { loop { } }
    }
    pub struct StrongMutCell<T: ObjcInstance>(NonNull<T>);
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl <T: ::core::fmt::Debug + ObjcInstance> ::core::fmt::Debug for
     StrongMutCell<T> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            loop { }
        }
    }
    impl <T: ObjcInstance> StrongMutCell<T> {
        pub fn retaining(cell: &mut T) -> Self { loop { } }
        pub fn autoreleasing<'a>(cell: &mut Self,
                                 pool: &'a ActiveAutoreleasePool)
         -> AutoreleasedMutCell<'a, T> {
            loop { }
        }
        pub fn as_const(self) -> StrongCell<T> { loop { } }
    }
    impl <T: ObjcInstance> StrongMutCell<T> {
        pub unsafe fn assume_retained(reference: &mut T) -> Self { loop { } }
        pub fn return_autoreleased(self) -> *mut T { loop { } }
    }
    unsafe impl <T: ObjcInstance> Send for StrongMutCell<T> { }
    impl <T: ObjcInstance> Drop for StrongMutCell<T> {
        fn drop(&mut self) { loop { } }
    }
    impl <T: ObjcInstance> std::ops::Deref for StrongMutCell<T> {
        type Target = T;
        #[inline]
        fn deref(&self) -> &T { loop { } }
    }
    impl <T: ObjcInstance> std::ops::DerefMut for StrongMutCell<T> {
        #[inline]
        fn deref_mut(&mut self) -> &mut T { loop { } }
    }
    impl <'a, T: ObjcInstance> std::fmt::Display for StrongMutCell<T> where
     T: std::fmt::Display {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            loop { }
        }
    }
    impl <T: PartialEq + ObjcInstance> PartialEq for StrongMutCell<T> {
        fn eq(&self, other: &Self) -> bool { loop { } }
    }
    impl <T: Eq + ObjcInstance> Eq for StrongMutCell<T> { }
    impl <T: Hash + ObjcInstance> Hash for StrongMutCell<T> {
        fn hash<H: Hasher>(&self, state: &mut H) { loop { } }
    }
}
mod nsobject {
    use objr::bindings::{ActiveAutoreleasePool, Sel};
    use super::nsstring::NSString;
    use super::objcinstance::ObjcInstance;
    use super::performselector::PerformsSelector;
    use super::bindings::*;
    extern { }
    pub trait NSObjectSelectors {
        unsafe fn alloc()
        -> ::objr::bindings::Sel;
        unsafe fn description()
        -> ::objr::bindings::Sel;
        unsafe fn respondsToSelector_()
        -> ::objr::bindings::Sel;
        unsafe fn init()
        -> ::objr::bindings::Sel;
        unsafe fn conformsToProtocol_()
        -> ::objr::bindings::Sel;
        unsafe fn dealloc()
        -> ::objr::bindings::Sel;
        unsafe fn copy()
        -> ::objr::bindings::Sel;
    }
    impl NSObjectSelectors for objr::bindings::Sel {
        unsafe fn alloc() -> ::objr::bindings::Sel {
            #[inline(never)]
            unsafe fn codegen_workaround() -> ::objr::bindings::Sel {
                #[link_section = "__TEXT,__objc_methname,cstring_literals"]
                static L_OBJC_METH_VAR_NAME_: [u8; 6] = *b"alloc\0";
                #[link_section =
                  "__DATA,__objc_selrefs,literal_pointers,no_dead_strip"]
                static L_OBJC_SELECTOR_REFERENCES_: &'static [u8; 6] =
                    &L_OBJC_METH_VAR_NAME_;
                loop { }
            }
            loop { }
        }
        unsafe fn description() -> ::objr::bindings::Sel {
            #[inline(never)]
            unsafe fn codegen_workaround() -> ::objr::bindings::Sel {
                #[link_section = "__TEXT,__objc_methname,cstring_literals"]
                static L_OBJC_METH_VAR_NAME_: [u8; 12] = *b"description\0";
                #[link_section =
                  "__DATA,__objc_selrefs,literal_pointers,no_dead_strip"]
                static L_OBJC_SELECTOR_REFERENCES_: &'static [u8; 12] =
                    &L_OBJC_METH_VAR_NAME_;
                loop { }
            }
            loop { }
        }
        unsafe fn respondsToSelector_() -> ::objr::bindings::Sel {
            #[inline(never)]
            unsafe fn codegen_workaround() -> ::objr::bindings::Sel {
                #[link_section = "__TEXT,__objc_methname,cstring_literals"]
                static L_OBJC_METH_VAR_NAME_: [u8; 20] =
                    *b"respondsToSelector:\0";
                #[link_section =
                  "__DATA,__objc_selrefs,literal_pointers,no_dead_strip"]
                static L_OBJC_SELECTOR_REFERENCES_: &'static [u8; 20] =
                    &L_OBJC_METH_VAR_NAME_;
                loop { }
            }
            loop { }
        }
        unsafe fn init() -> ::objr::bindings::Sel {
            #[inline(never)]
            unsafe fn codegen_workaround() -> ::objr::bindings::Sel {
                #[link_section = "__TEXT,__objc_methname,cstring_literals"]
                static L_OBJC_METH_VAR_NAME_: [u8; 5] = *b"init\0";
                #[link_section =
                  "__DATA,__objc_selrefs,literal_pointers,no_dead_strip"]
                static L_OBJC_SELECTOR_REFERENCES_: &'static [u8; 5] =
                    &L_OBJC_METH_VAR_NAME_;
                loop { }
            }
            loop { }
        }
        unsafe fn conformsToProtocol_() -> ::objr::bindings::Sel {
            #[inline(never)]
            unsafe fn codegen_workaround() -> ::objr::bindings::Sel {
                #[link_section = "__TEXT,__objc_methname,cstring_literals"]
                static L_OBJC_METH_VAR_NAME_: [u8; 20] =
                    *b"conformsToProtocol:\0";
                #[link_section =
                  "__DATA,__objc_selrefs,literal_pointers,no_dead_strip"]
                static L_OBJC_SELECTOR_REFERENCES_: &'static [u8; 20] =
                    &L_OBJC_METH_VAR_NAME_;
                loop { }
            }
            loop { }
        }
        unsafe fn dealloc() -> ::objr::bindings::Sel {
            #[inline(never)]
            unsafe fn codegen_workaround() -> ::objr::bindings::Sel {
                #[link_section = "__TEXT,__objc_methname,cstring_literals"]
                static L_OBJC_METH_VAR_NAME_: [u8; 8] = *b"dealloc\0";
                #[link_section =
                  "__DATA,__objc_selrefs,literal_pointers,no_dead_strip"]
                static L_OBJC_SELECTOR_REFERENCES_: &'static [u8; 8] =
                    &L_OBJC_METH_VAR_NAME_;
                loop { }
            }
            loop { }
        }
        unsafe fn copy() -> ::objr::bindings::Sel {
            #[inline(never)]
            unsafe fn codegen_workaround() -> ::objr::bindings::Sel {
                #[link_section = "__TEXT,__objc_methname,cstring_literals"]
                static L_OBJC_METH_VAR_NAME_: [u8; 5] = *b"copy\0";
                #[link_section =
                  "__DATA,__objc_selrefs,literal_pointers,no_dead_strip"]
                static L_OBJC_SELECTOR_REFERENCES_: &'static [u8; 5] =
                    &L_OBJC_METH_VAR_NAME_;
                loop { }
            }
            loop { }
        }
    }
    pub trait NSObjectTrait: Sized + ObjcInstance {
        fn description<'a>(&self, pool: &ActiveAutoreleasePool)
        -> StrongCell<NSString>;
        fn responds_to_selector(&self, pool: &ActiveAutoreleasePool, sel: Sel)
        -> bool;
        fn copy(&self, pool: &ActiveAutoreleasePool)
        -> StrongCell<Self>;
        unsafe fn init(receiver: *mut *mut Self,
                       pool: &ActiveAutoreleasePool);
        fn as_nsobject(&self)
        -> &NSObject;
    }
    impl <T: ObjcInstance> NSObjectTrait for T {
        fn description<'a>(&self, pool: &ActiveAutoreleasePool)
         -> StrongCell<NSString> {
            loop { }
        }
        fn responds_to_selector(&self, pool: &ActiveAutoreleasePool, sel: Sel)
         -> bool {
            loop { }
        }
        fn copy(&self, pool: &ActiveAutoreleasePool) -> StrongCell<Self> {
            loop { }
        }
        unsafe fn init(receiver: *mut *mut Self,
                       pool: &ActiveAutoreleasePool) {
            loop { }
        }
        fn as_nsobject(&self) -> &NSObject { loop { } }
    }
    mod no_constructNSObject {
        #[repr(transparent)]
        pub struct NSObject(core::ffi::c_void);
        impl ::objr::bindings::ObjcInstance for NSObject { }
        impl std::fmt::Display for NSObject {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                use ::objr::foundation::NSObjectTrait;
                loop { }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::fmt::Debug for NSObject {
            fn fmt(&self, f: &mut ::core::fmt::Formatter)
             -> ::core::fmt::Result {
                loop { }
            }
        }
    }
    pub use no_constructNSObject::NSObject;
    impl ::objr::bindings::ObjcClass for NSObject {
        fn class() -> &'static ::objr::bindings::Class<NSObject> {
            #[inline(never)]
            unsafe fn merge_compilation_units()
             -> &'static ::objr::bindings::Class<NSObject> {
                extern {
                    #[link_name = "\x01_OBJC_CLASS_$_NSObject"]
                    static CLASS: *mut core::ffi::c_void ;
                }
                #[link_section =
                  "__DATA,__objc_classrefs,regular,no_dead_strip"]
                static CLASS_REF: &'static ::objr::bindings::Class<NSObject> =
                    unsafe { std::mem::transmute(&CLASS) };
                loop { }
            }
            loop { }
        }
    }
}
mod nsstring {
    use super::bindings::*;
    use std::ffi::{CStr};
    use std::hash::{Hash, Hasher};
    use std::os::raw::{c_char};
    use crate::objcinstance::NonNullImmutable;
    use objr::typealias::NSUInteger;
    mod no_constructNSString {
        #[repr(transparent)]
        pub struct NSString(core::ffi::c_void);
        impl ::objr::bindings::ObjcInstance for NSString { }
        impl std::fmt::Display for NSString {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                use ::objr::foundation::NSObjectTrait;
                loop { }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::fmt::Debug for NSString {
            fn fmt(&self, f: &mut ::core::fmt::Formatter)
             -> ::core::fmt::Result {
                loop { }
            }
        }
    }
    pub use no_constructNSString::NSString;
    impl ::objr::bindings::ObjcClass for NSString {
        fn class() -> &'static ::objr::bindings::Class<NSString> {
            #[inline(never)]
            unsafe fn merge_compilation_units()
             -> &'static ::objr::bindings::Class<NSString> {
                extern {
                    #[link_name = "\x01_OBJC_CLASS_$_NSString"]
                    static CLASS: *mut core::ffi::c_void ;
                }
                #[link_section =
                  "__DATA,__objc_classrefs,regular,no_dead_strip"]
                static CLASS_REF: &'static ::objr::bindings::Class<NSString> =
                    unsafe { std::mem::transmute(&CLASS) };
                loop { }
            }
            loop { }
        }
    }
    pub trait NSStringSelectors {
        unsafe fn UTF8String()
        -> ::objr::bindings::Sel;
        unsafe fn initWithBytes_length_encoding()
        -> ::objr::bindings::Sel;
        unsafe fn isEqualToString_()
        -> ::objr::bindings::Sel;
        unsafe fn hash()
        -> ::objr::bindings::Sel;
    }
    impl NSStringSelectors for objr::bindings::Sel {
        unsafe fn UTF8String() -> ::objr::bindings::Sel {
            #[inline(never)]
            unsafe fn codegen_workaround() -> ::objr::bindings::Sel {
                #[link_section = "__TEXT,__objc_methname,cstring_literals"]
                static L_OBJC_METH_VAR_NAME_: [u8; 11] = *b"UTF8String\0";
                #[link_section =
                  "__DATA,__objc_selrefs,literal_pointers,no_dead_strip"]
                static L_OBJC_SELECTOR_REFERENCES_: &'static [u8; 11] =
                    &L_OBJC_METH_VAR_NAME_;
                loop { }
            }
            loop { }
        }
        unsafe fn initWithBytes_length_encoding() -> ::objr::bindings::Sel {
            #[inline(never)]
            unsafe fn codegen_workaround() -> ::objr::bindings::Sel {
                #[link_section = "__TEXT,__objc_methname,cstring_literals"]
                static L_OBJC_METH_VAR_NAME_: [u8; 31] =
                    *b"initWithBytes:length:encoding:\0";
                #[link_section =
                  "__DATA,__objc_selrefs,literal_pointers,no_dead_strip"]
                static L_OBJC_SELECTOR_REFERENCES_: &'static [u8; 31] =
                    &L_OBJC_METH_VAR_NAME_;
                loop { }
            }
            loop { }
        }
        unsafe fn isEqualToString_() -> ::objr::bindings::Sel {
            #[inline(never)]
            unsafe fn codegen_workaround() -> ::objr::bindings::Sel {
                #[link_section = "__TEXT,__objc_methname,cstring_literals"]
                static L_OBJC_METH_VAR_NAME_: [u8; 17] =
                    *b"isEqualToString:\0";
                #[link_section =
                  "__DATA,__objc_selrefs,literal_pointers,no_dead_strip"]
                static L_OBJC_SELECTOR_REFERENCES_: &'static [u8; 17] =
                    &L_OBJC_METH_VAR_NAME_;
                loop { }
            }
            loop { }
        }
        unsafe fn hash() -> ::objr::bindings::Sel {
            #[inline(never)]
            unsafe fn codegen_workaround() -> ::objr::bindings::Sel {
                #[link_section = "__TEXT,__objc_methname,cstring_literals"]
                static L_OBJC_METH_VAR_NAME_: [u8; 5] = *b"hash\0";
                #[link_section =
                  "__DATA,__objc_selrefs,literal_pointers,no_dead_strip"]
                static L_OBJC_SELECTOR_REFERENCES_: &'static [u8; 5] =
                    &L_OBJC_METH_VAR_NAME_;
                loop { }
            }
            loop { }
        }
    }
    #[allow(non_upper_case_globals)]
    const NSUTF8StringEncoding: NSUInteger = 4;
    impl PartialEq for NSString {
        fn eq(&self, other: &Self) -> bool { loop { } }
    }
    impl Eq for NSString { }
    impl Hash for NSString {
        fn hash<H: Hasher>(&self, state: &mut H) { loop { } }
    }
    impl NSString {
        pub fn to_str(&self, pool: &ActiveAutoreleasePool) -> &str {
            loop { }
        }
        pub fn with_str_copy(str: &str, pool: &ActiveAutoreleasePool)
         -> StrongCell<NSString> {
            loop { }
        }
    }
}
mod autorelease {
    use core::ffi::{c_void};
    use core::marker::PhantomData;
    use std::ops::Deref;
    extern "C" {
        pub fn objc_autoreleasePoolPush()
        -> *const c_void;
        pub fn objc_autoreleasePoolPop(ptr: *const c_void);
    }
    pub struct ActiveAutoreleasePool {
        _marker: PhantomData<*const ()>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for ActiveAutoreleasePool {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            loop { }
        }
    }
    impl ActiveAutoreleasePool {
        pub const unsafe fn assume_autoreleasepool()
         -> ActiveAutoreleasePool {
            ActiveAutoreleasePool{_marker: PhantomData,}
        }
    }
    pub struct AutoreleasePool {
        ptr: *const c_void,
        pool: ActiveAutoreleasePool,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for AutoreleasePool {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            loop { }
        }
    }
    impl Deref for AutoreleasePool {
        type Target = ActiveAutoreleasePool;
        fn deref(&self) -> &Self::Target { loop { } }
    }
    impl Drop for AutoreleasePool {
        fn drop(&mut self) { loop { } }
    }
    pub fn autoreleasepool<F: FnOnce(&ActiveAutoreleasePool) -> R, R>(f: F)
     -> R {
        loop { }
    }
    impl AutoreleasePool {
        pub unsafe fn new() -> Self { loop { } }
    }
}
mod arguments {
    use super::bindings::*;
    use std::ffi::c_void;
    use std::fmt::Debug;
    #[link(name = "objc", kind = "dylib")]
    extern "C" {
        fn objc_msgSend();
        fn objc_msgSendSuper2();
    }
    #[repr(C)]
    struct ObjcSuper {
        receiver: *mut c_void,
        class: *const AnyClass,
    }
    pub trait Arguments: Sized + Debug + crate::private::Sealed {
        unsafe fn invoke_primitive<R: Primitive>(receiver: *mut c_void,
                                                 sel: Sel,
                                                 pool: &ActiveAutoreleasePool,
                                                 args: Self)
        -> R;
        unsafe fn invoke_primitive_super<R: Primitive>(obj: *mut c_void,
                                                       sel: Sel,
                                                       _pool:
                                                           &ActiveAutoreleasePool,
                                                       class: *const AnyClass,
                                                       args: Self)
        -> R;
        unsafe fn invoke<R: ObjcInstance>(receiver: *mut c_void, sel: Sel,
                                          pool: &ActiveAutoreleasePool,
                                          args: Self)
        -> *const R;
        unsafe fn invoke_super<R: ObjcInstance>(receiver: *mut c_void,
                                                sel: Sel,
                                                pool: &ActiveAutoreleasePool,
                                                class: *const AnyClass,
                                                args: Self)
        -> *const R;
        unsafe fn invoke_error<'a,
                               R: ObjcInstance>(receiver: *mut c_void,
                                                sel: Sel,
                                                pool:
                                                    &'a ActiveAutoreleasePool,
                                                args: Self)
        -> Result<*const R, AutoreleasedCell<'a, NSError>>;
        unsafe fn invoke_error_trampoline_strong<'a,
                                                 R: ObjcInstance>(obj:
                                                                      *mut c_void,
                                                                  sel: Sel,
                                                                  _pool:
                                                                      &'a ActiveAutoreleasePool,
                                                                  args: Self)
        -> Result<*const R, AutoreleasedCell<'a, NSError>>;
        unsafe fn invoke_error_trampoline_strong_super<'a,
                                                       R: ObjcInstance>(obj:
                                                                            *mut c_void,
                                                                        sel:
                                                                            Sel,
                                                                        _pool:
                                                                            &'a ActiveAutoreleasePool,
                                                                        class:
                                                                            *const AnyClass,
                                                                        args:
                                                                            Self)
        -> Result<*const R, AutoreleasedCell<'a, NSError>>;
        unsafe fn invoke_error_trampoline_super<'a,
                                                R: ObjcInstance>(receiver:
                                                                     *mut c_void,
                                                                 sel: Sel,
                                                                 pool:
                                                                     &'a ActiveAutoreleasePool,
                                                                 class:
                                                                     *const AnyClass,
                                                                 args: Self)
        -> Result<*const R, AutoreleasedCell<'a, NSError>>;
    }
    pub unsafe trait Arguable { }
    unsafe impl <O: ObjcInstance> Arguable for &O { }
    unsafe impl <O: ObjcInstance> Arguable for *const O { }
    pub unsafe trait Primitive: Arguable { }
    unsafe impl Primitive for Sel { }
    unsafe impl Arguable for Sel { }
    unsafe impl Primitive for bool { }
    unsafe impl Arguable for bool { }
    unsafe impl Primitive for *mut c_void { }
    unsafe impl Arguable for *mut c_void { }
    unsafe impl Primitive for *const c_void { }
    unsafe impl Arguable for *const c_void { }
    unsafe impl Primitive for f64 { }
    unsafe impl Arguable for f64 { }
    unsafe impl Primitive for () { }
    unsafe impl Arguable for () { }
    unsafe impl Primitive for u64 { }
    unsafe impl Arguable for u64 { }
    unsafe impl Primitive for u32 { }
    unsafe impl Arguable for u32 { }
    unsafe impl Primitive for u16 { }
    unsafe impl Arguable for u16 { }
    unsafe impl Primitive for u8 { }
    unsafe impl Arguable for u8 { }
    unsafe impl Primitive for *const u8 { }
    unsafe impl Arguable for *const u8 { }
    unsafe impl Primitive for *mut u8 { }
    unsafe impl Arguable for *mut u8 { }
    unsafe impl Primitive for *const i8 { }
    unsafe impl Arguable for *const i8 { }
    unsafe impl Primitive for *mut i8 { }
    unsafe impl Arguable for *mut i8 { }
    unsafe impl Arguable for i64 { }
    unsafe impl Primitive for i64 { }
    unsafe impl Arguable for i32 { }
    unsafe impl Primitive for i32 { }
    unsafe impl Arguable for i16 { }
    unsafe impl Primitive for i16 { }
    unsafe impl Arguable for i8 { }
    unsafe impl Primitive for i8 { }
    macro_rules! arguments_impl {
        ($($identifier : ident : $type : ident), *) =>
        (impl < $($type : Arguable), * > crate :: objr :: private :: Sealed
         for($($type,) *) where $($type : Debug), * { } impl <
         $($type : Arguable), * > Arguments for($($type,) *) where
         $($type : Debug), *
         {
             #[inline] unsafe fn invoke_primitive < R : Primitive >
             (obj : * mut c_void, sel : Sel, _pool : & ActiveAutoreleasePool,
              ($($identifier,) *) : Self) -> R
             {
                 let impcast = objc_msgSend as unsafe extern fn() ; let imp :
                 unsafe extern fn(* mut c_void, Sel $(, $type) *) -> R = std
                 :: mem :: transmute(impcast) ;
                 imp(obj, sel $(, $identifier) *)
             } #[inline] unsafe fn invoke_primitive_super < R : Primitive >
             (obj : * mut c_void, sel : Sel, _pool : & ActiveAutoreleasePool,
              class : * const AnyClass, ($($identifier,) *) : Self) -> R
             {
                 let objc_super = ObjcSuper { receiver : obj, class : class }
                 ; let impcast = objc_msgSendSuper2 as unsafe extern fn() ;
                 let imp : unsafe extern
                 fn(* const ObjcSuper, Sel $(, $type) *) -> R = std :: mem ::
                 transmute(impcast) ;
                 imp(& objc_super, sel $(, $identifier) *)
             } #[inline] unsafe fn invoke < R : ObjcInstance >
             (obj : * mut c_void, sel : Sel, _pool : & ActiveAutoreleasePool,
              ($($identifier,) *) : Self) -> * const R
             {
                 let impcast = objc_msgSend as unsafe extern fn() ; let imp :
                 unsafe extern fn(* mut c_void, Sel $(, $type) *) -> * mut
                 c_void = std :: mem :: transmute(impcast) ; let ptr =
                 imp(obj, sel $(, $identifier) *) ; ptr as * const R
             } #[inline] unsafe fn invoke_super < R : ObjcInstance >
             (obj : * mut c_void, sel : Sel, _pool : & ActiveAutoreleasePool,
              class : * const AnyClass, ($($identifier,) *) : Self) -> * const
             R
             {
                 let objc_super = ObjcSuper { receiver : obj, class : class }
                 ; let impcast = objc_msgSendSuper2 as unsafe extern fn() ;
                 let imp : unsafe extern "C"
                 fn(* const ObjcSuper, Sel $(, $type) *) -> * mut c_void = std
                 :: mem :: transmute(impcast) ; let ptr =
                 imp(& objc_super, sel $(, $identifier) *) ; ptr as * const R
             }
             #[inline] unsafe fn invoke_error_trampoline_strong < 'a, R :
             ObjcInstance >
             (obj : * mut c_void, sel : Sel, pool : & 'a
              ActiveAutoreleasePool, ($($identifier,) *) : Self) -> Result < *
             const R, AutoreleasedCell < 'a, NSError >>
             {
                 use crate :: performselector ::
                 objc_retainAutoreleasedReturnValue ; let impcast =
                 objc_msgSend as unsafe extern fn() ; let mut error : * const
                 NSError = std :: ptr :: null() ; let imp : unsafe extern
                 fn(* mut c_void, Sel, $($type,) * & mut * const NSError) -> *
                 const R = std :: mem :: transmute(impcast) ; let ptr =
                 imp(obj, sel, $($identifier,) * & mut error) ;
                 objc_retainAutoreleasedReturnValue(ptr as * const c_void) ;
                 if ptr != std :: ptr :: null_mut() { Ok(ptr) } else
                 {
                     Err(NSError ::
                         assume_nonnil(error).assume_autoreleased(pool))
                 }
             } #[inline] unsafe fn invoke_error < 'a, R : ObjcInstance >
             (receiver : * mut c_void, sel : Sel, pool : & 'a
              ActiveAutoreleasePool, ($($identifier,) *) : Self) -> Result < *
             const R, AutoreleasedCell < 'a, NSError >>
             {
                 let impcast = objc_msgSend as unsafe extern fn() ; let mut
                 error : * const NSError = std :: ptr :: null() ; let imp :
                 unsafe extern
                 fn(* mut c_void, Sel, $($type,) * & mut * const NSError) -> *
                 const R = std :: mem :: transmute(impcast) ; let ptr =
                 imp(receiver, sel, $($identifier,) * & mut error) ; if ptr !=
                 std :: ptr :: null_mut() { Ok(ptr) } else
                 {
                     Err(NSError ::
                         assume_nonnil(error).assume_autoreleased(pool))
                 }
             } #[inline] unsafe fn invoke_error_trampoline_strong_super < 'a,
             R : ObjcInstance >
             (obj : * mut c_void, sel : Sel, pool : & 'a
              ActiveAutoreleasePool, class : * const AnyClass,
              ($($identifier,) *) : Self) -> Result < * const R,
             AutoreleasedCell < 'a, NSError >>
             {
                 let objc_super = ObjcSuper { receiver : obj, class : class }
                 ; use crate :: performselector ::
                 objc_retainAutoreleasedReturnValue ; let impcast =
                 objc_msgSendSuper2 as unsafe extern fn() ; let mut error : *
                 const NSError = std :: ptr :: null() ; let imp : unsafe
                 extern
                 fn(* const ObjcSuper, Sel, $($type,) * & mut * const NSError)
                 -> * const R = std :: mem :: transmute(impcast) ; let ptr =
                 imp(& objc_super, sel, $($identifier,) * & mut error) ;
                 objc_retainAutoreleasedReturnValue(ptr as * const c_void) ;
                 if ptr != std :: ptr :: null_mut() { Ok(ptr) } else
                 {
                     Err(NSError ::
                         assume_nonnil(error).assume_autoreleased(pool))
                 }
             } #[inline] unsafe fn invoke_error_trampoline_super < 'a, R :
             ObjcInstance >
             (receiver : * mut c_void, sel : Sel, pool : & 'a
              ActiveAutoreleasePool, class : * const AnyClass,
              ($($identifier,) *) : Self) -> Result < * const R,
             AutoreleasedCell < 'a, NSError >>
             {
                 let objc_super = ObjcSuper
                 { receiver : receiver, class : class } ; let impcast =
                 objc_msgSendSuper2 as unsafe extern fn() ; let mut error : *
                 const NSError = std :: ptr :: null() ; let imp : unsafe
                 extern
                 fn(* const ObjcSuper, Sel, $($type,) * & mut * const NSError)
                 -> * const R = std :: mem :: transmute(impcast) ; let ptr =
                 imp(& objc_super, sel, $($identifier,) * & mut error) ; if
                 ptr != std :: ptr :: null_mut() { Ok(ptr) } else
                 {
                     Err(NSError ::
                         assume_nonnil(error).assume_autoreleased(pool))
                 }
             }
         }) ;
    }
    impl crate::objr::private::Sealed for () where  { }
    impl Arguments for () where  {
        #[inline]
        unsafe fn invoke_primitive<R: Primitive>(obj: *mut c_void, sel: Sel,
                                                 _pool:
                                                     &ActiveAutoreleasePool,
                                                 (): Self) -> R {
            loop { }
        }
        #[inline]
        unsafe fn invoke_primitive_super<R: Primitive>(obj: *mut c_void,
                                                       sel: Sel,
                                                       _pool:
                                                           &ActiveAutoreleasePool,
                                                       class: *const AnyClass,
                                                       (): Self) -> R {
            loop { }
        }
        #[inline]
        unsafe fn invoke<R: ObjcInstance>(obj: *mut c_void, sel: Sel,
                                          _pool: &ActiveAutoreleasePool,
                                          (): Self) -> *const R {
            loop { }
        }
        #[inline]
        unsafe fn invoke_super<R: ObjcInstance>(obj: *mut c_void, sel: Sel,
                                                _pool: &ActiveAutoreleasePool,
                                                class: *const AnyClass,
                                                (): Self) -> *const R {
            loop { }
        }
        #[inline]
        unsafe fn invoke_error_trampoline_strong<'a,
                                                 R: ObjcInstance>(obj:
                                                                      *mut c_void,
                                                                  sel: Sel,
                                                                  pool:
                                                                      &'a ActiveAutoreleasePool,
                                                                  (): Self)
         -> Result<*const R, AutoreleasedCell<'a, NSError>> {
            loop { }
        }
        #[inline]
        unsafe fn invoke_error<'a,
                               R: ObjcInstance>(receiver: *mut c_void,
                                                sel: Sel,
                                                pool:
                                                    &'a ActiveAutoreleasePool,
                                                (): Self)
         -> Result<*const R, AutoreleasedCell<'a, NSError>> {
            loop { }
        }
        #[inline]
        unsafe fn invoke_error_trampoline_strong_super<'a,
                                                       R: ObjcInstance>(obj:
                                                                            *mut c_void,
                                                                        sel:
                                                                            Sel,
                                                                        pool:
                                                                            &'a ActiveAutoreleasePool,
                                                                        class:
                                                                            *const AnyClass,
                                                                        ():
                                                                            Self)
         -> Result<*const R, AutoreleasedCell<'a, NSError>> {
            loop { }
        }
        #[inline]
        unsafe fn invoke_error_trampoline_super<'a,
                                                R: ObjcInstance>(receiver:
                                                                     *mut c_void,
                                                                 sel: Sel,
                                                                 pool:
                                                                     &'a ActiveAutoreleasePool,
                                                                 class:
                                                                     *const AnyClass,
                                                                 (): Self)
         -> Result<*const R, AutoreleasedCell<'a, NSError>> {
            loop { }
        }
    }
    impl <A: Arguable> crate::objr::private::Sealed for (A,) where A: Debug {
    }
    impl <A: Arguable> Arguments for (A,) where A: Debug {
        #[inline]
        unsafe fn invoke_primitive<R: Primitive>(obj: *mut c_void, sel: Sel,
                                                 _pool:
                                                     &ActiveAutoreleasePool,
                                                 (a,): Self) -> R {
            loop { }
        }
        #[inline]
        unsafe fn invoke_primitive_super<R: Primitive>(obj: *mut c_void,
                                                       sel: Sel,
                                                       _pool:
                                                           &ActiveAutoreleasePool,
                                                       class: *const AnyClass,
                                                       (a,): Self) -> R {
            loop { }
        }
        #[inline]
        unsafe fn invoke<R: ObjcInstance>(obj: *mut c_void, sel: Sel,
                                          _pool: &ActiveAutoreleasePool,
                                          (a,): Self) -> *const R {
            loop { }
        }
        #[inline]
        unsafe fn invoke_super<R: ObjcInstance>(obj: *mut c_void, sel: Sel,
                                                _pool: &ActiveAutoreleasePool,
                                                class: *const AnyClass,
                                                (a,): Self) -> *const R {
            loop { }
        }
        unsafe fn invoke_error_trampoline_strong<'a,
                                                 R: ObjcInstance>(obj:
                                                                      *mut c_void,
                                                                  sel: Sel,
                                                                  pool:
                                                                      &'a ActiveAutoreleasePool,
                                                                  (a,): Self)
         -> Result<*const R, AutoreleasedCell<'a, NSError>> {
            loop { }
        }
        #[inline]
        unsafe fn invoke_error<'a,
                               R: ObjcInstance>(receiver: *mut c_void,
                                                sel: Sel,
                                                pool:
                                                    &'a ActiveAutoreleasePool,
                                                (a,): Self)
         -> Result<*const R, AutoreleasedCell<'a, NSError>> {
            loop { }
        }
        #[inline]
        unsafe fn invoke_error_trampoline_strong_super<'a,
                                                       R: ObjcInstance>(obj:
                                                                            *mut c_void,
                                                                        sel:
                                                                            Sel,
                                                                        pool:
                                                                            &'a ActiveAutoreleasePool,
                                                                        class:
                                                                            *const AnyClass,
                                                                        (a,):
                                                                            Self)
         -> Result<*const R, AutoreleasedCell<'a, NSError>> {
            loop { }
        }
        #[inline]
        unsafe fn invoke_error_trampoline_super<'a,
                                                R: ObjcInstance>(receiver:
                                                                     *mut c_void,
                                                                 sel: Sel,
                                                                 pool:
                                                                     &'a ActiveAutoreleasePool,
                                                                 class:
                                                                     *const AnyClass,
                                                                 (a,): Self)
         -> Result<*const R, AutoreleasedCell<'a, NSError>> {
            loop { }
        }
    }
    impl <A: Arguable, B: Arguable> crate::objr::private::Sealed for (A, B)
     where A: Debug, B: Debug {
    }
    impl <A: Arguable, B: Arguable> Arguments for (A, B) where A: Debug,
     B: Debug {
        #[inline]
        unsafe fn invoke_primitive<R: Primitive>(obj: *mut c_void, sel: Sel,
                                                 _pool:
                                                     &ActiveAutoreleasePool,
                                                 (a, b): Self) -> R {
            loop { }
        }
        #[inline]
        unsafe fn invoke_primitive_super<R: Primitive>(obj: *mut c_void,
                                                       sel: Sel,
                                                       _pool:
                                                           &ActiveAutoreleasePool,
                                                       class: *const AnyClass,
                                                       (a, b): Self) -> R {
            loop { }
        }
        #[inline]
        unsafe fn invoke<R: ObjcInstance>(obj: *mut c_void, sel: Sel,
                                          _pool: &ActiveAutoreleasePool,
                                          (a, b): Self) -> *const R {
            loop { }
        }
        #[inline]
        unsafe fn invoke_super<R: ObjcInstance>(obj: *mut c_void, sel: Sel,
                                                _pool: &ActiveAutoreleasePool,
                                                class: *const AnyClass,
                                                (a, b): Self) -> *const R {
            loop { }
        }
        unsafe fn invoke_error_trampoline_strong<'a,
                                                 R: ObjcInstance>(obj:
                                                                      *mut c_void,
                                                                  sel: Sel,
                                                                  pool:
                                                                      &'a ActiveAutoreleasePool,
                                                                  (a, b):
                                                                      Self)
         -> Result<*const R, AutoreleasedCell<'a, NSError>> {
            loop { }
        }
        #[inline]
        unsafe fn invoke_error<'a,
                               R: ObjcInstance>(receiver: *mut c_void,
                                                sel: Sel,
                                                pool:
                                                    &'a ActiveAutoreleasePool,
                                                (a, b): Self)
         -> Result<*const R, AutoreleasedCell<'a, NSError>> {
            loop { }
        }
        #[inline]
        unsafe fn invoke_error_trampoline_strong_super<'a,
                                                       R: ObjcInstance>(obj:
                                                                            *mut c_void,
                                                                        sel:
                                                                            Sel,
                                                                        pool:
                                                                            &'a ActiveAutoreleasePool,
                                                                        class:
                                                                            *const AnyClass,
                                                                        (a,
                                                                         b):
                                                                            Self)
         -> Result<*const R, AutoreleasedCell<'a, NSError>> {
            loop { }
        }
        #[inline]
        unsafe fn invoke_error_trampoline_super<'a,
                                                R: ObjcInstance>(receiver:
                                                                     *mut c_void,
                                                                 sel: Sel,
                                                                 pool:
                                                                     &'a ActiveAutoreleasePool,
                                                                 class:
                                                                     *const AnyClass,
                                                                 (a, b): Self)
         -> Result<*const R, AutoreleasedCell<'a, NSError>> {
            loop { }
        }
    }
    impl <A: Arguable, B: Arguable, C: Arguable> crate::objr::private::Sealed
     for (A, B, C) where A: Debug, B: Debug, C: Debug {
    }
    impl <A: Arguable, B: Arguable, C: Arguable> Arguments for (A, B, C) where
     A: Debug, B: Debug, C: Debug {
        #[inline]
        unsafe fn invoke_primitive<R: Primitive>(obj: *mut c_void, sel: Sel,
                                                 _pool:
                                                     &ActiveAutoreleasePool,
                                                 (a, b, c): Self) -> R {
            loop { }
        }
        #[inline]
        unsafe fn invoke_primitive_super<R: Primitive>(obj: *mut c_void,
                                                       sel: Sel,
                                                       _pool:
                                                           &ActiveAutoreleasePool,
                                                       class: *const AnyClass,
                                                       (a, b, c): Self) -> R {
            loop { }
        }
        #[inline]
        unsafe fn invoke<R: ObjcInstance>(obj: *mut c_void, sel: Sel,
                                          _pool: &ActiveAutoreleasePool,
                                          (a, b, c): Self) -> *const R {
            loop { }
        }
        #[inline]
        unsafe fn invoke_super<R: ObjcInstance>(obj: *mut c_void, sel: Sel,
                                                _pool: &ActiveAutoreleasePool,
                                                class: *const AnyClass,
                                                (a, b, c): Self) -> *const R {
            loop { }
        }
        #[inline]
        unsafe fn invoke_error_trampoline_strong<'a,
                                                 R: ObjcInstance>(obj:
                                                                      *mut c_void,
                                                                  sel: Sel,
                                                                  pool:
                                                                      &'a ActiveAutoreleasePool,
                                                                  (a, b, c):
                                                                      Self)
         -> Result<*const R, AutoreleasedCell<'a, NSError>> {
            loop { }
        }
        #[inline]
        unsafe fn invoke_error<'a,
                               R: ObjcInstance>(receiver: *mut c_void,
                                                sel: Sel,
                                                pool:
                                                    &'a ActiveAutoreleasePool,
                                                (a, b, c): Self)
         -> Result<*const R, AutoreleasedCell<'a, NSError>> {
            loop { }
        }
        #[inline]
        unsafe fn invoke_error_trampoline_strong_super<'a,
                                                       R: ObjcInstance>(obj:
                                                                            *mut c_void,
                                                                        sel:
                                                                            Sel,
                                                                        pool:
                                                                            &'a ActiveAutoreleasePool,
                                                                        class:
                                                                            *const AnyClass,
                                                                        (a, b,
                                                                         c):
                                                                            Self)
         -> Result<*const R, AutoreleasedCell<'a, NSError>> {
            loop { }
        }
        #[inline]
        unsafe fn invoke_error_trampoline_super<'a,
                                                R: ObjcInstance>(receiver:
                                                                     *mut c_void,
                                                                 sel: Sel,
                                                                 pool:
                                                                     &'a ActiveAutoreleasePool,
                                                                 class:
                                                                     *const AnyClass,
                                                                 (a, b, c):
                                                                     Self)
         -> Result<*const R, AutoreleasedCell<'a, NSError>> {
            loop { }
        }
    }
    impl <A: Arguable, B: Arguable, C: Arguable, D: Arguable>
     crate::objr::private::Sealed for (A, B, C, D) where A: Debug, B: Debug,
     C: Debug, D: Debug {
    }
    impl <A: Arguable, B: Arguable, C: Arguable, D: Arguable> Arguments for
     (A, B, C, D) where A: Debug, B: Debug, C: Debug, D: Debug {
        #[inline]
        unsafe fn invoke_primitive<R: Primitive>(obj: *mut c_void, sel: Sel,
                                                 _pool:
                                                     &ActiveAutoreleasePool,
                                                 (a, b, c, d): Self) -> R {
            loop { }
        }
        #[inline]
        unsafe fn invoke_primitive_super<R: Primitive>(obj: *mut c_void,
                                                       sel: Sel,
                                                       _pool:
                                                           &ActiveAutoreleasePool,
                                                       class: *const AnyClass,
                                                       (a, b, c, d): Self)
         -> R {
            loop { }
        }
        #[inline]
        unsafe fn invoke<R: ObjcInstance>(obj: *mut c_void, sel: Sel,
                                          _pool: &ActiveAutoreleasePool,
                                          (a, b, c, d): Self) -> *const R {
            loop { }
        }
        #[inline]
        unsafe fn invoke_super<R: ObjcInstance>(obj: *mut c_void, sel: Sel,
                                                _pool: &ActiveAutoreleasePool,
                                                class: *const AnyClass,
                                                (a, b, c, d): Self)
         -> *const R {
            loop { }
        }
        unsafe fn invoke_error_trampoline_strong<'a,
                                                 R: ObjcInstance>(obj:
                                                                      *mut c_void,
                                                                  sel: Sel,
                                                                  pool:
                                                                      &'a ActiveAutoreleasePool,
                                                                  (a, b, c,
                                                                   d): Self)
         -> Result<*const R, AutoreleasedCell<'a, NSError>> {
            loop { }
        }
        #[inline]
        unsafe fn invoke_error<'a,
                               R: ObjcInstance>(receiver: *mut c_void,
                                                sel: Sel,
                                                pool:
                                                    &'a ActiveAutoreleasePool,
                                                (a, b, c, d): Self)
         -> Result<*const R, AutoreleasedCell<'a, NSError>> {
            loop { }
        }
        #[inline]
        unsafe fn invoke_error_trampoline_strong_super<'a,
                                                       R: ObjcInstance>(obj:
                                                                            *mut c_void,
                                                                        sel:
                                                                            Sel,
                                                                        pool:
                                                                            &'a ActiveAutoreleasePool,
                                                                        class:
                                                                            *const AnyClass,
                                                                        (a, b,
                                                                         c,
                                                                         d):
                                                                            Self)
         -> Result<*const R, AutoreleasedCell<'a, NSError>> {
            loop { }
        }
        #[inline]
        unsafe fn invoke_error_trampoline_super<'a,
                                                R: ObjcInstance>(receiver:
                                                                     *mut c_void,
                                                                 sel: Sel,
                                                                 pool:
                                                                     &'a ActiveAutoreleasePool,
                                                                 class:
                                                                     *const AnyClass,
                                                                 (a, b, c, d):
                                                                     Self)
         -> Result<*const R, AutoreleasedCell<'a, NSError>> {
            loop { }
        }
    }
    impl <A: Arguable, B: Arguable, C: Arguable, D: Arguable, E: Arguable>
     crate::objr::private::Sealed for (A, B, C, D, E) where A: Debug,
     B: Debug, C: Debug, D: Debug, E: Debug {
    }
    impl <A: Arguable, B: Arguable, C: Arguable, D: Arguable, E: Arguable>
     Arguments for (A, B, C, D, E) where A: Debug, B: Debug, C: Debug,
     D: Debug, E: Debug {
        #[inline]
        unsafe fn invoke_primitive<R: Primitive>(obj: *mut c_void, sel: Sel,
                                                 _pool:
                                                     &ActiveAutoreleasePool,
                                                 (a, b, c, d, e): Self) -> R {
            loop { }
        }
        #[inline]
        unsafe fn invoke_primitive_super<R: Primitive>(obj: *mut c_void,
                                                       sel: Sel,
                                                       _pool:
                                                           &ActiveAutoreleasePool,
                                                       class: *const AnyClass,
                                                       (a, b, c, d, e): Self)
         -> R {
            loop { }
        }
        #[inline]
        unsafe fn invoke<R: ObjcInstance>(obj: *mut c_void, sel: Sel,
                                          _pool: &ActiveAutoreleasePool,
                                          (a, b, c, d, e): Self) -> *const R {
            loop { }
        }
        #[inline]
        unsafe fn invoke_super<R: ObjcInstance>(obj: *mut c_void, sel: Sel,
                                                _pool: &ActiveAutoreleasePool,
                                                class: *const AnyClass,
                                                (a, b, c, d, e): Self)
         -> *const R {
            loop { }
        }
        #[inline]
        unsafe fn invoke_error_trampoline_strong<'a,
                                                 R: ObjcInstance>(obj:
                                                                      *mut c_void,
                                                                  sel: Sel,
                                                                  pool:
                                                                      &'a ActiveAutoreleasePool,
                                                                  (a, b, c, d,
                                                                   e): Self)
         -> Result<*const R, AutoreleasedCell<'a, NSError>> {
            loop { }
        }
        #[inline]
        unsafe fn invoke_error<'a,
                               R: ObjcInstance>(receiver: *mut c_void,
                                                sel: Sel,
                                                pool:
                                                    &'a ActiveAutoreleasePool,
                                                (a, b, c, d, e): Self)
         -> Result<*const R, AutoreleasedCell<'a, NSError>> {
            loop { }
        }
        #[inline]
        unsafe fn invoke_error_trampoline_strong_super<'a,
                                                       R: ObjcInstance>(obj:
                                                                            *mut c_void,
                                                                        sel:
                                                                            Sel,
                                                                        pool:
                                                                            &'a ActiveAutoreleasePool,
                                                                        class:
                                                                            *const AnyClass,
                                                                        (a, b,
                                                                         c, d,
                                                                         e):
                                                                            Self)
         -> Result<*const R, AutoreleasedCell<'a, NSError>> {
            loop { }
        }
        #[inline]
        unsafe fn invoke_error_trampoline_super<'a,
                                                R: ObjcInstance>(receiver:
                                                                     *mut c_void,
                                                                 sel: Sel,
                                                                 pool:
                                                                     &'a ActiveAutoreleasePool,
                                                                 class:
                                                                     *const AnyClass,
                                                                 (a, b, c, d,
                                                                  e): Self)
         -> Result<*const R, AutoreleasedCell<'a, NSError>> {
            loop { }
        }
    }
    impl <A: Arguable, B: Arguable, C: Arguable, D: Arguable, E: Arguable,
          F: Arguable> crate::objr::private::Sealed for (A, B, C, D, E, F)
     where A: Debug, B: Debug, C: Debug, D: Debug, E: Debug, F: Debug {
    }
    impl <A: Arguable, B: Arguable, C: Arguable, D: Arguable, E: Arguable,
          F: Arguable> Arguments for (A, B, C, D, E, F) where A: Debug,
     B: Debug, C: Debug, D: Debug, E: Debug, F: Debug {
        #[inline]
        unsafe fn invoke_primitive<R: Primitive>(obj: *mut c_void, sel: Sel,
                                                 _pool:
                                                     &ActiveAutoreleasePool,
                                                 (a, b, c, d, e, f): Self)
         -> R {
            loop { }
        }
        #[inline]
        unsafe fn invoke_primitive_super<R: Primitive>(obj: *mut c_void,
                                                       sel: Sel,
                                                       _pool:
                                                           &ActiveAutoreleasePool,
                                                       class: *const AnyClass,
                                                       (a, b, c, d, e, f):
                                                           Self) -> R {
            loop { }
        }
        #[inline]
        unsafe fn invoke<R: ObjcInstance>(obj: *mut c_void, sel: Sel,
                                          _pool: &ActiveAutoreleasePool,
                                          (a, b, c, d, e, f): Self)
         -> *const R {
            loop { }
        }
        #[inline]
        unsafe fn invoke_super<R: ObjcInstance>(obj: *mut c_void, sel: Sel,
                                                _pool: &ActiveAutoreleasePool,
                                                class: *const AnyClass,
                                                (a, b, c, d, e, f): Self)
         -> *const R {
            loop { }
        }
        unsafe fn invoke_error_trampoline_strong<'a,
                                                 R: ObjcInstance>(obj:
                                                                      *mut c_void,
                                                                  sel: Sel,
                                                                  pool:
                                                                      &'a ActiveAutoreleasePool,
                                                                  (a, b, c, d,
                                                                   e, f):
                                                                      Self)
         -> Result<*const R, AutoreleasedCell<'a, NSError>> {
            loop { }
        }
        #[inline]
        unsafe fn invoke_error<'a,
                               R: ObjcInstance>(receiver: *mut c_void,
                                                sel: Sel,
                                                pool:
                                                    &'a ActiveAutoreleasePool,
                                                (a, b, c, d, e, f): Self)
         -> Result<*const R, AutoreleasedCell<'a, NSError>> {
            loop { }
        }
        #[inline]
        unsafe fn invoke_error_trampoline_strong_super<'a,
                                                       R: ObjcInstance>(obj:
                                                                            *mut c_void,
                                                                        sel:
                                                                            Sel,
                                                                        pool:
                                                                            &'a ActiveAutoreleasePool,
                                                                        class:
                                                                            *const AnyClass,
                                                                        (a, b,
                                                                         c, d,
                                                                         e,
                                                                         f):
                                                                            Self)
         -> Result<*const R, AutoreleasedCell<'a, NSError>> {
            loop { }
        }
        #[inline]
        unsafe fn invoke_error_trampoline_super<'a,
                                                R: ObjcInstance>(receiver:
                                                                     *mut c_void,
                                                                 sel: Sel,
                                                                 pool:
                                                                     &'a ActiveAutoreleasePool,
                                                                 class:
                                                                     *const AnyClass,
                                                                 (a, b, c, d,
                                                                  e, f): Self)
         -> Result<*const R, AutoreleasedCell<'a, NSError>> {
            loop { }
        }
    }
    impl <A: Arguable, B: Arguable, C: Arguable, D: Arguable, E: Arguable,
          F: Arguable, G: Arguable> crate::objr::private::Sealed for
     (A, B, C, D, E, F, G) where A: Debug, B: Debug, C: Debug, D: Debug,
     E: Debug, F: Debug, G: Debug {
    }
    impl <A: Arguable, B: Arguable, C: Arguable, D: Arguable, E: Arguable,
          F: Arguable, G: Arguable> Arguments for (A, B, C, D, E, F, G) where
     A: Debug, B: Debug, C: Debug, D: Debug, E: Debug, F: Debug, G: Debug {
        #[inline]
        unsafe fn invoke_primitive<R: Primitive>(obj: *mut c_void, sel: Sel,
                                                 _pool:
                                                     &ActiveAutoreleasePool,
                                                 (a, b, c, d, e, f, g): Self)
         -> R {
            loop { }
        }
        #[inline]
        unsafe fn invoke_primitive_super<R: Primitive>(obj: *mut c_void,
                                                       sel: Sel,
                                                       _pool:
                                                           &ActiveAutoreleasePool,
                                                       class: *const AnyClass,
                                                       (a, b, c, d, e, f, g):
                                                           Self) -> R {
            loop { }
        }
        #[inline]
        unsafe fn invoke<R: ObjcInstance>(obj: *mut c_void, sel: Sel,
                                          _pool: &ActiveAutoreleasePool,
                                          (a, b, c, d, e, f, g): Self)
         -> *const R {
            loop { }
        }
        #[inline]
        unsafe fn invoke_super<R: ObjcInstance>(obj: *mut c_void, sel: Sel,
                                                _pool: &ActiveAutoreleasePool,
                                                class: *const AnyClass,
                                                (a, b, c, d, e, f, g): Self)
         -> *const R {
            loop { }
        }
        unsafe fn invoke_error_trampoline_strong<'a,
                                                 R: ObjcInstance>(obj:
                                                                      *mut c_void,
                                                                  sel: Sel,
                                                                  pool:
                                                                      &'a ActiveAutoreleasePool,
                                                                  (a, b, c, d,
                                                                   e, f, g):
                                                                      Self)
         -> Result<*const R, AutoreleasedCell<'a, NSError>> {
            loop { }
        }
        #[inline]
        unsafe fn invoke_error<'a,
                               R: ObjcInstance>(receiver: *mut c_void,
                                                sel: Sel,
                                                pool:
                                                    &'a ActiveAutoreleasePool,
                                                (a, b, c, d, e, f, g): Self)
         -> Result<*const R, AutoreleasedCell<'a, NSError>> {
            loop { }
        }
        #[inline]
        unsafe fn invoke_error_trampoline_strong_super<'a,
                                                       R: ObjcInstance>(obj:
                                                                            *mut c_void,
                                                                        sel:
                                                                            Sel,
                                                                        pool:
                                                                            &'a ActiveAutoreleasePool,
                                                                        class:
                                                                            *const AnyClass,
                                                                        (a, b,
                                                                         c, d,
                                                                         e, f,
                                                                         g):
                                                                            Self)
         -> Result<*const R, AutoreleasedCell<'a, NSError>> {
            loop { }
        }
        #[inline]
        unsafe fn invoke_error_trampoline_super<'a,
                                                R: ObjcInstance>(receiver:
                                                                     *mut c_void,
                                                                 sel: Sel,
                                                                 pool:
                                                                     &'a ActiveAutoreleasePool,
                                                                 class:
                                                                     *const AnyClass,
                                                                 (a, b, c, d,
                                                                  e, f, g):
                                                                     Self)
         -> Result<*const R, AutoreleasedCell<'a, NSError>> {
            loop { }
        }
    }
    impl <A: Arguable, B: Arguable, C: Arguable, D: Arguable, E: Arguable,
          F: Arguable, G: Arguable, H: Arguable> crate::objr::private::Sealed
     for (A, B, C, D, E, F, G, H) where A: Debug, B: Debug, C: Debug,
     D: Debug, E: Debug, F: Debug, G: Debug, H: Debug {
    }
    impl <A: Arguable, B: Arguable, C: Arguable, D: Arguable, E: Arguable,
          F: Arguable, G: Arguable, H: Arguable> Arguments for
     (A, B, C, D, E, F, G, H) where A: Debug, B: Debug, C: Debug, D: Debug,
     E: Debug, F: Debug, G: Debug, H: Debug {
        #[inline]
        unsafe fn invoke_primitive<R: Primitive>(obj: *mut c_void, sel: Sel,
                                                 _pool:
                                                     &ActiveAutoreleasePool,
                                                 (a, b, c, d, e, f, g, h):
                                                     Self) -> R {
            loop { }
        }
        #[inline]
        unsafe fn invoke_primitive_super<R: Primitive>(obj: *mut c_void,
                                                       sel: Sel,
                                                       _pool:
                                                           &ActiveAutoreleasePool,
                                                       class: *const AnyClass,
                                                       (a, b, c, d, e, f, g,
                                                        h): Self) -> R {
            loop { }
        }
        #[inline]
        unsafe fn invoke<R: ObjcInstance>(obj: *mut c_void, sel: Sel,
                                          _pool: &ActiveAutoreleasePool,
                                          (a, b, c, d, e, f, g, h): Self)
         -> *const R {
            loop { }
        }
        #[inline]
        unsafe fn invoke_super<R: ObjcInstance>(obj: *mut c_void, sel: Sel,
                                                _pool: &ActiveAutoreleasePool,
                                                class: *const AnyClass,
                                                (a, b, c, d, e, f, g, h):
                                                    Self) -> *const R {
            loop { }
        }
        unsafe fn invoke_error_trampoline_strong<'a,
                                                 R: ObjcInstance>(obj:
                                                                      *mut c_void,
                                                                  sel: Sel,
                                                                  pool:
                                                                      &'a ActiveAutoreleasePool,
                                                                  (a, b, c, d,
                                                                   e, f, g,
                                                                   h): Self)
         -> Result<*const R, AutoreleasedCell<'a, NSError>> {
            loop { }
        }
        #[inline]
        unsafe fn invoke_error<'a,
                               R: ObjcInstance>(receiver: *mut c_void,
                                                sel: Sel,
                                                pool:
                                                    &'a ActiveAutoreleasePool,
                                                (a, b, c, d, e, f, g, h):
                                                    Self)
         -> Result<*const R, AutoreleasedCell<'a, NSError>> {
            loop { }
        }
        #[inline]
        unsafe fn invoke_error_trampoline_strong_super<'a,
                                                       R: ObjcInstance>(obj:
                                                                            *mut c_void,
                                                                        sel:
                                                                            Sel,
                                                                        pool:
                                                                            &'a ActiveAutoreleasePool,
                                                                        class:
                                                                            *const AnyClass,
                                                                        (a, b,
                                                                         c, d,
                                                                         e, f,
                                                                         g,
                                                                         h):
                                                                            Self)
         -> Result<*const R, AutoreleasedCell<'a, NSError>> {
            loop { }
        }
        #[inline]
        unsafe fn invoke_error_trampoline_super<'a,
                                                R: ObjcInstance>(receiver:
                                                                     *mut c_void,
                                                                 sel: Sel,
                                                                 pool:
                                                                     &'a ActiveAutoreleasePool,
                                                                 class:
                                                                     *const AnyClass,
                                                                 (a, b, c, d,
                                                                  e, f, g, h):
                                                                     Self)
         -> Result<*const R, AutoreleasedCell<'a, NSError>> {
            loop { }
        }
    }
    impl <A: Arguable, B: Arguable, C: Arguable, D: Arguable, E: Arguable,
          F: Arguable, G: Arguable, H: Arguable, I: Arguable>
     crate::objr::private::Sealed for (A, B, C, D, E, F, G, H, I) where
     A: Debug, B: Debug, C: Debug, D: Debug, E: Debug, F: Debug, G: Debug,
     H: Debug, I: Debug {
    }
    impl <A: Arguable, B: Arguable, C: Arguable, D: Arguable, E: Arguable,
          F: Arguable, G: Arguable, H: Arguable, I: Arguable> Arguments for
     (A, B, C, D, E, F, G, H, I) where A: Debug, B: Debug, C: Debug, D: Debug,
     E: Debug, F: Debug, G: Debug, H: Debug, I: Debug {
        #[inline]
        unsafe fn invoke_primitive<R: Primitive>(obj: *mut c_void, sel: Sel,
                                                 _pool:
                                                     &ActiveAutoreleasePool,
                                                 (a, b, c, d, e, f, g, h, i):
                                                     Self) -> R {
            loop { }
        }
        #[inline]
        unsafe fn invoke_primitive_super<R: Primitive>(obj: *mut c_void,
                                                       sel: Sel,
                                                       _pool:
                                                           &ActiveAutoreleasePool,
                                                       class: *const AnyClass,
                                                       (a, b, c, d, e, f, g,
                                                        h, i): Self) -> R {
            loop { }
        }
        #[inline]
        unsafe fn invoke<R: ObjcInstance>(obj: *mut c_void, sel: Sel,
                                          _pool: &ActiveAutoreleasePool,
                                          (a, b, c, d, e, f, g, h, i): Self)
         -> *const R {
            loop { }
        }
        #[inline]
        unsafe fn invoke_super<R: ObjcInstance>(obj: *mut c_void, sel: Sel,
                                                _pool: &ActiveAutoreleasePool,
                                                class: *const AnyClass,
                                                (a, b, c, d, e, f, g, h, i):
                                                    Self) -> *const R {
            loop { }
        }
        unsafe fn invoke_error_trampoline_strong<'a,
                                                 R: ObjcInstance>(obj:
                                                                      *mut c_void,
                                                                  sel: Sel,
                                                                  pool:
                                                                      &'a ActiveAutoreleasePool,
                                                                  (a, b, c, d,
                                                                   e, f, g, h,
                                                                   i): Self)
         -> Result<*const R, AutoreleasedCell<'a, NSError>> {
            loop { }
        }
        #[inline]
        unsafe fn invoke_error<'a,
                               R: ObjcInstance>(receiver: *mut c_void,
                                                sel: Sel,
                                                pool:
                                                    &'a ActiveAutoreleasePool,
                                                (a, b, c, d, e, f, g, h, i):
                                                    Self)
         -> Result<*const R, AutoreleasedCell<'a, NSError>> {
            loop { }
        }
        #[inline]
        unsafe fn invoke_error_trampoline_strong_super<'a,
                                                       R: ObjcInstance>(obj:
                                                                            *mut c_void,
                                                                        sel:
                                                                            Sel,
                                                                        pool:
                                                                            &'a ActiveAutoreleasePool,
                                                                        class:
                                                                            *const AnyClass,
                                                                        (a, b,
                                                                         c, d,
                                                                         e, f,
                                                                         g, h,
                                                                         i):
                                                                            Self)
         -> Result<*const R, AutoreleasedCell<'a, NSError>> {
            loop { }
        }
        #[inline]
        unsafe fn invoke_error_trampoline_super<'a,
                                                R: ObjcInstance>(receiver:
                                                                     *mut c_void,
                                                                 sel: Sel,
                                                                 pool:
                                                                     &'a ActiveAutoreleasePool,
                                                                 class:
                                                                     *const AnyClass,
                                                                 (a, b, c, d,
                                                                  e, f, g, h,
                                                                  i): Self)
         -> Result<*const R, AutoreleasedCell<'a, NSError>> {
            loop { }
        }
    }
}
mod performselector {
    use std::ffi::c_void;
    use super::arguments::{Arguments};
    use super::arguments::Primitive;
    use super::objectpointers::{AutoreleasedCell};
    use super::sel::Sel;
    use super::objcinstance::ObjcInstance;
    use super::autorelease::ActiveAutoreleasePool;
    use crate::bindings::{NSError, ObjcClass};
    use crate::class::AnyClass;
    pub unsafe trait PerformablePointer { }
    unsafe impl <O: ObjcInstance> PerformablePointer for O { }
    pub unsafe trait PerformableSuper: PerformablePointer {
        fn any_class()
        -> &'static AnyClass;
    }
    unsafe impl <O: ObjcClass + 'static> PerformableSuper for O {
        fn any_class() -> &'static AnyClass { loop { } }
    }
    #[link(name = "objc", kind = "dylib")]
    extern {
        pub(crate) fn objc_retainAutoreleasedReturnValue(id: *const c_void)
        -> *mut c_void;
    }
    pub trait PerformsSelector {
        unsafe fn perform_primitive<A: Arguments,
                                    R: Primitive>(receiver: *mut Self,
                                                  selector: Sel,
                                                  pool:
                                                      &ActiveAutoreleasePool,
                                                  args: A)
        -> R;
        unsafe fn perform<A: Arguments,
                          R: ObjcInstance>(receiver: *mut Self, selector: Sel,
                                           pool: &ActiveAutoreleasePool,
                                           args: A)
        -> *const R;
        unsafe fn perform_result<'a, A: Arguments,
                                 R: ObjcInstance>(receiver: *mut Self,
                                                  selector: Sel,
                                                  pool:
                                                      &'a ActiveAutoreleasePool,
                                                  args: A)
        -> Result<*const R, AutoreleasedCell<'a, NSError>>;
        unsafe fn perform_autorelease_to_retain<A: Arguments,
                                                R: ObjcInstance>(receiver:
                                                                     *mut Self,
                                                                 selector:
                                                                     Sel,
                                                                 pool:
                                                                     &ActiveAutoreleasePool,
                                                                 args: A)
        -> *const R;
        unsafe fn perform_result_autorelease_to_retain<A: Arguments,
                                                       R: ObjcInstance>(receiver:
                                                                            *mut Self,
                                                                        selector:
                                                                            Sel,
                                                                        pool:
                                                                            &ActiveAutoreleasePool,
                                                                        args:
                                                                            A)
        -> Result<*const R, AutoreleasedCell<'_, NSError>>;
    }
    unsafe fn magic_retaining_trampoline<A: Arguments,
                                         R: ObjcInstance>(ptr: *mut c_void,
                                                          selector: Sel,
                                                          pool:
                                                              &ActiveAutoreleasePool,
                                                          args: A)
     -> *const R {
        loop { }
    }
    unsafe fn magic_retaining_trampoline_super<A: Arguments,
                                               R: ObjcInstance>(ptr:
                                                                    *mut c_void,
                                                                selector: Sel,
                                                                pool:
                                                                    &ActiveAutoreleasePool,
                                                                class:
                                                                    *const AnyClass,
                                                                args: A)
     -> *const R {
        loop { }
    }
    impl <T: PerformablePointer> PerformsSelector for T {
        unsafe fn perform_primitive<A: Arguments,
                                    R: Primitive>(receiver: *mut Self,
                                                  selector: Sel,
                                                  pool:
                                                      &ActiveAutoreleasePool,
                                                  args: A) -> R {
            loop { }
        }
        unsafe fn perform<A: Arguments,
                          R: ObjcInstance>(receiver: *mut Self, selector: Sel,
                                           pool: &ActiveAutoreleasePool,
                                           args: A) -> *const R {
            loop { }
        }
        unsafe fn perform_result<'a, A: Arguments,
                                 R: ObjcInstance>(receiver: *mut Self,
                                                  selector: Sel,
                                                  pool:
                                                      &'a ActiveAutoreleasePool,
                                                  args: A)
         -> Result<*const R, AutoreleasedCell<'a, NSError>> {
            loop { }
        }
        unsafe fn perform_autorelease_to_retain<A: Arguments,
                                                R: ObjcInstance>(receiver:
                                                                     *mut Self,
                                                                 selector:
                                                                     Sel,
                                                                 pool:
                                                                     &ActiveAutoreleasePool,
                                                                 args: A)
         -> *const R {
            loop { }
        }
        unsafe fn perform_result_autorelease_to_retain<'a, A: Arguments,
                                                       R: ObjcInstance>(receiver:
                                                                            *mut Self,
                                                                        selector:
                                                                            Sel,
                                                                        pool:
                                                                            &'a ActiveAutoreleasePool,
                                                                        args:
                                                                            A)
         -> Result<*const R, AutoreleasedCell<'a, NSError>> {
            loop { }
        }
    }
    pub trait PerformsSelectorSuper {
        unsafe fn perform_super_primitive<A: Arguments,
                                          R: Primitive>(receiver: *mut Self,
                                                        selector: Sel,
                                                        pool:
                                                            &ActiveAutoreleasePool,
                                                        args: A)
        -> R;
        unsafe fn perform_super<A: Arguments,
                                R: ObjcInstance>(receiver: *mut Self,
                                                 selector: Sel,
                                                 pool: &ActiveAutoreleasePool,
                                                 args: A)
        -> *const R;
        unsafe fn perform_super_result<A: Arguments,
                                       R: ObjcInstance>(receiver: *mut Self,
                                                        selector: Sel,
                                                        pool:
                                                            &ActiveAutoreleasePool,
                                                        args: A)
        -> Result<*const R, AutoreleasedCell<'_, NSError>>;
        unsafe fn perform_super_autorelease_to_retain<A: Arguments,
                                                      R: ObjcInstance>(receiver:
                                                                           *mut Self,
                                                                       selector:
                                                                           Sel,
                                                                       pool:
                                                                           &ActiveAutoreleasePool,
                                                                       args:
                                                                           A)
        -> *const R;
        unsafe fn perform_super_result_autorelease_to_retain<A: Arguments,
                                                             R: ObjcInstance>(receiver:
                                                                                  *mut Self,
                                                                              selector:
                                                                                  Sel,
                                                                              pool:
                                                                                  &ActiveAutoreleasePool,
                                                                              args:
                                                                                  A)
        -> Result<*const R, AutoreleasedCell<'_, NSError>>;
    }
    impl <T: PerformableSuper> PerformsSelectorSuper for T {
        unsafe fn perform_super_primitive<A: Arguments,
                                          R: Primitive>(receiver: *mut Self,
                                                        selector: Sel,
                                                        pool:
                                                            &ActiveAutoreleasePool,
                                                        args: A) -> R {
            loop { }
        }
        unsafe fn perform_super<A: Arguments,
                                R: ObjcInstance>(receiver: *mut Self,
                                                 selector: Sel,
                                                 pool: &ActiveAutoreleasePool,
                                                 args: A) -> *const R {
            loop { }
        }
        unsafe fn perform_super_result<A: Arguments,
                                       R: ObjcInstance>(receiver: *mut Self,
                                                        selector: Sel,
                                                        pool:
                                                            &ActiveAutoreleasePool,
                                                        args: A)
         -> Result<*const R, AutoreleasedCell<'_, NSError>> {
            loop { }
        }
        unsafe fn perform_super_autorelease_to_retain<A: Arguments,
                                                      R: ObjcInstance>(receiver:
                                                                           *mut Self,
                                                                       selector:
                                                                           Sel,
                                                                       pool:
                                                                           &ActiveAutoreleasePool,
                                                                       args:
                                                                           A)
         -> *const R {
            loop { }
        }
        unsafe fn perform_super_result_autorelease_to_retain<A: Arguments,
                                                             R: ObjcInstance>(receiver:
                                                                                  *mut Self,
                                                                              selector:
                                                                                  Sel,
                                                                              pool:
                                                                                  &ActiveAutoreleasePool,
                                                                              args:
                                                                                  A)
         -> Result<*const R, AutoreleasedCell<'_, NSError>> {
            loop { }
        }
    }
}
mod objcinstance {
    use std::ptr::NonNull;
    use crate::bindings::{StrongCell, AutoreleasedCell, StrongLifetimeCell,
                          StrongMutCell};
    use crate::autorelease::ActiveAutoreleasePool;
    pub trait ObjcInstance { }
    #[repr(transparent)]
    pub struct NonNullImmutable<T: ?Sized>(NonNull<T>);
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl <T: ::core::fmt::Debug + ?Sized> ::core::fmt::Debug for
     NonNullImmutable<T> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            loop { }
        }
    }
    impl <T: ObjcInstance> NonNullImmutable<T> {
        pub(crate) fn from_reference(ptr: &T) -> Self { loop { } }
        pub unsafe fn assume_retained(self) -> StrongCell<T> { loop { } }
        pub unsafe fn assume_retained_limited<'a>(self)
         -> StrongLifetimeCell<'a, T> where T: 'a {
            loop { }
        }
        pub unsafe fn assume_autoreleased<'a>(self,
                                              pool: &'a ActiveAutoreleasePool)
         -> AutoreleasedCell<'a, T> {
            loop { }
        }
        pub(crate) fn as_ptr(&self) -> *const T { loop { } }
        pub(crate) unsafe fn assume_nonnil(ptr: *const T) -> Self { loop { } }
        unsafe fn as_ref(&self) -> &T { loop { } }
        pub unsafe fn retain(&self) -> StrongCell<T> { loop { } }
    }
    pub trait ObjcInstanceBehavior {
        unsafe fn cast<R: ObjcInstance>(&self)
        -> &R;
        unsafe fn cast_mut<R: ObjcInstance>(&mut self)
        -> &mut R;
        unsafe fn assume_nonnil(ptr: *const Self)
        -> NonNullImmutable<Self>;
        fn nullable(ptr: *const Self)
        -> Option<NonNullImmutable<Self>>;
        /// [objr::bindings::PerformsSelector::perform]
        unsafe fn assume_nonmut_perform(&self)
        -> *mut Self;
    }
    impl <T: ObjcInstance> ObjcInstanceBehavior for T {
        unsafe fn cast<R: ObjcInstance>(&self) -> &R { loop { } }
        unsafe fn cast_mut<R: ObjcInstance>(&mut self) -> &mut R { loop { } }
        unsafe fn assume_nonnil(ptr: *const Self) -> NonNullImmutable<Self> {
            loop { }
        }
        fn nullable(ptr: *const Self) -> Option<NonNullImmutable<Self>> {
            loop { }
        }
        unsafe fn assume_nonmut_perform(&self) -> *mut Self { loop { } }
    }
    pub trait NullableBehavior {
        type T: ObjcInstance;
        unsafe fn assume_autoreleased<'a>(self,
                                          pool: &'a ActiveAutoreleasePool)
        -> Option<AutoreleasedCell<'a, Self::T>>;
        unsafe fn assume_retained(self)
        -> Option<StrongCell<Self::T>>;
        unsafe fn retain(self)
        -> Option<StrongCell<Self::T>>;
        unsafe fn assume_retained_limited<'a>(self)
        -> Option<StrongLifetimeCell<'a, Self::T>>
        where
        Self::T: 'a;
    }
    impl <O: ObjcInstance> NullableBehavior for Option<NonNullImmutable<O>> {
        type T = O;
        unsafe fn assume_autoreleased<'a>(self,
                                          pool: &'a ActiveAutoreleasePool)
         -> Option<AutoreleasedCell<'a, O>> {
            loop { }
        }
        unsafe fn assume_retained(self) -> Option<StrongCell<Self::T>> {
            loop { }
        }
        unsafe fn retain(self) -> Option<StrongCell<Self::T>> { loop { } }
        unsafe fn assume_retained_limited<'a>(self)
         -> Option<StrongLifetimeCell<'a, Self::T>> where Self::T: 'a {
            loop { }
        }
    }
    pub trait NullableCellBehavior {
        type T: ObjcInstance;
        unsafe fn assume_mut(self)
        -> Option<StrongMutCell<Self::T>>;
    }
    impl <O: ObjcInstance> NullableCellBehavior for Option<StrongCell<O>> {
        type T = O;
        unsafe fn assume_mut(self) -> Option<StrongMutCell<Self::T>> {
            loop { }
        }
    }
    #[macro_export]
    macro_rules! objc_instance {
        ($(#[$attribute : meta]) * $pub : vis struct $objctype : ident ;) =>
        {
            :: objr :: bindings :: __mod!
            (no_construct, $objctype,
             {
                 $(#[$attribute]) * #[repr(transparent)]
                 #[derive(:: objr :: bindings :: ObjcInstance, Debug)] pub
                 struct $objctype(core :: ffi :: c_void) ;
             }) ; :: objr :: bindings :: __use!
            ($pub no_construct, $objctype, $objctype) ;
        } ;
    }
    #[macro_export]
    macro_rules! objc_instance_newtype {
        ($(#[$attribute : meta]) * $pub : vis struct $newtype : ident
         $(< $($T : ident), + >) ? : $oldtype : ident ;) =>
        {
            :: objr :: bindings :: __mod!
            (no_construct, $newtype,
             {
                 $(#[$attribute]) * #[repr(transparent)] #[derive(Debug)] pub
                 struct $newtype $(< $($T), + >) ?
                 (core :: ffi :: c_void,
                  $($(std :: marker :: PhantomData < $T >), +) ?) ;
             }) ; :: objr :: bindings :: __use!
            ($pub no_construct, $newtype, $newtype) ; impl $(< $($T), + >) ?
            ObjcInstance for $newtype $(< $($T), + >) ? { } impl < 'a,
            $($($T), *) ? > From < & 'a $newtype $(< $($T), + >) ? > for & 'a
            $oldtype
            {
                fn from(f : & 'a $newtype $(< $($T), + >) ?) -> & 'a $oldtype
                { unsafe { f.cast() } }
            } impl < 'a, $($($T), *) ? > From < & 'a mut $newtype
            $(< $($T), + >) ? > for & 'a mut $oldtype
            {
                fn from(f : & 'a mut $newtype $(< $($T), + >) ?) -> & 'a mut
                $oldtype { unsafe { f.cast_mut() } }
            }
        }
    }
    pub trait OptionalInstanceBehavior<Deref> {
        fn as_ptr(&self)
        -> *const Deref;
    }
    impl <T: ObjcInstance> OptionalInstanceBehavior<T> for Option<&T> {
        fn as_ptr(&self) -> *const T { loop { } }
    }
}
mod typealias {
    use std::os::raw::{c_ulong};
    #[cfg(target_pointer_width = "64")]
    pub(crate) type NSUInteger = c_ulong;
}
mod sel {
    use std::ffi::{c_void, CString};
    use std::os::raw::c_char;
    #[link(name = "objc", kind = "dylib")]
    extern "C" {
        fn sel_registerName(string: *const c_char)
        -> *const c_void;
    }
    #[repr(transparent)]
    pub struct Sel(*const c_void);
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for Sel { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for Sel {
        fn clone(&self) -> Sel { loop { } }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for Sel {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            loop { }
        }
    }
    impl Sel {
        pub fn from_str(string: &str) -> Self { loop { } }
        pub unsafe fn ptr(&self) -> *const c_void { loop { } }
        pub const fn from_ptr(ptr: *const c_void) -> Sel { Sel(ptr) }
    }
    pub struct _SyncWrapper<T>(pub T);
    unsafe impl <T> core::marker::Sync for _SyncWrapper<T> { }
    #[link_section = "__DATA,__objc_imageinfo,regular,no_dead_strip"]
    #[export_name = "\x01L_OBJC_IMAGE_INFO"]
    #[used]
    static IMAGE_INFO: [u32; 2] = [0, 64];
    #[macro_export]
    macro_rules! objc_selector_group {
        ($(#[$attribute : meta]) * $pub : vis trait $trait : ident
         { $(@ selector($selector : literal)) * } impl $trait2 : ident for Sel
         { }) =>
        ($pub trait $trait
         { $(objr :: bindings :: _objc_selector_decl! { $selector }) * } impl
         $trait for objr :: bindings :: Sel
         { $(objr :: bindings :: _objc_selector_impl! { $selector }) * })
    }
}
mod nserror {
    use super::bindings::*;
    mod no_constructNSError {
        #[repr(transparent)]
        pub struct NSError(core::ffi::c_void);
        impl ::objr::bindings::ObjcInstance for NSError { }
        impl std::fmt::Display for NSError {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                use ::objr::foundation::NSObjectTrait;
                loop { }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::fmt::Debug for NSError {
            fn fmt(&self, f: &mut ::core::fmt::Formatter)
             -> ::core::fmt::Result {
                loop { }
            }
        }
    }
    pub use no_constructNSError::NSError;
    impl ::objr::bindings::ObjcClass for NSError {
        fn class() -> &'static ::objr::bindings::Class<NSError> {
            #[inline(never)]
            unsafe fn merge_compilation_units()
             -> &'static ::objr::bindings::Class<NSError> {
                extern {
                    #[link_name = "\x01_OBJC_CLASS_$_NSError"]
                    static CLASS: *mut core::ffi::c_void ;
                }
                #[link_section =
                  "__DATA,__objc_classrefs,regular,no_dead_strip"]
                static CLASS_REF: &'static ::objr::bindings::Class<NSError> =
                    unsafe { std::mem::transmute(&CLASS) };
                loop { }
            }
            loop { }
        }
    }
    pub trait ResultNSError<T> {
        fn unwrap_nserror(self, pool: &ActiveAutoreleasePool)
        -> T;
    }
    impl <T> ResultNSError<T> for Result<T, AutoreleasedCell<'_, NSError>> {
        fn unwrap_nserror(self, pool: &ActiveAutoreleasePool) -> T {
            loop { }
        }
    }
    impl <T> ResultNSError<T> for Result<T, StrongCell<NSError>> {
        fn unwrap_nserror(self, pool: &ActiveAutoreleasePool) -> T {
            loop { }
        }
    }
}
mod exception {
    use std::ffi::c_void;
    extern "C" {
        fn hard_exception(call: extern "C" fn(*mut c_void),
                          context: *mut c_void);
    }
    extern "C" fn thunk_void<F: FnOnce()>(context: &mut Option<F>)
     -> *mut c_void {
        loop { }
    }
    pub fn try_unwrap_void<F: FnOnce()>(closure: F) { loop { } }
}
mod foundation {
    pub use super::nsstring::NSString;
    pub use super::nsobject::NSObject;
    pub use super::nsobject::NSObjectTrait;
    pub use super::nsobject::NSObjectSelectors;
    pub use super::class::ObjcClass;
    pub use super::nserror::NSError;
}
pub mod bindings {
    pub use super::autorelease::{ActiveAutoreleasePool, AutoreleasePool};
    pub use super::objectpointers::{StrongCell, AutoreleasedCell,
                                    StrongMutCell, AutoreleasedMutCell,
                                    StrongLifetimeCell};
    pub use super::sel::Sel;
    pub use super::nsobject::NSObjectTrait;
    pub use super::nsobject::NSObject;
    pub use super::objcinstance::{ObjcInstance, OptionalInstanceBehavior,
                                  NonNullImmutable, NullableBehavior};
    pub use super::performselector::{PerformsSelector, PerformablePointer};
    pub use super::class::{Class};
    pub use super::foundation::*;
    pub use objr::objcinstance::NullableCellBehavior;
    pub use crate::objc_instance;
    pub use crate::objc_class;
    pub use crate::objc_enum;
    pub use crate::objc_selector_group;
    pub use crate::objc_instance_newtype;
    pub use crate::objc_class_newtype;
    pub use super::class::AnyClass;
    pub use super::arguments::{Primitive, Arguable};
    pub use super::exception::{try_unwrap_void};
    pub use super::objcinstance::ObjcInstanceBehavior;
    #[doc(hidden)]
    pub use super::sel::_SyncWrapper;
}
mod private {
    pub trait Sealed { }
}
