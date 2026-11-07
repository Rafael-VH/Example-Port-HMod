// Not ported yet: the Java @Inject(INVOKE) on PlayerAdvancements.award (PlayerAdvancementsMixin.be_award).
// Why it's off: `horizon_client::game::advancements::Game<'_>::award` has a closure in its body: horizon-mixins doesn't rewrite such bodies with AFTER_CALL. A HEAD hook on it, or a AFTER_CALL on a function it calls, may work instead.
// horizon-mixins kind: AFTER_CALL on `horizon_client::game::advancements::Game<'_>::award`
// Note: The call it hooks (AdvancementRewards.grant) isn't mapped to a Rust function yet.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   AdvancementRewards.grant -> (not mapped yet)
//   PlayerAdvancements.award -> horizon_client::game::advancements::Game<'_>::award (suggested, high)
//
// Decompiled Java, org/betterx/betterend/mixin/common/PlayerAdvancementsMixin.java:19:
// | @Inject(
// |    method = "award",
// |    at = @At(
// |       value = "INVOKE",
// |       target = "Lnet/minecraft/advancements/AdvancementRewards;grant(Lnet/minecraft/server/level/ServerPlayer;)V",
// |       shift = Shift.AFTER
// |    )
// | )
// | public void be_award(AdvancementHolder advancementHolder, String criterionName, CallbackInfoReturnable<Boolean> cir) {
// |    ((PlayerAdvancementsCallback)PlayerAdvancementsCallback.PLAYER_ADVANCEMENT_COMPLETE.invoker())
// |       .onAdvancementComplete(this.player, advancementHolder, criterionName);
// | }
