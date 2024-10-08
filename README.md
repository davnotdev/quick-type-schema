# `quick-type-schema`

[![Version Badge](https://img.shields.io/crates/v/quick-type-schema)](https://crates.io/crates/quick-type-schema)
[![Docs Badge](https://img.shields.io/docsrs/quick-type-schema/latest)](https://docs.rs/quick-type-schema/latest/)
[![License Badge](https://img.shields.io/crates/l/quick-type-schema)](LICENSE)
[![Downloads Badge](https://img.shields.io/crates/d/quick-type-schema)](https://crates.io/crates/quick-type-schema)

`quick-type-schema` is a simple library that takes your rust types and converts them to whatever language by converting your types into JSON schema, then running it through [quicktype.io](quicktype.io).
Created for [`bubbel-backend`](https://github.com/joinbubbel/bubbel-backend), this library has been repurposed for the general use case!

> To be 100% clear, this library is a wrapper over [quicktype.io](quicktype.io).
> Additionally, not all cases and options are tested / available.
> Please report all issues you come across.

## Prerequisites

Since this library is a wrapper over quicktype, you will need either `npx` or `quicktype` installed.
The latter will startup faster.

Additionally, you will need the `JsonSchema` trait from [`schema-rs`](https://github.com/GREsau/schemars)
to be derived for your types.

## Usage

Add to `Cargo.toml`

```toml
[dependencies]
quick-type-schema = "0.2"
```

```rust
use quick_type_schema::{CodegenContext, Language};

// quicktype cli override arguments can be explicitly added here.
// These overrides even affect `Language` passed in `codegen.finish`
// Ex: `Some(&["-l", "typescript", "--just-types"])`
let mut codegen = CodegenContext::new("TopLevelName", None);

// Add a type that has derived `JsonSchema`
codegen.add_type::<MyStruct>();

// Add your own json schema
codegen.add_schema("{ ... }");

// See the docs or `quicktype --help` for all typescript options
codegen.finish(Language::Typescript(TypescriptOptions { ..Default::default() }));
```

You can add these to your build process, but because of the slow startup of quicktype and lack of caching, this is highly discouraged.

## Language Support

`quick-type-schema` supports pretty much every language quicktype supports.
However, some have been left out due to incompatibilities or complexity (notable ones being C, C++, Java, Kotlin, Objective-C, and Scala3).
For those languages, please use the override arguments and refer to the quicktype cli - `quicktype --help`.
Here are the supported languages:

```
Typescript
JsonSchema
CSharp
Crystal
Dart
Elm
Go
Haskell
Python
Ruby
Rust
Smithy
Swift
```
