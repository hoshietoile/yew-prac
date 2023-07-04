use yew::prelude::*;
use gloo_console::log;
use web_sys::{HtmlInputElement};

use crate::components::input::*;

#[function_component(LoginForm)]

pub fn login_form() -> Html {
  let username_handle = use_state(|| String::default());
  let username = (*username_handle).clone();

  let password_handle = use_state(|| String::default());
  let password = (*password_handle).clone();

  let onchange_username = Callback::from(move |e: Event| {
    let target = e.target_dyn_into::<HtmlInputElement>();
    if let Some(input) = target {
      username_handle.set(input.value())
    }
  });

  let onchange_password = Callback::from(move |e: Event| {
    let target = e.target_dyn_into::<HtmlInputElement>();
    if let Some(input) = target {
      password_handle.set(input.value())
    }
  });

  let onsubmit = {
    let username = username.clone();
    let password = password.clone();
    Callback::from(move |e: SubmitEvent| {
      e.prevent_default();
      log!("Submitting_form");
      log!(username.clone());
      log!(password.clone());
    })
  };

  html! {
    <form {onsubmit}>
      <div class="mb-3">
        <Input
          input_type="text"
          name="username"
          label="UserName"
          value={username}
          onchange={onchange_username}
        />
      </div>
      <div class="mb-3">
        <Input
          input_type="password"
          name="password"
          label="Password"
          value={password}
          onchange={onchange_password}
        />
      </div>
      <button type="submit" class="btn btn-primary">{"Login"}</button>
    </form>   
  }
}