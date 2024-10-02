// Generated by `wit-bindgen` 0.25.0. DO NOT EDIT!
// Options used:
#[allow(dead_code)]
pub mod docs {
    #[allow(dead_code)]
    pub mod adder {
        #[allow(dead_code, clippy::all)]
        pub mod add {
            #[used]
            #[doc(hidden)]
            #[cfg(target_arch = "wasm32")]
            static __FORCE_SECTION_REF: fn() =
                super::super::super::__link_custom_section_describing_imports;
            use super::super::super::_rt;
            #[allow(unused_unsafe, clippy::all)]
            pub fn add(a: f32, b: f32) -> f32 {
                unsafe {
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "docs:adder/add@0.1.0")]
                    extern "C" {
                        #[link_name = "add"]
                        fn wit_import(_: f32, _: f32) -> f32;
                    }

                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: f32, _: f32) -> f32 {
                        unreachable!()
                    }
                    let ret = wit_import(_rt::as_f32(&a), _rt::as_f32(&b));
                    ret
                }
            }
        }
    }
    #[allow(dead_code)]
    pub mod subtractor {
        #[allow(dead_code, clippy::all)]
        pub mod subtract {
            #[used]
            #[doc(hidden)]
            #[cfg(target_arch = "wasm32")]
            static __FORCE_SECTION_REF: fn() =
                super::super::super::__link_custom_section_describing_imports;
            use super::super::super::_rt;
            #[allow(unused_unsafe, clippy::all)]
            pub fn subtract(a: f32, b: f32) -> f32 {
                unsafe {
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "docs:subtractor/subtract@0.1.0")]
                    extern "C" {
                        #[link_name = "subtract"]
                        fn wit_import(_: f32, _: f32) -> f32;
                    }

                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: f32, _: f32) -> f32 {
                        unreachable!()
                    }
                    let ret = wit_import(_rt::as_f32(&a), _rt::as_f32(&b));
                    ret
                }
            }
        }
    }
}
#[allow(dead_code)]
pub mod exports {
    #[allow(dead_code)]
    pub mod docs {
        #[allow(dead_code)]
        pub mod calculator {
            #[allow(dead_code, clippy::all)]
            pub mod calculate {
                #[used]
                #[doc(hidden)]
                #[cfg(target_arch = "wasm32")]
                static __FORCE_SECTION_REF: fn() =
                    super::super::super::super::__link_custom_section_describing_imports;
                use super::super::super::super::_rt;
                #[repr(u8)]
                #[derive(Clone, Copy, Eq, PartialEq)]
                pub enum Op {
                    Add,
                    Subtract,
                    InterestRate,
                }
                impl ::core::fmt::Debug for Op {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                        match self {
                            Op::Add => f.debug_tuple("Op::Add").finish(),
                            Op::Subtract => f.debug_tuple("Op::Subtract").finish(),
                            Op::InterestRate => f.debug_tuple("Op::InterestRate").finish(),
                        }
                    }
                }

                impl Op {
                    #[doc(hidden)]
                    pub unsafe fn _lift(val: u8) -> Op {
                        if !cfg!(debug_assertions) {
                            return ::core::mem::transmute(val);
                        }

                        match val {
                            0 => Op::Add,
                            1 => Op::Subtract,
                            2 => Op::InterestRate,

                            _ => panic!("invalid enum discriminant"),
                        }
                    }
                }

                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_eval_expression_cabi<T: Guest>(
                    arg0: i32,
                    arg1: f32,
                    arg2: f32,
                ) -> f32 {
                    #[cfg(target_arch = "wasm32")]
                    _rt::run_ctors_once();
                    let result0 = T::eval_expression(Op::_lift(arg0 as u8), arg1, arg2);
                    _rt::as_f32(result0)
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_eval_expression_three_args_cabi<T: Guest>(
                    arg0: i32,
                    arg1: f32,
                    arg2: i32,
                    arg3: i32,
                ) -> f32 {
                    #[cfg(target_arch = "wasm32")]
                    _rt::run_ctors_once();
                    let result0 = T::eval_expression_three_args(
                        Op::_lift(arg0 as u8),
                        arg1,
                        arg2 as u32,
                        arg3 as u32,
                    );
                    _rt::as_f32(result0)
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_total_payable_cabi<T: Guest>(
                    arg0: f32,
                    arg1: i32,
                    arg2: i32,
                ) -> f32 {
                    #[cfg(target_arch = "wasm32")]
                    _rt::run_ctors_once();
                    let result0 = T::total_payable(arg0, arg1 as u32, arg2 as u32);
                    _rt::as_f32(result0)
                }
                pub trait Guest {
                    fn eval_expression(op: Op, x: f32, y: f32) -> f32;
                    fn eval_expression_three_args(op: Op, x: f32, y: u32, z: u32) -> f32;
                    fn total_payable(rate: f32, amount: u32, years: u32) -> f32;
                }
                #[doc(hidden)]

                macro_rules! __export_docs_calculator_calculate_0_1_0_cabi{
    ($ty:ident with_types_in $($path_to_types:tt)*) => (const _: () = {

      #[export_name = "docs:calculator/calculate@0.1.0#eval-expression"]
      unsafe extern "C" fn export_eval_expression(arg0: i32,arg1: f32,arg2: f32,) -> f32 {
        $($path_to_types)*::_export_eval_expression_cabi::<$ty>(arg0, arg1, arg2)
      }
      #[export_name = "docs:calculator/calculate@0.1.0#eval-expression-three-args"]
      unsafe extern "C" fn export_eval_expression_three_args(arg0: i32,arg1: f32,arg2: i32,arg3: i32,) -> f32 {
        $($path_to_types)*::_export_eval_expression_three_args_cabi::<$ty>(arg0, arg1, arg2, arg3)
      }
      #[export_name = "docs:calculator/calculate@0.1.0#total-payable"]
      unsafe extern "C" fn export_total_payable(arg0: f32,arg1: i32,arg2: i32,) -> f32 {
        $($path_to_types)*::_export_total_payable_cabi::<$ty>(arg0, arg1, arg2)
      }
    };);
  }
                #[doc(hidden)]
                pub(crate) use __export_docs_calculator_calculate_0_1_0_cabi;
            }
        }
    }
}
mod _rt {

