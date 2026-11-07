# Port de referencia: `mr_dynamic_light` → `mr_dynamic_light.hmod`

Guía paso a paso (basada en trabajo real) para crear el content mod **`.hmod`**
de **Dynamic Light** a partir del port parcial generado por Horizon-Translator.
Sirve como ejemplo/referencia para otros ports con la misma clase de brecha
(data files que requieren `.hmod`).

> **Rama/commits de referencia:** `port/mr-dynamic-light` (base `f37c98a`).
> Cambios de implementación en `ccb0589` + artefactos SDD del change en
> `fdbe04d` (ver historial local).

## 1) Objetivo

Cerrar la brecha reportada por Horizon-Translator en
`mr_dynamic_light-horizon/PORTING.md`:

- **Estado original:** `Ready to use? No` — *"20 data files (recipes, loot,
  tags, worldgen…) need a content mod (.hmod), which this tool doesn't make."*
- **Meta:** empaquetar esos 20 data files en un `.hmod` válido para Horizon,
  documentar uso y registrar verificación pendiente.

## 2) Referencias usadas (fuentes de verdad)

| Referencia | Qué aportó | Dónde |
| --- | --- | --- |
| `mr_dynamic_light-horizon/PORTING.md` | Brecha exacta, estado del bundle nativo (1 inyección `mods` on, 0 off, 0 TODO, 0/0 métodos), commit Horizon pineado (`a46ce6898...`), licencia (CC0-1.0, Diesse) | Carpeta del mod |
| `mr_dynamic_light-horizon/compatibility.json` | Target Horizon/game y revisiones (`game: 26.3`, horizon/horizon-mixins commits) | Carpeta del mod |
| JAR fuente `dynamic-light-0.8.jar` | Contenido real a portar (data + metadata) | Carpeta del mod |
| Horizon docs `docs/mods.md` (horizon-rust/horizon) | Formato `.hmod`: `horizon-mod.toml` + `assets/` + `data/`, API 0.2, `capabilities=["registries"]`, namespaces, adiciones a `data/minecraft/tags/`; mención de `functions` en data/recarga | <https://github.com/horizon-rust/horizon/blob/main/docs/mods.md> |
| Horizon-Translator (comportamiento) | `.hmod` no lo genera el translator; parciales quedan como bundle | README del proyecto |
| SDD del cambio | Proposal/spec/design/tasks y riesgos | `openspec/changes/mr-dynamic-light/` |

## 3) Inspección y evidencia (antes de empaquetar)

### 3.1 Confirmar la brecha y el perfil del mod

- PORTING: **1/0** inyecciones, **0** `todo/`, **0/0** métodos traducidos → sin
  trabajo de mixin/código Java; brecha **solo contenido/data**.
- El bundle nativo solo lista el mod en Options > Mods
  (`horizon-mixin.toml` → `native_mods` HEAD).

### 3.2 Inventariar el JAR (conteo/paths)

Comando usado:

```powershell
tar -tf "mr_dynamic_light-horizon\dynamic-light-0.8.jar"
```

Resultado: **20 archivos bajo `data/`** (coincide con PORTING) + `pack.mcmeta`,
`pack.png`, `fabric.mod.json`, etc.

**Los 20 data files (paridad requerida):**

- `data/diesse/function/dl/` (9 `.mcfunction`): `destroy`, `glow_squid`,
  `light`, `light_5`, `light_reflection`, `main`, `place`, `place_light`,
  `reset`
- `data/diesse/predicate/` (7 `.json`): `glowing`, `light`, `light_5_m`,
  `light_5_o`, `light_m`, `light_o`, `on_fire`
- `data/minecraft/tags/function/` (2): `load.json`, `tick.json`
- `data/minecraft/tags/item/` (2): `light.json`, `light_5.json`

### 3.3 Clasificación del gap

- **Tipo:** contenido/data que requiere **content mod `.hmod`** (no mixin, no
  `.hnmod`).
- **Riesgo principal:** Horizon debe soportar `mcfunction`/`predicate` y
  ejecución de tags `minecraft:load/tick` en el flujo de data de `.hmod`.
  `mods.md` menciona funciones en datos/recarga; la compatibilidad real se
  **verifica en runtime**.

## 4) Extracción (T1)

Staging del `.hmod` conservando paths:

```powershell
$base="E:\Horizon\Mods\mr_dynamic_light-horizon"
$dest="$base\hmod_src"
New-Item -ItemType Directory -Force -Path "$dest\data" | Out-Null
tar -xf "$base\dynamic-light-0.8.jar" -C $dest data pack.mcmeta pack.png
```

**Verificación T1:** conteo/listado recursivo de `hmod_src\data` → **20
archivos**, paths idénticos al JAR. `pack.mcmeta`/`pack.png` conservados como
metadata/icono.

## 5) Manifiesto `horizon-mod.toml` (T2.1) — decisiones y lógica

Archivo: `mr_dynamic_light-horizon/hmod_src/horizon-mod.toml`

```toml
id = "mr_dynamic_light"
name = "Dynamic light"
version = "0.8.0"
description = "Mobs on fire and some items emit light (datapack port). Data/functions/predicates/tags packaged as Horizon content mod."
authors = ["Diesse"]
api = "0.2"
side = "both"
capabilities = ["registries"]
namespaces = ["diesse"]
```

