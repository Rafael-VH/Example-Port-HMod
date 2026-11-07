// Not ported yet: the Java @Inject(HEAD) on LoadedBlockModels.lambda$bake$1 (Mixin_SpecialModelRenderers_SetCurrentSpecifiedModel.setEmf$Model).
// Why it's off: No Horizon function mapped for LoadedBlockModels.lambda$bake$1 (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for LoadedBlockModels.lambda$bake$1: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   LoadedBlockModels.bake -> (not mapped yet)
//   LoadedBlockModels.lambda$bake$1 -> (not mapped yet)
//   DefaultedRegistry.wrapAsHolder -> (not mapped yet)
//   Holder.unwrapKey -> (not mapped yet)
//   BuiltInRegistries.BLOCK -> content. Registration: carried as a content mod.
//   Identifier.getNamespace -> (not mapped yet)
//   Identifier.getPath -> (not mapped yet)
//   ResourceKey.identifier -> (not mapped yet)
//   Blocks.CHEST -> (not mapped yet)
//   Blocks.ENCHANTING_TABLE -> (not mapped yet)
//   Blocks.ENDER_CHEST -> (not mapped yet)
//   Blocks.LECTERN -> (not mapped yet)
//   Blocks.TRAPPED_CHEST -> (not mapped yet)
//   BlockState.getBlock -> (not mapped yet)
//   BlockState.is -> (not mapped yet)
//   BlockState.toString -> (not mapped yet)
//
// Decompiled Java, traben/entity_model_features/mixin/mixins/rendering/Mixin_SpecialModelRenderers_SetCurrentSpecifiedModel.java:46:
// | @Inject(method = "lambda$bake$1", at = @At("HEAD"))
// | private static void setEmf$Model(CallbackInfo ci, @Local(argsOnly = true) BlockState blockState, @Local(argsOnly = true) Unbaked unbakedModel) {
// |    if (unbakedModel instanceof SpecialModelWrapper) {
// |       setModel(blockState);
// |    }
// | }
