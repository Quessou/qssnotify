use clap::ArgMatches;
use rand::Rng;

use crate::data_objects::sentence::Sentence;
use crate::filesystem::{self, read};
use crate::settings::Settings;

use crate::actions;
use crate::traits::notifier::Notifier;

fn str_to_hash(s: &str) -> Result<u64, std::num::ParseIntError> {
    u64::from_str_radix(s, 16).into()
}

pub async fn run(arguments: ArgMatches, settings: Settings, notifier: Option<impl Notifier>) {
    if let Some(true) = arguments.get_one::<bool>("list") {
        tracing::trace!("listing registered sentences");
        actions::list::list_sentences().await.unwrap();
    }
    if let Some(hash) = arguments.get_one::<String>("edit") {
        tracing::trace!("Editing sentence {hash}");
        let hash = str_to_hash(hash).expect("Hash parsing failed");
        actions::edit::edit_sentence(hash, &settings)
            .await
            .expect("Could not edit sentence");
    }
    if let Some(true) = arguments.get_one::<bool>("add") {
        tracing::trace!("Adding a sentence");
        actions::add::add_sentence(&settings)
            .await
            .expect("Sentence addition failed");
    }
    if let Some(hash) = arguments.get_one::<String>("delete") {
        let hash = str_to_hash(hash).expect("Hash parsing failed");
        actions::delete::delete_sentence(hash)
            .await
            .expect("Sentence deletion failed");
    }
    if let Some(str_hash) = arguments.get_one::<String>("get") {
        let hash = str_to_hash(str_hash).expect("Hash parsing failed");
        let s = actions::get::get(hash)
            .await
            .expect("Sentence retrieving failed")
            .expect(&format!("Could not find sentence for hash {}", str_hash));
        println!("{}", s);
    }
    if arguments.get_one::<String>("get").is_none() {
        let get_arguments = arguments.get_many::<String>("get");
        if get_arguments.is_some() && get_arguments.unwrap().len() == 0 {
            println!("get without an arg !!!");
            let s = actions::get::get_random()
                .await
                .expect("Sentence retrieving failed")
                .expect("Could not find a random sentence (storage is empty ?)");
            println!("{}", s);
        }
    }
    if let Some(true) = arguments.get_one::<bool>("daemon") {
        let sentences = read::read_data_file(&filesystem::paths::get_data_file_path())
            .await
            .expect("Could not read data file");
        if sentences.is_empty() {
            tracing::info!("No sentence could be retrieved, exiting");
            return;
        }
        let mut generator = rand::thread_rng();
        loop {
            let duration = tokio::time::Duration::from_secs(settings.duration.num_seconds() as u64);
            tokio::time::sleep(duration).await;
            let s: Sentence = sentences[generator.gen::<usize>() % sentences.len()].clone();
            if let Err(_) = notifier.as_ref().unwrap().notify(s) {
                tracing::error!("Notifying the OS failed");
                panic!();
            }
        }
    }
}
