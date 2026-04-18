use std::borrow::Cow;

pub struct EnvOverride {
    varname: &'static str,
    default: &'static str,
}

impl EnvOverride {
    pub const fn new(varname: &'static str, default: &'static str) -> Self {
        Self { varname, default }
    }

    pub fn get(&self) -> Cow<'static, str> {
        if let Ok(value) = std::env::var(self.varname) {
            Cow::Owned(value)
        } else {
            Cow::Borrowed(self.default)
        }
    }
}
