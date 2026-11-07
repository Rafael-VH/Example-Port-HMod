// Not ported yet: the Java @Inject(HEAD) on ClientPacketListener.handleOpenSignEditor (ClientPlayNetworkHandlerMixin.openSignEditor).
// Why it's off: No Horizon function mapped for ClientPacketListener.handleOpenSignEditor (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for ClientPacketListener.handleOpenSignEditor: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   Minecraft.getInstance -> `self`. Hooks are methods on Game: self is what Minecraft.getInstance() gave.
//   Minecraft.gui -> (not mapped yet)
//   Minecraft.packetProcessor -> (not mapped yet)
//   Gui.setScreen -> (not mapped yet)
//   ClientLevel.getBlockEntity -> (not mapped yet)
//   ClientLevel.getBlockState -> (not mapped yet)
//   ClientPacketListener.handleOpenSignEditor -> (not mapped yet)
//   PacketUtils.ensureRunningOnSameThread -> (not mapped yet)
//   ClientboundOpenSignEditorPacket.pos -> (not mapped yet)
//
// Decompiled Java, com/lion/graveyard/mixin/ClientPlayNetworkHandlerMixin.java:25:
// | @Inject(method = "handleOpenSignEditor", at = @At("HEAD"), cancellable = true)
// | public void openSignEditor(ClientboundOpenSignEditorPacket packet, CallbackInfo info) {
// |    PacketUtils.ensureRunningOnSameThread(packet, (ClientGamePacketListener)this, Minecraft.getInstance().packetProcessor());
// |    BlockPos blockPos = packet.pos();
// |    BlockEntity foundEntity = this.level.getBlockEntity(blockPos);
// |    if (foundEntity instanceof GravestoneBlockEntity) {
// |       BlockState blockState = this.level.getBlockState(blockPos);
// |       GravestoneBlockEntity signBlockEntity2 = new GravestoneBlockEntity(blockPos, blockState);
// |       signBlockEntity2.setLevel(this.level);
// |       Minecraft.getInstance().gui.setScreen(new GravestoneScreen((GravestoneBlockEntity)foundEntity, false));
// |       info.cancel();
// |    }
// | }
