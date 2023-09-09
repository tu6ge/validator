//! define Rule trait, inner rule type

use std::slice::Iter;

use crate::value::{FromValue, Value, ValueMap};

use self::boxed::{ErasedRule, RuleIntoBoxed};

pub mod available;
mod boxed;

/// A Rule trait
pub trait Rule<M>: 'static + Sized + Clone {
    /// Named rule type, allow `a-z` | `A-Z` | `0-9` | `_`, and not start with `0-9`
    fn name(&self) -> &'static str;

    /// Rule specific implementation, data is gived type all field's value, and current field index.
    fn call(&mut self, data: &mut ValueMap) -> Result<(), Message>;

    fn into_boxed(self) -> RuleIntoBoxed<Self, M> {
        RuleIntoBoxed::new(self)
    }
}

/// Error message returned when validate fail
pub struct Message {
    inner: String,
}

impl Message {
    fn new(inner: String) -> Message {
        Message { inner }
    }
}

impl From<String> for Message {
    fn from(inner: String) -> Self {
        Self { inner }
    }
}
impl From<Message> for String {
    fn from(msg: Message) -> Self {
        msg.inner
    }
}
impl From<&str> for Message {
    fn from(value: &str) -> Self {
        Self {
            inner: value.to_owned(),
        }
    }
}

/// Rule extension, it contains some rules, such as
/// ```rust,ignore
/// Rule1.and(Rule2).and(Rule3)
/// ```
pub trait RuleExt {
    fn and<R: Rule<()> + Clone>(self, other: R) -> RuleList;
    fn custom<F, V>(self, other: F) -> RuleList
    where
        F: for<'a> FnOnce(&'a mut V) -> Result<(), String> + 'static + Clone,
        F: Rule<V>,
        V: FromValue + 'static;
}

impl<R: Rule<()> + Clone> RuleExt for R {
    fn and<R2: Rule<()> + Clone>(self, other: R2) -> RuleList {
        RuleList {
            list: vec![ErasedRule::new(self), ErasedRule::new(other)],
            ..Default::default()
        }
    }
    fn custom<F, V>(self, other: F) -> RuleList
    where
        F: for<'a> FnOnce(&'a mut V) -> Result<(), String> + 'static + Clone,
        F: Rule<V>,
        V: FromValue + 'static,
    {
        RuleList {
            list: vec![ErasedRule::new(self), ErasedRule::new(other)],
            ..Default::default()
        }
    }
}

/// Rules collection
#[derive(Default, Clone)]
pub struct RuleList {
    list: Vec<ErasedRule>,
    is_bail: bool,
}

impl RuleList {
    pub fn and<R: Rule<()> + Clone>(mut self, other: R) -> Self {
        self.list.push(ErasedRule::new(other));
        self
    }
    pub fn custom<F, V>(mut self, other: F) -> Self
    where
        F: for<'a> FnOnce(&'a mut V) -> Result<(), String> + 'static + Clone,
        F: Rule<V>,
        V: FromValue + 'static,
    {
        self.list.push(ErasedRule::new(other));
        self
    }

    pub fn bail(mut self) -> Self {
        self.is_bail = true;
        self
    }

    #[must_use]
    pub(crate) fn call(mut self, data: &mut ValueMap) -> Vec<(&'static str, String)> {
        let mut msg = Vec::new();
        for endpoint in self.list.iter_mut() {
            let _ = endpoint
                .call(data)
                .map_err(|e| msg.push((endpoint.name(), e)));
            if self.is_bail && !msg.is_empty() {
                return msg;
            }
        }
        msg
    }

    fn iter(&self) -> Iter<'_, ErasedRule> {
        self.list.iter()
    }

    /// check the rule name is existing
    pub(crate) fn contains(&self, rule: &str) -> bool {
        self.iter()
            .map(|endpoint| endpoint.name())
            .find(|&name| name == rule)
            .is_some()
    }

    /// check all rule names is valid or not
    pub(crate) fn valid_name(&self) -> bool {
        self.iter().map(|endpoint| endpoint.name()).all(|name| {
            let mut chares = name.chars();
            let first = match chares.next() {
                Some(ch) => ch,
                None => return false,
            };

            if !(first.is_ascii_alphabetic() || first == '_') {
                return false;
            }

            loop {
                match chares.next() {
                    Some(ch) if ch.is_ascii_alphanumeric() || ch == '_' => (),
                    None => break true,
                    _ => break false,
                }
            }
        })
    }
}

