//! Tests for unstable flag `--no-finished-line` (fixme: issue number)

use cargo_test_support::{basic_bin_manifest, main_file, project};

#[cargo_test]
fn cargo_compile_simple_no_finished_line() {
    let p = project()
        .file("Cargo.toml", &basic_bin_manifest("foo"))
        .file("src/foo.rs", &main_file(r#""i am foo""#, &[]))
        .build();
    p.cargo("clean").run();
    // On clean build the finished line should display.
    p.cargo("build -Z unstable-options --no-finished-line")
        .masquerade_as_nightly_cargo(&["no-finished-line"])
        .with_stderr_contains("[FINISHED] [..]")
        .run();

    // On fresh  rebuild the finished line should not display.
    p.cargo("build -Z unstable-options --no-finished-line")
        .masquerade_as_nightly_cargo(&["no-finished-line"])
        .with_stderr_does_not_contain("[FINISHED] [..]")
        .run();
    assert!(p.bin("foo").is_file());

    p.process(&p.bin("foo")).with_stdout("i am foo\n").run();
}

#[cargo_test]
fn cargo_rustc_simple_no_finished_line() {
    let p = project()
        .file("Cargo.toml", &basic_bin_manifest("foo"))
        .file("src/foo.rs", &main_file(r#""i am foo""#, &[]))
        .build();
    p.cargo("clean").run();
    // On clean build the finished line should display.
    p.cargo("rustc -Z unstable-options --no-finished-line")
        .masquerade_as_nightly_cargo(&["no-finished-line"])
        .with_stderr_contains("[FINISHED] [..]")
        .run();

    // On fresh  rebuild the finished line should not display.
    p.cargo("rustc -Z unstable-options --no-finished-line")
        .masquerade_as_nightly_cargo(&["no-finished-line"])
        .with_stderr_does_not_contain("[FINISHED] [..]")
        .run();
    assert!(p.bin("foo").is_file());

    p.process(&p.bin("foo")).with_stdout("i am foo\n").run();

    p.cargo("rustc")
        .with_stderr_contains("[FINISHED] [..]")
        .run();
}

#[cargo_test]
fn cargo_run_simple_no_finished_line() {
    let p = project()
        .file("Cargo.toml", &basic_bin_manifest("foo"))
        .file("src/foo.rs", &main_file(r#""i am foo""#, &[]))
        .build();
    p.cargo("clean").run();
    // On clean build the finished line should display.
    p.cargo("run -Z unstable-options --no-finished-line")
        .masquerade_as_nightly_cargo(&["no-finished-line"])
        .with_stderr_contains("[FINISHED] [..]")
        .with_stderr_contains("[RUNNING] [..]")
        .with_stdout("i am foo\n")
        .run();
    assert!(p.bin("foo").is_file());

    // On fresh  rebuild the finished line should not display.
    p.cargo("run -Z unstable-options --no-finished-line")
        .masquerade_as_nightly_cargo(&["no-finished-line"])
        .with_stderr_does_not_contain("[FINISHED] [..]")
        .with_stderr_does_not_contain("[RUNNING] [..]")
        .with_stdout("i am foo\n")
        .run();
    assert!(p.bin("foo").is_file());

    p.cargo("run")
        .with_stderr_contains("[FINISHED] [..]")
        .with_stderr_contains("[RUNNING] [..]")
        .with_stdout("i am foo\n")
        .run();
}
