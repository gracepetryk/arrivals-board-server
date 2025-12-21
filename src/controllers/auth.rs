use axum::response::Redirect;
use http::StatusCode;
use url::Url;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::AppContext;

pub fn router() -> OpenApiRouter<AppContext> {
    return OpenApiRouter::new().routes(routes!(login));
}

#[utoipa::path(get, path = "/login", responses(
        (status = FOUND, description = "redirects to auth0 login"),
))]
async fn login() -> Result<Redirect, StatusCode> {
    let auth0_client_id = env!("AUTH0_CLIENT_ID");
    let auth0_domain = env!("AUTH0_DOMAIN");

    let redirect_uri = Url::parse_with_params(
        format!("{auth0_domain}/authorize").as_str(),
        &[
            ("response_type", "code"),
            ("client_id", auth0_client_id),
            // TODO: don't use localhost here lol
            ("redirect_uri", "https://localhost:8080/auth/authorized"),
            // TODO: actual state implementation
            ("state", "state"),
        ],
    );

    match redirect_uri {
        Ok(uri) => Ok(Redirect::to(uri.to_string().as_str())),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
