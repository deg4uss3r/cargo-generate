use cargo;
use console::style;
use emoji;
use indicatif::ProgressBar;
use liquid;
use quicli::prelude::*;
use std::fs;
use std::path::PathBuf;
use walkdir::WalkDir;
use placeholders;

fn engine() -> liquid::Parser {
    liquid::ParserBuilder::new().build()
}

pub fn substitute(name: &str) -> Result<liquid::Object> {
    let mut template = liquid::Object::new();
    template.insert(String::from("project-name"), liquid::Value::scalar(name));
    template.insert(
        String::from("crate_name"),
        liquid::Value::scalar(&hyphen_to_lodash(name)),
    );
    template.insert(
        String::from("authors"),
        liquid::Value::scalar(&cargo::get_authors()?),
    );
    template.insert(
        String::from("date"),
        liquid::Value::scalar(&placeholders::get_date()?),
    );
    template.insert(
        String::from("year"),
        liquid::Value::scalar(&placeholders::get_year()?),
    );
    Ok(template)
}

fn hyphen_to_lodash(string: &str) -> String {
    string.to_string().replace("-", "_")
}

pub fn walk_dir(project_dir: &PathBuf, template: liquid::Object, pbar: ProgressBar) -> Result<()> {
    let engine = engine();
    for entry in WalkDir::new(project_dir) {
        let entry = entry?;
        if entry.metadata()?.is_dir() {
            continue;
        }

        let filename = entry.path();
        pbar.set_message(&filename.display().to_string());

        let new_contents = engine
            .clone()
            .parse_file(&filename)?
            .render(&template)
            .with_context(|_e| {
                format!(
                    "{} {} `{}`",
                    emoji::ERROR,
                    style("Error replacing placeholders").bold().red(),
                    style(filename.display()).bold()
                )
            })?;
        fs::write(&filename, new_contents).with_context(|_e| {
            format!(
                "{} {} `{}`",
                emoji::ERROR,
                style("Error writing").bold().red(),
                style(filename.display()).bold()
            )
        })?;
    }
    pbar.finish_and_clear();
    Ok(())
}
