use actix_web::{
    Error, HttpResponse, HttpResponseBuilder, Responder, body,
    http::{
        StatusCode,
        header::{self, HeaderValue},
    },
    mime,
};
use bytes::{BufMut, BytesMut};
use serde::Serialize;

pub type Response<T> = Result<CustomResponse<T>, Error>;

#[derive(Debug)]
pub struct CustomResponse<T: Serialize> {
    pub body: Option<T>,
    pub status_code: StatusCode,
    pub pagination: Option<Pagination>,
}

/// Builder for [`CustomResponse`] values.
///
/// This type follows the builder pattern and allows configuring the
/// HTTP status code, optional response body, and optional pagination
/// metadata before constructing a [`CustomResponse`].
///
/// Typical usage:
/// ```rust,ignore
/// let response = CustomResponseBuilder::new()
///     .status_code(StatusCode::OK)
///     .body(my_body)
///     .build();
/// ```
pub struct CustomResponseBuilder<T: Serialize> {
    pub body: Option<T>,
    pub status_code: StatusCode,
    pub pagination: Option<Pagination>,
}

#[derive(Debug)]
pub struct Pagination {
    pub count: i64,
    pub offset: i64,
    pub limit: i32,
}

impl Pagination {
    fn add_headers(&self, response: &mut HttpResponseBuilder) {
        response.append_header(("X-Pagination-Count", self.count.to_string()));
        response.append_header(("X-Pagination-Offset", self.offset.to_string()));
        response.append_header(("X-Pagination-Limit", self.limit.to_string()));
    }
}

impl<T> Default for CustomResponseBuilder<T>
where
    T: Serialize,
{
    fn default() -> Self {
        Self {
            body: None,
            status_code: StatusCode::OK,
            pagination: None,
        }
    }
}

impl<T> CustomResponseBuilder<T>
where
    T: Serialize,
{
    pub fn new() -> Self {
        Self::default()
    }

    pub fn body(mut self, body: T) -> Self {
        self.body = Some(body);
        self
    }

    pub fn status_code(mut self, status_code: StatusCode) -> Self {
        self.status_code = status_code;
        self
    }

    pub fn pagination(mut self, pagination: Pagination) -> Self {
        self.pagination = Some(pagination);
        self
    }

    pub fn build(self) -> CustomResponse<T> {
        CustomResponse {
            body: self.body,
            status_code: self.status_code,
            pagination: self.pagination,
        }
    }
}

impl<T> Responder for CustomResponse<T>
where
    T: Serialize,
{
    type Body = body::BoxBody;

    fn respond_to(self, _req: &actix_web::HttpRequest) -> HttpResponse<Self::Body> {
        let mut response = HttpResponse::build(self.status_code);

        if let Some(ref pagination) = self.pagination {
            pagination.add_headers(&mut response);
        }

        let body = match self.body {
            Some(body) => body,
            None => return response.finish(),
        };

        let mut bytes = BytesMut::new().writer();
        if let Err(err) = serde_json::to_writer(&mut bytes, &body) {
            eprintln!("Failed to serialize response body: {}", err); // TODO: Use proper logging
            return HttpResponse::InternalServerError().finish();
        }

        let bytes = bytes.into_inner().freeze();
        response.insert_header((
            header::CONTENT_TYPE,
            HeaderValue::from_static(mime::APPLICATION_JSON.as_ref()),
        ));
        response.body(bytes)
    }
}
