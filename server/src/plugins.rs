use std::{ffi::OsStr, sync::Arc};

use axum::Router;
use potassium_shot_plugin::{ApiRegister, UserId};

macro_rules! call_fn_impl {
    ($name: ident $($arg_name: ident : $arg_type: ident),*) => {
        fn $name<$($arg_type),*>(&self, fn_name: &str, $($arg_name: $arg_type),*)
        where $(
            $arg_type: Clone
        ),*
        {
            for (name, lib) in &self.libs {
                let maybe_fn = unsafe { lib.get::<fn($($arg_type),*)>(fn_name) };

                let mut fn_decl = fn_name.to_string();
                fn_decl.push('(');

                $(
                    fn_decl.push_str(std::any::type_name::<$arg_type>());
                    fn_decl.push_str(", ");
                )*

                fn_decl.pop();
                fn_decl.pop();
                fn_decl.push(')');

                let Ok(r#fn) = maybe_fn else {
                    tracing::error!(
                        "Could not find function `{}` in plugin `{}`. Make sure it is marked with #[unsafe(no_mangle)].",
                        fn_decl,
                        name
                    );
                    continue;
                };

                catch_unwind_and_log(name, fn_decl.as_str(), std::panic::AssertUnwindSafe(|| r#fn($($arg_name.clone()),*)));
            }
        }
    };
}

#[cfg(any(
    target_os = "linux",
    target_os = "freebsd",
    target_os = "openbsd",
    target_os = "netbsd"
))]
const EXTENSION: &str = "so";

#[cfg(target_os = "windows")]
const EXTENSION: &str = "dll";

#[cfg(target_os = "macos")]
const EXTENSION: &str = "dylib";

#[derive(Default, Clone)]
pub struct Plugins {
    router: Router,
    libs: Vec<(String, Arc<libloading::Library>)>,
}

impl Plugins {
    pub fn load() -> Self {
        let mut router = Router::default();
        let mut libs = Vec::new();

        let plugins_dir = crate::env::PLUGINS_PATH.get();
        let Ok(read_dir) = std::fs::read_dir(plugins_dir.as_ref()) else {
            return Self::default();
        };

        for entry in read_dir {
            let Ok(entry) = entry else {
                tracing::error!(
                    "Couldn't read an entry in the plugins directory ({}).",
                    &plugins_dir
                );
                continue;
            };

            let path = entry.path();

            let name = path
                .file_stem()
                .map(|s| s.to_string_lossy())
                .unwrap_or_default();

            #[cfg(target_family = "unix")]
            let name = name.trim_start_matches("lib").to_string();

            if path.extension() == Some(OsStr::new(EXTENSION)) {
                let maybe_lib = unsafe { libloading::Library::new(path.as_path()) };

                let Ok(lib) = maybe_lib else {
                    tracing::error!("Could not load plugin at {}.", path.to_string_lossy());
                    continue;
                };

                let maybe_register_function = unsafe {
                    lib.get::<fn(
                        potassium_shot_plugin::ApiRegister,
                    ) -> potassium_shot_plugin::BuiltApiRegister>(
                        b"register_api"
                    )
                };

                let Ok(register_function) = maybe_register_function else {
                    tracing::error!(
                        "Could not find function `register_api(ApiRegister) -> BuiltApiRegister` in plugin `{}`. Make sure it is marked with #[unsafe(no_mangle)].",
                        name
                    );
                    continue;
                };

                let register = ApiRegister::default();
                let maybe_built_register = catch_unwind_and_log(
                    name.as_str(),
                    "register_api(ApiRegister) -> BuiltApiRegister",
                    std::panic::AssertUnwindSafe(|| register_function(register)),
                );

                let Some(built_register) = maybe_built_register else {
                    continue;
                };

                router = router.merge(built_register.into_router());
                libs.push((name.clone().to_string(), Arc::new(lib)));
                tracing::info!("Loaded plugin '{}'.", name);
            }
        }

        Self { router, libs }
    }

    pub fn patch_router(&self, router: Router) -> Router {
        router.merge(self.router.clone())
    }

    pub fn init_all(&self) {
        self.call_func("init");
    }

    pub fn user_deleted(&self, id: UserId) {
        self.call_func_1("user_deleted", id);
    }

    call_fn_impl!(call_func);
    call_fn_impl!(call_func_1 arg1: Arg1);
}

fn catch_unwind_and_log<R>(
    plugin_name: &str,
    function_name: &str,
    f: impl FnOnce() -> R + std::panic::UnwindSafe,
) -> Option<R> {
    match std::panic::catch_unwind(f) {
        Ok(v) => Some(v),
        Err(payload) => {
            let msg = if let Some(s) = payload.downcast_ref::<&'static str>() {
                *s
            } else if let Some(s) = payload.downcast_ref::<String>() {
                s.as_str()
            } else {
                "<non-printable panic payload>"
            };

            tracing::error!(
                "Plugin `{}` panicked in function `{}`:\n{}",
                plugin_name,
                function_name,
                msg
            );

            None
        }
    }
}
