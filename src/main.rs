use std::collections::{BTreeMap, BTreeSet};
use std::env::var;
use std::fs::{create_dir, read_dir, write, File};
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};

use anyhow::{Context, Error};
use serde::Deserialize;
use serde_json::from_str;

fn main() {
    if let Err(e) = w_main() {
        let msg = e.to_string();
        write("error.log", msg).expect("failed to write error log");
    }
}

fn w_main() -> Result<(), Error> {
    let home = var("USERPROFILE").context("環境変数 USERPROFILE がありません。")?;

    let journal_dir = PathBuf::from(home)
        .join("Saved Games")
        .join("Frontier Developments")
        .join("Elite Dangerous");

    let mut visited_sets = BTreeMap::<String, BTreeSet<String>>::new();
    let mut cmdr_ids = BTreeMap::<String, String>::new();

    for node in read_dir(&journal_dir)
        .with_context(|| format!("フォルダ {:?} が開けません。", journal_dir))?
    {
        let node = node?;

        if !node.file_type()?.is_file() {
            continue;
        }

        let fname = node.file_name();
        let file_name = fname.to_string_lossy();
        if file_name.starts_with("Journal.") && file_name.ends_with(".log") {
            eprintln!("読み込み中: {}", file_name);
            let info = read_file(&node.path())?;

            if let Some(id) = info.user_id {
                cmdr_ids.insert(info.cmdr.clone(), id);
            }

            let set = visited_sets.entry(info.cmdr).or_insert_with(BTreeSet::new);
            set.extend(info.systems.into_iter());
        }
    }

    let out_path = PathBuf::from("./outputs");
    if !out_path.exists() {
        create_dir(&out_path)
            .with_context(|| format!("出力ディレクトリ {:?} の作成に失敗しました。", out_path))?;
    }

    for (cmdr, set) in visited_sets.iter() {
        // 空のセットをスキップしないと _Unknown が生成されてしまう。
        if set.is_empty() {
            continue;
        }

        let out_dir = out_path.join(escape_filename(cmdr));
        if !out_dir.exists() {
            create_dir(&out_dir).with_context(|| {
                format!("出力ディレクトリ {:?} の作成に失敗しました。", out_dir)
            })?;
        }

        let out_file = out_dir.join("ImportStars.txt");
        let f = File::create(&out_file)
            .with_context(|| format!("出力ファイル {:?} の作成に失敗しました。", out_file))?;
        let mut w = BufWriter::new(f);

        for sysytem in set {
            writeln!(w, "{}", sysytem)?;
        }
    }

    // ID一覧を出す
    let out_file = out_path.join("ids.txt");
    let f = File::create(&out_file)
        .with_context(|| format!("出力ファイル {:?} の作成に失敗しました。", out_file))?;
    let mut w = BufWriter::new(f);
    for (cmdr, id) in cmdr_ids.iter() {
        writeln!(w, "Cmdr:{}\tID:{}", cmdr, id)?;
    }

    Ok(())
}

fn read_file(path: &Path) -> Result<ScanInfo, Error> {
    let f = File::open(path).with_context(|| format!("ファイル {:?} が開けません", path))?;
    let r = BufReader::new(f);

    let mut cmdr = "_Unknown".to_owned();
    let mut user_id = None;
    let mut systems = Vec::new();

    for (i, line) in r.lines().enumerate() {
        let line = line?;

        let event: Event = from_str(&line)
            .with_context(|| format!("イベントのパースに失敗しました。 ({:?}:{})", path, i + 1))?;

        match event {
            Event::LoadGame(x) => cmdr = x.commander,
            Event::Commander(x) => {
                cmdr = x.name;
                user_id = x.fid;
            }
            Event::Location(x) => systems.push(x.star_system),
            Event::FSDJump(x) => systems.push(x.star_system),
            _ => {}
        }
    }

    Ok(ScanInfo {
        cmdr,
        user_id,
        systems,
    })
}

struct ScanInfo {
    cmdr: String,
    user_id: Option<String>,
    systems: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "event")]
enum Event {
    LoadGame(LoadGame),
    Commander(Commander),
    Location(Location),
    FSDJump(Location),
    #[serde(other)]
    Other,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct LoadGame {
    commander: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Commander {
    #[serde(rename = "FID")]
    fid: Option<String>,
    name: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Location {
    star_system: String,
}

fn escape_filename(name: &str) -> String {
    let mut escaped = String::with_capacity(name.len());

    for ch in name.chars() {
        match ch {
            ch if ch.is_ascii_alphanumeric() => escaped.push(ch),
            ch @ ' ' => escaped.push(ch),
            ch @ '_' => escaped.push(ch),
            ch @ '-' => escaped.push(ch),
            _ => escaped.push(' '),
        }
    }

    escaped
}
