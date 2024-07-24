Fluentbit ([website](https://github.com/fluent/fluent-bit), [github](https://github.com/fluent/fluent-bit)) is a popular open-source log-shipping tool that can take logs in from many different sources, filter and process them, then send them on to many different supported outputs.

One of its filtering 'plugins' is the [WASM filter](https://docs.fluentbit.io/manual/pipeline/filters/wasm), which currently embeds the 'WebAssembly Micro Runtime' ([website](https://bytecodealliance.github.io/wamr.dev/), [github](https://github.com/bytecodealliance/wasm-micro-runtime)) (see [here](https://github.com/fluent/fluent-bit/tree/v3.1.3/lib/wasm-micro-runtime-WAMR-1.3.0), [here](https://github.com/fluent/fluent-bit/blob/master/include/fluent-bit/wasm/flb_wasm.h)/[here](https://github.com/fluent/fluent-bit/blob/master/src/wasm/flb_wasm.c), and [here](https://github.com/fluent/fluent-bit/tree/master/plugins/filter_wasm) in fluentbit source) to facilitate executing [WebAssembly (WASM)](https://webassembly.org/) programs/code to process or transform particular flows of log messages that pass through Fluentbit.

This project is a [Rust](https://www.rust-lang.org/) project for prototyping WASM binaries for use in this WASM filter plugin in Fluentbit. It provides some sample functions that have the function signature/ABI that the Fluentbit plugin uses for passing data into the WASM context that runs inside the filter, and examples of how to safely return a C string back to Fluentbit, so you can get started quickly with the interface Fluentbit makes available for processing and transforming log records within a Rust context. It is intended as both a getting-started boilerplate, but also an educational set of code to explain all the things required for the interop between the various environments involved (Fluentbit's C code -> calling into the WASM VM -> execution of Rust functions designed to start with the paramaters passed from the C code).

## Setup / Depenecncies

* Rust compiler with the `wasm32-unknown-unknown` target set up (i.e. run `rustup target install wasm32-unknown-unknown`)
* Cargo, for getting Rust msgpack libraries
* Docker, for testing the filter against Fluentbit
* Make, for various shortcut targets used in development
* Optionally, you may want the [WebAssembly Binary Toolkit (wabt)](https://github.com/WebAssembly/wabt) and its `wasm2wat` tool for looking at the compiled output programs

With the above dependencies set up, you should be able to run `make build` and `make test_json` or `make test_msgpack` with success, and in the latter two cases, see a Fluentbit instance come up in your console with output as below:

```
Fluent Bit v3.1.2
* Copyright (C) 2015-2024 The Fluent Bit Authors
* Fluent Bit is a CNCF sub-project under the umbrella of Fluentd
* https://fluentbit.io

______ _                  _    ______ _ _           _____  __  
|  ___| |                | |   | ___ (_) |         |____ |/  | 
| |_  | |_   _  ___ _ __ | |_  | |_/ /_| |_  __   __   / /`| | 
|  _| | | | | |/ _ \ '_ \| __| | ___ \ | __| \ \ / /   \ \ | | 
| |   | | |_| |  __/ | | | |_  | |_/ / | |_   \ V /.___/ /_| |_
\_|   |_|\__,_|\___|_| |_|\__| \____/|_|\__|   \_/ \____(_)___/

[2024/07/24 13:12:55] [ info] [fluent bit] version=3.1.2, commit=a6feacd6e9, pid=1
[2024/07/24 13:12:55] [ info] [storage] ver=1.5.2, type=memory, sync=normal, checksum=off, max_chunks_up=128
[2024/07/24 13:12:55] [ info] [cmetrics] version=0.9.1
[2024/07/24 13:12:55] [ info] [ctraces ] version=0.5.1
[2024/07/24 13:12:55] [ info] [input:dummy:dummy.0] initializing
[2024/07/24 13:12:55] [ info] [input:dummy:dummy.0] storage_strategy='memory' (memory only)
[2024/07/24 13:12:55] [ info] [sp] stream processor started
[2024/07/24 13:12:55] [ info] [output:stdout:stdout.0] worker #0 started
[0] dummy.0: [[1721826775.984965222, {}], {"msg"=>"Hello world from rust wasm! 🙂"}]
```

## Misc

See https://chronosphere.io/learn/dynamic-log-routing-on-kubernetes-labels-fluent-bit/ for another example on writing a program to use in the WASM filter, using Go instead of Rust.
