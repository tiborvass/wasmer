// Rust test file autogenerated with cargo build (src/build_spectests.rs).
// Please do NOT modify it by hand, as it will be reseted on next build.
// Test based on spectests/start.wast
#![allow(
    warnings,
    dead_code
)]
use crate::webassembly::{instantiate, compile, ImportObject, ResultObject, VmCtx, Export};
use super::_common::spectest_importobject;
use wabt::wat2wasm;


// Line 2
#[test]
fn c0_l2_assert_invalid() {
    let wasm_binary = [0, 97, 115, 109, 1, 0, 0, 0, 1, 4, 1, 96, 0, 0, 3, 2, 1, 0, 8, 1, 1, 10, 4, 1, 2, 0, 11];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is invalid");
}

// Line 7
#[test]
fn c1_l7_assert_invalid() {
    let wasm_binary = [0, 97, 115, 109, 1, 0, 0, 0, 1, 5, 1, 96, 0, 1, 127, 3, 2, 1, 0, 8, 1, 0, 10, 7, 1, 5, 0, 65, 0, 15, 11];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is invalid");
}

// Line 14
#[test]
fn c2_l14_assert_invalid() {
    let wasm_binary = [0, 97, 115, 109, 1, 0, 0, 0, 1, 5, 1, 96, 1, 127, 0, 3, 2, 1, 0, 8, 1, 0, 10, 4, 1, 2, 0, 11];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is invalid");
}

