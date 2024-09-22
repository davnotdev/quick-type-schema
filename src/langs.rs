use super::*;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Language {
    Typescript(TypescriptOptions),
    JsonSchema(JsonSchemaOptions),
    CSharp(CSharpOptions),
    Crystal(CrystalOptions),
    Dart(DartOptions),
    Elm(ElmOptions),
    Go(GoOptions),
    Haskell(HaskellOptions),
    Python(PythonOptions),
    Ruby(RubyOptions),
    Rust(RustOptions),
    Smithy(SmithyOptions),
    Swift(SwiftOptions),
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Default)]
pub struct TypescriptOptions {
    /// Interfaces only
    pub just_types: bool,
    /// Transform property names to be JavaScripty
    pub nice_property_names: bool,
    /// Explicitly name unions
    pub explicit_unions: bool,
    /// Use union type instead of enum
    pub prefer_unions: bool,
    /// Use types instead of interfaces
    pub prefer_types: bool,
    /// Use string instead of enum for string enums with single value
    pub prefer_const_values: bool,
    /// Use readonly type members
    pub readonly: bool,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Default)]
pub struct JsonSchemaOptions {}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct CSharpOptions {
    /// Serialization framework
    pub framework: CSharpFramework,
    /// Use T[] or List<T>
    pub array_type: ArrayOrList,
    /// Property density
    pub density: Density,
    /// Generated namespace
    pub namespace: String,
    /// C# version
    pub csharp_version: CSharpVersion,
    /// Generate virtual properties
    pub r#virtual: bool,
    /// Type to use for "any"
    pub any_type: CSharpAnyType,
    /// Type to use for numbers
    pub number_type: CSharpNumberType,
    /// Output features
    pub features: CSharpFeatures,
    /// Base class
    pub base_class: CSharpBaseClass,
    /// Fail if required properties are missing
    pub check_required: bool,
    /// Keep original field name generate
    pub keep_property_name: bool,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Default)]
pub struct CrystalOptions {}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct DartOptions {
    /// Null Safety
    pub null_safety: bool,
    /// Types only
    pub just_types: bool,
    /// Put encoder & decoder in Class
    pub coders_in_class: bool,
    /// Use method names fromMap() & toMap()
    pub from_map: bool,
    /// Make all properties required
    pub required_props: bool,
    /// Generate CopyWith method
    pub copy_with: bool,
    /// Generate class definitions with @freezed compatibility
    pub use_freezed: bool,
    /// Generate annotations for Hive type adapters
    pub use_hive: bool,
    /// Generate annotations for json_serializable
    pub use_json_annotation: bool,
    /// Use this name in `part` directive
    pub part_name: String,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct ElmOptions {
    /// Plain types only
    pub just_types: bool,
    /// Use Array or List
    pub array_type: ArrayOrList,
    /// Generated module name
    pub module: String,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct GoOptions {
    /// Plain types only
    pub just_types: bool,
    /// Plain types with package only
    pub just_types_and_package: bool,
    /// Generated package name
    pub package: String,
    // Exposing this attribute breaks codegen.
    // multi_file_output: bool,
    /// list of tags which should be generated for fields
    pub field_tags: String,
    /// If set, all non-required objects will be tagged with ",omitempty"
    pub omit_empty: bool,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct HaskellOptions {
    /// Plain types only
    pub just_types: bool,
    /// Use Array or List
    pub array_type: ArrayOrList,
    /// Generated module name
    pub module: String,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Default)]
pub struct PythonOptions {
    /// Python Version
    pub python_version: PythonVersion,
    /// Classes only
    pub just_types: bool,
    /// Transform property names to be Pythonic
    pub nice_property_names: bool,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Default)]
pub struct RubyOptions {
    /// Plain types only
    pub just_types: bool,
    /// Type strictness
    pub strictness: RubyStrictness,
    /// Specify a wrapping Namespace
    pub namespace: String,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct RustOptions {
    /// Density
    pub density: Density,
    /// Field visibility
    pub visibility: RustVisibility,
    /// Derive Debug impl
    pub derive_debug: bool,
    /// Derive Clone impl
    pub derive_clone: bool,
    /// Derive PartialEq impl
    pub derive_partial_eq: bool,
    /// Skip serializing empty Option fields
    pub skip_serializing_none: bool,
    /// Edition 2018
    pub edition_2018: bool,
    /// Leading Comments
    pub leading_comments: bool,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct SmithyOptions {
    /// Serialization framework
    pub framework: SmithyFramework,
    /// Package
    pub package: String,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct SwiftOptions {
    /// Plain types only
    pub just_types: bool,
    /// Generate initializers and mutators
    pub convience_initializers: bool,
    /// Explicit CodingKey values in Codable types
    pub explicit_coding_keys: bool,
    /// CodingKeys implements protocols
    pub coding_keys_protocol: String,
    /// Alamofire extensions
    pub alamofire: bool,
    /// Prefix for type names
    pub type_prefix: String,
    /// Prefix for type names
    pub struct_or_class: StructOrClass,
    /// Use var instead of let for object properties
    pub mutable_properties: bool,
    /// Acronym naming style
    pub acronym_style: AcronymStyle,
    /// Code density
    pub density: Density,
    /// Support Linux
    pub support_linux: bool,
    /// Objects inherit from NSObject and @objcMembers is added to classes
    pub objective_c_support: bool,
    /// If no matching case is found enum value is set to null
    pub optional_enums: bool,
    /// Renders output in a Swift 5 compatible mode
    pub swift_5_support: bool,
    /// Mark generated models as Sendable
    pub sendable: bool,
    // Exposing this attribute breaks codegen.
    // multi_file_output: bool,
    /// Access level
    pub access_level: SwiftAccessLevel,
    /// Make types implement protocol   
    pub protocol: SwiftProtocol,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Default)]
pub enum ArrayOrList {
    #[default]
    Array,
    List,
}

impl ArrayOrList {
    fn as_str(&self) -> &'static str {
        match self {
            ArrayOrList::Array => "array",
            ArrayOrList::List => "list",
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Default)]
pub enum StructOrClass {
    #[default]
    Struct,
    Class,
}

impl StructOrClass {
    fn as_str(&self) -> &'static str {
        match self {
            StructOrClass::Struct => "struct",
            StructOrClass::Class => "class",
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Default)]
pub enum Density {
    #[default]
    Normal,
    Dense,
}

impl Density {
    fn as_str(&self) -> &'static str {
        match self {
            Density::Normal => "normal",
            Density::Dense => "dense",
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Default)]
pub enum AcronymStyle {
    #[default]
    Original,
    Pascal,
    Camel,
    LowerCase,
}

impl AcronymStyle {
    fn as_str(&self) -> &'static str {
        match self {
            AcronymStyle::Original => "original",
            AcronymStyle::Pascal => "pascal",
            AcronymStyle::Camel => "camel",
            AcronymStyle::LowerCase => "lowercase",
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Default)]
pub enum CSharpFramework {
    #[default]
    NewtonSoft,
    SystemTextJson,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Default)]
pub enum CSharpNumberType {
    #[default]
    Double,
    Decimal,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Default)]
pub enum CSharpAnyType {
    #[default]
    Object,
    Dynamic,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Default)]
