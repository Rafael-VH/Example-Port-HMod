// Not ported yet: the Java @Inject(HEAD) on ServerPlayer.findRespawnPositionAndUseSpawnBlock (PlayerMixin.be_findRespawnAndUseSpawnBlock).
// Why it's off: No Horizon function mapped for ServerPlayer.findRespawnPositionAndUseSpawnBlock (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for ServerPlayer.findRespawnPositionAndUseSpawnBlock: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   BlockPos.above -> (not mapped yet)
//   BlockPos.below -> (not mapped yet)
//   BlockPos.relative -> (not mapped yet)
//   MinecraftServer.getLevel -> (not mapped yet)
//   ServerLevel.getBlockState -> (not mapped yet)
//   ServerLevel.getRandom -> (not mapped yet)
//   ServerLevel.getServer -> (not mapped yet)
//   ServerPlayer.findRespawnPositionAndUseSpawnBlock -> (not mapped yet)
//   ServerPlayer.getRespawnConfig -> (not mapped yet)
//   ServerPlayer.level -> (not mapped yet)
//   RespawnConfig.respawnData -> (not mapped yet)
//   EntityTypes.PLAYER -> (not mapped yet)
//   DismountHelper.findSafeDismountLocation -> (not mapped yet)
//   BlockState.getValue -> (not mapped yet)
//   BlockState.is -> (not mapped yet)
//   TeleportTransition.<init> -> (not mapped yet)
//   RespawnData.dimension -> (not mapped yet)
//   RespawnData.pos -> (not mapped yet)
//   RespawnData.yaw -> (not mapped yet)
//   Vec3.ZERO -> (not mapped yet)
//
// Decompiled Java, org/betterx/betterend/mixin/common/PlayerMixin.java:30:
// | @Inject(method = "findRespawnPositionAndUseSpawnBlock", at = @At("HEAD"), cancellable = true)
// | private void be_findRespawnAndUseSpawnBlock(
// |    boolean useSpawnBlock, PostTeleportTransition postTeleportTransition, CallbackInfoReturnable<TeleportTransition> info
// | ) {
// |    ServerPlayer self = (ServerPlayer)this;
// |    RespawnConfig config = self.getRespawnConfig();
// |    if (config != null) {
// |       MinecraftServer server = self.level().getServer();
// |       if (server != null) {
// |          ServerLevel world = server.getLevel(config.respawnData().dimension());
// |          if (world != null) {
// |             BlockPos pos = config.respawnData().pos();
// |             BlockState blockState = world.getBlockState(pos);
// |             if (blockState.is(EndFunctionalBlocks.RESPAWN_OBELISK.block())) {
// |                Optional<Vec3> op = be_obeliskRespawnPosition(world, pos, blockState);
// |                if (op.isEmpty()) {
// |                   return;
// |                }
// |
// |                info.setReturnValue(new TeleportTransition(world, op.get(), Vec3.ZERO, config.respawnData().yaw(), 0.0F, postTeleportTransition));
// |             }
// |          }
// |       }
// |    }
// | }
