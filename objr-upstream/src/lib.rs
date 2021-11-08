#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2018::*;
#[macro_use]
extern crate std;
extern crate self as objr;

pub struct objc_enum;

pub struct objc_class;
pub struct objc_class_newtype;
mod class {
    pub struct AnyClass;
    pub struct ObjcClass;
    pub struct Class;
}

mod objectpointers {
    pub struct StrongCell<T>(T);
    pub struct StrongMutCell<T>(T);
    pub struct StrongLifetimeCell<'a,T>(&'a T);
    pub struct AutoreleasedCell<'a,T>(&'a T);
    pub struct AutoreleasedMutCell<'a,T>(&'a T);
}

// #[cfg(FALSE)]
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

// #[cfg(FALSE)]
mod objcinstance {
    use std::ptr::NonNull;
    use crate::bindings::{StrongCell, AutoreleasedCell, StrongLifetimeCell,
                          StrongMutCell};
    use crate::autorelease::ActiveAutoreleasePool;
    pub trait ObjcInstance { }
    #[repr(transparent)]
    pub struct NonNullImmutable<T: ?Sized>(NonNull<T>);
    impl <T: ObjcInstance> NonNullImmutable<T> {
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
    pub trait NullableCellBehavior {
        type T: ObjcInstance;
        unsafe fn assume_mut(self)
        -> Option<StrongMutCell<Self::T>>;
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

pub struct objc_selector_group;
mod sel {
    pub struct Sel;
    pub struct _SyncWrapper;
}

mod nserror {
    pub struct NSError;
}

mod exception {
    pub struct try_unwrap_void;
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
