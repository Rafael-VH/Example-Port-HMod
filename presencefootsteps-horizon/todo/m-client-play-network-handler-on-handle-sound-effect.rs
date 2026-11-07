// Not ported yet: the Java @Inject(INVOKE) on ClientPacketListener.handleSoundEvent (MClientPlayNetworkHandler.onHandleSoundEffect).
// Why it's off: No Horizon function mapped for ClientPacketListener.handleSoundEvent (run `match`, or map it in a --mappings file).
// horizon-mixins kind: BEFORE_CALL
// Note: Only whole statements, plain let initializers and block tails can be selected.
// Note: No Horizon function known for ClientPacketListener.handleSoundEvent: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   ClientPacketListener.handleSoundEvent -> (not mapped yet)
//
// Decompiled Java, eu/ha3/presencefootsteps/mixins/MClientPlayNetworkHandler.java:14:
// | @Inject(
// |    method = "handleSoundEvent(Lnet/minecraft/network/protocol/game/ClientboundSoundPacket;)V",
// |    at = @At(
// |       value = "INVOKE",
// |       target = "net/minecraft/client/multiplayer/ClientLevel.playSeededSound(Lnet/minecraft/world/entity/Entity;DDDLnet/minecraft/core/Holder;Lnet/minecraft/sounds/SoundSource;FFJ)V",
// |       shift = Shift.BEFORE
// |    ),
// |    cancellable = true
// | )
// | public void onHandleSoundEffect(ClientboundSoundPacket packet, CallbackInfo info) {
// |    if (PresenceFootsteps.getInstance().getEngine().onSoundRecieved(packet)) {
// |       info.cancel();
// |    }
// | }
