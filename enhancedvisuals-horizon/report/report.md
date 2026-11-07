# Horizon port report: EnhancedVisuals

`enhancedvisuals` 1.8.35 · EnhancedVisuals_FABRIC_v1.8.35_mc26.3.jar · 83 classes

**Partly convertible**

| Status | Count | Meaning |
|---|---|---|
| auto | 0 | The Horizon hook or injection is known: the wiring can be generated. The Java body still has to become Rust. |
| content | 1 | Data and assets: horizon-port-pack carries these into a content mod (.hmod). |
| needs-mapping | 11 | The kind of change carries over, but the matching Horizon function isn't in the table yet. Run `match`. |
| needs-translation | 3 | No direct equivalent: a person (or an AI agent) has to rewrite this part. |
| unsupported | 0 | Horizon has nothing this can attach to yet. |

## Entry points

- `main`: team.creative.enhancedvisuals.EnhancedVisuals
- `client`: team.creative.enhancedvisuals.EnhancedVisuals

## Mixins

| Injection | In | Horizon | Status | Note |
|---|---|---|---|---|
| @Inject(TAIL) on ClientPacketListener.handleRespawn | `ClientPacketListenerMixin.handleRespawn` | RETURN | needs-mapping | Horizon has no TAIL: RETURN runs at every exit, early ones too. No Horizon function known for ClientPacketListener.handleRespawn: run `match`. |
| @Accessor on Entity.wasEyeInWater | `EntityAccessor.getWasEyeInWater` | GET | needs-mapping | A [[bridge]] GET (SET for a setter) on the Rust struct's field. No Horizon function known for Entity.wasEyeInWater: run `match`. |
| @WrapOperation(INVOKE) on ServerExplosion.hurtEntities at Lnet/minecraft/server/level/ServerLevel;getEntities(Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/phys/AABB;)Ljava/util/List; | `ExplosionMixin.onDetonate` | WRAP_OPERATION | needs-mapping | No Horizon function known for ServerExplosion.hurtEntities: run `match`. The call it hooks (ServerLevel.getEntities) isn't mapped to a Rust function yet. |
| @WrapOperation(INVOKE) on ServerExplosion.hurtEntities at Lnet/minecraft/server/level/ServerLevel;getEntities(Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/phys/AABB;)Ljava/util/List; | `ExplosionMixin.onDetonateFabric` | WRAP_OPERATION | needs-mapping | No Horizon function known for ServerExplosion.hurtEntities: run `match`. The call it hooks (ServerLevel.getEntities) isn't mapped to a Rust function yet. |
| @Inject(INVOKE) on GameRenderer.render at Lnet/minecraft/client/gui/render/GuiRenderer;endFrame()V | `GameRendererMixin.extractGuiEnd` | AFTER_CALL on hook `frame` | needs-translation | Hook points are empty functions: use HEAD on the hook instead of AFTER_CALL. The call it hooks (GuiRenderer.endFrame) isn't mapped to a Rust function yet. Target mapping is approximate. |
| @Inject(INVOKE) on Player.actuallyHurt at Lnet/minecraft/world/entity/player/Player;getHealth()F | `PlayerMixin.actuallyHurt` | BEFORE_CALL | needs-mapping | Only whole statements, plain let initializers and block tails can be selected. No Horizon function known for Player.actuallyHurt: run `match`. The call it hooks (Player.getHealth) isn't mapped to a Rust function yet. |
| @Accessor on PostChain.passes | `PostChainAccessor.getPasses` | GET | needs-mapping | A [[bridge]] GET (SET for a setter) on the Rust struct's field. No Horizon function known for PostChain.passes: run `match`. |
| @Accessor on PostPass.customUniforms | `PostPassAccessor.getCustomUniforms` | GET | needs-mapping | A [[bridge]] GET (SET for a setter) on the Rust struct's field. No Horizon function known for PostPass.customUniforms: run `match`. |
| @Inject(HEAD) on Projectile.onHit | `ProjectileMixin.onHit` | HEAD | needs-mapping | No Horizon function known for Projectile.onHit: run `match`. |
| @Accessor on SoundEngine.instanceToChannel | `SoundEngineAccessor.getInstanceToChannel` | GET | needs-mapping | A [[bridge]] GET (SET for a setter) on the Rust struct's field. No Horizon function known for SoundEngine.instanceToChannel: run `match`. |
| @Invoker on SoundEngine.invokeCalculateVolume | `SoundEngineAccessor.invokeCalculateVolume` | INVOKE | needs-mapping | A [[bridge]] INVOKE of the Rust method. No Horizon function known for SoundEngine.invokeCalculateVolume: run `match`. |
| @Accessor on SoundManager.soundEngine | `SoundManagerAccessor.getSoundEngine` | GET | needs-mapping | A [[bridge]] GET (SET for a setter) on the Rust struct's field. No Horizon function known for SoundManager.soundEngine: run `match`. |

