use actix_service::{Service, Transform};
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::Error;
use futures::future::{ok, Ready};
use std::task::{Context, Poll};

// Use env_logger for robust logging
use env_logger::Builder;
use log::{info, Level, LevelFilter};

pub struct LogRequest;

impl<S, B> Transform<S, ServiceRequest> for LogRequest
    where
        S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
        S::Future: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = LogRequestMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        // Initialize logging (optional, adjust as needed)
        Builder::from_env("/* Provide an empty argument */")
            .filter_level(LevelFilter::Info)
            .init();
        ok(LogRequestMiddleware { service })
    }
}
pub struct LogRequestMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for LogRequestMiddleware<S>
    where
        S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
        S::Future: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = S::Future;

    // No need to modify state in poll_ready, use `&self` receiver
    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    // Change self reference to `&self` (immutable borrow)
    fn call(&self, req: ServiceRequest) -> Self::Future {
        // Use a more informative logging message
        info!("Received request: {:?}, path: {}", req, req.path());
        self.service.call(req)
    }
}
