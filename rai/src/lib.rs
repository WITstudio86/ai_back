pub const API_URL: &str = "https://open.bigmodel.cn/api/paas/v4/chat/completions";
pub const CONFIG_PATH: &str = "rai.toml";

mod ai;
pub use ai::Gpt;
mod argument;
pub use argument::{Arg, Command, Figure, Target};

mod local_data;
pub use local_data::{chat_init_data, teacher_init_data, Config, Data, Message};
mod process;
pub use process::chat_with;
