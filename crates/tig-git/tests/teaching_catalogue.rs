#![cfg(feature = "schema")]
use serde_json::Value;
use std::{fs, process::Command};
use tig_core::Object;
use tig_git::catalogue::{Catalogue, Config, storage::objects::Query};

#[test]
fn public_course_runs_real_git_and_providers_then_loads_the_catalogue() {
    let temp = tempfile::tempdir().unwrap();
    let root = temp.path().join("public sample");
    let external = temp.path().join("external");
    fs::create_dir(&external).unwrap();
    fs::write(external.join("keep"), b"unchanged external fixture").unwrap();
    let template = temp.path().join("template");
    fs::create_dir(&template).unwrap();
    fs::write(template.join("must-not-copy"), b"unadmitted template").unwrap();
    let global = temp.path().join("global.config");
    fs::write(
        &global,
        format!(
            "[init]\n\ttemplateDir = {}\n",
            template.display().to_string().replace('\\', "/")
        ),
    )
    .unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_tig-sample"))
        .arg("rehearse-ordinary")
        .arg(&root)
        .env("GIT_DIR", external.join("trap.git"))
        .env("GIT_WORK_TREE", &external)
        .env("GIT_INDEX_FILE", external.join("trap-index"))
        .env("GIT_TEMPLATE_DIR", &template)
        .env("GIT_CONFIG_GLOBAL", &global)
        .env("GIT_CONFIG_COUNT", "1")
        .env("GIT_CONFIG_KEY_0", "core.bare")
        .env("GIT_CONFIG_VALUE_0", "true")
        .output()
        .unwrap();
    assert!(output.status.success(), "{output:?}");
    assert!(output.stderr.is_empty());
    assert_eq!(
        fs::read(external.join("keep")).unwrap(),
        b"unchanged external fixture"
    );
    assert!(!external.join("trap.git").exists());
    assert!(!external.join("trap-index").exists());
    assert!(!root.join("project/.git/must-not-copy").exists());
    let result: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(result["completed_checkpoints"], 7);
    assert_eq!(result["complete_course"], false);
    let resolver: Value =
        serde_json::from_slice(&fs::read(root.join("resolver.json")).unwrap()).unwrap();
    let repository = tig_git::source::Repository::new(
        resolver["git_executable"].as_str().unwrap().into(),
        root.join("project"),
        tig_git::source::Limits::default(),
    )
    .unwrap();
    let heads = repository
        .named_commits()
        .unwrap()
        .into_iter()
        .filter(|(name, _)| name.starts_with("refs/heads/"))
        .map(|(name, _)| name)
        .collect::<Vec<_>>();
    assert_eq!(heads, vec!["refs/heads/main".to_owned()]);
    let trace: Value = serde_json::from_slice(&fs::read(root.join("trace.json")).unwrap()).unwrap();
    assert_eq!(trace["completed"].as_array().unwrap().len(), 7);
    let mut reports = std::collections::BTreeMap::new();
    for row in trace["completed"].as_array().unwrap() {
        let raw = fs::read(root.join(row["file"].as_str().unwrap())).unwrap();
        assert_eq!(
            serde_json::to_value(Object::from_bytes(&raw)).unwrap(),
            row["report"]
        );
        let report: Value = serde_json::from_slice(&raw).unwrap();
        assert_eq!(report["declared_fixture_domains"], 3);
        assert_eq!(report["independent_hardware_backup_proven"], false);
        reports.insert(row["lesson"].as_str().unwrap(), report);
    }
    assert_eq!(
        reports["merge-v2-job-active"]["observations"]["original"]["verified_copies"],
        serde_json::json!(["backup", "working"])
    );
    assert_eq!(
        reports["job-ended"]["observations"]["original"]["verified_copies"],
        serde_json::json!(["backup", "history"])
    );
    assert_eq!(
        reports["revert-v1"]["observations"]["original"]["verified_copies"],
        serde_json::json!(["backup", "working"])
    );
    assert_eq!(
        reports["return-v2"]["observations"]["fixed"]["verified_copies"],
        serde_json::json!(["backup", "working"])
    );
    let mut catalogue =
        Catalogue::load(Config::load(&root.join("catalogue.json")).unwrap()).unwrap();
    let summaries = catalogue.summaries();
    assert_eq!(summaries["projects"][0]["work_count"], 2);
    let commit = summaries["projects"][0]["commit"].as_str().unwrap();
    catalogue.observe_registered("sample", commit, "").unwrap();
    let inventory = catalogue
        .object_inventory(&Query {
            project: "sample",
            kind: "",
            usage: "",
            anomaly: "",
            primary: "",
            observed: "",
            q: "",
            sort: "bytes",
            offset: 0,
            sha: "",
        })
        .unwrap();
    assert_eq!(inventory["total"], 3);
    assert!(
        inventory["items"]
            .as_array()
            .unwrap()
            .iter()
            .all(|row| row["copies"].as_array().unwrap().len() == 2)
    );
}

#[test]
fn sample_discovery_and_existing_or_checkout_roots_cannot_mutate_data() {
    let cli = env!("CARGO_BIN_EXE_tig-sample");
    let described = Command::new(cli).arg("describe").output().unwrap();
    assert!(described.status.success());
    let temp = tempfile::tempdir().unwrap();
    fs::write(temp.path().join("keep"), b"keep").unwrap();
    let denied = Command::new(cli)
        .arg("rehearse-ordinary")
        .arg(temp.path())
        .output()
        .unwrap();
    assert!(!denied.status.success());
    assert_eq!(fs::read(temp.path().join("keep")).unwrap(), b"keep");
    fs::create_dir(temp.path().join(".git")).unwrap();
    let nested = temp.path().join("must-not-create");
    let denied = Command::new(cli)
        .arg("rehearse-ordinary")
        .arg(&nested)
        .output()
        .unwrap();
    assert!(!denied.status.success());
    assert!(!nested.exists());
}
