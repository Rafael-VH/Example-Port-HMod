// Not ported yet: the Java @ModifyExpressionValue(INVOKE) on FenceGateBlock.neighborChanged (FenceGateBlockMixin.fdrf$modifyNeighborChangedCloseSound).
// Why it's off: No Horizon function mapped for FenceGateBlock.neighborChanged (run `match`, or map it in a --mappings file).
// horizon-mixins kind: MODIFY_EXPRESSION_VALUE
// Note: Selects an exact Rust expression: pick it in the target function.
// Note: No Horizon function known for FenceGateBlock.neighborChanged: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   FenceGateBlock.neighborChanged -> (not mapped yet)
//   FenceGateBlock.onExplosionHit -> (not mapped yet)
//   FenceGateBlock.useWithoutItem -> (not mapped yet)
//
// Decompiled Java, vectorwing/farmersdelight/common/mixin/refabricated/FenceGateBlockMixin.java:53:
// | @ModifyExpressionValue(
// |    method = "neighborChanged",
// |    at = @At(value = "INVOKE", target = "Lnet/minecraft/world/level/block/state/properties/WoodType;fenceGateClose()Lnet/minecraft/sounds/SoundEvent;")
// | )
// | private SoundEvent fdrf$modifyNeighborChangedCloseSound(SoundEvent original) {
// |    return (FenceGateBlock)this instanceof RopeFenceGateBlock ? ModSounds.BLOCK_ROPE_FENCE_GATE_CLOSE.get() : original;
// | }
