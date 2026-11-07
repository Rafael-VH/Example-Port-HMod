# Design: mr-dynamic-light

## Overview
Cierre del port parcial de Dynamic Light. La brecha es **solo data** (20 archivos) — no hay mixins pendientes ni lógica Java por traducir. El diseño prioriza fidelidad al datapack original y compatibilidad con Horizon 26.3.

## Arquitectura / Artefactos

```text
mr_dynamic_light-horizon/
  dynamic-light-0.8.jar              # fuente (ya presente)
  mixins/mr_dynamic_light/           # native bundle existente (hook mods/list) — mantener
  horizon-mixin.toml, Cargo.toml, compatibility.json, report/, PORTING.md
  content/  (propuesto)              # fuente del content mod
    horizon-mod.toml
    data/
      diesse/function/dl/*.mcfunction
      diesse/predicate/*.json
      minecraft/tags/function/{load,tick}.json
      minecraft/tags/item/{light,light_5}.json
    pack.mcmeta (si requerido) / icono
  mr_dynamic_light.hmod (o <id>.hmod)  # artefacto construido
```

> Nota: la ubicación exacta (`content/` vs raíz, nombre del `.hmod`) se ajustará al formato real esperado por Horizon/`cargo horizon-mod`.

## Decisiones
1. **Preservar data original**: copiar byte/texto desde el JAR; no “mejorar” funciones.
2. **Namespaces**: declarar `diesse` y las adiciones necesarias para tags vanilla (`minecraft` tags) según reglas de Horizon (`docs/mods.md`: tags vanilla en `data/minecraft/tags/` permitidos como adiciones donde aplique). Verificar comportamiento real.
3. **Manifiesto**: content mod API 0.2, `capabilities = ["registries"]` si aplica al tipo de contenido; `id` estable (`mr_dynamic_light` o el que Horizon requiera) y sin colisión con el native mod — validar si pueden coexistir con mismo id o necesitan ids/nombres distintos.
4. **Native bundle**: conservar tal cual (ya funcional para listado). Si Horizon requiere integración específica entre `.hmod` data y native, documentar.

## Verificación (plan)
| Nivel | Qué | Cómo |
|---|---|---|
| Estática | Conteo/paths/data | Comparar listado JAR vs contenido empaquetado (20 files). |
| Tooling | Validez `.hmod` | `cargo horizon-mod check <file>` (si disponible) / build del mod. |
| Runtime | Carga + tags + tick | Cargar en Horizon 26.3, confirmar mod activo; probar items en `#minecraft:light`/`light_5` (si inspeccionable), ejecución `load`/`tick` (comportamiento de luz dinámica) en mundo de prueba. |
| Regresión doc | Estado | Actualizar `PORTING.md` + README raíz con resultado real. |

## Incertidumbres a resolver (bloqueadores potenciales)
- ¿Horizon `.hmod`/content pipeline carga `data/*/function` y `data/*/predicate` y ejecuta tags `minecraft:load`/`tick` como datapacks?
- Si no: ¿alternativa soportada? (data pack nativo de Horizon, mecanismo distinto, o implementación native). Esto define si el port puede cerrarse como “solo data”.

## Impacto en seguridad/licencia
- Data del mod (CC0-1.0 según `PORTING.md`). Sin ejecución de código nuevo fuera de mcfunctions/data; riesgo bajo. Mantener atribución/licencia en docs.
