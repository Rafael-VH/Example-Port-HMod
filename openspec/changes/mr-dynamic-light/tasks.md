# Tasks: mr-dynamic-light

> Plan de implementación (SDD) para cerrar el port de `mr_dynamic_light-horizon`. Rama: `port/mr-dynamic-light`. Artefactos: `openspec/changes/mr-dynamic-light/` (+ Engram si disponible).

## Phase 1 — Baseline y extracción (sin riesgo)

- [x] **T1.1** Confirmar baseline en rama `port/mr-dynamic-light` y estado limpio relevante (`git status`). Registrar commit base.
- [x] **T1.2** Extraer `data/`, `pack.mcmeta`, `pack.png` (si útil) del JAR a `mr_dynamic_light-horizon/content/` (o staging). Verificar **20 archivos** bajo `data/` y paths idénticos al JAR.
- [x] **T1.3** Revisar `docs/mods.md` de Horizon (content mods, namespaces, `data/minecraft/tags`) y anotar requisitos exactos de `horizon-mod.toml`/estructura `.hmod` para este caso (data-only, funciones/predicates/tags).

**Verify T1**: listado de archivos coincide con el JAR; notas de formato Horizon capturadas en design/PORTING draft.

## Phase 2 — Empaquetado `.hmod`

- [x] **T2.1** Crear manifiesto `horizon-mod.toml` para el content mod (id, name, version, api/capabilities/side/namespaces según Horizon). Resolver convivencia de id con el native bundle si aplica.
- [x] **T2.2** Estructurar `assets/`/`data/` según spec de Horizon y construir el `.hmod` (vía `cargo horizon-mod build` si el tooling aplica, o empaquetado equivalente documentado).
- [x] **T2.3** (pack generado; check no ejecutado: tooling ausente) Validar el artefacto: `cargo horizon-mod check <.hmod>` (si disponible) y/o inspección de contenido. Corregir errores de manifiesto/paths.

**Verify T2**: `.hmod` generado, contiene los 20 data files, pasa check (o se documenta bloqueador con evidencia).

## Phase 3 — Integración y documentación

- [x] **T3.1** Actualizar `mr_dynamic_light-horizon/PORTING.md`: estado real (qué se agregó, cómo instalar `.hmod` + native bundle, limitaciones, commit/Horizon target).
- [x] **T3.2** Actualizar tabla/estado de `mr_dynamic_light-horizon` en `README.md` raíz (y nota si pasa a “listo/parcial resuelto”).
- [x] **T3.3** (hmod_src/ añadido; staging temporal no requerido) (Si aplica) Añadir `.gitignore`/ubicación coherente para staging (`_jar_extracted`/build outputs) sin borrar fuentes necesarias.

**Verify T3**: docs reflejan pasos reproducibles y estado verificable.

## Phase 4 — Verificación runtime (manual/guiada)

- [x] **T4.1** Prueba mínima definida y ejecutada en instancia **Debug**, Horizon Nightly `nightly-2026-11-07-6560641` (`6560641`), mundo creativo `t4_dynamic_light`: mod cargado, intento de funciones/tags (`diesse:dl/reset|main`, `minecraft:load/tick`).
- [x] **T4.2** Resultado **BLOQUEADO/FAIL (soporte runtime mcfunction)**: `content mods switched live: {} -> {"mr_dynamic_light"}` y reload OK; `/function diesse:dl:reset|main` → `argument.id.invalid` (“ID inválido”). No se confirma ejecución de `load/tick` ni paridad funcional. Evidencia: `mr_dynamic_light-horizon/evidence/t4-runtime-debug-nightly-6560641.log` (redactada) + `PORTING.md`.
- [ ] **T4.3** No aplicable aún (port no completo funcionalmente). Pendiente alternativa soportada por Horizon (datapack/runtime functions en `.hmod`, o reimplementación native) y re-verificación.

**Verify T4**: evidencia de carga + bloqueador de funciones documentado con pasos reproducidos.

## Dependencias / Notas
- Riesgo principal **confirmado**: soporte de `mcfunction`/`predicate`/tags `load|tick` en content mods `.hmod` de Horizon (nightly 6560641). Alternativa requerida para paridad.
- No commitear/pushear sin confirmación explícita del usuario. Crear commits de unidad de trabajo solo si se solicita proceder con implementación.
- **Preflight SDD (sesión)**: `interactive` | artifact store `both` | `auto-forecast` PRs | budget 800 líneas. Chain strategy: pendiente si se activa (preguntar).

## Definition of Done
- [x] Brecha de `PORTING.md` (“20 data files need .hmod”) cerrada a nivel de empaquetado; **bloqueador runtime real documentado** con evidencia y próximo paso claro (funciones mcfunction no resolubles en `.hmod`).
- [x] `PORTING.md` y `README.md` actualizados.
- [x] Verificación runtime registrada (T4 bloqueado con evidencia).
