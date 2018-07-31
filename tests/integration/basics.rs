extern crate predicates;

use helpers::project_builder::dir;

use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;
use chrono::offset::Local;
use chrono::Datelike;

#[test]
fn it_substitutes_projectname_in_cargo_toml() {
    let template = dir("template")
        .file(
            "Cargo.toml",
            r#"[package]
name = "{{project-name}}"
description = "A wonderful project"
version = "0.1.0"
"#,
        )
        .init_git()
        .build();

    let dir = dir("main").build();

    Command::main_binary()
        .unwrap()
        .arg("generate")
        .arg("--git")
        .arg(template.path())
        .arg("--name")
        .arg("foobar-project")
        .current_dir(&dir.path())
        .assert()
        .success()
        .stdout(predicates::str::contains("Done!").from_utf8());

    assert!(
        dir.read("foobar-project/Cargo.toml")
            .contains("foobar-project")
    );
}

#[test]
fn it_substitutes_cratename_in_a_rust_file() {
    let template = dir("template")
        .file(
            "main.rs",
            r#"
extern crate {{crate_name}};          
"#,
        )
        .init_git()
        .build();

    let dir = dir("main").build();

    Command::main_binary()
        .unwrap()
        .arg("generate")
        .arg("--git")
        .arg(template.path())
        .arg("--name")
        .arg("foobar-project")
        .current_dir(&dir.path())
        .assert()
        .success()
        .stdout(predicates::str::contains("Done!").from_utf8());

    let file = dir.read("foobar-project/main.rs");
    assert!(file.contains("foobar_project"));
    assert!(!file.contains("foobar-project"));
}
#[test]
fn it_substitutes_date_in_a_rust_file() {
    let template = dir("template")
        .file(
            "main.rs",
            r#"
//Created on {{date}}
//Crate is a magical wonderland          
"#,
        )
        .init_git()
        .build();

    let dir = dir("main").build();

    Command::main_binary()
        .unwrap()
        .arg("generate")
        .arg("--git")
        .arg(template.path())
        .arg("--name")
        .arg("foobar-project")
        .current_dir(&dir.path())
        .assert()
        .success()
        .stdout(predicates::str::contains("Done!").from_utf8());

    let file = dir.read("foobar-project/main.rs");
    let dt = Local::now();
    let test_date = format!("{}{:02}{:02}", dt.year(), dt.month(), dt.day());
    assert!(file.contains(&test_date));
    assert!(!file.contains("20180530"));
}