// Not ported yet: the Java @Inject(INVOKE) on DedicatedServer.initServer (DedicatedServerMixin.initServer).
// Why it's off: No Horizon function mapped for DedicatedServer.initServer (run `match`, or map it in a --mappings file).
// horizon-mixins kind: BEFORE_CALL
// Note: Only whole statements, plain let initializers and block tails can be selected.
// Note: No Horizon function known for DedicatedServer.initServer: run `match`.
// Note: The call it hooks (DedicatedServer.loadLevel) isn't mapped to a Rust function yet.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   DedicatedServer.initServer -> (not mapped yet)
//   DedicatedServer.loadLevel -> (not mapped yet)
//
// Decompiled Java, dev/worldgen/lithostitched/mixin/server/DedicatedServerMixin.java:14:
// | @Inject(
// |    method = "initServer",
// |    at = @At(value = "INVOKE", target = "Lnet/minecraft/server/dedicated/DedicatedServer;loadLevel()V", shift = Shift.BEFORE),
// |    allow = 1
// | )
// | private void initServer(CallbackInfoReturnable<Boolean> info) {
// |    LithostitchedInternalHooks.onServerAboutToStart((MinecraftServer)this);
// | }
