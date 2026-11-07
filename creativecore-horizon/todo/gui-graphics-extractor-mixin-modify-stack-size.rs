// Not ported yet: the Java @WrapOperation(NEW) on GuiGraphicsExtractor.<init> (GuiGraphicsExtractorMixin.modifyStackSize).
// Why it's off: No Horizon function mapped for GuiGraphicsExtractor.<init> (run `match`, or map it in a --mappings file).
// horizon-mixins kind: WRAP_OPERATION
// Note: No Horizon function known for GuiGraphicsExtractor.<init>: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   Minecraft.font -> `(implicit)`. native_text and native_text_width use the game's font.
//   GuiGraphicsExtractor.<init> -> (not mapped yet)
//   GuiGraphicsExtractor.scissorStack -> (not mapped yet)
//   ScissorStack.peek -> (not mapped yet)
//
// Decompiled Java, team/creative/creativecore/mixin/GuiGraphicsExtractorMixin.java:50:
// | @WrapOperation(
// |    method = "<init>(Lnet/minecraft/client/Minecraft;Lnet/minecraft/client/renderer/state/gui/GuiRenderState;II)V",
// |    require = 1,
// |    at = @At(value = "NEW", target = "(I)Lorg/joml/Matrix3x2fStack;")
// | )
// | private static Matrix3x2fStack modifyStackSize(int stacksize, Operation<Matrix3x2fStack> op) {
// |    return new Matrix3x2fStack(64);
// | }