**Lógica de cada decisión:**

- `api="0.2"` + `capabilities=["registries"]`: requerido para content mods
  (Horizon docs); manifests inválidos se rechazan.
- `id="mr_dynamic_light"`: alineado con fabric.mod.json. *Convivencia:* el
  bundle nativo usa el mismo id en `horizon-mixin.toml`; validar en Horizon si
  native + content pueden coexistir sin colisión.
- `namespaces=["diesse"]`: archivos propios en `data/diesse/...`.
- **`data/minecraft/tags/*`**: adiciones a tags vanilla permitidas por Horizon
  (`data/minecraft/tags/`); `minecraft` no se lista como namespace propio.
  Validar en carga real.
- `side="both"`: data/tags afectan lógica de mundo/juego; original declara
  `environment: "*"`.
- `version="0.8.0"`: normalización desde `0.8`.

## 6) Empaquetado `.hmod` (T2.2)

**Formato esperado:** `.hmod` con `horizon-mod.toml`, `assets/`, `data/` en raíz.

**Herramienta ideal:** `cargo horizon-mod build/check` (no disponible).
**Alternativa:** empaquetar como ZIP y renombrar a `.hmod` (`Compress-Archive`
solo admite `.zip`):

```powershell
$src = "mr_dynamic_light-horizon\hmod_src"
$tmp = "mr_dynamic_light-horizon\mr_dynamic_light.zip"
$out = "mr_dynamic_light-horizon\mr_dynamic_light.hmod"
Compress-Archive -Path "$src\*" -DestinationPath $tmp -CompressionLevel Optimal
Move-Item $tmp $out -Force
tar -tf $out | Sort-Object   # verificar contenido
```

**Resultado:** `mr_dynamic_light-horizon/mr_dynamic_light.hmod` (~7247 B) con
`horizon-mod.toml`, `data/` (20 archivos) y metadata (`pack.mcmeta`,
`pack.png`).

## 7) Validación (T2.3, T3, T4)

### Realizada

- [x] **Paridad de data:** 20/20 archivos, paths iguales al JAR.
- [x] **Estructura:** `horizon-mod.toml` en raíz + `data/` (`diesse`,
  `minecraft/tags`).
- [x] **Documentación:** actualizado `mr_dynamic_light-horizon/PORTING.md` y
  fila en `README.md`.

### No realizada (entorno / requiere Horizon)

- [ ] `cargo horizon-mod check mr_dynamic_light.hmod` — **no ejecutado**
  (`cargo-horizon-mod` ausente).
- [ ] **Runtime Horizon 26.3:** carga del `.hmod`, tags
  `#minecraft:light`/`#minecraft:light_5`, ejecución `minecraft:load`/`tick`,
  comportamiento de luz dinámica. **Bloqueador potencial** si Horizon no ejecuta
  `mcfunction`/`predicate` en content mods.

## 8) Resultado final

| Artefacto | Propósito |
| --- | --- |
| `mr_dynamic_light-horizon/hmod_src/` | Fuente editable del content mod |
| `mr_dynamic_light-horizon/mr_dynamic_light.hmod` | Paquete instalable (candidato, pendiente verificación runtime) |
| `mr_dynamic_light-horizon/PORTING.md` | Estado actualizado, uso, riesgos |
| `README.md` (raíz) | Estado del workspace actualizado |

**Uso previsto (validar):** colocar/activar `mr_dynamic_light.hmod` en
`horizon-mods/` (o launcher). El native bundle aporta el hook `mods`; verificar
convivencia de artefactos/IDs.

## 9) Plantilla reutilizable para otros ports (checklist)

Usar cuando PORTING indique *"data/assets need a content mod (.hmod)"*:

1. **Leer evidencia:** `PORTING.md`, `report/*`, `compatibility.json`, JAR.
2. **Clasificar brecha:** mixin/código vs data/assets vs bloqueador Horizon.
3. **Inventariar JAR:** `tar -tf mod.jar` → contar/clasificar `data/`,
   `assets/`, namespaces, adiciones a `minecraft/tags`.
4. **Extraer con paridad:** copiar preservando paths; verificar conteo.
5. **Definir manifiesto:** `horizon-mod.toml`
   (id/name/version/authors/api/side/capabilities/namespaces). Revisar
   colisiones con native bundle.
6. **Empaquetar:** `cargo horizon-mod build` (preferido) o ZIP→`.hmod` con
   `horizon-mod.toml`, `data/`, `assets/` en raíz.
7. **Validar:** `cargo horizon-mod check` si existe; inspección; runtime en
   Horizon 26.3.
8. **Documentar:** actualizar `PORTING.md` del mod y `README.md`; registrar
   checks y bloqueadores con evidencia.
9. **Riesgos típicos:** soporte Horizon para el tipo de data
   (functions/predicates/loot/worldgen/advancements), namespaces, versión de
   comandos/pack, diferencia `.hmod` vs `.hnmod`.

## 10) Notas de licencia/proveniencia

- Contenido original: **CC0-1.0** (Diesse), según `fabric.mod.json`/PORTING.
  Sin condiciones adicionales.
- Mantener atribución. Verificar licencia antes de redistribuir.

---
*Documento de referencia (SDD apply, T1–T3 completados; T4 runtime pendiente).*
