use serde::{Deserialize, Serialize};

use crate::model::{SiteIndex, SiteSnapshot, Tick};

/// Enumerates all morally relevant deed kinds (justice-equivalent types).
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum DeedKind {
    // Core social / territorial actions.
    Help,
    Conflict,
    Colonize,
    Repair,

    // Addiction / pollution deeds.
    UseHabit,
    EmitPollution,
    DeployCleanTech,
    SupportCessation,
    BanEmission,
    RepairEnvironment,

    // Neuromorphic support / reflection.
    UseSupport,
    Abstain, // Explicit non-action in a morally loaded context.
}

/// Minimal cause context to keep provenance nonfictional.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CauseContext {
    /// Optional human-readable reason or rule label.
    pub rationale: String,
    /// Rule IDs fired in the decision engine (e.g., "rule_conflict_01").
    pub rule_ids: Vec<String>,
}

/// A single micro-unit: one deed plus complete pre/post state for all sites it touches.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Deed {
    pub tick: Tick,
    pub kind: DeedKind,
    /// Primary site index (actor, or focal location).
    pub primary_site: SiteIndex,
    /// Optional secondary site (target, neighbor, or affected region).
    pub other_site: Option<SiteIndex>,

    /// Pre-deed snapshots for all impacted sites.
    pub pre: Vec<SiteSnapshot>,
    /// Post-deed snapshots for all impacted sites.
    pub post: Vec<SiteSnapshot>,

    /// Cause and rule context for later judgment and W-cycles.
    pub cause: CauseContext,

    /// Cached deltas for quick justice / metric computation.
    pub delta_church: f64,
    pub delta_fear: f64,
    pub delta_power: f64,
    pub delta_tech: f64,
    pub delta_bioload: f64,
    pub delta_pollution: f64,
    pub delta_exposure: f64,
}

/// EventLog is an ordered, append-only ledger of Deeds.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct EventLog {
    pub deeds: Vec<Deed>,
}

impl EventLog {
    pub fn push(&mut self, deed: Deed) {
        self.deeds.push(deed);
    }

    pub fn iter(&self) -> impl Iterator<Item = &Deed> {
        self.deeds.iter()
    }
}
