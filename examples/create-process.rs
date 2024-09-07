use std::error::Error;
use serde::Deserialize;
use wmi::{COMLibrary, Variant, WMIConnection};
use wmi::exec::WmiExecParam;


fn type_of<T>(_: &T) -> String{
    format!("{}", std::any::type_name::<T>())
}

fn main() -> Result<(), Box<dyn Error>> {
    let com_con =  COMLibrary::new()?;
    let wmi_con = WMIConnection::with_namespace_path(r#"ROOT\CIMV2"#, com_con.into())?;
    #[derive(Deserialize, Debug, Default)]
    struct Win32_Process {
        ProcessId: u32,
        ReturnValue: u32,
    }
    let result = wmi_con.exec_method::<Win32_Process>("Win32_Process", "Create", &[
        WmiExecParam{
            key: "CommandLine".to_string(),
            value: Variant::String("notepad.exe".to_string()),
        },
    ])?;
    println!("result: {:?}", result);
    Ok(())
}