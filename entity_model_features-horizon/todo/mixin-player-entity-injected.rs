// Not ported yet: the Java @Inject(HEAD) on Player.interactOn (MixinPlayerEntity.emf$injected).
// Why it's off: No Horizon function mapped for Player.interactOn (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for Player.interactOn: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   Entity.getUUID -> (not mapped yet)
//   LivingEntity.level -> (not mapped yet)
//   Player.interactOn -> (not mapped yet)
//   Level.isClientSide -> horizon_world::level::is_client_side (suggested, high)
//
// Decompiled Java, traben/entity_model_features/mixin/mixins/MixinPlayerEntity.java:18:
// | @Inject(method = "interactOn", at = @At("HEAD"))
// | private void emf$injected(CallbackInfoReturnable<InteractionResult> cir, @Local(argsOnly = true) Entity entity) {
// |    if (((EMFConfig)EMF.config().getConfig()).debugOnRightClick && ((LivingEntity)this).level().isClientSide()) {
// |       EMFManager.getInstance().entityForDebugPrint = entity.getUUID();
// |    }
// | }
