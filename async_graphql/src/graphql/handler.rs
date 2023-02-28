use super::schema_with::Schema;
use axum::extract::FromRequest;
use axum::{
    http::{Request, StatusCode},
    response::Response,
};

#[derive(thiserror::Error, Debug)]
enum BadRequest {
    #[error(transparent)]
    Source(#[from] async_graphql::ParseRequestError),
}

impl axum::response::IntoResponse for BadRequest {
    fn into_response(self) -> Response {
        StatusCode::BAD_REQUEST.into_response()
    }
}

pub async fn graphql_handler(
    schema: axum::extract::Extension<Schema>,
    request: Request<axum::body::Body>,
) -> Result<async_graphql_axum::GraphQLResponse, StatusCode> {
    let (request_parts, body) = request.into_parts();

    let graphql_request = async_graphql_axum::GraphQLRequest::<BadRequest>::from_request(
        Request::from_parts(request_parts, body),
        &(),
    )
    .await
    .map_err(|_rejection| StatusCode::BAD_REQUEST)?;

    let request = graphql_request.into_inner();
    let response = schema.execute(request).await.into();
    Ok(response)
}
