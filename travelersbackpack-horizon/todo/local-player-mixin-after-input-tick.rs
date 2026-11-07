// Not ported yet: the Java @Inject(INVOKE) on LocalPlayer.aiStep (LocalPlayerMixin.afterInputTick).
// Why it's off: No Horizon function mapped for LocalPlayer.aiStep (run `match`, or map it in a --mappings file).
// horizon-mixins kind: AFTER_CALL
// Note: No Horizon function known for LocalPlayer.aiStep: run `match`.
// Note: The call it hooks (ClientInput.tick) isn't mapped to a Rust function yet.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   Minecraft.getInstance -> `self`. Hooks are methods on Game: self is what Minecraft.getInstance() gave.
//   Minecraft.gui -> (not mapped yet)
//   Minecraft.options -> (not mapped yet)
//   Minecraft.player -> `self.native_player()`. Option<NativePlayer>: x, y, z, yaw, pitch, dimension only.
//   Options.keyDown -> (not mapped yet)
//   Options.keyJump -> (not mapped yet)
//   Options.keyLeft -> (not mapped yet)
//   Options.keyRight -> (not mapped yet)
//   Options.keyShift -> (not mapped yet)
//   Options.keySprint -> (not mapped yet)
//   Options.keyUp -> (not mapped yet)
//   Gui.screen -> (not mapped yet)
//   ClientInput.keyPresses -> (not mapped yet)
//   ClientInput.moveVector -> (not mapped yet)
//   ClientInput.tick -> (not mapped yet)
//   LocalPlayer.aiStep -> (not mapped yet)
//   LocalPlayer.isMovingSlowly -> (not mapped yet)
//   Input.<init> -> (not mapped yet)
//   Input.backward -> (not mapped yet)
//   Input.forward -> (not mapped yet)
//   Input.left -> (not mapped yet)
//   Input.right -> (not mapped yet)
//   Vec2.<init> -> (not mapped yet)
//   Vec2.x -> (not mapped yet)
//   Vec2.y -> (not mapped yet)
//
// Decompiled Java, com/tiviacz/travelersbackpack/mixin/LocalPlayerMixin.java:23:
// | @Inject(method = "aiStep", at = @At(value = "INVOKE", target = "Lnet/minecraft/client/player/ClientInput;tick()V", shift = Shift.AFTER))
// | private void afterInputTick(CallbackInfo ci) {
// |    Minecraft mc = Minecraft.getInstance();
// |    if (mc.gui.screen() instanceof ToolsScreen) {
// |       Options settings = Minecraft.getInstance().options;
// |       this.input.keyPresses = new Input(
// |          KeybindHandler.isKeyDown(settings.keyUp),
// |          KeybindHandler.isKeyDown(settings.keyDown),
// |          KeybindHandler.isKeyDown(settings.keyLeft),
// |          KeybindHandler.isKeyDown(settings.keyRight),
// |          KeybindHandler.isKeyDown(settings.keyJump),
// |          KeybindHandler.isKeyDown(settings.keyShift),
// |          KeybindHandler.isKeyDown(settings.keySprint)
// |       );
// |       this.input.moveVector = new Vec2(
// |          this.input.keyPresses.left() == this.input.keyPresses.right() ? 0.0F : (this.input.keyPresses.left() ? 1.0F : -1.0F),
// |          this.input.keyPresses.forward() == this.input.keyPresses.backward() ? 0.0F : (this.input.keyPresses.forward() ? 1.0F : -1.0F)
// |       );
// |       if (Minecraft.getInstance().player.isMovingSlowly()) {
// |          this.input.moveVector = new Vec2((float)(this.input.moveVector.x * 0.3), (float)(this.input.moveVector.y * 0.3));
// |       }
// |    }
// | }
