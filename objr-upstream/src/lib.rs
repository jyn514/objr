extern crate self as objr;
pub mod macros;
mod class;

mod objectpointers;

mod nsobject;
mod nsstring;
mod autorelease;
mod arguments;

mod performselector;
mod objcinstance;
mod typealias;
mod sel;
mod nserror;
mod subclass;
mod exception;


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
    pub use super::autorelease::{ActiveAutoreleasePool,AutoreleasePool};
    pub use super::objectpointers::{StrongCell,AutoreleasedCell,StrongMutCell,AutoreleasedMutCell,StrongLifetimeCell};
    pub use super::sel::Sel;
    pub use super::nsobject::NSObjectTrait;
    pub use super::nsobject::NSObject;
    pub use super::objcinstance::{ObjcInstance,OptionalInstanceBehavior,NonNullImmutable,NullableBehavior};
    pub use super::performselector::{PerformsSelector,PerformablePointer};
    pub use super::class::{Class};
    pub use super::foundation::*;
    pub use objr::objcinstance::NullableCellBehavior;
    //import macros
    pub use crate::objc_instance;
    pub use crate::objc_class;
    pub use crate::objc_enum;
    pub use crate::objc_selector_group;
    pub use crate::objc_subclass;
    pub use crate::objc_instance_newtype;
    pub use crate::objc_class_newtype;
    pub use procmacro::{__objc_implement_class,ObjcInstance,__static_expr,__static_extern,__static_asciiz_ident_as_selector,__static_asciiz_ident_as_type_encoding,__count,__concat_idents,__static_asciiz,__static_expr3};
    pub use super::class::AnyClass;
    pub use super::arguments::{Primitive,Arguable};
    pub use super::exception::{try_unwrap_void};
    pub use super::objcinstance::ObjcInstanceBehavior;

    ///Used by macros, not public API
    #[doc(hidden)]
    pub use super::sel::_SyncWrapper;

    //used by macros
    #[doc(hidden)]
    pub use procmacro::{_objc_selector_decl,_objc_selector_impl,__use,__mod};

}

mod private {
    ///"Sealed trait" pattern.  Traits will inherit from this trait to indicate they cannot be implemented outside the crate.
    ///
    /// See [the documentation](https://rust-lang.github.io/api-guidelines/future-proofing.html) for details.
    ///
    /// We are free to implement `Sealed` on any type from inside the crate; this is often necessary to implement some other `Trait: Sealed` on the type.
    pub trait Sealed {}
}







