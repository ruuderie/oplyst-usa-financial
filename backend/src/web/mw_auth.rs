use axum::extract::FromRequestParts;
//use anyhow::Ok;
use axum::{response::Response, http::Request, middleware::Next};
use crate::{Result, Error};
use crate::web::AUTH_TOKEN;
use tower_cookies::Cookies;
use lazy_regex::regex_captures as regex_capture;



pub async fn mw_require_auth<B>(
    cookies: Cookies,
    req: Request<B>, 
    next: Next<B>) -> Result<Response>{
    let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_owned());
    println!("-->> {:<12} - mw_require_auth", "MIDDLEWARE");
    // TODO: Real auth-token parsing & validation
    let (user_id, exp, signature) = auth_token
    .ok_or(Error::AuthFailNoAuthTokenCookie)
    .and_then(parse_token)?;

    Ok(next.run(req).await)
}
impl <S: Send + Sync> FromRequestParts<S> for Ctx {
    type Rejection = Error;

    async
}
// parse a token of format 'user -[userid].[exp].[signature]'
// returns userid, exp, signature
fn parse_token(token: String) -> Result<(u32, String, String)>{
    let (_whole,user_id,exp,signature) = regex_capture!(
        r#"^user-(\d+)\.(.+)\.(.+)"#, 
        &token)
    .ok_or(Error::AuthFailTokenWrongFormat)?;

    let user_id: u32 = 
    user_id
    .parse()
    .map_err(|_| Error::AuthFailTokenWrongFormat)?;

    Ok((user_id, exp.to_string(), signature.to_string()))
}

