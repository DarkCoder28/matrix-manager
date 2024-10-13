use std::
    sync::{Arc, Mutex}
;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::{spawn_local, JsFuture};
use web_sys::{Request, RequestInit, Response};

pub(crate) fn get<T>(endpoint: &str, data: Arc<Mutex<T>>)
where
    T: serde::de::Deserialize<'static> + 'static,
{
    let window = web_sys::window().expect("No window");
    let opts = RequestInit::new();
    opts.set_method("GET");
    //
    let req = Request::new_with_str_and_init(endpoint, &opts).unwrap();
    let resp = wasm_bindgen_futures::JsFuture::from(window.fetch_with_request(&req));
    let data_clone = data.clone();
    spawn_local(async move {
        let resp_value = resp.await.unwrap();
        let resp: Response = resp_value.dyn_into().unwrap();
        let text = JsFuture::from(resp.text().unwrap())
            .await
            .unwrap()
            .as_string()
            .unwrap();
        let text: &'static str = Box::leak(text.into_boxed_str());
        let new_data = serde_json::from_str::<T>(text).unwrap();
        let mut data = data_clone.lock().unwrap();
        *data = new_data.into();
    });
}
