use async_trait::async_trait;
use axum::extract::{FromRequestParts, State};
use axum::http::Request;
use axum::http::request::Parts;
use axum::middleware::Next;
use axum::response::Response;
use lazy_regex::regex_captures;
use tower_cookies::{Cookies, Cookie};

use crate::ctx::Ctx;
use crate::model::ModelController;
use crate::web::AUTH_TOKEN;
use crate::error::{Error, Result};

// Implement auth check for crud operations
pub async fn mw_require_auth<B>(
    ctx: Result<Ctx>,
    req: Request<B>, 
    next: Next<B>) -> Result<Response> {
    println!("->> {:<12} - mw_require_auth", "MIDDLEWARE");
    
    ctx?;

    // TODO: Verify token
    

    Ok(next.run(req).await)
}

pub async fn mw_ctx_resolver<B>(
    _mc: State<ModelController>,
    cookies: Cookies,
    mut req: Request<B>,
    next: Next<B>,
) -> Result<Response>
{
    println!("->> {:<12} - mw_ctx_resolver", "MIDDLEWARE");
 
    let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());

    let result_ctx = match auth_token
        .ok_or(Error::AuthFailedAuthTokenCookies)
        .and_then(parse_token)
    {
        // TODO: token component validation
        Ok((user_id, _exp, _sign)) => {
            Ok(Ctx::new(user_id))
        }
        Err(e) => Err(e),
    };

    // Remove the cookie if something went wrong other than NoAuthTokenCookie.
    if result_ctx.is_err()
        && !matches!(result_ctx, Err(Error::AuthFailedAuthTokenCookies))
    {
        cookies.remove(Cookie::named(AUTH_TOKEN));
    }
    
    // Store the ctx_result in the request extension.
    req.extensions_mut().insert(result_ctx);

    Ok(next.run(req).await)
}

// region: --- Ctx Extractor
#[async_trait]
impl<S> FromRequestParts<S> for Ctx 
    where S: Send + Sync, {
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
        println!("->> {:<12} - Ctx", "EXTRACTOR");

        parts
            .extensions
            .get::<Result<Ctx>>()
            .ok_or(Error::AuthFailedCtxNotInrequestExt)?
            .clone()
    }
}

// endregion: --- Ctx Extractor


/// Parse a token of format `user-[user-id].[expiration].[signature]`
/// Returns (user_id, expiration, signature)
fn parse_token(token: String) -> Result<(u64, String, String)> {
    // Use the regex_captures macro to capture the token parts.
    // The regex is defined in the `lazy-regex` crate.   
    let (_whole, user_id, expiration, signature) = regex_captures!(
        r#"^user-(\d+)\.(.+)\.(.+)$"#,
        &token
    ).ok_or(Error::AuthFailedTokenWrongFormat)?;

    let user_id = user_id
        .parse::<u64>()
        .map_err(|_| Error::AuthFailedTokenWrongFormat)?;

    Ok((user_id, expiration.to_string(), signature.to_string()))
}