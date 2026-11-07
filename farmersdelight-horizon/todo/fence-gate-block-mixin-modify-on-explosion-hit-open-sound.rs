// Not ported yet: the Java @ModifyExpressionValue(INVOKE) on FenceGateBlock.onExplosionHit (FenceGateBlockMixin.fdrf$modifyOnExplosionHitOpenSound).
// Why it's off: No Horizon function mapped for FenceGateBlock.onExplosionHit (run `match`, or map it in a --mappings file).
// horizon-mixins kind: MODIFY_EXPRESSION_VALUE
// Note: Selects an exact Rust expression: pick it in the target function.
// Note: No Horizon function known for FenceGateBlock.onExplosionHit: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   FenceGateBlock.neighborChanged -> (not mapped yet)
//   FenceGateBlock.onExplosionHit -> (not mapped yet)
//   FenceGateBlock.useWithoutItem -> (not mapped yet)
//
// Decompiled Java, vectorwing/farmersdelight/common/mixin/refabricated/FenceGateBlockMixin.java:29:
// | @ModifyExpressionValue(
// |    method = "onExplosionHit",
// |    at = @At(value = "INVOKE", target = "Lnet/minecraft/world/level/block/state/properties/WoodType;fenceGateOpen()Lnet/minecraft/sounds/SoundEvent;")
// | )
// | private SoundEvent fdrf$modifyOnExplosionHitOpenSound(SoundEvent original) {
// |    return (FenceGateBlock)this instanceof RopeFenceGateBlock ? ModSounds.BLOCK_ROPE_FENCE_GATE_OPEN.get() : original;
// | }
