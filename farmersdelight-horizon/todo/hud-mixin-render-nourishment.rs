// Not ported yet: the Java @Inject(TAIL) on Hud.extractFood (HudMixin.farmersdelightrefabricated$renderNourishment).
// Why it's off: No Horizon function mapped for Hud.extractFood (run `match`, or map it in a --mappings file).
// horizon-mixins kind: RETURN
// Note: Horizon has no TAIL: RETURN runs at every exit, early ones too.
// Note: No Horizon function known for Hud.extractFood: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   Hud.extractFood -> (not mapped yet)
//   Hud.extractHearts -> (not mapped yet)
//   Hud.extractHotbarAndDecorations -> (not mapped yet)
//
// Decompiled Java, vectorwing/farmersdelight/common/mixin/refabricated/HudMixin.java:47:
// | @Inject(method = "extractFood", at = @At("TAIL"))
// | private void farmersdelightrefabricated$renderNourishment(GuiGraphicsExtractor guiGraphics, Player player, int y, int x, CallbackInfo ci) {
// |    HUDOverlays.NourishmentOverlay.INSTANCE.render(guiGraphics, this.farmersdelightrefabricated$deltaTracker);
// | }
