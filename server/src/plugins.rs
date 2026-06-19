use std::ffi::OsStr;

use axum::Router;
use potassium_shot_plugin::ApiRegister;

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

#[derive(Default)]
pub struct Plugins {
    router: Router,
    libs: Vec<(String, libloading::Library)>,
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
                let built_register = register_function(register);
                router = router.merge(built_register.into_router());
                libs.push((name.clone().to_string(), lib));
                tracing::info!("Loaded plugin '{}'.", name);
            }
        }

        Self { router, libs }
    }

    pub fn patch_router(&self, router: Router) -> Router {
        router.merge(self.router.clone())
    }

    pub fn init_all(&self) {
        for (name, lib) in &self.libs {
            let maybe_init_fn = unsafe { lib.get::<fn()>(b"init") };

            let Ok(init_fn) = maybe_init_fn else {
                tracing::error!(
                    "Could not find function `init()` in plugin `{}`. Make sure it is marked with #[unsafe(no_mangle)].",
                    name
                );
                continue;
            };

            init_fn();
        }
    }
}
