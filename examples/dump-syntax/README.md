Parse a Rust source file into a `syn::File` and print out a debug representation
of the syntax tree.

Use the following command from this directory to test this program by running it
on its own source code:

```
cargo run -- ~/2024/08/20/self-similar/ocaml-rust-unification/src/ocaml_types.rs
cargo run -- ~/2024/08/20/self-similar/ocaml-rust-unification/src/ocaml_types.rs | jq . >  ~/2024/08/20/self-similar/ocaml-rust-unification/src/ocaml_types.json
```

The output will begin with:

```
File {
    shebang: None,
    attrs: [
        Attribute {
            pound_token: Pound,
            style: Inner(
                Bang
            ),
            bracket_token: Bracket,
            path: Path {
                leading_colon: None,
                segments: [
    ...
}
```
