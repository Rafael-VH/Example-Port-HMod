// Not ported yet: the Java @Inject(HEAD) on EndSpike.getHeight (EndSpikeMixin.be_getSpikeHeight).
// Why it's off: No Horizon function mapped for EndSpike.getHeight (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for EndSpike.getHeight: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   CompoundTag.getInt -> (not mapped yet)
//   EndSpike.getHeight -> (not mapped yet)
//
// Decompiled Java, org/betterx/betterend/mixin/common/EndSpikeMixin.java:21:
// | @Inject(method = "getHeight", at = @At("HEAD"), cancellable = true)
// | private void be_getSpikeHeight(CallbackInfoReturnable<Integer> info) {
// |    if (!GeneratorOptions.isDirectSpikeHeight()) {
// |       int x = this.getCenterX();
// |       int z = this.getCenterZ();
// |       String pillarID = String.format("%d_%d", x, z);
// |       CompoundTag pillar = WorldConfig.getCompoundTag(BetterEnd.C, "pillars");
// |       int minY = pillar.getInt(pillarID).orElse(65);
// |       int maxY = minY + this.height - 54;
// |       info.setReturnValue(maxY);
// |    }
// | }
