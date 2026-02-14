use crate::os_type::OsType;

#[allow(unused)]
pub struct WindowsOs;

impl OsType for WindowsOs {
    fn name(&self) -> String {
        "Windows".to_string()
    }
}