use std::path::Path;
use std::sync::LazyLock;

use move_binary_format::{errors::VMError, CompiledModule};
use move_core_types::{
    account_address::AccountAddress,
    identifier::IdentStr,
    language_storage::ModuleId,
    runtime_value::{serialize_values, MoveValue},
};
use move_package::{compilation::build_plan::BuildPlan, BuildConfig};
use move_unit_test::{
    extensions::{self, set_extension_hook},
    test_runner::setup_test_storage,
};
use move_vm_runtime::{move_vm::MoveVM, session::Session};
pub use move_vm_test_utils::InMemoryStorage;
use move_vm_types::gas::UnmeteredGasMeter;
use sui_move::unit_test::new_testing_object_and_natives_cost_runtime;
use sui_move_build::set_sui_flavor;
use sui_protocol_config::ProtocolConfig;

static MOVE_VM: LazyLock<MoveVM> = LazyLock::new(|| create_vm());

pub struct LiteMVM<'r, 'l> {
    session: Session<'r, 'l, InMemoryStorage>,
}

impl LiteMVM<'_, '_> {
    pub fn new(starting_storage: &InMemoryStorage) -> Self {
        let session = MOVE_VM
            .new_session_with_extensions(starting_storage.clone(), extensions::new_extensions());

        Self { session }
    }

    pub fn call<'a, I>(
        &mut self,
        module: &str,
        function: &str,
        args: I,
    ) -> Result<Vec<Vec<u8>>, VMError>
    where
        I: IntoIterator<Item = &'a MoveValue>,
    {
        let serialized_return_values_result = self.session.execute_function_bypass_visibility(
            &ModuleId::new(
                AccountAddress::from_hex_literal("0x0").unwrap(),
                IdentStr::new(module).unwrap().into(),
            ),
            IdentStr::new(function).unwrap(),
            vec![],
            serialize_values(args),
            &mut UnmeteredGasMeter,
        );

        let return_result: Result<Vec<Vec<u8>>, VMError> =
            serialized_return_values_result.map(|res| {
                res.return_values
                    .into_iter()
                    .map(|(bytes, _layout)| bytes)
                    .collect()
            });

        return_result
    }
}

pub fn create_starting_storage(path: &str) -> InMemoryStorage {
    set_extension_hook(Box::new(new_testing_object_and_natives_cost_runtime));

    let mut build_config = BuildConfig::default();
    build_config.test_mode = true;

    set_sui_flavor(&mut build_config);

    let resolution_graph = build_config
        .resolution_graph_for_package(Path::new(path), None, &mut std::io::sink())
        .unwrap();

    let mut bytecode_deps_modules = vec![];
    for pkg in resolution_graph.package_table.values() {
        let source_available = !pkg
            .get_sources(&resolution_graph.build_options)
            .unwrap()
            .is_empty();
        if source_available {
            continue;
        }
        for bytes in pkg.get_bytecodes_bytes().unwrap() {
            let module = CompiledModule::deserialize_with_defaults(&bytes).unwrap();
            bytecode_deps_modules.push(module);
        }
    }

    let build_plan = BuildPlan::create(resolution_graph).unwrap();
    let pkg = build_plan.compile(&mut std::io::sink()).unwrap();

    let modules = pkg.all_compiled_units().map(|unit| &unit.module);

    setup_test_storage(modules, bytecode_deps_modules.iter()).unwrap()
}

fn create_vm() -> MoveVM {
    let native_function_table =
        sui_move_natives_latest::all_natives(true, &ProtocolConfig::get_for_max_version_UNSAFE());

    MoveVM::new(native_function_table).unwrap()
}
