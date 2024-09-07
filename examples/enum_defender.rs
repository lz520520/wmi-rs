use std::error::Error;
use serde::{Deserialize, Deserializer};
use wmi::{COMLibrary, Variant, WMIConnection};
use wmi::exec::WmiExecParam;

fn get_value_array(value: &Variant) -> Vec<String> {
    let mut ret = Vec::<String>::new();
    match value {
        Variant::String(val) => {
            ret.push(val.to_string());
        }
        Variant::Array(val) => {
            ret.extend(val.iter().filter_map(|x| {
                if let Variant::String(x) = x {
                    Some(x.to_string())
                } else {
                    None
                }
            }));
        }
        _ => {}
    }
    ret
}
fn main() -> Result<(), Box<dyn Error>> {
    let com_con =  COMLibrary::new()?;
    let wmi_con = WMIConnection::with_namespace_path(r#"ROOT\Microsoft\Windows\Defender"#, com_con.into())?;
    let results: Vec<std::collections::HashMap<String, Variant>> = wmi_con.raw_query("Select * from MSFT_MpPreference")?;
    let mut exclusion_path = Vec::<String>::new();

    #[derive(Deserialize, Debug,Default)]
    struct MSFT_MpPreferenceList {
        ExclusionPath: Vec<String>,
    }

    for valueMap in results {
        let value: Option<&Variant> = valueMap.get("ExclusionPath");
        if let Some(value) = value {
            exclusion_path.extend(value.to_vec());
        }

    }
    if !exclusion_path.is_empty() {
        println!("ExclusionPath:\n{}", exclusion_path.join("\n"));
    }
    #[derive(Deserialize, Debug,Default)]
    struct MSFT_MpPreferenceAdd {
        ReturnValue: u32,
    }
    let result: MSFT_MpPreferenceAdd = wmi_con.exec_method("MSFT_MpPreference", "Add", &[
        WmiExecParam{
            key: "ExclusionPath".to_string(),
            value: Variant::Array(vec![Variant::String("E:\\test".to_string())]),
        },
    ])?;
    println!("result: {:?}", result);

    Ok(())
}