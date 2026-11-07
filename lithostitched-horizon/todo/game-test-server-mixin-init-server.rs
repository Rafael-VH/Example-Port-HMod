// Not ported yet: the Java @Inject(INVOKE) on GameTestServer.initServer (GameTestServerMixin.initServer).
// Why it's off: No Horizon function mapped for GameTestServer.initServer (run `match`, or map it in a --mappings file).
// horizon-mixins kind: BEFORE_CALL
// Note: Only whole statements, plain let initializers and block tails can be selected.
// Note: No Horizon function known for GameTestServer.initServer: run `match`.
// Note: The call it hooks (GameTestServer.loadLevel) isn't mapped to a Rust function yet.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   GameTestServer.initServer -> (not mapped yet)
//   GameTestServer.loadLevel -> (not mapped yet)
//
// Decompiled Java, dev/worldgen/lithostitched/mixin/common/GameTestServerMixin.java:14:
// | @Inject(method = "initServer", at = @At(value = "INVOKE", target = "Lnet/minecraft/gametest/framework/GameTestServer;loadLevel()V", shift = Shift.BEFORE))
// | private void initServer(CallbackInfoReturnable<Boolean> info) {
// |    LithostitchedInternalHooks.onServerAboutToStart((MinecraftServer)this);
// | }
