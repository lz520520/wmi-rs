use serde::de;
use windows::Win32::System::Wmi;
use windows::Win32::System::Wmi::{IWbemClassObject};
use windows_core::{BSTR, PCWSTR};
use crate::utils::{ wide_rust_to_c_string};
use crate::{Variant, WMIConnection, WMIError, WMIResult};
use crate::de::wbem_class_de::from_wbem_class_obj;
use crate::result_enumerator::IWbemClassWrapper;


pub struct WmiExecParam {
    pub key: String,
    pub value: Variant,
}


impl WMIConnection {
    pub fn get_object(&self,name: &str)
                      -> WMIResult<IWbemClassObject> {
        let mut wmi_object = None;
        unsafe { self.svc.GetObject(&BSTR::from(name),
                                    Wmi::WBEM_FLAG_RETURN_WBEM_COMPLETE,
                                    None,
                                    Some(&mut wmi_object),
                                    None) }?;

        Ok(wmi_object.ok_or_else(||WMIError::SerdeError(format!("object {} is not found",name)))?)
    }
    pub fn exec_method<T>(&self,class_name: &str,  method_name: &str, params: &[WmiExecParam]) -> WMIResult<T>
    where
        T: de::DeserializeOwned + Default{
        unsafe {
            let mut in_params_class: Option<IWbemClassObject> = None;
            let object = self.get_object(class_name)?;
            object.GetMethod(PCWSTR::from_raw(wide_rust_to_c_string(method_name).as_ptr()), 0,&mut in_params_class, std::ptr::null_mut())?;
            let in_params_class = in_params_class.ok_or_else(||WMIError::SerdeError("in params class is none".into()))?;
            let in_params = in_params_class.SpawnInstance(0)?;
            for param in params {
                in_params.Put(PCWSTR::from_raw(wide_rust_to_c_string(&param.key).as_ptr()),0, &Variant::to_variant(&param.value)?, 0)?;
            }
            let mut out_params = None;
            self.svc.ExecMethod(
                &BSTR::from(class_name),
                &BSTR::from(method_name),
                Default::default(),
                None,
                &in_params,
                Some(&mut out_params),
                None,
            )?;
            if out_params == None {
                return Ok(T::default())
            }
            let out_params = out_params.ok_or_else(|| WMIError::SerdeError("out is none".into()))?;
            // let mut value = VARIANT::default();
            from_wbem_class_obj(IWbemClassWrapper::new(out_params))
            // out_params.Get(PCWSTR::from_raw(wide_rust_to_c_string("ReturnValue").as_ptr()), 0, &mut value, None, None) ?;
        }
    }
}