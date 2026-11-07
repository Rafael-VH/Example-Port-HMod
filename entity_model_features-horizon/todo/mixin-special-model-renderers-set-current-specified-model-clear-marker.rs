// Not ported yet: the Java @Inject(RETURN) on LoadedBlockModels.bake (Mixin_SpecialModelRenderers_SetCurrentSpecifiedModel.emf$clearMarker).
// Why it's off: No Horizon function mapped for LoadedBlockModels.bake (run `match`, or map it in a --mappings file).
// horizon-mixins kind: RETURN
// Note: Horizon's RETURN runs at every normal exit and passes the return value on.
// Note: No Horizon function known for LoadedBlockModels.bake: run `match`.
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
// Decompiled Java, traben/entity_model_features/mixin/mixins/rendering/Mixin_SpecialModelRenderers_SetCurrentSpecifiedModel.java:33:
// | @Inject(method = "bake", at = @At("RETURN"))
// | private static void emf$clearMarker(CallbackInfoReturnable<Map<Block, SpecialModelRenderer<?>>> cir) {
// |    if (!EMF.testForForgeLoadingError()) {
// |       EMFManager.getInstance().currentSpecifiedModelLoading.set("");
// |       EMFManager.getInstance().currentBlockEntityTypeLoading = null;
// |       if (((EMFConfig)EMF.config().getConfig()).logModelCreationData || ((EMFConfig)EMF.config().getConfig()).automaticModelExporting) {
// |          EMFUtils.log("Identified SPECIAL block entity renderers: " + emf$renderers);
// |       }
// |
// |       emf$renderers.clear();
// |    }
// | }
