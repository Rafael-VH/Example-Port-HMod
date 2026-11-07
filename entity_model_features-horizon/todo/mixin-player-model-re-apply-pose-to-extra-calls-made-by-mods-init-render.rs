// Not ported yet: the Java @Inject(TAIL) on PlayerModel.setupAnim (Mixin_PlayerModel_ReApplyPoseToExtraCallsMadeByMods.emf$initRender).
// Why it's off: No Horizon function mapped for PlayerModel.setupAnim (run `match`, or map it in a --mappings file).
// horizon-mixins kind: RETURN
// Note: Horizon has no TAIL: RETURN runs at every exit, early ones too.
// Note: No Horizon function known for PlayerModel.setupAnim: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   PlayerModel.setupAnim -> (not mapped yet)
//
// Decompiled Java, traben/entity_model_features/mixin/mixins/rendering/model/Mixin_PlayerModel_ReApplyPoseToExtraCallsMadeByMods.java:14:
// | @Inject(method = "setupAnim(Lnet/minecraft/client/renderer/entity/state/AvatarRenderState;)V", at = @At("TAIL"))
// | private void emf$initRender(CallbackInfo ci, @Local(argsOnly = true) AvatarRenderState avatarRenderState) {
// |    EMFEntityRenderState state = EMFEntityRenderState.from(avatarRenderState);
// |    if (state != null && state.getBipedPose() != null) {
// |       state.getBipedPose().applyTo((PlayerModel)this);
// |    }
// | }
