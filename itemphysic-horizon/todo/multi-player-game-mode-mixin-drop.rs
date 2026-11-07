// Not ported yet: the Java @Inject(HEAD) on MultiPlayerGameMode.dropItem (MultiPlayerGameModeMixin.drop).
// Why it's off: No Horizon function mapped for MultiPlayerGameMode.dropItem (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for MultiPlayerGameMode.dropItem: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   MultiPlayerGameMode.dropItem -> (not mapped yet)
//
// Decompiled Java, team/creative/itemphysic/mixin/MultiPlayerGameModeMixin.java:13:
// | @Inject(method = "dropItem(Lnet/minecraft/client/player/LocalPlayer;Z)V", at = @At("HEAD"), cancellable = true, require = 1)
// | public void drop(LocalPlayer player, boolean all, CallbackInfo info) {
// |    if (ItemPhysic.CONFIG.throwConfig.enabled) {
// |       info.cancel();
// |    }
// | }
