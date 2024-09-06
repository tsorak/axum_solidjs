pub mod res {
    use std::fmt::Debug;

    use axum::{
        body::Body,
        http::StatusCode,
        response::{IntoResponse, Response},
    };

    use serde::Serialize;

    // body only contains props
    pub fn json<T: Serialize>(status_code: u16, body: T) -> Response {
        let mut res = axum::Json(body).into_response();
        *res.status_mut() = StatusCode::from_u16(status_code).unwrap();
        res
    }

    pub fn text(status_code: u16, value: &str) -> Response {
        let mut res = Body::from(value.to_string()).into_response();
        *res.status_mut() = StatusCode::from_u16(status_code).unwrap();
        res
    }

    #[derive(Debug, Serialize)]
    pub struct Json {
        uid: Option<String>,
        message: String,
    }

    impl Json {
        pub fn new(message: &str) -> Self {
            Self {
                uid: None,
                message: message.to_string(),
            }
        }
        pub fn with_uid(mut self, uid: &str) -> Self {
            let _ = self.uid.insert(uid.to_string());
            self
        }
        pub fn with_props<T: Serialize + Debug>(self, props: T) -> JsonWithProps<T> {
            JsonWithProps::from_json(self, props)
        }
        pub fn with_message(mut self, message: &str) -> Self {
            self.message = message.to_string();
            self
        }
        pub fn status(self, status_code: u16) -> Response {
            let mut res = axum::Json(self).into_response();
            *res.status_mut() = StatusCode::from_u16(status_code).unwrap();
            res
        }
    }

    impl Default for Json {
        fn default() -> Self {
            Self {
                uid: None,
                message: "Hello World!".to_string(),
            }
        }
    }

    #[derive(Debug, Serialize)]
    pub struct JsonWithProps<T: Serialize + Debug> {
        uid: Option<String>,
        message: String,
        props: T,
    }

    impl<T: Serialize + Debug> JsonWithProps<T> {
        pub fn status(self, status_code: u16) -> Response {
            let mut res = axum::Json(self).into_response();
            *res.status_mut() = StatusCode::from_u16(status_code).unwrap();
            res
        }

        pub fn from_json(mut json: Json, props: T) -> Self {
            Self {
                uid: json.uid.take(),
                message: json.message,
                props,
            }
        }
    }
}
