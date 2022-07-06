use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct PageResult<T> {
    /// 是否还有更多的数据
    pub has_more: bool,
    /// 下一次分页的游标，如果has_more为false，表示没有更多的分页数据。
    pub next_cursor: Option<i64>,
    /// 列表
    pub list: Vec<T>,
}
