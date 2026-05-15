#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct EnemyWaves {
    pub waves: Vec<EnemyWave>,
}

impl EnemyWaves {
    pub fn get_wave(&self, wave: WaveAmount) -> Option<&EnemyWave> {
        self.waves.get(wave.0)
    }
}

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct EnemyWave {
    #[serde(skip_serializing_if = "delay_is_default")]
    pub delay: Delay,
    #[serde(skip_serializing_if = "repeat_is_default")]
    pub repeat: Repeat,
    #[serde(skip_serializing_if = "no_wave_amount")]
    pub first_minion: WaveAmount,
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub struct Delay(f32);
impl Default for Delay {
    fn default() -> Self {
        Delay(4.0)
    }
}
impl Delay {
    pub fn value(&self) -> f32 {
        self.0
    }
}

#[derive(
    Debug, Default, PartialEq, PartialOrd, Clone, Copy, serde::Serialize, serde::Deserialize,
)]
pub struct Repeat(usize);
impl Repeat {
    pub fn count(&self) -> usize {
        self.0
    }
}

#[derive(Debug, Default, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub struct WaveAmount(usize);
impl WaveAmount {
    pub const fn new(amount: usize) -> Self {
        WaveAmount(amount)
    }
    pub fn amount(&self) -> usize {
        self.0
    }
}

fn delay_is_default(delay: &Delay) -> bool {
    *delay == Delay::default()
}

fn repeat_is_default(repeat: &Repeat) -> bool {
    *repeat == Repeat::default()
}

fn no_wave_amount(wave: &WaveAmount) -> bool {
    wave.0 == 0
}
