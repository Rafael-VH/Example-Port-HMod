// Not ported yet: the Java @Inject(TAIL) on HumanoidMobRenderer.extractHumanoidRenderState (HumanoidMobRendererMixin.fdrf$extractSkilletEntityRenderState).
// Why it's off: No Horizon function mapped for HumanoidMobRenderer.extractHumanoidRenderState (run `match`, or map it in a --mappings file).
// horizon-mixins kind: RETURN
// Note: Horizon has no TAIL: RETURN runs at every exit, early ones too.
// Note: No Horizon function known for HumanoidMobRenderer.extractHumanoidRenderState: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   HumanoidMobRenderer.extractHumanoidRenderState -> (not mapped yet)
//   LivingEntity.getUseItem -> (not mapped yet)
//   ItemStack.getOrDefault -> (not mapped yet)
//
// Decompiled Java, vectorwing/farmersdelight/common/mixin/refabricated/HumanoidMobRendererMixin.java:16:
// | @Inject(method = "extractHumanoidRenderState", at = @At("TAIL"))
// | private static void fdrf$extractSkilletEntityRenderState(
// |    LivingEntity entity, HumanoidRenderState reusedState, float partialTick, ItemModelResolver itemModelResolver, CallbackInfo ci
// | ) {
// |    ((SkilletEntityRenderState)reusedState)
// |       .fdrf$setSkilletFlipTimestamp((Long)entity.getUseItem().getOrDefault(ModDataComponents.SKILLET_FLIP_TIMESTAMP.get(), -1L));
// | }
