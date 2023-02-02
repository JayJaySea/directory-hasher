use crate::{
    sha,
    Lang,
    HasherError
};
use std::thread;
use std::io::Write;
use std::fs;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;

pub struct Hasher {
    threads_max: u8,
    threads_num: u8,
    hashes: Vec<String>,
    sender: Sender<String>,
    receiver: Receiver<String>,
    lang: Lang
}

impl Default for Hasher {
    fn default() -> Hasher {
        let (sender, receiver) = mpsc::channel();
        Hasher {
            threads_max: 4,
            threads_num: 0,
            hashes: Vec::new(),
            sender,
            receiver,
            lang: Lang::C
        }
    }
}

impl Hasher {
    pub fn new(threads_max: u8, lang: Lang) -> Hasher {
        let (sender, receiver) = mpsc::channel();
        Hasher {
            threads_max,
            threads_num: 0,
            hashes: Vec::new(),
            sender,
            receiver,
            lang
        }

    }

    pub fn hash_files_in_directory(&mut self, input_path: &str, output_path: &str) -> Result<(), HasherError>  {
        let files = fs::read_dir(input_path).map_err(|_| HasherError::BadInputLocation)?;
        for file in files.filter_map(|p| p.ok()) {
            self.wait_if_max_threads();
            self.hash_file_async(file.path().display().to_string())?;
        }

        self.wait_for_remaining_threads();
        self.save_results_in_file(output_path)?;

        Ok(())
    }

    fn wait_if_max_threads(&mut self) {
        if self.threads_num >= self.threads_max {
            if let Ok(hash) = self.receiver.recv() {
                self.hashes.push(hash);
            }
        }

        self.threads_num += 1;
    }

    fn hash_file_async(&self, path: String) -> Result<(), HasherError> {
        let thread_sender = self.sender.clone();
        let lang = self.lang.clone();

        let mut sha = sha::SHA1::new(lang)?;
        thread::spawn(move || {
            let hashing_result = sha.from_file(&path);

            Self::send_hashing_result(thread_sender.clone(), hashing_result, path);
        });

        Ok(())
    }

    fn send_hashing_result(thread_sender: Sender<String>, hashing_result: Result<String, std::io::Error>, path: String) {
        if let Ok(hash) = hashing_result {
            thread_sender.send(format!("{}: {}\n", path, hash)).unwrap();
        }
        else {
            thread_sender.send(
                String::from(format!("{}: {}\n", path, "Unhashable item"))
            ).unwrap();
        }
    }

    fn wait_for_remaining_threads(&mut self) {
        while self.hashes.len() < self.threads_num as usize {
            if let Ok(hash) = self.receiver.recv() {
                self.hashes.push(hash);
            }
        }
    }

    fn save_results_in_file(&self, path: &str) -> Result<(), HasherError> {
        let mut file = std::fs::File::create(format!("{}/{}", path, "hashes.txt")).map_err(|_| HasherError::BadOutputLocation)?;
        for hash in &self.hashes {
            file.write_all(hash.as_bytes()).unwrap();
        }
        Ok(())
    }
}
