# Lite Move VM

Lite Move VM is a tool used to control a Move package directly in Rust. This will allow the use of the testing powers of Rust (advanced fuzzing, use of external libraries) in order to test a Move package, in a lightweight way. Currently very minimal, the recommended useage is to write the testing scenario in a `#[test_only]` function that will be controlled by a few simple inputs.
