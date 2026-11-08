# `.hmod` / content mod: `mcfunction`, `predicate` y `tags/function` (`load`/`tick`)

## Resumen

- **Síntoma (T4, `mr_dynamic_light-horizon`)**: el `.hmod` carga en vivo (`content mods switched live`), pero `/function diesse:dl:reset` y `/function diesse:dl:main` devolvieron `argument.id.invalid` (“ID inválido”). No se confirmó ejecución de `minecraft:load|tick` ni paridad funcional.
- **Estado**: **Parcial / pendiente de confirmación runtime**. Existe una **solución funcional alternativa (datapack de mundo)**, de alta confianza por el pipeline de Horizon. La ruta de funciones desde overlay `.hmod` requiere re-verificación limpia porque el error observado contradice el parser de IDs de Horizon (6560641).

## Contexto del caso

- Mod: `mr_dynamic_light-horizon` (Fabric/datapack “Dynamic Light”; datos extraídos de `dynamic-light-0.8.jar`).
- Artefacto: `mr_dynamic_light.hmod` (20 archivos bajo `data/`): 9 `diesse:function/dl/*.mcfunction`, 7 `diesse:predicate/*.json`, tags `minecraft:load|tick`, `minecraft:item/light|light_5`.
- Manifiesto: `id=mr_dynamic_light`, `api=0.2`, `side=both`, `capabilities=["registries"]`, `namespaces=["diesse"]`.
- Runtime probado: Horizon Nightly `nightly-2026-11-07-6560641` (commit `6560641`), instancia Debug, mundo `t4_dynamic_light`, `.hmod` en `instances\debug\horizon-mods\`.
- Evidencia: `mr_dynamic_light-horizon/evidence/t4-runtime-debug-nightly-6560641.log` (redactada).

## Hallazgos en repos locales (Horizon / Translator)

Repos revisados:
- `E:\Horizon\Repositorios\horizon` @ `6560641ba51a452f6bc2407baa4eb1b3be517ca8` (coincide con nightly probado). Nota: verificación remota falló por TLS timeout; Translator actualizado `d0e3566`.
- `E:\Horizon\Repositorios\Horizon-Translator` @ `d0e3566` — sin fix runtime identificado para ejecución de `mcfunction` desde `.hmod`; el runtime es responsabilidad de Horizon.

### Pipeline de Horizon (confirmado en código)

1. **Data packs / funciones**
   - `crates/horizon-client-render/src/datapack.rs`: `DataStore::load` lee `function/*.mcfunction`, `tags/function/*.json`, `predicate/*.json`, `item_modifier/*.json` desde packs. `resource_id` soporta subrutas (`data/diesse/function/dl/reset.mcfunction` → `diesse:dl/reset`; tests con `mine:sub/b`). `content_id` cubre tags/recipes/loot/etc., pero functions/predicates/item_modifiers se cargan en `store.functions/tags/predicates/item_modifiers`.
   - `crates/horizon-client/src/game/functions.rs`: `pack_stack` usa builtin desde `options.resources` + datapacks de mundo/feature; `rebuild_store`/`reload_datapacks` recargan el store y sincronizan registros. `/function` resuelve contra `store.functions`/tags. `tick_functions` ejecuta `minecraft:load` (pendiente) y `minecraft:tick`.

2. **Content mods `.hmod` → overlay de recursos**
   - `crates/horizon-client/src/mod_content.rs` (`build`): archivos `data/` fuera de `data/<id>/horizon/`:
     - `data/*/tags/**.json` → fusionados (`TagMerge`) y escritos (incluye `data/minecraft/tags/function/{load,tick}.json`).
     - `assets/minecraft/atlases/*.json` → fusionados.
     - **Resto** (incluye `data/diesse/function/dl/*.mcfunction`, `data/diesse/predicate/*.json`) → `overlay.write(file, &bytes)` **copiados crudos** al overlay.
   - `game/mods.rs`/`resource_root`: overlay estampado como root de recursos; en live switch `mods_reloaded` asigna `options.resources` y llama `reload_datapacks(true)`. `docs/mods.md`: assets+data (incluye functions) aplican en vivo; `/reload` recarga datos. `live_switch` excluye solo worldgen/dimension/biome_modifier.

3. **Parser de IDs**
   - `horizon-command` (`ArgType::Function` → `read_id`) + `horizon-client-core::identifier::parse_identifier` aceptan `/` en paths y validan IDs tipo `diesse:dl/reset`. Contradice `argument.id.invalid` observado en T4 para `diesse:dl/{reset,main}`; requiere re-verificación (estado de prueba, overlay/store real, sugerencias/logs de carga).

## Soluciones

### Solución A — Re-verificación limpia del `.hmod` (pendiente)

**Tipo:** verificación / posible solución completa si confirma carga desde overlay.

1. Reiniciar Debug (evitar estado stale de live switch). Confirmar `mod.mr_dynamic_light:true`.
2. Abrir mundo de prueba, ejecutar `/reload`.
3. Revisar logs por `Horizon: failed to load function diesse:dl:*` o avisos de contenido.
4. Probar `/function diesse:dl/reset`, `/function diesse:dl/main`; confirmar sugerencias/completado de `diesse:`; validar `minecraft:load|tick` y comportamiento de luz.
5. Capturar evidencia (redactada). Actualizar `PORTING.md`/README/SDD solo con resultado observado.

**Criterio PASS:** IDs resolubles + funciones ejecutan + `load/tick` funcional + paridad razonable.  
**Criterio FAIL/BLOQUEADO:** IDs no cargan/resuelven con evidencia; pasar a Solución B o C.

### Solución B — Datapack de mundo (workaround funcional recomendado)

**Tipo:** **parcial a nivel de empaquetado `.hmod`**, pero **funcional para runtime** si Horizon carga datapacks (pipeline probado).

Estructura (`<world>/datapacks/mr_dynamic_light/` o `.zip`):
- `pack.mcmeta`
- `data/diesse/function/dl/*.mcfunction` (9)
- `data/diesse/predicate/*.json` (7)
- `data/minecraft/tags/function/load.json`
- `data/minecraft/tags/function/tick.json`
- Verificar/añadir tags de ítems requeridos por la lógica (`minecraft:item/light`, `light_5`, etc.) según paridad con el JAR.

Pasos: instalar en el mundo → habilitar → `/reload` → probar funciones, `load|tick`, comportamiento de luz dinámica. Documentar como ruta funcional en el `PORTING.md` del mod.

### Solución C — Reimplementación nativa (sin solución completa aún en este repo)

**Tipo:** alternativa si A/B fallan; **no implementada**.

Portar la lógica de los 9 `mcfunction` a hooks nativos de Horizon (tick/inventario jugador → detección de ítems de luz/predicados equivalentes → `setblock/fill`/limpieza). Alcance mayor (translator 0/0), depende de hook points/estado disponibles.

## Riesgos / notas

- `diesse` es namespace separado (declarado en `namespaces`); válido para content mod.
- Tags `minecraft:*` se fusionan: revisar convivencia con otros mods/datapacks.
- No asumir paridad runtime sin prueba observada. ZIP de `.hmod`: rutas internas pueden mostrarse con `\` en herramientas Windows, pero el contenido lógico es `data/...`; verificar si se sospecha de separadores.
- `cargo horizon-mod check` no disponible en este entorno (no ejecutado).

## Reapertura / siguiente acción

- Ejecutar **A** con captura diagnóstica (sugerencias `diesse:dl/*`, errores de carga, comando exacto). Si falla, ejecutar **B** y documentar estado funcional vía datapack.
- Si se confirma bug/limitación en Horizon, abrir/registrar issue upstream con evidencia mínima reproducible (nightly/commit, `.hmod`/datapack, logs, pasos).
