// Not ported yet: the Java @Inject(RETURN) on Properties.ofLegacyCopy (MBlockSettings.onCopy).
// Why it's off: No Horizon function mapped for Properties.ofLegacyCopy (run `match`, or map it in a --mappings file).
// horizon-mixins kind: RETURN
// Note: Horizon's RETURN runs at every normal exit and passes the return value on.
// Note: No Horizon function known for Properties.ofLegacyCopy: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   Properties.ofLegacyCopy -> (not mapped yet)
//
// Decompiled Java, eu/ha3/presencefootsteps/mixins/MBlockSettings.java:29:
// | @Inject(method = "ofLegacyCopy", at = @At("RETURN"))
// | private static void onCopy(BlockBehaviour block, CallbackInfoReturnable<Properties> info) {
// |    if (block instanceof Block b) {
// |       ((DerivedBlock.Settings)info.getReturnValue()).setBaseBlock(b);
// |    }
// | }
