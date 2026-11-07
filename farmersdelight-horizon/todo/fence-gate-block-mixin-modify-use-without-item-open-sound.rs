// Not ported yet: the Java @ModifyExpressionValue(INVOKE) on FenceGateBlock.useWithoutItem (FenceGateBlockMixin.fdrf$modifyUseWithoutItemOpenSound).
// Why it's off: No Horizon function mapped for FenceGateBlock.useWithoutItem (run `match`, or map it in a --mappings file).
// horizon-mixins kind: MODIFY_EXPRESSION_VALUE
// Note: Selects an exact Rust expression: pick it in the target function.
// Note: No Horizon function known for FenceGateBlock.useWithoutItem: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   FenceGateBlock.neighborChanged -> (not mapped yet)
//   FenceGateBlock.onExplosionHit -> (not mapped yet)
//   FenceGateBlock.useWithoutItem -> (not mapped yet)
//
// Decompiled Java, vectorwing/farmersdelight/common/mixin/refabricated/FenceGateBlockMixin.java:13:
// | @ModifyExpressionValue(
// |    method = "useWithoutItem",
// |    at = @At(value = "INVOKE", target = "Lnet/minecraft/world/level/block/state/properties/WoodType;fenceGateOpen()Lnet/minecraft/sounds/SoundEvent;")
// | )
// | private SoundEvent fdrf$modifyUseWithoutItemOpenSound(SoundEvent original) {
// |    return (FenceGateBlock)this instanceof RopeFenceGateBlock ? ModSounds.BLOCK_ROPE_FENCE_GATE_OPEN.get() : original;
// | }
