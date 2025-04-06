use wasmi::{ImportsBuilder, Module, ModuleInstance, NopExternals, RuntimeValue};
use std::fs;
use std::path::Path;

pub struct SmartContractEngine;

impl SmartContractEngine {
    pub fn execute_wasm_contract(wasm_path: &str, func: &str, args: Vec<RuntimeValue>) -> Option<RuntimeValue> {
        if !Path::new(wasm_path).exists() {
            println!("[CONTRACT ERROR] WASM file not found: {}", wasm_path);
            return None;
        }

        let wasm_binary = fs::read(wasm_path).expect("Failed to read WASM contract");

        let module = Module::from_buffer(&wasm_binary).expect("Failed to load WASM module");
        let instance = ModuleInstance::new(&module, &ImportsBuilder::default())
            .expect("Failed to create WASM instance")
            .assert_no_start();

        let result = instance.invoke_export(func, &args, &mut NopExternals);
        match result {
            Ok(Some(val)) => {
                println!("[CONTRACT EXECUTED] Result: {:?}", val);
                Some(val)
            },
            Ok(None) => {
                println!("[CONTRACT EXECUTED] No return value.");
                None
            },
            Err(e) => {
                println!("[CONTRACT ERROR] Execution failed: {:?}", e);
                None
            }
        }
    }
}
