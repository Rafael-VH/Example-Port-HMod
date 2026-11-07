// Not ported yet: the Java @Inject(TAIL) on GuiRenderer.render (Mixin_GuiEntityTester.etf$afterRenderToTexture).
// Why it's off: No Horizon function mapped for GuiRenderer.render (run `match`, or map it in a --mappings file).
// horizon-mixins kind: RETURN
// Note: Horizon has no TAIL: RETURN runs at every exit, early ones too.
// Note: No Horizon function known for GuiRenderer.render: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   GuiRenderer.render -> (not mapped yet)
//
// Decompiled Java, traben/entity_model_features/mixin/mixins/accessor/Mixin_GuiEntityTester.java:17:
// | @Inject(method = "render", at = @At("TAIL"))
// | private void etf$afterRenderToTexture(CallbackInfo ci) {
// |    EMFState.isInGui = false;
// | }
