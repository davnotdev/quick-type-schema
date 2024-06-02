pub enum Language {
    Typescript,
    JsonSchema,
    CSharp {
        namespace: String,
        framework: Option<CSharpFramework>,
        array_or_list: ArrayOrList,
        number_type: CSharpNumberType,
        any_type: CSharpAnyType,
    },
    Crystal,
    Dart,
    Elm {
        module: Option<String>,
        array_or_list: ArrayOrList,
    },
    Go {
        package: Option<String>,
    },
    Haskell {
        module: Option<String>,
        array_or_list: ArrayOrList,
    },
    Java {
        array_or_list: ArrayOrList,
        package: String,
    },
    Kotlin {
        package: String,
        framework: Option<KotlinFramework>,
    },
    ObjectiveC,
    Python {
        version: PythonVersion,
    },
    Ruby {
        strictness: RubyStrictness,
    },
    Rust {
        derive_debug: bool,
        derive_clone: bool,
    },
    Scala3 {
        package: String,
        framework: Option<Scala3Framework>,
    },
    Smithy {
        package: String,
    },
    Swift {
        struct_or_class: StructOrClass,
        sendable: bool,
        swift5_support: bool,
        objective_c_support: bool,
        mutable_properties: bool,
    },
}

pub enum ArrayOrList {
    Array,
    List,
}

pub enum StructOrClass {
    Struct,
    Class,
}

pub enum CSharpFramework {
    NewtonSoft,
    SystemTextJson,
}

pub enum CSharpNumberType {
    Double,
    Decimal,
}

pub enum CSharpAnyType {
    Object,
    Dynamic,
}

pub enum KotlinFramework {
    Jackson,
    Klaxon,
    Kotlinx,
}

pub enum PythonVersion {
    V3_5,
    V3_6,
    V3_7,
}

pub enum RubyStrictness {
    Strict,
    Coercible,
    None,
}

pub enum Scala3Framework {
    Circe,
    Upickle,
}

impl Language {
    pub fn get_args(&self) -> Vec<&str> {
        match self {
            Language::Typescript => vec!["-l", "typescript", "--just-types"],
            Language::JsonSchema => vec!["-l", "schema"],
            Language::CSharp {
                namespace,
                framework,
                array_or_list,
                number_type,
                any_type,
            } => {
                let mut out = vec!["-l", "cs", "--namespace", &namespace];
                if let Some(framework) = framework {
                    out.push("--framework");
                    out.push(match framework {
                        CSharpFramework::NewtonSoft => "NewtonSoft",
                        CSharpFramework::SystemTextJson => "SystemTextJson",
                    });
                } else {
                    out.push("--features");
                    out.push("just-types-and-namespace");
                }
                out.push("--array-type");
                match array_or_list {
                    ArrayOrList::Array => out.push("array"),
                    ArrayOrList::List => out.push("list"),
                }
                out.push("--number-type");
                match number_type {
                    CSharpNumberType::Double => out.push("double"),
                    CSharpNumberType::Decimal => out.push("decimal"),
                }
                out.push("--any-type");
                match any_type {
                    CSharpAnyType::Object => out.push("object"),
                    CSharpAnyType::Dynamic => out.push("dynamic"),
                }
                out
            }
            Language::Crystal => vec!["-l", "cr"],
            Language::Dart => vec!["-l", "dart", "--just-types"],
            Language::Elm {
                module,
                array_or_list,
            } => {
                let mut out = vec!["-l", "elm"];
                out.push("--array-type");
                match array_or_list {
                    ArrayOrList::Array => out.push("array"),
                    ArrayOrList::List => out.push("list"),
                }
                if let Some(module) = module {
                    out.push("--module");
                    out.push(module);
                } else {
                    out.push("--just-types");
                }
                out
            }

            Language::Go { package } => {
                let mut out = vec!["-l", "go"];
                if let Some(package) = package {
                    out.push("--package");
                    out.push(package);
                    out.push("--just-types-and-package");
                } else {
                    out.push("--just-types");
                }
                out
            }
            Language::Haskell {
                module,
                array_or_list,
            } => {
                let mut out = vec!["-l", "haskell"];
                out.push("--array-type");
                match array_or_list {
                    ArrayOrList::Array => out.push("array"),
                    ArrayOrList::List => out.push("list"),
                }
                if let Some(module) = module {
                    out.push("--module");
                    out.push(module);
                } else {
                    out.push("--just-types");
                }
                out
            }
            Language::Java {
                array_or_list,
                package,
            } => {
                let mut out = vec!["-l", "java", "--package", package];
                out.push("--array-type");
                match array_or_list {
                    ArrayOrList::Array => out.push("array"),
                    ArrayOrList::List => out.push("list"),
                }
                out
            }
            Language::Kotlin { package, framework } => {
                let mut out = vec!["-l", "kotlin", "--package", package];
                if let Some(framework) = framework {
                    match framework {
                        KotlinFramework::Jackson => out.push("jackson"),
                        KotlinFramework::Klaxon => out.push("klaxon"),
                        KotlinFramework::Kotlinx => out.push("kotlinx"),
                    }
                } else {
                    out.push("--just-types")
                }
                out
            }
            Language::ObjectiveC => vec!["-l", "objc", "--just-types"],
            Language::Python { version } => {
                let mut out = vec!["-l", "py"];
                out.push("--python-version");
                match version {
                    PythonVersion::V3_5 => out.push("3.5"),
                    PythonVersion::V3_6 => out.push("3.6"),
                    PythonVersion::V3_7 => out.push("3.7"),
                }
                out
            }
            Language::Ruby { strictness } => {
                let mut out = vec!["-l", "ruby", "--just-types"];
                out.push("--strictness");
                match strictness {
                    RubyStrictness::Strict => out.push("strict"),
                    RubyStrictness::Coercible => out.push("coercible"),
                    RubyStrictness::None => out.push("none"),
                }
                out
            }
            Language::Rust {
                derive_debug,
                derive_clone,
            } => {
                let mut out = vec!["-l", "rs"];
                if *derive_debug {
                    out.push("--derive-debug");
                }
                if *derive_clone {
                    out.push("--derive-clone");
                }
                out
            }
            Language::Scala3 { package, framework } => {
                let mut out = vec!["-l", "scala3", "--package", package];
                if let Some(framework) = framework {
                    out.push("--framework");
                    match framework {
                        Scala3Framework::Circe => out.push("circe"),
                        Scala3Framework::Upickle => out.push("upickle"),
                    }
                } else {
                    out.push("--just-types");
                }
                out
            }
            Language::Smithy { package } => vec!["-l", "Smithy", "--package", package],
            Language::Swift {
                struct_or_class,
                sendable,
                swift5_support,
                objective_c_support,
                mutable_properties,
            } => {
                let mut out = vec!["-l", "swift", "--just-types"];
                out.push("--struct-or-class");
                match struct_or_class {
                    StructOrClass::Struct => out.push("struct"),
                    StructOrClass::Class => out.push("class"),
                }

                if *sendable {
                    out.push("--sendable");
                }

                if *swift5_support {
                    out.push("--swift-5-support");
                }

                if *objective_c_support {
                    out.push("--objective-c-support");
                }

                if *mutable_properties {
                    out.push("--mutable-properties");
                }

                out
            }
        }
    }
}
