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
    #[serde(skip_serializing_if = "no_wave_amount")]
    pub first_minion: WaveAmount,
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

fn no_wave_amount(wave: &WaveAmount) -> bool {
    wave.0 == 0
}
