#[allow(dead_code)]
pub mod wasi_mindmap {
    #[allow(dead_code)]
    pub mod interfaced_adder {
        #[allow(dead_code, clippy::all)]
        pub mod add {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() = super::super::super::__link_custom_section_describing_imports;
            use super::super::super::_rt;
            #[allow(unused_unsafe, clippy::all)]
            pub fn add(a: i32, b: i32) -> i32 {
                unsafe {
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "wasi-mindmap:interfaced-adder/add")]
                    extern "C" {
                        #[link_name = "add"]
                        fn wit_import(_: i32, _: i32) -> i32;
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: i32, _: i32) -> i32 {
                        unreachable!()
                    }
                    let ret = wit_import(_rt::as_i32(&a), _rt::as_i32(&b));
                    ret
                }
            }
        }
    }
}
mod _rt {
    pub fn as_i32<T: AsI32>(t: T) -> i32 {
        t.as_i32()
    }
    pub trait AsI32 {
        fn as_i32(self) -> i32;
    }
    impl<'a, T: Copy + AsI32> AsI32 for &'a T {
        fn as_i32(self) -> i32 {
            (*self).as_i32()
        }
    }
    impl AsI32 for i32 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for u32 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for i16 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for u16 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for i8 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for u8 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for char {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for usize {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
}
#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:wit-bindgen:0.35.0:wasi-mindmap:host:host:encoded world"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 215] = *b"\
\0asm\x0d\0\x01\0\0\x19\x16wit-component-encoding\x04\0\x07]\x01A\x02\x01A\x02\x01\
B\x02\x01@\x02\x01az\x01bz\0z\x04\0\x03add\x01\0\x03\0!wasi-mindmap:interfaced-a\
dder/add\x05\0\x04\0\x16wasi-mindmap:host/host\x04\0\x0b\x0a\x01\0\x04host\x03\0\
\0\0G\x09producers\x01\x0cprocessed-by\x02\x0dwit-component\x070.220.0\x10wit-bi\
ndgen-rust\x060.35.0";
#[inline(never)]
#[doc(hidden)]
pub fn __link_custom_section_describing_imports() {
    wit_bindgen_rt::maybe_link_cabi_realloc();
}
