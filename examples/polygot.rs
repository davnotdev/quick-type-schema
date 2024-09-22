use quick_type_schema::*;
use schemars::JsonSchema;

#[derive(JsonSchema)]
pub enum OtherStruct {
    Thing,
    AnotherThing,
}

fn main() {
    let mut gen = CodegenContext::new("MyData", None);
    gen.add_type::<OtherStruct>();

    let languages = &[
        // Language::Typescript(TypescriptOptions {
        //     prefer_unions: true,
        //     ..Default::default()
        // }),
        Language::JsonSchema(Default::default()),
        // Language::CSharp(Default::default()),
        // Language::Crystal(Default::default()),
        Language::Dart(DartOptions {
            part_name: "Hello".to_owned(),
            from_map: true,
            ..Default::default()

        }),
        // Language::Elm(Default::default()),
        // Language::Go(Default::default()),
        // Language::Haskell(Default::default()),
        // Language::Python(Default::default()),
        // Language::Ruby(Default::default()),
        // Language::Rust(Default::default()),
        // Language::Smithy(Default::default()),
        // Language::Swift(Default::default()),
    ];

    for lang in languages {
        eprintln!("--- {}", lang.name());
        eprintln!("{}", gen.finish(lang.clone()));
    }
}