pub enum CSharpVersion {
    V5,
    #[default]
    V6,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Default)]
pub enum CSharpFeatures {
    #[default]
    Complete,
    AttributesOnly,
    JustTypesAndNamespace,
    JustTypes,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Default)]
pub enum CSharpBaseClass {
    EntityData,
    #[default]
    Object,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Default)]
pub enum PythonVersion {
    V3_5,
    #[default]
    V3_6,
    V3_7,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Default)]
pub enum RubyStrictness {
    #[default]
    Strict,
    Coercible,
    None,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Default)]
pub enum RustVisibility {
    #[default]
    Private,
    Crate,
    Public,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Default)]
pub enum SmithyFramework {
    #[default]
    JustTypes,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Default)]
pub enum SwiftAccessLevel {
    #[default]
    Internal,
    Public,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Default)]
pub enum SwiftProtocol {
    #[default]
    None,
    Equatable,
    Hashable,
}

impl Language {
    pub fn get_args(&self) -> Vec<String> {
        match self {
            Language::Typescript(opt) => CliBuilder::new(self)
                .opt_bool("--just-types", opt.just_types)
                .opt_bool("--nice-property-names", opt.nice_property_names)
                .opt_bool("--explicit-unions", opt.explicit_unions)
                .opt_bool("--prefer-unions", opt.prefer_unions)
                .opt_bool("--prefer-types", opt.prefer_types)
                .opt_bool("--prefer-const-values", opt.prefer_const_values)
                .opt_bool("--readonly", opt.readonly)
                .build(),
            Language::JsonSchema(_) => CliBuilder::new(self).build(),
            Language::CSharp(opt) => CliBuilder::new(self)
                .opt_enum("--framework", &opt.framework, || match opt.framework {
                    CSharpFramework::NewtonSoft => "NetwonSoft",
                    CSharpFramework::SystemTextJson => "SystemTextJson",
                })
                .opt_enum("--array-type", &opt.array_type, || opt.array_type.as_str())
                .opt_enum("--density", &opt.density, || opt.density.as_str())
                .opt_string("--namespace", &opt.namespace)
                .opt_enum("--csharp-version", &opt.csharp_version, || {
                    match opt.csharp_version {
                        CSharpVersion::V5 => "5",
                        CSharpVersion::V6 => "6",
                    }
                })
                .opt_bool("--virtual", opt.r#virtual)
                .opt_enum("--any-type", &opt.any_type, || match opt.any_type {
                    CSharpAnyType::Object => "object",
                    CSharpAnyType::Dynamic => "dynamic",
                })
                .opt_enum("--number-type", &opt.number_type, || {
                    match opt.number_type {
                        CSharpNumberType::Double => "double",
                        CSharpNumberType::Decimal => "decimal",
                    }
                })
                .opt_enum("--features", &opt.features, || match opt.features {
                    CSharpFeatures::Complete => "complete",
                    CSharpFeatures::AttributesOnly => "attributes-only",
                    CSharpFeatures::JustTypesAndNamespace => "just-types-and-namespace",
                    CSharpFeatures::JustTypes => "just-types",
                })
                .opt_enum("--base-class", &opt.base_class, || match opt.base_class {
                    CSharpBaseClass::EntityData => "EntityData",
                    CSharpBaseClass::Object => "Object",
                })
                .opt_bool("--check-required", opt.check_required)
                .opt_bool("--keep-property-name", opt.keep_property_name)
                .build(),
            Language::Crystal(_) => CliBuilder::new(self).build(),
            Language::Dart(opt) => CliBuilder::new(self)
                .opt_bool("--null-safety", opt.null_safety)
                .opt_bool("--just-types", opt.just_types)
                .opt_bool("--coders-in-class", opt.coders_in_class)
                .opt_bool("--from-map", opt.from_map)
                .opt_bool("--required-props", opt.required_props)
                .opt_bool("--copy-with", opt.copy_with)
                .opt_bool("--use-freezed", opt.use_freezed)
                .opt_bool("--use-hive", opt.use_hive)
                .opt_bool("--use-json-annotation", opt.use_json_annotation)
                .opt_string("--part-name", &opt.part_name)
                .build(),
            Language::Elm(opt) => CliBuilder::new(self)
                .opt_bool("--just-types", opt.just_types)
                .opt_enum("--array-type", &opt.array_type, || opt.array_type.as_str())
                .opt_string("--module", &opt.module)
                .build(),
            Language::Go(opt) => CliBuilder::new(self)
                .opt_bool("--just-types", opt.just_types)
                .opt_bool("--just-types-and-package", opt.just_types_and_package)
                .opt_string("--package", &opt.package)
                .opt_string("--field-tags", &opt.field_tags)
                .opt_bool("--omit-empty", opt.omit_empty)
                .build(),
            Language::Haskell(opt) => CliBuilder::new(self)
                .opt_bool("--just-types", opt.just_types)
                .opt_enum("--array-type", &opt.array_type, || opt.array_type.as_str())
                .opt_string("--module", &opt.module)
                .build(),
            Language::Python(opt) => CliBuilder::new(self)
                .opt_enum("--python-version", &opt.python_version, || {
                    match opt.python_version {
                        PythonVersion::V3_5 => "3.5",
                        PythonVersion::V3_6 => "3.6",
                        PythonVersion::V3_7 => "3.7",
                    }
                })
                .opt_bool("--just-types", opt.just_types)
                .opt_bool("--nice-property-names", opt.nice_property_names)
                .build(),
            Language::Ruby(opt) => CliBuilder::new(self)
                .opt_bool("--just-types", opt.just_types)
                .opt_enum("--strictness", &opt.strictness, || match opt.strictness {
                    RubyStrictness::Strict => "strict",
                    RubyStrictness::Coercible => "coercible",
                    RubyStrictness::None => "none",
                })
                .opt_string("--namespace", &opt.namespace)
                .build(),
            Language::Rust(opt) => CliBuilder::new(self)
                .opt_enum("--density", &opt.density, || opt.density.as_str())
                .opt_enum("--visibility", &opt.visibility, || match opt.visibility {
                    RustVisibility::Private => "private",
                    RustVisibility::Crate => "crate",
                    RustVisibility::Public => "public",
                })
                .opt_bool("--derive-debug", opt.derive_debug)
                .opt_bool("--derive-clone", opt.derive_clone)
                .opt_bool("--derive-partial-eq", opt.derive_partial_eq)
                .opt_bool("--skip-seriaziling-none", opt.skip_serializing_none)
                .opt_bool("--edition-2018", opt.edition_2018)
                .opt_bool("--leading-comments", opt.leading_comments)
                .build(),
            Language::Smithy(opt) => CliBuilder::new(self)
                .opt_enum("--framework", &opt.framework, || match opt.framework {
                    SmithyFramework::JustTypes => "just-types",
                })
                .opt_string("--package", &opt.package)
                .build(),
            Language::Swift(opt) => CliBuilder::new(self)
                .opt_bool("--just-types", opt.just_types)
                .opt_bool("--initializers", opt.convience_initializers)
                .opt_bool("--coding-keys", opt.explicit_coding_keys)
                .opt_string("--coding-keys-protocol", &opt.coding_keys_protocol)
                .opt_bool("--alamofire", opt.alamofire)
                .opt_string("--type-prefix", &opt.type_prefix)
                .opt_enum("--struct-or-class", &opt.struct_or_class, || {
                    opt.struct_or_class.as_str()
                })
                .opt_bool("--mutable-properties", opt.mutable_properties)
                .opt_enum("--acronym-style", &opt.acronym_style, || {
                    opt.acronym_style.as_str()
                })
                .opt_enum("--density", &opt.density, || opt.density.as_str())
                .opt_bool("--support-linux", opt.support_linux)
                .opt_bool("--objective-c-support", opt.objective_c_support)
                .opt_bool("--optional-enums", opt.optional_enums)
                .opt_bool("--swift-5-support", opt.swift_5_support)
                .opt_bool("--sendable", opt.sendable)
                .opt_enum("--access-level", &opt.access_level, || {
                    match opt.access_level {
                        SwiftAccessLevel::Internal => "internal",
                        SwiftAccessLevel::Public => "public",
                    }
                })
                .opt_enum("--protocol", &opt.protocol, || match opt.protocol {
                    SwiftProtocol::None => "none",
                    SwiftProtocol::Equatable => "equatable",
                    SwiftProtocol::Hashable => "hashable",
                })
                .build(),
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Language::Typescript(_) => "typescript",
            Language::JsonSchema(_) => "schema",
            Language::CSharp(_) => "csharp",
            Language::Crystal(_) => "crystal",
            Language::Dart(_) => "dart",
            Language::Elm(_) => "elm",
            Language::Go(_) => "go",
            Language::Haskell(_) => "haskell",
            Language::Python(_) => "python",
            Language::Ruby(_) => "ruby",
            Language::Rust(_) => "rust",
            Language::Smithy(_) => "smithy",
            Language::Swift(_) => "swift",
        }
    }
}

impl Default for CSharpOptions {
    fn default() -> Self {
        Self {
            framework: CSharpFramework::default(),
            array_type: ArrayOrList::default(),
            density: Density::default(),
            namespace: "QuickType".to_owned(),
            csharp_version: CSharpVersion::default(),
            r#virtual: false,
            any_type: CSharpAnyType::default(),
            number_type: CSharpNumberType::default(),
            features: CSharpFeatures::default(),
            base_class: CSharpBaseClass::default(),
            check_required: false,
            keep_property_name: false,
        }
    }
}

impl Default for DartOptions {
    fn default() -> Self {
        Self {
            null_safety: true,
            just_types: false,
            coders_in_class: false,
            from_map: false,
            required_props: false,
            copy_with: false,
            use_freezed: false,
            use_hive: false,
            use_json_annotation: false,
            part_name: "".to_owned(),
        }
    }
}

impl Default for ElmOptions {
    fn default() -> Self {
        Self {
            just_types: false,
            array_type: ArrayOrList::default(),
            module: "QuickType".to_owned(),
        }
    }
}

impl Default for GoOptions {
    fn default() -> Self {
        Self {
            just_types: false,
            just_types_and_package: false,
            package: "main".to_owned(),
            field_tags: "json".to_owned(),
            omit_empty: false,
        }
    }
}

impl Default for HaskellOptions {
    fn default() -> Self {
        Self {
            just_types: false,
            array_type: ArrayOrList::default(),
            module: "QuickType".to_owned(),
        }
    }
}

impl Default for RustOptions {
    fn default() -> Self {
        Self {
            density: Density::default(),
            visibility: RustVisibility::default(),
            derive_debug: false,
            derive_clone: false,
            derive_partial_eq: false,
            skip_serializing_none: false,
            edition_2018: true,
            leading_comments: true,
        }
    }
}

impl Default for SmithyOptions {
    fn default() -> Self {
        Self {
            framework: SmithyFramework::default(),
            package: "quicktype".to_owned(),
        }
    }
}

impl Default for SwiftOptions {
    fn default() -> Self {
        Self {
            just_types: false,
            convience_initializers: true,
            explicit_coding_keys: true,
            coding_keys_protocol: "".to_owned(),
            alamofire: false,
            type_prefix: "".to_owned(),
            struct_or_class: StructOrClass::default(),
            mutable_properties: false,
            acronym_style: AcronymStyle::Pascal,
            density: Density::default(),
            support_linux: false,
            objective_c_support: false,
            optional_enums: false,
            swift_5_support: false,
            sendable: false,
            access_level: SwiftAccessLevel::default(),
            protocol: SwiftProtocol::default(),
        }
    }
}
