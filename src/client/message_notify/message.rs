use serde::{Deserialize, Serialize};

// 参考： https://open.dingtalk.com/document/orgapp-server/message-types-and-data-format

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Message {
    /// 消息类型。
    /// 文本消息类型为：text
    pub msgtype: String,
    #[serde(flatten)]
    pub content: MessageBody,
}

impl From<MessageBody> for Message {
    fn from(body: MessageBody) -> Self {
        let msgtype = match &body {
            MessageBody::Text { content: _ } => "text",
            MessageBody::File { media_id: _ } => "file",
            MessageBody::Image { media_id: _ } => "image",
            MessageBody::Voice {
                media_id: _,
                duration: _,
            } => "voice",
            MessageBody::Link {
                message_url: _,
                pic_url: _,
                title: _,
                text: _,
            } => "link",
            MessageBody::Oa {
                message_url: _,
                pc_message_url: _,
                head: _,
                status_bar: _,
                body: _,
            } => "oa",
            MessageBody::Markdown { title: _, text: _ } => "markdown",
            MessageBody::ActionCard {
                markdown: _,
                title: _,
                single_title: _,
                single_url: _,
                btn_orientation: _,
                btn_json_list: _,
            } => "action_card",
        };

        Self {
            msgtype: msgtype.to_string(),
            content: body,
        }
    }
}

/// 消息体
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum MessageBody {
    /// 文本消息
    Text {
        /// 消息内容，建议500字符以内
        content: String,
    },
    /// 图片消息
    Image {
        /// 媒体文件mediaid，建议宽600像素 x 400像素，宽高比3 : 2。
        /// 企业内部应用，通过上传媒体文件接口获取
        /// 第三方企业应用，通过上传媒体文件接口获取
        media_id: String,
    },
    /// 语音消息
    Voice {
        /// 媒体文件ID。
        /// 企业内部应用，通过上传媒体文件接口获取
        /// 第三方企业应用，通过上传媒体文件接口获取
        media_id: String,
        /// 正整数，小于60，表示音频时长
        duration: i32,
    },
    /// 文件消息
    File {
        /// 媒体文件ID。
        /// 企业内部应用，通过上传媒体文件接口获取
        /// 第三方企业应用，通过上传媒体文件接口获取
        media_id: String,
    },
    /// 链接消息
    Link {
        /// 消息点击链接地址，当发送消息为小程序时支持小程序跳转链接。
        /// 企业内部应用参考消息链接说明
        /// 第三方企业应用参考消息链接说明
        #[serde(rename = "messageUrl")]
        message_url: String,
        /// 企业内部应用通过上传媒体文件接口获取
        /// 第三方企业应用通过上传媒体文件接口获取
        #[serde(rename = "picUrl")]
        pic_url: String,
        /// 消息标题，建议100字符以内
        title: String,
        /// 消息描述，建议500字符以内
        text: String,
    },
    /// OA消息
    Oa {
        /// 消息点击链接地址，当发送消息为小程序时支持小程序跳转链接。
        /// 企业内部应用参考消息链接说明
        /// 第三方企业应用参考消息链接说明
        message_url: String,
        /// PC端点击消息时跳转到的地址
        #[serde(skip_serializing_if = "Option::is_none")]
        pc_message_url: Option<String>,
        /// 消息头部内容
        head: OaHead,
        /// 消息状态栏，只支持接收者的userid列表，userid最多不能超过5个人。
        /// 说明 不支持部门id列表，并且to_all_user不能传true
        #[serde(skip_serializing_if = "Option::is_none")]
        status_bar: Option<OaStatusBar>,
        /// 消息体
        body: OaBody,
    },
    /// markdown消息
    Markdown {
        /// 首屏会话透出的展示内容
        title: String,
        /// markdown格式的消息，最大不超过5000字符
        text: String,
    },
    /// 卡片消息
    ActionCard {
        /// 消息内容，支持markdown，语法参考标准markdown语法。建议1000个字符以内
        markdown: String,
        /// 透出到会话列表和通知的文案
        #[serde(skip_serializing_if = "Option::is_none")]
        title: Option<String>,
        /// 使用整体跳转ActionCard样式时的标题。必须与single_url同时设置，最长20个字符。
        /// 说明 如果是整体跳转的ActionCard样式，则single_title和single_url必须设置。
        #[serde(skip_serializing_if = "Option::is_none")]
        single_title: Option<String>,
        /// 消息点击链接地址，当发送消息为小程序时支持小程序跳转链接，最长500个字符。
        /// 企业内部应用通过上传媒体文件接口获取
        /// 第三方企业应用通过上传媒体文件接口获取
        #[serde(skip_serializing_if = "Option::is_none")]
        single_url: Option<String>,
        /// 使用独立跳转ActionCard样式时的按钮排列方式：
        /// 0：竖直排列
        /// 1：横向排列
        /// 必须与btn_json_list同时设置
        #[serde(skip_serializing_if = "Option::is_none")]
        btn_orientation: Option<String>,
        /// 使用独立跳转ActionCard样式时的按钮列表；必须与btn_orientation同时设置，且长度不超过1000字符。
        /// 说明 如果是独立跳转的ActionCard样式，则btn_json_list和btn_orientation必须设置
        #[serde(skip_serializing_if = "Option::is_none")]
        btn_json_list: Option<Vec<BtnJson>>,
    },
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct OaHead {
    /// 消息头部的背景颜色。
    /// 长度限制为8个英文字符，其中前2为表示透明度，后6位表示颜色值。不要添加0x
    pub bgcolor: String,
    /// 消息的头部标题。
    /// 企业内部应用
    /// 如果是发送工作通知消息，该参数会被替换为当前应用名称。
    /// 如果是发送消息到企业群或者发送普通消息，该参数有效，长度限制为最多10个字符。
    /// 第三方企业应用
    /// 如果是发送工作通知消息，该参数会被替换为当前应用名称。
    /// 如果是发送普通消息，该参数有效，长度限制为最多10个字符
    pub text: String,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct OaStatusBar {
    /// 状态栏文案
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_value: Option<String>,
    /// 状态栏背景色，默认为黑色，推荐0xFF加六位颜色值
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_bg: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct OaBody {
    /// 消息体的标题，建议50个字符以内
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// 消息体的表单，最多显示6个，超过会被隐藏
    #[serde(skip_serializing_if = "Option::is_none")]
    pub form: Option<Vec<KvPair>>,
    /// 单行富文本信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rich: Option<Rich>,
    /// 消息体的内容，最多显示3行
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// 消息体中的图片，支持图片资源@mediaId。建议宽600像素 x 400像素，宽高比3 : 2。
    /// 企业内部应用通过上传媒体文件接口获取
    /// 第三方企业应用通过上传媒体文件接口获取
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    /// 自定义的附件数目。此数字仅供显示，钉钉不作验证
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_count: Option<String>,
    /// 自定义的作者名字
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct KvPair {
    /// 消息体的关键字
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// 消息体的关键字对应的值
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Rich {
    /// 单行富文本信息的数目
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num: Option<String>,
    /// 单行富文本信息的单位
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct BtnJson {
    /// 使用独立跳转ActionCard样式时的按钮的标题，最长20个字符
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// 使用独立跳转ActionCard样式时的跳转链接，最长700个字符
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_url: Option<String>,
}

impl Default for MessageBody {
    fn default() -> Self {
        Self::Text {
            content: String::from("默认消息"),
        }
    }
}
