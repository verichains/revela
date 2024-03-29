use std::{
    collections::{BTreeMap, HashMap},
    path::PathBuf,
};

use move_binary_format::{
    binary_views::BinaryIndexedView, file_format::CompiledScript, CompiledModule,
};
use move_command_line_common::address::NumericalAddress;
use move_compiler::{compiled_unit::CompiledUnit, shared::known_attributes::KnownAttribute, Flags};

#[allow(dead_code)]
fn default_testing_addresses() -> BTreeMap<String, NumericalAddress> {
    let mapping = [
        ("std", "0x1"),
        ("NamedAddr", "0xbadbadbad"),
        ("aptos_framework", "0x1"),
        ("aptos_std", "0x1"),
        ("aptos_token", "0x1337"),
        ("Extensions", "0x1"),
        ("admin_addr", "0xbeef"),
        ("mint_nft", "0x1234"),
        ("source_addr", "0x2345"),
        ("core_resources", "0x3000"),
        ("vm_reserved", "0x3001"),
    ];
    mapping
        .iter()
        .map(|(name, addr)| (name.to_string(), NumericalAddress::parse_str(addr).unwrap()))
        .collect()
}

#[allow(dead_code)]
pub(crate) fn into_binary_indexed_view<'a>(
    scripts: &'a Vec<CompiledScript>,
    modules: &'a Vec<CompiledModule>,
) -> Vec<BinaryIndexedView<'a>> {
    let mut binaries: Vec<BinaryIndexedView<'a>> = Vec::new();

    binaries.extend(modules.iter().map(BinaryIndexedView::Module));
    binaries.extend(scripts.iter().map(BinaryIndexedView::Script));

    binaries
}

#[allow(dead_code)]
pub(crate) fn run_compiler(
    sources: Vec<&str>,
    flags: Flags,
    stdlib_as_sources: bool,
) -> (Vec<CompiledScript>, Vec<CompiledModule>) {
    let stdlib_files = move_command_line_common::files::find_filenames(
        &[
            aptos_framework::path_in_crate("aptos-stdlib/sources"),
            aptos_framework::path_in_crate("move-stdlib/sources"),
            aptos_framework::path_in_crate("aptos-framework/sources"),
            aptos_framework::path_in_crate("aptos-token/sources"),
        ],
        |p| {
            move_command_line_common::files::extension_equals(
                p,
                move_command_line_common::files::MOVE_EXTENSION,
            ) && !p.file_name().unwrap().to_str().unwrap().contains(".spec.")
        },
    )
    .unwrap();

    let stdlib_files_str = stdlib_files.iter().map(|f| f.as_str()).collect::<Vec<_>>();
    let (compiler_sources, compiler_stdlibs) = if stdlib_as_sources {
        (
            sources
                .iter()
                .chain(stdlib_files_str.iter())
                .cloned()
                .collect::<Vec<_>>(),
            Vec::<&str>::new(),
        )
    } else {
        (sources, stdlib_files_str)
    };

    let (files, units_res) = move_compiler::Compiler::from_files(
        compiler_sources,
        compiler_stdlibs,
        default_testing_addresses(),
        flags,
        KnownAttribute::get_all_attribute_names(),
    )
    .build()
    .expect("compiling failed");

    let (compiled_units, _warnings) = if units_res.is_ok() {
        units_res.unwrap()
    } else {
        move_compiler::diagnostics::unwrap_or_report_diagnostics(&files, units_res);
        panic!("compilation failed")
    };

    let (compiled_modules, compiled_scripts): (Vec<_>, Vec<_>) = compiled_units
        .into_iter()
        .map(|x| x.into_compiled_unit())
        .partition(|x| matches!(x, CompiledUnit::Module(_)));

    let modules: Vec<_> = compiled_modules
        .into_iter()
        .map(|x| match x {
            CompiledUnit::Module(m) => m.module,
            _ => unreachable!(),
        })
        .collect();

    let scripts: Vec<_> = compiled_scripts
        .into_iter()
        .map(|x| match x {
            CompiledUnit::Script(s) => s.script,
            _ => unreachable!(),
        })
        .collect();

    (scripts, modules)
}

