// Not ported yet: the Java @Inject(HEAD) on ServerPlayer.findRespawnPositionAndUseSpawnBlock (PlayerEntityMixin.bn_statueRespawn).
// Why it's off: No Horizon function mapped for ServerPlayer.findRespawnPositionAndUseSpawnBlock (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for ServerPlayer.findRespawnPositionAndUseSpawnBlock: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   BlockPos.below -> (not mapped yet)
//   BlockPos.relative -> (not mapped yet)
//   MinecraftServer.getLevel -> (not mapped yet)
//   ServerLevel.getBlockState -> (not mapped yet)
//   ServerLevel.getServer -> (not mapped yet)
//   ServerPlayer.findRespawnPositionAndUseSpawnBlock -> (not mapped yet)
//   ServerPlayer.getRespawnConfig -> (not mapped yet)
//   ServerPlayer.level -> (not mapped yet)
//   RespawnConfig.respawnData -> (not mapped yet)
//   BlockState.getBlock -> (not mapped yet)
//   BlockState.getCollisionShape -> horizon_client_render::collisions::<SceneCollisions<'_, '_> as PistonCollisionWorld>::collision_shape (suggested, high)
//   BlockState.getValue -> (not mapped yet)
//   BlockState.isSolid -> (not mapped yet)
//   TeleportTransition.<init> -> (not mapped yet)
//   RespawnData.dimension -> (not mapped yet)
//   RespawnData.pos -> (not mapped yet)
//   RespawnData.yaw -> (not mapped yet)
//   Vec3.ZERO -> (not mapped yet)
//   Vec3.add -> (not mapped yet)
//   Vec3.atLowerCornerOf -> (not mapped yet)
//   VoxelShape.isEmpty -> (not mapped yet)
//
// Decompiled Java, org/betterx/betternether/mixin/common/PlayerEntityMixin.java:23:
// | @Inject(method = "findRespawnPositionAndUseSpawnBlock", at = @At("HEAD"), cancellable = true)
// | private void bn_statueRespawn(boolean useSpawnBlock, PostTeleportTransition postTeleportTransition, CallbackInfoReturnable<TeleportTransition> info) {
// |    ServerPlayer self = (ServerPlayer)this;
// |    RespawnConfig respawnConfig = self.getRespawnConfig();
// |    if (respawnConfig != null) {
// |       MinecraftServer server = self.level().getServer();
// |       if (server != null) {
// |          ServerLevel level = server.getLevel(respawnConfig.respawnData().dimension());
// |          if (level != null) {
// |             BlockPos pos = respawnConfig.respawnData().pos();
// |             BlockState blockState = level.getBlockState(pos);
// |             if (blockState.getBlock() instanceof BlockStatueRespawner) {
// |                Optional<Vec3> op = bn_findRespawnPosition(level, pos, blockState);
// |                if (!op.isEmpty()) {
// |                   info.setReturnValue(new TeleportTransition(level, op.get(), Vec3.ZERO, respawnConfig.respawnData().yaw(), 0.0F, postTeleportTransition));
// |                }
// |             }
// |          }
// |       }
// |    }
// | }
