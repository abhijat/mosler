macro_rules! try_get_json {
    ($e:expr) => {
        $e.map_err(|e| e.to_string())?
            .json::<Value>()
            .map_err(|e| e.to_string())
    };
}