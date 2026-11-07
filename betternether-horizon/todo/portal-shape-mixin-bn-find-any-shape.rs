// Not ported yet: the Java @Inject(RETURN) on PortalShape.findAnyShape (PortalShapeMixin.bn_findAnyShape).
// Why it's off: No Horizon function mapped for PortalShape.findAnyShape (run `match`, or map it in a --mappings file).
// horizon-mixins kind: RETURN
// Note: Horizon's RETURN runs at every normal exit and passes the return value on.
// Note: No Horizon function known for PortalShape.findAnyShape: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   PortalShape.createPortalBlocks -> horizon_world::level::portal::Sim<'_, L>::create_portal_blocks (suggested, high)
//   PortalShape.findAnyShape -> (not mapped yet)
//   PortalShape.isComplete -> (not mapped yet)
//   PortalShape.isValid -> (not mapped yet)
//
// Decompiled Java, org/betterx/betternether/mixin/common/PortalShapeMixin.java:32:
// | @Inject(method = "findAnyShape", at = @At("RETURN"))
// | private static void bn_findAnyShape(BlockGetter blockGetter, BlockPos blockPos, Axis axis, CallbackInfoReturnable<PortalShape> cir) {
// |    PortalShape shape = (PortalShape)cir.getReturnValue();
// |    if (shape != null) {
// |       ((BNPortalShapeProvider)shape).bn_setShape(new BNPortalShape(blockGetter, blockPos, axis));
// |    }
// | }
