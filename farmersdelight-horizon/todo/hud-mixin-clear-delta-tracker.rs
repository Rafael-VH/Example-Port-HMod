// Not ported yet: the Java @Inject(TAIL) on Hud.extractHotbarAndDecorations (HudMixin.farmersdelightrefabricated$clearDeltaTracker).
// Why it's off: No Horizon function mapped for Hud.extractHotbarAndDecorations (run `match`, or map it in a --mappings file).
// horizon-mixins kind: RETURN
// Note: Horizon has no TAIL: RETURN runs at every exit, early ones too.
// Note: No Horizon function known for Hud.extractHotbarAndDecorations: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   Hud.extractFood -> (not mapped yet)
//   Hud.extractHearts -> (not mapped yet)
//   Hud.extractHotbarAndDecorations -> (not mapped yet)
//
// Decompiled Java, vectorwing/farmersdelight/common/mixin/refabricated/HudMixin.java:24:
// | @Inject(method = "extractHotbarAndDecorations", at = @At("TAIL"))
// | private void farmersdelightrefabricated$clearDeltaTracker(GuiGraphicsExtractor guiGraphics, DeltaTracker deltaTracker, CallbackInfo ci) {
// |    this.farmersdelightrefabricated$deltaTracker = null;
// | }
