use gloo_net::http::Request;

pub fn api_login(username: String, password: String) {
  let response = Request::post(format!("{}/login". APP_HOST))
    .json(json!({
      "username": username,
      "password": password,
    }))
}