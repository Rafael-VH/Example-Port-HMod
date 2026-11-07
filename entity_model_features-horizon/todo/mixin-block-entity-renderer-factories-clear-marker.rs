// Not ported yet: the Java @Inject(RETURN) on BlockEntityRenderers.createEntityRenderers (MixinBlockEntityRendererFactories.emf$clearMarker).
// Why it's off: No Horizon function mapped for BlockEntityRenderers.createEntityRenderers (run `match`, or map it in a --mappings file).
// horizon-mixins kind: RETURN
// Note: Horizon's RETURN runs at every normal exit and passes the return value on.
// Note: No Horizon function known for BlockEntityRenderers.createEntityRenderers: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   Map.forEach -> (not mapped yet)
//   BlockEntityRenderers.createEntityRenderers -> (not mapped yet)
//   Reference.unwrapKey -> (not mapped yet)
//   Identifier.getNamespace -> (not mapped yet)
//   Identifier.getPath -> (not mapped yet)
//   ResourceKey.identifier -> (not mapped yet)
//   BlockEntityType.builtInRegistryHolder -> (not mapped yet)
//
// Decompiled Java, traben/entity_model_features/mixin/mixins/MixinBlockEntityRendererFactories.java:30:
// | @Inject(method = "createEntityRenderers", at = @At("RETURN"))
// | private static void emf$clearMarker(Context args, CallbackInfoReturnable<Map<BlockEntityType<?>, BlockEntityRenderer>> cir) {
// |    if (!EMF.testForForgeLoadingError()) {
// |       EMFManager.getInstance().currentSpecifiedModelLoading.set("");
// |       EMFManager.getInstance().currentBlockEntityTypeLoading = null;
// |       if (((EMFConfig)EMF.config().getConfig()).logModelCreationData || ((EMFConfig)EMF.config().getConfig()).automaticModelExporting) {
// |          EMFUtils.log("Identified block entity renderers: " + emf$renderers);
// |       }
// |
// |       emf$renderers.clear();
// |    }
// | }
