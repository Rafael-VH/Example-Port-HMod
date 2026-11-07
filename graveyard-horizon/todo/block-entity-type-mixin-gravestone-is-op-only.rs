// Not ported yet: the Java @Inject(HEAD) on BlockEntityType.onlyOpCanSetNbt (BlockEntityTypeMixin.graveyard$gravestoneIsOpOnly).
// Why it's off: No Horizon function mapped for BlockEntityType.onlyOpCanSetNbt (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for BlockEntityType.onlyOpCanSetNbt: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   BlockEntityType.onlyOpCanSetNbt -> (not mapped yet)
//
// Decompiled Java, com/lion/graveyard/mixin/BlockEntityTypeMixin.java:12:
// | @Inject(method = "onlyOpCanSetNbt", at = @At("HEAD"), cancellable = true)
// | private void graveyard$gravestoneIsOpOnly(CallbackInfoReturnable<Boolean> info) {
// |    if (this == TGBlockEntities.GRAVESTONE_BLOCK_ENTITY.get()) {
// |       info.setReturnValue(true);
// |    }
// | }
