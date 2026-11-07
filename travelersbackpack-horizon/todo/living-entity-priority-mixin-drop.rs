// Not ported yet: the Java @Inject(HEAD) on LivingEntity.dropAllDeathLoot (LivingEntityPriorityMixin.drop).
// Why it's off: No Horizon function mapped for LivingEntity.dropAllDeathLoot (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for LivingEntity.dropAllDeathLoot: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   LivingEntity.dropAllDeathLoot -> (not mapped yet)
//
// Decompiled Java, com/tiviacz/travelersbackpack/mixin/LivingEntityPriorityMixin.java:16:
// | @Inject(method = "dropAllDeathLoot", at = @At("HEAD"))
// | private void drop(ServerLevel level, DamageSource damageSource, CallbackInfo ci) {
// |    LivingEntity e = (LivingEntity)this;
// |    if (e instanceof ServerPlayer player) {
// |       if (AttachmentUtils.isWearingBackpack(player)) {
// |          BackpackManager.addBackpack(player, AttachmentUtils.getWearingBackpack(player));
// |       }
// |    }
// | }
