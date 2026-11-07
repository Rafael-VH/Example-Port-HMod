// Not ported yet: the Java @Inject(TAIL) on ClientPacketListener.handleLogin (ClientPacketListenerMixin.handleLogin).
// Why it's off: No Horizon function mapped for ClientPacketListener.handleLogin (run `match`, or map it in a --mappings file).
// horizon-mixins kind: RETURN
// Note: Horizon has no TAIL: RETURN runs at every exit, early ones too.
// Note: No Horizon function known for ClientPacketListener.handleLogin: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   ClientPacketListener.handleLogin -> (not mapped yet)
//
// Decompiled Java, com/sonicether/soundphysics/mixin/ClientPacketListenerMixin.java:18:
// | @Inject(method = "handleLogin", at = @At("TAIL"))
// | private void handleLogin(ClientboundLoginPacket clientboundLoginPacket, CallbackInfo ci) {
// |    LevelAccessUtils.onLoadLevel(this.level);
// | }
