// Not ported yet: the Java @Inject(INVOKE) on IntegratedServer.tickServer (IntegratedServerMixin.tickPaused).
// Why it's off: No Horizon function mapped for IntegratedServer.tickServer (run `match`, or map it in a --mappings file).
// horizon-mixins kind: BEFORE_CALL
// Note: Only whole statements, plain let initializers and block tails can be selected.
// Note: No Horizon function known for IntegratedServer.tickServer: run `match`.
// Note: The call it hooks (IntegratedServer.tickPaused) isn't mapped to a Rust function yet.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   IntegratedServer.tickPaused -> (not mapped yet)
//   IntegratedServer.tickServer -> (not mapped yet)
//
// Decompiled Java, org/popcraft/chunky/mixin/client/IntegratedServerMixin.java:13:
// | @Inject(method = "tickServer", at = @At(value = "INVOKE", target = "Lnet/minecraft/client/server/IntegratedServer;tickPaused()V"))
// | private void tickPaused(BooleanSupplier booleanSupplier, CallbackInfo ci) {
// |    this.chunky$runChunkSystemHousekeeping(booleanSupplier);
// | }
