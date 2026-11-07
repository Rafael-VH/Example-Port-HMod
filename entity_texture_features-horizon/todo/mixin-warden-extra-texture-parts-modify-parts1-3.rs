// Not ported yet: the Java @WrapOperation(INVOKE) on WardenModel.createBioluminescentLayer (MixinWardenExtraTextureParts.etf$modifyParts1).
// Why it's off: No Horizon function mapped for WardenModel.createBioluminescentLayer (run `match`, or map it in a --mappings file).
// horizon-mixins kind: WRAP_OPERATION
// Note: No Horizon function known for WardenModel.createBioluminescentLayer: run `match`.
// Note: The call it hooks (LayerDefinition.apply) isn't mapped to a Rust function yet.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   EntityModel.<init> -> (not mapped yet)
//   LayerDefinition.apply -> (not mapped yet)
//   WardenModel.createBioluminescentLayer -> (not mapped yet)
//   WardenModel.createHeartLayer -> (not mapped yet)
//   WardenModel.createPulsatingSpotsLayer -> (not mapped yet)
//   WardenModel.createTendrilsLayer -> (not mapped yet)
//
// Decompiled Java, traben/entity_texture_features/mixin/mixins/entity/renderer/feature/MixinWardenExtraTextureParts.java:20:
// | @WrapOperation(
// |    method = {"createTendrilsLayer", "createHeartLayer", "createBioluminescentLayer", "createPulsatingSpotsLayer"},
// |    at = @At(
// |       value = "INVOKE",
// |       target = "Lnet/minecraft/client/model/geom/builders/LayerDefinition;apply(Lnet/minecraft/client/model/geom/builders/MeshTransformer;)Lnet/minecraft/client/model/geom/builders/LayerDefinition;"
// |    )
// | )
// | private static LayerDefinition etf$modifyParts1(LayerDefinition instance, MeshTransformer meshTransformer, Operation<LayerDefinition> original) {
// |    return ETF.config().getConfig().enableFullBodyWardenTextures ? instance : (LayerDefinition)original.call(new Object[]{instance, meshTransformer});
// | }
