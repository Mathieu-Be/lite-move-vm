module lite_svm_test_pkg::main;

use sui::table::{Self, Table};

public fun num(): u64 {
    8
}

public fun tuple(): (u64, u64) {
    (1, 2)
}

public fun ping(): std::string::String {
    b"pong".to_string()
}

public fun revert(): u64 {
    assert!(false, 49);

    0
}

public fun num_input(a: u64): u64 {
    2 * a
}

public fun tuple_input(a: u64, b: u64): u64 {
    a + b
}

public struct Counter has key, store {
    id: UID,
    value: u64,
    table: Table<u64, u64>,
}

public fun value(counter: &Counter): u64 {
    counter.value
}

public fun create_counter(ctx: &mut tx_context::TxContext): Counter {
    Counter { id: object::new(ctx), value: 0, table: table::new(ctx) }
}

public fun increment(counter: &mut Counter) {
    counter.value = counter.value + 1;
}
