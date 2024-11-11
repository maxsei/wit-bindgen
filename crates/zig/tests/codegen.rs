use std::path::Path;
use std::process::Command;

use heck::ToSnakeCase;

macro_rules! codegen_test {
    (allow_unused $name: tt $test:tt) => {};
    (char $name: tt $test:tt) => {};
    (conventions $name: tt $test:tt) => {};
    (empty $name: tt $test:tt) => {};
    (enum_has_go_keyword $name: tt $test:tt) => {};
    (flags $name: tt $test:tt) => {};
    // (floats $name: tt $test:tt) => {};
    (fully_qualified_java_address $name: tt $test:tt) => {};
    (go_params $name: tt $test:tt) => {};
    (guest_name $name: tt $test:tt) => {};
    (import_and_export_resource $name: tt $test:tt) => {};
    (import_and_export_resource_alias $name: tt $test:tt) => {};
    (import_func $name: tt $test:tt) => {};
    (integers $name: tt $test:tt) => {};
    (interface_has_go_keyword $name: tt $test:tt) => {};
    (issue544 $name: tt $test:tt) => {};
    (issue551 $name: tt $test:tt) => {};
    (issue569 $name: tt $test:tt) => {};
    (issue573 $name: tt $test:tt) => {};
    (issue607 $name: tt $test:tt) => {};
    (issue668 $name: tt $test:tt) => {};
    (issue929 $name: tt $test:tt) => {};
    (issue929_no_export $name: tt $test:tt) => {};
    (issue929_no_import $name: tt $test:tt) => {};
    (issue929_only_methods $name: tt $test:tt) => {};
    (just_export $name: tt $test:tt) => {};
    (keywords $name: tt $test:tt) => {};
    (keywords_in_interfaces_and_worlds $name: tt $test:tt) => {};
    (lift_lower_foreign $name: tt $test:tt) => {};
    (lists $name: tt $test:tt) => {};
    (many_arguments $name: tt $test:tt) => {};
    (multi_return $name: tt $test:tt) => {};
    (multiversion $name: tt $test:tt) => {};
    (option_result $name: tt $test:tt) => {};
    (record_has_go_keyword_and_used_in_fn $name: tt $test:tt) => {};
    (records $name: tt $test:tt) => {};
    (rename_interface $name: tt $test:tt) => {};
    (resource_alias $name: tt $test:tt) => {};
    (resource_borrow_in_record $name: tt $test:tt) => {};
    (resource_borrow_in_record_export $name: tt $test:tt) => {};
    (resource_local_alias $name: tt $test:tt) => {};
    (resource_local_alias_borrow $name: tt $test:tt) => {};
    (resource_local_alias_borrow_import $name: tt $test:tt) => {};
    (resource_own_in_other_interface $name: tt $test:tt) => {};
    (resources $name: tt $test:tt) => {};
    (resources_in_aggregates $name: tt $test:tt) => {};
    (resources_with_lists $name: tt $test:tt) => {};
    (result_empty $name: tt $test:tt) => {};
    (ret_areas $name: tt $test:tt) => {};
    (return_resource_from_export $name: tt $test:tt) => {};
    (same_names1 $name: tt $test:tt) => {};
    (same_names2 $name: tt $test:tt) => {};
    (same_names3 $name: tt $test:tt) => {};
    (same_names4 $name: tt $test:tt) => {};
    (same_names5 $name: tt $test:tt) => {};
    (simple_enum $name: tt $test:tt) => {};
    (simple_functions $name: tt $test:tt) => {};
    (simple_http $name: tt $test:tt) => {};
    (simple_lists $name: tt $test:tt) => {};
    (simple_option $name: tt $test:tt) => {};
    (small_anonymous $name: tt $test:tt) => {};
    (smoke $name: tt $test:tt) => {};
    (smoke_default $name: tt $test:tt) => {};
    (smoke_export $name: tt $test:tt) => {};
    (strings $name: tt $test:tt) => {};
    (unused_import $name: tt $test:tt) => {};
    (use_across_interfaces $name: tt $test:tt) => {};
    (variants $name: tt $test:tt) => {};
    (variants_unioning_types $name: tt $test:tt) => {};
    (wasi_cli $name: tt $test:tt) => {};
    (wasi_clocks $name: tt $test:tt) => {};
    (wasi_filesystem $name: tt $test:tt) => {};
    (wasi_http $name: tt $test:tt) => {};
    (wasi_io $name: tt $test:tt) => {};
    (world_has_go_keyword $name: tt $test:tt) => {};
    (worlds_with_types $name: tt $test:tt) => {};
    (zero_size_tuple $name: tt $test:tt) => {};




    ($id:ident $name:tt $test:tt) => {
        #[test]
        fn $id() {
            test_helpers::run_world_codegen_test(
                "guest-zig",
                $test.as_ref(),
                |resolve, world, files| {
                    wit_bindgen_zig::Opts {}
                        .build()
                        .generate(resolve, world, files)
                        .unwrap()
                },
                verify,
            )
        }
    };
}
test_helpers::codegen_tests!();

fn verify(dir: &Path, name: &str) {
    let name = name.to_snake_case();

    let main = dir.join(format!("{name}.go"));
    let _file = std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open(&main)
        .expect("failed to open file");

    let mut cmd = Command::new("zig");
    cmd.arg("build-exe");
    cmd.arg("-target");
    // .arg("-OReleaseSmall");
    cmd.arg("wasm32-freestanding");
    cmd.arg("-fno-entry");
    cmd.arg("-rdynamic");
    cmd.arg(format!("{name}.zig"));
    test_helpers::run_command(&mut cmd);
}
