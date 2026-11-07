// Not ported yet: the Java @Inject(HEAD) on DefaultAttributes.getSupplier (DefaultAttributeRegistryMixin.getAttribute).
// Why it's off: No Horizon function mapped for DefaultAttributes.getSupplier (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for DefaultAttributes.getSupplier: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   DefaultAttributes.getSupplier -> (not mapped yet)
//   DefaultAttributes.hasSupplier -> (not mapped yet)
//
// Decompiled Java, org/betterx/betternether/mixin/common/DefaultAttributeRegistryMixin.java:15:
// | @Inject(method = "getSupplier", at = @At("HEAD"), cancellable = true)
// | private static void getAttribute(EntityType<? extends LivingEntity> type, CallbackInfoReturnable<AttributeSupplier> info) {
// |    AttributeSupplier container = NetherEntities.ATTRIBUTES.get(type);
// |    if (container != null) {
// |       info.setReturnValue(container);
// |       info.cancel();
// |    }
// | }
