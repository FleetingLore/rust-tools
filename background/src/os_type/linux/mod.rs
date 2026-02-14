use crate::os_type::OsType;

#[allow(unused)]
pub struct LinuxOs;

impl OsType for LinuxOs {
    fn name(&self) -> String {
        "Linux".to_string()
    }
}