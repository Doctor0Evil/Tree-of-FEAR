# The Tree‑of‑FEAR

The Tree‑of‑FEAR is a non‑actuating observer layer that sits alongside Tree‑of‑Life and NATURE, turning **FEAR** and **PAIN** into bounded, diagnostic signals instead of weapons or control levers.[file:56][file:43]

It treats fear as a safety asset and teaching signal, never as a punishment channel.

---

## Core intent

- Map FEAR and PAIN into normalized TREE assets, clamped in \[0,1\], with RoH ≤ 0.30 preserved for any CapControlledHuman coupling.[file:56][file:43]  
- Derive FEAR‑droplet / “Tree‑of‑FEAR” structures as readonly diagnostics over logged trajectories, never as guards in CapabilityTransitionRequest or ReversalConditions.[file:56][file:44]  
- Keep all outputs advisory only: labels, scalars, and droplets that help humans see overload, hidden drain, and recovery corridors, with zero write‑paths into CapabilityState, ConsentState, BiophysicalEnvelopeSpec, or PolicyStack.[file:56][file:44]

---

## Relationship to Tree‑of‑Life and NATURE

- Tree‑of‑Life projects BiophysicalEnvelopeSnapshot, RoH, and CapabilityState into TREE assets BLOOD, OXYGEN, DECAY, LIFEFORCE, FEAR, PAIN, etc., all clamped to \[0,1\].[file:56][file:43]  
- NATURE defines four boolean predicates over TREE histories: CALMSTABLE, OVERLOADED, RECOVERY, UNFAIRDRAIN, all pure functions of logs.[file:43]  
- Tree‑of‑FEAR specializes this spine around FEAR and PAIN:
  - Detects rising FEAR/PAIN slopes under bounded RoH and envelopes.  
  - Tracks who is absorbing fear‑load for others (UNFAIRDRAIN) vs who is being allowed to recover (RECOVERY).[file:43][file:44]

---

## FEAR‑droplets and Fate windows

Tree‑of‑FEAR represents episodes of risk and recovery as FEAR‑droplets:

- Each droplet is a readonly snapshot over a time window:  
  - Inputs: normalized FEAR, PAIN, DECAY, LIFEFORCE traces, plus NATURE flags CALMSTABLE, OVERLOADED, RECOVERY, UNFAIRDRAIN.[file:43]  
  - Outputs: a small JSON/ALN object with start/end indices, peak FEAR/PAIN, minimum LIFEFORCE, and whether RECOVERY occurred.[file:43]  
- Droplets are logged to WORM streams aligned with `.evolve.jsonl` / `.donutloop.aln`, using the same hash‑linked, append‑only pattern and external Googolswarm anchoring.[file:56]  
- FEAR‑droplets carry `ROLE=DIAGNOSTIC_ONLY` and are forbidden from appearing as guards or capability writes.[file:56][file:44]

---

## Safety invariants

Tree‑of‑FEAR inherits and reinforces existing invariants:

- RoH ceiling: for CapControlledHuman, RoH is clamped to \[0.0, 0.30\], monotone, and non‑waivable at the schema level.[file:56]  
- Non‑actuation: Tree‑of‑FEAR modules MAY read logs and envelopes, and MAY emit diagnostics, but MUST NOT change CapabilityState, ConsentState, PolicyStack, ReversalPolicy, or envelopes.[file:56][file:44]  
- Guard separation: FEAR, PAIN, Tree‑of‑FEAR droplets, and NATURE predicates MUST NOT appear as direct guards in CapabilityTransitionRequest or ReversalConditions; they are permitted only as evidence for explanation and “no safer alternative” reasoning.[file:56][file:44]

---

## Roles and scope

Tree‑of‑FEAR is designed for:

- **OrganicallyIntegratedAugmentedCitizen**: teacher / learner / mentor / regulator pools that need clear, bounded language for fear, overload, and recovery without any hidden control channels.[file:44]  
- **Church‑of‑FEAR / Tree‑of‑FEAR stack**: AutoChurch‑style observers that read Tree‑of‑Life + NATURE logs, compute fairness and FEAR‑aware diagnostics, and emit advisory CHURCH suggestions (reflection, cooldown, disclosure), never GPU, money, or capability decisions.[file:56][file:44]  
- **MicroSociety**: 1D lattice experiments where CALMSTABLE, OVERLOADED, RECOVERY, and UNFAIRDRAIN are validated against FEAR/PAIN histories to ensure all predicates remain bounded and non‑actuating.[file:43]

---

## Repository layout

```text
Tree-of-FEAR/
  README.md              # This file
  aln/
    tree-of-fear-spec.aln        # SECTION/ROW spec for droplets, invariants
    fear-droplet-schema.aln      # JSONL/ALN shard for FEAR episodes
  src/
    lib.rs                        # Public crate surface (diagnostics only)
    droplets.rs                   # FEAR-droplet construction over TREE logs
    predicates.rs                 # FEAR/PAIN-derived CALMSTABLE/OVERLOADED variants
    nature_bridge.rs              # Glue to existing Tree-of-Life + NATURE views
  tests/
    droplets_tests.rs             # Boundedness, non-actuation, hash-chain tests
