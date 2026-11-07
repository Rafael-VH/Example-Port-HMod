// Not ported yet: the Java @ModifyExpressionValue(INVOKE) on FenceGateBlock.<init> (LaunchpadFenceGateBlockMixin.fdrf$modifyOpenSound).
// Why it's off: No Horizon function mapped for FenceGateBlock.<init> (run `match`, or map it in a --mappings file).
// horizon-mixins kind: MODIFY_EXPRESSION_VALUE
// Note: Selects an exact Rust expression: pick it in the target function.
// Note: No Horizon function known for FenceGateBlock.<init>: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   Identifier.getPath -> (not mapped yet)
//   ResourceKey.identifier -> (not mapped yet)
//   FenceGateBlock.<init> -> (not mapped yet)
//   Properties.blockIdOrThrow -> (not mapped yet)
//
// Decompiled Java, vectorwing/farmersdelight/common/mixin/refabricated/LaunchpadFenceGateBlockMixin.java:14:
// | @ModifyExpressionValue(
// |    method = "<init>(Lnet/minecraft/world/level/block/state/properties/WoodType;Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;)V",
// |    at = @At(value = "INVOKE", target = "Lnet/minecraft/world/level/block/state/properties/WoodType;fenceGateOpen()Lnet/minecraft/sounds/SoundEvent;")
// | )
// | private static SoundEvent fdrf$modifyOpenSound(SoundEvent original, @Local Properties properties) {
// |    return properties.blockIdOrThrow().identifier().getPath().equals("rope_fence_gate") ? ModSounds.BLOCK_ROPE_FENCE_GATE_OPEN.get() : original;
// | }
