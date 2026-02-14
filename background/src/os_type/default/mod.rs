use crate::os_type::{OsType, DefaultOsType};

#[allow(unused)]
pub struct DefaultOsImpl;

impl OsType for DefaultOsImpl {
    fn name(&self) -> String {
        "Default OS".to_string()
    }
}

impl DefaultOsType for DefaultOsImpl {}