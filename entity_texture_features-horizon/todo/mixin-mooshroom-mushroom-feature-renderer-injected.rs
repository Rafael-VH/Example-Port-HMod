// Not ported yet: the Java @Inject(HEAD) on MushroomCowMushroomLayer.submit (MixinMooshroomMushroomFeatureRenderer.etf$injected).
// Why it's off: No Horizon function mapped for MushroomCowMushroomLayer.submit (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for MushroomCowMushroomLayer.submit: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   NativeImage.close -> (not mapped yet)
//   NativeImage.getHeight -> (not mapped yet)
//   NativeImage.getWidth -> (not mapped yet)
//   Minecraft.getInstance -> `self`. Hooks are methods on Game: self is what Minecraft.getInstance() gave.
//   Minecraft.getResourceManager -> (not mapped yet)
//   PartPose.ZERO -> (not mapped yet)
//   CubeDeformation.<init> -> (not mapped yet)
//   CubeListBuilder.addBox -> (not mapped yet)
//   CubeListBuilder.create -> (not mapped yet)
//   CubeListBuilder.texOffs -> (not mapped yet)
//   MeshDefinition.<init> -> (not mapped yet)
//   MeshDefinition.getRoot -> (not mapped yet)
//   PartDefinition.addOrReplaceChild -> (not mapped yet)
//   PartDefinition.bake -> (not mapped yet)
//   PartDefinition.getChild -> (not mapped yet)
//   MushroomCowMushroomLayer.submit -> (not mapped yet)
//   MushroomCowMushroomLayer.submitMushroomBlock -> (not mapped yet)
//   MushroomCowRenderState.variant -> (not mapped yet)
//   Identifier.toString -> (not mapped yet)
//   ResourceManager.getResource -> (not mapped yet)
//   Variant.BROWN -> (not mapped yet)
//   Variant.RED -> (not mapped yet)
//   Variant.equals -> (not mapped yet)
//
// Decompiled Java, traben/entity_texture_features/mixin/mixins/entity/renderer/feature/MixinMooshroomMushroomFeatureRenderer.java:197:
// | @Inject(
// |    method = "submit(Lcom/mojang/blaze3d/vertex/PoseStack;Lnet/minecraft/client/renderer/SubmitNodeCollector;ILnet/minecraft/client/renderer/entity/state/MushroomCowRenderState;FF)V",
// |    at = @At("HEAD")
// | )
// | private void etf$injected(CallbackInfo ci, @Local(argsOnly = true) MushroomCowRenderState state) {
// |    this.isRed = state.variant.equals(Variant.RED);
// |    this.isBrown = state.variant.equals(Variant.BROWN);
// | }
