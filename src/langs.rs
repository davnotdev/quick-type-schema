#[derive(Debug, Clone, Hash, PartialEq, Eq)]
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
    Dart {
        use_freezed: bool,
    },
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

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum ArrayOrList {
    Array,
    List,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum StructOrClass {
    Struct,
    Class,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum CSharpFramework {
    NewtonSoft,
    SystemTextJson,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum CSharpNumberType {
    Double,
    Decimal,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum CSharpAnyType {
    Object,
    Dynamic,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum PythonVersion {
    V3_5,
    V3_6,
    V3_7,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum RubyStrictness {
    Strict,
    Coercible,
    None,
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
            Language::Dart { use_freezed } => vec![
                "-l",
                "dart",
                "--just-types",
                if *use_freezed {
                    "--use-freezed"
                } else {
                    "--no-use-freezed"
                },
            ],
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

    pub fn name(&self) -> &str {
        match self {
            Language::Typescript => "typescript",
            Language::JsonSchema => "schema",
            Language::CSharp { .. } => "csharp",
            Language::Crystal => "crystal",
            Language::Dart { .. } => "dart",
            Language::Elm { .. } => "elm",
            Language::Go { .. } => "go",
            Language::Haskell { .. } => "haskell",
            Language::Python { .. } => "python",
            Language::Ruby { .. } => "ruby",
            Language::Rust { .. } => "rust",
            Language::Smithy { .. } => "smithy",
            Language::Swift { .. } => "swift",
        }
    }
}
