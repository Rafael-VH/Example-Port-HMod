// Not ported yet: the Java @Inject(HEAD) on DefaultAttributes.hasSupplier (DefaultAttributeRegistryMixin.hasDefinition).
// Why it's off: No Horizon function mapped for DefaultAttributes.hasSupplier (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for DefaultAttributes.hasSupplier: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   DefaultAttributes.getSupplier -> (not mapped yet)
//   DefaultAttributes.hasSupplier -> (not mapped yet)
//
// Decompiled Java, org/betterx/betternether/mixin/common/DefaultAttributeRegistryMixin.java:24:
// | @Inject(method = "hasSupplier", at = @At("HEAD"), cancellable = true)
// | private static void hasDefinition(EntityType<?> type, CallbackInfoReturnable<Boolean> info) {
// |    if (NetherEntities.ATTRIBUTES.containsKey(type)) {
// |       info.setReturnValue(true);
// |       info.cancel();
// |    }
// | }
