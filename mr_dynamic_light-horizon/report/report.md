# Horizon port report: Dynamic light

`mr_dynamic_light` 0.8 · dynamic-light-0.8.jar · 0 classes

**Convertible**

| Status | Count | Meaning |
|---|---|---|
| auto | 0 | The Horizon hook or injection is known: the wiring can be generated. The Java body still has to become Rust. |
| content | 1 | Data and assets: horizon-port-pack carries these into a content mod (.hmod). |
| needs-mapping | 0 | The kind of change carries over, but the matching Horizon function isn't in the table yet. Run `match`. |
| needs-translation | 0 | No direct equivalent: a person (or an AI agent) has to rewrite this part. |
| unsupported | 0 | Horizon has nothing this can attach to yet. |

## Content

| What | In | Status | Note |
|---|---|---|---|
| 0 asset files, 20 data files | `jar resources` | content | Recipes, loot, tags, models, textures, worldgen: horizon-port-pack carries these. |

## Game methods and fields used

0 of 0 are mapped to Horizon. Every Java body that uses an unmapped one needs it mapped before its Rust can be written.

