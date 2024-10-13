use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::spawn_local;
use web_sys::{Headers, Request, RequestInit, Response};

pub(crate) fn post<T>(endpoint: &str, data: &T)
where
    T: serde::ser::Serialize,
{
    let window = web_sys::window().expect("No window");
    let opts = RequestInit::new();
    opts.set_method("POST");
    let headers = Headers::new().unwrap();
    let _ = headers.append("Content-Type", "application/json");
    opts.set_headers(&headers);
    opts.set_body(&JsValue::from_str(&serde_json::to_string(data).unwrap()));
    //
    let req = Request::new_with_str_and_init(endpoint, &opts).unwrap();
    let resp = wasm_bindgen_futures::JsFuture::from(window.fetch_with_request(&req));
    spawn_local(async move {
        let resp_value = resp.await.unwrap();
        let resp: Response = resp_value.dyn_into().unwrap();
        if resp.ok() {
            log::info!("Posted Data Successfully");
        } else {
            log::error!("Failed to Post Data");
        }
    });
}
