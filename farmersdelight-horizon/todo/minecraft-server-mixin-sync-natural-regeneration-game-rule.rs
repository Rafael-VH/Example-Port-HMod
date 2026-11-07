// Not ported yet: the Java @Inject(HEAD) on MinecraftServer.onGameRuleChanged (MinecraftServerMixin.fdrf$syncNaturalRegenerationGameRule).
// Why it's off: No Horizon function mapped for MinecraftServer.onGameRuleChanged (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for MinecraftServer.onGameRuleChanged: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   MinecraftServer.onGameRuleChanged -> (not mapped yet)
//   PlayerList.getPlayers -> (not mapped yet)
//   GameRules.NATURAL_HEALTH_REGENERATION -> (not mapped yet)
//
// Decompiled Java, vectorwing/farmersdelight/common/mixin/refabricated/MinecraftServerMixin.java:20:
// | @Inject(method = "onGameRuleChanged", at = @At("HEAD"))
// | private <T> void fdrf$syncNaturalRegenerationGameRule(GameRule<T> rule, T value, CallbackInfo ci) {
// |    if (rule == GameRules.NATURAL_HEALTH_REGENERATION) {
// |       this.playerList.getPlayers().forEach(player -> ServerPlayNetworking.send(player, new SendNaturalRegenerationValuePayload((Boolean)value)));
// |    }
// | }
