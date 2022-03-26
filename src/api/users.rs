use super::TwitterApi;
use crate::authorization::Authorization;
use crate::data::User;
use crate::error::Result;
use crate::query::{get_req_builder, ToId, ToQuery};
use reqwest::Method;

get_req_builder! {
pub struct GetUsersRequestBuilder {
    user_fields,
    tweet_fields,
    user_expansions
}
}

impl<A> TwitterApi<A>
where
    A: Authorization,
{
    pub fn get_users(
        &self,
        ids: impl IntoIterator<Item = impl ToId>,
    ) -> Result<GetUsersRequestBuilder<A, Vec<User>, Option<()>>> {
        Ok(GetUsersRequestBuilder::new(
            self,
            self.request(Method::GET, "users")?
                .query(&ids.to_query("ids")),
        ))
    }
    pub fn get_user(&self, id: impl ToId) -> Result<GetUsersRequestBuilder<A, User, Option<()>>> {
        Ok(GetUsersRequestBuilder::new(
            self,
            self.request(Method::GET, &format!("users/{id}"))?,
        ))
    }
    pub fn get_users_by_usernames(
        &self,
        usernames: impl IntoIterator<Item = impl ToString>,
    ) -> Result<GetUsersRequestBuilder<A, Vec<User>, Option<()>>> {
        Ok(GetUsersRequestBuilder::new(
            self,
            self.request(Method::GET, "users/by")?
                .query(&usernames.to_query("usernames")),
        ))
    }
    pub fn get_user_by_username(
        &self,
        username: impl ToString,
    ) -> Result<GetUsersRequestBuilder<A, User, Option<()>>> {
        Ok(GetUsersRequestBuilder::new(
            self,
            self.request(
                Method::GET,
                &format!("users/by/username/{}", username.to_string()),
            )?,
        ))
    }
    pub fn get_users_me(&self) -> Result<GetUsersRequestBuilder<A, User, Option<()>>> {
        Ok(GetUsersRequestBuilder::new(
            self,
            self.request(Method::GET, "users/me")?,
        ))
    }
}