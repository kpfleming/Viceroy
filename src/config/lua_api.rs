use crate::config::Backend;
use mlua::prelude::*;
use std::sync::Arc;

impl LuaUserData for Backend {
    fn add_fields<F: mlua::UserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("health", |_, this| Ok(this.health()));
        fields.add_field_method_set("health", |_, this, val| {
            this.set_health(val)
                .map_err(|e| mlua::Error::ExternalError(Arc::new(e)))
        });

        fields.add_field_method_get("override_host", |_, this| Ok(this.override_host()));
        fields.add_field_method_set("override_host", |_, this, val| {
            this.set_override_host(val)
                .map_err(|e| mlua::Error::ExternalError(Arc::new(e)))
        });
    }
}