## Classes extending the game

| Class | In | Status | Note |
|---|---|---|---|
| extends SimplePreparableReloadListener | `EVClient$1` | needs-translation | No mapping for this superclass yet. |
| extends AbstractSoundInstance | `PositionedSound` | needs-translation | No mapping for this superclass yet. |

## Content

| What | In | Status | Note |
|---|---|---|---|
| 133 asset files, 0 data files | `jar resources` | content | Recipes, loot, tags, models, textures, worldgen: horizon-port-pack carries these. |

## Game methods and fields used

12 of 182 are mapped to Horizon. Every Java body that uses an unmapped one needs it mapped before its Rust can be written.

<details><summary>Mapped</summary>

| Java | Horizon |
|---|---|
| `net/minecraft/client/Minecraft#font` | `(implicit)` |
| `net/minecraft/client/Minecraft#getInstance` | `self` |
| `net/minecraft/client/Minecraft#player` | `self.native_player()` |
| `net/minecraft/client/gui/Font#width` | `self.native_text_width(text)` |
| `net/minecraft/client/renderer/GameRenderer#render` | hook `frame` |
| `net/minecraft/world/entity/Entity#getX` | `self.native_player()?.x` |
| `net/minecraft/world/entity/Entity#getY` | `self.native_player()?.y` |
| `net/minecraft/world/entity/Entity#getZ` | `self.native_player()?.z` |
| `net/minecraft/world/entity/player/Player#onGround` | horizon_client::game::minecarts::Minecart::on_ground (suggested) |
| `net/minecraft/world/level/Level#isClientSide` | horizon_world::level::is_client_side (suggested) |
| `net/minecraft/world/level/Level#isRainingAt` | horizon_world::level::weather::Behaviours::is_raining_at (suggested) |
| `net/minecraft/world/level/ServerExplosion#getSeenPercent` | horizon_client::game::explosion::Game<'_>::seen_percent (suggested) |

</details>

<details><summary>Not mapped yet</summary>

