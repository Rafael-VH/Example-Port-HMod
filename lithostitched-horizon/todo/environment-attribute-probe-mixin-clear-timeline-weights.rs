// Not ported yet: the Java @Inject(HEAD) on EnvironmentAttributeProbe.tick (EnvironmentAttributeProbeMixin.clearTimelineWeights).
// Why it's off: No Horizon function mapped for EnvironmentAttributeProbe.tick (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for EnvironmentAttributeProbe.tick: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   EnvironmentAttributeProbe.lambda$tick$0 -> (not mapped yet)
//   EnvironmentAttributeProbe.tick -> (not mapped yet)
//
// Decompiled Java, dev/worldgen/lithostitched/mixin/common/timeline/EnvironmentAttributeProbeMixin.java:23:
// | @Inject(method = "tick", at = @At("HEAD"))
// | private void clearTimelineWeights(Level level, Vec3 position, CallbackInfo ci) {
// |    BiomeWeightingDuck duck = (BiomeWeightingDuck)this.biomeInterpolator;
// |    duck.lithostitched$clear();
// | }
