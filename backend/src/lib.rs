#![allow(clippy::derive_partial_eq_without_eq, clippy::get_first)]
#![deny(unused_crate_dependencies)]
#![warn(clippy::used_underscore_binding)]
#![forbid(unsafe_code)]

mod handlers;

pub struct MySelf {
    pub ud: ft_sdk::UserData,
    pub conn: ft_sdk::Connection,
}

impl ft_sdk::FromRequest for MySelf {
    fn from_request(req: &http::Request<serde_json::Value>) -> Result<Self, ft_sdk::Error> {
        let cookie: ft_sdk::Cookie<{ ft_sdk::auth::SESSION_KEY }> =
            ft_sdk::FromRequest::from_request(req)?;
        let mut conn = ft_sdk::FromRequest::from_request(req)?;
        let ud = match ft_sdk::auth::ud(cookie, &mut conn)? {
            Some(v) => v,
            None => return Err(ft_sdk::unauthorised!("Not logged in").into()),
        };

        Ok(MySelf {
            ud,
            conn,
        })
    }
}
