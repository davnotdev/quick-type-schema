use quick_type_schema::*;
use schemars::JsonSchema;

#[derive(JsonSchema)]
pub struct MyStruct {
    _a: String,
    _b: Option<String>,
    _c: i32,
    _d: OtherStruct,
    _e: (),
}

#[derive(JsonSchema)]
pub struct OtherStruct {
    _a: String,
}

fn main() {
    let mut gen = CodegenContext::new(None);
    gen.add_type::<MyStruct>();
    gen.add_type::<OtherStruct>();

    let languages = &[
        Language::Typescript,
        Language::JsonSchema,
        Language::CSharp {
            namespace: "polygot".to_owned(),
            framework: Some(CSharpFramework::SystemTextJson),
            array_or_list: ArrayOrList::Array,
            number_type: CSharpNumberType::Double,
            any_type: CSharpAnyType::Object,
        },
        Language::CSharp {
            namespace: "polygot".to_owned(),
            framework: None,
            array_or_list: ArrayOrList::Array,
            number_type: CSharpNumberType::Double,
            any_type: CSharpAnyType::Object,
        },
        Language::Crystal,
        Language::Dart { use_freezed: false },
        Language::Elm {
            module: Some("polygot".to_owned()),
            array_or_list: ArrayOrList::Array,
        },
        Language::Elm {
            module: None,
            array_or_list: ArrayOrList::Array,
        },
        Language::Go {
            package: Some("polygot".to_owned()),
        },
        Language::Go { package: None },
        Language::Haskell {
            module: Some("polygot".to_owned()),
            array_or_list: ArrayOrList::Array,
        },
        Language::Haskell {
            module: None,
            array_or_list: ArrayOrList::Array,
        },
        Language::Python {
            version: PythonVersion::V3_7,
        },
        Language::Ruby {
            strictness: RubyStrictness::Strict,
        },
        Language::Rust {
            derive_debug: true,
            derive_clone: true,
        },
        Language::Smithy {
            package: "polygot".to_owned(),
        },
        Language::Swift {
            struct_or_class: StructOrClass::Class,
            sendable: false,
            swift5_support: false,
            objective_c_support: false,
            mutable_properties: false,
        },
    ];

    for lang in languages {
        eprintln!("--- {}", lang.name());
        eprintln!("{}", gen.finish(lang.clone()));
    }
}
