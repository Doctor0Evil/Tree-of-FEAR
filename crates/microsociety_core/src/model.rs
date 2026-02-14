use serde::{Deserialize, Serialize};

pub type SiteIndex = usize;
pub type Tick = u64;

/// Core scalar rails at each Jetson-Line site.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct TokenState {
    pub church: f64,
    pub fear: f64,
    pub power: f64,
    pub tech: f64,
}

/// Biophysical state for a site (Tree-of-Life trunk).
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct BiophysicalState {
    /// Normalized biophysical load in [0, 1].
    pub bioload: f64,
    /// Optional territorial / aggregate bioload view.
    pub computebioload: f64,
    /// Habit load Hi, normalized to [0, habit_capacity].
    pub habit_load: f64,
    /// Habit capacity H_max,i.
    pub habit_capacity: f64,
    /// Pollution stock Ei (normalized or unit-scaled).
    pub pollution: f64,
    /// Exposure dose Di (cumulative).
    pub exposure: f64,
}

/// Justice rails computed over Episodes; stored here for local snapshots.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct JusticeState {
    pub hpcc: f64, // Habit–Pollution–Health Coherence
    pub erg: f64,  // Exposure–Responsibility Gap
    pub tecr: f64, // Token-Enforced Collapse Rate
}

/// Optional trust / social ties relevant for CHURCH obligations.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct TrustState {
    /// Local trust in neighbors / attached communities [0, 1].
    pub trust: f64,
}

/// Full per-site state on the Jetson-Line.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SiteState {
    pub index: SiteIndex,
    pub occupied: bool,
    pub tokens: TokenState,
    pub bio: BiophysicalState,
    pub justice: JusticeState,
    pub trust: TrustState,
}

/// Global invariants and parameters (Neuromorph-GOD corridors).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalConstraints {
    /// RoH ceiling (risk-of-harm slice).
    pub roh_max: f64,
    /// DECAY ceiling.
    pub decay_max: f64,
    /// Territorial max bioload in [0, 1].
    pub max_bioload: f64,
    /// POWER ≤ k * CHURCH multiplier.
    pub power_church_k: f64,
    /// FEAR safe band [min, max].
    pub fear_min: f64,
    pub fear_max: f64,
}

/// World state for one Jetson-Line instance.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct World {
    pub tick: Tick,
    pub sites: Vec<SiteState>,
    pub constraints: GlobalConstraints,
}

/// Convenience snapshot for Deed logging (pre/post).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SiteSnapshot {
    pub index: SiteIndex,
    pub tokens: TokenState,
    pub bio: BiophysicalState,
    pub justice: JusticeState,
    pub trust: TrustState,
}

impl From<&SiteState> for SiteSnapshot {
    fn from(site: &SiteState) -> Self {
        Self {
            index: site.index,
            tokens: site.tokens,
            bio: site.bio,
            justice: site.justice,
            trust: site.trust,
        }
    }
}
