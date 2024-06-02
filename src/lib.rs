use json::{object, JsonValue};

#[cfg(feature = "add_type")]
use schemars::{schema_for, JsonSchema};

mod langs;

pub enum Language {
    Typescript,
}

pub struct CodegenContext {
    schema: Schema,
}

impl CodegenContext {
    pub fn new() -> Self {
        let schema = Schema {
            current_num: 0,
            final_val: object! {
                "$schema": "http://json-schema.org/draft-07/schema#",
                "type": "object",
                "definitions": {},
                "properties": {},
            },
            raw_schemas: vec![],
        };
        CodegenContext { schema }
    }

    #[cfg(feature = "add_type")]
    pub fn add_type<T: JsonSchema>(&mut self) {
        self.schema
            .push_schema_str(&serde_json::to_string(&schema_for!(T)).unwrap());
    }

    pub fn add_schema(&mut self, schema: &str) {
        self.schema.push_schema_str(schema);
    }

    pub fn finish(&self, lang: Language) -> String {
        let args = match lang {
            Language::Typescript => langs::TYPESCRIPT_ARGS,
        };
        let proc_id = std::process::id();

        let mut out_path = std::env::temp_dir();
        out_path.push(format!("quick-type-code-{}", proc_id));

        let mut schema_path = std::env::temp_dir();
        schema_path.push(format!("quick-type-schema-{}.json", proc_id));

        std::fs::write(&schema_path, self.schema.final_val.to_string()).unwrap();

        let mut quicktype_args = vec![
            "quicktype",
            "-o",
            &out_path.to_str().unwrap(),
            "--src-lang",
            "schema",
            &schema_path.to_str().unwrap(),
        ];
        quicktype_args.append(&mut args.to_vec());

        std::process::Command::new("npx")
            .args(args.iter())
            .output()
            .unwrap();
        let output = std::fs::read_to_string(&out_path).unwrap();
        let _ = std::fs::remove_file(out_path);
        let _ = std::fs::remove_file(schema_path);
        output
    }
}

impl Default for CodegenContext {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Endpoint {
    pub fn_name: &'static str,
    pub endpoint: &'static str,
    pub in_ty: &'static str,
    pub out_ty: &'static str,
}

pub struct Schema {
    current_num: usize,
    final_val: JsonValue,
    raw_schemas: Vec<(String, String)>,
}

impl Schema {
    pub fn push_schema_str(&mut self, s: &str) {
        let val = json::parse(s).unwrap();
        let title = val["title"].as_str().unwrap();

        self.raw_schemas.push((title.to_owned(), s.to_owned()));

        self.final_val["definitions"][title] = val.clone();
        self.final_val["properties"][format!("t{}", self.current_num)] = object! {
            "$ref": format!("#/definitions/{}", title)
        };
        let JsonValue::Object(val) = val else {
            unreachable!()
        };
        if let Some(definitions) = val.get("definitions") {
            for (k, v) in definitions.entries() {
                self.final_val["definitions"][k] = v.clone();
            }
        }
        self.current_num += 1;
    }
}
