/* automatically generated by rust-bindgen */


#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]


#[repr(C)]
#[derive(Copy)]
pub struct S {
    pub large_array: [::std::os::raw::c_char; 33usize],
}
#[test]
fn bindgen_test_layout_S() {
    assert_eq!(::std::mem::size_of::<S>() , 33usize , concat ! (
               "Size of: " , stringify ! ( S ) ));
    assert_eq! (::std::mem::align_of::<S>() , 1usize , concat ! (
                "Alignment of " , stringify ! ( S ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const S ) ) . large_array as * const _ as usize
                } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( S ) , "::" , stringify
                ! ( large_array ) ));
}
impl Clone for S {
    fn clone(&self) -> Self { *self }
}
impl Default for S {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
pub struct ST<T> {
    pub large_array: [T; 33usize],
    pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<T>>,
}
impl <T> Default for ST<T> {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
