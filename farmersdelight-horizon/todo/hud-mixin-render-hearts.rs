// Not ported yet: the Java @Inject(TAIL) on Hud.extractHearts (HudMixin.farmersdelightrefabricated$renderHearts).
// Why it's off: No Horizon function mapped for Hud.extractHearts (run `match`, or map it in a --mappings file).
// horizon-mixins kind: RETURN
// Note: Horizon has no TAIL: RETURN runs at every exit, early ones too.
// Note: No Horizon function known for Hud.extractHearts: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   Hud.extractFood -> (not mapped yet)
//   Hud.extractHearts -> (not mapped yet)
//   Hud.extractHotbarAndDecorations -> (not mapped yet)
//
// Decompiled Java, vectorwing/farmersdelight/common/mixin/refabricated/HudMixin.java:29:
// | @Inject(method = "extractHearts", at = @At("TAIL"))
// | private void farmersdelightrefabricated$renderHearts(
// |    GuiGraphicsExtractor guiGraphics,
// |    Player player,
// |    int x,
// |    int y,
// |    int height,
// |    int offsetHeartIndex,
// |    float maxHealth,
// |    int currentHealth,
// |    int displayHealth,
// |    int absorptionAmount,
// |    boolean renderHighlight,
// |    CallbackInfo ci
// | ) {
// |    HUDOverlays.ComfortOverlay.INSTANCE.render(guiGraphics, this.farmersdelightrefabricated$deltaTracker);
// | }
