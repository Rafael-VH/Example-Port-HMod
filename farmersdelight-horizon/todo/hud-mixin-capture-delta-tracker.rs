// Not ported yet: the Java @Inject(HEAD) on Hud.extractHotbarAndDecorations (HudMixin.farmersdelightrefabricated$captureDeltaTracker).
// Why it's off: No Horizon function mapped for Hud.extractHotbarAndDecorations (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for Hud.extractHotbarAndDecorations: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   Hud.extractFood -> (not mapped yet)
//   Hud.extractHearts -> (not mapped yet)
//   Hud.extractHotbarAndDecorations -> (not mapped yet)
//
// Decompiled Java, vectorwing/farmersdelight/common/mixin/refabricated/HudMixin.java:19:
// | @Inject(method = "extractHotbarAndDecorations", at = @At("HEAD"))
// | private void farmersdelightrefabricated$captureDeltaTracker(GuiGraphicsExtractor guiGraphics, DeltaTracker deltaTracker, CallbackInfo ci) {
// |    this.farmersdelightrefabricated$deltaTracker = deltaTracker;
// | }
