use xitca_web::{
    WebContext,
    error::Error,
    handler::FromRequest,
    http::HeaderName,
};

use super::XitcaError::BadRequest;

#[derive(Debug)]
pub struct Signature<'a> {
    pub key: &'a str,
}

impl<'a, 'r, C, B> FromRequest<'a, WebContext<'r, C, B>> for Signature<'a> {
    type Type<'b> = Signature<'b>;
    type Error = Error;

    async fn from_request(ctx: &'a WebContext<'r, C, B>) -> Result<Self, Self::Error> {
        ctx.req()
            .headers()
            .get(&HeaderName::from_static("x-line-signature"))
            .map(|value| match value.to_str() {
                Ok(key) => Ok(Signature { key }),
                Err(_) => Err(BadRequest::new("Invalid header value").into()),
            })
            .ok_or_else(|| BadRequest::new("Can't find specific header"))?
    }
}
