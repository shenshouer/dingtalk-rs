/// 客户端
pub mod client;
pub use client::{
    Client, DepartmentManager, DingTalkCrypto, EventSubscriber, UserManager, WorkNotifier,
};

mod error;
pub use error::Result;
