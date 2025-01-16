use std::fs::{self, File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;
use lru_cache::lru::{Cache, traits::CacheTrait};
use std::time::{SystemTime, UNIX_EPOCH};

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
        println!("Aucun cache existant trouvé. Création d'un nouveau cache.");
        return Ok(());
    }

    println!("Chargement du cache existant...");
    let file = File::open(cache_path)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        if let Some((key, value)) = line.split_once(':') {
            println!("Chargé: {} -> {}", key, value);
            cache.put(key.to_string(), value.to_string());
        }
    }

    Ok(())
}

fn get_timestamp() -> String {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
        .to_string()
}

fn main() -> io::Result<()> {
    // Créer un cache avec une capacité de 5 éléments
    let mut cache = Cache::new(5);
    
    // Charger les données existantes
    load_cache(&mut cache)?;
    
    // Ajouter de nouvelles données avec un timestamp
    let timestamp = get_timestamp();
    println!("\nAjout de nouvelles données avec timestamp {}:", timestamp);
    
    let new_key = format!("nouvelle_clé_{}", timestamp);
    let new_value = format!("nouvelle_valeur_{}", timestamp);
    
    println!("Ajout: {} -> {}", new_key, new_value);
    cache.put(new_key, new_value);
    
    // Sauvegarder le cache
    save_cache(&cache)?;
    
    println!("\nCache sauvegardé avec succès dans {}/{}!", CACHE_DIR, CACHE_FILE);
    println!("\nContenu actuel du cache:");
    for (key, value) in cache.iter() {
        println!("{}: {}", key, value);
    }
    
    Ok(())
} 