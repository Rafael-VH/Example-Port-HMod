# Proposal: mr-dynamic-light

## Intent
Completar el port de **Dynamic Light** (`mr_dynamic_light`, JAR `dynamic-light-0.8.jar`, carpeta `mr_dynamic_light-horizon/`) para Horizon, cerrando la única brecha reportada por Horizon-Translator: los **20 data files** del mod deben entregarse como un **content mod `.hmod`** (Horizon no lo genera el translator).

## Contexto (evidencia)
- Rama de trabajo: `port/mr-dynamic-light`.
- `mr_dynamic_light-horizon/PORTING.md`: **1** inyección on / **0** off, **0** archivos en `todo/`, **0/0** métodos traducidos; `Ready to use? No` únicamente por: *“20 data files (recipes, loot, tags, worldgen…) need a content mod (.hmod), which this tool doesn't make.”*
- Contenido del JAR (20 archivos bajo `data/` + metadata): funciones `data/diesse/function/dl/*` (9), predicados `data/diesse/predicate/*` (7), tags `data/minecraft/tags/function/load.json|tick.json`, `data/minecraft/tags/item/light.json|light_5.json` (4), más `pack.mcmeta`, `pack.png`, `fabric.mod.json`, manifests loader.
- El bundle nativo existente (`mixins/mr_dynamic_light/*`, `horizon-mixin.toml`, `Cargo.toml`, `compatibility.json`) ya expone el hook `mods` (listado en Options > Mods). **No hay lógica Java a portar** según el reporte.

## Scope

### In scope
1. Extraer/aislar los data files del JAR fuente y verificar integridad (conteo/paths/namespaces).
2. Crear un **content mod `.hmod`** compatible con Horizon que incluya `data/` (y assets/metadata necesarios) con manifiesto válido.
3. Asegurar compatibilidad con Horizon 26.3 / commit pineado en `compatibility.json` (`a46ce6898...`) y reglas de namespaces/tags (incluyendo adiciones a `data/minecraft/tags/*` donde Horizon lo permita).
4. Documentar instalación/uso junto al bundle nativo existente y actualizar estado en `README.md` raíz cuando se cierre.
5. Validación: build/pack del `.hmod`, checks de Horizon (`cargo horizon-mod check` si disponible) y prueba manual/guiada en Horizon (load/tick/tag resolution) según factibilidad.

### Out of scope
- Reescribir la lógica del datapack (mcfunctions/predicates) salvo ajustes mínimos estrictamente necesarios para compatibilidad con Horizon 26.3.
- Portar otros mods.
- Publicación en registry/firma, salvo que se solicite.

## Approach
- Tratar el mod como **datapack data-driven**: empaquetar sus namespaces (`diesse`, y adiciones a `minecraft` tags) en un `.hmod` (API de content mod de Horizon).
- Mantener separado el **native bundle** existente (hook de listado) del **content `.hmod`**; documentar si Horizon requiere ambos activos o si basta uno (verificar en Horizon).
- Trabajar en rama `port/mr-dynamic-light`, con commits por unidad de trabajo (cambios + docs).

## Risks
| Riesgo | Impacto | Mitigación |
|---|---|---|
| Horizon content mods (`.hmod`) no soportan `function`/`predicate` como datapack vanilla | Alto (bloquea port fiel) | Verificar soporte en docs/código de Horizon; si no hay soporte, plan alterno: implementar equivalente como native mod/data hooks o marcar como bloqueado con evidencia. |
| Restricciones de namespaces/tags en `data/minecraft/tags/*` | Medio | Seguir reglas de `docs/mods.md` (adiciones a tags vanilla permitidas donde aplique); ajustar manifiesto `namespaces`/estructura. |
| Diferencias de comandos/mcfunction entre versión origen y Horizon 26.3 | Medio | Revisar funciones; smoke test en mundo; registrar incompatibilidades en `PORTING.md`. |
| Formato exacto de `horizon-mod.toml`/`.hmod` para contenido puro de data | Medio | Basar en `docs/mods.md` y ejemplos de Horizon; validar con `cargo horizon-mod check`. |

## Rollback
Revertir commits de la rama `port/mr-dynamic-light` y/o eliminar el `.hmod` generado; no afecta otros mods.

## Acceptance Criteria
- [ ] Existe un `.hmod` (o artefacto equivalente documentado) que contiene los 20 data files del mod con namespaces/paths correctos.
- [ ] El manifiesto del content mod pasa validación de Horizon (o se documenta el bloqueador real con evidencia).
- [ ] `mr_dynamic_light-horizon/PORTING.md` y `README.md` raíz reflejan el estado actualizado y pasos de uso.
- [ ] Prueba de carga/ejecución en Horizon realizada o plan de verificación manual documentado con pasos y resultado esperado.
