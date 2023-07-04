use clap::ArgMatches;
use rand::Rng;

use crate::data_objects::sentence::Sentence;
use crate::settings::Settings;

use crate::actions;
use crate::traits::{displayer::Displayer, notifier::Notifier, storage::Storage};

fn str_to_hash(s: &str) -> Result<u64, std::num::ParseIntError> {
    u64::from_str_radix(s, 16).into()
}

pub struct Core<N, S, D>
where
    N: Notifier + Default,
    S: Storage + Default,
    D: Displayer<Sentence> + Default,
{
    notifier: N,
    storage: S,
    displayer: D,
}

impl<N: Notifier + Default, S: Storage + Default, D: Displayer<Sentence> + Default> Core<N, S, D> {
    pub async fn run(&mut self, arguments: ArgMatches, settings: Settings) {
        if let Some(true) = arguments.get_one::<bool>("list") {
            tracing::trace!("listing registered sentences");
            actions::list::list_sentences(&self.storage, &self.displayer)
                .await
                .expect("Listing sentences failed");
        }
        if let Some(hash) = arguments.get_one::<String>("edit") {
            tracing::trace!("Editing sentence {hash}");
            let hash = str_to_hash(hash).expect("Hash parsing failed");
            actions::edit::edit_sentence(&self.storage, hash, &settings)
                .await
                .expect("Could not edit sentence");
        }
        if let Some(true) = arguments.get_one::<bool>("add") {
            tracing::trace!("Adding a sentence");
            actions::add::add_sentence(&self.storage, &settings)
                .await
                .expect("Sentence addition failed");
        }
        if let Some(hash) = arguments.get_one::<String>("delete") {
            let hash = str_to_hash(hash).expect("Hash parsing failed");
            actions::delete::delete_sentence(&self.storage, hash)
                .await
                .expect("Sentence deletion failed");
        }
        if let Some(str_hash) = arguments.get_one::<String>("get") {
            let hash = str_to_hash(str_hash).expect("Hash parsing failed");
            let s = actions::get::get(&self.storage, hash)
                .await
                .expect("Sentence retrieving failed")
                .expect(&format!("Could not find sentence for hash {}", str_hash));
            self.displayer.display_item(s).await;
        }
        if arguments.get_one::<String>("get").is_none() {
            let get_arguments = arguments.get_many::<String>("get");
            if get_arguments.is_some() && get_arguments.unwrap().len() == 0 {
                let s = actions::get::get_random(&self.storage)
                    .await
                    .expect("Sentence retrieving failed")
                    .expect("Could not find a random sentence (storage is empty ?)");
                self.displayer.display_item(s).await;
            }
        }
        if let Some(true) = arguments.get_one::<bool>("daemon") {
            self.demon_mode(settings).await;
        }
    }

    pub async fn demon_mode(&self, settings: Settings) -> ! {
        // TODO : Put this in its own function
        let mut sentences = self
            .storage
            .get_all()
            .await
            .expect("Could not read data file");
        let mut last_edition_time = self.storage.get_last_edition_time().await.unwrap();
        if sentences.is_empty() {
            tracing::info!("No sentence could be retrieved, exiting");
            panic!();
        }
        let mut generator = rand::thread_rng();
        loop {
            let duration = tokio::time::Duration::from_secs(settings.duration.num_seconds() as u64);
            tokio::time::sleep(duration).await;
            let new_last_edition_time = self.storage.get_last_edition_time().await.unwrap();
            if new_last_edition_time != last_edition_time {
                sentences = self
                    .storage
                    .get_all()
                    .await
                    .expect("Could not read data file");
                last_edition_time = new_last_edition_time;
            }
            let s: Sentence = sentences[generator.gen::<usize>() % sentences.len()].clone();
            if let Err(_) = self.notifier.notify(s) {
                tracing::error!("Notifying the OS failed");
                panic!();
            }
        }
    }
}
impl<N, S, D> Default for Core<N, S, D>
where
    N: Notifier + Default,
    S: Storage + Default,
    D: Displayer<Sentence> + Default,
{
    fn default() -> Self {
        Self {
            notifier: Default::default(),
            storage: Default::default(),
            displayer: Default::default(),
        }
    }
}
