use rustpython_vm::pymodule;

#[pymodule(name = "oink")]
pub mod oink_impl {
    use rustpython_vm::{PyObjectRef, PyResult, VirtualMachine};
    use std::sync::Mutex;

    static OINK_CALLBACK: Mutex<Option<PyObjectRef>> = Mutex::new(None);

    #[pyfunction]
    fn reg_cb(callback: PyObjectRef, vm: &VirtualMachine) -> PyResult<()> {
        if !callback.is_callable() {
            // raise TypeError exception
            return Err(vm.new_type_error("callback must be a callable"));
        }

        let mut cb_ref = OINK_CALLBACK.lock().unwrap();
        println!("previous callback: {cb_ref:?}");
        *cb_ref = Some(callback.clone());

        Ok(())
    }

    #[pyfunction]
    fn call_cb(vm: &VirtualMachine) -> PyResult<()> {
        let cb_ref = OINK_CALLBACK.lock().unwrap();
        if cb_ref.is_none() {
            // raise RuntimeError exception
            return Err(vm.new_runtime_error("no oink callback set"));
        }

        println!("call it, yeah!");
        let cb_obj = cb_ref.as_ref().unwrap();
        match cb_obj.call((), vm) {
            // propogate exception for the callback
            Err(err) => Err(err),
            // 'swallow' result, always return None
            Ok(_) => Ok(()),
        }
    }
}
