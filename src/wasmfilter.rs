use std::ffi::CString;
use std::ffi::c_void;
use std::os::raw::c_char;

use rmp;
use rmpv::Value as MsgpackValue;

// tell rustc not to change/mangle the name of this function, as FluentBit
// needs to be able to find it by name
#[no_mangle]
pub extern "C" fn hello_world__json(
    tag: *const c_char,    // uint_32 from fluentbit, presumably "wasm-address-space" 32-bit pointer
    tag_len: usize,        // size_t from fluentbit - so possibly downcast??
    tim_sec: u32,          // long int from time.h, presumably downcast?
    time_ncsec: u32,       // long int from time.h, presumably downcast?
    record: *const c_char, // uint_32 from fluentbit, presumably "wasm-address-space" 32-bit pointer
    record_len: usize      // size_t from fluentbit - so possibly downcast??
) -> *const c_char {
    let hello_world_cstr = CString::new(
            "{\"msg\":\"Hello world from rust wasm! 🙂\"}"
        )
        // CString provides a guarantee that the bytes behind the string have
        // no nul bytes in them other than one at the end. An unwrap is
        // required to get to the result success inner type (seems this can't
        // be done statically)
        .unwrap();
    // Fluentbit expects a pointer to a C string to be returned from the filter
    // execution. "Leak" a pointer to the string so that Rust does not generate
    // code to "drop"/deallocate the string - it needs to still be valid and
    // in the heap of the WebAssembly execution context for Fluentbit to pull
    // it out once the filter function execution is complete.
    return Box::leak(hello_world_cstr.into_boxed_c_str()).as_ptr();
}

#[no_mangle]
pub extern "C" fn hello_world__msgpack(
    tag: *const c_char,    // uint_32 from fluentbit, presumably "wasm-address-space" 32-bit pointer
    tag_len: usize,        // size_t from fluentbit - so possibly downcast??
    tim_sec: u32,          // long int from time.h, presumably downcast?
    time_ncsec: u32,       // long int from time.h, presumably downcast?
    record: *const c_char, // uint_32 from fluentbit, presumably "wasm-address-space" 32-bit pointer
    record_len: usize      // size_t from fluentbit - so possibly downcast??
    ) -> *const c_char {
    let hello_world_msgpack = MsgpackValue::Map(
        vec![
            (
                MsgpackValue::String("msg".to_string().into()),
                MsgpackValue::String("Hello world from rust wasm! In msgpack!".into())
            )
        ]
    );

    // The Msgpack Value type still needs to be encoded down to actual
    // messagepack bytes for output.

    let mut output_buf = Vec::new();

    match rmpv::encode::write_value(&mut output_buf, &hello_world_msgpack) {
        Ok(()) => return output_buf
            // as above, the buffer must be "leaked" so that its allocation
            // lives beyond the scope of this function so Fluentbit can read
            // out the value after the filter function execution is complete
            .leak()
            .as_ptr() as *const c_char,
        Err(e) => return "bzt".as_ptr() as *const c_char
    }
}