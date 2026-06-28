use axum::{Json, Router, routing::post};

pub use crate::axum_error::AxumResult;

pub use potassium_shot_common::types::{LoginToken, UserId};

mod axum_error;

pub struct ApiRegister {
    router: Router,
}

pub struct NamedApiRegister {
    base: ApiRegister,
    name: String,
}

pub struct BuiltApiRegister {
    base: NamedApiRegister,
}

#[cfg(feature = "server-impl")]
impl Default for ApiRegister {
    fn default() -> Self {
        Self {
            router: Router::new(),
        }
    }
}

impl ApiRegister {
    pub fn name(self, s: impl Into<String>) -> NamedApiRegister {
        NamedApiRegister {
            base: self,
            name: s.into(),
        }
    }
}

impl NamedApiRegister {
    pub fn register<I: serde::de::DeserializeOwned + Send + 'static, O: serde::Serialize, F, Fut>(
        mut self,
        path: impl AsRef<str>,
        f: F,
    ) -> Self
    where
        F: FnOnce(I) -> Fut + Clone + Send + Sync + 'static,
        Fut: Future<Output = AxumResult<O>> + Send,
    {
        let full_path = format!("/{}{}", self.name, path.as_ref());

        self.base.router = self.base.router.route(
            full_path.as_str(),
            post(async |json: Json<I>| f(json.0).await.map(Json)),
        );

        Self {
            base: self.base,
            name: self.name,
        }
    }

    pub fn build(self) -> BuiltApiRegister {
        BuiltApiRegister { base: self }
    }
}

impl BuiltApiRegister {
    #[cfg(feature = "server-impl")]
    pub fn into_router(self) -> Router {
        self.base.base.router
    }
}
