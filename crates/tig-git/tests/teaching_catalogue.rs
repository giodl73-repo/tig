#![cfg(feature = "schema")]
use serde_json::Value;
use std::{fs, process::Command};
use tig_core::Object;
use tig_git::catalogue::{Catalogue, Config, storage::objects::Query};
#[path = "common/catalogue_server.rs"]
mod catalogue_server;

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
    assert_eq!(summaries["teaching_available"], true);
    check_lessons_and_refusals(&root, &catalogue);
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

fn check_lessons_and_refusals(root: &std::path::Path, catalogue: &Catalogue) {
    use tig_git::catalogue::teaching;
    let book = teaching::execute(catalogue.teaching_request().unwrap()).unwrap();
    assert_eq!(book["lessons"].as_array().unwrap().len(), 7);
    assert_eq!(book["authorizes_storage"], false);
    assert_eq!(book["independent_hardware_backup_proven"], false);
    let original = |index: usize| {
        book["lessons"][index]["objects"]
            .as_array()
            .unwrap()
            .iter()
            .find(|o| o["name"] == "original")
            .unwrap()
    };
    assert_eq!(
        original(3)["observed"],
        serde_json::json!(["backup", "working"])
    );
    assert_eq!(
        original(4)["observed"],
        serde_json::json!(["backup", "history"])
    );
    assert_eq!(
        original(5)["observed"],
        serde_json::json!(["backup", "working"])
    );
    assert_eq!(original(3)["object"], original(5)["object"]);
    let server = catalogue_server::Server::new(&root.join("catalogue.json"));
    assert_ne!(server.get("/api/lessons", false).0, 200);
    let (code, received) = server.json("/api/lessons");
    assert_eq!(code, 200);
    assert_eq!(received, book);
    let mut renderer = Command::new("node")
        .arg(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/ux/recorded-lesson.cjs"
        ))
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .expect("teaching UI qualification requires Node.js");
    {
        use std::io::Write;
        renderer
            .stdin
            .take()
            .unwrap()
            .write_all(&serde_json::to_vec(&received).unwrap())
            .unwrap();
    }
    let rendered = renderer.wait_with_output().unwrap();
    assert!(rendered.status.success(), "{rendered:?}");
    assert_ne!(server.json("/api/lessons?path=unadmitted").0, 200);
    let (code, script) = server.get("/lessons.js", false);
    assert_eq!(code, 200);
    assert!(script.contains("recorded-rehearsal"));
    drop(server);

    let original_config: Value =
        serde_json::from_slice(&fs::read(root.join("catalogue.json")).unwrap()).unwrap();
    let trace: Value = serde_json::from_slice(&fs::read(root.join("trace.json")).unwrap()).unwrap();
    let object: Object = serde_json::from_value(trace["completed"][0]["report"].clone()).unwrap();
    let path = tig_store::existing::Layout::Flat
        .object_path(&root.join("lesson-evidence"), &object.sha256);
    let raw = fs::read(&path).unwrap();
    fs::remove_file(&path).unwrap();
    assert!(teaching::execute(catalogue.teaching_request().unwrap()).is_err());
    fs::write(&path, b"corrupt").unwrap();
    assert!(teaching::execute(catalogue.teaching_request().unwrap()).is_err());
    fs::write(&path, &raw).unwrap();

    let run = |config: &Value| {
        let path = root.join("negative-catalogue.json");
        fs::write(&path, serde_json::to_vec(config).unwrap()).unwrap();
        let c = Catalogue::load(Config::load(&path).unwrap()).unwrap();
        teaching::execute(c.teaching_request().unwrap())
    };
    let mut swapped = original_config.clone();
    swapped["teaching"]["roles"]
        .as_array_mut()
        .unwrap()
        .swap(0, 1);
    assert!(run(&swapped).is_err());

    // Repin admitted synthetic negative metadata so semantic checks, rather
    // than the hash guard alone, must reject it. No payload store is modified.
    for mutation in ["schema", "plan", "role", "hardware", "size"] {
        let mut report: Value = serde_json::from_slice(&raw).unwrap();
        match mutation {
            "schema" => report["schema"] = "unsupported".into(),
            "plan" => {
                report["receipt"]["plan"] =
                    serde_json::to_value(Object::from_bytes(b"foreign").sha256).unwrap()
            }
            "role" => {
                report["observations"]["original"]["verified_copies"] =
                    serde_json::json!(["foreign"])
            }
            "hardware" => report["independent_hardware_backup_proven"] = true.into(),
            "size" => report["observations"]["original"]["object"]["bytes"] = 999.into(),
            _ => unreachable!(),
        }
        let mut edited = trace.clone();
        edited["completed"][0]["report"] = save_evidence(root, &report);
        let mut config = original_config.clone();
        config["teaching"]["trace"] = save_evidence(root, &edited);
        assert!(run(&config).is_err(), "accepted {mutation}");
    }
    let mut oversized = trace.clone();
    oversized["completed"] = Value::Array(vec![trace["completed"][0].clone(); 17]);
    let mut config = original_config;
    config["teaching"]["trace"] = save_evidence(root, &oversized);
    assert!(run(&config).is_err());
    assert_eq!(
        teaching::execute(catalogue.teaching_request().unwrap()).unwrap(),
        book
    );
}

fn save_evidence(root: &std::path::Path, value: &Value) -> Value {
    let raw = tig_core::identity::canonical_json(value).unwrap();
    let object = Object::from_bytes(&raw);
    let path = tig_store::existing::Layout::Flat
        .object_path(&root.join("lesson-evidence"), &object.sha256);
    fs::write(path, raw).unwrap();
    serde_json::to_value(object).unwrap()
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
