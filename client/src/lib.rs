use potassium_shot_common::com::*;
use reqwest::Method;
use zeroize::Zeroizing;

pub use potassium_shot_common::types::{LoginToken, UserId};

pub struct PotassiumShotClient {
    client: reqwest::Client,
    pub url: reqwest::Url,
    pub port: u16,
}

impl Default for PotassiumShotClient {
    fn default() -> Self {
        Self::new("https://api.potassium-shot.eu", 443).expect("The url should be correct.")
    }
}

impl PotassiumShotClient {
    pub fn new(url: impl reqwest::IntoUrl, port: u16) -> reqwest::Result<Self> {
        Ok(Self {
            client: reqwest::Client::default(),
            url: url.into_url()?,
            port,
        })
    }

    pub async fn create_user(
        &self,
        username: impl Into<String>,
        password: String,
    ) -> reqwest::Result<UserId> {
        Ok(self
            .request::<_, CreateUserResult>(CreateUserArgs {
                username: username.into(),
                password: Zeroizing::new(password),
            })
            .await?
            .userid)
    }

    pub async fn delete_user(&self, token: LoginToken, password: String) -> reqwest::Result<()> {
        self.request::<_, ()>(DeleteUserArgs {
            token,
            password: Zeroizing::new(password),
        })
        .await
    }

    pub async fn login(
        &self,
        username: impl Into<String>,
        password: String,
        remember: bool,
    ) -> reqwest::Result<LoginToken> {
        Ok(self
            .request::<_, LoginResult>(LoginArgs {
                username: username.into(),
                password: Zeroizing::new(password),
                remember,
            })
            .await?
            .token)
    }

    pub async fn get_userid_from_token(&self, token: LoginToken) -> reqwest::Result<UserId> {
        Ok(self
            .request::<_, GetUserIdFromTokenResult>(GetUserIdFromTokenArgs { token })
            .await?
            .userid)
    }

    pub async fn is_admin(&self, userid: UserId) -> reqwest::Result<bool> {
        Ok(self
            .request::<_, IsAdminResult>(IsAdminArgs { userid })
            .await?
            .is_admin)
    }

    pub async fn promote_user(&self, userid: UserId, token: LoginToken) -> reqwest::Result<()> {
        self.request::<_, ()>(PromoteUserArgs {
            token,
            user_to_promote: userid,
        })
        .await
    }

    async fn request<I: serde::Serialize, O: serde::de::DeserializeOwned>(
        &self,
        input: I,
    ) -> reqwest::Result<O> {
        self.client
            .request(Method::POST, self.url.clone())
            .json(&input)
            .send()
            .await?
            .json::<O>()
            .await
    }
}
