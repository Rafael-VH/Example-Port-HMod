// Not ported yet: the Java @Inject(HEAD) on Bootstrap.run (FlatLevelGeneratorPresetsMixin.bcl_run).
// Why it's off: No Horizon function mapped for Bootstrap.run (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for Bootstrap.run: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   Bootstrap.run -> (not mapped yet)
//
// Decompiled Java, org/betterx/betternether/mixin/common/FlatLevelGeneratorPresetsMixin.java:30:
// | @Inject(method = "run", at = @At("HEAD"))
// | void bcl_run(CallbackInfo ci) {
// | }
