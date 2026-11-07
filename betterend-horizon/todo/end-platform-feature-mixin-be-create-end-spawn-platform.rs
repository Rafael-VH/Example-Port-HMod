// Not ported yet: the Java @Inject(HEAD) on EndPlatformFeature.createEndPlatform (EndPlatformFeatureMixin.be_createEndSpawnPlatform).
// Why it's off: No Horizon function mapped for EndPlatformFeature.createEndPlatform (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for EndPlatformFeature.createEndPlatform: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   EndPlatformFeature.createEndPlatform -> (not mapped yet)
//
// Decompiled Java, org/betterx/betterend/mixin/common/EndPlatformFeatureMixin.java:14:
// | @Inject(method = "createEndPlatform", at = @At("HEAD"), cancellable = true)
// | private static void be_createEndSpawnPlatform(ServerLevelAccessor serverLevelAccessor, BlockPos blockPos, boolean bl, CallbackInfo info) {
// |    TerrainGenerator.makeObsidianPlatform(serverLevelAccessor, info);
// | }
