# Java translated to Rust

Translated 1 of 16 methods and 5 of 7 static fields to Rust. It's in `runtime.rs`, under `pub mod java`; code that calls the game is in `game_side.rs`.

Each method is translated whole or not at all. A method that can't be (it uses Minecraft, a lambda, null...) is left out with its reason, and so is every method that calls it.

## Why methods were left out

| Cause | Methods | For example |
|---|---|---|
| Other Java this translator doesn't handle yet | 7 | `ItemEntityExtender.vanillaRendered(0)`: it has no body |
| Uses Minecraft classes (needs Horizon's game API) | 6 | `ClientPhysic.calculateRotation(2)`: it uses Minecraft's `ItemEntity` |
| Calls or uses other code that isn't translated | 2 | `ItemPhysicLite.ItemPhysicLite(0)`: its class isn't translated (it implements `ClientLoader` (team.creative.creativecore.client)) |

## Every class


### `ClientPhysic`

- [x] `ClientPhysic.baseMultiplier` (team/creative/itemphysiclite/ClientPhysic.java:11)
- [ ] `ClientPhysic.MC`: it uses Minecraft's `Minecraft` (team/creative/itemphysiclite/ClientPhysic.java:12)
- [ ] `ClientPhysic.calculateRotation(2)`: it uses Minecraft's `ItemEntity` (team/creative/itemphysiclite/ClientPhysic.java:14)

### `ItemEntityExtender`: it's an interface

- [ ] `ItemEntityExtender.vanillaRendered(0)`: it has no body (team/creative/itemphysiclite/ItemEntityExtender.java:4)

### `ItemEntityRenderStateExtender`: it's an interface

- [ ] `ItemEntityRenderStateExtender.isBlock(0)`: it has no body (team/creative/itemphysiclite/ItemEntityRenderStateExtender.java:6)
- [ ] `ItemEntityRenderStateExtender.getXRot(0)`: it has no body (team/creative/itemphysiclite/ItemEntityRenderStateExtender.java:8)
- [ ] `ItemEntityRenderStateExtender.getYRot(0)`: it has no body (team/creative/itemphysiclite/ItemEntityRenderStateExtender.java:10)
- [ ] `ItemEntityRenderStateExtender.skipRendering(0)`: it has no body (team/creative/itemphysiclite/ItemEntityRenderStateExtender.java:12)
- [ ] `ItemEntityRenderStateExtender.hasAdditionalOffset(0)`: it has no body (team/creative/itemphysiclite/ItemEntityRenderStateExtender.java:14)
- [ ] `ItemEntityRenderStateExtender.extractPhysic(1)`: it has no body (team/creative/itemphysiclite/ItemEntityRenderStateExtender.java:16)

### `ItemPhysicLite`: it implements `ClientLoader` (team.creative.creativecore.client)

- [x] `ItemPhysicLite.LOGGER` (team/creative/itemphysiclite/ItemPhysicLite.java:31)
- [x] `ItemPhysicLite.MODID` (team/creative/itemphysiclite/ItemPhysicLite.java:32)
- [ ] `ItemPhysicLite.CONFIG`: it uses ItemPhysicLiteConfig, which isn't translated (it uses `SortingBlockList` (team.creative.creativecore.common.util.type.list)) (team/creative/itemphysiclite/ItemPhysicLite.java:33)
- [x] `ItemPhysicLite.lastTickTime` (team/creative/itemphysiclite/ItemPhysicLite.java:34)
- [x] `ItemPhysicLite.RANDOM_Y_OFFSET_SCALE` (team/creative/itemphysiclite/ItemPhysicLite.java:35)
- [ ] `ItemPhysicLite.submit(5)`: it uses Minecraft's `ItemEntityRenderState` (team/creative/itemphysiclite/ItemPhysicLite.java:37)
- [x] `ItemPhysicLite.getModelCount(1)` (team/creative/itemphysiclite/ItemPhysicLite.java:101)
- [ ] `ItemPhysicLite.calculateFluid(1)`: it uses Minecraft's `Fluid` (team/creative/itemphysiclite/ItemPhysicLite.java:113)
- [ ] `ItemPhysicLite.calculateFluid(2)`: it uses Minecraft's `Fluid` (team/creative/itemphysiclite/ItemPhysicLite.java:117)
- [ ] `ItemPhysicLite.getStuckSpeedMultiplier(1)`: it uses Minecraft's `Entity` (team/creative/itemphysiclite/ItemPhysicLite.java:142)
- [ ] `ItemPhysicLite.getViscosity(2)`: it uses Minecraft's `Fluid` (team/creative/itemphysiclite/ItemPhysicLite.java:146)
- [ ] `ItemPhysicLite.ItemPhysicLite(0)`: its class isn't translated (it implements `ClientLoader` (team.creative.creativecore.client)) (team/creative/itemphysiclite/ItemPhysicLite.java:150)
- [ ] `ItemPhysicLite.onInitializeClient(0)`: its class isn't translated (it implements `ClientLoader` (team.creative.creativecore.client)) (team/creative/itemphysiclite/ItemPhysicLite.java:155)

### `ItemPhysicLiteConfig`: it uses `SortingBlockList` (team.creative.creativecore.common.util.type.list)

- [ ] `EntityAccessor`: a Mixin class: its handlers are the injections, in the bundle's helpers (team/creative/itemphysiclite/ItemPhysicLiteConfig.java:9)
- [ ] `ItemEntityMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (team/creative/itemphysiclite/ItemPhysicLiteConfig.java:13)
- [ ] `ItemEntityRendererMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (team/creative/itemphysiclite/ItemPhysicLiteConfig.java:22)
- [ ] `ItemEntityRenderStateMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (team/creative/itemphysiclite/ItemPhysicLiteConfig.java:13)
- [ ] `ItemStackRenderStateAccessor`: a Mixin class: its handlers are the injections, in the bundle's helpers (team/creative/itemphysiclite/ItemPhysicLiteConfig.java:9)
- [ ] `LayerRenderStateAccessor`: a Mixin class: its handlers are the injections, in the bundle's helpers (team/creative/itemphysiclite/ItemPhysicLiteConfig.java:9)
- [ ] `RenderTypeAccessor`: a Mixin class: its handlers are the injections, in the bundle's helpers (team/creative/itemphysiclite/ItemPhysicLiteConfig.java:8)
