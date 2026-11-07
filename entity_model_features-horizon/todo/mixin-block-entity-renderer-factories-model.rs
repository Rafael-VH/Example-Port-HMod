// Not ported yet: the Java @ModifyArg(INVOKE) on BlockEntityRenderers.createEntityRenderers (MixinBlockEntityRendererFactories.setEmf$Model).
// Why it's off: No Horizon function mapped for BlockEntityRenderers.createEntityRenderers (run `match`, or map it in a --mappings file).
// horizon-mixins kind: MODIFY_ARG
// Note: No Horizon function known for BlockEntityRenderers.createEntityRenderers: run `match`.
// Note: The call it hooks (Map.forEach) isn't mapped to a Rust function yet.
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
// Decompiled Java, traben/entity_model_features/mixin/mixins/MixinBlockEntityRendererFactories.java:43:
// | @ModifyArg(method = "createEntityRenderers", at = @At(value = "INVOKE", target = "Ljava/util/Map;forEach(Ljava/util/function/BiConsumer;)V"))
// | private static BiConsumer<BlockEntityType<?>, Object> setEmf$Model(BiConsumer<BlockEntityType<?>, Object> action) {
// |    return (type, idk) -> {
// |       if (!EMF.testForForgeLoadingError()) {
// |          EMFManager.getInstance().currentBlockEntityTypeLoading = type;
// |          if (UEntityTypes.ENCHANTING_TABLE.equals(type)) {
// |             EMFManager.getInstance().currentSpecifiedModelLoading.set("enchanting_book");
// |          } else if (UEntityTypes.LECTERN.equals(type)) {
// |             EMFManager.getInstance().currentSpecifiedModelLoading.set("lectern_book");
// |          } else if (UEntityTypes.CHEST.equals(type)) {
// |             EMFManager.getInstance().currentSpecifiedModelLoading.set("chest");
// |          } else if (UEntityTypes.ENDER_CHEST.equals(type)) {
// |             EMFManager.getInstance().currentSpecifiedModelLoading.set("ender_chest");
// |          } else if (UEntityTypes.TRAPPED_CHEST.equals(type)) {
// |             EMFManager.getInstance().currentSpecifiedModelLoading.set("trapped_chest");
// |          } else if (type.builtInRegistryHolder() != null && type.builtInRegistryHolder().unwrapKey().isPresent()) {
// |             Identifier id = ((ResourceKey)type.builtInRegistryHolder().unwrapKey().get()).identifier();
// |             if (id.getNamespace().equals("minecraft")) {
// |                EMFManager.getInstance().currentSpecifiedModelLoading.set(id.getPath());
// |             } else {
// |                EMFManager.getInstance().currentSpecifiedModelLoading.set(id.getNamespace() + ":" + id.getPath());
// |             }
// |          }
// |
// |          emf$renderers.add(EMFManager.getInstance().currentSpecifiedModelLoading.get());
// |          if (((EMFConfig)EMF.config().getConfig()).logModelCreationData) {
// |             EMFUtils.log("Seeing block entity renderer init for: " + EMFManager.getInstance().currentSpecifiedModelLoading.get());
// |          }
// |
// |          action.accept(type, idk);
// |       }
// |    };
// | }
