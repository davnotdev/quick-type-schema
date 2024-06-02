# `quick-type-schema`

[![Version Badge](https://img.shields.io/crates/v/quick-type-schema)](https://crates.io/crates/quick-type-schema)
[![Docs Badge](https://img.shields.io/docsrs/quick-type-schema/latest)](https://docs.rs/quick-type-schema/latest/quick-type-schema/)
[![License Badge](https://img.shields.io/crates/l/quick-type-schema)](LICENSE)
[![Downloads Badge](https://img.shields.io/crates/d/quick-type-schema)](https://crates.io/crates/quick-type-schema)

`quick-type-schema` is a simple library that takes your rust types and converts
them to whatever language by converting your types into JSON schema, then
running it through [quicktype.io](quicktype.io).
Created for [`bubbel-backend`](https://github.com/joinbubbel/bubbel-backend),
this library has been repurposed for the general use case!

> To be 100% clear, this library is a wrapper over [quicktype.io](quicktype.io).

