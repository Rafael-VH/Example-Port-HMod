# Horizon port report: ItemPhysicLite

`itemphysiclite` 1.6.13 · ItemPhysicLite_FABRIC_v1.6.13_mc26.3.jar · 12 classes

**Not convertible yet**

| Status | Count | Meaning |
|---|---|---|
| auto | 0 | The Horizon hook or injection is known: the wiring can be generated. The Java body still has to become Rust. |
| content | 0 | Data and assets: horizon-port-pack carries these into a content mod (.hmod). |
| needs-mapping | 6 | The kind of change carries over, but the matching Horizon function isn't in the table yet. Run `match`. |
| needs-translation | 2 | No direct equivalent: a person (or an AI agent) has to rewrite this part. |
| unsupported | 0 | Horizon has nothing this can attach to yet. |

## Entry points

- `client`: team.creative.itemphysiclite.ItemPhysicLite

## Mixins

| Injection | In | Horizon | Status | Note |
|---|---|---|---|---|
| @Accessor on Entity.stuckSpeedMultiplier | `EntityAccessor.getStuckSpeedMultiplier` | GET | needs-mapping | A [[bridge]] GET (SET for a setter) on the Rust struct's field. No Horizon function known for Entity.stuckSpeedMultiplier: run `match`. |
| mixin into ItemEntity with no injectors (interfaces or @Shadow only) | `ItemEntityMixin` | – | needs-translation | Usually adds an interface or fields to a game class: needs a manual change. |
| mixin into ItemEntityRenderState with no injectors (interfaces or @Shadow only) | `ItemEntityRenderStateMixin` | – | needs-translation | Usually adds an interface or fields to a game class: needs a manual change. |
| @Inject(HEAD) on ItemEntityRenderer.submit | `ItemEntityRendererMixin.submit` | HEAD | needs-mapping | No Horizon function known for ItemEntityRenderer.submit: run `match`. |
| @Inject(TAIL) on ItemEntityRenderer.extractRenderState | `ItemEntityRendererMixin.injectExtract` | RETURN | needs-mapping | Horizon has no TAIL: RETURN runs at every exit, early ones too. No Horizon function known for ItemEntityRenderer.extractRenderState: run `match`. |
| @Invoker on ItemStackRenderState.callFirstLayer | `ItemStackRenderStateAccessor.callFirstLayer` | INVOKE | needs-mapping | A [[bridge]] INVOKE of the Rust method. No Horizon function known for ItemStackRenderState.callFirstLayer: run `match`. |
| @Accessor on LayerRenderState.itemTransform | `LayerRenderStateAccessor.getItemTransform` | GET | needs-mapping | A [[bridge]] GET (SET for a setter) on the Rust struct's field. No Horizon function known for LayerRenderState.itemTransform: run `match`. |
| @Accessor on RenderType.name | `RenderTypeAccessor.getName` | GET | needs-mapping | A [[bridge]] GET (SET for a setter) on the Rust struct's field. No Horizon function known for RenderType.name: run `match`. |

## Game methods and fields used

3 of 57 are mapped to Horizon. Every Java body that uses an unmapped one needs it mapped before its Rust can be written.

<details><summary>Mapped</summary>

| Java | Horizon |
|---|---|
| `net/minecraft/client/Minecraft#getInstance` | `self` |
| `net/minecraft/client/renderer/item/ItemStackRenderState#submit` | horizon_client::item_render::ItemRenderState::submit (suggested) |
| `net/minecraft/world/level/Level#isClientSide` | horizon_world::level::is_client_side (suggested) |

</details>

<details><summary>Not mapped yet</summary>

