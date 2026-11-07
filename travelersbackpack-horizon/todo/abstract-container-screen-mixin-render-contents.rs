// Not ported yet: the Java @Inject(TAIL) on AbstractContainerScreen.extractContents (AbstractContainerScreenMixin.renderContents).
// Why it's off: No Horizon function mapped for AbstractContainerScreen.extractContents (run `match`, or map it in a --mappings file).
// horizon-mixins kind: RETURN
// Note: Horizon has no TAIL: RETURN runs at every exit, early ones too.
// Note: No Horizon function known for AbstractContainerScreen.extractContents: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   AbstractContainerScreen.extractContents -> (not mapped yet)
//
// Decompiled Java, com/tiviacz/travelersbackpack/mixin/AbstractContainerScreenMixin.java:13:
// | @Inject(at = @At("TAIL"), method = "extractContents")
// | private void renderContents(GuiGraphicsExtractor guiGraphics, int i, int j, float f, CallbackInfo ci) {
// |    if (this instanceof AbstractContainerScreen<?> screen) {
// |       ScreenRenderHandler.renderAboveContents(screen, guiGraphics, i, j);
// |    }
// | }
