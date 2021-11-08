#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2018::*;
#[macro_use]
extern crate std;
extern crate self as objr;
pub mod macros {






    //import macros


    //used by macros









    //! Implements a variety of macros for simple objc binding declarations
    ///Helps generate bindings for an objc enum, as a struct with const members.
    ///
    /// # example
    ///
    /// ```
    ///# use objr::bindings::*;
    ///objc_enum! {
    ///     pub struct MTLPixelFormat<u32>;
    ///     impl MTLPixelFormat {
    ///         MTLPixelFormatInvalid = 0
    ///     }
    /// }
    ///```
    /// # Notes
    /// This macro requires
    /// * a struct with a single field
    /// * implementation block
    /// * value-level macros, like `API_AVAILABLE`, to be removed.  If you need to figure out a situation for old OS, do it yourself.
    ///   You can find and remove such lines with the regex `API_AVAILABLE\(.*\)`.
    /// * Certain complex comments need to be removed, although simple block comments appear to work in my testing.
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
    ///! Implementation of ObjC classes.  Classes are distinct from instances (which could be, for example, protocols).
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
    ///Untyped pointer to ObjC class.
    ///
    /// The actual class type is erased.  Any use of this type is likely unsafe.
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
    ///A trait for Rust types that map to ObjC classes.
    ///
    /// This is similar to [ObjcInstance] (and requires it) but imposes additional class requirements.
    ///
    /// In particular, this rules out the possibility it is a protocol.
    ///
    ///
    /// # Stability
    /// It is not stable API to impelment this trait directly.  Instead use the [objc_class!] macro.
    ///
    /// # Safety
    /// This is safe because the linker checks that this is a valid class
    pub trait ObjcClass: ObjcInstance + Sized {
        fn class()
        -> &'static Class<Self>;
    }
    ///Typed pointer to ObjC Class.  Analogous to `*const T`, but points to the class, not the instance.
    ///
    /// Used to call "class methods" like `[alloc]`.
    ///
    /// To create this type, it's recommended to use `Class::new()`.  For more information, see [objc_class!].
    #[repr(transparent)]
    pub struct Class<T: ObjcClass>(c_void, PhantomData<T>);
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl <T: ::core::fmt::Debug + ObjcClass> ::core::fmt::Debug for Class<T> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            loop { }
        }
    }
    ///Classes can use performSelector
    unsafe impl <T: ObjcClass> PerformablePointer for Class<T> { }
    impl <T: ObjcClass> PartialEq for Class<T> {
        fn eq(&self, other: &Self) -> bool { loop { } }
    }
    impl <T: ObjcClass> Class<T> {
        ///Dynamically creates a Class from some string by querying the ObjC runtime.  Note that in most cases, [NSObject::class()] in combination
        /// with [objc_class!] macro is a faster implementation because it uses compile-time knowledge.
        pub unsafe fn from_str(cstr: &CStr) -> &'static Self { loop { } }
        ///Converts to an anyclass
        pub fn as_anyclass(&self) -> &'static AnyClass { loop { } }
    }
    impl <T: ObjcClass> Class<T> {
        ///`[[Class alloc] init]`
        ///
        pub fn alloc_init(&self, pool: &ActiveAutoreleasePool)
         -> StrongCell<T> {
            loop { }
        }
        ///`[Class alloc]`
        ///
        /// # Safety
        /// Unsafe because the underlying memory is uninitialized after this call
        pub unsafe fn alloc(&self, pool: &ActiveAutoreleasePool) -> *mut T {
            loop { }
        }
        ///See [ObjcInstanceBehavior::assume_nonmut_perform()]
        pub unsafe fn assume_nonmut_perform(&self) -> *mut Self { loop { } }
    }
    impl <T: ObjcClass> std::fmt::Display for Class<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result { loop { } }
    }
    ///This declares an instance type which is also a class.  See [objc_instance!] for a version which is not a class.
    /// ```
    /// use objr::bindings::*;
    /// objc_class! {
    ///     //Declare a struct with this name, representing our objc class
    ///     pub struct Example {
    ///         @class(NSObject)
    ///     }
    /// }
    /// autoreleasepool(|pool| {
    ///     let instance = Example::class().alloc_init(&pool);
    ///     let class = Example::class();
    ///    });
    ///
    /// ```
    ///
    /// This version does not support generics, to declare a wrapper type (that can be generic), see [objc_class_newtype!]
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
    /**
Declares a newtype that wraps an existing [objc_class].

See also:
* [objc_class].  The oldtype must be declared with this macro.
* [objc_instance_newtype], the equivalent macro for [objc_instance].

Downcasts to the raw type will be implemented for you.  Upcasts will not, implement them yourself with [objr::bindings::ObjcInstanceBehavior::cast()] if applicable.

```no_run
use objr::bindings::*;
objc_class! {
    struct NSObject {
        @class(NSObject)
    }
}
objc_class_newtype! {
    struct NSSecondObject: NSObject;
}
let s: &NSSecondObject = todo!();
let e: &NSObject = s.into();

let s: &mut NSSecondObject = todo!();
let e: &mut NSObject = s.into();
```

unlike [objc_class!], this macro supports generic types, allowing you to wrap some other type with generics bolted on top.

At the moment, restrictions on generic arguments are not supported at the type level, but you can add them on your own impl blocks
```
use objr::bindings::*;
objc_class! {
    struct NSObject { @class(NSObject) }
}
objc_class_newtype! {
    struct SecondObject<A,B>: NSObject;
}
//further restriction
impl<A: PartialEq,B: PartialEq> SecondObject<A,B> { }
```

Although newtypes declared with this macro conform to ObjcClass, keep in mind that their newtypeness is a Rust construct,
and is not visible to ObjC:

```
use objr::bindings::*;
objc_class_newtype! {
    struct NotNSObject: NSObject;
}
fn static_assert_isclass<T: ObjcClass>(t: &T) {}

autoreleasepool(|pool| {
    //create a plain old NSObject
    let oldtype = NSObject::class().alloc_init(pool);
    //upgrade it to newtype
    let newtype: &NotNSObject = unsafe{ oldtype.cast() };
    //confirm newtype conforms to ObjcClass
    static_assert_isclass(newtype);
    //however, it isn't a distinct class.  It was NSObject the whole time!
    assert_eq!(NSObject::class().as_anyclass(),NotNSObject::class().as_anyclass())
})
```
*/
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
    ///Turning this on may help debug retain/release
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
        ///for lifetime
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
        ///Converts to [Self] by autoreleasing the reference.
        pub fn autoreleasing(cell: &T, _pool: &'a ActiveAutoreleasePool)
         -> Self {
            loop { }
        }
        ///Converts to [Self] by assuming the pointer is already autoreleased.
        ///
        /// This is the case for many objc methods, depending on convention.
        pub unsafe fn assume_autoreleased(ptr: &T,
                                          _pool: &'a ActiveAutoreleasePool)
         -> Self {
            loop { }
        }
        ///Converts to a mutable version.
        /// 
        /// # Safety
        /// You are responsible to check:
        /// * There are no other references to the type, mutable or otherwise
        /// * The type is in fact "mutable", whatever that means.  Specifically, to whatever extent `&mut` functions are forbidden
        ///   generally, you must ensure it is appropriate to call them here.
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
        ///for lifetime
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
        ///Converts to [Self] by autoreleasing the reference.
        pub fn autoreleasing(cell: &mut T, _pool: &'a ActiveAutoreleasePool)
         -> Self {
            loop { }
        }
        ///Converts to [Self] by assuming the pointer is already autoreleased.
        ///
        /// This is the case for many objc methods, depending on convention.
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
        ///Converts to [AutoreleasedCell] by calling `autorelease` on `self`.
        ///
        ///Safe, but needs to be a moving function, because the StrongCell will not be valid once we
        /// decrement its reference counter.
        pub fn autoreleasing<'a>(cell: &Self, pool: &'a ActiveAutoreleasePool)
         -> AutoreleasedCell<'a, T> {
            loop { }
        }
        ///Converts to [Self] by assuming the argument is already retained.
        ///
        /// This is usually the case for some objc methods with names like `new`, `copy`, `init`, etc.
        /// # Safety
        /// You are responsible to check:
        /// * That the type is retained
        /// * That the type is 'static, that is, it has no references to external (Rust) memory.
        ///   If this is not the case, see [StrongLifetimeCell].
        pub unsafe fn assume_retained(reference: &T) -> Self { loop { } }
        ///Converts to a mutable version.
        ///
        /// # Safety
        /// You are responsible to check:
        /// * There are no other references to the type, mutable or otherwise
        /// * The type is in fact "mutable", whatever that means.  Specifically, to whatever extent `&mut` functions are forbidden
        ///   generally, you must ensure it is appropriate to call them here.
        pub unsafe fn assume_mut(self) -> StrongMutCell<T> { loop { } }
        ///Attempts to use the "trampoline" trick to return an autoreleased value to objc.
        ///
        /// This is largely used when implementing a subclass.
        ///
        ///You must return the return value of this function, to your caller to get optimized results.
        /// Results are not guaranteed to be optimized, in part because inline assembly is not stabilized.
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
    ///Like StrongCell, but restricted to a particular lifetime.
    ///
    /// This is typically used for objects that borrow some Rust data
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
        ///Converts to [AutoreleasedCell] by calling `autorelease` on `self`.
        ///
        ///Safe, but needs to be a moving function, because the StrongCell will not be valid once we
        /// decrement its reference counter.
        pub fn autoreleasing<'b: 'a>(cell: &'a Self,
                                     pool: &'b ActiveAutoreleasePool)
         -> AutoreleasedCell<'b, T> {
            loop { }
        }
        ///Converts to [Self] by assuming the argument is already retained.
        ///
        /// This is usually the case for some objc methods with names like `new`, `copy`, `init`, etc.
        /// # Safety
        /// You are repsonsible to check:
        /// * That the type is retained
        /// * That the type can remain valid for the lifetime specified.  e.g., all "inner pointers" or "borrowed data" involved
        /// in this object will remain valid for the lifetime specified, which is unbounded.
        /// * That all objc APIs which end up seeing this pointer will either only access it for the lifetime specified,
        ///   or will take some other step (usually, copying) the object into a longer lifetime.
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
    ///[StrongCell], but mutable
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
        ///Converts to [AutoreleasedCell] by calling `autorelease` on `self`.
        ///
        ///Safe, but needs to be a moving function, because the StrongCell will not be valid once we
        /// decrement its reference counter.
        pub fn autoreleasing<'a>(cell: &mut Self,
                                 pool: &'a ActiveAutoreleasePool)
         -> AutoreleasedMutCell<'a, T> {
            loop { }
        }
        ///Converts to [StrongCell], e.g. dropping the mutable portion.
        ///
        /// This consumes the cell, e.g. you can't have an exclusive and nonexclusive reference to the same object.
        pub fn as_const(self) -> StrongCell<T> { loop { } }
    }
    impl <T: ObjcInstance> StrongMutCell<T> {
        ///Converts to [Self] by assuming the argument is already retained.
        ///
        /// This is usually the case for some objc methods with names like `new`, `copy`, `init`, etc.
        /// # Safety
        /// If this isn't actually retained, will UB
        pub unsafe fn assume_retained(reference: &mut T) -> Self { loop { } }
        ///Attempts to use the "trampoline" trick to return an autoreleased value to objc.
        ///
        /// This is largely used when implementing a subclass.
        ///
        /// You must return the return value of this function, to your caller to get optimized results.
        /// Results are not guaranteed to be optimized, in part because inline assembly is not stabilized.
        #[inline(always)]
        pub fn return_autoreleased(self) -> *mut T { loop { } }
    }
    ///we send in objc all the time
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
    //! Bindings for NSObject
    //!
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
    ///Trait for NSObject.  This will be autoimplemented by all [ObjcInstance].
    ///
    /// This type provides bindings to common `NSObject` functions.
    pub trait NSObjectTrait: Sized + ObjcInstance {
        fn description<'a>(&self, pool: &ActiveAutoreleasePool)
        -> StrongCell<NSString>;
        fn responds_to_selector(&self, pool: &ActiveAutoreleasePool, sel: Sel)
        -> bool;
        fn copy(&self, pool: &ActiveAutoreleasePool)
        -> StrongCell<Self>;
        ///Calls `[instance init]`.;
        unsafe fn init(receiver: *mut *mut Self,
                       pool: &ActiveAutoreleasePool);
        ///erases type to NSObject
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
        ///Initializes the object by calling `[self init]`
        ///
        ///By objc convention, `init` may return a distinct pointer than the one that's passed in.
        /// For this reason, a mutable reference is required.
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
    //! Provides NSString
    //!
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
        ///Converts to a stringslice
        pub fn to_str(&self, pool: &ActiveAutoreleasePool) -> &str {
            loop { }
        }
        ///Copies the string into foundation storage
        pub fn with_str_copy(str: &str, pool: &ActiveAutoreleasePool)
         -> StrongCell<NSString> {
            loop { }
        }
    }
}
mod autorelease {
    //! Autorelease pools and similar
    use core::ffi::{c_void};
    use core::marker::PhantomData;
    use std::ops::Deref;
    extern "C" {
        pub fn objc_autoreleasePoolPush()
        -> *const c_void;
        pub fn objc_autoreleasePoolPop(ptr: *const c_void);
    }
    ///Marker type that indicates you have an active autorelease pool.
    ///
    /// This type is generally appropriate for passing around as an argument.  In practice, it is zero-sized,
    /// so it should be the zero-cost abstraction.
    ///
    /// Generally, you work with borrows of this type.  The lifetime of the borrow
    /// is the lifetime that the autoreleasepool is statically guaranteed to be active.  This lets
    /// you check autorelease behavior statically.
    ///
    /// There are two ways to construct this type:
    /// 1.  by dereferencing an [AutoreleasePool] (preferred)
    ///2.   [ActiveAutoreleasePool::assume_autoreleasepool()].
    pub struct ActiveAutoreleasePool {
        ///don't allow anyone else to construct this
        /// !Send !Sync
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
        ///This function makes the [ActiveAutoreleasePool] marker type guaranteeing we have an autoreleasepool
        /// active on the thread.
        ///
        /// # Safety
        /// This is generally unsafe, but if you are certain an autoreleasepool is active on the thread,
        /// you can use this constructor to create your own marker tpe.
        pub const unsafe fn assume_autoreleasepool()
         -> ActiveAutoreleasePool {
            ActiveAutoreleasePool{_marker: PhantomData,}
        }
    }
    ///Tracks an active autoreleasepool.
    ///
    /// This is generally used at the "top level" to create a new pool, for a
    /// type to use as an argument instead, see [ActiveAutoreleasePool].
    ///
    /// This type can be dereferenced into [ActiveAutoreleasePool].
    ///
    /// Pops the pool on drop.
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
    ///Pops the pool
    impl Drop for AutoreleasePool {
        fn drop(&mut self) { loop { } }
    }
    pub fn autoreleasepool<F: FnOnce(&ActiveAutoreleasePool) -> R, R>(f: F)
     -> R {
        loop { }
    }
    impl AutoreleasePool {
        ///Creates a new pool.  The pool will be dropped when this type is dropped.
        ///
        /// # Safety
        /// Autorelease pools must be dropped in reverse order to when they are created. If you don't want to maintain
        /// this invariant yourself, see the [autoreleasepool] safe wrapper.
        pub unsafe fn new() -> Self { loop { } }
    }
}
mod arguments {
    ///!Rust doesn't natively support varargs, so encoding the args
    ///!into an "anonymous" type that implements this trait is a convenient
    ///! way to pass the objcargs to functions.
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
    ///Trait describing a type that can be used as arugments.  Generally, this is a tuple of all the arguments to some method.
    ///
    /// This type is sealed; you may not implement it from outside the crate.
    /// All implementations are provided via macro.
    pub trait Arguments: Sized + Debug + crate::private::Sealed {
        ///Implementation deatil of [PerformsSelector::perform_primitive]
        unsafe fn invoke_primitive<R: Primitive>(receiver: *mut c_void,
                                                 sel: Sel,
                                                 pool: &ActiveAutoreleasePool,
                                                 args: Self)
        -> R;
        ///Implementation detail of [PerformsSelectorSuper::perform_super_primitive]
        unsafe fn invoke_primitive_super<R: Primitive>(obj: *mut c_void,
                                                       sel: Sel,
                                                       _pool:
                                                           &ActiveAutoreleasePool,
                                                       class: *const AnyClass,
                                                       args: Self)
        -> R;
        ///Implementation detail of [PerformsSelector::perform]
        unsafe fn invoke<R: ObjcInstance>(receiver: *mut c_void, sel: Sel,
                                          pool: &ActiveAutoreleasePool,
                                          args: Self)
        -> *const R;
        ///Implementation detail of [PerformsSelectorSuper::perform_super]
        unsafe fn invoke_super<R: ObjcInstance>(receiver: *mut c_void,
                                                sel: Sel,
                                                pool: &ActiveAutoreleasePool,
                                                class: *const AnyClass,
                                                args: Self)
        -> *const R;
        ///Implementation detail of [PerformsSelector::perform_result]
        unsafe fn invoke_error<'a,
                               R: ObjcInstance>(receiver: *mut c_void,
                                                sel: Sel,
                                                pool:
                                                    &'a ActiveAutoreleasePool,
                                                args: Self)
        -> Result<*const R, AutoreleasedCell<'a, NSError>>;
        ///Implementation detail of [PerformablePointer::perform_result_autorelease_to_retain]
        unsafe fn invoke_error_trampoline_strong<'a,
                                                 R: ObjcInstance>(obj:
                                                                      *mut c_void,
                                                                  sel: Sel,
                                                                  _pool:
                                                                      &'a ActiveAutoreleasePool,
                                                                  args: Self)
        -> Result<*const R, AutoreleasedCell<'a, NSError>>;
        ///Implementation detail of [PerformsSelectorSuper::perform_super_result_autorelease_to_retain]
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
        ///Implementation detail of [PerformsSelectorSuper::perform_super_autorelease_to_retain]
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
    ///Can be used as an argument in objr
    ///
    /// This constraint provides additional safety around transmuting fp types.
    ///
    /// # Safety
    /// The primary constraint of this protocol is it needs to be FFI-safe (`#[repr(transparent)]` or `#[repr(C)]`).
    /// Since this cannot be otherwise verified, we're going to declare it `unsafe`.
    /// # See also
    /// [Primitive], which implies this trait. The difference is that [Arguable] does not allow the [PerformsSelector::perform_primitive()]
    /// family in its return type.
    pub unsafe trait Arguable { }
    unsafe impl <O: ObjcInstance> Arguable for &O { }
    unsafe impl <O: ObjcInstance> Arguable for *const O { }
    ///Non-reference types that are ObjC FFI-safe.  This marker
    /// allows access to the [PerformsSelector::perform_primitive()] family.
    ///
    /// # Safety
    /// Type must be FFI-safe.
    ///
    /// # Note
    /// This is unsealed because we want to allow structs to be declared as primitives in external crates.
    ///
    /// # See also
    /// [Arguable], which is implied by this trait.  The difference is that [Primitive] allows [PerformsSelector::perform_primitive()]
    /// family in its return type.
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
    ///Implementation macro for declaring [Argument] types.
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
             ///This function combines various common behaviors in a fast implementation.
             /// In particular I want to make sure we generate the right machinecode for `objc_retainAutoreleasedReturnValue`
             ///
             /// 1.  Invoke / performSelector
             /// 2.  Assumes trailing error parameter
             /// 3.  Caller wants +1 / StrongCell, but callee returns +0 / autoreleased.  Resolved via the magic trampoline `objc_retainAutoreleasedReturnValue`.
             ///
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
        ///This function combines various common behaviors in a fast implementation.
        /// In particular I want to make sure we generate the right machinecode for `objc_retainAutoreleasedReturnValue`
        ///
        /// 1.  Invoke / performSelector
        /// 2.  Assumes trailing error parameter
        /// 3.  Caller wants +1 / StrongCell, but callee returns +0 / autoreleased.  Resolved via the magic trampoline `objc_retainAutoreleasedReturnValue`.
        ///
        #[inline]
        unsafe fn invoke_error_trampoline_strong<'a,
                                                 R: ObjcInstance>(obj:
                                                                      *mut c_void,
                                                                  sel: Sel,
                                                                  pool:
                                                                      &'a ActiveAutoreleasePool,
                                                                  (): Self)
         -> Result<*const R, AutoreleasedCell<'a, NSError>> {
            use crate::performselector::objc_retainAutoreleasedReturnValue;
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
            use crate::performselector::objc_retainAutoreleasedReturnValue;
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
        ///This function combines various common behaviors in a fast implementation.
        /// In particular I want to make sure we generate the right machinecode for `objc_retainAutoreleasedReturnValue`
        ///
        /// 1.  Invoke / performSelector
        /// 2.  Assumes trailing error parameter
        /// 3.  Caller wants +1 / StrongCell, but callee returns +0 / autoreleased.  Resolved via the magic trampoline `objc_retainAutoreleasedReturnValue`.
        ///
        #[inline]
        unsafe fn invoke_error_trampoline_strong<'a,
                                                 R: ObjcInstance>(obj:
                                                                      *mut c_void,
                                                                  sel: Sel,
                                                                  pool:
                                                                      &'a ActiveAutoreleasePool,
                                                                  (a,): Self)
         -> Result<*const R, AutoreleasedCell<'a, NSError>> {
            use crate::performselector::objc_retainAutoreleasedReturnValue;
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
            use crate::performselector::objc_retainAutoreleasedReturnValue;
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
        ///This function combines various common behaviors in a fast implementation.
        /// In particular I want to make sure we generate the right machinecode for `objc_retainAutoreleasedReturnValue`
        ///
        /// 1.  Invoke / performSelector
        /// 2.  Assumes trailing error parameter
        /// 3.  Caller wants +1 / StrongCell, but callee returns +0 / autoreleased.  Resolved via the magic trampoline `objc_retainAutoreleasedReturnValue`.
        ///
        #[inline]
        unsafe fn invoke_error_trampoline_strong<'a,
                                                 R: ObjcInstance>(obj:
                                                                      *mut c_void,
                                                                  sel: Sel,
                                                                  pool:
                                                                      &'a ActiveAutoreleasePool,
                                                                  (a, b):
                                                                      Self)
         -> Result<*const R, AutoreleasedCell<'a, NSError>> {
            use crate::performselector::objc_retainAutoreleasedReturnValue;
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
            use crate::performselector::objc_retainAutoreleasedReturnValue;
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
        ///This function combines various common behaviors in a fast implementation.
        /// In particular I want to make sure we generate the right machinecode for `objc_retainAutoreleasedReturnValue`
        ///
        /// 1.  Invoke / performSelector
        /// 2.  Assumes trailing error parameter
        /// 3.  Caller wants +1 / StrongCell, but callee returns +0 / autoreleased.  Resolved via the magic trampoline `objc_retainAutoreleasedReturnValue`.
        ///
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
            use crate::performselector::objc_retainAutoreleasedReturnValue;
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
            use crate::performselector::objc_retainAutoreleasedReturnValue;
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
        ///This function combines various common behaviors in a fast implementation.
        /// In particular I want to make sure we generate the right machinecode for `objc_retainAutoreleasedReturnValue`
        ///
        /// 1.  Invoke / performSelector
        /// 2.  Assumes trailing error parameter
        /// 3.  Caller wants +1 / StrongCell, but callee returns +0 / autoreleased.  Resolved via the magic trampoline `objc_retainAutoreleasedReturnValue`.
        ///
        #[inline]
        unsafe fn invoke_error_trampoline_strong<'a,
                                                 R: ObjcInstance>(obj:
                                                                      *mut c_void,
                                                                  sel: Sel,
                                                                  pool:
                                                                      &'a ActiveAutoreleasePool,
                                                                  (a, b, c,
                                                                   d): Self)
         -> Result<*const R, AutoreleasedCell<'a, NSError>> {
            use crate::performselector::objc_retainAutoreleasedReturnValue;
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
            use crate::performselector::objc_retainAutoreleasedReturnValue;
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
        ///This function combines various common behaviors in a fast implementation.
        /// In particular I want to make sure we generate the right machinecode for `objc_retainAutoreleasedReturnValue`
        ///
        /// 1.  Invoke / performSelector
        /// 2.  Assumes trailing error parameter
        /// 3.  Caller wants +1 / StrongCell, but callee returns +0 / autoreleased.  Resolved via the magic trampoline `objc_retainAutoreleasedReturnValue`.
        ///
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
            use crate::performselector::objc_retainAutoreleasedReturnValue;
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
            use crate::performselector::objc_retainAutoreleasedReturnValue;
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
        ///This function combines various common behaviors in a fast implementation.
        /// In particular I want to make sure we generate the right machinecode for `objc_retainAutoreleasedReturnValue`
        ///
        /// 1.  Invoke / performSelector
        /// 2.  Assumes trailing error parameter
        /// 3.  Caller wants +1 / StrongCell, but callee returns +0 / autoreleased.  Resolved via the magic trampoline `objc_retainAutoreleasedReturnValue`.
        ///
        #[inline]
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
            use crate::performselector::objc_retainAutoreleasedReturnValue;
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
            use crate::performselector::objc_retainAutoreleasedReturnValue;
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
        ///This function combines various common behaviors in a fast implementation.
        /// In particular I want to make sure we generate the right machinecode for `objc_retainAutoreleasedReturnValue`
        ///
        /// 1.  Invoke / performSelector
        /// 2.  Assumes trailing error parameter
        /// 3.  Caller wants +1 / StrongCell, but callee returns +0 / autoreleased.  Resolved via the magic trampoline `objc_retainAutoreleasedReturnValue`.
        ///
        #[inline]
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
            use crate::performselector::objc_retainAutoreleasedReturnValue;
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
            use crate::performselector::objc_retainAutoreleasedReturnValue;
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
        ///This function combines various common behaviors in a fast implementation.
        /// In particular I want to make sure we generate the right machinecode for `objc_retainAutoreleasedReturnValue`
        ///
        /// 1.  Invoke / performSelector
        /// 2.  Assumes trailing error parameter
        /// 3.  Caller wants +1 / StrongCell, but callee returns +0 / autoreleased.  Resolved via the magic trampoline `objc_retainAutoreleasedReturnValue`.
        ///
        #[inline]
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
            use crate::performselector::objc_retainAutoreleasedReturnValue;
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
            use crate::performselector::objc_retainAutoreleasedReturnValue;
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
        ///This function combines various common behaviors in a fast implementation.
        /// In particular I want to make sure we generate the right machinecode for `objc_retainAutoreleasedReturnValue`
        ///
        /// 1.  Invoke / performSelector
        /// 2.  Assumes trailing error parameter
        /// 3.  Caller wants +1 / StrongCell, but callee returns +0 / autoreleased.  Resolved via the magic trampoline `objc_retainAutoreleasedReturnValue`.
        ///
        #[inline]
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
            use crate::performselector::objc_retainAutoreleasedReturnValue;
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
            use crate::performselector::objc_retainAutoreleasedReturnValue;
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
    ///Types that can be performedSelector.
    ///
    /// # Stability
    /// Do not implement this type directly.  Instead use [objc_instance!] or [objc_class!].
    ///
    /// # Safety
    /// This requires the underlying type to be FFI-safe and a valid ObjC pointer.
    ///
    pub unsafe trait PerformablePointer { }
    unsafe impl <O: ObjcInstance> PerformablePointer for O { }
    ///Trait where we can also call methods on super.  This requires knowing a superclass.
    /// # Stability
    /// Do not implement this type directly.  Instead use [objc_instance!] or [objc_class!].
    ///
    /// # Safety
    /// This requires the underlying type to be FFI-safe and a valid Objc pointer.
    ///
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
    ///Trait that provides `PerformSelector` implementations.  Autoimplelmented for `T: PerformablePointer`
    ///
    /// # Stability
    /// Do not implement this trait yourself.  Instead use [objc_instance!] or [objc_class!]
    pub trait PerformsSelector {
        ///Performs selector, returning a primitive type.
        /// # Safety
        /// See the safety section of [objc_instance!].
        unsafe fn perform_primitive<A: Arguments,
                                    R: Primitive>(receiver: *mut Self,
                                                  selector: Sel,
                                                  pool:
                                                      &ActiveAutoreleasePool,
                                                  args: A)
        -> R;
        ///Performs, returning the specified [ObjcInstance].  You must coerce this into some type according to your knowledge of ObjC convention.
        unsafe fn perform<A: Arguments,
                          R: ObjcInstance>(receiver: *mut Self, selector: Sel,
                                           pool: &ActiveAutoreleasePool,
                                           args: A)
        -> *const R;
        ///Performs, returning the result of the specified [ObjcInstance].  You must coerce this into some type according to your knowledge of ObjC convention.
        ///
        /// By convention, the error value is an autoreleased [NSError].
        ///
        ///# Safety
        ///See the safety section of [objc_instance!].
        unsafe fn perform_result<'a, A: Arguments,
                                 R: ObjcInstance>(receiver: *mut Self,
                                                  selector: Sel,
                                                  pool:
                                                      &'a ActiveAutoreleasePool,
                                                  args: A)
        -> Result<*const R, AutoreleasedCell<'a, NSError>>;
        ///Performs, returning the specified [ObjcInstance].
        ///
        /// This variant assumes 1) the calling convention is +0, 2) the type returned to you is +1.  The implementation
        /// knows a trick to perform this conversion faster than you can do it manually.
        ///# Safety
        ///See the safety section of [objc_instance!].
        unsafe fn perform_autorelease_to_retain<A: Arguments,
                                                R: ObjcInstance>(receiver:
                                                                     *mut Self,
                                                                 selector:
                                                                     Sel,
                                                                 pool:
                                                                     &ActiveAutoreleasePool,
                                                                 args: A)
        -> *const R;
        ///Performs, returning the specified [ObjcInstance].
        ///
        /// This variant assumes 1) the calling convention is +0, 2) the type returned to you is +1.  The implementation
        /// knows a trick to perform this conversion faster than you can do it manually.
        ///By convention, the error value is an autoreleased [NSError].
        ///# Safety
        ///See the safety section of [objc_instance!].
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
    ///implementation detail of perform_autorelease_to_strong_nonnull
    /// written here to ensure tailcall optimization
    ///
    /// # Safety
    /// Issues include:
    /// 1.  ptr argument is raw and we don't check anything
    /// 2.  This function logically increments a reference count (may be elided at runtime)
    ///
    /// Optimal performance of this function requires the compiler to do tailcall optimization.
    /// Hopefully I've written it clearly enough for it to understand.
    #[inline(always)]
    unsafe fn magic_retaining_trampoline<A: Arguments,
                                         R: ObjcInstance>(ptr: *mut c_void,
                                                          selector: Sel,
                                                          pool:
                                                              &ActiveAutoreleasePool,
                                                          args: A)
     -> *const R {
        loop { }
    }
    /// Variant of [magic_retaining_trampoline] for super.
    /// # Safety
    /// In addition to the issues of [magic_retaining_trampoline], there is no verification that you have passed the correct super_class.
    #[inline(always)]
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
        #[inline]
        unsafe fn perform_primitive<A: Arguments,
                                    R: Primitive>(receiver: *mut Self,
                                                  selector: Sel,
                                                  pool:
                                                      &ActiveAutoreleasePool,
                                                  args: A) -> R {
            loop { }
        }
        #[inline]
        unsafe fn perform<A: Arguments,
                          R: ObjcInstance>(receiver: *mut Self, selector: Sel,
                                           pool: &ActiveAutoreleasePool,
                                           args: A) -> *const R {
            loop { }
        }
        #[inline]
        unsafe fn perform_result<'a, A: Arguments,
                                 R: ObjcInstance>(receiver: *mut Self,
                                                  selector: Sel,
                                                  pool:
                                                      &'a ActiveAutoreleasePool,
                                                  args: A)
         -> Result<*const R, AutoreleasedCell<'a, NSError>> {
            loop { }
        }
        #[inline]
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
        #[inline]
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
    ///Variants of the perform functions that talk to `super` instead of `self`.  In general, this is supported on classes.
    pub trait PerformsSelectorSuper {
        ///Performs selector, returning a primitive type.
        ///
        /// # Safety
        ///See the safety section of [objc_instance!].
        unsafe fn perform_super_primitive<A: Arguments,
                                          R: Primitive>(receiver: *mut Self,
                                                        selector: Sel,
                                                        pool:
                                                            &ActiveAutoreleasePool,
                                                        args: A)
        -> R;
        ///Performs, returning the specified [ObjcInstance].  You must coerce this into some type according to your knowledge of ObjC convention.
        ///
        /// # Safety
        ///See the safety section of [objc_instance!].
        unsafe fn perform_super<A: Arguments,
                                R: ObjcInstance>(receiver: *mut Self,
                                                 selector: Sel,
                                                 pool: &ActiveAutoreleasePool,
                                                 args: A)
        -> *const R;
        ///Performs, returning the result of the specified [ObjcInstance].  You must coerce this into some type according to your knowledge of ObjC convention.
        ///
        /// By convention, the error value is an autoreleased [NSError].
        ///
        ///
        /// # Safety
        ///See the safety section of [objc_instance!].
        unsafe fn perform_super_result<A: Arguments,
                                       R: ObjcInstance>(receiver: *mut Self,
                                                        selector: Sel,
                                                        pool:
                                                            &ActiveAutoreleasePool,
                                                        args: A)
        -> Result<*const R, AutoreleasedCell<'_, NSError>>;
        ///Performs, returning the specified [ObjcInstance].
        ///
        /// This variant assumes 1) the calling convention is +0, 2) the type returned to you is +1.  The implementation
        /// knows a trick to perform this conversion faster than you can do it manually.
        ///
        ///
        /// # Safety
        ///See the safety section of [objc_instance!].
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
        ///Performs, returning the specified [ObjcInstance].
        ///
        /// This variant assumes 1) the calling convention is +0, 2) the type returned to you is +1.  The implementation
        /// knows a trick to perform this conversion faster than you can do it manually.
        ///By convention, the error value is an autoreleased [NSError].
        ///
        /// # Safety
        ///See the safety section of [objc_instance!].
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
        #[inline]
        unsafe fn perform_super_primitive<A: Arguments,
                                          R: Primitive>(receiver: *mut Self,
                                                        selector: Sel,
                                                        pool:
                                                            &ActiveAutoreleasePool,
                                                        args: A) -> R {
            loop { }
        }
        #[inline]
        unsafe fn perform_super<A: Arguments,
                                R: ObjcInstance>(receiver: *mut Self,
                                                 selector: Sel,
                                                 pool: &ActiveAutoreleasePool,
                                                 args: A) -> *const R {
            loop { }
        }
        #[inline]
        unsafe fn perform_super_result<A: Arguments,
                                       R: ObjcInstance>(receiver: *mut Self,
                                                        selector: Sel,
                                                        pool:
                                                            &ActiveAutoreleasePool,
                                                        args: A)
         -> Result<*const R, AutoreleasedCell<'_, NSError>> {
            loop { }
        }
        #[inline]
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
        #[inline]
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
    ///Marks that a given type is an objc type, e.g. its instances are an objc object.
    ///This is the case for classes, but also for protocols.
    ///
    /// # Stability
    /// It is not stable API to implement this trait yourself.  Instead, declare a conforming
    /// type via [objc_instance!] macro.
    ///
    pub trait ObjcInstance { }
    ///A nonnull, but immutable type.  This allows various optimizations like pointer-packing `Option<T>`.
    ///
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
        ///Assumes the object has been retained and converts to a StrongCell.
        ///
        /// # Safety
        /// You must guarantee each of the following:
        /// * Object was retained (+1)
        /// * Object is not deallocated
        /// * Object was initialized
        /// * Object is 'static, that is, it has no references to external (Rust) memory.
        /// If this is not the case, see [NonNullImmutable::assume_retained_limited].
        pub unsafe fn assume_retained(self) -> StrongCell<T> { loop { } }
        ///Assumes the object has been retained and converts to a StrongLifetimeCell.
        ///
        /// # Safety
        /// You must guarantee each of the following:
        /// * Object was retained (+1)
        /// * Object is not deallocated
        /// * Object was initialized
        /// * That the object can remain valid for the lifetime specified.  e.g., all "inner pointers" or "borrowed data" involved
        /// in this object will remain valid for the lifetime specified, which is unbounded.
        /// * That all objc APIs which end up seeing this instance will either only access it for the lifetime specified,
        ///   or will take some other step (usually, copying) the object into a longer lifetime.
        pub unsafe fn assume_retained_limited<'a>(self)
         -> StrongLifetimeCell<'a, T> where T: 'a {
            loop { }
        }
        ///Assumes the object has been autoreleased and converts to an AutoreleasedCell.
        ///
        /// # Safety:
        /// You must guarantee each of the following:
        /// * Object is autoreleased already
        /// * Object is not deallocated
        /// * Object was initialized
        pub unsafe fn assume_autoreleased<'a>(self,
                                              pool: &'a ActiveAutoreleasePool)
         -> AutoreleasedCell<'a, T> {
            loop { }
        }
        ///Converts to a raw pointer
        pub(crate) fn as_ptr(&self) -> *const T { loop { } }
        ///Assumes the passed pointer is non-nil.
        ///
        /// # Safety
        /// You must guarantee each of the following:
        /// * Pointer is non-nil
        /// * Points to a valid objc object of the type specified
        pub(crate) unsafe fn assume_nonnil(ptr: *const T) -> Self { loop { } }
        ///Dereferences the inner pointer.
        ///
        /// # Safety
        /// You must guarantee each of the following
        /// * Object is not deallocated
        /// * Object will not be deallocated for the lifetime of `self` (e.g., the lifetime of the returned reference)
        /// * Object was initialized
        unsafe fn as_ref(&self) -> &T { loop { } }
        ///Retains the inner pointer and converts to [StrongCell]
        ///
        /// # Safety
        /// You must guarantee each of the following
        /// * Object is not deallocated
        /// * object was initialized
        pub unsafe fn retain(&self) -> StrongCell<T> { loop { } }
    }
    ///Behavior we define for any [ObjcInstance].
    pub trait ObjcInstanceBehavior {
        ///Casts the type to another type.
        ///
        /// # Safety
        /// There is no guarantee that the source type is compatible with the destination type.
        unsafe fn cast<R: ObjcInstance>(&self)
        -> &R;
        ///Casts the type to another type.
        ///
        /// # Safety
        /// There is no guarantee that the source type is compatible with the destination type.
        /// To the extent that you create two pointers pointing to the same instance,
        /// this may be UB
        unsafe fn cast_mut<R: ObjcInstance>(&mut self)
        -> &mut R;
        ///Assuming the pointer is non-nil, returns a pointer type.
        ///
        /// The opposite of this function is [Self::nullable].
        ///
        /// # Safety
        /// You must guarantee each of the following:
        /// * Pointer is non-nil
        /// * Points to a valid objc object of the type specified
        unsafe fn assume_nonnil(ptr: *const Self)
        -> NonNullImmutable<Self>;
        ///Safely casts the object to an `Option<NonNullImmutable>`.  Suitable for implementing nullable functions.
        fn nullable(ptr: *const Self)
        -> Option<NonNullImmutable<Self>>;
        ///Allows you to call [objr::bindings::PerformsSelector::perform] from a nonmutating context.
        ///
        /// This function should not be used for general-purpose pointer casting.
        ///
        /// # Safety
        /// This is only safe when the underlying objc method does not mutate its contents.  See [objc_instance#Mutability] for details.
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
    ///Helper for Option<NonNullable>
    pub trait NullableBehavior {
        type T: ObjcInstance;
        ///Assumes the object has been autoreleased and converts to an Option<AutoreleasedCell>
        ///
        /// # Safety:
        /// You must guarantee each of the following:
        /// * Object (if any) is autoreleased already
        /// * Object (if any) is not deallocated
        /// * Object (if any) was initialized
        unsafe fn assume_autoreleased<'a>(self,
                                          pool: &'a ActiveAutoreleasePool)
        -> Option<AutoreleasedCell<'a, Self::T>>;
        ///Assumes the object has been retained and converts to a StrongCell.
        ///
        /// # Safety
        /// You must guarantee each of the following:
        /// * Object was retained (+1)
        /// * Object (if any) is not deallocated
        /// * Object (if any) was initialized
        unsafe fn assume_retained(self)
        -> Option<StrongCell<Self::T>>;
        ///Retains the inner pointer and converts to [StrongCell]
        ///
        /// # Safety
        /// You must guarantee each of the following
        /// * Object (if any) is not deallocated
        /// * object (if any) was initialized
        unsafe fn retain(self)
        -> Option<StrongCell<Self::T>>;
        ///Assumes the object has been retained and converts to a StrongLifetimeCell.
        ///
        /// # Safety
        /// You must guarantee each of the following:
        /// * Object (if any) was retained (+1)
        /// * Object (if any) is not deallocated
        /// * Object (if any) was initialized
        /// * That the object (if any) can remain valid for the lifetime specified.  e.g., all "inner pointers" or "borrowed data" involved
        /// in this object will remain valid for the lifetime specified, which is unbounded.
        /// * That all objc APIs which end up seeing this instance will either only access it for the lifetime specified,
        ///   or will take some other step (usually, copying) the object into a longer lifetime.
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
    ///Helper for Option<StrongCell>
    pub trait NullableCellBehavior {
        type T: ObjcInstance;
        ///Converts to a mutable version.
        ///
        /// # Safety
        /// You are responsible to check:
        /// * There are no other references to the type, mutable or otherwise
        /// * The type is in fact "mutable", whatever that means.  Specifically, to whatever extent `&mut` functions are forbidden
        ///   generally, you must ensure it is appropriate to call them here.
        unsafe fn assume_mut(self)
        -> Option<StrongMutCell<Self::T>>;
    }
    impl <O: ObjcInstance> NullableCellBehavior for Option<StrongCell<O>> {
        type T = O;
        unsafe fn assume_mut(self) -> Option<StrongMutCell<Self::T>> {
            loop { }
        }
    }
    /**
Defines a struct (binding) for a specific ObjC type.  This doesn't assume the type is a class, if it is a class consider [objc_class!].

The type will automagically conform to [objr::bindings::ObjcInstance], but will not conform to [objr::bindings::ObjcClass].

# Example

```
#![link(name="Foundation",kind="framework")]
use objr::bindings::*;
objc_instance! {
    pub struct NSExample;
}
```

# The problem

ObjC and Rust disagree on a great many things.  Rust prefers
stack allocation, ObjC types are all heap-allocated.  Rust expects static
lifetime proofs, ObjC has significant runtime memory management.  Rust expects
 to know things like whether a reference is exclusive or not that ObjC withholds.
 And of course silly things like `snake_case_methods()` vs `camelCaseMethods`.

This library is in the unenviable position of trying to please everybody, which cannot really
be done, but meanwhile I have software to write, so here is the Grand Compromise in use around here.

# Representing ObjC types

ObjC types are declared as 'opaque' Rust types.  While these types technically have a memory layout in Rust,
the memory layout is not the same as the corresponding ObjC layout.  Therefore, such types are "effectively" DSTs,
and cannot be stored on the stack or dereferenced.  For more information, see unstable feature [RFC 1861](https://rust-lang.github.io/rfcs/1861-extern-types.html).
We implement a "similar" feature in stable Rust.

In short, this is the type situation for Rust code:

1. `example: NSExample`.  This type effectively is not instantiable, but you can imagine it as the object "owned" by the ObjC runtime.  Since you can't
    move it out of the runtime you cannot use instances of it.  There are some gotchas of this type even without constructing it, e.g. its memory
    layout may be different than the situation in ObjC really would be for example.
2.  `example: *mut NSExample`.  What an ObjC (non-ARC) developer would think of as a normal reference, a Rust raw pointer.
3.  `example: *const NSExample`.  What an ObjC (non-ARC) developer would think of as an immutable reference, as some mutable methods may not be available.
4.  `example: &mut NSExample` 2, but checked by the borrowchecker. One limitation of this type is it is UB if you make it `nil`, so consider modeling with `Option`.  While this type is appropriate for parameters, it is somewhat unusual for return values as ObjC is reluctant to relate object lifetimes to each other.
5.  `example: &NSExample` 3, but checked by the borrowchecker.  One limitation of this type is it is UB if you make it `nil`, so consider modeling with `Option`.  hile this type is appropriate for parameters, it is somewhat unusual for return values as ObjC is reluctant to relate object lifetimes to each other.

ARC is its own topic, which in Rust is handled by various smart pointers.  See [objr::bindings::StrongCell] and [objr::bindings::AutoreleasedCell] for details on the pointer types.

Let's stub out a binding for some ObjC type `NSExample`:

```
//we're writing bindings
use objr::bindings::*;
///NSExample is some objc instance (such as a protocol or similar).
//If it were a class, consider objc_class! for extra features.
objc_instance! {
    //declares a Rust struct for this type.
    //Note that there is no real connection to the actual objc type, you can name it anything.
    //The connection arises by casting some pointer to this type, such as the result of [PerformsSelector::perform].
    pub struct NSExample;
}
//We can write normal Rust functions on our type
impl NSExample {
    fn new() -> Self { todo!() }

    //I generally follow ObjC syntax for method names, although I'm not gonna tell you what to do.
    #[allow(non_snake_case)]
    fn instanceMethod(&self) { todo!() }
}
```
Declaring our type with the `objc_instance!` macro performs several tasks:

1.  It declares a type which we can use as a standin for the ObjC type, in the Rust typesystem.  This allows
Rust programs to be typesafe when they work with ObjC objects
2.  It provides a container in which to write bindings for the underlying ObjC type
3.  It allows end users to call methods and get familiar behavior like [std::fmt::Display].

# Safety

This library *intends* to follow the normal Rust safety guarantees, although there are a few areas that are
more risky than other libraries.

In general, ObjC is a giant ball of unsafe, opaque code.  If you are using this macro, or using something that uses this macro,
you are calling into that giant ball of unsafe code, and who knows if it's sound or not.

With that background, there are really two bad options:

a) Insert various runtime checks everywhere.  This is slow and stuff still slips through.
b) Just assume ObjC works as specified.  This is fast and more stuff slips through.

This library picks b.  Now we cover the topic with examples.


## FFI-safety

[ObjcInstance] type is declared `#[repr(C)]` and pointers to it are valid objc pointers.
So they can be passed directly to any method that expects an ObjC argument.  For example, C functions,
fucntions that implement subclassing inside Rust, etc.

Real ObjC objects can (usually) not be allocated on the stack.  This macro should prevent
owned pointers from being constructed (e.g. on the stack).

## Memory management

You can find out more about in the documentation
for [StrongCell](objr::bindings::StrongCell) or [AutoreleasedCell](objr::bindings::AutoreleasedCell).  Suffice it to say here that use of either cell
largely protects you from dangling pointers, but ObjC is pretty much one giant `unsafe` block, so
it's always possible the ObjC side accidentally frees your memory or even does it on purpose.

Return types for your binding require special consideration.  In general, ObjC memory rules are "by convention",
based on the method name, widely assumed by ObjC programmers, or be buggy and do the opposite thing in rare cases.

However, there are deep mysteries of ObjC not even known to most ObjC programmers.  In practice, the return type
you want is usually [StrongCell](objr::bindings::StrongCell), even in cases where the function is known to
be autoreleased (+0 convention).  Why is this?

In fact, the conventional model that ObjC methods return "either" +0 (autoreleased) or +1 (retain/create/copy) is out of date.
Most +0 methods don't return an autorelease object, but return the result of [`_objc_autoreleaseReturnValue`](https://clang.llvm.org/docs/AutomaticReferenceCounting.html#id63).
This obscure runtime function walks up the stack frame to inspect callers.  Callers that are "dumb" get the +0 object,
but smart callers can get a +1 object.

To be a smart caller, call a function like [`objr::bindings::PerformsSelector::perform_autorelease_to_retain`].  This will promote your +0 pointer to +1,
which can then be passed to [StrongCell](objr::bindings::StrongCell).

## Nullability

Another issue is that in ObjC, whether APIs are "nullable" (return a null pointer, usually called 'nil', treated in practice
like Rust `Option<T>::None`) is also by convention.

Unfortunately, in Rust it is UB to construct a reference to null.  Therefore a choice needs to be made about
whether an ObjC pointer should be interepreted as `Option<&T>` or `&T` and the wrong one may UB.

In general, this library takes the view that ObjC functions are correctly implemented.  Therefore, when something is
documented or "widely known" to be nonnull we use `&T` without checking.  This follows the precedent of languages like Swift,
although Swift has had trouble with this too.  For more information, see [SR-8622](https://bugs.swift.org/browse/SR-8622).

## Mutability

The fact is that every ObjC object is behind many shared mutable references.  ObjC has no law against
mutating its own state at any time, and effectly all pointers in the language are always mutable.  This is undesireable
to Rust developers who may be used to holding inner references to a type and using the borrow checker to prove
that the type is not mutated during the lifetime of inner references.  This pattern of inner references
is substantially less likely for ObjC objects although it does crop up in the context of a few types.

ObjC does have a concept of mutability/immutability, through type pairs (like `NSString` vs `NSMutableString`).
This can be used to achieve some version of mutability guarantees, however `NSString` may do some "inner mutation" somewhere,
so as the basis for a Rust system it isn't great.

Instead, I have implemented `&` and `&mut` as orthogonal to `NSString` vs `NSMutableString`.  You can have `&mut NSString`
and `&NSMutableString`.

Methods that I have reason to suspect mutate the inner storage are declared `fn mutating(&mut self)`, while methods I think
do not are implemented `fn nonmutating(&self)`.  In practice, this means a lot of the `NSMutable` type methods are (`&mut`) and
the former are `&`.

This generally works as Rust developers expect, with the proviso that it relies on, yet again, convention.  In practice,
there is no law that ObjC can't release your references internally if you call some "immutable" method, so maybe your safe code
can do UB.  I consider these to be bugs, and please file them if you encounter them, but effectively, I think it's preferable
for "immutable" methods to be immutable, than for everything to be `&mut`.

There are some methods that can create "additional" `&mut` references to a type, these are declared `unsafe` because
they may be used to violate Rust's exclusive references.

## Exceptions

ObjC exceptions are analogous to Rust panics.  In practice they abort your program, there is technically some way to handle them
but nobody does, and the decision to support that is a very unfortunate design mistake that now lives on forever.

More unfortunately, encountering an ObjC exception in Rust is UB.  This is substantially worse than a normal abort,
because you may not even get a reasonable abort or error message.

Since these are primarily not intended to be handled, it is undesireable to try to catch them.  Instead, the recommended approach
is to validate arguments on the Rust side (such as with a Rust assert or panic) so that they won't encountered on the ObjC side.
Or alternatively, to mark bindings as `unsafe` when there is some suspicion that ObjC exceptions may occur and push the problem
into the caller.

There is a [objr::bindings::try_unwrap_void] function which can upgrade the UB to a hard abort.
This function is expensive and not recommended for general use, but it is useful for debugging when you get a weird crash
and need to see an exception print to understand what is wrong.

Having exceptions as UB is a bit scary.  Once again though, we are following in the footsteps of Swift which does something very
similar.  Unfortunately, Swift is better at wringing a proper error message out of the exception, even though it isn't totally
reliable either.

# Generic types
Both ObjC and Rust support generics, which are vaguely similar concepts.  However, ObjC's notion of generics is highly 'bolted
on top': it serves as a compile-time assertion that some function accepts or returns a particular type, but it does not
actually constrain the runtime behavior, not does specialization create a distinct type.

The best way to project this in Rust is to project the "bolted on top" model.  Therefore (and also for technical reasons), this
macro does not accept generic arguments, but [objc_instance_newtype] does.

 */
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
    /**
Declares a newtype that wraps an existing objc instance type.

Downcasts to the raw type will be implemented for you.  Upcasts will not, implement them yourself with [objr::bindings::ObjcInstanceBehavior::cast()] if applicable.
```no_run
use objr::bindings::*;
objc_instance! {
    struct NSExample;
}
objc_instance_newtype! {
    struct SecondExample: NSExample;
}
let s: &SecondExample = todo!();
let e: &NSExample = s.into();

let s: &mut SecondExample = todo!();
let e: &mut NSExample = s.into();
```

unlike [objc_instance!], this macro supports generic types, allowing you to wrap some other type with generics bolted on top.

At the moment, restrictions on generic arguments are not supported at the type level, but you can add them on your own impl blocks

```
use objr::bindings::*;
objc_instance! {
    struct NSExample;
}
objc_instance_newtype! {
    struct SecondExample<A,B>: NSExample;
}
//further restriction
impl<A: PartialEq,B: PartialEq> SecondExample<A,B> { }
```
*/
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
    ///Defines some behavior on `Option<&ObjcInstance>`
    pub trait OptionalInstanceBehavior<Deref> {
        ///Gets a pointer for the option.  If `self` is `nil`, the pointer will be `null`, otherwise it will be the underlying reference.
        fn as_ptr(&self)
        -> *const Deref;
    }
    impl <T: ObjcInstance> OptionalInstanceBehavior<T> for Option<&T> {
        fn as_ptr(&self) -> *const T { loop { } }
    }
}
mod typealias {
    //! These are typealiases to the types used in objc
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
    ///ObjC-compatible selector.  This type is repr-transparent and can go over the wire as an arg.
    #[repr(transparent)]
    pub struct Sel(*const c_void);
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for Sel { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for Sel {
        #[inline]
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
        ///Dynamically creates `Sel` from a string by quering the ObjC runtime.  Note that in most cases, [objc_selector_group!()] is a faster method
        /// to get selectors.
        pub fn from_str(string: &str) -> Self { loop { } }
        pub unsafe fn ptr(&self) -> *const c_void { loop { } }
        pub const fn from_ptr(ptr: *const c_void) -> Sel { Sel(ptr) }
    }
    ///Primarily used by [objc_subclass!] and similar.
    #[repr(transparent)]
    #[doc(hidden)]
    pub struct _SyncWrapper<T>(pub T);
    unsafe impl <T> core::marker::Sync for _SyncWrapper<T> { }
    #[link_section = "__DATA,__objc_imageinfo,regular,no_dead_strip"]
    #[export_name = "\x01L_OBJC_IMAGE_INFO"]
    #[used]
    static IMAGE_INFO: [u32; 2] = [0, 64];
    ///Statically declares a selector and makes it available for use.
    ///
    /// Before the program entrypoint, dyld will identify these selectors and replace them
    /// with the value known to the ObjC runtime.  This is substantially faster than `Sel::from_str()` which is a runtime behavior
    /// that involves acquiring a lock.
    ///
    /// # Example
    /// ```
    /// use objr::objc_selector_group;
    /// use objr::bindings::*;
    /// objc_selector_group!(
    ///         //Declare a trait.  The trait will have members for each selector.
    ///         trait NSObjectSelectors {
    ///             //each ObjC selector, in normal ObjC selector syntax
    ///             @selector("description")
    ///             @selector("respondsToSelector:")
    ///             @selector("init")
    ///         }
    ///         //Implement the trait on Sel.  This allows the use of `Sel::description()` etc.
    ///         impl NSObjectSelectors for Sel {}
    ///     );
    /// unsafe {
    ///     let my_selector = Sel::description();
    /// }
    /// ```
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
    //! NSError implementation
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
        ///A friendlier unwrap for [NSError] that prints the error if you encounter it.
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
mod subclass {
    #[macro_export]
    #[doc(hidden)]
    macro_rules! __objc_sublcass_implpart_method_prelude {
        ($MethodT : ident, $MethodListT : ident) =>
        {
            #[repr(C)] struct $MethodT
            { name : * const u8, types : * const u8, imp : * const c_void }
            #[repr(C)] struct $MethodListT < const SIZE : usize >
            { magic : u32, count : u32, methods : [MethodT ; SIZE], }
        }
    }
    #[macro_export]
    #[doc(hidden)]
    macro_rules! __objc_subclass_implpart_a {
        ($pub : vis, $identifier : ident, $objcname : ident, $superclass :
         ident, $IvarListT : ident, $ClassRoT : ident, $CLASS_NAME : ident,
         $CLASS_FLAGS : ident, $METACLASS_FLAGS : ident, $CLASST : ident,
         $NSSUPER_CLASS : ident, $OBJC_EMPTY_CACHE : ident) =>
        {
            use core :: ffi :: c_void ; #[repr(C)] struct $IvarListT
            {
                magic : u32, count : u32, offset : * const u32, name : * const
                u8, r#type : * const u8, alignment : u32, size : u32
            } #[repr(C)] struct $ClassRoT
            {
                flags : u32, instance_start : u32, instance_size : u32,
                reserved : u32, ivar_layout : * const c_void, name : * const
                u8, base_method_list : * const c_void, base_protocols : *
                const c_void, ivars : * const IvarListT, weak_ivar_layout : *
                const c_void, base_properties : * const c_void,
            } objr :: bindings :: __static_asciiz!
            ("__TEXT,__objc_classname,cstring_literals", $CLASS_NAME,
             $objcname) ; const RO_FLAGS_METACLASS : u32 = 1 ; const
            RO_FLAGS_HIDDEN : u32 = 1 << 4 ; const RO_FLAGS_ARR : u32 = 1 << 7
            ; const $CLASS_FLAGS : u32 = RO_FLAGS_HIDDEN | RO_FLAGS_ARR ;
            const METACLASS_FLAGS : u32 = RO_FLAGS_METACLASS | RO_FLAGS_HIDDEN
            | RO_FLAGS_ARR ; objr :: bindings :: __static_expr!
            ("__DATA,__objc_const", "_OBJC_METACLASS_RO_$_", $objcname, static
             METACLASS_RO : objr :: bindings :: _SyncWrapper < $ClassRoT > =
             objr :: bindings ::
             _SyncWrapper(ClassRoT
                          {
                              flags : METACLASS_FLAGS, instance_start : 40,
                              instance_size : 40, reserved : 0, ivar_layout :
                              std :: ptr :: null(), name : & CLASS_NAME as *
                              const u8, base_method_list : std :: ptr ::
                              null(), base_protocols : std :: ptr :: null(),
                              ivars : std :: ptr :: null(), weak_ivar_layout :
                              std :: ptr :: null(), base_properties : std ::
                              ptr :: null(),
                          }) ;) ; #[repr(C)] pub struct $CLASST
            {
                isa : * const * const c_void, superclass : * const * const
                c_void, cache : * const * const c_void, vtable : * const
                c_void, ro : * const ClassRoT
            } #[link(name = "CoreFoundation", kind = "framework")] extern
            {
                #[link_name = "OBJC_METACLASS_$_NSObject"] static
                NSOBJECT_METACLASS : * const c_void ; objr :: bindings ::
                __static_extern!
                ("OBJC_CLASS_$_", $superclass, static $NSSUPER_CLASS : * const
                 c_void ;) ; objr :: bindings :: __static_extern!
                ("OBJC_METACLASS_$_", $superclass, static NSSUPER_METACLASS :
                 * const c_void ;) ;
            } #[link(name = "objc", kind = "dylib")] extern
            {
                #[link_name = "_objc_empty_cache"] static $OBJC_EMPTY_CACHE :
                * const c_void ;
            } objr :: bindings :: __static_expr!
            ("__DATA,__objc_data", "OBJC_METACLASS_$_", $objcname, static
             METACLASS : objr :: bindings :: _SyncWrapper < $CLASST > = objr
             :: bindings ::
             _SyncWrapper($CLASST
                          {
                              isa : unsafe { & NSOBJECT_METACLASS },
                              superclass : unsafe { & NSSUPER_METACLASS },
                              cache : unsafe { & OBJC_EMPTY_CACHE }, vtable :
                              std :: ptr :: null(), ro : & METACLASS_RO.0
                          }) ;) ;
        }
    }
    #[macro_export]
    #[doc(hidden)]
    macro_rules! __objc_subclass_implpart_class_ro {
        ($objcname : ident, $CLASS_RO : ident, $ClassRoT : ident, $CLASS_FLAGS
         : expr, $payload : ty, $CLASS_NAME : expr, $IVARLISTEXPR : expr,
         $METHODLISTEXPR : expr) =>
        {
            objr :: bindings :: __static_expr!
            ("__DATA,__objc_const", "_OBJC_CLASS_RO_$_", $objcname, static
             $CLASS_RO : objr :: bindings :: _SyncWrapper < $ClassRoT > = objr
             :: bindings ::
             _SyncWrapper($ClassRoT
                          {
                              flags : $CLASS_FLAGS, instance_start : 8,
                              instance_size : 8 + std :: mem :: size_of :: <
                              $payload > () as u32, reserved : 0, ivar_layout
                              : std :: ptr :: null(), name : & $CLASS_NAME as
                              * const u8, base_method_list : $METHODLISTEXPR,
                              base_protocols : std :: ptr :: null(), ivars :
                              $IVARLISTEXPR, weak_ivar_layout : std :: ptr ::
                              null(), base_properties : std :: ptr :: null(),
                          }) ;) ;
        }
    }
    ///Declares a method list
    #[macro_export]
    #[doc(hidden)]
    macro_rules! __objc_subclass_implpart_method_list {
        ($objcname : ident, [$($objcmethod : literal, $methodfn : expr), +],
         $METHOD_LIST : ident) =>
        {
            objr :: __objc_sublcass_implpart_method_prelude!
            (MethodT, MethodListT) ;
            $(objr :: bindings :: __static_asciiz_ident_as_selector!
              ("__TEXT,__objc_methname,cstring_literals", "METHNAME_",
               $methodfn, $objcmethod) ; objr :: bindings ::
              __static_asciiz_ident_as_type_encoding!
              ("__TEXT,__objc_methtype,cstring_literals", "METHTYPE_",
               $methodfn, $objcmethod) ;) + const COUNT : usize = objr ::
            bindings :: __count! ($($methodfn), *) ; objr :: bindings ::
            __static_expr!
            ("__DATA,__objc_const", "_OBJC_$_INSTANCE_METHODS_", $objcname,
             static $METHOD_LIST : objr :: bindings :: _SyncWrapper <
             MethodListT < COUNT >> = objr :: bindings ::
             _SyncWrapper(MethodListT
                          {
                              magic : 24, count : COUNT as u32, methods :
                              [$(MethodT
                                 {
                                     name : & objr :: bindings ::
                                     __concat_idents! ("METHNAME_", $methodfn)
                                     as * const u8, types : & objr :: bindings
                                     :: __concat_idents!
                                     ("METHTYPE_", $methodfn) as * const u8,
                                     imp : $methodfn as * const c_void
                                 }), *]
                          }) ;) ;
        }
    }
    ///Declares an ivarlist (e.g., payload variants)
    #[macro_export]
    #[doc(hidden)]
    macro_rules! __objc_subclass_implpart_ivar_list {
        ($objcname : ident, $payloadtype : ty, $FRAGILE_BASE_CLASS_OFFSET :
         ident, $IVAR_LIST : ident) =>
        {
            objr :: bindings :: __static_asciiz!
            ("__TEXT,__objc_methname,cstring_literals", IVAR_NAME, "payload")
            ; objr :: bindings :: __static_asciiz!
            ("__TEXT,__objc_methtype,cstring_literals", IVAR_TYPE, "?") ; objr
            :: bindings :: __static_expr3!
            ("__DATA,__objc_ivar", "OBJC_IVAR_$_", $objcname, ".payload",
             static $FRAGILE_BASE_CLASS_OFFSET : u32 = 8 ;) ; objr :: bindings
            :: __static_expr!
            ("__DATA,__objc_const", "_OBJC_INSTANCE_VARIABLES_", $objcname,
             static $IVAR_LIST : objr :: bindings :: _SyncWrapper < IvarListT
             > = objr :: bindings ::
             _SyncWrapper(IvarListT
                          {
                              magic : 32, count : 1, offset : &
                              FRAGILE_BASE_CLASS_OFFSET, name : & IVAR_NAME as
                              * const u8, r#type : & IVAR_TYPE as * const u8,
                              alignment : std :: mem :: align_of :: <
                              $payloadtype > () as u32, size : std :: mem ::
                              size_of :: < $payloadtype > () as u32,
                          }) ;) ;
        }
    }
    ///This macro implements some methods on the wrapper type
    ///to access the underlying payload.
    #[macro_export]
    #[doc(hidden)]
    macro_rules! __objc_subclass_impl_payload_access {
        ($pub : vis, $identifier : ident, $payload : ty,
         $FRAGILE_BASE_CLASS_OFFSET : ident) =>
        {
            impl $identifier
            {
                /// Gets a mutable reference to the underlying payload.
                ///
                /// # Safety
                /// You must guarantee you are called from an exclusive, mutable context.
                ///
                /// # Design
                /// Similar to `UnsafeCell`, but
                /// 1.  Difficult to initialize a cell here
                /// 2.  I'm not sure if `UnsafeCell` is FFI-safe
                /// 3.  In practice, you need to initialize the objc memory close to 100% of the time to avoid UB.
                #[allow(dead_code)] $pub unsafe fn payload_mut(& self) -> &
                mut $payload
                {
                    let self_addr = (self as * const _ as * const u8) ; let
                    payload_addr =
                    self_addr.offset(std :: ptr ::
                                     read_volatile(&
                                                   $FRAGILE_BASE_CLASS_OFFSET)
                                     as isize) ; let payload_typed_addr = std
                    :: mem :: transmute(payload_addr) ; payload_typed_addr
                } #[allow(dead_code)] $pub fn payload(& self) -> & $payload
                { unsafe { self.payload_mut() } }
            }
        }
    }
    #[macro_export]
    #[doc(hidden)]
    macro_rules! __objc_subclass_implpart_finalize {
        ($pub : vis, $identifier : ident, $objcname : ident, $superclass :
         ident, $CLASST : ident, $CLASS_RO : ident, $NSSUPER_CLASS : expr,
         $OBJC_EMPTY_CACHE : expr) =>
        {
            objr :: bindings :: __static_expr!
            ("__DATA,__objc_data", "OBJC_CLASS_$_", $objcname, pub static
             CLASS : objr :: bindings :: _SyncWrapper < $CLASST > = objr ::
             bindings ::
             _SyncWrapper($CLASST
                          {
                              isa : unsafe
                              { std :: mem :: transmute(& METACLASS) },
                              superclass : unsafe { & $NSSUPER_CLASS }, cache
                              : unsafe { & $OBJC_EMPTY_CACHE }, vtable : std
                              :: ptr :: null(), ro : & $CLASS_RO.0
                          }) ;) ; use objr :: bindings :: { objc_instance } ;
            objc_instance! { pub struct $identifier ; } impl objr :: bindings
            :: ObjcClass for $identifier
            {
                #[inline] fn class() -> & 'static :: objr :: bindings :: Class
                < Self >
                {
                    unsafe
                    {
                        & *
                        (& CLASS.0 as * const _ as * const :: objr :: bindings
                         :: Class < Self >)
                    }
                }
            }
        }
    }
    ///Emits the subclass impl in the case have a payload
    #[macro_export]
    #[doc(hidden)]
    macro_rules! __objc_subclass_impl_with_payload_no_methods {
        ($pub : vis, $identifier : ident, $objcname : ident, $superclass :
         ident, $payload : ty) =>
        {
            objr :: __objc_subclass_implpart_a!
            ($pub, $identifier, $objcname, $superclass, IvarListT, ClassRoT,
             CLASS_NAME, CLASS_FLAGS, METACLASS_FLAGS, CLASST, NSSUPER_CLASS,
             OBJC_EMPTY_CACHE) ; objr :: __objc_subclass_implpart_ivar_list!
            ($objcname, $payload, FRAGILE_BASE_CLASS_OFFSET, IVAR_LIST) ; objr
            :: __objc_subclass_implpart_class_ro!
            ($objcname, CLASS_RO, ClassRoT, CLASS_FLAGS, $payload, CLASS_NAME,
             & IVAR_LIST.0, std :: ptr :: null()) ; objr ::
            __objc_subclass_implpart_finalize!
            ($pub, $identifier, $objcname, $superclass, CLASST, CLASS_RO,
             NSSUPER_CLASS, OBJC_EMPTY_CACHE) ; objr ::
            __objc_subclass_impl_payload_access!
            ($pub, $identifier, $payload, FRAGILE_BASE_CLASS_OFFSET) ;
        }
    }
    #[macro_export]
    #[doc(hidden)]
    macro_rules! __objc_subclass_impl_no_payload_no_methods {
        ($pub : vis, $identifier : ident, $objcname : ident, $superclass :
         ident) =>
        {
            objr :: __objc_subclass_implpart_a!
            ($pub, $identifier, $objcname, $superclass, IvarListT, ClassRoT,
             CLASS_NAME, CLASS_FLAGS, METACLASS_FLAGS, CLASST, NSSUPER_CLASS,
             OBJC_EMPTY_CACHE) ; objr :: __objc_subclass_implpart_class_ro!
            ($objcname, CLASS_RO, ClassRoT, CLASS_FLAGS, (), CLASS_NAME, std
             :: ptr :: null(), std :: ptr :: null()) ; objr ::
            __objc_subclass_implpart_finalize!
            ($pub, $identifier, $objcname, $superclass, CLASST, CLASS_RO,
             NSSUPER_CLASS, OBJC_EMPTY_CACHE) ;
        }
    }
    #[macro_export]
    #[doc(hidden)]
    macro_rules! __objc_subclass_impl_no_payload_with_methods {
        ($pub : vis, $identifier : ident, $objcname : ident, $superclass :
         ident, [$($objcmethod : literal => $methodfn : expr $(,) *) +]) =>
        {
            objr :: __objc_subclass_implpart_a!
            ($pub, $identifier, $objcname, $superclass, IvarListT, ClassRoT,
             CLASS_NAME, CLASS_FLAGS, METACLASS_FLAGS, CLASST, NSSUPER_CLASS,
             OBJC_EMPTY_CACHE) ; objr :: __objc_subclass_implpart_method_list!
            ($objcname, [$($objcmethod, $methodfn), *], METHOD_LIST) ; objr ::
            __objc_subclass_implpart_class_ro!
            ($objcname, CLASS_RO, ClassRoT, CLASS_FLAGS, (), CLASS_NAME, std
             :: ptr :: null(), unsafe
             { std :: mem :: transmute(& METHOD_LIST.0) }) ; objr ::
            __objc_subclass_implpart_finalize!
            ($pub, $identifier, $objcname, $superclass, CLASST, CLASS_RO,
             NSSUPER_CLASS, OBJC_EMPTY_CACHE) ;
        }
    }
    ///Variant with payload and methods
    #[macro_export]
    #[doc(hidden)]
    macro_rules! __objc_subclass_impl_with_payload_with_methods {
        ($pub : vis, $identifier : ident, $objcname : ident, $superclass :
         ident, $payload : ty,
         [$($objcmethod : literal => $methodfn : expr $(,) *) +]) =>
        {
            objr :: __objc_subclass_implpart_a!
            ($pub, $identifier, $objcname, $superclass, IvarListT, ClassRoT,
             CLASS_NAME, CLASS_FLAGS, METACLASS_FLAGS, CLASST, NSSUPER_CLASS,
             OBJC_EMPTY_CACHE) ; objr :: __objc_subclass_implpart_ivar_list!
            ($objcname, $payload, FRAGILE_BASE_CLASS_OFFSET, IVAR_LIST) ; objr
            :: __objc_subclass_implpart_method_list!
            ($objcname, [$($objcmethod, $methodfn), *], METHOD_LIST) ; objr ::
            __objc_subclass_implpart_class_ro!
            ($objcname, CLASS_RO, ClassRoT, CLASS_FLAGS, $payload, CLASS_NAME,
             unsafe { std :: mem :: transmute(& IVAR_LIST.0) }, unsafe
             { std :: mem :: transmute(& METHOD_LIST.0) }) ; objr ::
            __objc_subclass_implpart_finalize!
            ($pub, $identifier, $objcname, $superclass, CLASST, CLASS_RO,
             NSSUPER_CLASS, OBJC_EMPTY_CACHE) ; objr ::
            __objc_subclass_impl_payload_access!
            ($pub, $identifier, $payload, FRAGILE_BASE_CLASS_OFFSET) ;
        }
    }
    ///Declares an objc subclass.
    /// ```rust
    /// use objr::objc_subclass;
    /// objc_subclass! {
    ///     //Declare a Rust type named `Example`, which maps to the underlying objc class
    ///     pub struct Example {
    ///         //In the ObjC runtime, our type will be named `Example`
    ///         @class(Example)
    ///         //And will have `NSNull` as its superclass
    ///         @superclass(NSNull)
    ///         //Do not allocate any ivar storage for the class
    ///         payload: (),
    ///         methods: []
    ///     }
    /// }
    /// ```
    ///
    /// # Methods
    ///
    /// To declare a method on the subclass, use a syntax like
    /// ```ignore
    /// methods = [
    ///             "-(void) mySelector" => unsafe myRustFunction
    /// ]
    /// ```
    ///
    /// Where the left part is an ObjC declaration and the right part is a Rust function.  Couple of notes:
    ///
    /// 1.  Rust function must be `extern "C"`.  Failing to do this is UB.
    /// 2.  The first two arguments to the Rust function are the pointer to Self, and the selector.
    ///     (arguments that are repr-transparent to these are OK as well).
    /// 3.  All arguments and return values must be FFI-safe.
    ///
    /// Here's a simple example
    /// ```
    /// use objr::bindings::*;
    /// extern "C" fn example(objcSelf: Example, //repr-transparent to the pointer type
    ///                     sel: Sel) {
    ///     println!("Hello from rustdoc!");
    /// }
    /// objc_subclass! {
    ///     pub struct Example {
    ///         @class(Example)
    ///         @superclass(NSObject)
    ///         payload: (),
    ///         methods: [ "-(void) example" => unsafe example ]
    ///     }
    /// }
    /// ```
    ///
    /// ## Returning values
    ///
    /// In general, if you're implementing a method of +1 (that is, retain/strong) convention, you need to return a retained value.
    /// This means you must use [std::mem::forget] on a StrongCell.
    ///
    /// Alternatively, if you're implementing a method of +0 (that is, autorelease) convention, you need to return an autoreleased value.
    /// While you can create an [objr::bindings::AutoreleasedCell] yourself, the best strategy is usually to return [objr::bindings::StrongCell::return_autoreleased()].
    ///
    /// ## Dealloc
    ///
    /// You can supply an implementation of dealloc in order to roll your own 'drop' behavior.
    ///
    /// Note that unlike "modern ARC" objc, you must chain to `[super dealloc]`.
    ///
    /// ### `.cxx_destruct`
    ///
    /// A real objc compiler uses a different strategy for the compiler generated deinitializer than `deinit`.  When
    /// the you create an objc class with `id` (e.g., strong) payloads, the compiler synthesizes a `.cxx_destruct`
    /// selector and uses special runtime flags to indicate this selector should be called.  This allows
    /// compiler synthesis to co-exist with a user-written `deinit`.
    ///
    /// This is not currently supported by the macro but may be added in the future.
    ///
    /// ## Arguments
    /// The first argument to your C function is a pointer to `self`, and the second argument is a selector-pointer.
    /// You may use any memory-compatible types for these arguments in Rust.  For example, the self argument can be
    /// * `*const c_void` or `*mut c_void`.
    /// * `*const Example` or `*mut Example` (it's memory-compatible with the `*const c_void`).  Convenience functions are implemented
    ///   on the wrapper type so this may be the useful one.  Keep in mind that it's up to you to not mutate from an immutable context.
    ///   For more info, see [objc_instance!#safety]
    ///
    /// For the selector argument, typically you use `Sel`.  `*const c_void` and `*const c_char` are also allowed.
    ///
    /// # Payloads
    /// Your ObjC type may have its own storage, inside the object.  This obviates the need
    /// to allocate any external storage or somehow map between Rust and ObjC memory.
    ///
    /// Currently, a single field is supported.  However, this field can be a Rust struct.
    /// Payloads may also be 0-sized, for example `()` may be used.
    ///
    /// To specify a payload, you use one of the following "payload specifiers"
    ///
    /// ## `()`
    /// Indicates a zero-sized payload.
    ///
    /// Note that there is a subtle difference between using the tokens `()` and specifying a payload of 0-size (ex, `unsafe ininitialized nondrop ()`).
    /// In the former case, we emit no payload to objc.  In the latter case, we emit storage of 0 size.  The `()` syntax is preferred.
    ///
    /// ## `unsafe uninitialized nondrop T`
    ///
    /// Storage for type T will be created.  This is
    /// * uninitialized.  It is UB to read this before initialization.  Presumably, you need to write an objc `init` method and ensure it is called.
    ///   If you somehow read this memory without initialization, this is UB.
    /// * nondrop.  Drop will never be called on this type
    /// * `unsafe`, no memory management is performed.
    ///
    ///
    /// ```
    /// use objr::bindings::*;
    /// objc_subclass! {
    ///     //Declare a Rust type named `Example`, which maps to the underlying objc class
    ///     pub struct Example {
    ///         //In the ObjC runtime, our type will be named `Example`
    ///         @class(Example)
    ///         //And will have `NSNull` as its superclass
    ///         @superclass(NSNull)
    ///         //The following storage will be allocated.  See the payload section.
    ///         payload: unsafe uninitialized nondrop u8,
    ///         methods: ["-(id) init" => unsafe init]
    ///     }
    /// }
    ///
    ///     extern "C" fn init(objcSelf: *mut Example, sel: Sel) -> *const Example {
    ///         let new_self: &Example = unsafe{ &*(Example::perform_super(objcSelf,  Sel::init(), &ActiveAutoreleasePool::assume_autoreleasepool(), ()))};
    ///         //initialize the payload to 5
    ///         *(unsafe{new_self.payload_mut()}) = 5;
    ///         //return self per objc convention
    ///         new_self
    ///     }
    ///```
    /// ### Payload memory management
    /// One thing to keep in mind is that in general, memory management is significantly
    /// different in ObjC and most Rust patterns simply do not work.
    ///
    /// Suppose you try to have a `struct Payload<'a> {&'a Type}` payload.  A few issues with this:
    ///
    /// 1.  Currently, Rust does not understand that `Payload` is inside `Example`.  Therefore,
    ///     the borrowchecker does not check that `'a` is valid for the lifetime of `Example`.
    ///
    /// 2.  Even if this worked, in practice ObjC types are usually donated to the runtime
    ///     either explicitly or implicitly.  The extent of this is not necessarily documented
    ///     by ObjC people.  For example, in `https://lapcatsoftware.com/articles/working-without-a-nib-part-12.html`
    ///     it's discussed that `NSWindow` effectively had its lifetime extended in an SDK
    ///     release, with little in the way of documentation (in fact, I can only find discussion
    ///     of it there).  In practice, this "just happens" in ObjC.
    ///
    ///     Therefore, your options are generally some combination of:
    ///
    ///     1.  Store `'static` data only
    ///     2.  Use `StrongCell` for ObjC types.  This is simlar to what ObjC does internally anyway.
    ///     3.  Use `Rc` or similar for Rust data.
    ///     4.  I'm not gonna be the safety police and tell you not to use raw pointers,
    ///         but you are on your own as far as the unbounded lifetimes of ObjC objects.
    ///
    /// Keep in mind that for several of these, you need to implement your own dealloc that calls drop.
    ///
    /// ### Coda on init
    ///
    /// The payload is born in an uninitialized state, which means any use of it is undefined.  Obviously,
    /// you need to init it in some initializer.
    ///
    /// Less obviously, it is tricky to init it correctly.  For example, you assign to the payload, you may
    /// drop the "prior" (uninitialized) value, which is UB.
    ///
    /// In theory, [std::mem::MaybeUninit] would solve this – assuming you remember to wrap all your values (or the payload itself).
    /// In practice however, [std::mem::MaybeUnint.assume_init()] requires moving the value outside the payload,
    /// which cannot really be done in this case.  See `https://github.com/rust-lang/rust/issues/63568` for details.
    ///
    /// The alternative is to write into your payload_mut with [std::ptr::write], which does not drop the uninitialized value.
    ///
    #[macro_export]
    macro_rules! objc_subclass {
        ($pub : vis struct $identifier : ident
         {
             @ class($objcname : ident) @ superclass($superclass : ident)
             payload : unsafe uninitialized nondrop $payload : ty, methods :
             []
         }) =>
        {
            objr :: __objc_subclass_impl_with_payload_no_methods!
            ($pub, $identifier, $objcname, $superclass, $payload) ;
        } ;
        ($pub : vis struct $identifier : ident
         {
             @ class($objcname : ident) @ superclass($superclass : ident)
             payload : (), methods : []
         }) =>
        {
            objr :: __objc_subclass_impl_no_payload_no_methods!
            ($pub, $identifier, $objcname, $superclass) ;
        } ;
        ($pub : vis struct $identifier : ident
         {
             @ class($objcname : ident) @ superclass($superclass : ident)
             payload : (), methods :
             [$($objcmethod : literal => unsafe $methodfn : expr $(,) ?) +]
         }) =>
        {
            objr :: __objc_subclass_impl_no_payload_with_methods!
            ($pub, $identifier, $objcname, $superclass,
             [$($objcmethod => $methodfn) *]) ;
        } ;
        ($pub : vis struct $identifier : ident
         {
             @ class($objcname : ident) @ superclass($superclass : ident)
             payload : unsafe uninitialized nondrop $payload : ty, methods :
             [$($objcmethod : literal => unsafe $methodfn : expr $(,) ?) +]
         }) =>
        {
            objr :: __objc_subclass_impl_with_payload_with_methods!
            ($pub, $identifier, $objcname, $superclass, $payload,
             [$($objcmethod => $methodfn) *]) ;
        } ;
    }
}
mod exception {
    ///! Support for objc exceptions.
    use std::ffi::c_void;
    ///Declared in hard-exception.m and compiled with build.rs
    extern "C" {
        fn hard_exception(call: extern "C" fn(*mut c_void),
                          context: *mut c_void);
    }
    extern "C" fn thunk_void<F: FnOnce()>(context: &mut Option<F>)
     -> *mut c_void {
        loop { }
    }
    ///This function catches an objc exception raised in the closure.
    ///
    /// Return values are not supported, this is primarily intended to facilitate debugging.
    pub fn try_unwrap_void<F: FnOnce()>(closure: F) { loop { } }
}
///This prelude provides a "foundation-like" experience.  This brings
/// in various foundation types, like NSObject, NSString, etc.
///
/// In this crate we generally only implement types that are strictly necessary,
/// for other foundation types see other crates.
mod foundation {
    pub use super::nsstring::NSString;
    pub use super::nsobject::NSObject;
    pub use super::nsobject::NSObjectTrait;
    pub use super::nsobject::NSObjectSelectors;
    pub use super::class::ObjcClass;
    pub use super::nserror::NSError;
}
///This namespace includes items that are appropriate for writing bindings
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
    pub use crate::objc_subclass;
    pub use crate::objc_instance_newtype;
    pub use crate::objc_class_newtype;
    pub use procmacro::{__objc_implement_class, ObjcInstance, __static_expr,
                        __static_extern, __static_asciiz_ident_as_selector,
                        __static_asciiz_ident_as_type_encoding, __count,
                        __concat_idents, __static_asciiz, __static_expr3};
    pub use super::class::AnyClass;
    pub use super::arguments::{Primitive, Arguable};
    pub use super::exception::{try_unwrap_void};
    pub use super::objcinstance::ObjcInstanceBehavior;
    ///Used by macros, not public API
    #[doc(hidden)]
    pub use super::sel::_SyncWrapper;
    #[doc(hidden)]
    pub use procmacro::{_objc_selector_decl, _objc_selector_impl, __use,
                        __mod};
}
mod private {
    ///"Sealed trait" pattern.  Traits will inherit from this trait to indicate they cannot be implemented outside the crate.
    ///
    /// See [the documentation](https://rust-lang.github.io/api-guidelines/future-proofing.html) for details.
    ///
    /// We are free to implement `Sealed` on any type from inside the crate; this is often necessary to implement some other `Trait: Sealed` on the type.
    pub trait Sealed { }
}
