use base64::prelude::{Engine as _, BASE64_STANDARD};
use sha2::{Sha256, Digest};

#[derive(Clone,Debug)]
pub struct KeyBind {
    pub_key: String,
    identity: String,
    signature: String,
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

impl KeyBind {
    pub fn new(pub_key:String, identity:String, signature:String) -> KeyBind {
        KeyBind { pub_key, identity, signature }
    }
    pub fn to_string(&self) -> String {
        self.identity.clone() + &self.pub_key + &self.signature
    }
}

impl DataBlock {
    pub fn new(data: Vec<KeyBind>) -> DataBlock {
        let hash256 = Sha256::digest(keybind_vec_to_string(&data));
        DataBlock {
            data,
            hash:  BASE64_STANDARD.encode(&hash256),
        }
        
    }
    pub fn hash(&self) -> String {
        self.hash.clone()
    }
    pub fn to_string(&self) -> String {
        keybind_vec_to_string(&self.data)
    }
}