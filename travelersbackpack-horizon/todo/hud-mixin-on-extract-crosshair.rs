// Not ported yet: the Java @Inject(HEAD) on Hud.extractCrosshair (HudMixin.onExtractCrosshair).
// Why it's off: No Horizon function mapped for Hud.extractCrosshair (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for Hud.extractCrosshair: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   Minecraft.getInstance -> `self`. Hooks are methods on Game: self is what Minecraft.getInstance() gave.
//   Minecraft.gui -> (not mapped yet)
//   Gui.screen -> (not mapped yet)
//   Hud.extractCrosshair -> (not mapped yet)
//
// Decompiled Java, com/tiviacz/travelersbackpack/mixin/HudMixin.java:16:
// | @Inject(method = "extractCrosshair", at = @At("HEAD"), cancellable = true)
// | private void onExtractCrosshair(GuiGraphicsExtractor graphics, DeltaTracker deltaTracker, CallbackInfo ci) {
// |    if (Minecraft.getInstance().gui.screen() instanceof ToolsScreen && !RadialToolsOverlay.drawCrosshair) {
// |       ci.cancel();
// |    }
// | }
