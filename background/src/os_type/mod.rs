// 模块声明
pub mod windows;
pub mod darwin;
pub mod linux;
pub mod default;

// 定义基础 trait
#[allow(unused)]
pub trait OsType {
    fn name(&self) -> String;
}

// 定义子 trait
#[allow(unused)]
pub trait DefaultOsType: OsType {}

// 环境结构体
#[allow(unused)]
pub struct Environment {
    os_type_status: Box<dyn OsType>,
}

impl Default for Environment {
    fn default() -> Self {
        Environment {
            // 注意：这里应该是 default::Something，而不是 default::DefaultOsType
            // DefaultOsType 是 trait，不能直接 new
            os_type_status: Box::new(default::DefaultOsImpl {}) // 假设你有一个 DefaultOsImpl 结构体
        }
    }
}
