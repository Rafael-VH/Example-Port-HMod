// Not ported yet: the Java @Inject(HEAD) on MappedRegistry.freeze (MappedRegistryMixin.bn_debug).
// Why it's off: No Horizon function mapped for MappedRegistry.freeze (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for MappedRegistry.freeze: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   MappedRegistry.freeze -> (not mapped yet)
//
// Decompiled Java, org/betterx/betternether/mixin/common/MappedRegistryMixin.java:13:
// | @Inject(method = "freeze", at = @At("HEAD"))
// | void bn_debug(CallbackInfoReturnable<Registry<?>> cir) {
// |    if (this.toString().contains("worlds_together:worldgen/betterx/surface_rules")) {
// |       BetterNether.C.log.info("Freeze " + this.toString());
// |    }
// | }
