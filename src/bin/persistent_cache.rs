use std::fs::{self, File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;
use lru_cache::lru::{Cache, traits::CacheTrait};

const CACHE_FILE: &str = "cache_data.txt";
const CACHE_DIR: &str = "cache";

fn save_cache(cache: &Cache<String, String>) -> io::Result<()> {
    // Créer le dossier cache s'il n'existe pas
    fs::create_dir_all(CACHE_DIR)?;
    
    let cache_path = Path::new(CACHE_DIR).join(CACHE_FILE);
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(cache_path)?;

    // Sauvegarder chaque paire clé-valeur
    for (key, value) in cache.iter() {
        writeln!(file, "{}:{}", key, value)?;
    }
    
    Ok(())
}

fn load_cache(cache: &mut Cache<String, String>) -> io::Result<()> {
    let cache_path = Path::new(CACHE_DIR).join(CACHE_FILE);
    
    // Si le fichier n'existe pas, on retourne sans erreur
    if !cache_path.exists() {
        return Ok(());
    }

    let file = File::open(cache_path)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        if let Some((key, value)) = line.split_once(':') {
            cache.put(key.to_string(), value.to_string());
        }
    }

    Ok(())
}

fn main() -> io::Result<()> {
    // Créer un cache avec une capacité de 5 éléments
    let mut cache = Cache::new(5);
    
    // Charger les données existantes
    load_cache(&mut cache)?;
    
    // Exemple d'utilisation
    cache.put("clé1".to_string(), "valeur1".to_string());
    cache.put("clé2".to_string(), "valeur2".to_string());
    cache.put("clé3".to_string(), "valeur3".to_string());
    
    // Sauvegarder le cache
    save_cache(&cache)?;
    
    println!("Cache sauvegardé avec succès dans {}/{}!", CACHE_DIR, CACHE_FILE);
    println!("Contenu du cache:");
    for (key, value) in cache.iter() {
        println!("{}: {}", key, value);
    }
    
    Ok(())
} 