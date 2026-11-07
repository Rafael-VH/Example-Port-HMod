# Horizon port report: JeremySeq's Damage Indicator

`jeremyseqsdamageindicators` 2.3.0 · jdamageindicators-fabric-26.3-2.3.0.jar · 18 classes

**Convertible**

| Status | Count | Meaning |
|---|---|---|
| auto | 3 | The Horizon hook or injection is known: the wiring can be generated. The Java body still has to become Rust. |
| content | 1 | Data and assets: horizon-port-pack carries these into a content mod (.hmod). |
| needs-mapping | 0 | The kind of change carries over, but the matching Horizon function isn't in the table yet. Run `match`. |
| needs-translation | 0 | No direct equivalent: a person (or an AI agent) has to rewrite this part. |
| unsupported | 0 | Horizon has nothing this can attach to yet. |

## Entry points

- `main`: dev.jeremyseq.fabric.DamageIndicators
- `client`: dev.jeremyseq.fabric.DamageIndicatorsClient

## Fabric events

| Event | In | Horizon | Status | Note |
|---|---|---|---|---|
| HudElementRegistry.addLast | `FabricClientEvents` | hook `hud` | auto |  |
| ClientTickEvents.END_CLIENT_TICK | `FabricClientTickHandler` | hook `tick` | auto | The Fabric API layer (jfabricapi.rs) runs START_CLIENT_TICK, then the level ticks (START/END_LEVEL_TICK, while in a world), then END_CLIENT_TICK, from Horizon's tick hook, in the window's Game only. |

## Mixins

| Injection | In | Horizon | Status | Note |
|---|---|---|---|---|
| @Inject(HEAD) on Player.attack | `PlayerAttackMixin.checkCrit` | HEAD on horizon_client::game::Game<'_>::attack | auto |  |

## Content

| What | In | Status | Note |
|---|---|---|---|
| 4 asset files, 0 data files | `jar resources` | content | Recipes, loot, tags, models, textures, worldgen: horizon-port-pack carries these. |

## Game methods and fields used

11 of 100 are mapped to Horizon. Every Java body that uses an unmapped one needs it mapped before its Rust can be written.

<details><summary>Mapped</summary>

| Java | Horizon |
|---|---|
| `net/minecraft/client/Minecraft#font` | `(implicit)` |
| `net/minecraft/client/Minecraft#getInstance` | `self` |
| `net/minecraft/client/Minecraft#player` | `self.native_player()` |
| `net/minecraft/client/gui/Font#width` | `self.native_text_width(text)` |
| `net/minecraft/world/entity/Entity#getX` | `self.native_player()?.x` |
| `net/minecraft/world/entity/Entity#getY` | `self.native_player()?.y` |
| `net/minecraft/world/entity/Entity#getZ` | `self.native_player()?.z` |
| `net/minecraft/world/entity/player/Player#attack` | horizon_client::game::Game<'_>::attack (suggested) |
| `net/minecraft/world/entity/player/Player#getAttackStrengthScale` | horizon_client::game::Game<'_>::attack_strength_scale (suggested) |
| `net/minecraft/world/entity/player/Player#hasEffect` | horizon_client::mobs::PlayerView::has_effect (suggested) |
| `net/minecraft/world/entity/player/Player#onGround` | horizon_client::game::minecarts::Minecart::on_ground (suggested) |

</details>

<details><summary>Not mapped yet</summary>

