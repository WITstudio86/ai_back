use serde::Deserialize;
#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct ResponseType {
    // 任务 id
    pub id: String,
    // 请求创建时间 , 时间戳
    pub created: u64,
    // 模型名字
    pub model: String,
    // 当前对话的输出内容
    pub choices: Vec<Choice>,
    // 结束时返回本次模型调用的 tokens 数量统计。
    pub usage: Usage,
    // 返回网页搜索的相关信息。
    pub web_search: Option<Vec<String>>,
}
#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct Choice {
    // 结果下标
    pub index: u32,
    // 终止原因
    pub finish_reason: String,
    // 当前返回的文本信息
    pub message: Message,
}
#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct Message {
    // 发送者
    pub role: String,
    // 发送内容
    pub content: String,
    //模型生成的应调用函数的名称和参数。
    pub tool_calls: Option<Vec<Calls>>,
}
#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct Calls {
    pub function: Function,
    // 命中函数的唯一标识符。
    pub id: String,
    // 模型调用工具的类型，目前仅支持function。
    pub type_: String,
}
#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct Function {
    // 模型生成的应调用函数的名称。
    pub name: String,
    //模型生成的 JSON 格式的函数调用参数。
    // 请注意，模型生成的 JSON 并不总是有效的，可能会出现函数模式未定义的参数。
    // 在调用函数之前，请在代码中验证参数。
    pub arguments: String,
}
#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct Usage {
    // 请求的输入 token 数量
    pub prompt_tokens: u64,
    // 请求的输出 token 数量
    pub completion_tokens: u64,
    // 请求的总 token 数量
    pub total_tokens: u64,
}
