use react_component_id::ComponentIdTransformer;
use std::path::PathBuf;
use swc_ecma_parser::{Syntax, TsConfig};
use swc_ecma_transforms_testing::{test_fixture, FixtureTestConfig};

fn syntax() -> Syntax {
    Syntax::Typescript(TsConfig {
        tsx: true,
        ..Default::default()
    })
}

#[testing::fixture("tests/fixture/**/input.tsx")]
fn fixture(input: PathBuf) {
    let output = input.parent().unwrap().join("output.tsx");
    test_fixture(
        syntax(),
        &|_| ComponentIdTransformer { id: 1 },
        &input,
        &output,
        FixtureTestConfig {
            ..Default::default()
        },
    );
}
