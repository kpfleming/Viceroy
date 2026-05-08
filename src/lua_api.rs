use mlua::prelude::*;
use crate::ExecuteCtx;
use std::fmt;

impl fmt::Display for ExecuteCtx {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (name, details) in self.backends() {
            writeln!(f, "Backend: {name}")?;
            writeln!(f, "{details:?}")?;
        }
        Ok(())
    }
}

impl LuaUserData for ExecuteCtx {
    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("dump", |_, this, ()| Ok(format!("{this}")));
    }
}
