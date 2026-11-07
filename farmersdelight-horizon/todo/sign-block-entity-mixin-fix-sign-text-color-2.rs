// Not ported yet: the Java @ModifyExpressionValue(FIELD) on SignBlockEntity.loadAdditional (SignBlockEntityMixin.fdrf$fixSignTextColor).
// Why it's off: No Horizon function mapped for SignBlockEntity.loadAdditional (run `match`, or map it in a --mappings file).
// horizon-mixins kind: MODIFY_EXPRESSION_VALUE
// Note: Selects an exact Rust expression: pick it in the target function.
// Note: No Horizon function known for SignBlockEntity.loadAdditional: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   DyeColor.WHITE -> (not mapped yet)
//   BlockEntity.<init> -> (not mapped yet)
//   SignBlockEntity.<init> -> (not mapped yet)
//   SignBlockEntity.applyImplicitComponents -> (not mapped yet)
//   SignBlockEntity.loadAdditional -> (not mapped yet)
//   SignText.asMutable -> (not mapped yet)
//   Mutable.asImmutable -> (not mapped yet)
//   Mutable.setColor -> (not mapped yet)
//   BlockState.getBlock -> (not mapped yet)
//
// Decompiled Java, vectorwing/farmersdelight/common/mixin/refabricated/SignBlockEntityMixin.java:23:
// |          "<init>(Lnet/minecraft/world/level/block/entity/BlockEntityType;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;)V",
// |          "loadAdditional",
// |          "applyImplicitComponents"
// |    },
// |    at = @At(value = "FIELD", target = "Lnet/minecraft/world/level/block/entity/SignText;EMPTY:Lnet/minecraft/world/level/block/entity/SignText;")
// | )
// | private SignText fdrf$fixSignTextColor(SignText signText) {
// |    return this.getBlockState().getBlock() instanceof CanvasSign canvasSignBlock && canvasSignBlock.isDarkBackground()
// |       ? signText.asMutable().setColor(DyeColor.WHITE).asImmutable()
// |       : signText;
// | }
