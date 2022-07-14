# dingtalk-rs
钉钉Rust版SDK

接口参考：https://open.dingtalk.com/document/orgapp-server/api-overview

## doc
[crates.io](https://crates.io/crates/dingtalk-rs)

## examples

[examples](./examples)

### run examples

- 将 `.env.sample` 改名为 `.env`, 替换`.env`中的`APP_KEY`与`APP_SECRET`
- 修改[examples](./examples)对应rs文件中的参数后，使用`cargo run --example xxx `运行，如运行user示例： `cargo run --example user`

## todo list

- [ ] 事件订阅 [消息加解密](./src/client/event_subscribe/callback_crypto.rs)
- [ ] 获取访问凭证
    - [✓] [获取企业内部应用的access_token](./src/client/mod.rs)
    - [ ] 服务商获取第三方应用授权企业的access_token
    - [ ] 获取jsapi_ticket
    - [ ] 获取微应用后台免登的access_token
- [ ] 身份验证
- [ ] 酷应用
- [ ] 通讯录管理
    - [✓] 用户管理 [实现](./src/client/user/mod.rs) [例子](./examples/user.rs)
    - [ ] 专属账号 
    - [✓] 部门管理 [实现](./src/client/department/mod.rs) [例子](./examples/department.rs)
    - [ ] 角色管理
    - [ ] 外部联系人
    - [ ] 企业管理
    - [ ] 通讯录可见性管理
    - [ ] 获取用户通讯录个人信息
    - [ ] 通讯录事件
- [ ] 互联平台
- [ ] 群会话管理
- [ ] 消息通知
- [ ] OA审批
- [ ] 智能填表
- [ ] 智能人事
- [ ] 考勤
- [ ] 代办任务
- [ ] 日程
- [ ] 钉盘
- [ ] 签到
- [ ] 公告
- [ ] 钉工牌
- [ ] 客户管理