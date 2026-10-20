use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

const SUBJECTS: [&str; 2] = ["IT", "Wirtschaft"];

#[derive(Serialize, Deserialize, Default)]
struct Entry {
    title: String,
    #[serde(default)]
    category: String,
    text: String,
}

fn main() {
    let cmd = std::env::args().nth(1).unwrap_or_default();
    let root = repo_root();
    let result = match cmd.as_str() {
        "" | "add" => add(&root),
        "build" => build(&root),
        _ => {
            println!("inputer [add|build]");
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

    let text = read_text();
    if text.is_empty() {
        println!("abgebrochen, kein Text eingegeben");
        return Ok(());
    }

    let category = ask("Kategorie (optional, leer = Allgemein): ");

    let file = json_path(root, &subject);
    let mut entries: Vec<Entry> = load(&file)?;
    entries.push(Entry { title: title.clone(), category, text: text.clone() });
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

// -------------------------------------------------------------- build

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

    let mut groups: Vec<(String, Vec<String>)> = Vec::new();
    let mut used: Vec<String> = Vec::new();

    for entry in &entries {
        let mut slug = slugify(&entry.title);
        let base = slug.clone();
        let mut n = 2;
        while used.contains(&slug) {
            slug = format!("{base}-{n}");
            n += 1;
        }
        used.push(slug.clone());

        let dir = tree.join(subject);
        fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
        let page = format!("# {}\n\n{}\n", entry.title, entry.text);
        fs::write(dir.join(format!("{slug}.md")), page).map_err(|e| e.to_string())?;

        let category = if entry.category.trim().is_empty() {
            "Allgemein".to_string()
        } else {
            entry.category.trim().to_string()
        };
        let link = format!("- [{}]({subject}/{slug}.md)", entry.title);
        match groups.iter_mut().find(|(c, _)| *c == category) {
            Some((_, links)) => links.push(link),
            None => groups.push((category, vec![link])),
        }
    }

    let mut doc = format!("# {subject}\n");
    for (category, links) in &groups {
        doc += &format!("\n## {category}\n\n");
        for link in links {
            doc += &format!("{link}\n");
        }
    }

    let out = tree.join(format!("{subject}.md"));
    fs::write(&out, doc).map_err(|e| e.to_string())?;
    println!(
        "{}: {} Themen in {} Kategorien",
        out.display(),
        entries.len(),
        groups.len()
    );
    Ok(())
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
                let lower = c.to_lowercase().to_string();
                slug.push_str(&lower);
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
