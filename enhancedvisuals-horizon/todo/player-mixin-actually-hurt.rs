// Not ported yet: the Java @Inject(INVOKE) on Player.actuallyHurt (PlayerMixin.actuallyHurt).
// Why it's off: No Horizon function mapped for Player.actuallyHurt (run `match`, or map it in a --mappings file).
// horizon-mixins kind: BEFORE_CALL
// Note: Only whole statements, plain let initializers and block tails can be selected.
// Note: No Horizon function known for Player.actuallyHurt: run `match`.
// Note: The call it hooks (Player.getHealth) isn't mapped to a Rust function yet.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   Player.actuallyHurt -> (not mapped yet)
//   Player.getHealth -> (not mapped yet)
//
// Decompiled Java, team/creative/enhancedvisuals/mixin/PlayerMixin.java:14:
// | @Inject(
// |    at = @At(value = "INVOKE", target = "Lnet/minecraft/world/entity/player/Player;getHealth()F"),
// |    method = "Lnet/minecraft/world/entity/player/Player;actuallyHurt(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/damagesource/DamageSource;F)V",
// |    require = 1
// | )
// | private void actuallyHurt(ServerLevel serverLevel, DamageSource damageSource, float f, CallbackInfo info) {
// |    EnhancedVisuals.EVENTS.damage((Player)this, damageSource, f);
// | }
