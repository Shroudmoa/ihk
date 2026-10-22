use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

const SUBJECTS: [&str; 2] = ["IT", "Wirtschaft"];

const IT_CATEGORIES: [&str; 7] = [
    "Grundlagen & Hardware",
    "Netzwerk",
    "Sicherheit & Datenschutz",
    "Virtualisierung & Server",
    "Storage & Backup",
    "Cloud & Wirtschaftlichkeit",
    "Referenz",
];

const WIRTSCHAFT_CATEGORIES: [&str; 4] = [
    "Ausbildung & Arbeitsrecht",
    "Sozialversicherung",
    "Wirtschaft & Unternehmen",
    "Recht & Steuern",
];

fn categories(subject: &str) -> &'static [&'static str] {
    if subject == "IT" {
        &IT_CATEGORIES
    } else {
        &WIRTSCHAFT_CATEGORIES
    }
}

#[derive(Serialize, Deserialize, Default, Clone)]
struct Entry {
    title: String,
    #[serde(default)]
    category: String,
    #[serde(default)]
    summary: String,
    text: String,
}

fn main() {
    let cmd = std::env::args().nth(1).unwrap_or_default();
    let root = repo_root();
    let result = match cmd.as_str() {
        "" | "add" => add(&root),
        "build" => build(&root),
        "check" => check(&root),
        _ => {
            println!("inputer [add|build|check]");
            return;
        }
    };
    if let Err(e) = result {
        eprintln!("Fehler: {e}");
        std::process::exit(1);
    }
}

// --------------------------------------------------------------- add

fn add(root: &Path) -> Result<(), String> {
    let subject = loop {
        if let Some(s) = normalize_subject(&ask("Fach (IT/Wirtschaft): ")) {
            break s;
        }
        println!("bitte IT oder Wirtschaft eingeben");
    };

    let title = loop {
        let t = ask("Titel: ");
        if !t.is_empty() {
            break t;
        }
        println!("Titel darf nicht leer sein");
    };

    let cats = categories(subject);
    println!("Kategorie:");
    for (i, c) in cats.iter().enumerate() {
        println!("  {}. {c}", i + 1);
    }
    let category = loop {
        let a = ask("Nummer: ");
        if let Ok(n) = a.parse::<usize>() {
            if n >= 1 && n <= cats.len() {
                break cats[n - 1].to_string();
            }
        }
        if let Some(c) = cats.iter().find(|c| c.eq_ignore_ascii_case(a.trim())) {
            break c.to_string();
        }
        println!("bitte 1-{} wählen", cats.len());
    };

    let summary = ask("Ein Satz zu dem Thema: ");

    let text = read_text();
    if text.is_empty() {
        println!("abgebrochen, kein Text eingegeben");
        return Ok(());
    }

    let file = json_path(root, &subject);
    let mut entries: Vec<Entry> = load(&file)?;
    entries.push(Entry { title: title.clone(), category, summary, text: text.clone() });
    save(&file, &entries)?;

    let raw = rawinfos_path(root, &subject);
    append_raw(&raw, &title, &text)?;

    println!("gespeichert: {} + {}", file.display(), raw.display());
    render(root, subject)
}

fn normalize_subject(input: &str) -> Option<&'static str> {
    match input.trim().to_lowercase().as_str() {
        "it" => Some("IT"),
        "w" | "wi" | "wirt" | "wirtschaft" => Some("Wirtschaft"),
        _ => None,
    }
}

// --------------------------------------------------------- build/check

fn build(root: &Path) -> Result<(), String> {
    for subject in SUBJECTS {
        render(root, subject)?;
    }
    Ok(())
}

fn render(root: &Path, subject: &str) -> Result<(), String> {
    let tree = root.join("learningtree");
    let file = json_path(root, subject);
    if !file.exists() {
        println!("{} fehlt, übersprungen", file.display());
        return Ok(());
    }
    let entries: Vec<Entry> = load(&file)?;
    let slugs = slugs_of(&entries);

    let dir = tree.join(subject);
    fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    for (entry, slug) in entries.iter().zip(&slugs) {
        let page = format!("# {}\n\n{}\n", entry.title, entry.text);
        fs::write(dir.join(format!("{slug}.md")), page).map_err(|e| e.to_string())?;
    }

    let order = category_order(subject, &entries);
    let mut doc = format!("# {subject}\n\n> {} Themen in {} Kategorien.\n", entries.len(), order.len());
    for category in &order {
        let items: Vec<(&Entry, &String)> = entries
            .iter()
            .zip(&slugs)
            .filter(|(e, _)| effective_category(e) == *category)
            .collect();
        doc += &format!("\n## {category} ({})\n\n| Thema | In einem Satz |\n|---|---|\n", items.len());
        for (entry, slug) in items {
            let summary = if entry.summary.trim().is_empty() {
                "–"
            } else {
                entry.summary.trim()
            };
            doc += &format!("| [{}]({subject}/{slug}.md) | {summary} |\n", entry.title);
        }
    }

    let out = tree.join(format!("{subject}.md"));
    fs::write(&out, doc).map_err(|e| e.to_string())?;
    println!("{}: {} Themen, {} Kategorien", out.display(), entries.len(), order.len());
    Ok(())
}