pub trait IntoRuleList {
    fn into_list(self) -> RuleList;
}

pub fn custom<F, V>(f: F) -> RuleList
where
    F: for<'a> FnOnce(&'a mut V) -> Result<(), String> + 'static + Clone,
    F: Rule<V>,
    V: FromValue + 'static,
{
    RuleList {
        list: vec![ErasedRule::new(f)],
        ..Default::default()
    }
}

impl IntoRuleList for RuleList {
    fn into_list(self) -> Self {
        self
    }
}
impl<R> IntoRuleList for R
where
    R: Rule<()> + Clone,
{
    fn into_list(self) -> RuleList {
        RuleList {
            list: vec![ErasedRule::new(self)],
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod test_regster {
    use super::available::*;
    use super::*;
    fn register<R: IntoRuleList>(_: R) {}

    fn hander(_val: &mut ValueMap) -> Result<(), String> {
        Ok(())
    }
    fn hander2(_val: &mut Value) -> Result<(), String> {
        Ok(())
    }

    #[test]
    fn test() {
        register(Required);
        register(Required.custom(hander2));
        register(Required.custom(hander));
        register(Required.and(StartWith("foo")));
        register(Required.and(StartWith("foo")).bail());
        register(Required.and(StartWith("foo")).custom(hander2).bail());
        register(
            Required
                .and(StartWith("foo"))
                .custom(hander2)
                .custom(hander)
                .bail(),
        );
        register(custom(hander2));
        register(custom(hander));
        register(custom(hander).and(StartWith("foo")));
        register(custom(hander).and(StartWith("foo")).bail());
        register(custom(|_a: &mut u8| Ok(())));
        register(custom(|_a: &mut u8| Ok(())));
    }
}

pub trait RuleShortcut {
    /// Named rule type
    fn name(&self) -> &'static str;

    /// Default rule error message, when validate fails, return the message to user
    fn message(&self) -> Message;

    /// Rule specific implementation, data is gived type all field's value, and current field index.
    /// when the method return true, call_message will return Ok(()), or else return Err(String)
    ///
    /// *Panic*
    /// when not found value
    fn call_with_relate(&mut self, data: &mut ValueMap) -> bool {
        self.call(data.current_mut().expect("not found value with fields"))
    }

    /// Rule specific implementation, data is current field's value
    fn call(&mut self, data: &mut Value) -> bool;
}

impl<T> Rule<()> for T
where
    T: RuleShortcut + 'static + Clone,
{
    fn name(&self) -> &'static str {
        self.name()
    }
    /// Rule specific implementation, data is gived type all field's value, and current field index.
    fn call(&mut self, data: &mut ValueMap) -> Result<(), Message> {
        if self.call_with_relate(data) {
            Ok(())
        } else {
            Err(self.message())
        }
    }
}

// impl<F> Rule<ValueMap> for F
// where
//     F: for<'a> FnOnce(&'a mut ValueMap) -> Result<(), String> + 'static + Clone,
// {
//     fn call(&mut self, data: &mut ValueMap) -> Result<(), Message> {
//         self.clone()(data).map_err(|s| s.into())
//     }

//     fn name(&self) -> &'static str {
//         "relate"
//     }
// }

// impl<F> Rule<Value> for F
// where
//     F: for<'a> FnOnce(&'a mut Value) -> Result<(), String> + 'static + Clone,
// {
//     /// *Panic*
//     /// when not found value
//     fn call(&mut self, data: &mut ValueMap) -> Result<(), Message> {
//         let value = data.current_mut().expect("not found value with fields");
//         self.clone()(value).map_err(|e| e.into())
//     }

//     fn name(&self) -> &'static str {
//         "custom"
//     }
// }

impl<F, V> Rule<V> for F
where
    F: for<'a> FnOnce(&'a mut V) -> Result<(), String> + 'static + Clone,
    V: FromValue,
{
    fn call(&mut self, data: &mut ValueMap) -> Result<(), Message> {
        let val = V::from_value(data).unwrap();
        self.clone()(val).map_err(Message::new)
    }
    fn name(&self) -> &'static str {
        "custom"
    }
}
