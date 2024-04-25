//! Value can not be empty, supported `Vec`, `String`, `HashMap`
//! or `BTreeMap`. other types always return true.
//!
//! # Examples
//! ```
//! # use serde::Serialize;
//! # use valitron::{available::{Required, MessageKind}, Validatable, Validator};
//! #[derive(Serialize, Debug)]
//! struct Input {
//!     username: String,
//!     password: String,
//! }
//!
//! let input = Input {
//!     username: String::default(),
//!     password: String::default(),
//! };
//! let err = input
//!     .validate(
//!         Validator::new()
//!             .rule("username", Required)
//!             .rule("password", Required),
//!     )
//!     .unwrap_err();
//!
//! assert!(matches!(
//!     err.get("username").unwrap()[0].kind(),
//!     MessageKind::Required
//! ));
//! assert!(matches!(
//!     err.get("password").unwrap()[0].kind(),
//!     MessageKind::Required
//! ));
//!
//! let input = Input {
//!     username: String::from("foo"),
//!     password: String::from("bar"),
//! };
//! input
//!     .validate(
//!         Validator::new()
//!             .rule("username", Required)
//!             .rule("password", Required),
//!     )
//!     .unwrap();
//! ```

use super::Message;
use crate::{RuleShortcut, Value};

#[derive(Clone, Copy, Debug)]
pub struct Required;

const NAME: &'static str = "required";

impl RuleShortcut for Required {
    type Message = Message;

    const NAME: &'static str = NAME;

    fn message(&self) -> Self::Message {
        Message::new(super::MessageKind::Required)
    }

    fn call(&mut self, value: &mut Value) -> bool {
        match value {
            Value::String(s) => !s.is_empty(),
            Value::Array(arr) => !arr.is_empty(),
            Value::Map(map) => !map.is_empty(),
            _ => true,
        }
    }
}
