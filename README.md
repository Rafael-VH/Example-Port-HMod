# Horizon Mods — Puertos Fabric → Horizon (Rust)

Workspace para **portear mods Java (loader Fabric) de Minecraft** a **[Horizon](https://github.com/horizon-rust/horizon)**, la reimplementación de *Minecraft: Java Edition* en Rust, usando **[Horizon-Translator](https://github.com/merenotnoonereal/Horizon-Translator)**.

> Estado del repo: colección de puertos **iniciales/parciales** generados con Horizon-Translator (mc26.3). La mayoría produce un *bundle* con lo necesario para terminar el port; **solo Fabric API** generó un artefacto `.hnmod` listo para compilar/usar.

## Estructura del repositorio

Cada mod vive en su propia carpeta `<mod>-horizon/` y normalmente contiene:

- `PORTING.md` — **fuente de verdad** del estado: qué está activo, qué falta, TODOs, licencia y pasos.
- `mixins/` — configuración `horizon-mixin.toml` + código/runtime generado.
- `report/` — análisis de la conversión (`report.md/json`, `translation.md`, etc.).
- `todo/` — notas/stubs de piezas no mapeadas (cuando aplica).
- `Cargo.toml`, `compatibility.json`, JAR fuente original.
- `<modid>.hnmod` — **solo en puertos completos** (listo para instalar/build con el launcher de Horizon).

## Estado global

| Métrica | Valor |
|---|---:|
| Mods en el workspace | 26 |
| Con `.hnmod` (puerto completo listo) | 1 (`fabric_api-horizon`) |
| Parciales / pendientes de completar | 25 |
| Fuente de estado por mod | `*/PORTING.md` + `*/report/` |

## Estado por mod

> **Leyenda**
> - **Listo (`.hnmod`)**: Horizon-Translator marcó el port como completo (`Ready to use? Yes`) y generó `.hnmod`. Aun así, verifica `PORTING.md` antes de usarlo.
> - **Parcial**: no hay `.hnmod`; falta trabajo (mapeos de mixins/Horizon, lógica Java sin traducir, TODOs en código, y/o *data/assets* que requieren un content mod `.hmod`).
> - **Iny. on/off**: inyecciones activas / comentadas (según `PORTING.md`).
> - **Traducido**: métodos Java→Rust traducidos (según `PORTING.md`).
> - **TODOs (`todo/`)**: archivos en la carpeta `todo/` (no cuenta todos los TODOs inline; ver `PORTING.md`).

| Mod | Estado | `.hnmod` | Iny. on/off | Traducido (métodos) | TODOs en `todo/` | Qué falta (resumen) / contexto |
|---|---|:---:|:---:|:---:|:---:|---|
| `ambientsounds-horizon` | Parcial | No | 1/1 | 20/259 | 1 | Completar mixins/lógica pendiente; ver `PORTING.md`. |
| `betterend-horizon` | Parcial | No | 9/43 | 214/2864 | 43 | Muchos mixins sin mapeo + lógica/data/assets pendientes; ver `PORTING.md`. |
| `betternether-horizon` | Parcial | No | 3/35 | 186/2580 | 35 | Muchos mixins sin mapeo (LavaFluid, worldgen/blocks, etc.) + inicializador/registry pendientes; ver `PORTING.md`. |
| `chunky-horizon` | Parcial | No | 2/14 | 148/1013 | 14 | Completar mapeos/comandos/lógica; ver `PORTING.md`. |
| `continuity-horizon` | Parcial | No | 5/15 | 28/494 | 15 | Render/model mixins y lógica pendiente; ver `PORTING.md`. |
| `creativecore-horizon` | Parcial | No | 8/22 | 237/5325 | 22 | Librería grande; mucha lógica/API pendiente; ver `PORTING.md`. |
| `dynamictrees-horizon` | Parcial | No | 3/11 | 132/4210 | 11 | Sistemas de árboles/worldgen/lógica pendientes; ver `PORTING.md`. |
| `enhancedvisuals-horizon` | Parcial | No | 1/12 | 1/269 | 12 | Render/HUD/efectos mayormente pendientes; ver `PORTING.md`. |
| `entity_model_features-horizon` | Parcial | No | 4/96 | 97/961 | 96 | Alto nº de mixins de render/modelos sin mapeo; ver `PORTING.md`. |
| `entity_sound_features-horizon` | Parcial | No | 4/6 | 4/64 | 6 | Hooks de sonido/entidades pendientes; ver `PORTING.md`. |
| `entity_texture_features-horizon` | Parcial | No | 4/82 | 24/1075 | 82 | Alto nº de mixins de texturas/render sin mapeo; ver `PORTING.md`. |
| `fabric_api-horizon` | **Listo** | **Sí** (`fabric_api.hnmod`) | 1/0 | 0/0 | 0 | Puerto completo según translator. Instalar/compilar vía launcher de Horizon; ver `PORTING.md`. |
| `farmersdelight-horizon` | Parcial | No | 4/54 | 37/1723 | 53 | Contenido/juego (bloques/items/recipes/world) + mixins pendientes; ver `PORTING.md`. |
| `graveyard-horizon` | Parcial | No | 7/23 | 156/2382 | 23 | Entidades/estructuras/worldgen + mixins pendientes; ver `PORTING.md`. |
| `itemphysic-horizon` | Parcial | No | 2/22 | 2/53 | 22 | Física de ítems/render/entidades pendiente; ver `PORTING.md`. |
| `itemphysiclite-horizon` | Parcial | No | 1/6 | 1/16 | 6 | Física/render de ítems pendiente; ver `PORTING.md`. |
| `jeremyseqsdamageindicators-horizon` | Parcial | No | 6/0 | 4/48 | 0 | Sin `todo/`, pero hay TODOs inline (HUD/tick/render) y assets que requieren `.hmod`; ver `PORTING.md`. |
| `lithostitched-horizon` | Parcial | No | 2/116 | 42/1532 | 116 | Muy alto nº de hooks/worldgen/datapack sin mapeo; ver `PORTING.md`. |
| `mr_dynamic_light-horizon` | Parcial | No | 1/0 | 0/0 | 0 | Sin `.hnmod`; faltan data files/contenido (`.hmod`) según `PORTING.md`. |
| `mr_dynamic_lightscreepermeyt-horizon` | Parcial | No | 1/0 | 0/0 | 0 | Sin `.hnmod`; faltan data files/contenido (`.hmod`) según `PORTING.md`. |
| `presencefootsteps-horizon` | Parcial | No | 5/6 | 40/482 | 6 | Sonido/pasos/acústica + hooks pendientes; ver `PORTING.md`. |
| `sound_physics_remastered-horizon` | Parcial | No | 5/16 | 14/436 | 16 | Audio/oclusión/reverb + mixins pendientes; ver `PORTING.md`. |
| `storagedrawers-horizon` | Parcial | No | 4/0 | 41/2076 | 0 | Sin `todo/`, pero inicialización/registries sin traducir y **muchos data/assets** que requieren `.hmod`; ver `PORTING.md`. |
| `tectonic-horizon` | Parcial | No | 5/15 | 35/156 | 15 | Worldgen/terreno + hooks pendientes; ver `PORTING.md`. |
| `terralith-horizon` | Parcial | No | 4/0 | 5/39 | 0 | Sin `todo/`, pero lógica no traducida y **muchos data files** (worldgen) que requieren `.hmod`; ver `PORTING.md`. |
| `travelersbackpack-horizon` | Parcial | No | 11/22 | 37/1955 | 22 | Items/bloques/UI/red/network + mixins pendientes; ver `PORTING.md`. |

## Cómo interpretar y completar un puerto parcial

Según Horizon-Translator:

1. Abrir `PORTING.md` del mod y revisar **“What each part became”**, **TODOs/limits** y **licencia**.
2. Completar lo pendiente en `mixins/<modid>/` (inyecciones comentadas, `todo/*`, `game_side.rs`, `runtime.rs`, etc.) y/o aportar los **data/assets** mediante un content mod `.hmod` cuando el reporte lo indique.
3. Cuando el port esté completo, generar/obtener el `.hnmod` o compilar el bundle con el launcher de Horizon:
   ```bash
   horizon-launcher native-build "<carpeta-del-mod>" --trust-native-code
   ```
4. Validar en Horizon (launcher + build de native mods). Los native mods se compilan con permisos del usuario: **solo compilar código confiable**.

> Nota: Horizon-Translator genera `.hnmod` automáticamente solo en puertos completos; los parciales (la mayoría) quedan como bundle hasta terminar el trabajo.

## Contexto útil

- Horizon apunta a paridad con MC Java 26.3 y usa `horizon-mixins`/native mods; las APIs/mapeos dependen del commit/versión de Horizon usado por el translator (ver `PORTING.md` de cada mod).
- Antes de retomar un mod, revisa también `report/translation.md` y `report/report.json` dentro de su carpeta para priorizar trabajo real vs. bloqueos de mapeo/API.

## Próximos pasos sugeridos

- [ ] Elegir mods objetivo y priorizar por impacto/dependencias (p. ej. libs/base antes que contenido complejo).
- [ ] Auditar bloqueos comunes: mixins sin mapeo en Horizon, inicializadores Fabric, data/assets que requieren `.hmod`.
- [ ] Documentar decisiones/compatibilidad por mod (si se usa Engram, guardar bajo el proyecto `mods`).
- [ ] Actualizar esta tabla cuando cambie el estado de un mod (especialmente al generar `.hnmod`).
