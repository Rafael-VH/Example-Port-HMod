// Not ported yet: the Java @Inject(INVOKE) on Minecraft.startUseItem (MinecraftMixin.rightClickEarly).
// Why it's off: The call it hooks (Lnet/minecraft/world/InteractionHand;values()[Lnet/minecraft/world/InteractionHand;) isn't mapped to a Rust function.
// horizon-mixins kind: BEFORE_CALL on `horizon_client::first_person::Clicks::start_use_item`
// Note: Only whole statements, plain let initializers and block tails can be selected.
// Note: The call it hooks (InteractionHand.values) isn't mapped to a Rust function yet.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   Minecraft.hitResult -> (not mapped yet)
//   Minecraft.level -> (not mapped yet)
//   Minecraft.player -> `self.native_player()`. Option<NativePlayer>: x, y, z, yaw, pitch, dimension only.
//   Minecraft.startUseItem -> horizon_client::first_person::Clicks::start_use_item (suggested, high)
//   InteractionHand.values -> (not mapped yet)
//   HitResult.getType -> (not mapped yet)
//   Type.MISS -> (not mapped yet)
//
// Decompiled Java, team/creative/itemphysic/mixin/MinecraftMixin.java:22:
// | @Inject(
// |    method = "startUseItem()V",
// |    at = @At(value = "INVOKE", target = "Lnet/minecraft/world/InteractionHand;values()[Lnet/minecraft/world/InteractionHand;"),
// |    cancellable = true,
// |    require = 1
// | )
// | public void rightClickEarly(CallbackInfo info) {
// |    if (ItemPhysicClient.onPlayerInteract(((Minecraft)this).player)) {
// |       info.cancel();
// |    }
// | }
