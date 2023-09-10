use crate::{rule::Message, RuleShortcut, Value};

#[derive(Clone, Debug)]
pub struct Required;

impl RuleShortcut for Required {
    fn name(&self) -> &'static str {
        "required"
    }
    fn message(&self) -> Message {
        "this field is required".into()
    }

    fn call(&mut self, value: &mut Value) -> bool {
        match value {
            Value::Uint8(_)
            | Value::Uint16(_)
            | Value::Uint32(_)
            | Value::Uint64(_)
            | Value::Int8(_)
            | Value::Int16(_)
            | Value::Int32(_)
            | Value::Int64(_) => true,
            Value::String(s) => !s.is_empty(),
            Value::Struct(_) => true,
            _ => todo!(),
        }
    }
}
