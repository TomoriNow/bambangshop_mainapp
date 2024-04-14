use std::thread;

use bambangshop::{Result, compose_error_response};
use rocket::http::Status;
use crate::model::product::Product;
use crate::repository::product::ProductRepository;