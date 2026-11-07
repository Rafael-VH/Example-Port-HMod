// Not ported yet: the Java @Inject(INVOKE) on ServerRecipeBook.sendInitialRecipeBook (ServerRecipeBookMixin.fdrf$sendInitialCategoryRecipeBook).
// Why it's off: No Horizon function mapped for ServerRecipeBook.sendInitialRecipeBook (run `match`, or map it in a --mappings file).
// horizon-mixins kind: BEFORE_CALL
// Note: Only whole statements, plain let initializers and block tails can be selected.
// Note: No Horizon function known for ServerRecipeBook.sendInitialRecipeBook: run `match`.
// Note: The call it hooks (ServerGamePacketListenerImpl.send) isn't mapped to a Rust function yet.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   ServerGamePacketListenerImpl.send -> (not mapped yet)
//   RecipeBook.<init> -> (not mapped yet)
//   RecipeBookSettings.isFiltering -> (not mapped yet)
//   RecipeBookSettings.isOpen -> (not mapped yet)
//   ServerRecipeBook.sendInitialRecipeBook -> (not mapped yet)
//
// Decompiled Java, vectorwing/farmersdelight/common/mixin/refabricated/ServerRecipeBookMixin.java:16:
// | @Inject(
// |    method = "sendInitialRecipeBook",
// |    at = @At(
// |       value = "INVOKE",
// |       target = "Lnet/minecraft/server/network/ServerGamePacketListenerImpl;send(Lnet/minecraft/network/protocol/Packet;)V",
// |       ordinal = 1
// |    )
// | )
// | private void fdrf$sendInitialCategoryRecipeBook(ServerPlayer player, CallbackInfo ci) {
// |    ServerPlayNetworking.send(
// |       player,
// |       new SendRecipeBookValuesPayload(
// |          this.getBookSettings().isOpen(FDRecipeBookTypes.COOKING), this.getBookSettings().isFiltering(FDRecipeBookTypes.COOKING)
// |       )
// |    );
// | }
