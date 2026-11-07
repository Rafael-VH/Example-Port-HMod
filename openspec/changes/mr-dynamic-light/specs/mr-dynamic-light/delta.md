# Delta Spec: mr-dynamic-light

## ADDED Requirements

### REQ-DL-001: Data parity
The port MUST include all 20 vanilla datapack data files shipped in `dynamic-light-0.8.jar` under `data/`:
- `diesse:function/dl/*` (9 mcfunctions)
- `diesse:predicate/*` (7 predicates)
- `minecraft:tags/function/load.json`, `minecraft:tags/function/tick.json`
- `minecraft:tags/item/light.json`, `minecraft:tags/item/light_5.json`

Paths and JSON/mcfunction content MUST be preserved unless a Horizon 26.3 compatibility change is required; any change MUST be documented.

#### Scenario: Packaged data completeness
- GIVEN the source JAR `dynamic-light-0.8.jar`
- WHEN the `.hmod` is built
- THEN it contains the same 20 `data/` files at equivalent resource paths and the content matches (or diffs are documented as compatibility fixes).

### REQ-DL-002: Horizon content mod packaging
The missing data MUST be delivered as a Horizon content mod (`.hmod`) with a valid `horizon-mod.toml`/manifest per Horizon's content mod format, loadable without code execution beyond Horizon's supported data mechanisms.

#### Scenario: Valid content mod
- GIVEN the produced `.hmod`
- WHEN validated with Horizon's tooling (`cargo horizon-mod check`/equivalent) or loaded by Horizon
- THEN the mod is accepted or failures are diagnosed with evidence and tracked as blockers.

### REQ-DL-003: Documentation and state
The port MUST document what artifact to install (`.hmod` and/or existing native bundle), Horizon version/commit assumptions, and update workspace status.

#### Scenario: Operator can follow docs
- GIVEN `mr_dynamic_light-horizon/PORTING.md` and root `README.md`
- WHEN a user follows the instructions
- THEN they can identify required files, install/build steps, and known limitations/verification status.
