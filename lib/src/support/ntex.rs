use ntex::{
    http::Payload,
    web::{
        ErrorRenderer, FromRequest, HttpRequest,
        error::{self, ErrorBadRequest},
    },
};

#[derive(Debug)]
pub struct Signature {
    pub key: String,
}

impl<Err: ErrorRenderer> FromRequest<Err> for Signature {
    type Error = error::Error;

    async fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Result<Self, Self::Error> {
        if let Some(x_line_signature) = req.headers().get("x-line-signature") {
            if let Ok(key) = String::from_utf8(x_line_signature.as_bytes().to_vec()) {
                Ok(Signature { key })
            } else {
                Err(ErrorBadRequest("x-line-signature is missing").into())
            }
        } else {
            Err(ErrorBadRequest("x-line-signature is missing").into())
        }
    }
}
