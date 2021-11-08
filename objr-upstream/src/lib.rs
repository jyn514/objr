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
    pub struct NSObject;
    pub struct NSObjectSelectors;
    pub trait NSObjectTrait {}
}

mod nsstring {
    pub struct NSString;
}

mod autorelease {
    pub struct ActiveAutoreleasePool;
    pub struct AutoreleasePool;
}

mod arguments {
    pub trait Arguments {}
    pub trait Primitive {}
    pub struct Arguable;
}

mod performselector {
    pub unsafe trait PerformablePointer {}
    pub struct PerformsSelector;
}

pub struct objc_instance;
pub struct objc_instance_newtype;

mod objcinstance {
    pub struct NonNullImmutable<T>(T);
    pub trait ObjcInstance {}
    pub trait ObjcInstanceBehavior {}
    pub struct OptionalInstanceBehavior;
    pub struct NullableCellBehavior;
    pub struct NullableBehavior;
}

#[cfg(FALSE)]
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
