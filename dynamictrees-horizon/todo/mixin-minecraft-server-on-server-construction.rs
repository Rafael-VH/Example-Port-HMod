// Not ported yet: the Java @Inject(RETURN) on MinecraftServer.<init> (MixinMinecraftServer.onServerConstruction).
// Why it's off: No Horizon function mapped for MinecraftServer.<init> (run `match`, or map it in a --mappings file).
// horizon-mixins kind: RETURN
// Note: Horizon's RETURN runs at every normal exit and passes the return value on.
// Note: No Horizon function known for MinecraftServer.<init>: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   MinecraftServer.<init> -> (not mapped yet)
//
// Decompiled Java, com/dtteam/dynamictrees/mixin/MixinMinecraftServer.java:12:
// | @Inject(
// |    method = "<init>(Ljava/lang/Thread;Lnet/minecraft/class_32$class_5143;Lnet/minecraft/class_3283;Lnet/minecraft/class_6904;Ljava/net/Proxy;Lcom/mojang/datafixers/DataFixer;Lnet/minecraft/class_7497;Lnet/minecraft/class_3950;)V",
// |    at = @At("RETURN")
// | )
// | private void onServerConstruction(CallbackInfo ci) {
// |    FabricMiscHelper.currentServer = (MinecraftServer)this;
// | }