    pub fn as_f32<T: AsF32>(t: T) -> f32 {
        t.as_f32()
    }

    pub trait AsF32 {
        fn as_f32(self) -> f32;
    }

    impl<'a, T: Copy + AsF32> AsF32 for &'a T {
        fn as_f32(self) -> f32 {
            (*self).as_f32()
        }
    }

    impl AsF32 for f32 {
        #[inline]
        fn as_f32(self) -> f32 {
            self as f32
        }
    }

    #[cfg(target_arch = "wasm32")]
    pub fn run_ctors_once() {
        wit_bindgen_rt::run_ctors_once();
    }
}

/// Generates `#[no_mangle]` functions to export the specified type as the
/// root implementation of all generated traits.
///
/// For more information see the documentation of `wit_bindgen::generate!`.
///
/// ```rust
/// # macro_rules! export{ ($($t:tt)*) => (); }
/// # trait Guest {}
/// struct MyType;
///
/// impl Guest for MyType {
///     // ...
/// }
///
/// export!(MyType);
/// ```
#[allow(unused_macros)]
#[doc(hidden)]

macro_rules! __export_calculator_impl {
  ($ty:ident) => (self::export!($ty with_types_in self););
  ($ty:ident with_types_in $($path_to_types_root:tt)*) => (
  $($path_to_types_root)*::exports::docs::calculator::calculate::__export_docs_calculator_calculate_0_1_0_cabi!($ty with_types_in $($path_to_types_root)*::exports::docs::calculator::calculate);
  )
}
#[doc(inline)]
pub(crate) use __export_calculator_impl as export;

#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:wit-bindgen:0.25.0:calculator:encoded world"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 486] = *b"\
\0asm\x0d\0\x01\0\0\x19\x16wit-component-encoding\x04\0\x07\xe5\x02\x01A\x02\x01\
A\x06\x01B\x02\x01@\x02\x01av\x01bv\0v\x04\0\x03add\x01\0\x03\x01\x14docs:adder/\
add@0.1.0\x05\0\x01B\x02\x01@\x02\x01av\x01bv\0v\x04\0\x08subtract\x01\0\x03\x01\
\x1edocs:subtractor/subtract@0.1.0\x05\x01\x01B\x08\x01m\x03\x03add\x08subtract\x0d\
interest-rate\x04\0\x02op\x03\0\0\x01@\x03\x02op\x01\x01xv\x01yv\0v\x04\0\x0feva\
l-expression\x01\x02\x01@\x04\x02op\x01\x01xv\x01yy\x01zy\0v\x04\0\x1aeval-expre\
ssion-three-args\x01\x03\x01@\x03\x04ratev\x06amounty\x05yearsy\0v\x04\0\x0dtota\
l-payable\x01\x04\x04\x01\x1fdocs:calculator/calculate@0.1.0\x05\x02\x04\x01\x20\
docs:calculator/calculator@0.1.0\x04\0\x0b\x10\x01\0\x0acalculator\x03\0\0\0G\x09\
producers\x01\x0cprocessed-by\x02\x0dwit-component\x070.208.1\x10wit-bindgen-rus\
t\x060.25.0";

#[inline(never)]
#[doc(hidden)]
#[cfg(target_arch = "wasm32")]
pub fn __link_custom_section_describing_imports() {
    wit_bindgen_rt::maybe_link_cabi_realloc();
}
