// Not ported yet: the Java @Inject(RETURN) on ModelPart.render (MixinModelPart.etf$doEmissiveIfInitialPart).
// Why it's off: No Horizon function mapped for ModelPart.render (run `match`, or map it in a --mappings file).
// horizon-mixins kind: RETURN
// Note: Horizon's RETURN runs at every normal exit and passes the return value on.
// Note: No Horizon function known for ModelPart.render: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   BufferBuilder.building -> (not mapped yet)
//   ModelPart.render -> (not mapped yet)
//
// Decompiled Java, traben/entity_texture_features/mixin/mixins/MixinModelPart.java:30:
// | @Inject(method = "render(Lcom/mojang/blaze3d/vertex/PoseStack;Lcom/mojang/blaze3d/vertex/VertexConsumer;III)V", at = @At("RETURN"))
// | private void etf$doEmissiveIfInitialPart(PoseStack matrices, VertexConsumer vertices, int light, int overlay, int k, CallbackInfo ci) {
// |    if (ETFState.currentModelPartDepth > 1) {
// |       ETFState.currentModelPartDepth--;
// |    } else {
// |       if (ETFState.isStateActive() && vertices instanceof ETFVertexConsumer etfVertexConsumer) {
// |          ETFTexture texture = etfVertexConsumer.etf$getETFTexture();
// |          if (texture != null && (texture.isEmissive() || texture.isEnchanted())) {
// |             URenderTypeToVertexConsumer provider = etfVertexConsumer.etf$getProvider();
// |             RenderType layer = etfVertexConsumer.etf$getRenderLayer();
// |             if (provider != null && layer != null) {
// |                ETFUtils2.RenderMethodForOverlay renderer = (a, b) -> this.render(matrices, a, b, overlay, k);
// |                if (ETFUtils2.renderEmissive(texture, provider, renderer) | ETFUtils2.renderEnchanted(texture, provider, light, renderer)) {
// |                }
// |             }
// |          }
// |       }
// |
// |       ETFState.currentModelPartDepth = 0;
// |    }
// | }
