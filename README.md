# dingtalk-rs
钉钉Rust版SDK

接口参考：https://open.dingtalk.com/document/orgapp-server/api-overview

## doc
[crates.io](https://crates.io/crates/dingtalk)

## examples

[examples](./examples)

### run examples

- 将 `.env.sample` 改名为 `.env`, 替换`.env`中的`APP_KEY`与`APP_SECRET`
- 修改[examples](./examples)对应rs文件中的参数后，使用`cargo run --example xxx `运行，如运行user示例： `cargo run --example user`