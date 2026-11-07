// Not ported yet: the Java @Inject(HEAD) on Player.interactOn (MixinPlayerEntity_DebugClick.etf$injected).
// Why it's off: No Horizon function mapped for Player.interactOn (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for Player.interactOn: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   Entity.<init> -> (not mapped yet)
//   Entity.getUUID -> (not mapped yet)
//   Player.interactOn -> (not mapped yet)
//   Level.isClientSide -> horizon_world::level::is_client_side (suggested, high)
//
// Decompiled Java, traben/entity_texture_features/mixin/mixins/entity/misc/MixinPlayerEntity_DebugClick.java:23:
// | @Inject(method = "interactOn", at = @At("HEAD"))
// | private void etf$injected(CallbackInfoReturnable<InteractionResult> cir, @Local(argsOnly = true) Entity entity) {
// |    if (this.level().isClientSide() && ETF.config().getConfig().debugLoggingMode != ETFConfig.DebugLogMode.None) {
// |       ETFManager.getInstance().markEntityForDebugPrint(entity.getUUID());
// |    }
// | }
