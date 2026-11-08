# Investigación de problemas y soluciones (ports Horizon)

Registro vivo de problemas encontrados al portar mods Java/Fabric a Horizon (26.3 / nightlies), con soluciones **completas**, **parciales** o **sin solución temporal**, para reutilizar en futuros ports.

## Cómo usar este registro

1. Antes de portar un mod, busca si su gap encaja en un problema existente (data/`mcfunction`, mixins, assets, worldgen, hooks nativos, etc.).
2. Registra evidencia (versión/nightly/commit de Horizon, logs redactados, comandos, resultado PASS/FAIL/BLOQUEADO).
3. Si aparece una solución funcional, actualiza el estado y enlaza el `PORTING.md`/evidencia del mod.

## Índice

| Archivo | Tema | Estado |
| --- | --- | --- |
| [`hmod-mcfunction-predicate-tags.md`](hmod-mcfunction-predicate-tags.md) | Data pack (`.mcfunction`, `predicate`, `tags/function load|tick`) dentro de `.hmod` / overlay | **Parcial / pendiente de confirmación runtime** (workaround funcional: datapack de mundo) |
| [`plantilla-problema.md`](plantilla-problema.md) | Plantilla para registrar nuevos problemas | N/A |

## Convenciones

- **Completa**: solución verificada en runtime con evidencia.
- **Parcial**: funciona en parte, requiere workaround, o depende de condiciones.
- **Sin solución temporal**: bloqueado por soporte/API de Horizon; documentar evidencia y criterio de reapertura.
- No marcar “listo” sin evidencia runtime real cuando el comportamiento dependa del runtime de Horizon.
- Redactar rutas locales/usuario en logs publicados dentro del repo cuando aplique.
