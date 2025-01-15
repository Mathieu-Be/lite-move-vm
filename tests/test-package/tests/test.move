#[test_only]
module lite_svm_test_pkg::tests;

use lite_svm_test_pkg::main::{ping, create_counter, increment};
use sui::test_scenario;

#[test]
fun basic_test() {
    ping();
    assert!(2==2);
}

#[test]
public fun test_with_object(): u64 {
    let mut scenario = test_scenario::begin(@0xA);

    let mut counter = create_counter(scenario.ctx());

    scenario.next_tx(@0xA);

    increment(&mut counter);

    let final_value = counter.value();

    sui::transfer::public_transfer(counter, @0xA);

    scenario.end();

    final_value
}
