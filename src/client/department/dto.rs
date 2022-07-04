use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct ParamsDepartmentList {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dept_id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<ParamLanguage>,
}

#[derive(Debug, Deserialize, Serialize)]
// #[serde(untagged)]
pub enum ParamLanguage {
    #[serde(rename = "zh_CN")]
    CN,
    #[serde(rename = "en_US")]
    Us,
}

impl Default for ParamLanguage {
    fn default() -> Self {
        Self::CN
    }
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct ParamsDepartmentCreate {
    /// 部门名称
    /// 长度限制为1~64个字符，不允许包含字符"-"","以及","
    pub name: String,
    /// 父部门ID，根部门ID为1
    pub parent_id: i64,
    /// 是否隐藏本部门：
    /// true：隐藏部门，隐藏后本部门将不会显示在公司通讯录中
    /// false（默认值）：显示部门
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hide_dept: Option<bool>,
    /// 指定可以查看本部门的其他部门列表，总数不能超过200。
    /// 当hide_dept为true时，则此值生效
    /// 示例值："123,456"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dept_permits: Option<String>,
    /// 指定可以查看本部门的人员userId列表，总数不能超过200。
    /// 当hide_dept为true时，则此值生效
    /// 示例值："user123,manager222"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_permits: Option<String>,
    /// 是否限制本部门成员查看通讯录：
    /// true：开启限制。开启后本部门成员只能看到限定范围内的通讯录
    /// false（默认值）：不限制
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outer_dept: Option<bool>,
    /// 本部门成员是否只能看到所在部门及下级部门通讯录：
    /// true：只能看到所在部门及下级部门通讯录
    /// false：不能查看所有通讯录，在通讯录中仅能看到自己
    /// 当outer_dept为true时，此参数生效
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outer_dept_only_self: Option<bool>,
    /// 指定本部门成员可查看的通讯录用户userId列表，总数不能超过200。
    /// 当outer_dept为true时，此参数生效
    /// 示例值："manager123,user123"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outer_permit_users: Option<String>,
    /// 指定本部门成员可查看的通讯录部门ID列表，总数不能超过200。
    /// 当outer_dept为true时，此参数生效
    /// 示例值: "456,123"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outer_permit_depts: Option<String>,
    /// 是否创建一个关联此部门的企业群，默认为false即不创建。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_dept_group: Option<bool>,
    /// 是否默认同意加入该部门的申请：
    /// true：表示加入该部门的申请将默认同意
    /// false：表示加入该部门的申请需要有权限的管理员同意
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_approve_apply: Option<bool>,
    /// 在父部门中的排序值，order值小的排序靠前。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<u64>,
    /// 部门标识字段，开发者可用该字段来唯一标识一个部门，并与钉钉外部通讯录里的部门做映射。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_identifier: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ParamsDepartmentUpdate {
    /// 部门ID，可通过获取部门列表接口获取
    pub dept_id: i64,
    /// 父部门ID，根部ID为1
    pub parent_id: Option<i64>,
    /// 是否隐藏本部门：
    /// true：隐藏部门
    /// 隐藏后本部门将不会显示在公司通讯录中。
    /// false：显示部门
    /// 不传值，则保持不变
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hide_dept: Option<bool>,
    /// 指定可以查看本部门的其他部门列表。
    /// 当hide_dept为true时，则此值生效
    /// 说明 该参数列表总数和user_permits列表总数之和不能超过50
    /// 示例值: 123,456
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dept_permits: Option<String>,
    /// 指定可以查看本部门的用户userid列表。
    /// 当hide_dept为true时，则此值生效。
    /// 说明 该参数列表总数和dept_permits列表总数之和不能超过50。
    /// 示例值：user123,manager222
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_permits: Option<String>,
    /// 是否创建一个关联此部门的企业群，默认为false即不创建。
    /// 不传值，则保持不变
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_dept_group: Option<bool>,
    /// 在父部门中的排序值，order值小的排序靠前
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<i64>,
    /// 部门名称，长度限制为1~64个字符，不允许包含字符‘-’‘，’以及‘,’
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 部门标识字段，开发者可用该字段来唯一标识一个部门，并与钉钉外部通讯录里的部门做映射。
    /// 说明 该字段在企业管理后台部门信息中不可见
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_identifier: Option<String>,
    /// 是否限制本部门成员查看通讯录：
    /// true：开启限制。开启后本部门成员只能看到限定范围内的通讯录
    /// false：不限制
    /// 不传值，则保持不变
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outer_dept: Option<bool>,
    /// 指定本部门成员可查看的通讯录用户userid列表。
    /// 当outer_dept为true时，此参数生效。
    /// 说明 该参数列表总数和outer_permit_depts列表总数之和不能超过50。
    /// 示例值: user123,manager123
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outer_permit_users: Option<String>,
    /// 指定本部门成员可查看的通讯录部门ID列表。
    /// 当outer_dept为true时，此参数生效。
    /// 说明 该参数列表总数和outer_permit_users列表总数之和不能超过50。
    /// 示例值：123,456
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outer_permit_depts: Option<String>,
    /// 本部门成员是否只能看到所在部门及下级部门通讯录：
    /// true：只能看到所在部门及下级部门通讯录
    /// false：不能查看所有通讯录，在通讯录中仅能看到自己
    /// 当outer_dept为true时，此参数生效。
    /// 不传值，则保持不变。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outer_dept_only_self: Option<bool>,
    /// 通讯录语言：
    /// zh_CN：中文
    /// en_US：英文
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    /// 当部门群已经创建后，有新人加入部门时是否会自动加入该群：
    /// true：自动加入群
    /// false：不会自动加入群
    /// 不传值，则保持不变
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_add_user: Option<bool>,
    /// 是否默认同意加入该部门的申请：
    /// true：表示加入该部门的申请将默认同意
    /// false：表示加入该部门的申请需要有权限的管理员同意
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_approve_apply: Option<bool>,
    /// 部门的主管userId列表，多个userid之间使用英文逗号分隔。
    /// 说明 部门主管必须在当前部门内，否则接口会报错不存在的userId
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dept_manager_userid_list: Option<String>,
    /// 部门群是否包含子部门：
    /// true：包含
    /// false：不包含
    /// 不传值，则保持不变
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_contain_sub_dept: Option<bool>,
    /// 部门群是否包含外包部门：
    /// true：包含
    /// false：不包含
    /// 不传值，则保持不变
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_contain_outer_dept: Option<bool>,
    /// 部门群是否包含隐藏部门：
    /// true：包含
    /// false：不包含
    /// 不传值，则保持不变
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_contain_hidden_dept: Option<bool>,
    /// 企业群群主的userId。
    /// 说明 群主必须在当前部门内
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org_dept_owner: Option<String>,
    /// 强制更新的字段，支持清空指定的字段，多个字段之间使用英文逗号分隔。目前支持字段: dept_manager_userid_list
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_update_fields: Option<String>,
}

impl ParamsDepartmentUpdate {
    pub fn is_empty(&self) -> bool {
        if self.parent_id.is_none()
            && self.hide_dept.is_none()
            && self.dept_permits.is_none()
            && self.user_permits.is_none()
            && self.create_dept_group.is_none()
            && self.order.is_none()
            && self.name.is_none()
            && self.source_identifier.is_none()
            && self.outer_dept.is_none()
            && self.outer_permit_users.is_none()
            && self.outer_permit_depts.is_none()
            && self.outer_dept_only_self.is_none()
            && self.language.is_none()
            && self.auto_add_user.is_none()
            && self.auto_approve_apply.is_none()
            && self.dept_manager_userid_list.is_none()
            && self.group_contain_sub_dept.is_none()
            && self.group_contain_outer_dept.is_none()
            && self.group_contain_hidden_dept.is_none()
            && self.org_dept_owner.is_none()
            && self.force_update_fields.is_none()
        {
            return true;
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Deserialize, Serialize, Default)]
    struct Test {
        id: String,
        language: ParamLanguage,
    }

    #[test]
    fn test_to_json_string() {
        let item = Test {
            id: "x".to_string(),
            ..Default::default()
        };
        println!("{:?}", serde_json::to_string(&item));
    }
}
