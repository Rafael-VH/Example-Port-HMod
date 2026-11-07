// Not ported yet: the Java @Inject(INVOKE) on GameRenderer.render (GameRendererMixin.extractGuiEnd).
// Why it's off: Its target is the `frame` hook point, an empty function: AFTER_CALL has nothing inside it to select. Port it in the `frame` hook's helper.
// horizon-mixins kind: AFTER_CALL
// Note: Hook points are empty functions: use HEAD on the hook instead of AFTER_CALL.
// Note: The call it hooks (GuiRenderer.endFrame) isn't mapped to a Rust function yet.
// Note: Target mapping is approximate.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   Minecraft.getDeltaTracker -> (not mapped yet)
//   GuiRenderer.endFrame -> (not mapped yet)
//   GameRenderer.render -> hook `frame`
//
// Decompiled Java, team/creative/enhancedvisuals/mixin/GameRendererMixin.java:25:
// | @Inject(
// |    method = "render()V",
// |    require = 1,
// |    at = @At(value = "INVOKE", shift = Shift.AFTER, target = "Lnet/minecraft/client/gui/render/GuiRenderer;endFrame()V")
// | )
// | public void extractGuiEnd(CallbackInfo info) {
// |    EVRenderer.renderShaders(this.minecraft.getDeltaTracker());
// | }
