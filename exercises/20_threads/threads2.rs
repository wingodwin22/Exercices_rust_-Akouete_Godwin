// threads2.rs
//
// Building on the last exercise, we want all of the threads to complete their
// work but this time the spawned threads need to be in charge of updating a
// shared value: JobStatus.jobs_completed
//
// Execute `rustlings hint threads2` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

/* Before correction
use std::sync::Arc;
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_completed: u32,
}

fn main() {
    // TODO: `Arc` isn't enough if you want a **mutable** shared state
    let status = Arc::new(JobStatus { jobs_completed: 0 });

    let mut handles = vec![];
    for _ in 0..10 {
        let status_shared = Arc::clone(&status);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));
            // TODO: You must take an action before you update a shared value
            status_shared.jobs_completed += 1;
        });
        handles.push(handle);
    }

    // Waiting for all jobs to complete
    for handle in handles {
        handle.join().unwrap();
    }

    // TODO: Print the value of `JobStatus.jobs_completed`
    println!("Jobs completed: {}", ???);
}
*/


// After correction
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_completed: u32,
}

fn main() {
    // Utilisation de Arc<Mutex<T>> pour une valeur partagée mutable
    let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));

    let mut handles = vec![];
    for _ in 0..10 {
        let status_shared = Arc::clone(&status);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));
            // Verrouillage du mutex avant de mettre à jour la valeur partagée
            let mut status = status_shared.lock().unwrap();
            status.jobs_completed += 1;
        });
        handles.push(handle);
    }

    // Attendre la fin de tous les threads
    for handle in handles {
        handle.join().unwrap();
    }

    // Impression de la valeur de jobs_completed
    println!("Jobs completed: {}", status.lock().unwrap().jobs_completed);
}

/// Explication
/// J'ai corrigé le code en utilisant Arc<Mutex<JobStatus>> pour permettre un accès partagé et mutable en toute sécurité entre plusieurs threads. 
/// J'ai ajouté un verrouillage du mutex avant d'incrémenter jobs_completed pour empêcher les conditions de concurrence.
/// Enfin, j'ai verrouillé le mutex pour lire jobs_completed après la fin de tous les threads, garantissant que l'état final est correctement synchronisé. 