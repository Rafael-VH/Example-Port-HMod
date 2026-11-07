// Not ported yet: the Java @ModifyVariable(HEAD) on ModelPart.render (MixinModelPart.etf$modify).
// Why it's off: No Horizon function mapped for ModelPart.render (run `match`, or map it in a --mappings file).
// horizon-mixins kind: MODIFY_VARIABLE
// Note: Selects a Rust local by name: pick it in the target function.
// Note: No Horizon function known for ModelPart.render: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   BufferBuilder.building -> (not mapped yet)
//   ModelPart.render -> (not mapped yet)
//
// Decompiled Java, traben/entity_texture_features/mixin/mixins/MixinModelPart.java:52:
// | @ModifyVariable(
// |    method = "render(Lcom/mojang/blaze3d/vertex/PoseStack;Lcom/mojang/blaze3d/vertex/VertexConsumer;III)V",
// |    at = @At("HEAD"),
// |    ordinal = 0,
// |    argsOnly = true
// | )
// | private VertexConsumer etf$modify(VertexConsumer value) {
// |    return value instanceof BufferBuilder builder
// |          && !builder.building
// |          && value instanceof ETFVertexConsumer etf
// |          && etf.etf$getRenderLayer() != null
// |          && etf.etf$getProvider() != null
// |       ? etf.etf$getProvider().getBuffer(etf.etf$getRenderLayer())
// |       : value;
// | }
