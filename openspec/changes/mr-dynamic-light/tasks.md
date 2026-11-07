# Tasks: mr-dynamic-light

> Plan de implementaciÃ³n (SDD) para cerrar el port de `mr_dynamic_light-horizon`. Rama: `port/mr-dynamic-light`. Artefactos: `openspec/changes/mr-dynamic-light/` (+ Engram si disponible).

## Phase 1 â€” Baseline y extracciÃ³n (sin riesgo)

- [x] **T1.1** Confirmar baseline en rama `port/mr-dynamic-light` y estado limpio relevante (`git status`). Registrar commit base.
- [x] **T1.2** Extraer `data/`, `pack.mcmeta`, `pack.png` (si Ãºtil) del JAR a `mr_dynamic_light-horizon/content/` (o staging). Verificar **20 archivos** bajo `data/` y paths idÃ©nticos al JAR.
- [x] **T1.3** Revisar `docs/mods.md` de Horizon (content mods, namespaces, `data/minecraft/tags`) y anotar requisitos exactos de `horizon-mod.toml`/estructura `.hmod` para este caso (data-only, funciones/predicates/tags).

**Verify T1**: listado de archivos coincide con el JAR; notas de formato Horizon capturadas en design/PORTING draft.

## Phase 2 â€” Empaquetado `.hmod`

- [x] **T2.1** Crear manifiesto `horizon-mod.toml` para el content mod (id, name, version, api/capabilities/side/namespaces segÃºn Horizon). Resolver convivencia de id con el native bundle si aplica.
- [x] **T2.2** Estructurar `assets/`/`data/` segÃºn spec de Horizon y construir el `.hmod` (vÃ­a `cargo horizon-mod build` si el tooling aplica, o empaquetado equivalente documentado).
- [x] **T2.3** (pack generado; check no ejecutado: tooling ausente) Validar el artefacto: `cargo horizon-mod check <.hmod>` (si disponible) y/o inspecciÃ³n de contenido. Corregir errores de manifiesto/paths.

**Verify T2**: `.hmod` generado, contiene los 20 data files, pasa check (o se documenta bloqueador con evidencia).

## Phase 3 â€” IntegraciÃ³n y documentaciÃ³n

- [x] **T3.1** Actualizar `mr_dynamic_light-horizon/PORTING.md`: estado real (quÃ© se agregÃ³, cÃ³mo instalar `.hmod` + native bundle, limitaciones, commit/Horizon target).
- [x] **T3.2** Actualizar tabla/estado de `mr_dynamic_light-horizon` en `README.md` raÃ­z (y nota si pasa a â€œlisto/parcial resueltoâ€).
- [x] **T3.3** (hmod_src/ añadido; staging temporal no requerido) (Si aplica) AÃ±adir `.gitignore`/ubicaciÃ³n coherente para staging (`_jar_extracted`/build outputs) sin borrar fuentes necesarias.

**Verify T3**: docs reflejan pasos reproducibles y estado verificable.

## Phase 4 â€” VerificaciÃ³n runtime (manual/guiada)

- [ ] **T4.1** Definir prueba mÃ­nima en Horizon 26.3: mod cargado, tags presentes, comportamiento de luz dinÃ¡mica (Ã­tems/entidades en fuego/glow squid segÃºn predicates/functions) en mundo creativo.
- [ ] **T4.2** Ejecutar prueba y registrar resultado (Ã©xito, fallo, o bloqueado por soporte de functions/predicates en `.hmod`). Si bloqueado: documentar evidencia y alternativa.
- [ ] **T4.3** Si completo: considerar generaciÃ³n de `.hnmod`/paquete final segÃºn flujo Horizon (solo si aplica y estÃ¡ verificado) y actualizar estado a listo.

**Verify T4**: evidencia de carga/comportamiento o bloqueador documentado con pasos reproducidos.

## Dependencias / Notas
- Riesgo principal: soporte de `mcfunction`/`predicate`/tags `load|tick` en content mods de Horizon. Resolver en T1.3/T2 temprano.
- No commitear/pushear sin confirmaciÃ³n explÃ­cita del usuario (salvo lo que autorice el flujo acordado). Crear commits de unidad de trabajo solo si se solicita proceder con implementaciÃ³n.
- **Preflight SDD (sesiÃ³n)**: `interactive` | artifact store `both` | `auto-forecast` PRs | budget 800 lÃ­neas. Chain strategy: pendiente si se activa (preguntar).

## Definition of Done
- [ ] Brecha de `PORTING.md` (â€œ20 data files need .hmodâ€) cerrada con `.hmod` vÃ¡lido **o** bloqueador real documentado con evidencia y prÃ³ximo paso claro.
- [ ] `PORTING.md` y `README.md` actualizados.
- [ ] VerificaciÃ³n (tooling y/o runtime) registrada.

