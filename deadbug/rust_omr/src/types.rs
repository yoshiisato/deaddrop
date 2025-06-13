pub type Payload = Vec<u8>; // x ∈ {0,1}^P
pub type Clue = Vec<u8>;
pub type BulletinBoard = Vec<(Clue, Payload)>;

pub type PKDetect = Vec<u8>; // Public key for detection
pub type PKClue = Vec<u8>; // Public key for clue generation
#[derive(Clone)]
pub struct PublicParams {
    pub lambda: usize,
    pub epsilon_p: f64,
    pub epsilon_n: f64,
}

#[derive(Clone)]
pub struct PublicKey {
    pub pk_clue: PKClue,
    pub pk_detect: PKDetect,
}

#[derive(Clone)]
pub struct SecretKey {
    pub sk_bytes: Vec<u8>,
}

pub enum DecodeResult {
    PayloadList(Vec<Payload>),
    Overflow,
}
