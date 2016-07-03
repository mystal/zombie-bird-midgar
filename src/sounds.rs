use ears::Sound;


pub struct Sounds {
    pub flap: Sound,
    pub coin: Sound,
    pub dead: Sound,
}

impl Sounds {
    pub fn new() -> Self {
        Sounds {
            flap: Sound::new("assets/flap.wav").expect("Error on loading flap."),
            coin: Sound::new("assets/coin.wav").expect("Error on loading coin."),
            dead: Sound::new("assets/dead.wav").expect("Error on loading dead."),
        }
    }
}
