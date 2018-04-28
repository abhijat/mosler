use serde_json::Value;

pub fn key_from_object<'a>(v: &'a Value, key: &str) -> &'a Value {
    v.as_object()
        .expect("unable to convert to object")
        .get(key)
        .expect("unable to extract key")
}

pub fn str_from_object(v: &Value, key: &str) -> String {
    v.as_object()
        .expect("unable to convert to object")
        .get(key)
        .expect("unable to extract key")
        .as_str()
        .unwrap()
        .to_owned()
}