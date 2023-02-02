use std::{
    fs::File,
    io::Read,
};

use crate::{
    Lang,
    HasherError
};

pub struct SHA1 {
    lib_c: libloading::Library,
    lib_asm: libloading::Library,
    h: [u32; 5],
    lang: Lang,
}

impl SHA1 {
    pub fn new(lang: Lang) -> Result<Self, HasherError> {
        let mut h = [0; 5];
        h[0] = 0x67452301;
        h[1] = 0xEFCDAB89;
        h[2] = 0x98BADCFE;
        h[3] = 0x10325476;
        h[4] = 0xC3D2E1F0;

        Ok(SHA1 {
            lib_c: SHA1::load_library_c()?,
            lib_asm: SHA1::load_library_asm()?,
            h,
            lang
        })

    }

    pub fn from_file(&mut self, path: &str) -> Result<String, std::io::Error> {
        self.init_asm_array().expect("Cannot use procedure from shared object");

        let mut file = File::open(path)?;
        let mut original_length: usize = 0;
        let chunk_size: usize = 30*1024*1024;
        let mut chunk: Vec<u8> = Vec::with_capacity(chunk_size);

        loop {
            self.take_chunk_from_file(&mut file, &mut chunk)?;

            original_length = original_length.wrapping_add(chunk.len());

            if chunk.len() == chunk_size {
                self.process_chunk(&chunk);
            }
            else {
                self.handle_message_end(chunk, original_length);
                break;
            }
        }

        Ok(self.prepare_final_hash())
    }

    fn take_chunk_from_file(&self, file: &mut File, chunk: &mut Vec<u8>) -> Result<(), std::io::Error> {
        chunk.clear();
        file.by_ref().take(chunk.capacity() as u64).read_to_end(chunk)?;
        Ok(())
    }

    fn handle_message_end(&mut self, mut chunk: Vec<u8>, original_length: usize) {
        self.append_padding(&mut chunk);
        self.append_length_info(&mut chunk, original_length);

        self.process_chunk(&chunk);
    }

    fn append_padding(&self, chunk: &mut Vec<u8>) {
        chunk.push(128);

        while chunk.len() % 64 != 56 % 64 {
            chunk.push(0);
        }
    }

    fn append_length_info(&self, chunk: &mut Vec<u8>, original_length: usize) {
        let length: usize = 0_usize.wrapping_add(original_length * 8);
        let bytes = length.to_be_bytes();
        chunk.append(&mut bytes.to_vec());
    }

    fn process_chunk(&mut self, chunk: &[u8]) {
        for block in chunk.chunks(64) {
            let words = self.prepare_80_words(block);
            let buffers = self.compute_word_buffers(words);

            self.update_h_values(buffers);
        }
    }

    fn prepare_80_words(&self, block: &[u8]) -> [u32; 80] {
        let mut words: [u32; 80] = [0; 80];

        for i in 0..16 {
            let t = i*4;
            words[i] = u32::from_be_bytes(
                block[t..t + 4]
                .try_into()
                .expect("Wat? Shouldn't happen")
                );
        }
        for i in 16..80 {
            words[i] = (words[i - 3]^words[i - 8]^words[i - 14]^words[i - 16]).rotate_left(1);
        }

        words
    }

    fn compute_word_buffers(&self, words: [u32; 80]) -> [u32; 5] {
        let mut buffers = self.h.clone();

        match self.lang {
            Lang::C => {
                self.compute_buffer_values_c(&words, &mut buffers).expect("Cannot load shared object!");
            }
            Lang::Asm => {
                self.compute_buffer_values_asm(&words, &mut buffers).expect("Cannot load shared object!");
            }
        }

        buffers
    }
    
    fn update_h_values(&mut self, buffers: [u32; 5]) {
        for i in 0..buffers.len() {
            self.h[i] = self.h[i].wrapping_add(buffers[i]);
        }
    }

    fn prepare_final_hash(&self) -> String {
        format!("{:0>8x}{:0>8x}{:0>8x}{:0>8x}{:0>8x}", self.h[0], self.h[1], self.h[2], self.h[3], self.h[4])
    }

    fn init_asm_array(&self) -> Result<(), Box<dyn std::error::Error>> {
        unsafe {
            let func: libloading::Symbol<unsafe extern fn()> = self.lib_asm.get(b"init_asm")?;
            Ok(func())
        }
    }

    fn compute_buffer_values_asm(&self, words: &[u32], buffers: &mut [u32]) -> Result<(), Box<dyn std::error::Error>> {
        unsafe {
            let func: libloading::Symbol<unsafe extern fn(words: *const u32, buffers: *mut u32)> = self.lib_asm.get(b"compute_buffer_values_asm")?;
            Ok(func(words.as_ptr(), buffers.as_mut_ptr()))
        }
    }

    fn compute_buffer_values_c(&self, words: &[u32], buffers: &mut [u32]) -> Result<(), Box<dyn std::error::Error>> {
        unsafe {
            let func: libloading::Symbol<unsafe extern fn(words: *const u32, buffers: *mut u32)> = self.lib_c.get(b"compute_buffer_values_c")?;
            Ok(func(words.as_ptr(), buffers.as_mut_ptr()))
        }
    }

    fn load_library_c() -> Result<libloading::Library, HasherError> {
        unsafe {
            libloading::Library::new(".\\lib\\c\\sha_c.dll").map_err(|_| HasherError::CLibLoadingError)
        }
    }

    fn load_library_asm() -> Result<libloading::Library, HasherError> {
        unsafe {
            libloading::Library::new(".\\lib\\asm\\sha_asm.dll").map_err(|_| HasherError::AsmLibLoadingError)
        }
    }
}
