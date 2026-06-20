use axum::{
    extract::Request, http::request::Parts, http::Extensions, middleware::Next, response::Response,
};

/// Locale extracted from Accept-Language header, `?locale=`, or `?lang=` query param.
/// Default "en".
#[derive(Clone, Debug)]
pub struct Locale(pub String);

impl Locale {
    pub fn en() -> Self {
        Locale("en".to_string())
    }
    pub fn id() -> Self {
        Locale("id".to_string())
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Implements `FromRequestParts` so handlers can extract locale directly via
/// `Locale(locale): Locale` and forward `&locale` to response helpers.
impl<S> axum::extract::FromRequestParts<S> for Locale
where
    S: Send + Sync,
{
    type Rejection = std::convert::Infallible;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // The middleware below has already inserted the locale; fall back to "en"
        // only if the extractor is used on a route that bypassed the middleware.
        Ok(parts
            .extensions
            .get::<Locale>()
            .cloned()
            .unwrap_or_else(Locale::en))
    }
}

pub async fn locale_middleware(mut req: Request, next: Next) -> Response {
    let locale = extract_locale(&req);
    req.extensions_mut().insert(locale);
    next.run(req).await
}

fn extract_locale(req: &Request) -> Locale {
    // 1. Query param `?locale=id` or `?lang=id` (alias)
    if let Some(query) = req.uri().query() {
        for pair in query.split('&') {
            if let Some((k, v)) = pair.split_once('=') {
                // accept both `locale` and `lang` as the locale key
                if (k == "locale" || k == "lang") && (v == "en" || v == "id") {
                    return Locale(v.to_string());
                }
            }
        }
    }
    // 2. Accept-Language header
    if let Some(header) = req.headers().get("accept-language") {
        if let Ok(val) = header.to_str() {
            let v = val.to_lowercase();
            if v.starts_with("id") {
                return Locale::id();
            }
            if v.starts_with("en") {
                return Locale::en();
            }
        }
    }
    Locale::en()
}

/// Read locale from Axum request extensions.
/// Use in handlers: `let locale = crate::common::locale::from_parts(&parts);`
/// OR call via State + middleware pattern.
///
/// For handler functions, use the `get_locale` helper with the Request object.
pub fn from_extensions(extensions: &Extensions) -> String {
    extensions
        .get::<Locale>()
        .map(|l| l.0.clone())
        .unwrap_or_else(|| "en".to_string())
}
