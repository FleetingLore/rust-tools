use crate::os_type::OsType;

#[allow(unused)]
pub struct DarwinOs;

impl OsType for DarwinOs {
    fn name(&self) -> String {
        "Darwin".to_string()
    }
}