#[allow(dead_code)]
pub(crate) fn tmp_project(tmp_files: Vec<(&str, &str)>, mut runner: impl FnMut(Vec<&str>)) {
    let manifest = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let tmp_dir = std::env::temp_dir();
    let project_root = tmp_dir.join(format!(
        "revela--test-project-{}",
        uuid::Uuid::new_v4()
    ));

    std::fs::create_dir(&project_root).unwrap();
    let tmp_files: Vec<_> = tmp_files
        .iter()
        .map(|(name, content)| {
            let path = project_root.join(name);
            std::fs::create_dir_all(path.parent().unwrap()).unwrap();
            std::fs::write(&path, content).unwrap();
            path
        })
        .collect();

    if !tmp_files
        .iter()
        .any(|x| x.file_name() == Some(std::ffi::OsStr::new("Move.toml")))
    {
        // copy "$MANIFEST/tests/Move.toml"  to project root
        let move_toml = PathBuf::from(manifest).join("tests/Move.toml");
        // copy the file
        let path = project_root.join("Move.toml");
        std::fs::copy(&move_toml, &path).unwrap();
    }

    runner(
        tmp_files
            .iter()
            .map(|x| x.to_str().unwrap())
            .collect::<Vec<_>>(),
    );

    // only remove the project root if the test passed
    std::fs::remove_dir_all(&project_root).unwrap();
}

#[allow(dead_code)]
// Compare output and output2 which has variables may be renamed
// all variables are in the form v\d+
pub(crate) fn assert_same_source(output: &String, output2: &String) {
    let s1 = output.as_bytes();
    let s2 = output2.as_bytes();

    let mut rename_map = HashMap::new();
    let (mut i, n) = (0, s1.len());
    let (mut j, m) = (0, s2.len());

    while i < n && j < m {
        if s1[i] == s2[j] {
            i += 1;
            j += 1;
        } else if i > 0 && j > 0 && s1[i - 1] == b'v' && s2[j - 1] == b'v' {
            let i0 = i;
            let j0 = j;
            let mut n1 = String::new();
            while i < n && (s1[i] as char).is_numeric() {
                n1.push(s1[i] as char);
                i += 1;
            }
            let mut n2 = String::new();
            while j < m && (s2[j] as char).is_numeric() {
                n2.push(s2[j] as char);
                j += 1;
            }
            if let Some(old_remap) = rename_map.get(&n1) {
                if &n2 != old_remap {
                    panic!(
                        "output and output2 are not the same\nOutput=====\n{}\n\nOutput2=====\n{}",
                        &output[i0..],
                        &output2[j0..]
                    );
                }
            } else {
                rename_map.insert(n1, n2);
            }
        } else {
            panic!(
                "output and output2 are not the same\nOutput=====\n{}\n\nOutput2=====\n{}",
                &output[i..],
                &output2[j..]
            );
        }
    }

    if i < n || j < m {
        panic!(
            "output and output2 are not the same\nOutput=====\n{}\n\nOutput2=====\n{}",
            &output[i..],
            &output2[j..]
        );
    }
}

#[allow(dead_code)]
pub(crate) fn should_same_script_bytecode(src_scripts: &[CompiledScript], scripts: &[CompiledScript]) {
    assert_eq!(src_scripts.len(), scripts.len());

    for (src_script, script) in src_scripts.iter().zip(scripts.iter()) {
        let mut binary = vec![];
        let mut binary2 = vec![];
        src_script.serialize(&mut binary).unwrap();
        script.serialize(&mut binary2).unwrap();
        assert_eq!(binary, binary2);
        // assert_eq!(src_script.as_inner(), script.as_inner());
    }
}

#[allow(dead_code)]
pub(crate) fn should_same_module_bytecode(src_modules: &[CompiledModule], modules: &[CompiledModule]) {
    assert_eq!(src_modules.len(), modules.len());

    for (src_module, module) in src_modules.iter().zip(modules.iter()) {
        let mut binary = vec![];
        let mut binary2 = vec![];
        src_module.serialize(&mut binary).unwrap();
        module.serialize(&mut binary2).unwrap();
        assert_eq!(binary, binary2);
        // assert_eq!(src_module.as_inner(), module.as_inner());
    }
}
