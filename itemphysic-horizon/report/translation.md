# Java translated to Rust

Translated 2 of 53 methods and 6 of 11 static fields to Rust. It's in `runtime.rs`, under `pub mod java`; code that calls the game is in `game_side.rs`.

Each method is translated whole or not at all. A method that can't be (it uses Minecraft, a lambda, null...) is left out with its reason, and so is every method that calls it.

## Why methods were left out

| Cause | Methods | For example |
|---|---|---|
| Calls or uses other code that isn't translated | 17 | `ItemPhysicClient.gameTick(0)`: it uses ItemPhysic.CONFIG, which isn't translated (it uses ItemPhysicConfig, which isn't translated (it uses General, which isn't translated (it uses `SortingList` (team.creative.creativecore.common.util.type.list)))) |
| Uses Minecraft classes (needs Horizon's game API) | 17 | `ClientPhysic.calculateRotation(2)`: it uses Minecraft's `ItemEntity` |
| Other Java this translator doesn't handle yet | 13 | `ItemPhysicAPI.addSortingObjects(2)`: it uses `Object` |
| Uses null | 2 | `ItemPhysicClient.getChargeStage(0)`: a nullable object kept in a changing static |
| Uses other libraries | 2 | `ItemPhysicClient.initEarly(0)`: it calls `CreativeCoreClient` (team.creative.creativecore.client).loader |

## Every class


### `ItemPhysicAPI`

- [ ] `ItemPhysicAPI.addSortingObjects(2)`: it uses `Object` (team/creative/itemphysic/api/ItemPhysicAPI.java:6)

### `ClientPhysic`

- [x] `ClientPhysic.baseMultiplier` (team/creative/itemphysic/client/ClientPhysic.java:13)
- [ ] `ClientPhysic.MC`: it uses Minecraft's `Minecraft` (team/creative/itemphysic/client/ClientPhysic.java:14)
- [ ] `ClientPhysic.calculateRotation(2)`: it uses Minecraft's `ItemEntity` (team/creative/itemphysic/client/ClientPhysic.java:16)

### `ItemEntityRendering`: it's an interface

- [ ] `ItemEntityRendering.skipRendering(0)`: it has no body (team/creative/itemphysic/client/ItemEntityRendering.java:4)

### `ItemEntityRenderStateExtender`: it's an interface

- [ ] `ItemEntityRenderStateExtender.isBlock(0)`: it has no body (team/creative/itemphysic/client/ItemEntityRenderStateExtender.java:6)
- [ ] `ItemEntityRenderStateExtender.getXRot(0)`: it has no body (team/creative/itemphysic/client/ItemEntityRenderStateExtender.java:8)
- [ ] `ItemEntityRenderStateExtender.getYRot(0)`: it has no body (team/creative/itemphysic/client/ItemEntityRenderStateExtender.java:10)
- [ ] `ItemEntityRenderStateExtender.skipRendering(0)`: it has no body (team/creative/itemphysic/client/ItemEntityRenderStateExtender.java:12)
- [ ] `ItemEntityRenderStateExtender.hasAdditionalOffset(0)`: it has no body (team/creative/itemphysic/client/ItemEntityRenderStateExtender.java:14)
- [ ] `ItemEntityRenderStateExtender.extractPhysic(1)`: it has no body (team/creative/itemphysic/client/ItemEntityRenderStateExtender.java:16)

### `ItemPhysicClient`

- [ ] `ItemPhysicClient.PICKUP`: it uses Minecraft's `KeyMapping` (team/creative/itemphysic/client/ItemPhysicClient.java:51)
- [x] `ItemPhysicClient.throwCharge` (team/creative/itemphysic/client/ItemPhysicClient.java:52)
- [x] `ItemPhysicClient.RANDOM_Y_OFFSET_SCALE` (team/creative/itemphysic/client/ItemPhysicClient.java:53)
- [x] `ItemPhysicClient.LOADED_EARLY` (team/creative/itemphysic/client/ItemPhysicClient.java:54)
- [ ] `ItemPhysicClient.initEarly(0)`: it calls `CreativeCoreClient` (team.creative.creativecore.client).loader (team/creative/itemphysic/client/ItemPhysicClient.java:56)
- [ ] `ItemPhysicClient.init(0)`: it uses `ICreativeLoader` (team.creative.creativecore) (team/creative/itemphysic/client/ItemPhysicClient.java:63)
- [ ] `ItemPhysicClient.getChargeStage(0)`: a nullable object kept in a changing static (team/creative/itemphysic/client/ItemPhysicClient.java:70)
- [ ] `ItemPhysicClient.gameTick(0)`: it uses ItemPhysic.CONFIG, which isn't translated (it uses ItemPhysicConfig, which isn't translated (it uses General, which isn't translated (it uses `SortingList` (team.creative.creativecore.common.util.type.list)))) (team/creative/itemphysic/client/ItemPhysicClient.java:74)
- [ ] `ItemPhysicClient.renderTick(1)`: it uses `Object` (team/creative/itemphysic/client/ItemPhysicClient.java:99)
- [ ] `ItemPhysicClient.renderTooltip(1)`: `null` (Rust has no null: this needs an Option) (team/creative/itemphysic/client/ItemPhysicClient.java:105)
- [ ] `ItemPhysicClient.submit(5)`: it uses Minecraft's `ItemEntityRenderState` (team/creative/itemphysic/client/ItemPhysicClient.java:173)
- [x] `ItemPhysicClient.getModelCount(1)` (team/creative/itemphysic/client/ItemPhysicClient.java:237)
- [ ] `ItemPhysicClient.onPlayerInteractClient(3)`: it uses Minecraft's `Level` (team/creative/itemphysic/client/ItemPhysicClient.java:249)
- [ ] `ItemPhysicClient.onPlayerInteract(1)`: it uses ItemPhysic.CONFIG, which isn't translated (it uses ItemPhysicConfig, which isn't translated (it uses General, which isn't translated (it uses `SortingList` (team.creative.creativecore.common.util.type.list)))) (team/creative/itemphysic/client/ItemPhysicClient.java:264)
- [ ] `ItemPhysicClient.getItemInFocus(1)`: it uses Minecraft's `HitResult` (team/creative/itemphysic/client/ItemPhysicClient.java:272)
- [ ] `ItemPhysicClient.pick(7)`: it uses Minecraft's `HitResult` (team/creative/itemphysic/client/ItemPhysicClient.java:288)
- [ ] `ItemPhysicClient.filterHitResult(3)`: it uses Minecraft's `HitResult` (team/creative/itemphysic/client/ItemPhysicClient.java:307)

### `CommonPhysic`

- [ ] `CommonPhysic.getViscosity(2)`: it uses Minecraft's `Fluid` (team/creative/itemphysic/common/CommonPhysic.java:25)
- [ ] `CommonPhysic.fireImmune(1)`: it uses Minecraft's `ItemEntity` (team/creative/itemphysic/common/CommonPhysic.java:29)
- [ ] `CommonPhysic.update(1)`: it uses Minecraft's `ItemEntity` (team/creative/itemphysic/common/CommonPhysic.java:33)
- [ ] `CommonPhysic.updatePre(2)`: it uses Minecraft's `ItemEntity` (team/creative/itemphysic/common/CommonPhysic.java:56)
- [ ] `CommonPhysic.calculateFluid(1)`: it uses Minecraft's `Fluid` (team/creative/itemphysic/common/CommonPhysic.java:79)
- [ ] `CommonPhysic.calculateFluid(2)`: it uses Minecraft's `Fluid` (team/creative/itemphysic/common/CommonPhysic.java:83)
- [ ] `CommonPhysic.getReachDistance(1)`: it uses ItemPhysic.CONFIG, which isn't translated (it uses ItemPhysicConfig, which isn't translated (it uses General, which isn't translated (it uses `SortingList` (team.creative.creativecore.common.util.type.list)))) (team/creative/itemphysic/common/CommonPhysic.java:108)
- [ ] `CommonPhysic.getItemInFocus(3)`: it uses Minecraft's `HitResult` (team/creative/itemphysic/common/CommonPhysic.java:112)

### `ItemEntityExtender`: it's an interface

- [ ] `ItemEntityExtender.getFluid(0)`: it has no body (team/creative/itemphysic/common/ItemEntityExtender.java:6)
- [ ] `ItemEntityExtender.setFluid(1)`: it has no body (team/creative/itemphysic/common/ItemEntityExtender.java:8)
- [ ] `ItemEntityExtender.canSwim(0)`: it has no body (team/creative/itemphysic/common/ItemEntityExtender.java:10)
- [ ] `ItemEntityExtender.canBurn(0)`: it has no body (team/creative/itemphysic/common/ItemEntityExtender.java:12)

### `DropPacket`: it extends `CreativePacket` (team.creative.creativecore.common.network)

- [ ] `DropPacket.DropPacket(0)`: its class isn't translated (it extends `CreativePacket` (team.creative.creativecore.common.network)) (team/creative/itemphysic/common/packet/DropPacket.java:21)
- [ ] `DropPacket.DropPacket(2)`: its class isn't translated (it extends `CreativePacket` (team.creative.creativecore.common.network)) (team/creative/itemphysic/common/packet/DropPacket.java:24)
- [ ] `DropPacket.executeClient(1)`: its class isn't translated (it extends `CreativePacket` (team.creative.creativecore.common.network)) (team/creative/itemphysic/common/packet/DropPacket.java:29)
- [ ] `DropPacket.createEntity(4)`: its class isn't translated (it extends `CreativePacket` (team.creative.creativecore.common.network)) (team/creative/itemphysic/common/packet/DropPacket.java:32)
- [ ] `DropPacket.executeServer(1)`: its class isn't translated (it extends `CreativePacket` (team.creative.creativecore.common.network)) (team/creative/itemphysic/common/packet/DropPacket.java:73)

### `PickupPacket`: it extends `CreativePacket` (team.creative.creativecore.common.network)

- [ ] `PickupPacket.PickupPacket(0)`: its class isn't translated (it extends `CreativePacket` (team.creative.creativecore.common.network)) (team/creative/itemphysic/common/packet/PickupPacket.java:17)
- [ ] `PickupPacket.PickupPacket(2)`: its class isn't translated (it extends `CreativePacket` (team.creative.creativecore.common.network)) (team/creative/itemphysic/common/packet/PickupPacket.java:20)
- [ ] `PickupPacket.executeClient(1)`: its class isn't translated (it extends `CreativePacket` (team.creative.creativecore.common.network)) (team/creative/itemphysic/common/packet/PickupPacket.java:25)
- [ ] `PickupPacket.executeServer(1)`: its class isn't translated (it extends `CreativePacket` (team.creative.creativecore.common.network)) (team/creative/itemphysic/common/packet/PickupPacket.java:28)

### `ItemPhysic`: it implements `ClientLoader` (team.creative.creativecore.client)

- [x] `ItemPhysic.LOGGER` (team/creative/itemphysic/ItemPhysic.java:18)
- [x] `ItemPhysic.MODID` (team/creative/itemphysic/ItemPhysic.java:19)
- [ ] `ItemPhysic.NETWORK`: it uses `CreativeNetwork` (team.creative.creativecore.common.network) (team/creative/itemphysic/ItemPhysic.java:20)
- [ ] `ItemPhysic.CONFIG`: it uses ItemPhysicConfig, which isn't translated (it uses General, which isn't translated (it uses `SortingList` (team.creative.creativecore.common.util.type.list))) (team/creative/itemphysic/ItemPhysic.java:21)
- [ ] `ItemPhysic.ItemPhysic(0)`: its class isn't translated (it implements `ClientLoader` (team.creative.creativecore.client)) (team/creative/itemphysic/ItemPhysic.java:23)
- [ ] `ItemPhysic.onInitialize(0)`: its class isn't translated (it implements `ClientLoader` (team.creative.creativecore.client)) (team/creative/itemphysic/ItemPhysic.java:32)
- [ ] `ItemPhysic.onInitializeClient(0)`: its class isn't translated (it implements `ClientLoader` (team.creative.creativecore.client)) (team/creative/itemphysic/ItemPhysic.java:39)

### `ItemPhysicConfig`: it uses General, which isn't translated (it uses `SortingList` (team.creative.creativecore.common.util.type.list))


### `ItemPhysicConfig.General`: it uses `SortingList` (team.creative.creativecore.common.util.type.list)

- [ ] `ItemPhysicConfig.General.General(0)`: its class isn't translated (it uses `SortingList` (team.creative.creativecore.common.util.type.list)) (team/creative/itemphysic/ItemPhysicConfig.java:40)

### `ItemPhysicConfig.Pickup`: it uses `SortingList` (team.creative.creativecore.common.util.type.list)

- [ ] `ItemPhysicConfig.Pickup.canPickup(1)`: its class isn't translated (it uses `SortingList` (team.creative.creativecore.common.util.type.list)) (team/creative/itemphysic/ItemPhysicConfig.java:218)

### `ItemPhysicConfig.Rendering`: it uses `SortingList` (team.creative.creativecore.common.util.type.list)


### `ItemPhysicConfig.Throw`

- [ ] `EntityAccessor`: a Mixin class: its handlers are the injections, in the bundle's helpers (team/creative/itemphysic/ItemPhysicConfig.java:12)
- [ ] `ItemEntityAccessor`: a Mixin class: its handlers are the injections, in the bundle's helpers (team/creative/itemphysic/ItemPhysicConfig.java:10)
- [ ] `ItemEntityMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (team/creative/itemphysic/ItemPhysicConfig.java:35)
- [ ] `ItemEntityMixinClient`: a Mixin class: its handlers are the injections, in the bundle's helpers (team/creative/itemphysic/ItemPhysicConfig.java:15)
- [ ] `ItemEntityRendererMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (team/creative/itemphysic/ItemPhysicConfig.java:22)
- [ ] `ItemEntityRenderStateMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (team/creative/itemphysic/ItemPhysicConfig.java:13)
- [ ] `ItemStackRenderStateAccessor`: a Mixin class: its handlers are the injections, in the bundle's helpers (team/creative/itemphysic/ItemPhysicConfig.java:9)
- [ ] `LayerRenderStateAccessor`: a Mixin class: its handlers are the injections, in the bundle's helpers (team/creative/itemphysic/ItemPhysicConfig.java:9)
- [ ] `MinecraftMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (team/creative/itemphysic/ItemPhysicConfig.java:13)
- [ ] `MultiPlayerGameModeMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (team/creative/itemphysic/ItemPhysicConfig.java:12)
- [ ] `RenderTypeAccessor`: a Mixin class: its handlers are the injections, in the bundle's helpers (team/creative/itemphysic/ItemPhysicConfig.java:8)

### `ItemPhysicServer`

- [ ] `ItemPhysicServer.fluid`: it uses `ThreadLocal` (team/creative/itemphysic/server/ItemPhysicServer.java:23)
- [x] `ItemPhysicServer.init(0)` (team/creative/itemphysic/server/ItemPhysicServer.java:25)
- [ ] `ItemPhysicServer.playerTouch(2)`: it uses Minecraft's `ItemEntity` (team/creative/itemphysic/server/ItemPhysicServer.java:28)
- [ ] `ItemPhysicServer.playerPickup(2)`: it uses Minecraft's `ItemEntity` (team/creative/itemphysic/server/ItemPhysicServer.java:44)
- [ ] `ItemPhysicServer.interact(3)`: it uses Minecraft's `InteractionResult` (team/creative/itemphysic/server/ItemPhysicServer.java:93)
- [ ] `ItemPhysicServer.hurt(3)`: it uses Minecraft's `ItemEntity` (team/creative/itemphysic/server/ItemPhysicServer.java:102)
