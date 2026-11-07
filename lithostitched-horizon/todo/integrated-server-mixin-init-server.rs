// Not ported yet: the Java @Inject(INVOKE) on IntegratedServer.initServer (IntegratedServerMixin.initServer).
// Why it's off: No Horizon function mapped for IntegratedServer.initServer (run `match`, or map it in a --mappings file).
// horizon-mixins kind: BEFORE_CALL
// Note: Only whole statements, plain let initializers and block tails can be selected.
// Note: No Horizon function known for IntegratedServer.initServer: run `match`.
// Note: The call it hooks (IntegratedServer.loadLevel) isn't mapped to a Rust function yet.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   IntegratedServer.initServer -> (not mapped yet)
//   IntegratedServer.loadLevel -> (not mapped yet)
//
// Decompiled Java, dev/worldgen/lithostitched/mixin/client/IntegratedServerMixin.java:14:
// | @Inject(method = "initServer", at = @At(value = "INVOKE", target = "Lnet/minecraft/client/server/IntegratedServer;loadLevel()V", shift = Shift.BEFORE))
// | private void initServer(CallbackInfoReturnable<Boolean> info) {
// |    LithostitchedInternalHooks.onServerAboutToStart((MinecraftServer)this);
// | }
