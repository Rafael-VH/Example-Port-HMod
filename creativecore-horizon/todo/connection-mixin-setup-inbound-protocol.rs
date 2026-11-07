// Not ported yet: the Java @Inject(INVOKE) on Connection.setupInboundProtocol (ConnectionMixin.setupInboundProtocol).
// Why it's off: No Horizon function mapped for Connection.setupInboundProtocol (run `match`, or map it in a --mappings file).
// horizon-mixins kind: BEFORE_CALL
// Note: Only whole statements, plain let initializers and block tails can be selected.
// Note: No Horizon function known for Connection.setupInboundProtocol: run `match`.
// Note: The call it hooks (UnconfiguredPipelineHandler.setupInboundProtocol) isn't mapped to a Rust function yet.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   Connection.setupInboundProtocol -> (not mapped yet)
//   UnconfiguredPipelineHandler.setupInboundProtocol -> (not mapped yet)
//
// Decompiled Java, team/creative/creativecore/mixin/ConnectionMixin.java:17:
// | @Inject(
// |    method = "setupInboundProtocol(Lnet/minecraft/network/ProtocolInfo;Lnet/minecraft/network/PacketListener;)V",
// |    at = @At(
// |       value = "INVOKE",
// |       target = "Lnet/minecraft/network/UnconfiguredPipelineHandler;setupInboundProtocol(Lnet/minecraft/network/ProtocolInfo;)Lnet/minecraft/network/UnconfiguredPipelineHandler$InboundConfigurationTask;"
// |    )
// | )
// | public <T extends PacketListener> void setupInboundProtocol(ProtocolInfo<T> protocolInfo, T packetListener, CallbackInfo info) {
// |    this.protocolInfo = protocolInfo;
// | }
