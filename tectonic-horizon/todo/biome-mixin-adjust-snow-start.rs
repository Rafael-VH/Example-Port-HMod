// Not ported yet: the Java @ModifyVariable(HEAD) on Biome.getHeightAdjustedTemperature (BiomeMixin.tectonic$adjustSnowStart).
// Why it's off: No Horizon function mapped for Biome.getHeightAdjustedTemperature (run `match`, or map it in a --mappings file).
// horizon-mixins kind: MODIFY_VARIABLE
// Note: Selects a Rust local by name: pick it in the target function.
// Note: No Horizon function known for Biome.getHeightAdjustedTemperature: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   BlockPos.below -> (not mapped yet)
//   Biome.getHeightAdjustedTemperature -> (not mapped yet)
//
// Decompiled Java, dev/worldgen/tectonic/mixin/BiomeMixin.java:13:
// | @ModifyVariable(method = "getHeightAdjustedTemperature", at = @At("HEAD"), ordinal = 0, argsOnly = true)
// | private BlockPos tectonic$adjustSnowStart(BlockPos pos) {
// |    return pos == null ? null : pos.below(((ConfigState)Tectonic.CONFIG.getState()).general.snowStartOffset);
// | }
