# `quick-type-schema`

[![Version Badge](https://img.shields.io/crates/v/quick-type-schema)](https://crates.io/crates/quick-type-schema)
[![Docs Badge](https://img.shields.io/docsrs/quick-type-schema/latest)](https://docs.rs/quick-type-schema/latest/quick-type-schema/)
[![License Badge](https://img.shields.io/crates/l/quick-type-schema)](LICENSE)
[![Downloads Badge](https://img.shields.io/crates/d/quick-type-schema)](https://crates.io/crates/quick-type-schema)

`quick-type-schema` is a simple library that takes your rust types and converts them to whatever language by converting your types into JSON schema, then running it through [quicktype.io](quicktype.io).
Created for [`bubbel-backend`](https://github.com/joinbubbel/bubbel-backend), this library has been repurposed for the general use case!

> To be 100% clear, this library is a wrapper over [quicktype.io](quicktype.io).

## Prerequisites

Since this library is a wrapper over quicktype, you will need either `npx` or `quicktype` installed.
The latter will startup faster.

## Usage

Add to `Cargo.toml`

```
quick-type-schema = "0.1"
```

```
use quick_type_schema::{CodegenContext, Language};

// quicktype cli override arguments can be explicitly added here.
// Ex: `Some(&["-l", "typescript", "--just-types"])`
let mut codegen = CodegenContext::new(None);

// Add a type that has derived `JsonSchema`
codegen.add_type::<MyStruct>();

// Add your own json schema
codegen.add_schema("{ ... }");

codegen.finish(Language::Typescript);
```

You can add these to your build process, but because of the slow startup of quicktype and lack of caching, this is highly discouraged.

## Language Support

Generally, `quick-type-schema` supports pretty much every language quicktype supports.
However, some have been left out due to incompatibilities or complexity (notable ones being C and C++).
For those languages, please use the override arguments and refer to the quicktype cli - `quicktype --help`.
Here are the supported languages:

```
Typescript
JsonSchema
CSharp
    +via Newton Soft
    +via System.Text.Json
Crystal
Dart
Elm
Go
Haskell
Java
Kotlin
    +via Jackson
    +via Klaxon
    +via Kotlinx
ObjectiveC
Python
    +via v3.5
    +via v3.6
    +via v3.7
Ruby
Rust
Scala3
    +via Circe
    +via Upickle
Smithy
Swift
```

