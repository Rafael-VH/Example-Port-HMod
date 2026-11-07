// Not ported yet: the Java @WrapOperation(INVOKE) on PlayerList.placeNewPlayer (PlayerListMixin.bind).
// Why it's off: `horizon_client::lan::hub::Hub::place_new_player` has a closure in its body: horizon-mixins doesn't rewrite such bodies with WRAP_OPERATION. A HEAD hook on it, or a WRAP_OPERATION on a function it calls, may work instead.
// horizon-mixins kind: WRAP_OPERATION on `horizon_client::lan::hub::Hub::place_new_player`
// Note: The call it hooks (UnboundProtocol.bind) isn't mapped to a Rust function yet.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   UnboundProtocol.bind -> (not mapped yet)
//   ServerPlayer.registryAccess -> (not mapped yet)
//   ServerGamePacketListenerImpl.player -> (not mapped yet)
//   PlayerList.placeNewPlayer -> horizon_client::lan::hub::Hub::place_new_player (suggested, high)
//
// Decompiled Java, team/creative/creativecore/mixin/PlayerListMixin.java:17:
// | @WrapOperation(
// |    method = "Lnet/minecraft/server/players/PlayerList;placeNewPlayer(Lnet/minecraft/network/Connection;Lnet/minecraft/server/level/ServerPlayer;Lnet/minecraft/server/network/CommonListenerCookie;)V",
// |    at = @At(
// |       value = "INVOKE",
// |       target = "Lnet/minecraft/network/protocol/UnboundProtocol;bind(Ljava/util/function/Function;Ljava/lang/Object;)Lnet/minecraft/network/ProtocolInfo;"
// |    ),
// |    require = 1
// | )
// | private ProtocolInfo bind(UnboundProtocol unbound, Function function, Object context, Operation<ProtocolInfo> operation) {
// |    ServerCommonPacketListenerImplAccessor l = (ServerCommonPacketListenerImplAccessor)context;
// |    return unbound.bind(x -> new CreativeByteBuf((ByteBuf)x, ((ServerGamePacketListenerImpl)l).player.registryAccess(), l.getConnection()), context);
// | }
