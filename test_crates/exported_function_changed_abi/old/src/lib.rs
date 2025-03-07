#![no_std]

#[no_mangle]
pub fn public_null_export_name_func_will_become_c() {}

#[no_mangle]
fn private_null_export_name_func() {}

#[no_mangle]
extern "C" fn no_name_export_func() {}

#[no_mangle]
extern "C" fn exported_c_func_that_will_become_c_unwind() {}

#[no_mangle]
extern "C-unwind" fn exported_c_unwind_func_that_will_become_c() {}

#[no_mangle]
extern "Rust" fn exported_rust_func_that_will_become_c() {}

#[no_mangle]
extern "Rust" fn exported_rust_func_that_will_become_c_unwind() {}

#[export_name = "different_exported_name"]
extern "Rust" fn actual_function_name_that_will_become_c() {}

#[export_name = "different_exported_name2"]
extern "Rust" fn actual_function_name_that_will_become_c_unwind() {}
