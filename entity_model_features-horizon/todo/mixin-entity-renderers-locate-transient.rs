// Not ported yet: the Java @Inject(HEAD) on EntityRenderers.lambda$createEntityRenderers$0 (MixinEntityRenderers.emf$locateTransient).
// Why it's off: No Horizon function mapped for EntityRenderers.lambda$createEntityRenderers$0 (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for EntityRenderers.lambda$createEntityRenderers$0: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   EntityRenderers.lambda$createEntityRenderers$0 -> (not mapped yet)
//   Reference.is -> (not mapped yet)
//   EntityTypeTags.BOAT -> (not mapped yet)
//   EntityType.builtInRegistryHolder -> (not mapped yet)
//   EntityType.getDescriptionId -> (not mapped yet)
//
// Decompiled Java, traben/entity_model_features/mixin/mixins/rendering/MixinEntityRenderers.java:20:
// | @Inject(method = "lambda$createEntityRenderers$0", at = @At("HEAD"))
// | private static void emf$locateTransient(
// |    Builder<?, ?> builder, Context context, EntityType<?> entityType, EntityRendererProvider<?> entityRendererProvider, CallbackInfo ci
// | ) {
// |    if (entityType.equals(UEntityTypes.SPECTRAL_ARROW)) {
// |       EMFManager.getInstance().currentSpecifiedModelLoading.set("spectral_arrow");
// |    } else if (entityType.equals(UEntityTypes.BREEZE_WIND_CHARGE)) {
// |       EMFManager.getInstance().currentSpecifiedModelLoading.set("breeze_wind_charge");
// |    } else if (entityType.builtInRegistryHolder().is(EntityTypeTags.BOAT)) {
// |       EMFManager.getInstance()
// |          .currentSpecifiedModelLoading
// |          .set("emf$boat$" + entityType.getDescriptionId().replaceAll("entity.minecraft.", "").replaceAll("(_boat|_raft|_chest_boat)$", ""));
// |    } else if (entityType.equals(UEntityTypes.CAMEL_HUSK)) {
// |       EMFManager.getInstance().currentSpecifiedModelLoading.set("camel_husk");
// |    }
// | }
