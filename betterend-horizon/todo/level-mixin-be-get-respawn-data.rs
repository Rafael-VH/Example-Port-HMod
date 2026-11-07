// Not ported yet: the Java @Inject(HEAD) on ServerLevel.getRespawnData (LevelMixin.be_getRespawnData).
// Why it's off: No Horizon function mapped for ServerLevel.getRespawnData (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for ServerLevel.getRespawnData: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   ServerLevel.dimension -> (not mapped yet)
//   ServerLevel.getRespawnData -> (not mapped yet)
//   Level.END -> (not mapped yet)
//   RespawnData.of -> (not mapped yet)
//
// Decompiled Java, org/betterx/betterend/mixin/common/LevelMixin.java:15:
// | @Inject(method = "getRespawnData", at = @At("HEAD"), cancellable = true)
// | private void be_getRespawnData(CallbackInfoReturnable<RespawnData> info) {
// |    ServerLevel self = (ServerLevel)this;
// |    if (GeneratorOptions.changeSpawn() && self.dimension() == Level.END) {
// |       BlockPos pos = GeneratorOptions.getSpawn();
// |       info.setReturnValue(RespawnData.of(self.dimension(), pos, 0.0F, 0.0F));
// |    }
// | }
