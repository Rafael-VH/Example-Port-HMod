// Not ported yet: the Java @Inject(HEAD) on GuiRenderer.render (Mixin_GuiEntityTester.etf$beforeRenderToTexture).
// Why it's off: No Horizon function mapped for GuiRenderer.render (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for GuiRenderer.render: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   GuiRenderer.render -> (not mapped yet)
//
// Decompiled Java, traben/entity_model_features/mixin/mixins/accessor/Mixin_GuiEntityTester.java:12:
// | @Inject(method = "render", at = @At("HEAD"))
// | private void etf$beforeRenderToTexture(CallbackInfo ci) {
// |    EMFState.isInGui = true;
// | }
