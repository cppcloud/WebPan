use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserInfo {
    pub uid: u64,                     // 用户账号 ID
    pub nickname: String,             // 昵称
    pub head_image: String,           // 头像 URL
    pub passport: String,             // 手机号码
    pub mail: String,                 // 邮箱
    pub space_used: u64,              // 已用空间
    pub space_permanent: u64,         // 永久空间
    pub space_temp: u64,              // 临时空间
    #[serde(deserialize_with = "deserialize_number_or_string")] // 处理 spaceTempExpr
    pub space_temp_expr: String,      // 临时空间到期日
    pub vip: bool,                    // 是否为 VIP 用户
    pub direct_traffic: u64,          // 直链流量
    #[serde(rename = "isHideUID")]     // 确保字段映射正确
    pub is_hide_uid: bool,            // 是否隐藏 UID
}

// 自定义反序列化函数，用于兼容整数或字符串类型
use serde::de::{self, Deserializer};
use std::fmt;

fn deserialize_number_or_string<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    struct StringOrNumber;

    impl<'de> de::Visitor<'de> for StringOrNumber {
        type Value = String;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a string or a number")
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(value.to_string())
        }

        fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(value.to_string())
        }

        fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(value.to_string())
        }
    }

    deserializer.deserialize_any(StringOrNumber)
}
