use std::io;
fn main() {
    println!("Comment est-ce que tu t'appelles ");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("TODO: panic message");
    println!("Bonjour {}", name);
    let mut todo = Vec::new();
    let mut todo_archive = Vec::new();
    loop {
        if todo.is_empty() {
            println!("aucune tache");
            println!("N: Créer une nouvelle tâche");
            println!("A: Voir les tâches archivées");
        } else {
            println!("liste de tache");
            for (tache, todos) in todo.iter().enumerate() {
                println!("{}. {}", tache + 1, todos);
            }

            println!("N: Créer une nouvelle tâche");
            println!("A: Voir les tâches archivées");
            let mut archive_choice = String::new();
            println!("Entrez le numéro de la tâche à archiver");

            io::stdin().read_line(&mut archive_choice).expect("un numéro de la todo pour l'archiver");

            if archive_choice.trim().to_uppercase() == "Q" {
                continue;
            }

            if let Ok(index) = archive_choice.trim().parse::<usize>() {
                if index > 0 && index <= todo.len() {
                    let archived_todo = todo.remove(index - 1);
                    todo_archive.push(archived_todo);
                    println!("Tâche archivée !");
                    println!("N: Créer une nouvelle tâche");
                    println!("A: Voir les tâches archivées");

                } else {
                    println!("Numéro de tâche invalide !");
                }
            } else {
                println!("Veuillez entrer un numéro valide !");
            }
        }
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("TODO: panic message");

        match choice.trim().to_string().to_uppercase().as_str() {
            "N" => {
                println!("Entrez la nouvelle tâche :");
                let mut new_todo = String::new();
                io::stdin()
                    .read_line(&mut new_todo)
                    .expect("Erreur lors de la lecture de la tâche");
                todo.push(new_todo.trim().to_string());
            }
            "A" => {
                if todo_archive.is_empty() {
                    println!("Aucune tâche archivée ");
                } else {
                    println!("Tâches archivées :");
                    for (archiv, todos) in todo_archive.iter().enumerate() {
                        println!("{}. {}", archiv + 1, todos);
                    }
                }
                let mut quit = String::new();
                println!("Appuyez sur Q pour revenir à la vue de base.");

                io::stdin().read_line(&mut quit).expect("Erreur lors de la lecture de l'entrée.");
                if quit.trim().to_uppercase() == "Q" {
                    continue;
                }
            }
            _ => {
                println!("Veuillez entrer une valeur correcte");
            }
        }
    }
}