// Line 21
fn create_module_1() -> ResultObject {
    let module_str = "(module
      (type (;0;) (func))
      (type (;1;) (func (result i32)))
      (func (;0;) (type 0)
        i32.const 0
        i32.const 0
        i32.load8_u
        i32.const 1
        i32.add
        i32.store8)
      (func (;1;) (type 1) (result i32)
        i32.const 0
        i32.load8_u
        return)
      (func (;2;) (type 0)
        call 0
        call 0
        call 0)
      (memory (;0;) 1 1)
      (export \"inc\" (func 0))
      (export \"get\" (func 1))
      (start 2)
      (data (i32.const 0) \"A\"))
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    instantiate(wasm_binary, spectest_importobject()).expect("WASM can't be instantiated")
}
fn start_module_1(result_object: &ResultObject, vm_context: &VmCtx) {
    result_object.instance.start(&vm_context);
}

// Line 45
fn c4_l45_action_invoke(result_object: &ResultObject, vm_context: &VmCtx) {
    println!("Executing function {}", "c4_l45_action_invoke");
    let func_index = match result_object.module.info.exports.get("get") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(&VmCtx) -> i32 = get_instance_function!(result_object.instance, func_index);
    let result = invoke_fn(&vm_context);
    assert_eq!(result, 68 as i32);
}

// Line 46
fn c5_l46_action_invoke(result_object: &ResultObject, vm_context: &VmCtx) {
    println!("Executing function {}", "c5_l46_action_invoke");
    let func_index = match result_object.module.info.exports.get("inc") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(&VmCtx) = get_instance_function!(result_object.instance, func_index);
    let result = invoke_fn(&vm_context);
    
}

// Line 47
fn c6_l47_action_invoke(result_object: &ResultObject, vm_context: &VmCtx) {
    println!("Executing function {}", "c6_l47_action_invoke");
    let func_index = match result_object.module.info.exports.get("get") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(&VmCtx) -> i32 = get_instance_function!(result_object.instance, func_index);
    let result = invoke_fn(&vm_context);
    assert_eq!(result, 69 as i32);
}

// Line 48
fn c7_l48_action_invoke(result_object: &ResultObject, vm_context: &VmCtx) {
    println!("Executing function {}", "c7_l48_action_invoke");
    let func_index = match result_object.module.info.exports.get("inc") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(&VmCtx) = get_instance_function!(result_object.instance, func_index);
    let result = invoke_fn(&vm_context);
    
}

// Line 49
fn c8_l49_action_invoke(result_object: &ResultObject, vm_context: &VmCtx) {
    println!("Executing function {}", "c8_l49_action_invoke");
    let func_index = match result_object.module.info.exports.get("get") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(&VmCtx) -> i32 = get_instance_function!(result_object.instance, func_index);
    let result = invoke_fn(&vm_context);
    assert_eq!(result, 70 as i32);
}

// Line 51

#[test]
fn test_module_1() {
    let result_object = create_module_1();
    let vm_context = result_object.instance.generate_context();
    // We group the calls together
    start_module_1(&result_object, &vm_context);
    c4_l45_action_invoke(&result_object, &vm_context);
    c5_l46_action_invoke(&result_object, &vm_context);
    c6_l47_action_invoke(&result_object, &vm_context);
    c7_l48_action_invoke(&result_object, &vm_context);
    c8_l49_action_invoke(&result_object, &vm_context);
}
fn create_module_2() -> ResultObject {
    let module_str = "(module
      (type (;0;) (func))
      (type (;1;) (func (result i32)))
      (func (;0;) (type 0)
        i32.const 0
        i32.const 0
        i32.load8_u
        i32.const 1
        i32.add
        i32.store8)
      (func (;1;) (type 1) (result i32)
        i32.const 0
        i32.load8_u
        return)
      (func (;2;) (type 0)
        call 0
        call 0
        call 0)
      (memory (;0;) 1 1)
      (export \"inc\" (func 0))
      (export \"get\" (func 1))
      (start 2)
      (data (i32.const 0) \"A\"))
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    instantiate(wasm_binary, spectest_importobject()).expect("WASM can't be instantiated")
}
fn start_module_2(result_object: &ResultObject, vm_context: &VmCtx) {
    result_object.instance.start(&vm_context);
}

// Line 74
fn c10_l74_action_invoke(result_object: &ResultObject, vm_context: &VmCtx) {
    println!("Executing function {}", "c10_l74_action_invoke");
    let func_index = match result_object.module.info.exports.get("get") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(&VmCtx) -> i32 = get_instance_function!(result_object.instance, func_index);
    let result = invoke_fn(&vm_context);
    assert_eq!(result, 68 as i32);
}

// Line 75
fn c11_l75_action_invoke(result_object: &ResultObject, vm_context: &VmCtx) {
    println!("Executing function {}", "c11_l75_action_invoke");
    let func_index = match result_object.module.info.exports.get("inc") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(&VmCtx) = get_instance_function!(result_object.instance, func_index);
    let result = invoke_fn(&vm_context);
    
}

// Line 76
fn c12_l76_action_invoke(result_object: &ResultObject, vm_context: &VmCtx) {
    println!("Executing function {}", "c12_l76_action_invoke");
    let func_index = match result_object.module.info.exports.get("get") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(&VmCtx) -> i32 = get_instance_function!(result_object.instance, func_index);
    let result = invoke_fn(&vm_context);
    assert_eq!(result, 69 as i32);
}

// Line 77
fn c13_l77_action_invoke(result_object: &ResultObject, vm_context: &VmCtx) {
    println!("Executing function {}", "c13_l77_action_invoke");
    let func_index = match result_object.module.info.exports.get("inc") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(&VmCtx) = get_instance_function!(result_object.instance, func_index);
    let result = invoke_fn(&vm_context);
    
}

// Line 78
fn c14_l78_action_invoke(result_object: &ResultObject, vm_context: &VmCtx) {
    println!("Executing function {}", "c14_l78_action_invoke");
    let func_index = match result_object.module.info.exports.get("get") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(&VmCtx) -> i32 = get_instance_function!(result_object.instance, func_index);
    let result = invoke_fn(&vm_context);
    assert_eq!(result, 70 as i32);
}

// Line 80

#[test]
fn test_module_2() {
    let result_object = create_module_2();
    let vm_context = result_object.instance.generate_context();
    // We group the calls together
    start_module_2(&result_object, &vm_context);
    c10_l74_action_invoke(&result_object, &vm_context);
    c11_l75_action_invoke(&result_object, &vm_context);
    c12_l76_action_invoke(&result_object, &vm_context);
    c13_l77_action_invoke(&result_object, &vm_context);
    c14_l78_action_invoke(&result_object, &vm_context);
}
fn create_module_3() -> ResultObject {
    let module_str = "(module
      (type (;0;) (func (param i32)))
      (type (;1;) (func))
      (import \"spectest\" \"print_i32\" (func (;0;) (type 0)))
      (func (;1;) (type 1)
        i32.const 1
        call 0)
      (start 1))
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    instantiate(wasm_binary, spectest_importobject()).expect("WASM can't be instantiated")
}
fn start_module_3(result_object: &ResultObject, vm_context: &VmCtx) {
    result_object.instance.start(&vm_context);
}

// Line 86

#[test]
fn test_module_3() {
    let result_object = create_module_3();
    let vm_context = result_object.instance.generate_context();
    // We group the calls together
    start_module_3(&result_object, &vm_context);
}
fn create_module_4() -> ResultObject {
    let module_str = "(module
      (type (;0;) (func (param i32)))
      (type (;1;) (func))
      (import \"spectest\" \"print_i32\" (func (;0;) (type 0)))
      (func (;1;) (type 1)
        i32.const 2
        call 0)
      (start 1))
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    instantiate(wasm_binary, spectest_importobject()).expect("WASM can't be instantiated")
}
fn start_module_4(result_object: &ResultObject, vm_context: &VmCtx) {
    result_object.instance.start(&vm_context);
}

// Line 92

#[test]
fn test_module_4() {
    let result_object = create_module_4();
    let vm_context = result_object.instance.generate_context();
    // We group the calls together
    start_module_4(&result_object, &vm_context);
}
fn create_module_5() -> ResultObject {
    let module_str = "(module
      (type (;0;) (func))
      (import \"spectest\" \"print\" (func (;0;) (type 0)))
      (start 0))
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    instantiate(wasm_binary, spectest_importobject()).expect("WASM can't be instantiated")
}
fn start_module_5(result_object: &ResultObject, vm_context: &VmCtx) {
    result_object.instance.start(&vm_context);
}

// Line 98

#[test]
fn test_module_5() {
    let result_object = create_module_5();
    let vm_context = result_object.instance.generate_context();
    // We group the calls together
    start_module_5(&result_object, &vm_context);
}