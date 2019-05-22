use actix_web::{AsyncResponder, FutureResponse, HttpResponse, Json, ResponseError, State, Path};
use futures::future::Future;

use crate::app::AppState;
use crate::register_handler::{RegisterUser, UserData};

pub fn register_user((invitaion_id, user_data, state): (Path<String>, Json<UserData>, State<AppState>)) -> FutureResponse<HttpResponse> {

    let msg = RegisterUser {
        // into_inner() returns the inner string value from Path
        invitation_id: invitaion_id.into_inner(),
        password: user_data.password.clone(),
    };

    state
        .db
        .send(msg)
        .from_err()
        .and_then(|db_response| match db_response {
            Ok(slim_user) => Ok(HttpResponse::Ok().json(slim_user)),
            Err(err) => Ok(err.error_response()),
        })
        .responder()
}