| Java | Kind | Classes using it |
|---|---|---|
| `com/mojang/blaze3d/audio/Channel#setVolume` | method | 1 |
| `com/mojang/blaze3d/buffers/Std140Builder#get` | method | 3 |
| `com/mojang/blaze3d/buffers/Std140Builder#onStack` | method | 3 |
| `com/mojang/blaze3d/buffers/Std140Builder#putFloat` | method | 3 |
| `com/mojang/blaze3d/buffers/Std140SizeCalculator#<init>` | method | 3 |
| `com/mojang/blaze3d/buffers/Std140SizeCalculator#get` | method | 3 |
| `com/mojang/blaze3d/buffers/Std140SizeCalculator#putFloat` | method | 3 |
| `com/mojang/blaze3d/pipeline/RenderTarget#height` | field | 1 |
| `com/mojang/blaze3d/pipeline/RenderTarget#width` | field | 1 |
| `com/mojang/blaze3d/platform/Window#getGuiScale` | method | 1 |
| `com/mojang/blaze3d/platform/Window#getGuiScaledHeight` | method | 1 |
| `com/mojang/blaze3d/platform/Window#getGuiScaledWidth` | method | 1 |
| `com/mojang/blaze3d/platform/Window#getHeight` | method | 2 |
| `com/mojang/blaze3d/platform/Window#getWidth` | method | 2 |
| `com/mojang/blaze3d/resource/GraphicsResourceAllocator#UNPOOLED` | field | 1 |
| `com/mojang/blaze3d/systems/RenderSystem#getDevice` | method | 3 |
| `net/minecraft/client/DeltaTracker#getGameTimeDeltaPartialTick` | method | 1 |
| `net/minecraft/client/Minecraft#gameRenderer` | field | 2 |
| `net/minecraft/client/Minecraft#getDeltaTracker` | method | 2 |
| `net/minecraft/client/Minecraft#getResourceManager` | method | 2 |
| `net/minecraft/client/Minecraft#getShaderManager` | method | 1 |
| `net/minecraft/client/Minecraft#getSoundManager` | method | 2 |
| `net/minecraft/client/Minecraft#getWindow` | method | 2 |
| `net/minecraft/client/Minecraft#gui` | field | 1 |
| `net/minecraft/client/Minecraft#isPaused` | method | 2 |
| `net/minecraft/client/gui/Gui#screen` | method | 1 |
| `net/minecraft/client/gui/GuiGraphicsExtractor#pose` | method | 1 |
| `net/minecraft/client/gui/GuiGraphicsExtractor#text` | method | 1 |
| `net/minecraft/client/gui/render/GuiRenderer#endFrame` | method | 1 |
| `net/minecraft/client/gui/screens/Screen#width` | field | 1 |
| `net/minecraft/client/multiplayer/ClientLevel#entitiesForRendering` | method | 1 |
| `net/minecraft/client/multiplayer/ClientPacketListener#handleRespawn` | method | 1 |
| `net/minecraft/client/player/LocalPlayer#isCreative` | method | 1 |
| `net/minecraft/client/player/LocalPlayer#isSpectator` | method | 1 |
| `net/minecraft/client/renderer/GameRenderer#mainRenderTarget` | method | 2 |
| `net/minecraft/client/renderer/LevelTargetBundle#MAIN_TARGETS` | field | 1 |
| `net/minecraft/client/renderer/PostChain#passes` | method | 1 |
| `net/minecraft/client/renderer/PostChain#process` | method | 1 |
| `net/minecraft/client/renderer/PostPass#customUniforms` | method | 1 |
| `net/minecraft/client/renderer/ShaderManager#getPostChain` | method | 1 |
| `net/minecraft/client/resources/sounds/AbstractSoundInstance#<init>` | method | 1 |
| `net/minecraft/client/resources/sounds/SoundInstance#getIdentifier` | method | 1 |
| `net/minecraft/client/resources/sounds/SoundInstance$Attenuation#LINEAR` | field | 1 |
| `net/minecraft/client/resources/sounds/SoundInstance$Attenuation#NONE` | field | 1 |
| `net/minecraft/client/sounds/ChannelAccess$ChannelHandle#execute` | method | 1 |
| `net/minecraft/client/sounds/SoundEngine#instanceToChannel` | method | 1 |
| `net/minecraft/client/sounds/SoundEngine#invokeCalculateVolume` | method | 1 |
| `net/minecraft/client/sounds/SoundManager#play` | method | 1 |
| `net/minecraft/client/sounds/SoundManager#soundEngine` | method | 1 |
| `net/minecraft/core/BlockPos#below` | method | 1 |
| `net/minecraft/core/BlockPos#containing` | method | 1 |
| `net/minecraft/core/BlockPos#getX` | method | 2 |
| `net/minecraft/core/BlockPos#getY` | method | 1 |
| `net/minecraft/core/BlockPos#getZ` | method | 2 |
| `net/minecraft/core/Holder#unwrapKey` | method | 1 |
| `net/minecraft/core/Registry#getOrThrow` | method | 1 |
| `net/minecraft/core/RegistryAccess#lookupOrThrow` | method | 1 |
| `net/minecraft/core/component/DataComponentMap#get` | method | 1 |
| `net/minecraft/core/component/DataComponents#POTION_CONTENTS` | field | 1 |
| `net/minecraft/core/registries/Registries#ENCHANTMENT` | field | 1 |
| `net/minecraft/network/chat/Component#getString` | method | 1 |
| `net/minecraft/network/chat/Component#literal` | method | 1 |
| `net/minecraft/resources/Identifier#equals` | method | 1 |
| `net/minecraft/resources/Identifier#getPath` | method | 2 |
| `net/minecraft/resources/Identifier#tryBuild` | method | 12 |
| `net/minecraft/resources/ResourceKey#identifier` | method | 1 |
| `net/minecraft/server/level/ServerLevel#getEntities` | method | 2 |
| `net/minecraft/server/packs/resources/Resource#open` | method | 2 |
| `net/minecraft/server/packs/resources/ResourceManager#getResource` | method | 2 |
| `net/minecraft/server/packs/resources/SimplePreparableReloadListener#<init>` | method | 1 |
| `net/minecraft/sounds/SoundSource#MASTER` | field | 1 |
| `net/minecraft/tags/BlockTags#SAND` | field | 1 |
| `net/minecraft/tags/DamageTypeTags#IS_DROWNING` | field | 1 |
| `net/minecraft/tags/DamageTypeTags#IS_FALL` | field | 1 |
| `net/minecraft/tags/DamageTypeTags#IS_FIRE` | field | 1 |
| `net/minecraft/tags/DamageTypeTags#IS_FREEZING` | field | 1 |
| `net/minecraft/tags/DamageTypeTags#IS_LIGHTNING` | field | 1 |
| `net/minecraft/util/RandomSource#create` | method | 1 |
| `net/minecraft/world/damagesource/DamageSource#<init>` | method | 2 |
| `net/minecraft/world/damagesource/DamageSource#getDirectEntity` | method | 2 |
| `net/minecraft/world/damagesource/DamageSource#getEntity` | method | 1 |
| `net/minecraft/world/damagesource/DamageSource#getLocalizedDeathMessage` | method | 1 |
| `net/minecraft/world/damagesource/DamageSource#getMsgId` | method | 1 |
| `net/minecraft/world/damagesource/DamageSource#is` | method | 2 |
| `net/minecraft/world/damagesource/DamageSource#sourcePositionRaw` | method | 1 |
| `net/minecraft/world/damagesource/DamageSource#typeHolder` | method | 2 |
| `net/minecraft/world/damagesource/DamageTypes#CACTUS` | field | 1 |
| `net/minecraft/world/damagesource/DamageTypes#FLY_INTO_WALL` | field | 1 |
| `net/minecraft/world/damagesource/DamageTypes#ON_FIRE` | field | 1 |
| `net/minecraft/world/damagesource/DamageTypes#STARVE` | field | 1 |
| `net/minecraft/world/damagesource/DamageTypes#WITHER` | field | 1 |
| `net/minecraft/world/damagesource/DamageTypes#WITHER_SKULL` | field | 1 |
| `net/minecraft/world/entity/Entity#distanceToSqr` | method | 1 |
| `net/minecraft/world/entity/Entity#getId` | method | 2 |
| `net/minecraft/world/entity/Entity#wasEyeInWater` | method | 1 |
| `net/minecraft/world/entity/LivingEntity#getMainHandItem` | method | 1 |
| `net/minecraft/world/entity/LivingEntity#isAffectedByPotions` | method | 1 |
| `net/minecraft/world/entity/ai/targeting/TargetingConditions#forNonCombat` | method | 1 |
| `net/minecraft/world/entity/player/Player#actuallyHurt` | method | 1 |
| `net/minecraft/world/entity/player/Player#blockPosition` | method | 2 |
| `net/minecraft/world/entity/player/Player#distanceToSqr` | method | 1 |
| `net/minecraft/world/entity/player/Player#getBoundingBox` | method | 2 |
| `net/minecraft/world/entity/player/Player#getFoodData` | method | 1 |
| `net/minecraft/world/entity/player/Player#getHealth` | method | 4 |
| `net/minecraft/world/entity/player/Player#getMainHandItem` | method | 1 |
| `net/minecraft/world/entity/player/Player#getMaxHealth` | method | 2 |
| `net/minecraft/world/entity/player/Player#getOffhandItem` | method | 1 |
| `net/minecraft/world/entity/player/Player#getX` | method | 1 |
| `net/minecraft/world/entity/player/Player#getXRot` | method | 1 |
| `net/minecraft/world/entity/player/Player#getY` | method | 1 |
| `net/minecraft/world/entity/player/Player#getZ` | method | 1 |
| `net/minecraft/world/entity/player/Player#isAlive` | method | 8 |
| `net/minecraft/world/entity/player/Player#isSprinting` | method | 1 |
| `net/minecraft/world/entity/player/Player#level` | method | 6 |
| `net/minecraft/world/entity/player/Player#registryAccess` | method | 1 |
| `net/minecraft/world/entity/player/Player#sendSystemMessage` | method | 1 |
| `net/minecraft/world/entity/projectile/Projectile#level` | method | 1 |
| `net/minecraft/world/entity/projectile/Projectile#onHit` | method | 1 |
| `net/minecraft/world/entity/projectile/hurtingprojectile/windcharge/WindCharge#getOwner` | method | 1 |
| `net/minecraft/world/entity/projectile/throwableitemprojectile/ThrownSplashPotion#distanceToSqr` | method | 1 |
| `net/minecraft/world/entity/projectile/throwableitemprojectile/ThrownSplashPotion#getBoundingBox` | method | 1 |
| `net/minecraft/world/entity/projectile/throwableitemprojectile/ThrownSplashPotion#getItem` | method | 1 |
| `net/minecraft/world/entity/projectile/throwableitemprojectile/ThrownSplashPotion#level` | method | 1 |
| `net/minecraft/world/food/FoodData#getFoodLevel` | method | 1 |
| `net/minecraft/world/item/ItemStack#getComponents` | method | 1 |
| `net/minecraft/world/item/ItemStack#getEnchantments` | method | 1 |
| `net/minecraft/world/item/ItemStack#getItem` | method | 2 |
| `net/minecraft/world/item/ItemStack#isEmpty` | method | 1 |
| `net/minecraft/world/item/Items#ARROW` | field | 1 |
| `net/minecraft/world/item/Items#DIAMOND_AXE` | field | 1 |
| `net/minecraft/world/item/Items#DIAMOND_HOE` | field | 1 |
| `net/minecraft/world/item/Items#DIAMOND_PICKAXE` | field | 1 |
| `net/minecraft/world/item/Items#DIAMOND_SHOVEL` | field | 1 |
| `net/minecraft/world/item/Items#DIAMOND_SWORD` | field | 1 |
| `net/minecraft/world/item/Items#GOLDEN_AXE` | field | 1 |
| `net/minecraft/world/item/Items#GOLDEN_HOE` | field | 1 |
| `net/minecraft/world/item/Items#GOLDEN_PICKAXE` | field | 1 |
| `net/minecraft/world/item/Items#GOLDEN_SHOVEL` | field | 1 |
| `net/minecraft/world/item/Items#GOLDEN_SWORD` | field | 1 |
| `net/minecraft/world/item/Items#IRON_AXE` | field | 1 |
| `net/minecraft/world/item/Items#IRON_HOE` | field | 1 |
| `net/minecraft/world/item/Items#IRON_PICKAXE` | field | 1 |
| `net/minecraft/world/item/Items#IRON_SHOVEL` | field | 1 |
| `net/minecraft/world/item/Items#IRON_SWORD` | field | 1 |
| `net/minecraft/world/item/Items#MACE` | field | 1 |
| `net/minecraft/world/item/Items#STONE_AXE` | field | 1 |
| `net/minecraft/world/item/Items#STONE_HOE` | field | 1 |
| `net/minecraft/world/item/Items#STONE_PICKAXE` | field | 1 |
| `net/minecraft/world/item/Items#STONE_SHOVEL` | field | 1 |
| `net/minecraft/world/item/Items#STONE_SWORD` | field | 1 |
| `net/minecraft/world/item/Items#WOODEN_AXE` | field | 1 |
| `net/minecraft/world/item/Items#WOODEN_HOE` | field | 1 |
| `net/minecraft/world/item/Items#WOODEN_PICKAXE` | field | 1 |
| `net/minecraft/world/item/Items#WOODEN_SHOVEL` | field | 1 |
| `net/minecraft/world/item/Items#WOODEN_SWORD` | field | 1 |
| `net/minecraft/world/item/alchemy/PotionContents#getColor` | method | 1 |
| `net/minecraft/world/item/enchantment/Enchantments#WIND_BURST` | field | 1 |
| `net/minecraft/world/item/enchantment/ItemEnchantments#getLevel` | method | 1 |
| `net/minecraft/world/level/Explosion#center` | method | 1 |
| `net/minecraft/world/level/Explosion#getBlockInteraction` | method | 1 |
| `net/minecraft/world/level/Explosion#getDirectSourceEntity` | method | 1 |
| `net/minecraft/world/level/Explosion#radius` | method | 1 |
| `net/minecraft/world/level/Explosion$BlockInteraction#TRIGGER_BLOCK` | field | 1 |
| `net/minecraft/world/level/Level#getBlockState` | method | 1 |
| `net/minecraft/world/level/Level#getEntitiesOfClass` | method | 1 |
| `net/minecraft/world/level/Level#getEntity` | method | 2 |
| `net/minecraft/world/level/ServerExplosion#hurtEntities` | method | 2 |
| `net/minecraft/world/level/block/state/BlockState#is` | method | 1 |
| `net/minecraft/world/phys/AABB#inflate` | method | 2 |
| `net/minecraft/world/phys/AABB#maxY` | field | 1 |

</details>

