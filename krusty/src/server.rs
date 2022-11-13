use crate::tasks::Answers;
use iron::{error, status, Iron, IronResult, Request, Response};
use router::Router;
use serde_json::to_string;
use std::env;

pub struct Server {
    router: Router,
}

impl Server {
    pub fn new(answers: Answers) -> Self {
        let mut router = Router::new();
        let mut answers_clone = answers.clone();

        router.get(
            "/",
            move |_: &mut Request| -> IronResult<Response> {
                Ok(Response::with((
                    status::Ok,
                    to_string(&(answers_clone)).unwrap(),
                )))
            },
            "all",
        );
        answers_clone = answers.clone();
        router.get(
            "/first",
            move |_: &mut Request| -> IronResult<Response> {
                Ok(Response::with((
                    status::Ok,
                    to_string(&(answers_clone.first)).unwrap(),
                )))
            },
            "1",
        );
        answers_clone = answers.clone();
        router.get(
            "/second",
            move |_: &mut Request| -> IronResult<Response> {
                Ok(Response::with((
                    status::Ok,
                    to_string(&(answers_clone.second)).unwrap(),
                )))
            },
            "2",
        );
        answers_clone = answers.clone();
        router.get(
            "/third",
            move |_: &mut Request| -> IronResult<Response> {
                Ok(Response::with((
                    status::Ok,
                    to_string(&(answers_clone.third)).unwrap(),
                )))
            },
            "3",
        );
        answers_clone = answers.clone();
        router.get(
            "/forth",
            move |_: &mut Request| -> IronResult<Response> {
                Ok(Response::with((
                    status::Ok,
                    to_string(&(answers_clone.forth)).unwrap(),
                )))
            },
            "4",
        );
        answers_clone = answers.clone();
        router.get(
            "/fifth",
            move |_: &mut Request| -> IronResult<Response> {
                Ok(Response::with((
                    status::Ok,
                    to_string(&(answers_clone.fifth)).unwrap(),
                )))
            },
            "5",
        );

        Self {
            router: router,
        }
    }

    pub fn start(self) -> Result<iron::Listening, error::HttpError> {
        let port = env::var("PORT").expect("PORT must be set");

        Iron::new(self.router).http(
            format!("0.0.0.0:{}", port)
        )
    }
}
