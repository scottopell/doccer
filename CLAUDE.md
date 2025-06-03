<project_goal>
emit human-readable text for terminal viewing of rust documentation
</project_goal>

<generate_json_doc>
cargo +nightly rustdoc --lib -- -Zunstable-options --output-format json
</generate_json_doc>


<commands>
    <test> cargo test </test>
    <run> cargo run target/doc/<PLACEHOLDER>.json</run>
</commands>
