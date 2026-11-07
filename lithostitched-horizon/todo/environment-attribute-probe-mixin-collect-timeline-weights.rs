// Not ported yet: the Java @Inject(TAIL) on EnvironmentAttributeProbe.lambda$tick$0 (EnvironmentAttributeProbeMixin.collectTimelineWeights).
// Why it's off: No Horizon function mapped for EnvironmentAttributeProbe.lambda$tick$0 (run `match`, or map it in a --mappings file).
// horizon-mixins kind: RETURN
// Note: Horizon has no TAIL: RETURN runs at every exit, early ones too.
// Note: No Horizon function known for EnvironmentAttributeProbe.lambda$tick$0: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   EnvironmentAttributeProbe.lambda$tick$0 -> (not mapped yet)
//   EnvironmentAttributeProbe.tick -> (not mapped yet)
//
// Decompiled Java, dev/worldgen/lithostitched/mixin/common/timeline/EnvironmentAttributeProbeMixin.java:29:
// | @Inject(method = "lambda$tick$0", at = @At("TAIL"))
// | private void collectTimelineWeights(double weight, Holder<Biome> biome, CallbackInfo ci) {
// |    BiomeWeightingDuck duck = (BiomeWeightingDuck)this.biomeInterpolator;
// |    duck.lithostitched$accumulate(weight, biome);
// | }
