use move_core_types::runtime_value::MoveValue;
use move_vm_test_utils::InMemoryStorage;
use rstest::*;

use lite_move_vm::{create_starting_storage, LiteMVM};

#[fixture]
#[once]
fn starting_storage() -> InMemoryStorage {
    create_starting_storage("tests/test-package")
}

#[rstest]
fn call_num(starting_storage: &InMemoryStorage) {
    let mut move_vm = LiteMVM::new(starting_storage);

    let result = move_vm.call("main", "num", vec![]).unwrap();
    let result = bcs::from_bytes::<u64>(result.get(0).unwrap()).unwrap();

    assert_eq!(result, 8);
}

#[rstest]
fn call_tuple(starting_storage: &InMemoryStorage) {
    let mut move_vm = LiteMVM::new(starting_storage);

    let result = move_vm.call("main", "tuple", vec![]).unwrap();
    let result = result
        .iter()
        .map(|r| bcs::from_bytes::<u64>(r).unwrap())
        .collect::<Vec<_>>();

    assert_eq!(*result.get(0).unwrap(), 1);
    assert_eq!(*result.get(1).unwrap(), 2);
}

#[rstest]
fn call_string(starting_storage: &InMemoryStorage) {
    let mut move_vm = LiteMVM::new(starting_storage);

    let result = move_vm.call("main", "ping", vec![]).unwrap();
    let result = bcs::from_bytes::<String>(result.get(0).unwrap()).unwrap();

    assert_eq!(result, "pong");
}

#[rstest]
fn call_revert(starting_storage: &InMemoryStorage) {
    let mut move_vm = LiteMVM::new(starting_storage);

    move_vm.call("main", "revert", vec![]).expect_err("");
}

#[rstest]
fn call_num_input(starting_storage: &InMemoryStorage) {
    let mut move_vm = LiteMVM::new(starting_storage);

    let result = move_vm
        .call("main", "num_input", vec![&MoveValue::U64(33)])
        .unwrap();
    let result = bcs::from_bytes::<u64>(result.get(0).unwrap()).unwrap();

    assert_eq!(result, 66);
}

#[rstest]
fn call_tuple_input(starting_storage: &InMemoryStorage) {
    let mut move_vm = LiteMVM::new(starting_storage);

    let result = move_vm
        .call(
            "main",
            "tuple_input",
            vec![&MoveValue::U64(1), &MoveValue::U64(1)],
        )
        .unwrap();
    let result = bcs::from_bytes::<u64>(result.get(0).unwrap()).unwrap();

    assert_eq!(result, 2);
}

#[rstest]
fn call_test(starting_storage: &InMemoryStorage) {
    let mut move_vm = LiteMVM::new(starting_storage);

    let result = move_vm.call("tests", "test_with_object", vec![]).unwrap();
    let result = bcs::from_bytes::<u64>(result.get(0).unwrap()).unwrap();

    assert_eq!(result, 1);
}
