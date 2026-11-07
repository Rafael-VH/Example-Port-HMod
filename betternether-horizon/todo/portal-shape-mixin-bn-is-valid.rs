// Not ported yet: the Java @Inject(HEAD) on PortalShape.isValid (PortalShapeMixin.bn_isValid).
// Why it's off: No Horizon function mapped for PortalShape.isValid (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for PortalShape.isValid: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   PortalShape.createPortalBlocks -> horizon_world::level::portal::Sim<'_, L>::create_portal_blocks (suggested, high)
//   PortalShape.findAnyShape -> (not mapped yet)
//   PortalShape.isComplete -> (not mapped yet)
//   PortalShape.isValid -> (not mapped yet)
//
// Decompiled Java, org/betterx/betternether/mixin/common/PortalShapeMixin.java:40:
// | @Inject(method = "isValid", at = @At("HEAD"), cancellable = true)
// | private void bn_isValid(CallbackInfoReturnable<Boolean> cir) {
// |    if (this.bn_shape != null) {
// |       cir.setReturnValue(this.bn_shape.isValid());
// |    }
// | }
