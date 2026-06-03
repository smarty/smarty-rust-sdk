use hyper::http::Extensions;
use reqwest::{Request, Response};
use reqwest_middleware::{Middleware, Next};

pub struct LoggingMiddleware;

#[async_trait::async_trait]
impl Middleware for LoggingMiddleware {
    async fn handle(
        &self,
        req: Request,
        extensions: &mut Extensions,
        next: Next<'_>,
    ) -> reqwest_middleware::Result<Response> {
        log::trace!("Request Started {:?}", req);
        let res = next.run(req, extensions).await;
        log::trace!("Result: {:?}", res);
        res
    }
}
