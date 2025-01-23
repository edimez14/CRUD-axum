use axum::{
    routing::{get, post, put, delete},
    Router,
    middleware,
};
use std::sync::Arc;
use crate::{
    views::{
        root,
        get_users,
        view_users,
        put_user,
        delete_users,
    },
    auth::{
        register_user,
        login_user,
        auth_middleware,
    },
    AppState,
};


pub fn route(app_state: Arc<AppState>) -> Router {
    Router::new()
            .route("/", get(root))
            .route("/users/register", post(register_user))
            .route("/users/login", post(login_user))
            .route("/users", get(get_users).route_layer(middleware::from_fn(auth_middleware)))
            .route("/users/{id}", get(view_users).route_layer(middleware::from_fn(auth_middleware)))
            .route("/users/put/{id}", put(put_user).route_layer(middleware::from_fn(auth_middleware)))
            .route("/users/delete/{id}", delete(delete_users).route_layer(middleware::from_fn(auth_middleware)))
            .with_state(app_state)
}