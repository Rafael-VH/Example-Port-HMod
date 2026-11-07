// Not ported yet: the Java @Inject(HEAD) on AbstractContainerScreen.mouseReleased (AbstractContainerScreenMixin.handleWhenYouReleaseMouse).
// Why it's off: No Horizon function mapped for AbstractContainerScreen.mouseReleased (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for AbstractContainerScreen.mouseReleased: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   Screen.<init> -> (not mapped yet)
//   Screen.mouseReleased -> (not mapped yet)
//   AbstractContainerScreen.mouseReleased -> (not mapped yet)
//
// Decompiled Java, team/creative/creativecore/mixin/AbstractContainerScreenMixin.java:18:
// | @Inject(method = "mouseReleased", at = @At("HEAD"))
// | private void handleWhenYouReleaseMouse(MouseButtonEvent mouseButtonEvent, CallbackInfoReturnable<Boolean> cir) {
// |    super.mouseReleased(mouseButtonEvent);
// | }
