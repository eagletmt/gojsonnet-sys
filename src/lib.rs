#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    #[test]
    fn library_version() {
        let cstr = unsafe { std::ffi::CStr::from_ptr(super::jsonnet_version()) };
        assert_eq!(cstr.to_str().unwrap(), "v0.17.0 (go-jsonnet)");
    }

    #[test]
    fn evaluate_snippet() {
        let filename = std::ffi::CString::new("evaluate_snippet.jsonnet")
            .unwrap()
            .into_raw();
        let code = std::ffi::CString::new("{foo: 1+2, bar: std.isBoolean(false)}")
            .unwrap()
            .into_raw();
        let result = unsafe {
            let vm = super::jsonnet_make();
            let mut err = 0;
            let result = std::ffi::CStr::from_ptr(super::jsonnet_evaluate_snippet(
                vm, filename, code, &mut err,
            ))
            .to_str()
            .unwrap();
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
        let filename = std::ffi::CString::new("evaluate_snippet.jsonnet")
            .unwrap()
            .into_raw();
        let code = std::ffi::CString::new("{foo: 1+}").unwrap().into_raw();
        let result = unsafe {
            let vm = super::jsonnet_make();
            let mut err = 0;
            let result = std::ffi::CStr::from_ptr(super::jsonnet_evaluate_snippet(
                vm, filename, code, &mut err,
            ))
            .to_str()
            .unwrap();
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
        let filename = std::ffi::CString::new("native_callback.jsonnet")
            .unwrap()
            .into_raw();
        let code = std::ffi::CString::new(
            r#"
            local hello = std.native("hello");
            { message: hello("world") }
        "#,
        )
        .unwrap()
        .into_raw();
        let name = std::ffi::CString::new("hello").unwrap().into_raw();
        unsafe extern "C" fn callback(
            ctx: *mut std::ffi::c_void,
            argv: *const *const super::JsonnetJsonValue,
            success: *mut i32,
        ) -> *mut super::JsonnetJsonValue {
            let vm = ctx as *mut super::JsonnetVm;
            let arg_c =
                super::jsonnet_json_extract_string(vm, *argv as *mut super::JsonnetJsonValue);
            let arg = std::ffi::CStr::from_ptr(arg_c).to_str().unwrap();
            let s = super::jsonnet_json_make_string(
                vm,
                std::ffi::CString::new(format!("hello {}", arg))
                    .unwrap()
                    .into_raw(),
            );
            *success = 1;
            s
        }
        let mut params = vec![
            std::ffi::CString::new("s").unwrap().into_raw(),
            std::ptr::null_mut(),
        ];
        let result = unsafe {
            let vm = super::jsonnet_make();
            super::jsonnet_native_callback(
                vm,
                name,
                Some(callback),
                vm as *mut std::ffi::c_void,
                params.as_mut_ptr(),
            );
            let mut err = 0;
            let result = std::ffi::CStr::from_ptr(super::jsonnet_evaluate_snippet(
                vm, filename, code, &mut err,
            ))
            .to_str()
            .unwrap();
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
