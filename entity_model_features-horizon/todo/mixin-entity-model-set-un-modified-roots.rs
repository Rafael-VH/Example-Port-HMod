// Not ported yet: the Java @Inject(RETURN) on EntityModelSet.vanilla (MixinEntityModelSet.emf$unModifiedRoots).
// Why it's off: No Horizon function mapped for EntityModelSet.vanilla (run `match`, or map it in a --mappings file).
// horizon-mixins kind: RETURN
// Note: Horizon's RETURN runs at every normal exit and passes the return value on.
// Note: No Horizon function known for EntityModelSet.vanilla: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   EntityModelSet.vanilla -> (not mapped yet)
//   LayerDefinitions.createRoots -> (not mapped yet)
//
// Decompiled Java, traben/entity_model_features/mixin/mixins/exporting/MixinEntityModelSet.java:22:
// | @Inject(method = "vanilla", at = @At("RETURN"))
// | private static void emf$unModifiedRoots(CallbackInfoReturnable<EntityModelSet> cir) {
// |    EMF.tempDisableModelModifications = true;
// |    ((IEMFUnmodifiedLayerRootGetter)cir.getReturnValue()).emf$setUnmodifiedRoots(ImmutableMap.copyOf(LayerDefinitions.createRoots()));
// |    EMF.tempDisableModelModifications = false;
// | }
