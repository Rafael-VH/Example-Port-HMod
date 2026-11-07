// Not ported yet: the Java @ModifyExpressionValue(INVOKE) on BlockEntityRenderState.extractBase (MixinBlockEntityRenderState.etf$vanillaLightOverride).
// Why it's off: No Horizon function mapped for BlockEntityRenderState.extractBase (run `match`, or map it in a --mappings file).
// horizon-mixins kind: MODIFY_EXPRESSION_VALUE
// Note: Selects an exact Rust expression: pick it in the target function.
// Note: No Horizon function known for BlockEntityRenderState.extractBase: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   BlockEntityRenderState.extractBase -> (not mapped yet)
//
// Decompiled Java, traben/entity_texture_features/mixin/mixins/entity/misc/MixinBlockEntityRenderState.java:45:
// | @ModifyExpressionValue(
// |    method = "extractBase",
// |    at = @At(
// |       value = "INVOKE",
// |       target = "Lnet/minecraft/util/LightCoordsUtil;getLightCoords(Lnet/minecraft/world/level/BlockAndLightGetter;Lnet/minecraft/core/BlockPos;)I"
// |    )
// | )
// | private static int etf$vanillaLightOverride(int light, @Local(argsOnly = true) BlockEntityRenderState blockEntityRenderState) {
// |    return ETF.config().getConfig().getLightOverrideBE(light, ((HoldsETFRenderState)blockEntityRenderState).etf$getState());
// | }
