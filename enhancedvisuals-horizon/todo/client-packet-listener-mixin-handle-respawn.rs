// Not ported yet: the Java @Inject(TAIL) on ClientPacketListener.handleRespawn (ClientPacketListenerMixin.handleRespawn).
// Why it's off: No Horizon function mapped for ClientPacketListener.handleRespawn (run `match`, or map it in a --mappings file).
// horizon-mixins kind: RETURN
// Note: Horizon has no TAIL: RETURN runs at every exit, early ones too.
// Note: No Horizon function known for ClientPacketListener.handleRespawn: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   ClientPacketListener.handleRespawn -> (not mapped yet)
//
// Decompiled Java, team/creative/enhancedvisuals/mixin/ClientPacketListenerMixin.java:14:
// | @Inject(method = "handleRespawn(Lnet/minecraft/network/protocol/game/ClientboundRespawnPacket;)V", at = @At("TAIL"))
// | public void handleRespawn(ClientboundRespawnPacket packet, CallbackInfo info) {
// |    ((EVManagerClient)EnhancedVisuals.MANAGER).respawn();
// | }
