// Not ported yet: the Java @Redirect(INVOKE) on ClientConfigurationPacketListenerImpl.handleConfigurationFinished (ClientConfigurationPacketListenerImplMixin.handleConfigurationFinished).
// Why it's off: No Horizon function mapped for ClientConfigurationPacketListenerImpl.handleConfigurationFinished (run `match`, or map it in a --mappings file).
// horizon-mixins kind: WRAP_OPERATION
// Note: A redirect is a WRAP_OPERATION that doesn't call `original`.
// Note: No Horizon function known for ClientConfigurationPacketListenerImpl.handleConfigurationFinished: run `match`.
// Note: The call it hooks (RegistryFriendlyByteBuf.decorator) isn't mapped to a Rust function yet.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   ClientCommonPacketListenerImpl.<init> -> (not mapped yet)
//   ClientConfigurationPacketListenerImpl.handleConfigurationFinished -> (not mapped yet)
//   RegistryFriendlyByteBuf.decorator -> (not mapped yet)
//
// Decompiled Java, team/creative/creativecore/mixin/ClientConfigurationPacketListenerImplMixin.java:23:
// | @Redirect(
// |    method = "handleConfigurationFinished(Lnet/minecraft/network/protocol/configuration/ClientboundFinishConfigurationPacket;)V",
// |    at = @At(
// |       value = "INVOKE",
// |       target = "Lnet/minecraft/network/RegistryFriendlyByteBuf;decorator(Lnet/minecraft/core/RegistryAccess;)Ljava/util/function/Function;"
// |    ),
// |    require = 2
// | )
// | private Function<ByteBuf, RegistryFriendlyByteBuf> handleConfigurationFinished(RegistryAccess access) {
// |    return x -> new CreativeByteBuf(x, access, this.connection);
// | }
