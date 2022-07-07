/// 客户端
pub mod client;
pub use client::{Client, DepartmentManager, UserManager};

mod error;
pub use error::Result;
