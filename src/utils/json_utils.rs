use errors::Error;
use serde_json::Value;

pub fn get_object<'a>(v: &'a Value, key: &str) -> Result<&'a Value, String> {
    let v = v.as_object()
        .ok_or(Error::JsonParseError)
        .map_err(|e| e.to_string())?
        .get(key)
        .ok_or(Error::KeyNotFound(key.to_owned()))
        .map_err(|e| e.to_string())?;
    Ok(&v)
}

pub fn get_string<'a>(v: &'a Value, key: &str) -> Result<String, String> {
    let result = v.get(key)
        .ok_or(Error::KeyNotFound(key.to_owned()))
        .map_err(|e| e.to_string())?
        .as_str()
        .ok_or(Error::JsonToStringFailed)
        .map_err(|e| e.to_string())?;
    Ok(result.to_owned())
}

pub fn get_string_from_path<'a>(v: &'a Value, keys: &Vec<&str>) -> Result<String, String> {
    let mut obj = v;
    for i in 0 .. keys.len() - 1 {
        obj = get_object(obj, keys[i])?;
    }

    get_string(obj, keys.last().unwrap())
}

pub fn get_object_from_path<'a>(v: &'a Value, keys: &Vec<&str>) -> Result<&'a Value, String> {
    let mut obj = v;
    for key in keys {
        obj = get_object(obj, key)?;
    }

    Ok(obj)
}