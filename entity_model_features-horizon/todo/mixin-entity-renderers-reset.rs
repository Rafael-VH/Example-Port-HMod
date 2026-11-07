// Not ported yet: the Java @Inject(TAIL) on EntityRenderers.lambda$createEntityRenderers$0 (MixinEntityRenderers.emf$reset).
// Why it's off: No Horizon function mapped for EntityRenderers.lambda$createEntityRenderers$0 (run `match`, or map it in a --mappings file).
// horizon-mixins kind: RETURN
// Note: Horizon has no TAIL: RETURN runs at every exit, early ones too.
// Note: No Horizon function known for EntityRenderers.lambda$createEntityRenderers$0: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   EntityRenderers.lambda$createEntityRenderers$0 -> (not mapped yet)
//   Reference.is -> (not mapped yet)
//   EntityTypeTags.BOAT -> (not mapped yet)
//   EntityType.builtInRegistryHolder -> (not mapped yet)
//   EntityType.getDescriptionId -> (not mapped yet)
//
// Decompiled Java, traben/entity_model_features/mixin/mixins/rendering/MixinEntityRenderers.java:37:
// | @Inject(method = "lambda$createEntityRenderers$0", at = @At("TAIL"))
// | private static void emf$reset(
// |    Builder<?, ?> builder, Context context, EntityType<?> entityType, EntityRendererProvider<?> entityRendererProvider, CallbackInfo ci
// | ) {
// |    EMFManager.getInstance().currentSpecifiedModelLoading.set("");
// | }
