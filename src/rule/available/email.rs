//! Value must be a valid email address, supported `String`, and other types always return false.
//!
//! # Examples
//! ```
//! # use serde::Serialize;
//! # use valitron::{available::{Email, MessageKind}, Validatable, Validator};
//! #[derive(Serialize, Debug)]
//! struct Input {
//!     email: String,
//!     password: String,
//! }
//!
//! let input = Input {
//!     email: String::from("user"),
//!     password: String::default(),
//! };
//! let err = input
//!     .validate(
//!         Validator::new()
//!             .rule("email", Email)
//!     )
//!     .unwrap_err();
//!
//! assert!(matches!(
//!     err.get("email").unwrap()[0].kind(),
//!     MessageKind::Email
//! ));
//!
//! let input = Input {
//!     email: String::from("user@example.com"),
//!     password: String::from("bar"),
//! };
//! input
//!     .validate(
//!         Validator::new()
//!             .rule("email", Email)
//!     )
//!     .unwrap();
//! ```

use super::Message;
use crate::{RuleShortcut, Value};

mod parse;

pub use parse::validate_email;

#[derive(Clone, Debug)]
pub struct Email;

const NAME: &'static str = "email";

impl RuleShortcut for Email {
    type Message = Message;

    const NAME: &'static str = NAME;

    fn message(&self) -> Self::Message {
        Message::new(super::MessageKind::Email)
    }

    fn call(&mut self, value: &mut Value) -> bool {
        match value {
            Value::String(s) => validate_email(s),
            _ => false,
        }
    }
}
