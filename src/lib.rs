#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    #[test]
    fn library_version() {
        let cstr = unsafe { std::ffi::CStr::from_ptr(super::jsonnet_version()) };
        assert_eq!(
            cstr.to_str().unwrap(),
            format!(
                "{} (go-jsonnet)",
                std::ffi::CStr::from_bytes_with_nul(super::LIB_JSONNET_VERSION)
                    .unwrap()
                    .to_str()
                    .unwrap()
            ),
        );
    }

    #[test]
    fn evaluate_snippet() {
        let filename = std::ffi::CString::new("evaluate_snippet.jsonnet").unwrap();
        let code = std::ffi::CString::new("{foo: 1+2, bar: std.isBoolean(false)}").unwrap();
        let result = unsafe {
            let vm = super::jsonnet_make();
            let mut err = 0;
            let result_ptr =
                super::jsonnet_evaluate_snippet(vm, filename.as_ptr(), code.as_ptr(), &mut err);
            let result = std::ffi::CStr::from_ptr(result_ptr)
                .to_string_lossy()
                .into_owned();
            super::jsonnet_realloc(vm, result_ptr, 0);
            super::jsonnet_destroy(vm);
            if err == 0 {
                Ok(result)
            } else {
                Err(result)
            }
        };
        assert!(result.is_ok());
        #[derive(Debug, PartialEq, serde::Deserialize)]
        struct S {
            foo: i32,
            bar: bool,
        }
        let s: S = serde_json::from_str(&result.unwrap()).unwrap();
        assert_eq!(s, S { foo: 3, bar: true });
    }

    #[test]
    fn evaluate_snippet_syntax_error() {
        let filename = std::ffi::CString::new("evaluate_snippet.jsonnet").unwrap();
        let code = std::ffi::CString::new("{foo: 1+}").unwrap();
        let result = unsafe {
            let vm = super::jsonnet_make();
            let mut err = 0;
            let result_ptr =
                super::jsonnet_evaluate_snippet(vm, filename.as_ptr(), code.as_ptr(), &mut err);
            let result = std::ffi::CStr::from_ptr(result_ptr)
                .to_string_lossy()
                .into_owned();
            super::jsonnet_realloc(vm, result_ptr, 0);
            super::jsonnet_destroy(vm);
            if err == 0 {
                Ok(result)
            } else {
                Err(result)
            }
        };
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .starts_with("evaluate_snippet.jsonnet:1:"));
    }

    #[test]
    fn native_callback() {
        let filename = std::ffi::CString::new("native_callback.jsonnet").unwrap();
        let code = std::ffi::CString::new(
            r#"
            local hello = std.native("hello");
            { message: hello("world") }
        "#,
        )
        .unwrap();
        let name = std::ffi::CString::new("hello").unwrap();
        unsafe extern "C" fn callback(
            ctx: *mut std::ffi::c_void,
            argv: *const *const super::JsonnetJsonValue,
            success: *mut i32,
        ) -> *mut super::JsonnetJsonValue {
            let vm = ctx as *mut super::JsonnetVm;
            let arg_c =
                super::jsonnet_json_extract_string(vm, *argv as *mut super::JsonnetJsonValue);
            let arg = std::ffi::CStr::from_ptr(arg_c).to_str().unwrap();
            let message = std::ffi::CString::new(format!("hello {}", arg)).unwrap();
            let s = super::jsonnet_json_make_string(vm, message.as_ptr());
            *success = 1;
            s
        }
        let arg = std::ffi::CString::new("s").unwrap();
        let params = vec![arg.as_ptr(), std::ptr::null()];
        let result = unsafe {
            let vm = super::jsonnet_make();
            super::jsonnet_native_callback(
                vm,
                name.as_ptr(),
                Some(callback),
                vm as *mut std::ffi::c_void,
                params.as_ptr(),
            );
            let mut err = 0;
            let result_ptr =
                super::jsonnet_evaluate_snippet(vm, filename.as_ptr(), code.as_ptr(), &mut err);
            let result = std::ffi::CStr::from_ptr(result_ptr)
                .to_string_lossy()
                .into_owned();
            super::jsonnet_realloc(vm, result_ptr, 0);
            super::jsonnet_destroy(vm);
            if err == 0 {
                Ok(result)
            } else {
                Err(result)
            }
        };
        assert!(result.is_ok());
        #[derive(Debug, PartialEq, serde::Deserialize)]
        struct S {
            message: String,
        }
        let s: S = serde_json::from_str(&result.unwrap()).unwrap();
        assert_eq!(
            s,
            S {
                message: "hello world".to_owned()
            }
        );
    }
}