| Java | Kind | Classes using it |
|---|---|---|
| `com/mojang/blaze3d/vertex/PoseStack#popPose` | method | 1 |
| `com/mojang/blaze3d/vertex/PoseStack#pushPose` | method | 1 |
| `com/mojang/blaze3d/vertex/PoseStack#rotate` | method | 1 |
| `com/mojang/blaze3d/vertex/PoseStack#translate` | method | 2 |
| `net/minecraft/client/DeltaTracker#getRealtimeDeltaTicks` | method | 1 |
| `net/minecraft/client/Minecraft#getDeltaTracker` | method | 1 |
| `net/minecraft/client/Minecraft#isPaused` | method | 1 |
| `net/minecraft/client/Minecraft#options` | field | 1 |
| `net/minecraft/client/renderer/entity/EntityRenderer#<init>` | method | 1 |
| `net/minecraft/client/renderer/entity/EntityRenderer#submit` | method | 1 |
| `net/minecraft/client/renderer/entity/ItemEntityRenderer#extractRenderState` | method | 1 |
| `net/minecraft/client/renderer/entity/ItemEntityRenderer#submit` | method | 1 |
| `net/minecraft/client/renderer/entity/state/ItemEntityRenderState#ageInTicks` | field | 1 |
| `net/minecraft/client/renderer/entity/state/ItemEntityRenderState#bobOffset` | field | 1 |
| `net/minecraft/client/renderer/entity/state/ItemEntityRenderState#count` | field | 1 |
| `net/minecraft/client/renderer/entity/state/ItemEntityRenderState#item` | field | 2 |
| `net/minecraft/client/renderer/entity/state/ItemEntityRenderState#lightCoords` | field | 1 |
| `net/minecraft/client/renderer/entity/state/ItemEntityRenderState#outlineColor` | field | 1 |
| `net/minecraft/client/renderer/entity/state/ItemEntityRenderState#seed` | field | 1 |
| `net/minecraft/client/renderer/item/ItemStackRenderState#callFirstLayer` | method | 1 |
| `net/minecraft/client/renderer/item/ItemStackRenderState#usesBlockLight` | method | 1 |
| `net/minecraft/client/renderer/item/ItemStackRenderState$LayerRenderState#itemTransform` | method | 1 |
| `net/minecraft/client/renderer/rendertype/RenderType#name` | method | 1 |
| `net/minecraft/client/renderer/texture/OverlayTexture#NO_OVERLAY` | field | 1 |
| `net/minecraft/client/resources/model/cuboid/ItemTransform#scale` | method | 1 |
| `net/minecraft/core/BlockPos#below` | method | 2 |
| `net/minecraft/core/BlockPos#getY` | method | 1 |
| `net/minecraft/util/RandomSource#nextFloat` | method | 1 |
| `net/minecraft/util/RandomSource#setSeed` | method | 1 |
| `net/minecraft/world/entity/Entity#<init>` | method | 1 |
| `net/minecraft/world/entity/Entity#onSyncedDataUpdated` | method | 1 |
| `net/minecraft/world/entity/Entity#stuckSpeedMultiplier` | method | 1 |
| `net/minecraft/world/entity/item/ItemEntity#blockPosition` | method | 2 |
| `net/minecraft/world/entity/item/ItemEntity#getItem` | method | 1 |
| `net/minecraft/world/entity/item/ItemEntity#getX` | method | 1 |
| `net/minecraft/world/entity/item/ItemEntity#getXRot` | method | 2 |
| `net/minecraft/world/entity/item/ItemEntity#getY` | method | 1 |
| `net/minecraft/world/entity/item/ItemEntity#getYRot` | method | 1 |
| `net/minecraft/world/entity/item/ItemEntity#getZ` | method | 1 |
| `net/minecraft/world/entity/item/ItemEntity#level` | method | 3 |
| `net/minecraft/world/entity/item/ItemEntity#onGround` | method | 1 |
| `net/minecraft/world/entity/item/ItemEntity#position` | method | 1 |
| `net/minecraft/world/entity/item/ItemEntity#setXRot` | method | 1 |
| `net/minecraft/world/level/Level#getBlockState` | method | 1 |
| `net/minecraft/world/level/Level#getFluidState` | method | 1 |
| `net/minecraft/world/level/block/Blocks#MUD` | field | 1 |
| `net/minecraft/world/level/block/Blocks#SNOW` | field | 1 |
| `net/minecraft/world/level/block/Blocks#SOUL_SAND` | field | 1 |
| `net/minecraft/world/level/material/Fluid#getTickDelay` | method | 1 |
| `net/minecraft/world/level/material/FluidState#getHeight` | method | 1 |
| `net/minecraft/world/level/material/FluidState#getType` | method | 1 |
| `net/minecraft/world/phys/Vec3#lengthSqr` | method | 1 |
| `net/minecraft/world/phys/Vec3#x` | field | 1 |
| `net/minecraft/world/phys/Vec3#y` | field | 1 |

</details>

