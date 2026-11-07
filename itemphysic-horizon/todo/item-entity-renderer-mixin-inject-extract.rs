// Not ported yet: the Java @Inject(TAIL) on ItemEntityRenderer.extractRenderState (ItemEntityRendererMixin.injectExtract).
// Why it's off: No Horizon function mapped for ItemEntityRenderer.extractRenderState (run `match`, or map it in a --mappings file).
// horizon-mixins kind: RETURN
// Note: Horizon has no TAIL: RETURN runs at every exit, early ones too.
// Note: No Horizon function known for ItemEntityRenderer.extractRenderState: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   EntityRenderer.<init> -> (not mapped yet)
//   EntityRenderer.submit -> (not mapped yet)
//   ItemEntityRenderer.extractRenderState -> (not mapped yet)
//   ItemEntityRenderer.submit -> (not mapped yet)
//
// Decompiled Java, team/creative/itemphysic/mixin/ItemEntityRendererMixin.java:44:
// | @Inject(
// |    method = "extractRenderState(Lnet/minecraft/world/entity/item/ItemEntity;Lnet/minecraft/client/renderer/entity/state/ItemEntityRenderState;F)V",
// |    at = @At("TAIL"),
// |    require = 1
// | )
// | public void injectExtract(ItemEntity item, ItemEntityRenderState state, float partialTicks, CallbackInfo info) {
// |    ((ItemEntityRenderStateExtender)state).extractPhysic(item);
// | }
