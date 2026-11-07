// Not ported yet: the Java @Inject(TAIL) on HangingSignEditScreen.<init> (HangingSignEditScreenMixin.fdrf$fixSignTextures).
// Why it's off: No Horizon function mapped for HangingSignEditScreen.<init> (run `match`, or map it in a --mappings file).
// horizon-mixins kind: RETURN
// Note: Horizon has no TAIL: RETURN runs at every exit, early ones too.
// Note: No Horizon function known for HangingSignEditScreen.<init>: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   HangingSignEditScreen.<init> -> (not mapped yet)
//   DyeColor.getName -> (not mapped yet)
//   SignBlockEntity.getBlockState -> (not mapped yet)
//   BlockState.getBlock -> (not mapped yet)
//
// Decompiled Java, vectorwing/farmersdelight/common/mixin/refabricated/HangingSignEditScreenMixin.java:24:
// | @Inject(method = "<init>", at = @At("TAIL"))
// | private void fdrf$fixSignTextures(SignBlockEntity sign, SignTextSlot slot, boolean shouldFilter, CallbackInfo ci) {
// |    if (sign.getBlockState().getBlock() instanceof CanvasSign canvas) {
// |       this.texture = FarmersDelight.id(
// |          "textures/gui/hanging_signs/canvas" + (canvas.getBackgroundColor() == null ? "" : "_" + canvas.getBackgroundColor().getName()) + ".png"
// |       );
// |    }
// | }
