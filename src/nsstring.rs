//! Provides NSString
//!
use super::bindings::*;
use std::ffi::{CStr};
use std::os::raw::{c_char};
use crate::objcinstance::NonNullImmutable;
use objr::typealias::NSUInteger;

objc_class! {
	pub struct NSString {
		@class (NSString)
	}
}

objc_selector_group!(
	pub trait NSStringSelectors {
		@selector("UTF8String")
		@selector("initWithBytes:length:encoding:")
	}
	impl NSStringSelectors for Sel {}
);

#[allow(non_upper_case_globals)]
const NSUTF8StringEncoding: NSUInteger = 4;




impl NSString {
	///Converts to a stringslice
	pub fn to_str(&self, pool: &ActiveAutoreleasePool) -> &str {
		unsafe {
			let str_pointer: *const c_char = Self::perform_primitive(self.assume_nonmut_perform(), Sel::UTF8String(), pool, ());
			let msg = CStr::from_ptr(str_pointer);
			msg.to_str().unwrap()
		}
	}
	///Copies the string into foundation storage
	pub fn with_str_copy(str: &str, pool: &ActiveAutoreleasePool) -> StrongCell<NSString> {
		unsafe {
			let instance = Self::class().alloc(pool);
			let bytes = str.as_bytes().as_ptr();
			let len = str.as_bytes().len() as NSUInteger;

			let instance: *const NSString = Self::perform(instance,Sel::initWithBytes_length_encoding(),pool, (bytes,len,NSUTF8StringEncoding));
			//although this method is technically nullable, the fact that the string is already statically known to be utf8
			//suggests we should be fine
			NonNullImmutable::assume_nonnil(instance).assume_retained()
		}
	}
}



#[test] fn from_str() {
	use crate::autorelease::AutoreleasePool;
	let example = "example string here";
	let pool = unsafe{ AutoreleasePool::new() };
	let nsstring = NSString::with_str_copy(example, &pool);
	assert_eq!(nsstring.to_str(&pool), example);
}

#[test] fn static_str() {
	use crate::autorelease::AutoreleasePool;
	let pool = unsafe{ AutoreleasePool::new() };

	let test = objc_nsstring!("My example literal");
	let description = test.description(&pool);
	assert_eq!(description.to_str(&pool), "My example literal");
}