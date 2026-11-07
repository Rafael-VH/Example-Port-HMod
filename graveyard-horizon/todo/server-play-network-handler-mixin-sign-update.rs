// Not ported yet: the Java @Inject(HEAD) on ServerGamePacketListenerImpl.updateSignText (ServerPlayNetworkHandlerMixin.signUpdate).
// Why it's off: No Horizon function mapped for ServerGamePacketListenerImpl.updateSignText (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for ServerGamePacketListenerImpl.updateSignText: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   ServerboundSignUpdatePacket.pos -> (not mapped yet)
//   ServerLevel.getBlockEntity -> (not mapped yet)
//   ServerLevel.hasChunkAt -> horizon_client::raid::has_chunk_at (suggested, high)
//   ServerPlayer.level -> (not mapped yet)
//   ServerPlayer.resetLastActionTime -> (not mapped yet)
//   ServerGamePacketListenerImpl.updateSignText -> (not mapped yet)
//
// Decompiled Java, com/lion/graveyard/mixin/ServerPlayNetworkHandlerMixin.java:22:
// | @Inject(method = "updateSignText(Lnet/minecraft/network/protocol/game/ServerboundSignUpdatePacket;Ljava/util/List;)V", at = @At("HEAD"))
// | private void signUpdate(ServerboundSignUpdatePacket packet, List<FilteredText> signText, CallbackInfo info) {
// |    this.player.resetLastActionTime();
// |    ServerLevel serverWorld = this.player.level();
// |    BlockPos blockPos = packet.pos();
// |    if (serverWorld.hasChunkAt(blockPos)) {
// |       if (!(serverWorld.getBlockEntity(blockPos) instanceof GravestoneBlockEntity signBlockEntity)) {
// |          return;
// |       }
// |
// |       signBlockEntity.tryChangeText(this.player, signText);
// |    }
// | }
