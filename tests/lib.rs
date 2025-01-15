use move_vm_test_utils::InMemoryStorage;
use rstest::*;

use lite_move_vm::{create_starting_storage, LiteMVM};

#[fixture]
#[once]
fn starting_storage() -> InMemoryStorage {
    create_starting_storage("tests/test-package")
}

#[rstest]
fn call_with_simple_return(starting_storage: &InMemoryStorage) {
    let mut move_vm = LiteMVM::new(starting_storage);

    move_vm.call("main", "eight");
}
