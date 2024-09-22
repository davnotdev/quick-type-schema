use json::{object, JsonValue};

#[cfg(feature = "add_type")]
use schemars::{schema_for, JsonSchema};

mod cli_builder;
mod langs;

use cli_builder::CliBuilder;
pub use langs::*;

#[derive(Debug, Clone)]
pub struct CodegenContext {
    base_name: String,
    schema: Schema,
    override_quicktype_args: Option<Vec<String>>,
}

impl CodegenContext {
    pub fn new(base_name: &str, override_quicktype_args: Option<&[&str]>) -> Self {
        let schema = Schema {
            current_num: 0,
            final_val: object! {
                "$schema": "http://json-schema.org/draft-07/schema#",
                "type": "object",
                "definitions": {

                },
            },
            raw_schemas: vec![],
        };
        CodegenContext {
            base_name: base_name.to_owned(),
            schema,
            override_quicktype_args: override_quicktype_args.map(|override_quicktype_args| {
                override_quicktype_args
                    .iter()
                    .map(|s| s.to_string())
                    .collect()
            }),
        }
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
        let args = lang.get_args();
        let proc_id = std::process::id();

        let mut out_path = std::env::temp_dir();
        out_path.push(format!("quick-type-code-{}-{}", lang.name(), proc_id));

        let mut schema_path = std::env::temp_dir();
        schema_path.push(format!("quick-type-schema-{}.json", proc_id));

        std::fs::write(&schema_path, self.schema.final_val.to_string()).unwrap();

        let mut quicktype_args = [
            "--quiet",
            "-t",
            &self.base_name,
            "-o",
            out_path.to_str().unwrap(),
            "--src-lang",
            "schema",
            schema_path.to_str().unwrap(),
        ]
        .into_iter()
        .map(|s| s.to_owned())
        .collect::<Vec<_>>();
        if let Some(overrides) = self.override_quicktype_args.as_ref() {
            quicktype_args.append(&mut overrides.clone());
        } else {
            quicktype_args.append(&mut args.to_vec());
        }

        let cmd = if std::process::Command::new("quicktype")
            .arg("--version")
            .output()
            .is_ok()
        {
            "quicktype"
        } else if std::process::Command::new("npx")
            .arg("--version")
            .output()
            .is_ok()
        {
            quicktype_args.insert(0, "quicktype".to_owned());
            "npx"
        } else {
            panic!("Neither `quicktype` and `npx` are in $PATH")
        };

        let out = String::from_utf8(
            std::process::Command::new(cmd)
                .args(quicktype_args.iter())
                .output()
                .unwrap()
                .stderr,
        )
        .unwrap();

        if !out.is_empty() {
            panic!("quicktype {}", out);
        }

        if !out_path.exists() {
            panic!("Error: quicktype generated an unexpected noutput");
        }

        let output = std::fs::read_to_string(&out_path).unwrap();
        let _ = std::fs::remove_file(out_path);
        let _ = std::fs::remove_file(schema_path);
        output
    }
}

#[derive(Debug, Clone)]
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
