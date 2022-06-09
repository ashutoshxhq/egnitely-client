#[macro_export]
macro_rules! register_egnitely_fn {
    ($func_name:ident) => {
        extern crate libc;
        use libc::c_char;
        use std::ffi::CString;

        #[no_mangle]
        pub extern "C" fn handler(data_pointer: *mut c_char) -> *mut c_char {
            let data = unsafe { CString::from_raw(data_pointer) };
            let data_str = data.into_string().unwrap();
            println!("Recieved In Handler: {}", data_str);

            let json_data: Value = serde_json::from_str(data_str.as_str()).unwrap();

            let rt = tokio::runtime::Runtime::new().unwrap();
            let response = rt.block_on(async { $func_name(json_data).await });

            println!("Returning form Handler: {}", data_str);
            let result_cstr = CString::new(response.to_string()).unwrap();
            result_cstr.into_raw()
        }
    };
}
