mod cli;
mod crypto;
mod domain;
mod error;
mod git;
mod watcher;

use domain::Key;
use sha2::{Digest, Sha256};
use std::path::{Path, PathBuf};
use tokio::sync::mpsc;

const WATCH_DIR: &str = "./notes";
const REPO_DIR: &str = "./xored_notes";
const KEY: &str = "ABCDEFGHIJKLMNOPQRSTUVWXY"; // Remplacez par votre clé 25 chars

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Gérer l'arrêt propre avec Ctrl+C
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        println!("\n✓ Arrêt du programme...");
        r.store(false, Ordering::SeqCst);
    })?;

    // Créer la clé
    let key = Key::from_str(KEY)?;

    // Canal pour les événements
    let (tx, mut rx) = mpsc::channel(100);

    // Démarrer la surveillance
    let watch_path = Path::new(WATCH_DIR);
    watcher::watch_directory(watch_path, tx).await?;

    println!("🔍 Surveillance de {} démarrée", WATCH_DIR);
    println!("📝 Les notes chiffrées seront dans {}", REPO_DIR);
    println!("⌨️  Appuyez sur Ctrl+C pour arrêter\n");

    // Boucle principale
    while running.load(Ordering::SeqCst) {
        // Timeout pour vérifier régulièrement si on doit s'arrêter
        match tokio::time::timeout(tokio::time::Duration::from_secs(1), rx.recv()).await {
            Ok(Some(file_path)) => {
                if let Err(e) = process_file_change(&file_path, &key).await {
                    eprintln!("❌ Erreur: {}", e);
                }
            }
            Ok(None) => break,  // Canal fermé
            Err(_) => continue, // Timeout, on continue
        }
    }

    println!("✓ Programme arrêté proprement");
    Ok(())
}

async fn process_file_change(
    file_path: &Path,
    key: &Key,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Fichier modifié: {:?}", file_path);

    // Lire le contenu
    let content = tokio::fs::read_to_string(file_path).await?;

    // Chiffrer et compresser
    let encrypted = crypto::process_file(&content, key)?;

    // Calculer le nom du fichier chiffré
    let file_name = file_path.file_name().unwrap().to_str().unwrap();
    let output_name = format!("{}.xor", file_name);
    let output_path = Path::new(REPO_DIR).join(&output_name);

    // Écrire le fichier chiffré
    tokio::fs::write(&output_path, encrypted.as_bytes()).await?;

    // Git add, commit et push
    let message = format!("Update {}", output_name);
    git::push_to_github(Path::new(REPO_DIR), &output_path, &message)?;

    println!("✓ {} chiffré et poussé sur GitHub", file_name);

    Ok(())
}
