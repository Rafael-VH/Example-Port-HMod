// Not ported yet: the Java @ModifyVariable(HEAD) on 2.modifyTemperature (TemperatureModifierMixin.tectonic$adjustSnowStart).
// Why it's off: No Horizon function mapped for 2.modifyTemperature (run `match`, or map it in a --mappings file).
// horizon-mixins kind: MODIFY_VARIABLE
// Note: Selects a Rust local by name: pick it in the target function.
// Note: No Horizon function known for 2.modifyTemperature: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   BlockPos.above -> (not mapped yet)
//   2.modifyTemperature -> (not mapped yet)
//
// Decompiled Java, dev/worldgen/tectonic/mixin/TemperatureModifierMixin.java:12:
// | @ModifyVariable(method = "modifyTemperature", at = @At("HEAD"), ordinal = 0, argsOnly = true)
// | public BlockPos tectonic$adjustSnowStart(BlockPos pos) {
// |    return pos == null ? null : pos.above(((ConfigState)Tectonic.CONFIG.getState()).general.snowStartOffset);
// | }
