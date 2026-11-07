// Not ported yet: the Java @Inject(INVOKE) on MinecraftServer.tickServer (MinecraftServerMixin.tickPaused).
// Why it's off: `horizon_client::game::levels::Game<'_>::tick_server` has a `?` in its body: horizon-mixins doesn't rewrite such bodies with BEFORE_CALL. A HEAD hook on it, or a BEFORE_CALL on a function it calls, may work instead.
// horizon-mixins kind: BEFORE_CALL on `horizon_client::game::levels::Game<'_>::tick_server`
// Note: Only whole statements, plain let initializers and block tails can be selected.
// Note: The call it hooks (MinecraftServer.tickConnection) isn't mapped to a Rust function yet.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   MinecraftServer.tickConnection -> (not mapped yet)
//   MinecraftServer.tickServer -> horizon_client::game::levels::Game<'_>::tick_server (suggested, high)
//   ServerChunkCache.chunkMap -> (not mapped yet)
//   ServerLevel.getChunkSource -> (not mapped yet)
//   InactiveProfiler.INSTANCE -> (not mapped yet)
//   PersistentEntitySectionManager.tick -> (not mapped yet)
//
// Decompiled Java, org/popcraft/chunky/mixin/MinecraftServerMixin.java:25:
// | @Inject(method = "tickServer", at = @At(value = "INVOKE", target = "Lnet/minecraft/server/MinecraftServer;tickConnection()V"))
// | private void tickPaused(BooleanSupplier booleanSupplier, CallbackInfo ci) {
// |    this.chunky$runChunkSystemHousekeeping(booleanSupplier);
// | }
