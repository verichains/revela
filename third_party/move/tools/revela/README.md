Revela decompiler
---

Designed and implemented by Verichains, Revela is a tool to decompile
low-level Move bytecode back to Move source code.

To build Revela, from the root directory of aptos-core, execute
the following command.

```
cargo build -p revela
```

Then to decompile a Move bytecode file, pass the file path as a command line
argument using `--bytecode` (or `-b`), as shown below.

```
cargo run -p revela -- --bytecode <file_path>
```

Please note that `<file_path>` should be replaced with the actual path to
the bytecode file you want to decompile.

For example:

```
cargo run -p revela -- -b third_party/move/tools/revela/tests/bytecode/BasicCoin.mv
```
