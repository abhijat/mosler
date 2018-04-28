use serde_json::Value;

pub struct JsonExtractor<'a> {
    pub value_ref: &'a Value
}

impl<'a> JsonExtractor<'a> {
    fn cast_error(&self, cast_type: &str) -> String {
        format!("unable to cast {} to type {}", self.value_ref, cast_type)
    }

    fn extract_error(&self, key: &str) -> String {
        format!("unable to extract key {} from {}", key, self.value_ref)
    }

    pub fn new(v: &'a Value) -> Self {
        JsonExtractor { value_ref: v }
    }

    pub fn get_value(&self, key: &str) -> JsonExtractor {
        let v = self.value_ref
            .as_object()
            .expect(&self.cast_error("object"))
            .get(key)
            .expect(&self.extract_error(key));

        JsonExtractor { value_ref: v }
    }

    pub fn get_str(&self, key: &str) -> String {
        self.value_ref
            .get(key)
            .expect(&self.extract_error(key))
            .as_str()
            .expect(&self.cast_error("string"))
            .to_owned()
    }
}