use aes::{Aes256,BlockEncrypt,BlockDecrypt};
use argon2::{Argon2,PasswordHash,Saltstring};
use block_modes::{Cbc,BlockMode};
use block_modes::block_padding::Pkcs7;
use rand::Rng;

 fn derive_key(master_password:&str,salt:&[u8]) -> Vec<u8>{
    let argon2=Argon2::default();
    let hash = argon2.hash_password(master_password.as_bytes(),salt).unwrap()
    hash.hash.as_bytes.to_vec()
 }
 fn encrypt_password(key:&[u8],password:&str)-> (Vec<u8>,Vec<u8>){
    let mut rng = rand::thread_rng;
    let mut iv = vec![0u8;16];
    rng.fill(&mut iv);

    let cipher = Cbc::<Aes256,Pkcs7>::new_from_slices(key,&iv).unwrap();    

    let password_bytes = password.as_bytes();
    let ciphertext = cipher.encrypt_vec(password_bytes);
 }

 fn decrypt_password(key)