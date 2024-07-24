mod process;
pub use process::{read_message, save_message, show_message, verify_file, Data, Message};
mod const_var;
pub use const_var::{API_KEY, API_URL};
mod ai;
pub use ai::{get_content, write_content_to_file};
mod argument;
pub use argument::Args;
