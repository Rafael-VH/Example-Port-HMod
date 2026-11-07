// Not ported yet: the Java @Inject(INVOKE) on ExperienceOrb.playerTouch (ExperienceOrbEntityMixin.onPlayerCollision).
// Why it's off: No Horizon function mapped for ExperienceOrb.playerTouch (run `match`, or map it in a --mappings file).
// horizon-mixins kind: AFTER_CALL
// Note: No Horizon function known for ExperienceOrb.playerTouch: run `match`.
// Note: The call it hooks (Player.take) isn't mapped to a Rust function yet.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   ExperienceOrb.playerTouch -> (not mapped yet)
//   Player.take -> (not mapped yet)
//
// Decompiled Java, com/tiviacz/travelersbackpack/mixin/abilities/ExperienceOrbEntityMixin.java:22:
// | @Inject(
// |    method = "playerTouch",
// |    at = @At(value = "INVOKE", target = "Lnet/minecraft/world/entity/player/Player;take(Lnet/minecraft/world/entity/Entity;I)V", shift = Shift.AFTER)
// | )
// | public void onPlayerCollision(Player player, CallbackInfo ci) {
// |    if (TravelersBackpackConfig.serverSpec.isLoaded() && (Boolean)TravelersBackpackConfig.SERVER.backpackAbilities.enableBackpackAbilities.get()) {
// |       int value = this.getValue() * BackpackAbilities.ABILITIES.lapisAbility(player);
// |       this.setValue(value);
// |    }
// | }
