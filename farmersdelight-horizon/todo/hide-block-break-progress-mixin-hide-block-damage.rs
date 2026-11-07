// Not ported yet: the Java @ModifyExpressionValue(FIELD) on LevelRenderer.submitBlockDestroyAnimation (HideBlockBreakProgressMixin.hideBlockDamage).
// Why it's off: No Horizon function mapped for LevelRenderer.submitBlockDestroyAnimation (run `match`, or map it in a --mappings file).
// horizon-mixins kind: MODIFY_EXPRESSION_VALUE
// Note: Selects an exact Rust expression: pick it in the target function.
// Note: No Horizon function known for LevelRenderer.submitBlockDestroyAnimation: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   LevelRenderer.submitBlockDestroyAnimation -> (not mapped yet)
//   BlockBreakingRenderState.blockState -> (not mapped yet)
//   BlockState.getBlock -> (not mapped yet)
//
// Decompiled Java, vectorwing/farmersdelight/common/mixin/HideBlockBreakProgressMixin.java:14:
// | @ModifyExpressionValue(
// |    method = "submitBlockDestroyAnimation",
// |    at = @At(value = "FIELD", target = "Lnet/minecraft/world/level/block/RenderShape;MODEL:Lnet/minecraft/world/level/block/RenderShape;", opcode = 178)
// | )
// | private RenderShape hideBlockDamage(RenderShape original, @Local(name = "state") BlockBreakingRenderState state) {
// |    return state.blockState().getBlock() == ModBlocks.CANVAS_RUG.get() ? null : original;
// | }