| Java | Kind | Classes using it |
|---|---|---|
| `com/mojang/blaze3d/platform/NativeImage#close` | method | 1 |
| `com/mojang/blaze3d/platform/NativeImage#read` | method | 1 |
| `com/mojang/blaze3d/platform/Window#getGuiScaledHeight` | method | 1 |
| `com/mojang/blaze3d/platform/Window#getGuiScaledWidth` | method | 1 |
| `com/mojang/blaze3d/vertex/PoseStack#<init>` | method | 2 |
| `com/mojang/blaze3d/vertex/PoseStack#rotate` | method | 2 |
| `com/mojang/blaze3d/vertex/PoseStack#scale` | method | 2 |
| `com/mojang/blaze3d/vertex/PoseStack#translate` | method | 2 |
| `net/minecraft/client/Camera#position` | method | 2 |
| `net/minecraft/client/Camera#rotation` | method | 2 |
| `net/minecraft/client/CameraType#isMirrored` | method | 1 |
| `net/minecraft/client/DeltaTracker#getGameTimeDeltaTicks` | method | 1 |
| `net/minecraft/client/Minecraft#gameRenderer` | field | 1 |
| `net/minecraft/client/Minecraft#getWindow` | method | 1 |
| `net/minecraft/client/Minecraft#gui` | field | 1 |
| `net/minecraft/client/Minecraft#hitResult` | field | 1 |
| `net/minecraft/client/Minecraft#level` | field | 2 |
| `net/minecraft/client/Minecraft#options` | field | 1 |
| `net/minecraft/client/Options#getCameraType` | method | 1 |
| `net/minecraft/client/gui/Font$DisplayMode#SEE_THROUGH` | field | 2 |
| `net/minecraft/client/gui/Gui#hud` | field | 1 |
| `net/minecraft/client/gui/GuiGraphicsExtractor#blit` | method | 1 |
| `net/minecraft/client/gui/GuiGraphicsExtractor#fill` | method | 1 |
| `net/minecraft/client/gui/GuiGraphicsExtractor#pose` | method | 1 |
| `net/minecraft/client/gui/Hud#isHidden` | method | 1 |
| `net/minecraft/client/multiplayer/ClientLevel#getEntity` | method | 1 |
| `net/minecraft/client/player/LocalPlayer#getForward` | method | 1 |
| `net/minecraft/client/player/LocalPlayer#getId` | method | 1 |
| `net/minecraft/client/player/LocalPlayer#level` | method | 1 |
| `net/minecraft/client/player/LocalPlayer#position` | method | 1 |
| `net/minecraft/client/renderer/GameRenderer#mainCamera` | method | 1 |
| `net/minecraft/client/renderer/RenderPipelines#GUI_TEXTURED` | field | 1 |
| `net/minecraft/client/renderer/SubmitNodeCollector#submitText` | method | 2 |
| `net/minecraft/network/chat/Component#literal` | method | 2 |
| `net/minecraft/network/chat/MutableComponent#getVisualOrderText` | method | 2 |
| `net/minecraft/resources/Identifier#fromNamespaceAndPath` | method | 2 |
| `net/minecraft/resources/Identifier#getPath` | method | 1 |
| `net/minecraft/server/packs/PackType#CLIENT_RESOURCES` | field | 1 |
| `net/minecraft/server/packs/resources/Resource#open` | method | 1 |
| `net/minecraft/server/packs/resources/ResourceManager#getResource` | method | 1 |
| `net/minecraft/server/packs/resources/ResourceManager#listResources` | method | 1 |
| `net/minecraft/world/damagesource/DamageSource#getEntity` | method | 1 |
| `net/minecraft/world/damagesource/DamageSource#getSourcePosition` | method | 1 |
| `net/minecraft/world/effect/MobEffects#BLINDNESS` | field | 1 |
| `net/minecraft/world/entity/Entity#getEyeHeight` | method | 1 |
| `net/minecraft/world/entity/Entity#getId` | method | 2 |
| `net/minecraft/world/entity/LivingEntity#getBbHeight` | method | 1 |
| `net/minecraft/world/entity/LivingEntity#getEyeY` | method | 2 |
| `net/minecraft/world/entity/LivingEntity#getHealth` | method | 1 |
| `net/minecraft/world/entity/LivingEntity#getId` | method | 1 |
| `net/minecraft/world/entity/LivingEntity#getLastDamageSource` | method | 1 |
| `net/minecraft/world/entity/LivingEntity#getMaxHealth` | method | 1 |
| `net/minecraft/world/entity/LivingEntity#getX` | method | 2 |
| `net/minecraft/world/entity/LivingEntity#getY` | method | 1 |
| `net/minecraft/world/entity/LivingEntity#getZ` | method | 2 |
| `net/minecraft/world/entity/LivingEntity#isAlive` | method | 1 |
| `net/minecraft/world/entity/player/Player#fallDistance` | field | 1 |
| `net/minecraft/world/entity/player/Player#getHealth` | method | 1 |
| `net/minecraft/world/entity/player/Player#getId` | method | 1 |
| `net/minecraft/world/entity/player/Player#getLastDamageSource` | method | 1 |
| `net/minecraft/world/entity/player/Player#getX` | method | 1 |
| `net/minecraft/world/entity/player/Player#getY` | method | 1 |
| `net/minecraft/world/entity/player/Player#getZ` | method | 1 |
| `net/minecraft/world/entity/player/Player#isInWater` | method | 1 |
| `net/minecraft/world/entity/player/Player#isPassenger` | method | 1 |
| `net/minecraft/world/entity/player/Player#level` | method | 1 |
| `net/minecraft/world/entity/player/Player#onClimbable` | method | 1 |
| `net/minecraft/world/level/ClipContext#<init>` | method | 2 |
| `net/minecraft/world/level/ClipContext$Block#COLLIDER` | field | 2 |
| `net/minecraft/world/level/ClipContext$Fluid#ANY` | field | 1 |
| `net/minecraft/world/level/ClipContext$Fluid#NONE` | field | 1 |
| `net/minecraft/world/level/Level#clip` | method | 2 |
| `net/minecraft/world/level/Level#getEntities` | method | 1 |
| `net/minecraft/world/phys/AABB#<init>` | method | 1 |
| `net/minecraft/world/phys/BlockHitResult#getType` | method | 2 |
| `net/minecraft/world/phys/EntityHitResult#getEntity` | method | 1 |
| `net/minecraft/world/phys/HitResult$Type#MISS` | field | 2 |
| `net/minecraft/world/phys/Vec2#<init>` | method | 1 |
| `net/minecraft/world/phys/Vec2#add` | method | 1 |
| `net/minecraft/world/phys/Vec2#negated` | method | 1 |
| `net/minecraft/world/phys/Vec2#x` | field | 1 |
| `net/minecraft/world/phys/Vec2#y` | field | 1 |
| `net/minecraft/world/phys/Vec3#<init>` | method | 2 |
| `net/minecraft/world/phys/Vec3#add` | method | 1 |
| `net/minecraft/world/phys/Vec3#distanceTo` | method | 2 |
| `net/minecraft/world/phys/Vec3#subtract` | method | 1 |
| `net/minecraft/world/phys/Vec3#x` | field | 3 |
| `net/minecraft/world/phys/Vec3#y` | field | 2 |
| `net/minecraft/world/phys/Vec3#z` | field | 3 |

</details>

