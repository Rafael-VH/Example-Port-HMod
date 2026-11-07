// Not ported yet: the Java @Inject(HEAD) on PiglinAi.isWearingSafeArmor (PiglinAiMixin.bn_isWearingSafeArmor).
// Why it's off: No Horizon function mapped for PiglinAi.isWearingSafeArmor (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for PiglinAi.isWearingSafeArmor: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   EquipmentSlotGroup.ARMOR -> (not mapped yet)
//   EquipmentSlotGroup.iterator -> (not mapped yet)
//   LivingEntity.getItemBySlot -> (not mapped yet)
//   PiglinAi.isWearingSafeArmor -> (not mapped yet)
//   ItemStack.getItem -> (not mapped yet)
//
// Decompiled Java, org/betterx/betternether/mixin/common/piglin/PiglinAiMixin.java:38:
// | @Inject(method = "isWearingSafeArmor", at = @At("HEAD"), cancellable = true)
// | private static void bn_isWearingSafeArmor(LivingEntity livingEntity, CallbackInfoReturnable<Boolean> cir) {
// |    if ((Boolean)Configs.GAME_RULES.piglinIgnoreNetherArmor.get()) {
// |       for (EquipmentSlot slot : EquipmentSlotGroup.ARMOR) {
// |          if (bn_isNetherArmor(livingEntity.getItemBySlot(slot).getItem())) {
// |             cir.setReturnValue(true);
// |             return;
// |          }
// |       }
// |    }
// | }