fn check(root: &Path) -> Result<(), String> {
    let tree = root.join("learningtree");
    let mut problems = 0usize;

    for subject in SUBJECTS {
        let file = json_path(root, subject);
        if !file.exists() {
            continue;
        }
        let entries: Vec<Entry> = load(&file)?;
        let slugs = slugs_of(&entries);
        let dir = tree.join(subject);

        let mut seen = BTreeSet::new();
        for entry in &entries {
            if !seen.insert(entry.title.to_lowercase()) {
                println!("[{subject}] doppelter Titel: {}", entry.title);
                problems += 1;
            }
            if entry.summary.trim().is_empty() {
                println!("[{subject}] kein Ein-Satz: {}", entry.title);
                problems += 1;
            }
            for target in link_targets(&entry.text) {
                let local = target.split('#').next().unwrap_or("");
                if local.is_empty() || local.contains("://") || local.starts_with("mailto:") {
                    continue;
                }
                if !dir.join(local).exists() {
                    println!("[{subject}] toter Link in „{}“: {local}", entry.title);
                    problems += 1;
                }
            }
        }

        if let Ok(read) = fs::read_dir(&dir) {
            for item in read.flatten() {
                let name = item.file_name().to_string_lossy().to_string();
                if name.ends_with(".md") && !slugs.iter().any(|s| format!("{s}.md") == name) {
                    println!("[{subject}] verwaiste Datei: {subject}/{name}");
                    problems += 1;
                }
            }
        }
    }

    if problems == 0 {
        println!("alles sauber");
    } else {
        println!("{problems} Punkte gefunden");
    }
    Ok(())
}

fn effective_category(entry: &Entry) -> String {
    if entry.category.trim().is_empty() {
        "Sonstiges".to_string()
    } else {
        entry.category.trim().to_string()
    }
}

fn category_order(subject: &str, entries: &[Entry]) -> Vec<String> {
    let mut order: Vec<String> = categories(subject).iter().map(|c| c.to_string()).collect();
    for entry in entries {
        let category = effective_category(entry);
        if !order.contains(&category) {
            order.push(category);
        }
    }
    order
        .into_iter()
        .filter(|c| entries.iter().any(|e| effective_category(e) == *c))
        .collect()
}

fn slugs_of(entries: &[Entry]) -> Vec<String> {
    let mut used: Vec<String> = Vec::new();
    for entry in entries {
        let mut slug = slugify(&entry.title);
        let base = slug.clone();
        let mut n = 2;
        while used.contains(&slug) {
            slug = format!("{base}-{n}");
            n += 1;
        }
        used.push(slug);
    }
    used
}

fn link_targets(text: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut rest = text;
    while let Some(i) = rest.find("](") {
        rest = &rest[i + 2..];
        match rest.find(')') {
            Some(j) => {
                out.push(rest[..j].to_string());
                rest = &rest[j + 1..];
            }
            None => break,
        }
    }
    out
}

fn slugify(title: &str) -> String {
    let mut slug = String::new();
    for c in title.chars() {
        let mapped = match c {
            'ä' | 'Ä' => "ae",
            'ö' | 'Ö' => "oe",
            'ü' | 'Ü' => "ue",
            'ß' => "ss",
            c if c.is_alphanumeric() => {
                slug.push_str(&c.to_lowercase().to_string());
                continue;
            }
            _ => "-",
        };
        slug.push_str(mapped);
    }
    let mut out = String::new();
    for c in slug.chars() {
        if c == '-' && out.ends_with('-') {
            continue;
        }
        out.push(c);
    }
    out.trim_matches('-').to_string()
}

// ---------------------------------------------------------------- io

fn ask(prompt: &str) -> String {
    print!("{prompt}");
    let _ = io::stdout().flush();
    let mut buf = String::new();
    match io::stdin().read_line(&mut buf) {
        Ok(0) | Err(_) => String::new(),
        Ok(_) => buf.trim().to_string(),
    }
}

fn read_text() -> String {
    println!("Text (\"ACATS\" in eigener Zeile beendet):");
    let mut lines = Vec::new();
    loop {
        let mut buf = String::new();
        match io::stdin().read_line(&mut buf) {
            Ok(0) | Err(_) => break,
            Ok(_) => {
                let line = buf.trim_end_matches(['\n', '\r']);
                if line.trim().eq_ignore_ascii_case("acats") {
                    break;
                }
                lines.push(line.to_string());
            }
        }
    }
    lines.join("\n").trim().to_string()
}

fn repo_root() -> PathBuf {
    let cwd = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    let mut dir = cwd.clone();
    loop {
        if dir.join("learningtree").is_dir() {
            return dir;
        }
        if !dir.pop() {
            return cwd;
        }
    }
}

fn json_path(root: &Path, subject: &str) -> PathBuf {
    root.join("archive").join("jsonformat").join(format!("{subject}.json"))
}

fn rawinfos_path(root: &Path, subject: &str) -> PathBuf {
    root.join("archive").join("rawinfos").join(subject)
}

fn append_raw(path: &Path, title: &str, text: &str) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    let entry = format!("\n--- {title} ---\n{text}\n");
    let mut f = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .map_err(|e| e.to_string())?;
    f.write_all(entry.as_bytes()).map_err(|e| e.to_string())
}

fn load(path: &Path) -> Result<Vec<Entry>, String> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let raw = fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
    if raw.trim().is_empty() {
        return Ok(Vec::new());
    }
    serde_json::from_str(&raw).map_err(|e| format!("{}: {e}", path.display()))
}

fn save(path: &Path, entries: &[Entry]) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    let json = serde_json::to_string_pretty(entries).map_err(|e| e.to_string())?;
    fs::write(path, json + "\n").map_err(|e| e.to_string())
}
