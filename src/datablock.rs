use base64::prelude::{Engine as _, BASE64_STANDARD};
use sha2::{Sha256, Digest};

pub const DATA_ENTRY_LIMIT: u8 = 255;
#[derive(Clone,Debug)]
pub struct KeyBind {
    pub_key: String,
    identity: String,
    privledge: String,
}

#[derive(Clone,Debug)]
pub struct DataBlock {
    data: Vec<KeyBind>,
    hash: String,
}

pub fn keybind_vec_to_string(data: &Vec<KeyBind>) -> String {
    let mut s = "".to_string();
    for bind in data.clone() {
        s = s + &bind.to_string();
    }
    s
}

impl ToString for KeyBind {
    fn to_string(&self) -> String {
        self.identity.clone() + &self.pub_key + &self.privledge
    }
}

impl ToString for DataBlock {
    fn to_string(&self) -> String {
        keybind_vec_to_string(&self.data)
    }
}

impl KeyBind {
    pub fn new(pub_key:String, identity:String, role:String) -> KeyBind {
        KeyBind { pub_key, identity, privledge: role }
    }
    pub fn identity(&self) -> &str {
        &self.identity
    }
    pub fn key(&self) -> &str {
        &self.pub_key
    }
    pub fn role(&self) -> &str {
        &self.privledge
    }
}

impl DataBlock {
    pub fn new(data: Vec<KeyBind>) -> DataBlock {
        if data.len() > DATA_ENTRY_LIMIT.into() {
            println!("Too much data in block.");
            println!("Continuing anyways.");
        }
        let hash256 = Sha256::digest(keybind_vec_to_string(&data));
        DataBlock {
            data,
            hash:  BASE64_STANDARD.encode(&hash256),
        }
        
    }
    pub fn hash(&self) -> String {
        self.hash.clone()
    }
    pub fn data_ref(&self) -> &Vec<KeyBind> {
        &self.data
    }
}