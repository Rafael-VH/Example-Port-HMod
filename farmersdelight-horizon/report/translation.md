# Java translated to Rust

Translated 37 of 1723 methods (1 more is done another way in Horizon) and 85 of 1262 static fields to Rust (3 of those call the game, through Horizon's helpers); 0 of 1 Fabric event handlers. It's in `runtime.rs`, under `pub mod java`; code that calls the game is in `game_side.rs`.

Each method is translated whole or not at all. A method that can't be (it uses Minecraft, a lambda, null...) is left out with its reason, and so is every method that calls it.

## Why methods were left out

| Cause | Methods | For example |
|---|---|---|
| Calls or uses other code that isn't translated | 1352 | `ClientSetupEvents.onRegisterRenderers(0)`: it uses ModEntityTypes.ROTTEN_TOMATO, which isn't translated (it uses Minecraft's `EntityType`) |
| Uses Minecraft classes (needs Horizon's game API) | 195 | `ClientSetupEvents.registerCustomTooltipRenderers(1)`: it uses Minecraft's `ClientTooltipComponent` |
| Other Java this translator doesn't handle yet | 118 | `HUDOverlays.drawComfortOverlay(5)`: its body couldn't be read: an assert |
| Uses Fabric API | 14 | `ClientSetupEvents.registerParticles(0)`: it calls Fabric's `ParticleProviderRegistry`.getInstance |
| Uses other libraries | 4 | `LaunchpadClientEvents.register(1)`: it uses `RegisterGuiLayersEvent` (net.neoforged.neoforge.client.event) |
| Calls the game where Horizon has no helper yet | 1 | `Basket.canMergeItems(2)`: it calls Minecraft's ItemStack.getMaxStackSize (no Horizon helper for it yet) |
| Done another way in Horizon | 1 | `ModMenuCompat.getModConfigScreenFactory(0)`: done another way in Horizon: the mod's page in Options > Mods (its settings buttons) |
| Uses translated text (Component.translatable) | 1 | `DecompositionCategory.translateKey(1)`: a translated text with a computed key |

## Fabric event handlers

| Handler | Hook | Result |
|---|---|---|
| vectorwing/farmersdelight/client/FarmersDelightClient.java:23 | `init` | not translated: it calls ClientSetupEvents.onRegisterRenderers, which isn't translated |

## Every class


### `vectorwing/farmersdelight/common/item/package-info.java`: couldn't be read: expected a class at `package` (line 2)


### `vectorwing/farmersdelight/integration/jei/category/package-info.java`: couldn't be read: expected a class at `package` (line 2)


### `vectorwing/farmersdelight/integration/jei/resource/package-info.java`: couldn't be read: expected a class at `package` (line 2)


### `ClientSetupEvents`

- [x] `ClientSetupEvents.registerRecipeBookCategories(0)` (vectorwing/farmersdelight/client/event/ClientSetupEvents.java:25)
- [ ] `ClientSetupEvents.registerCustomTooltipRenderers(1)`: it uses Minecraft's `ClientTooltipComponent` (vectorwing/farmersdelight/client/event/ClientSetupEvents.java:28)
- [x] `ClientSetupEvents.registerGuiLayers(0)` (vectorwing/farmersdelight/client/event/ClientSetupEvents.java:34)
- [ ] `ClientSetupEvents.onRegisterRenderers(0)`: it uses ModEntityTypes.ROTTEN_TOMATO, which isn't translated (it uses Minecraft's `EntityType`) (vectorwing/farmersdelight/client/event/ClientSetupEvents.java:37)
- [ ] `ClientSetupEvents.registerMenuScreens(0)`: it uses ModMenuTypes.COOKING_POT, which isn't translated (it uses Minecraft's `MenuType`) (vectorwing/farmersdelight/client/event/ClientSetupEvents.java:44)
- [ ] `ClientSetupEvents.registerParticles(0)`: it calls Fabric's `ParticleProviderRegistry`.getInstance (vectorwing/farmersdelight/client/event/ClientSetupEvents.java:48)

### `TooltipEvents`

- [ ] `TooltipEvents.addTooltipToVanillaSoups(4)`: it uses Minecraft's `TooltipContext` (vectorwing/farmersdelight/client/event/TooltipEvents.java:22)

### `FarmersDelightClient`: it implements Fabric's `ClientModInitializer`

- [x] `FarmersDelightClient.naturalRegenerationEnabled` (vectorwing/farmersdelight/client/FarmersDelightClient.java:21)
- [ ] `FarmersDelightClient.onInitializeClient(0)`: its class isn't translated (it implements Fabric's `ClientModInitializer`) (vectorwing/farmersdelight/client/FarmersDelightClient.java:23)

### `CookingPotRecipeBookComponent`: it extends Minecraft's `RecipeBookComponent`

- [ ] `CookingPotRecipeBookComponent.COOKING_SEARCH_CATEGORY`: it uses Minecraft's `SearchRecipeBookCategory` (vectorwing/farmersdelight/client/gui/CookingPotRecipeBookComponent.java:26)
- [ ] `CookingPotRecipeBookComponent.RECIPE_BOOK_BUTTONS`: it uses Minecraft's `WidgetSprites` (vectorwing/farmersdelight/client/gui/CookingPotRecipeBookComponent.java:27)
- [ ] `CookingPotRecipeBookComponent.TABS`: it uses Minecraft's `TabInfo` (vectorwing/farmersdelight/client/gui/CookingPotRecipeBookComponent.java:33)
- [ ] `CookingPotRecipeBookComponent.CookingPotRecipeBookComponent(1)`: its class isn't translated (it extends Minecraft's `RecipeBookComponent`) (vectorwing/farmersdelight/client/gui/CookingPotRecipeBookComponent.java:40)
- [ ] `CookingPotRecipeBookComponent.getFilterButtonTextures(0)`: its class isn't translated (it extends Minecraft's `RecipeBookComponent`) (vectorwing/farmersdelight/client/gui/CookingPotRecipeBookComponent.java:44)
- [ ] `CookingPotRecipeBookComponent.isCraftingSlot(1)`: its class isn't translated (it extends Minecraft's `RecipeBookComponent`) (vectorwing/farmersdelight/client/gui/CookingPotRecipeBookComponent.java:48)
- [ ] `CookingPotRecipeBookComponent.selectMatchingRecipes(2)`: its class isn't translated (it extends Minecraft's `RecipeBookComponent`) (vectorwing/farmersdelight/client/gui/CookingPotRecipeBookComponent.java:55)
- [ ] `CookingPotRecipeBookComponent.getRecipeFilterName(0)`: its class isn't translated (it extends Minecraft's `RecipeBookComponent`) (vectorwing/farmersdelight/client/gui/CookingPotRecipeBookComponent.java:59)
- [ ] `CookingPotRecipeBookComponent.fillGhostRecipe(3)`: its class isn't translated (it extends Minecraft's `RecipeBookComponent`) (vectorwing/farmersdelight/client/gui/CookingPotRecipeBookComponent.java:63)

### `CookingPotScreen`: it extends Minecraft's `AbstractRecipeBookScreen`

- [x] `CookingPotScreen.BACKGROUND_TEXTURE` (vectorwing/farmersdelight/client/gui/CookingPotScreen.java:25)
- [ ] `CookingPotScreen.HEAT_ICON`: it uses `Rectangle` (java.awt) (vectorwing/farmersdelight/client/gui/CookingPotScreen.java:26)
- [ ] `CookingPotScreen.PROGRESS_ARROW`: it uses `Rectangle` (java.awt) (vectorwing/farmersdelight/client/gui/CookingPotScreen.java:27)
- [ ] `CookingPotScreen.CookingPotScreen(3)`: its class isn't translated (it extends Minecraft's `AbstractRecipeBookScreen`) (vectorwing/farmersdelight/client/gui/CookingPotScreen.java:29)
- [ ] `CookingPotScreen.init(0)`: its class isn't translated (it extends Minecraft's `AbstractRecipeBookScreen`) (vectorwing/farmersdelight/client/gui/CookingPotScreen.java:33)
- [ ] `CookingPotScreen.getRecipeBookButtonPosition(0)`: its class isn't translated (it extends Minecraft's `AbstractRecipeBookScreen`) (vectorwing/farmersdelight/client/gui/CookingPotScreen.java:45)
- [ ] `CookingPotScreen.renderHeatIndicatorTooltip(3)`: its class isn't translated (it extends Minecraft's `AbstractRecipeBookScreen`) (vectorwing/farmersdelight/client/gui/CookingPotScreen.java:49)
- [ ] `CookingPotScreen.extractTooltip(3)`: its class isn't translated (it extends Minecraft's `AbstractRecipeBookScreen`) (vectorwing/farmersdelight/client/gui/CookingPotScreen.java:56)
- [ ] `CookingPotScreen.renderMealDisplayTooltip(3)`: its class isn't translated (it extends Minecraft's `AbstractRecipeBookScreen`) (vectorwing/farmersdelight/client/gui/CookingPotScreen.java:63)
- [ ] `CookingPotScreen.extractBackground(4)`: its class isn't translated (it extends Minecraft's `AbstractRecipeBookScreen`) (vectorwing/farmersdelight/client/gui/CookingPotScreen.java:86)

### `CookingPotTooltip`: it implements Minecraft's `ClientTooltipComponent`

- [x] `CookingPotTooltip.ITEM_SIZE` (vectorwing/farmersdelight/client/gui/CookingPotTooltip.java:15)
- [x] `CookingPotTooltip.MARGIN` (vectorwing/farmersdelight/client/gui/CookingPotTooltip.java:16)
- [ ] `CookingPotTooltip.CookingPotTooltip(1)`: its class isn't translated (it implements Minecraft's `ClientTooltipComponent`) (vectorwing/farmersdelight/client/gui/CookingPotTooltip.java:20)
- [ ] `CookingPotTooltip.getHeight(1)`: its class isn't translated (it implements Minecraft's `ClientTooltipComponent`) (vectorwing/farmersdelight/client/gui/CookingPotTooltip.java:24)
- [ ] `CookingPotTooltip.getWidth(1)`: its class isn't translated (it implements Minecraft's `ClientTooltipComponent`) (vectorwing/farmersdelight/client/gui/CookingPotTooltip.java:28)
- [ ] `CookingPotTooltip.extractImage(6)`: its class isn't translated (it implements Minecraft's `ClientTooltipComponent`) (vectorwing/farmersdelight/client/gui/CookingPotTooltip.java:39)
- [ ] `CookingPotTooltip.extractText(4)`: its class isn't translated (it implements Minecraft's `ClientTooltipComponent`) (vectorwing/farmersdelight/client/gui/CookingPotTooltip.java:45)

### `CookingPotTooltip.CookingPotTooltipComponent`: it implements Minecraft's `TooltipComponent`

- [ ] `CookingPotTooltip.CookingPotTooltipComponent.CookingPotTooltipComponent(1)`: its class isn't translated (it implements Minecraft's `TooltipComponent`) (vectorwing/farmersdelight/client/gui/CookingPotTooltip.java:59)
- [ ] `CookingPotTooltip.CookingPotTooltipComponent.mealStack(0)`: its class isn't translated (it implements Minecraft's `TooltipComponent`) (vectorwing/farmersdelight/client/gui/CookingPotTooltip.java:59)

### `HUDOverlays`

- [x] `HUDOverlays.healthIconsOffset` (vectorwing/farmersdelight/client/gui/HUDOverlays.java:21)
- [x] `HUDOverlays.foodIconsOffset` (vectorwing/farmersdelight/client/gui/HUDOverlays.java:22)
- [x] `HUDOverlays.MOD_ICONS_TEXTURE` (vectorwing/farmersdelight/client/gui/HUDOverlays.java:23)
- [x] `HUDOverlays.register(0)` (vectorwing/farmersdelight/client/gui/HUDOverlays.java:25)
- [ ] `HUDOverlays.drawNourishmentOverlay(6)`: it uses Minecraft's `FoodData` (vectorwing/farmersdelight/client/gui/HUDOverlays.java:29)
- [ ] `HUDOverlays.drawComfortOverlay(5)`: its body couldn't be read: an assert (vectorwing/farmersdelight/client/gui/HUDOverlays.java:54)

### `HUDOverlays.BaseOverlay`

- [ ] `HUDOverlays.BaseOverlay.render(7)`: it has no body (vectorwing/farmersdelight/client/gui/HUDOverlays.java:99)
- [ ] `HUDOverlays.BaseOverlay.render(2)`: it uses Minecraft's `GuiGraphicsExtractor` (vectorwing/farmersdelight/client/gui/HUDOverlays.java:101)
- [ ] `HUDOverlays.BaseOverlay.shouldRenderOverlay(4)`: it uses Minecraft's `Minecraft` (vectorwing/farmersdelight/client/gui/HUDOverlays.java:111)

### `HUDOverlays.ComfortOverlay`: it extends `BaseOverlay`

- [x] `HUDOverlays.ComfortOverlay.ID` (vectorwing/farmersdelight/client/gui/HUDOverlays.java:117)
- [ ] `HUDOverlays.ComfortOverlay.INSTANCE`: it uses ComfortOverlay, which isn't translated (it extends `BaseOverlay`) (vectorwing/farmersdelight/client/gui/HUDOverlays.java:118)
- [ ] `HUDOverlays.ComfortOverlay.render(7)`: its class isn't translated (it extends `BaseOverlay`) (vectorwing/farmersdelight/client/gui/HUDOverlays.java:120)
- [ ] `HUDOverlays.ComfortOverlay.shouldRenderOverlay(4)`: its class isn't translated (it extends `BaseOverlay`) (vectorwing/farmersdelight/client/gui/HUDOverlays.java:129)

### `HUDOverlays.NourishmentOverlay`: it extends `BaseOverlay`

- [x] `HUDOverlays.NourishmentOverlay.ID` (vectorwing/farmersdelight/client/gui/HUDOverlays.java:136)
- [ ] `HUDOverlays.NourishmentOverlay.INSTANCE`: it uses NourishmentOverlay, which isn't translated (it extends `BaseOverlay`) (vectorwing/farmersdelight/client/gui/HUDOverlays.java:137)
- [ ] `HUDOverlays.NourishmentOverlay.render(7)`: its class isn't translated (it extends `BaseOverlay`) (vectorwing/farmersdelight/client/gui/HUDOverlays.java:139)
- [ ] `HUDOverlays.NourishmentOverlay.shouldRenderOverlay(4)`: its class isn't translated (it extends `BaseOverlay`) (vectorwing/farmersdelight/client/gui/HUDOverlays.java:148)

### `SkilletCookingConditionalItemModelProperty`: it implements Minecraft's `ConditionalItemModelProperty`

- [ ] `SkilletCookingConditionalItemModelProperty.MAP_CODEC`: it uses Minecraft's `MapCodec` (vectorwing/farmersdelight/client/model/SkilletCookingConditionalItemModelProperty.java:13)
- [ ] `SkilletCookingConditionalItemModelProperty.get(5)`: its class isn't translated (it implements Minecraft's `ConditionalItemModelProperty`) (vectorwing/farmersdelight/client/model/SkilletCookingConditionalItemModelProperty.java:15)
- [ ] `SkilletCookingConditionalItemModelProperty.type(0)`: its class isn't translated (it implements Minecraft's `ConditionalItemModelProperty`) (vectorwing/farmersdelight/client/model/SkilletCookingConditionalItemModelProperty.java:19)

### `SparkleParticle`: it extends Minecraft's `SingleQuadParticle`

- [ ] `SparkleParticle.SparkleParticle(6)`: its class isn't translated (it extends Minecraft's `SingleQuadParticle`) (vectorwing/farmersdelight/client/particle/SparkleParticle.java:17)
- [ ] `SparkleParticle.getLightCoords(1)`: its class isn't translated (it extends Minecraft's `SingleQuadParticle`) (vectorwing/farmersdelight/client/particle/SparkleParticle.java:25)
- [ ] `SparkleParticle.tick(0)`: its class isn't translated (it extends Minecraft's `SingleQuadParticle`) (vectorwing/farmersdelight/client/particle/SparkleParticle.java:29)
- [ ] `SparkleParticle.getLayer(0)`: its class isn't translated (it extends Minecraft's `SingleQuadParticle`) (vectorwing/farmersdelight/client/particle/SparkleParticle.java:40)

### `SparkleParticle.Factory`: it implements Minecraft's `ParticleProvider`

- [ ] `SparkleParticle.Factory.Factory(1)`: its class isn't translated (it implements Minecraft's `ParticleProvider`) (vectorwing/farmersdelight/client/particle/SparkleParticle.java:47)
- [ ] `SparkleParticle.Factory.createParticle(9)`: its class isn't translated (it implements Minecraft's `ParticleProvider`) (vectorwing/farmersdelight/client/particle/SparkleParticle.java:51)

### `StarParticle`: it extends Minecraft's `SingleQuadParticle`

- [ ] `StarParticle.StarParticle(5)`: its class isn't translated (it extends Minecraft's `SingleQuadParticle`) (vectorwing/farmersdelight/client/particle/StarParticle.java:15)
- [ ] `StarParticle.getLayer(0)`: its class isn't translated (it extends Minecraft's `SingleQuadParticle`) (vectorwing/farmersdelight/client/particle/StarParticle.java:26)
- [ ] `StarParticle.getQuadSize(1)`: its class isn't translated (it extends Minecraft's `SingleQuadParticle`) (vectorwing/farmersdelight/client/particle/StarParticle.java:30)
- [ ] `StarParticle.tick(0)`: its class isn't translated (it extends Minecraft's `SingleQuadParticle`) (vectorwing/farmersdelight/client/particle/StarParticle.java:34)

### `StarParticle.Factory`: it implements Minecraft's `ParticleProvider`

- [ ] `StarParticle.Factory.Factory(1)`: its class isn't translated (it implements Minecraft's `ParticleProvider`) (vectorwing/farmersdelight/client/particle/StarParticle.java:60)
- [ ] `StarParticle.Factory.createParticle(9)`: its class isn't translated (it implements Minecraft's `ParticleProvider`) (vectorwing/farmersdelight/client/particle/StarParticle.java:64)

### `SteamParticle`: it extends Minecraft's `SingleQuadParticle`

- [ ] `SteamParticle.SteamParticle(8)`: its class isn't translated (it extends Minecraft's `SingleQuadParticle`) (vectorwing/farmersdelight/client/particle/SteamParticle.java:14)
- [ ] `SteamParticle.tick(0)`: its class isn't translated (it extends Minecraft's `SingleQuadParticle`) (vectorwing/farmersdelight/client/particle/SteamParticle.java:27)
- [ ] `SteamParticle.getLayer(0)`: its class isn't translated (it extends Minecraft's `SingleQuadParticle`) (vectorwing/farmersdelight/client/particle/SteamParticle.java:44)

### `SteamParticle.Factory`: it implements Minecraft's `ParticleProvider`

- [ ] `SteamParticle.Factory.Factory(1)`: its class isn't translated (it implements Minecraft's `ParticleProvider`) (vectorwing/farmersdelight/client/particle/SteamParticle.java:51)
- [ ] `SteamParticle.Factory.createParticle(9)`: its class isn't translated (it implements Minecraft's `ParticleProvider`) (vectorwing/farmersdelight/client/particle/SteamParticle.java:55)

### `CookingPotRecipeDisplay`: it implements Minecraft's `RecipeDisplay`

- [ ] `CookingPotRecipeDisplay.MAP_CODEC`: it uses Minecraft's `MapCodec` (vectorwing/farmersdelight/client/recipe/CookingPotRecipeDisplay.java:19)
- [ ] `CookingPotRecipeDisplay.STREAM_CODEC`: it uses Minecraft's `StreamCodec` (vectorwing/farmersdelight/client/recipe/CookingPotRecipeDisplay.java:30)
- [ ] `CookingPotRecipeDisplay.TYPE`: it uses Minecraft's `Type` (vectorwing/farmersdelight/client/recipe/CookingPotRecipeDisplay.java:45)
- [ ] `CookingPotRecipeDisplay.CookingPotRecipeDisplay(6)`: its class isn't translated (it implements Minecraft's `RecipeDisplay`) (vectorwing/farmersdelight/client/recipe/CookingPotRecipeDisplay.java:16)
- [ ] `CookingPotRecipeDisplay.type(0)`: its class isn't translated (it implements Minecraft's `RecipeDisplay`) (vectorwing/farmersdelight/client/recipe/CookingPotRecipeDisplay.java:47)
- [ ] `CookingPotRecipeDisplay.isEnabled(1)`: its class isn't translated (it implements Minecraft's `RecipeDisplay`) (vectorwing/farmersdelight/client/recipe/CookingPotRecipeDisplay.java:51)
- [ ] `CookingPotRecipeDisplay.ingredients(0)`: its class isn't translated (it implements Minecraft's `RecipeDisplay`) (vectorwing/farmersdelight/client/recipe/CookingPotRecipeDisplay.java:16)
- [ ] `CookingPotRecipeDisplay.container(0)`: its class isn't translated (it implements Minecraft's `RecipeDisplay`) (vectorwing/farmersdelight/client/recipe/CookingPotRecipeDisplay.java:16)
- [ ] `CookingPotRecipeDisplay.result(0)`: its class isn't translated (it implements Minecraft's `RecipeDisplay`) (vectorwing/farmersdelight/client/recipe/CookingPotRecipeDisplay.java:16)
- [ ] `CookingPotRecipeDisplay.craftingStation(0)`: its class isn't translated (it implements Minecraft's `RecipeDisplay`) (vectorwing/farmersdelight/client/recipe/CookingPotRecipeDisplay.java:16)
- [ ] `CookingPotRecipeDisplay.duration(0)`: its class isn't translated (it implements Minecraft's `RecipeDisplay`) (vectorwing/farmersdelight/client/recipe/CookingPotRecipeDisplay.java:16)
- [ ] `CookingPotRecipeDisplay.experience(0)`: its class isn't translated (it implements Minecraft's `RecipeDisplay`) (vectorwing/farmersdelight/client/recipe/CookingPotRecipeDisplay.java:16)

### `AbstractStoveRenderer`: it's generic

- [x] `AbstractStoveRenderer.SIZE` (vectorwing/farmersdelight/client/renderer/AbstractStoveRenderer.java:27)
- [ ] `AbstractStoveRenderer.AbstractStoveRenderer(1)`: its class isn't translated (it's generic) (vectorwing/farmersdelight/client/renderer/AbstractStoveRenderer.java:30)
- [ ] `AbstractStoveRenderer.extractRenderState(5)`: its class isn't translated (it's generic) (vectorwing/farmersdelight/client/renderer/AbstractStoveRenderer.java:34)
- [ ] `AbstractStoveRenderer.submit(4)`: its class isn't translated (it's generic) (vectorwing/farmersdelight/client/renderer/AbstractStoveRenderer.java:52)
- [ ] `AbstractStoveRenderer.getStoveItemOffset(1)`: it has no body (vectorwing/farmersdelight/client/renderer/AbstractStoveRenderer.java:73)

### `CuttingBoardRenderer`: it implements Minecraft's `BlockEntityRenderer`

- [ ] `CuttingBoardRenderer.CuttingBoardRenderer(1)`: its class isn't translated (it implements Minecraft's `BlockEntityRenderer`) (vectorwing/farmersdelight/client/renderer/CuttingBoardRenderer.java:31)
- [ ] `CuttingBoardRenderer.createRenderState(0)`: its class isn't translated (it implements Minecraft's `BlockEntityRenderer`) (vectorwing/farmersdelight/client/renderer/CuttingBoardRenderer.java:35)
- [ ] `CuttingBoardRenderer.extractRenderState(5)`: its class isn't translated (it implements Minecraft's `BlockEntityRenderer`) (vectorwing/farmersdelight/client/renderer/CuttingBoardRenderer.java:39)
- [ ] `CuttingBoardRenderer.submit(4)`: its class isn't translated (it implements Minecraft's `BlockEntityRenderer`) (vectorwing/farmersdelight/client/renderer/CuttingBoardRenderer.java:58)
- [ ] `CuttingBoardRenderer.renderItemLayingDown(5)`: its class isn't translated (it implements Minecraft's `BlockEntityRenderer`) (vectorwing/farmersdelight/client/renderer/CuttingBoardRenderer.java:85)
- [ ] `CuttingBoardRenderer.renderBlock(5)`: its class isn't translated (it implements Minecraft's `BlockEntityRenderer`) (vectorwing/farmersdelight/client/renderer/CuttingBoardRenderer.java:93)
- [ ] `CuttingBoardRenderer.renderItemCarved(3)`: its class isn't translated (it implements Minecraft's `BlockEntityRenderer`) (vectorwing/farmersdelight/client/renderer/CuttingBoardRenderer.java:100)
- [ ] `CuttingBoardRenderer.getModelCount(1)`: its class isn't translated (it implements Minecraft's `BlockEntityRenderer`) (vectorwing/farmersdelight/client/renderer/CuttingBoardRenderer.java:118)

### `DefaultStoveRenderer`: it's generic

- [ ] `DefaultStoveRenderer.DefaultStoveRenderer(1)`: its class isn't translated (it's generic) (vectorwing/farmersdelight/client/renderer/DefaultStoveRenderer.java:9)
- [ ] `DefaultStoveRenderer.createRenderState(0)`: its class isn't translated (it's generic) (vectorwing/farmersdelight/client/renderer/DefaultStoveRenderer.java:13)
- [ ] `DefaultStoveRenderer.getStoveItemOffset(1)`: its class isn't translated (it's generic) (vectorwing/farmersdelight/client/renderer/DefaultStoveRenderer.java:17)

### `SkilletFlipItemRenderer`: it implements Minecraft's `ItemModel`

- [x] `SkilletFlipItemRenderer.ID` (vectorwing/farmersdelight/client/renderer/SkilletFlipItemRenderer.java:39)
- [ ] `SkilletFlipItemRenderer.SPECIAL_RENDERER`: it uses SpecialRenderer, which isn't translated (it implements Minecraft's `SpecialModelRenderer`) (vectorwing/farmersdelight/client/renderer/SkilletFlipItemRenderer.java:40)
- [ ] `SkilletFlipItemRenderer.SkilletFlipItemRenderer(2)`: its class isn't translated (it implements Minecraft's `ItemModel`) (vectorwing/farmersdelight/client/renderer/SkilletFlipItemRenderer.java:44)
- [ ] `SkilletFlipItemRenderer.update(7)`: its class isn't translated (it implements Minecraft's `ItemModel`) (vectorwing/farmersdelight/client/renderer/SkilletFlipItemRenderer.java:49)
- [ ] `SkilletFlipItemRenderer.applyTransform(4)`: it uses Minecraft's `ClientLevel` (vectorwing/farmersdelight/client/renderer/SkilletFlipItemRenderer.java:81)

### `SkilletFlipItemRenderer.Argument`: it uses Minecraft's `ItemStackRenderState`


### `SkilletFlipItemRenderer.SpecialRenderer`: it implements Minecraft's `SpecialModelRenderer`

- [ ] `SkilletFlipItemRenderer.SpecialRenderer.submit(7)`: its class isn't translated (it implements Minecraft's `SpecialModelRenderer`) (vectorwing/farmersdelight/client/renderer/SkilletFlipItemRenderer.java:104)
- [ ] `SkilletFlipItemRenderer.SpecialRenderer.getExtents(1)`: its class isn't translated (it implements Minecraft's `SpecialModelRenderer`) (vectorwing/farmersdelight/client/renderer/SkilletFlipItemRenderer.java:153)
- [ ] `SkilletFlipItemRenderer.SpecialRenderer.extractArgument(1)`: its class isn't translated (it implements Minecraft's `SpecialModelRenderer`) (vectorwing/farmersdelight/client/renderer/SkilletFlipItemRenderer.java:156)

### `SkilletFlipItemRenderer.Unbaked`: it implements `Unbaked`

- [ ] `SkilletFlipItemRenderer.Unbaked.CODEC`: it uses Minecraft's `MapCodec` (vectorwing/farmersdelight/client/renderer/SkilletFlipItemRenderer.java:166)
- [ ] `SkilletFlipItemRenderer.Unbaked.Unbaked(1)`: its class isn't translated (it implements `Unbaked`) (vectorwing/farmersdelight/client/renderer/SkilletFlipItemRenderer.java:164)
- [ ] `SkilletFlipItemRenderer.Unbaked.bake(2)`: its class isn't translated (it implements `Unbaked`) (vectorwing/farmersdelight/client/renderer/SkilletFlipItemRenderer.java:169)
- [ ] `SkilletFlipItemRenderer.Unbaked.resolveDependencies(1)`: its class isn't translated (it implements `Unbaked`) (vectorwing/farmersdelight/client/renderer/SkilletFlipItemRenderer.java:177)
- [ ] `SkilletFlipItemRenderer.Unbaked.type(0)`: its class isn't translated (it implements `Unbaked`) (vectorwing/farmersdelight/client/renderer/SkilletFlipItemRenderer.java:181)
- [ ] `SkilletFlipItemRenderer.Unbaked.model(0)`: its class isn't translated (it implements `Unbaked`) (vectorwing/farmersdelight/client/renderer/SkilletFlipItemRenderer.java:164)

### `SkilletRenderer`: it implements Minecraft's `BlockEntityRenderer`

- [ ] `SkilletRenderer.SkilletRenderer(1)`: its class isn't translated (it implements Minecraft's `BlockEntityRenderer`) (vectorwing/farmersdelight/client/renderer/SkilletRenderer.java:30)
- [ ] `SkilletRenderer.createRenderState(0)`: its class isn't translated (it implements Minecraft's `BlockEntityRenderer`) (vectorwing/farmersdelight/client/renderer/SkilletRenderer.java:34)
- [ ] `SkilletRenderer.extractRenderState(5)`: its class isn't translated (it implements Minecraft's `BlockEntityRenderer`) (vectorwing/farmersdelight/client/renderer/SkilletRenderer.java:38)
- [ ] `SkilletRenderer.submit(4)`: its class isn't translated (it implements Minecraft's `BlockEntityRenderer`) (vectorwing/farmersdelight/client/renderer/SkilletRenderer.java:54)
- [ ] `SkilletRenderer.getModelCount(1)`: its class isn't translated (it implements Minecraft's `BlockEntityRenderer`) (vectorwing/farmersdelight/client/renderer/SkilletRenderer.java:80)

### `CuttingBoardRenderState`: it extends Minecraft's `BlockEntityRenderState`


### `SkilletRenderState`: it extends Minecraft's `BlockEntityRenderState`


### `StoveRenderState`: it extends Minecraft's `BlockEntityRenderState`


### `CuttingBoardTrigger`: it extends Minecraft's `SimpleCriterionTrigger`

- [ ] `CuttingBoardTrigger.codec(0)`: its class isn't translated (it extends Minecraft's `SimpleCriterionTrigger`) (vectorwing/farmersdelight/common/advancement/CuttingBoardTrigger.java:15)
- [ ] `CuttingBoardTrigger.trigger(1)`: its class isn't translated (it extends Minecraft's `SimpleCriterionTrigger`) (vectorwing/farmersdelight/common/advancement/CuttingBoardTrigger.java:19)

### `CuttingBoardTrigger.TriggerInstance`: it implements Minecraft's `SimpleInstance`

- [ ] `CuttingBoardTrigger.TriggerInstance.CODEC`: it uses Minecraft's `Codec` (vectorwing/farmersdelight/common/advancement/CuttingBoardTrigger.java:24)
- [ ] `CuttingBoardTrigger.TriggerInstance.TriggerInstance(1)`: its class isn't translated (it implements Minecraft's `SimpleInstance`) (vectorwing/farmersdelight/common/advancement/CuttingBoardTrigger.java:23)
- [ ] `CuttingBoardTrigger.TriggerInstance.simple(0)`: it uses Minecraft's `Criterion` (vectorwing/farmersdelight/common/advancement/CuttingBoardTrigger.java:29)
- [ ] `CuttingBoardTrigger.TriggerInstance.test(0)`: its class isn't translated (it implements Minecraft's `SimpleInstance`) (vectorwing/farmersdelight/common/advancement/CuttingBoardTrigger.java:33)
- [ ] `CuttingBoardTrigger.TriggerInstance.player(0)`: its class isn't translated (it implements Minecraft's `SimpleInstance`) (vectorwing/farmersdelight/common/advancement/CuttingBoardTrigger.java:23)

### `BlockShapes`

- [ ] `BlockShapes.TRAY_OUTER_SHAPE`: it uses Minecraft's `VoxelShape` (vectorwing/farmersdelight/common/BlockShapes.java:9)
- [ ] `BlockShapes.TRAY_INNER_SHAPE`: it uses Minecraft's `VoxelShape` (vectorwing/farmersdelight/common/BlockShapes.java:10)
- [ ] `BlockShapes.TRAY_SHAPE`: it uses Minecraft's `VoxelShape` (vectorwing/farmersdelight/common/BlockShapes.java:11)
- [ ] `BlockShapes.ROAST_CHICKEN_SHAPES`: it uses Minecraft's `VoxelShape` (vectorwing/farmersdelight/common/BlockShapes.java:12)
- [ ] `BlockShapes.HONEY_GLAZED_HAM_SHAPES`: it uses Minecraft's `VoxelShape` (vectorwing/farmersdelight/common/BlockShapes.java:18)
- [ ] `BlockShapes.SHEPHERDS_PIE_SHAPES`: it uses Minecraft's `VoxelShape` (vectorwing/farmersdelight/common/BlockShapes.java:24)

### `AbstractStoveBlock`: it extends Minecraft's `BaseEntityBlock`

- [ ] `AbstractStoveBlock.FACING`: it uses Minecraft's `Property` (vectorwing/farmersdelight/common/block/AbstractStoveBlock.java:56)
- [ ] `AbstractStoveBlock.LIT`: it uses Minecraft's `BooleanProperty` (vectorwing/farmersdelight/common/block/AbstractStoveBlock.java:57)
- [ ] `AbstractStoveBlock.GRILLING_AREA`: it uses Minecraft's `VoxelShape` (vectorwing/farmersdelight/common/block/AbstractStoveBlock.java:58)
- [ ] `AbstractStoveBlock.AbstractStoveBlock(1)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (vectorwing/farmersdelight/common/block/AbstractStoveBlock.java:60)
- [ ] `AbstractStoveBlock.useItemOn(7)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (vectorwing/farmersdelight/common/block/AbstractStoveBlock.java:69)
- [ ] `AbstractStoveBlock.tryToIgnite(7)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (vectorwing/farmersdelight/common/block/AbstractStoveBlock.java:85)
- [ ] `AbstractStoveBlock.tryToExtinguish(7)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (vectorwing/farmersdelight/common/block/AbstractStoveBlock.java:115)
- [ ] `AbstractStoveBlock.tryToPlaceFoodItem(7)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (vectorwing/farmersdelight/common/block/AbstractStoveBlock.java:143)
- [ ] `AbstractStoveBlock.ignite(4)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (vectorwing/farmersdelight/common/block/AbstractStoveBlock.java:174)
- [ ] `AbstractStoveBlock.extinguish(4)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (vectorwing/farmersdelight/common/block/AbstractStoveBlock.java:186)
- [ ] `AbstractStoveBlock.getRenderShape(1)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (vectorwing/farmersdelight/common/block/AbstractStoveBlock.java:198)
- [ ] `AbstractStoveBlock.getStateForPlacement(1)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (vectorwing/farmersdelight/common/block/AbstractStoveBlock.java:202)
- [ ] `AbstractStoveBlock.stepOn(4)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (vectorwing/farmersdelight/common/block/AbstractStoveBlock.java:206)
- [ ] `AbstractStoveBlock.burnEntitySteppingOnStove(4)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (vectorwing/farmersdelight/common/block/AbstractStoveBlock.java:211)
- [ ] `AbstractStoveBlock.isStoveTopCovered(3)`: it uses Minecraft's `Level` (vectorwing/farmersdelight/common/block/AbstractStoveBlock.java:223)
- [ ] `AbstractStoveBlock.createBlockStateDefinition(1)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (vectorwing/farmersdelight/common/block/AbstractStoveBlock.java:233)
- [ ] `AbstractStoveBlock.createStoveTicker(2)`: it's generic (vectorwing/farmersdelight/common/block/AbstractStoveBlock.java:238)
- [ ] `AbstractStoveBlock.getBlockPathType(4)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (vectorwing/farmersdelight/common/block/AbstractStoveBlock.java:248)
- [ ] `AbstractStoveBlock.rotate(2)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (vectorwing/farmersdelight/common/block/AbstractStoveBlock.java:252)
- [ ] `AbstractStoveBlock.mirror(2)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (vectorwing/farmersdelight/common/block/AbstractStoveBlock.java:256)

### `BasketBlock`: it extends Minecraft's `BaseEntityBlock`

- [ ] `BasketBlock.FACING`: it uses Minecraft's `Property` (vectorwing/farmersdelight/common/block/BasketBlock.java:45)
- [ ] `BasketBlock.ENABLED`: it uses Minecraft's `BooleanProperty` (vectorwing/farmersdelight/common/block/BasketBlock.java:46)
- [ ] `BasketBlock.WATERLOGGED`: it uses Minecraft's `BooleanProperty` (vectorwing/farmersdelight/common/block/BasketBlock.java:47)
- [ ] `BasketBlock.OUT_SHAPE`: it uses Minecraft's `VoxelShape` (vectorwing/farmersdelight/common/block/BasketBlock.java:48)
- [ ] `BasketBlock.RENDER_SHAPE`: it uses Minecraft's `VoxelShape` (vectorwing/farmersdelight/common/block/BasketBlock.java:49)
- [ ] `BasketBlock.COLLISION_SHAPE_FACING`: it uses `ImmutableMap` (com.google.common.collect) (vectorwing/farmersdelight/common/block/BasketBlock.java:50)
- [ ] `BasketBlock.makeHollowCubeShape(1)`: it uses Minecraft's `VoxelShape` (vectorwing/farmersdelight/common/block/BasketBlock.java:61)
- [ ] `BasketBlock.BasketBlock(1)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (vectorwing/farmersdelight/common/block/BasketBlock.java:65)
- [ ] `BasketBlock.createBlockStateDefinition(1)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (vectorwing/farmersdelight/common/block/BasketBlock.java:72)
- [ ] `BasketBlock.useWithoutItem(5)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (vectorwing/farmersdelight/common/block/BasketBlock.java:76)
- [ ] `BasketBlock.updateShape(8)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (vectorwing/farmersdelight/common/block/BasketBlock.java:88)
- [ ] `BasketBlock.getFluidState(1)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (vectorwing/farmersdelight/common/block/BasketBlock.java:105)
- [ ] `BasketBlock.neighborChanged(6)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (vectorwing/farmersdelight/common/block/BasketBlock.java:109)
- [ ] `BasketBlock.hasAnalogOutputSignal(1)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (vectorwing/farmersdelight/common/block/BasketBlock.java:118)
- [ ] `BasketBlock.getAnalogOutputSignal(4)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (vectorwing/farmersdelight/common/block/BasketBlock.java:122)
- [ ] `BasketBlock.getStateForPlacement(1)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (vectorwing/farmersdelight/common/block/BasketBlock.java:126)
- [ ] `BasketBlock.useShapeForLightOcclusion(1)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (vectorwing/farmersdelight/common/block/BasketBlock.java:132)
- [ ] `BasketBlock.getShape(4)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (vectorwing/farmersdelight/common/block/BasketBlock.java:136)
- [ ] `BasketBlock.getOcclusionShape(1)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (vectorwing/farmersdelight/common/block/BasketBlock.java:140)
- [ ] `BasketBlock.getRenderShape(1)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (vectorwing/farmersdelight/common/block/BasketBlock.java:144)
- [ ] `BasketBlock.getInteractionShape(3)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (vectorwing/farmersdelight/common/block/BasketBlock.java:148)
- [ ] `BasketBlock.isPathfindable(2)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (vectorwing/farmersdelight/common/block/BasketBlock.java:152)
- [ ] `BasketBlock.newBlockEntity(2)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (vectorwing/farmersdelight/common/block/BasketBlock.java:156)
- [ ] `BasketBlock.getTicker(3)`: it's generic (vectorwing/farmersdelight/common/block/BasketBlock.java:160)
- [ ] `BasketBlock.rotate(2)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (vectorwing/farmersdelight/common/block/BasketBlock.java:166)
- [ ] `BasketBlock.mirror(2)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (vectorwing/farmersdelight/common/block/BasketBlock.java:170)

### `BuddingBushBlock`: it extends Minecraft's `VegetationBlock`

- [x] `BuddingBushBlock.MAX_AGE` (vectorwing/farmersdelight/common/block/BuddingBushBlock.java:29)
- [ ] `BuddingBushBlock.AGE`: it uses Minecraft's `IntegerProperty` (vectorwing/farmersdelight/common/block/BuddingBushBlock.java:30)
- [ ] `BuddingBushBlock.SHAPE_BY_AGE`: it uses Minecraft's `VoxelShape` (vectorwing/farmersdelight/common/block/BuddingBushBlock.java:31)
- [ ] `BuddingBushBlock.BuddingBushBlock(1)`: its class isn't translated (it extends Minecraft's `VegetationBlock`) (vectorwing/farmersdelight/common/block/BuddingBushBlock.java:39)
- [ ] `BuddingBushBlock.getShape(4)`: its class isn't translated (it extends Minecraft's `VegetationBlock`) (vectorwing/farmersdelight/common/block/BuddingBushBlock.java:43)
- [ ] `BuddingBushBlock.mayPlaceOn(3)`: its class isn't translated (it extends Minecraft's `VegetationBlock`) (vectorwing/farmersdelight/common/block/BuddingBushBlock.java:47)
- [ ] `BuddingBushBlock.getAgeProperty(0)`: its class isn't translated (it extends Minecraft's `VegetationBlock`) (vectorwing/farmersdelight/common/block/BuddingBushBlock.java:51)
- [ ] `BuddingBushBlock.getMaxAge(0)`: its class isn't translated (it extends Minecraft's `VegetationBlock`) (vectorwing/farmersdelight/common/block/BuddingBushBlock.java:55)
- [ ] `BuddingBushBlock.getAge(1)`: its class isn't translated (it extends Minecraft's `VegetationBlock`) (vectorwing/farmersdelight/common/block/BuddingBushBlock.java:59)
- [ ] `BuddingBushBlock.getStateForAge(1)`: its class isn't translated (it extends Minecraft's `VegetationBlock`) (vectorwing/farmersdelight/common/block/BuddingBushBlock.java:63)
- [ ] `BuddingBushBlock.isMaxAge(1)`: its class isn't translated (it extends Minecraft's `VegetationBlock`) (vectorwing/farmersdelight/common/block/BuddingBushBlock.java:67)
- [ ] `BuddingBushBlock.isRandomlyTicking(1)`: its class isn't translated (it extends Minecraft's `VegetationBlock`) (vectorwing/farmersdelight/common/block/BuddingBushBlock.java:71)
- [ ] `BuddingBushBlock.randomTick(4)`: its class isn't translated (it extends Minecraft's `VegetationBlock`) (vectorwing/farmersdelight/common/block/BuddingBushBlock.java:75)
- [ ] `BuddingBushBlock.canGrowPastMaxAge(0)`: its class isn't translated (it extends Minecraft's `VegetationBlock`) (vectorwing/farmersdelight/common/block/BuddingBushBlock.java:93)
- [ ] `BuddingBushBlock.growPastMaxAge(4)`: its class isn't translated (it extends Minecraft's `VegetationBlock`) (vectorwing/farmersdelight/common/block/BuddingBushBlock.java:97)
- [ ] `BuddingBushBlock.getGrowthSpeed(3)`: it uses Minecraft's `BlockState` (vectorwing/farmersdelight/common/block/BuddingBushBlock.java:100)
- [ ] `BuddingBushBlock.canSurvive(3)`: its class isn't translated (it extends Minecraft's `VegetationBlock`) (vectorwing/farmersdelight/common/block/BuddingBushBlock.java:143)
- [ ] `BuddingBushBlock.entityInside(6)`: its class isn't translated (it extends Minecraft's `VegetationBlock`) (vectorwing/farmersdelight/common/block/BuddingBushBlock.java:147)
- [ ] `BuddingBushBlock.getBaseSeedId(0)`: its class isn't translated (it extends Minecraft's `VegetationBlock`) (vectorwing/farmersdelight/common/block/BuddingBushBlock.java:155)
- [ ] `BuddingBushBlock.getCloneItemStack(4)`: its class isn't translated (it extends Minecraft's `VegetationBlock`) (vectorwing/farmersdelight/common/block/BuddingBushBlock.java:159)
- [ ] `BuddingBushBlock.createBlockStateDefinition(1)`: its class isn't translated (it extends Minecraft's `VegetationBlock`) (vectorwing/farmersdelight/common/block/BuddingBushBlock.java:163)

### `BuddingTomatoBlock`: it extends `BuddingBushBlock`

- [ ] `BuddingTomatoBlock.BuddingTomatoBlock(1)`: its class isn't translated (it extends `BuddingBushBlock`) (vectorwing/farmersdelight/common/block/BuddingTomatoBlock.java:19)
- [ ] `BuddingTomatoBlock.updateShape(8)`: its class isn't translated (it extends `BuddingBushBlock`) (vectorwing/farmersdelight/common/block/BuddingTomatoBlock.java:23)
- [ ] `BuddingTomatoBlock.canGrowPastMaxAge(0)`: its class isn't translated (it extends `BuddingBushBlock`) (vectorwing/farmersdelight/common/block/BuddingTomatoBlock.java:38)
- [ ] `BuddingTomatoBlock.growPastMaxAge(4)`: its class isn't translated (it extends `BuddingBushBlock`) (vectorwing/farmersdelight/common/block/BuddingTomatoBlock.java:43)
- [ ] `BuddingTomatoBlock.isValidBonemealTarget(4)`: its class isn't translated (it extends `BuddingBushBlock`) (vectorwing/farmersdelight/common/block/BuddingTomatoBlock.java:48)
- [ ] `BuddingTomatoBlock.isBonemealSuccess(5)`: its class isn't translated (it extends `BuddingBushBlock`) (vectorwing/farmersdelight/common/block/BuddingTomatoBlock.java:52)
- [ ] `BuddingTomatoBlock.getBonemealAgeIncrease(1)`: its class isn't translated (it extends `BuddingBushBlock`) (vectorwing/farmersdelight/common/block/BuddingTomatoBlock.java:56)
- [ ] `BuddingTomatoBlock.performBonemeal(5)`: its class isn't translated (it extends `BuddingBushBlock`) (vectorwing/farmersdelight/common/block/BuddingTomatoBlock.java:60)

### `CabbageBlock`: it extends Minecraft's `CropBlock`

- [ ] `CabbageBlock.SHAPE_BY_AGE`: it uses Minecraft's `VoxelShape` (vectorwing/farmersdelight/common/block/CabbageBlock.java:15)
- [ ] `CabbageBlock.CabbageBlock(1)`: its class isn't translated (it extends Minecraft's `CropBlock`) (vectorwing/farmersdelight/common/block/CabbageBlock.java:26)
- [ ] `CabbageBlock.getBaseSeedId(0)`: its class isn't translated (it extends Minecraft's `CropBlock`) (vectorwing/farmersdelight/common/block/CabbageBlock.java:30)
- [ ] `CabbageBlock.getShape(4)`: its class isn't translated (it extends Minecraft's `CropBlock`) (vectorwing/farmersdelight/common/block/CabbageBlock.java:34)

### `CabinetBlock`: it extends Minecraft's `BaseEntityBlock`

- [ ] `CabinetBlock.FACING`: it uses Minecraft's `Property` (vectorwing/farmersdelight/common/block/CabinetBlock.java:31)
- [ ] `CabinetBlock.OPEN`: it uses Minecraft's `BooleanProperty` (vectorwing/farmersdelight/common/block/CabinetBlock.java:32)
- [ ] `CabinetBlock.CabinetBlock(1)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (vectorwing/farmersdelight/common/block/CabinetBlock.java:34)
- [ ] `CabinetBlock.useWithoutItem(5)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (vectorwing/farmersdelight/common/block/CabinetBlock.java:39)
- [ ] `CabinetBlock.tick(4)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (vectorwing/farmersdelight/common/block/CabinetBlock.java:48)
- [ ] `CabinetBlock.getStateForPlacement(1)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (vectorwing/farmersdelight/common/block/CabinetBlock.java:54)
- [ ] `CabinetBlock.createBlockStateDefinition(1)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (vectorwing/farmersdelight/common/block/CabinetBlock.java:58)
- [ ] `CabinetBlock.hasAnalogOutputSignal(1)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (vectorwing/farmersdelight/common/block/CabinetBlock.java:63)
- [ ] `CabinetBlock.getAnalogOutputSignal(4)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (vectorwing/farmersdelight/common/block/CabinetBlock.java:67)
- [ ] `CabinetBlock.newBlockEntity(2)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (vectorwing/farmersdelight/common/block/CabinetBlock.java:71)
- [ ] `CabinetBlock.getRenderShape(1)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (vectorwing/farmersdelight/common/block/CabinetBlock.java:75)
- [ ] `CabinetBlock.rotate(2)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (vectorwing/farmersdelight/common/block/CabinetBlock.java:79)
- [ ] `CabinetBlock.mirror(2)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (vectorwing/farmersdelight/common/block/CabinetBlock.java:83)
- [ ] `CabinetBlock.affectNeighborsAfterRemoval(4)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (vectorwing/farmersdelight/common/block/CabinetBlock.java:87)

### `CanvasRugBlock`: it extends Minecraft's `Block`

- [ ] `CanvasRugBlock.SHAPE`: it uses Minecraft's `VoxelShape` (vectorwing/farmersdelight/common/block/CanvasRugBlock.java:12)
- [ ] `CanvasRugBlock.CanvasRugBlock(1)`: its class isn't translated (it extends Minecraft's `Block`) (vectorwing/farmersdelight/common/block/CanvasRugBlock.java:14)
- [ ] `CanvasRugBlock.getShape(4)`: its class isn't translated (it extends Minecraft's `Block`) (vectorwing/farmersdelight/common/block/CanvasRugBlock.java:18)
- [ ] `CanvasRugBlock.propagatesSkylightDown(1)`: its class isn't translated (it extends Minecraft's `Block`) (vectorwing/farmersdelight/common/block/CanvasRugBlock.java:22)

### `CeilingHangingCanvasSignBlock`: it extends Minecraft's `CeilingHangingSignBlock`

- [ ] `CeilingHangingCanvasSignBlock.CeilingHangingCanvasSignBlock(2)`: its class isn't translated (it extends Minecraft's `CeilingHangingSignBlock`) (vectorwing/farmersdelight/common/block/CeilingHangingCanvasSignBlock.java:13)
- [ ] `CeilingHangingCanvasSignBlock.getBackgroundColor(0)`: its class isn't translated (it extends Minecraft's `CeilingHangingSignBlock`) (vectorwing/farmersdelight/common/block/CeilingHangingCanvasSignBlock.java:18)

### `CookingPotBlock`: it extends Minecraft's `BaseEntityBlock`

- [ ] `CookingPotBlock.FACING`: it uses Minecraft's `EnumProperty` (vectorwing/farmersdelight/common/block/CookingPotBlock.java:53)
- [ ] `CookingPotBlock.SUPPORT`: it uses Minecraft's `EnumProperty` (vectorwing/farmersdelight/common/block/CookingPotBlock.java:54)
- [ ] `CookingPotBlock.WATERLOGGED`: it uses Minecraft's `BooleanProperty` (vectorwing/farmersdelight/common/block/CookingPotBlock.java:55)
- [ ] `CookingPotBlock.SHAPE`: it uses Minecraft's `VoxelShape` (vectorwing/farmersdelight/common/block/CookingPotBlock.java:56)
- [ ] `CookingPotBlock.SHAPE_WITH_TRAY`: it uses Minecraft's `VoxelShape` (vectorwing/farmersdelight/common/block/CookingPotBlock.java:57)
- [ ] `CookingPotBlock.CookingPotBlock(1)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (vectorwing/farmersdelight/common/block/CookingPotBlock.java:59)
- [ ] `CookingPotBlock.useItemOn(7)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (vectorwing/farmersdelight/common/block/CookingPotBlock.java:68)
- [ ] `CookingPotBlock.getRenderShape(1)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (vectorwing/farmersdelight/common/block/CookingPotBlock.java:100)
- [ ] `CookingPotBlock.getShape(4)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (vectorwing/farmersdelight/common/block/CookingPotBlock.java:104)
- [ ] `CookingPotBlock.getCollisionShape(4)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (vectorwing/farmersdelight/common/block/CookingPotBlock.java:108)
- [ ] `CookingPotBlock.getStateForPlacement(1)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (vectorwing/farmersdelight/common/block/CookingPotBlock.java:112)
- [ ] `CookingPotBlock.updateShape(8)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (vectorwing/farmersdelight/common/block/CookingPotBlock.java:123)
- [ ] `CookingPotBlock.affectNeighborsAfterRemoval(4)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (vectorwing/farmersdelight/common/block/CookingPotBlock.java:142)
- [ ] `CookingPotBlock.getTrayState(2)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (vectorwing/farmersdelight/common/block/CookingPotBlock.java:146)
- [ ] `CookingPotBlock.getCloneItemStack(4)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (vectorwing/farmersdelight/common/block/CookingPotBlock.java:150)
- [ ] `CookingPotBlock.createBlockStateDefinition(1)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (vectorwing/farmersdelight/common/block/CookingPotBlock.java:160)
- [ ] `CookingPotBlock.animateTick(4)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (vectorwing/farmersdelight/common/block/CookingPotBlock.java:165)
- [ ] `CookingPotBlock.hasAnalogOutputSignal(1)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (vectorwing/farmersdelight/common/block/CookingPotBlock.java:177)
- [ ] `CookingPotBlock.getAnalogOutputSignal(4)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (vectorwing/farmersdelight/common/block/CookingPotBlock.java:181)
- [ ] `CookingPotBlock.getFluidState(1)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (vectorwing/farmersdelight/common/block/CookingPotBlock.java:191)
- [ ] `CookingPotBlock.newBlockEntity(2)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (vectorwing/farmersdelight/common/block/CookingPotBlock.java:195)
- [ ] `CookingPotBlock.getTicker(3)`: it's generic (vectorwing/farmersdelight/common/block/CookingPotBlock.java:199)

### `CuttingBoardBlock`: it extends Minecraft's `BaseEntityBlock`

- [ ] `CuttingBoardBlock.FACING`: it uses Minecraft's `Property` (vectorwing/farmersdelight/common/block/CuttingBoardBlock.java:53)
- [ ] `CuttingBoardBlock.WATERLOGGED`: it uses Minecraft's `BooleanProperty` (vectorwing/farmersdelight/common/block/CuttingBoardBlock.java:54)
- [ ] `CuttingBoardBlock.SHAPE`: it uses Minecraft's `VoxelShape` (vectorwing/farmersdelight/common/block/CuttingBoardBlock.java:55)
- [ ] `CuttingBoardBlock.CuttingBoardBlock(1)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (vectorwing/farmersdelight/common/block/CuttingBoardBlock.java:57)
- [ ] `CuttingBoardBlock.init(0)`: it uses Fabric's `UseBlockCallback`.EVENT (vectorwing/farmersdelight/common/block/CuttingBoardBlock.java:64)
- [ ] `CuttingBoardBlock.useItemOn(7)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (vectorwing/farmersdelight/common/block/CuttingBoardBlock.java:68)
- [ ] `CuttingBoardBlock.affectNeighborsAfterRemoval(4)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (vectorwing/farmersdelight/common/block/CuttingBoardBlock.java:109)
- [ ] `CuttingBoardBlock.isPossibleToRespawnInThis(1)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (vectorwing/farmersdelight/common/block/CuttingBoardBlock.java:113)
- [ ] `CuttingBoardBlock.getStateForPlacement(1)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (vectorwing/farmersdelight/common/block/CuttingBoardBlock.java:117)
- [ ] `CuttingBoardBlock.updateShape(8)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (vectorwing/farmersdelight/common/block/CuttingBoardBlock.java:123)
- [ ] `CuttingBoardBlock.canSurvive(3)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (vectorwing/farmersdelight/common/block/CuttingBoardBlock.java:142)
- [ ] `CuttingBoardBlock.createBlockStateDefinition(1)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (vectorwing/farmersdelight/common/block/CuttingBoardBlock.java:147)
- [ ] `CuttingBoardBlock.getFluidState(1)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (vectorwing/farmersdelight/common/block/CuttingBoardBlock.java:152)
- [ ] `CuttingBoardBlock.hasAnalogOutputSignal(1)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (vectorwing/farmersdelight/common/block/CuttingBoardBlock.java:156)
- [ ] `CuttingBoardBlock.getAnalogOutputSignal(4)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (vectorwing/farmersdelight/common/block/CuttingBoardBlock.java:160)
- [ ] `CuttingBoardBlock.getRenderShape(1)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (vectorwing/farmersdelight/common/block/CuttingBoardBlock.java:174)
- [ ] `CuttingBoardBlock.getShape(4)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (vectorwing/farmersdelight/common/block/CuttingBoardBlock.java:178)
- [ ] `CuttingBoardBlock.newBlockEntity(2)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (vectorwing/farmersdelight/common/block/CuttingBoardBlock.java:182)
- [ ] `CuttingBoardBlock.rotate(2)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (vectorwing/farmersdelight/common/block/CuttingBoardBlock.java:186)
- [ ] `CuttingBoardBlock.mirror(2)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (vectorwing/farmersdelight/common/block/CuttingBoardBlock.java:190)
- [ ] `CuttingBoardBlock.spawnCuttingParticles(4)`: it uses Minecraft's `Level` (vectorwing/farmersdelight/common/block/CuttingBoardBlock.java:194)

### `CuttingBoardBlock.ToolCarvingEvent`

- [ ] `CuttingBoardBlock.ToolCarvingEvent.onSneakPlaceTool(4)`: it uses Minecraft's `InteractionResult` (vectorwing/farmersdelight/common/block/CuttingBoardBlock.java:225)

### `AbstractStoveBlockEntity`: it extends Minecraft's `BlockEntity`

- [ ] `AbstractStoveBlockEntity.AbstractStoveBlockEntity(4)`: its class isn't translated (it extends Minecraft's `BlockEntity`) (vectorwing/farmersdelight/common/block/entity/AbstractStoveBlockEntity.java:43)
- [ ] `AbstractStoveBlockEntity.getInventorySlotCount(0)`: it has no body (vectorwing/farmersdelight/common/block/entity/AbstractStoveBlockEntity.java:54)
- [ ] `AbstractStoveBlockEntity.getStoveItemOffset(1)`: it has no body (vectorwing/farmersdelight/common/block/entity/AbstractStoveBlockEntity.java:56)
- [ ] `AbstractStoveBlockEntity.getItems(0)`: its class isn't translated (it extends Minecraft's `BlockEntity`) (vectorwing/farmersdelight/common/block/entity/AbstractStoveBlockEntity.java:59)
- [ ] `AbstractStoveBlockEntity.loadAdditional(1)`: its class isn't translated (it extends Minecraft's `BlockEntity`) (vectorwing/farmersdelight/common/block/entity/AbstractStoveBlockEntity.java:63)
- [ ] `AbstractStoveBlockEntity.saveAdditional(1)`: its class isn't translated (it extends Minecraft's `BlockEntity`) (vectorwing/farmersdelight/common/block/entity/AbstractStoveBlockEntity.java:77)
- [ ] `AbstractStoveBlockEntity.writeItems(1)`: its class isn't translated (it extends Minecraft's `BlockEntity`) (vectorwing/farmersdelight/common/block/entity/AbstractStoveBlockEntity.java:83)
- [ ] `AbstractStoveBlockEntity.preRemoveSideEffects(2)`: its class isn't translated (it extends Minecraft's `BlockEntity`) (vectorwing/farmersdelight/common/block/entity/AbstractStoveBlockEntity.java:88)
- [ ] `AbstractStoveBlockEntity.getUpdatePacket(0)`: its class isn't translated (it extends Minecraft's `BlockEntity`) (vectorwing/farmersdelight/common/block/entity/AbstractStoveBlockEntity.java:96)
- [ ] `AbstractStoveBlockEntity.getUpdateTag(1)`: its class isn't translated (it extends Minecraft's `BlockEntity`) (vectorwing/farmersdelight/common/block/entity/AbstractStoveBlockEntity.java:100)
- [ ] `AbstractStoveBlockEntity.serverTick(4)`: it uses Minecraft's `ServerLevel` (vectorwing/farmersdelight/common/block/entity/AbstractStoveBlockEntity.java:122)
- [ ] `AbstractStoveBlockEntity.cookAndOutputItems(1)`: its class isn't translated (it extends Minecraft's `BlockEntity`) (vectorwing/farmersdelight/common/block/entity/AbstractStoveBlockEntity.java:137)
- [ ] `AbstractStoveBlockEntity.coolItems(0)`: its body couldn't be read: an assert (vectorwing/farmersdelight/common/block/entity/AbstractStoveBlockEntity.java:176)
- [ ] `AbstractStoveBlockEntity.getCookingRecipe(1)`: its class isn't translated (it extends Minecraft's `BlockEntity`) (vectorwing/farmersdelight/common/block/entity/AbstractStoveBlockEntity.java:193)
- [ ] `AbstractStoveBlockEntity.getNextEmptySlot(0)`: its class isn't translated (it extends Minecraft's `BlockEntity`) (vectorwing/farmersdelight/common/block/entity/AbstractStoveBlockEntity.java:199)
- [ ] `AbstractStoveBlockEntity.placeFood(3)`: its body couldn't be read: an assert (vectorwing/farmersdelight/common/block/entity/AbstractStoveBlockEntity.java:203)
- [ ] `AbstractStoveBlockEntity.shouldDropItems(0)`: its class isn't translated (it extends Minecraft's `BlockEntity`) (vectorwing/farmersdelight/common/block/entity/AbstractStoveBlockEntity.java:221)
- [ ] `AbstractStoveBlockEntity.streamItems(0)`: its class isn't translated (it extends Minecraft's `BlockEntity`) (vectorwing/farmersdelight/common/block/entity/AbstractStoveBlockEntity.java:225)
- [ ] `AbstractStoveBlockEntity.isEmpty(0)`: its class isn't translated (it extends Minecraft's `BlockEntity`) (vectorwing/farmersdelight/common/block/entity/AbstractStoveBlockEntity.java:229)
- [ ] `AbstractStoveBlockEntity.isFull(0)`: its class isn't translated (it extends Minecraft's `BlockEntity`) (vectorwing/farmersdelight/common/block/entity/AbstractStoveBlockEntity.java:233)
- [ ] `AbstractStoveBlockEntity.dropAllItems(0)`: its class isn't translated (it extends Minecraft's `BlockEntity`) (vectorwing/farmersdelight/common/block/entity/AbstractStoveBlockEntity.java:237)
- [ ] `AbstractStoveBlockEntity.extinguish(0)`: its class isn't translated (it extends Minecraft's `BlockEntity`) (vectorwing/farmersdelight/common/block/entity/AbstractStoveBlockEntity.java:246)
- [ ] `AbstractStoveBlockEntity.ignite(0)`: its class isn't translated (it extends Minecraft's `BlockEntity`) (vectorwing/farmersdelight/common/block/entity/AbstractStoveBlockEntity.java:253)
- [ ] `AbstractStoveBlockEntity.clearContent(0)`: its class isn't translated (it extends Minecraft's `BlockEntity`) (vectorwing/farmersdelight/common/block/entity/AbstractStoveBlockEntity.java:260)
- [ ] `AbstractStoveBlockEntity.createHandler(1)`: its body couldn't be read: an anonymous class (vectorwing/farmersdelight/common/block/entity/AbstractStoveBlockEntity.java:264)

### `Basket`: it's an interface

- [ ] `Basket.getFacingCollectionArea(1)`: its class isn't translated (it's an interface) (vectorwing/farmersdelight/common/block/entity/Basket.java:24)
- [ ] `Basket.getLevelX(0)`: it has no body (vectorwing/farmersdelight/common/block/entity/Basket.java:28)
- [ ] `Basket.getLevelY(0)`: it has no body (vectorwing/farmersdelight/common/block/entity/Basket.java:30)
- [ ] `Basket.getLevelZ(0)`: it has no body (vectorwing/farmersdelight/common/block/entity/Basket.java:32)
- [ ] `Basket.setCooldown(1)`: it has no body (vectorwing/farmersdelight/common/block/entity/Basket.java:34)
- [ ] `Basket.isOnCooldown(0)`: it has no body (vectorwing/farmersdelight/common/block/entity/Basket.java:36)
- [ ] `Basket.isOnCustomCooldown(0)`: it has no body (vectorwing/farmersdelight/common/block/entity/Basket.java:38)
- [ ] `Basket.tryTransfer(1)`: it has no body (vectorwing/farmersdelight/common/block/entity/Basket.java:40)
- [ ] `Basket.collectItems(2)`: its class isn't translated (it's an interface) (vectorwing/farmersdelight/common/block/entity/Basket.java:42)
- [ ] `Basket.getItemsToCollect(2)`: its class isn't translated (it's an interface) (vectorwing/farmersdelight/common/block/entity/Basket.java:52)
- [ ] `Basket.collectItem(1)`: its class isn't translated (it's an interface) (vectorwing/farmersdelight/common/block/entity/Basket.java:65)
- [ ] `Basket.insert(1)`: its class isn't translated (it's an interface) (vectorwing/farmersdelight/common/block/entity/Basket.java:79)
- [ ] `Basket.insert(2)`: its class isn't translated (it's an interface) (vectorwing/farmersdelight/common/block/entity/Basket.java:89)
- [ ] `Basket.canMergeItems(2)`: it calls Minecraft's ItemStack.getMaxStackSize (no Horizon helper for it yet) (vectorwing/farmersdelight/common/block/entity/Basket.java:117)

### `BasketBlockEntity`: it extends Minecraft's `RandomizableContainerBlockEntity`

- [ ] `BasketBlockEntity.BasketBlockEntity(2)`: its class isn't translated (it extends Minecraft's `RandomizableContainerBlockEntity`) (vectorwing/farmersdelight/common/block/entity/BasketBlockEntity.java:29)
- [ ] `BasketBlockEntity.init(0)`: it uses Fabric's `ItemStorage`.SIDED (vectorwing/farmersdelight/common/block/entity/BasketBlockEntity.java:33)
- [ ] `BasketBlockEntity.loadAdditional(1)`: its class isn't translated (it extends Minecraft's `RandomizableContainerBlockEntity`) (vectorwing/farmersdelight/common/block/entity/BasketBlockEntity.java:37)
- [ ] `BasketBlockEntity.saveAdditional(1)`: its class isn't translated (it extends Minecraft's `RandomizableContainerBlockEntity`) (vectorwing/farmersdelight/common/block/entity/BasketBlockEntity.java:47)
- [ ] `BasketBlockEntity.getContainerSize(0)`: its class isn't translated (it extends Minecraft's `RandomizableContainerBlockEntity`) (vectorwing/farmersdelight/common/block/entity/BasketBlockEntity.java:56)
- [ ] `BasketBlockEntity.removeItem(2)`: its class isn't translated (it extends Minecraft's `RandomizableContainerBlockEntity`) (vectorwing/farmersdelight/common/block/entity/BasketBlockEntity.java:60)
- [ ] `BasketBlockEntity.setItem(2)`: its class isn't translated (it extends Minecraft's `RandomizableContainerBlockEntity`) (vectorwing/farmersdelight/common/block/entity/BasketBlockEntity.java:65)
- [ ] `BasketBlockEntity.getDefaultName(0)`: its class isn't translated (it extends Minecraft's `RandomizableContainerBlockEntity`) (vectorwing/farmersdelight/common/block/entity/BasketBlockEntity.java:73)
- [ ] `BasketBlockEntity.getItems(0)`: its class isn't translated (it extends Minecraft's `RandomizableContainerBlockEntity`) (vectorwing/farmersdelight/common/block/entity/BasketBlockEntity.java:77)
- [ ] `BasketBlockEntity.setItems(1)`: its class isn't translated (it extends Minecraft's `RandomizableContainerBlockEntity`) (vectorwing/farmersdelight/common/block/entity/BasketBlockEntity.java:81)
- [ ] `BasketBlockEntity.createMenu(2)`: its class isn't translated (it extends Minecraft's `RandomizableContainerBlockEntity`) (vectorwing/farmersdelight/common/block/entity/BasketBlockEntity.java:85)
- [ ] `BasketBlockEntity.setCooldown(1)`: its class isn't translated (it extends Minecraft's `RandomizableContainerBlockEntity`) (vectorwing/farmersdelight/common/block/entity/BasketBlockEntity.java:89)
- [ ] `BasketBlockEntity.isOnCooldown(0)`: its class isn't translated (it extends Minecraft's `RandomizableContainerBlockEntity`) (vectorwing/farmersdelight/common/block/entity/BasketBlockEntity.java:94)
- [ ] `BasketBlockEntity.isOnCustomCooldown(0)`: its class isn't translated (it extends Minecraft's `RandomizableContainerBlockEntity`) (vectorwing/farmersdelight/common/block/entity/BasketBlockEntity.java:99)
- [ ] `BasketBlockEntity.tryTransfer(1)`: its class isn't translated (it extends Minecraft's `RandomizableContainerBlockEntity`) (vectorwing/farmersdelight/common/block/entity/BasketBlockEntity.java:104)
- [ ] `BasketBlockEntity.isFull(0)`: its class isn't translated (it extends Minecraft's `RandomizableContainerBlockEntity`) (vectorwing/farmersdelight/common/block/entity/BasketBlockEntity.java:119)
- [ ] `BasketBlockEntity.getLevelX(0)`: its class isn't translated (it extends Minecraft's `RandomizableContainerBlockEntity`) (vectorwing/farmersdelight/common/block/entity/BasketBlockEntity.java:129)
- [ ] `BasketBlockEntity.getLevelY(0)`: its class isn't translated (it extends Minecraft's `RandomizableContainerBlockEntity`) (vectorwing/farmersdelight/common/block/entity/BasketBlockEntity.java:134)
- [ ] `BasketBlockEntity.getLevelZ(0)`: its class isn't translated (it extends Minecraft's `RandomizableContainerBlockEntity`) (vectorwing/farmersdelight/common/block/entity/BasketBlockEntity.java:139)
- [ ] `BasketBlockEntity.pushItemsTick(4)`: it uses Minecraft's `Level` (vectorwing/farmersdelight/common/block/entity/BasketBlockEntity.java:144)

### `CabinetBlockEntity`: it extends Minecraft's `RandomizableContainerBlockEntity`

- [ ] `CabinetBlockEntity.CabinetBlockEntity(2)`: its class isn't translated (it extends Minecraft's `RandomizableContainerBlockEntity`) (vectorwing/farmersdelight/common/block/entity/CabinetBlockEntity.java:56)
- [ ] `CabinetBlockEntity.init(0)`: it uses Fabric's `ItemStorage`.SIDED (vectorwing/farmersdelight/common/block/entity/CabinetBlockEntity.java:60)
- [ ] `CabinetBlockEntity.saveAdditional(1)`: its class isn't translated (it extends Minecraft's `RandomizableContainerBlockEntity`) (vectorwing/farmersdelight/common/block/entity/CabinetBlockEntity.java:64)
- [ ] `CabinetBlockEntity.loadAdditional(1)`: its class isn't translated (it extends Minecraft's `RandomizableContainerBlockEntity`) (vectorwing/farmersdelight/common/block/entity/CabinetBlockEntity.java:71)
- [ ] `CabinetBlockEntity.getContainerSize(0)`: its class isn't translated (it extends Minecraft's `RandomizableContainerBlockEntity`) (vectorwing/farmersdelight/common/block/entity/CabinetBlockEntity.java:79)
- [ ] `CabinetBlockEntity.getItems(0)`: its class isn't translated (it extends Minecraft's `RandomizableContainerBlockEntity`) (vectorwing/farmersdelight/common/block/entity/CabinetBlockEntity.java:83)
- [ ] `CabinetBlockEntity.setItems(1)`: its class isn't translated (it extends Minecraft's `RandomizableContainerBlockEntity`) (vectorwing/farmersdelight/common/block/entity/CabinetBlockEntity.java:87)
- [ ] `CabinetBlockEntity.getDefaultName(0)`: its class isn't translated (it extends Minecraft's `RandomizableContainerBlockEntity`) (vectorwing/farmersdelight/common/block/entity/CabinetBlockEntity.java:91)
- [ ] `CabinetBlockEntity.createMenu(2)`: its class isn't translated (it extends Minecraft's `RandomizableContainerBlockEntity`) (vectorwing/farmersdelight/common/block/entity/CabinetBlockEntity.java:95)
- [ ] `CabinetBlockEntity.startOpen(1)`: its class isn't translated (it extends Minecraft's `RandomizableContainerBlockEntity`) (vectorwing/farmersdelight/common/block/entity/CabinetBlockEntity.java:99)
- [ ] `CabinetBlockEntity.stopOpen(1)`: its class isn't translated (it extends Minecraft's `RandomizableContainerBlockEntity`) (vectorwing/farmersdelight/common/block/entity/CabinetBlockEntity.java:105)
- [ ] `CabinetBlockEntity.recheckOpen(0)`: its class isn't translated (it extends Minecraft's `RandomizableContainerBlockEntity`) (vectorwing/farmersdelight/common/block/entity/CabinetBlockEntity.java:111)
- [ ] `CabinetBlockEntity.updateBlockState(2)`: its class isn't translated (it extends Minecraft's `RandomizableContainerBlockEntity`) (vectorwing/farmersdelight/common/block/entity/CabinetBlockEntity.java:117)
- [ ] `CabinetBlockEntity.playSound(2)`: its class isn't translated (it extends Minecraft's `RandomizableContainerBlockEntity`) (vectorwing/farmersdelight/common/block/entity/CabinetBlockEntity.java:123)

### `CookingPotMealSlot`: it extends `ItemHandlerSlot` (vectorwing.farmersdelight.refabricated.inventory)

- [ ] `CookingPotMealSlot.CookingPotMealSlot(4)`: its class isn't translated (it extends `ItemHandlerSlot` (vectorwing.farmersdelight.refabricated.inventory)) (vectorwing/farmersdelight/common/block/entity/container/CookingPotMealSlot.java:9)
- [ ] `CookingPotMealSlot.mayPlace(1)`: its class isn't translated (it extends `ItemHandlerSlot` (vectorwing.farmersdelight.refabricated.inventory)) (vectorwing/farmersdelight/common/block/entity/container/CookingPotMealSlot.java:13)
- [ ] `CookingPotMealSlot.mayPickup(1)`: its class isn't translated (it extends `ItemHandlerSlot` (vectorwing.farmersdelight.refabricated.inventory)) (vectorwing/farmersdelight/common/block/entity/container/CookingPotMealSlot.java:18)

### `CookingPotMenu`: it extends Minecraft's `RecipeBookMenu`

- [x] `CookingPotMenu.EMPTY_CONTAINER_SLOT_BOWL` (vectorwing/farmersdelight/common/block/entity/container/CookingPotMenu.java:35)
- [x] `CookingPotMenu.INDEX_MEAL` (vectorwing/farmersdelight/common/block/entity/container/CookingPotMenu.java:36)
- [x] `CookingPotMenu.INDEX_CONTAINER` (vectorwing/farmersdelight/common/block/entity/container/CookingPotMenu.java:37)
- [x] `CookingPotMenu.INDEX_OUTPUT` (vectorwing/farmersdelight/common/block/entity/container/CookingPotMenu.java:38)
- [ ] `CookingPotMenu.CookingPotMenu(3)`: its class isn't translated (it extends Minecraft's `RecipeBookMenu`) (vectorwing/farmersdelight/common/block/entity/container/CookingPotMenu.java:45)
- [ ] `CookingPotMenu.CookingPotMenu(4)`: its body couldn't be read: an anonymous class (vectorwing/farmersdelight/common/block/entity/container/CookingPotMenu.java:49)
- [ ] `CookingPotMenu.getTileEntity(2)`: it uses CookingPotBlockEntity, which isn't translated (it extends `SyncedBlockEntity`) (vectorwing/farmersdelight/common/block/entity/container/CookingPotMenu.java:90)
- [ ] `CookingPotMenu.stillValid(1)`: its class isn't translated (it extends Minecraft's `RecipeBookMenu`) (vectorwing/farmersdelight/common/block/entity/container/CookingPotMenu.java:101)
- [ ] `CookingPotMenu.quickMoveStack(2)`: its class isn't translated (it extends Minecraft's `RecipeBookMenu`) (vectorwing/farmersdelight/common/block/entity/container/CookingPotMenu.java:105)
- [ ] `CookingPotMenu.getCookProgressionScaled(0)`: its class isn't translated (it extends Minecraft's `RecipeBookMenu`) (vectorwing/farmersdelight/common/block/entity/container/CookingPotMenu.java:145)
- [ ] `CookingPotMenu.isHeated(0)`: its class isn't translated (it extends Minecraft's `RecipeBookMenu`) (vectorwing/farmersdelight/common/block/entity/container/CookingPotMenu.java:151)
- [ ] `CookingPotMenu.handlePlacement(5)`: its body couldn't be read: an anonymous class (vectorwing/farmersdelight/common/block/entity/container/CookingPotMenu.java:155)
- [ ] `CookingPotMenu.fillCraftSlotsStackedContents(1)`: its class isn't translated (it extends Minecraft's `RecipeBookMenu`) (vectorwing/farmersdelight/common/block/entity/container/CookingPotMenu.java:175)
- [ ] `CookingPotMenu.getRecipeBookType(0)`: its class isn't translated (it extends Minecraft's `RecipeBookMenu`) (vectorwing/farmersdelight/common/block/entity/container/CookingPotMenu.java:181)

### `CookingPotResultSlot`: it extends `ItemHandlerSlot` (vectorwing.farmersdelight.refabricated.inventory)

- [ ] `CookingPotResultSlot.CookingPotResultSlot(6)`: its class isn't translated (it extends `ItemHandlerSlot` (vectorwing.farmersdelight.refabricated.inventory)) (vectorwing/farmersdelight/common/block/entity/container/CookingPotResultSlot.java:15)
- [ ] `CookingPotResultSlot.mayPlace(1)`: its class isn't translated (it extends `ItemHandlerSlot` (vectorwing.farmersdelight.refabricated.inventory)) (vectorwing/farmersdelight/common/block/entity/container/CookingPotResultSlot.java:21)
- [ ] `CookingPotResultSlot.remove(1)`: its class isn't translated (it extends `ItemHandlerSlot` (vectorwing.farmersdelight.refabricated.inventory)) (vectorwing/farmersdelight/common/block/entity/container/CookingPotResultSlot.java:26)
- [ ] `CookingPotResultSlot.onTake(2)`: its class isn't translated (it extends `ItemHandlerSlot` (vectorwing.farmersdelight.refabricated.inventory)) (vectorwing/farmersdelight/common/block/entity/container/CookingPotResultSlot.java:35)
- [ ] `CookingPotResultSlot.onQuickCraft(2)`: its class isn't translated (it extends `ItemHandlerSlot` (vectorwing.farmersdelight.refabricated.inventory)) (vectorwing/farmersdelight/common/block/entity/container/CookingPotResultSlot.java:40)
- [ ] `CookingPotResultSlot.checkTakeAchievements(1)`: its class isn't translated (it extends `ItemHandlerSlot` (vectorwing.farmersdelight.refabricated.inventory)) (vectorwing/farmersdelight/common/block/entity/container/CookingPotResultSlot.java:45)

### `CookingPotBlockEntity`: it extends `SyncedBlockEntity`

- [x] `CookingPotBlockEntity.LOGGER` (vectorwing/farmersdelight/common/block/entity/CookingPotBlockEntity.java:80)
- [ ] `CookingPotBlockEntity.RECIPES_USED_CODEC`: it uses Minecraft's `Codec` (vectorwing/farmersdelight/common/block/entity/CookingPotBlockEntity.java:81)
- [x] `CookingPotBlockEntity.MEAL_DISPLAY_SLOT` (vectorwing/farmersdelight/common/block/entity/CookingPotBlockEntity.java:82)
- [x] `CookingPotBlockEntity.CONTAINER_SLOT` (vectorwing/farmersdelight/common/block/entity/CookingPotBlockEntity.java:83)
- [x] `CookingPotBlockEntity.OUTPUT_SLOT` (vectorwing/farmersdelight/common/block/entity/CookingPotBlockEntity.java:84)
- [x] `CookingPotBlockEntity.INVENTORY_SIZE` (vectorwing/farmersdelight/common/block/entity/CookingPotBlockEntity.java:85)
- [ ] `CookingPotBlockEntity.INGREDIENT_REMAINDER_OVERRIDES`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/block/entity/CookingPotBlockEntity.java:86)
- [ ] `CookingPotBlockEntity.CookingPotBlockEntity(2)`: its class isn't translated (it extends `SyncedBlockEntity`) (vectorwing/farmersdelight/common/block/entity/CookingPotBlockEntity.java:113)
- [ ] `CookingPotBlockEntity.init(0)`: it uses Fabric's `ItemStorage`.SIDED (vectorwing/farmersdelight/common/block/entity/CookingPotBlockEntity.java:117)
- [ ] `CookingPotBlockEntity.getMealFromItem(1)`: it uses ModItems.COOKING_POT, which isn't translated (it uses Minecraft's `Item`) (vectorwing/farmersdelight/common/block/entity/CookingPotBlockEntity.java:121)
- [ ] `CookingPotBlockEntity.takeServingFromItem(1)`: it uses ModItems.COOKING_POT, which isn't translated (it uses Minecraft's `Item`) (vectorwing/farmersdelight/common/block/entity/CookingPotBlockEntity.java:127)
- [ ] `CookingPotBlockEntity.getContainerFromItem(1)`: it uses ModItems.COOKING_POT, which isn't translated (it uses Minecraft's `Item`) (vectorwing/farmersdelight/common/block/entity/CookingPotBlockEntity.java:135)
- [ ] `CookingPotBlockEntity.loadAdditional(1)`: its class isn't translated (it extends `SyncedBlockEntity`) (vectorwing/farmersdelight/common/block/entity/CookingPotBlockEntity.java:141)
- [ ] `CookingPotBlockEntity.saveAdditional(1)`: its class isn't translated (it extends `SyncedBlockEntity`) (vectorwing/farmersdelight/common/block/entity/CookingPotBlockEntity.java:152)
- [ ] `CookingPotBlockEntity.writeItems(1)`: its class isn't translated (it extends `SyncedBlockEntity`) (vectorwing/farmersdelight/common/block/entity/CookingPotBlockEntity.java:165)
- [ ] `CookingPotBlockEntity.preRemoveSideEffects(2)`: its class isn't translated (it extends `SyncedBlockEntity`) (vectorwing/farmersdelight/common/block/entity/CookingPotBlockEntity.java:171)
- [ ] `CookingPotBlockEntity.writeMeal(1)`: its class isn't translated (it extends `SyncedBlockEntity`) (vectorwing/farmersdelight/common/block/entity/CookingPotBlockEntity.java:181)
- [ ] `CookingPotBlockEntity.getAsItem(0)`: its class isn't translated (it extends `SyncedBlockEntity`) (vectorwing/farmersdelight/common/block/entity/CookingPotBlockEntity.java:198)
- [ ] `CookingPotBlockEntity.cookingTick(4)`: it uses Minecraft's `ServerLevel` (vectorwing/farmersdelight/common/block/entity/CookingPotBlockEntity.java:204)
- [ ] `CookingPotBlockEntity.animationTick(4)`: it uses Minecraft's `Level` (vectorwing/farmersdelight/common/block/entity/CookingPotBlockEntity.java:234)
- [ ] `CookingPotBlockEntity.getMatchingRecipe(2)`: its class isn't translated (it extends `SyncedBlockEntity`) (vectorwing/farmersdelight/common/block/entity/CookingPotBlockEntity.java:254)
- [ ] `CookingPotBlockEntity.getContainer(0)`: its class isn't translated (it extends `SyncedBlockEntity`) (vectorwing/farmersdelight/common/block/entity/CookingPotBlockEntity.java:262)
- [ ] `CookingPotBlockEntity.hasInput(0)`: its class isn't translated (it extends `SyncedBlockEntity`) (vectorwing/farmersdelight/common/block/entity/CookingPotBlockEntity.java:271)
- [ ] `CookingPotBlockEntity.canCook(1)`: its class isn't translated (it extends `SyncedBlockEntity`) (vectorwing/farmersdelight/common/block/entity/CookingPotBlockEntity.java:281)
- [ ] `CookingPotBlockEntity.processCooking(2)`: its class isn't translated (it extends `SyncedBlockEntity`) (vectorwing/farmersdelight/common/block/entity/CookingPotBlockEntity.java:303)
- [ ] `CookingPotBlockEntity.ejectIngredientRemainder(1)`: its class isn't translated (it extends `SyncedBlockEntity`) (vectorwing/farmersdelight/common/block/entity/CookingPotBlockEntity.java:343)
- [ ] `CookingPotBlockEntity.setRecipeUsed(1)`: its class isn't translated (it extends `SyncedBlockEntity`) (vectorwing/farmersdelight/common/block/entity/CookingPotBlockEntity.java:351)
- [ ] `CookingPotBlockEntity.getRecipeUsed(0)`: its class isn't translated (it extends `SyncedBlockEntity`) (vectorwing/farmersdelight/common/block/entity/CookingPotBlockEntity.java:358)
- [ ] `CookingPotBlockEntity.awardUsedRecipes(2)`: its class isn't translated (it extends `SyncedBlockEntity`) (vectorwing/farmersdelight/common/block/entity/CookingPotBlockEntity.java:362)
- [ ] `CookingPotBlockEntity.getUsedRecipesAndPopExperience(2)`: its class isn't translated (it extends `SyncedBlockEntity`) (vectorwing/farmersdelight/common/block/entity/CookingPotBlockEntity.java:370)
- [ ] `CookingPotBlockEntity.splitAndSpawnExperience(4)`: it uses Minecraft's `ServerLevel` (vectorwing/farmersdelight/common/block/entity/CookingPotBlockEntity.java:385)
- [ ] `CookingPotBlockEntity.isHeated(0)`: its class isn't translated (it extends `SyncedBlockEntity`) (vectorwing/farmersdelight/common/block/entity/CookingPotBlockEntity.java:395)
- [ ] `CookingPotBlockEntity.getInventory(0)`: its class isn't translated (it extends `SyncedBlockEntity`) (vectorwing/farmersdelight/common/block/entity/CookingPotBlockEntity.java:399)
- [ ] `CookingPotBlockEntity.getMeal(0)`: its class isn't translated (it extends `SyncedBlockEntity`) (vectorwing/farmersdelight/common/block/entity/CookingPotBlockEntity.java:403)
- [ ] `CookingPotBlockEntity.getDroppableInventory(0)`: its class isn't translated (it extends `SyncedBlockEntity`) (vectorwing/farmersdelight/common/block/entity/CookingPotBlockEntity.java:407)
- [ ] `CookingPotBlockEntity.moveMealToOutput(0)`: its class isn't translated (it extends `SyncedBlockEntity`) (vectorwing/farmersdelight/common/block/entity/CookingPotBlockEntity.java:419)
- [ ] `CookingPotBlockEntity.useStoredContainersOnMeal(0)`: its class isn't translated (it extends `SyncedBlockEntity`) (vectorwing/farmersdelight/common/block/entity/CookingPotBlockEntity.java:431)
- [ ] `CookingPotBlockEntity.useHeldItemOnMeal(1)`: its class isn't translated (it extends `SyncedBlockEntity`) (vectorwing/farmersdelight/common/block/entity/CookingPotBlockEntity.java:449)
- [ ] `CookingPotBlockEntity.doesMealHaveContainer(1)`: its class isn't translated (it extends `SyncedBlockEntity`) (vectorwing/farmersdelight/common/block/entity/CookingPotBlockEntity.java:458)
- [ ] `CookingPotBlockEntity.isContainerValid(1)`: its class isn't translated (it extends `SyncedBlockEntity`) (vectorwing/farmersdelight/common/block/entity/CookingPotBlockEntity.java:462)
- [ ] `CookingPotBlockEntity.getName(0)`: its class isn't translated (it extends `SyncedBlockEntity`) (vectorwing/farmersdelight/common/block/entity/CookingPotBlockEntity.java:470)
- [ ] `CookingPotBlockEntity.getDisplayName(0)`: its class isn't translated (it extends `SyncedBlockEntity`) (vectorwing/farmersdelight/common/block/entity/CookingPotBlockEntity.java:474)
- [ ] `CookingPotBlockEntity.getCustomName(0)`: its class isn't translated (it extends `SyncedBlockEntity`) (vectorwing/farmersdelight/common/block/entity/CookingPotBlockEntity.java:478)
- [ ] `CookingPotBlockEntity.createMenu(3)`: its class isn't translated (it extends `SyncedBlockEntity`) (vectorwing/farmersdelight/common/block/entity/CookingPotBlockEntity.java:482)
- [ ] `CookingPotBlockEntity.getStorage(1)`: its class isn't translated (it extends `SyncedBlockEntity`) (vectorwing/farmersdelight/common/block/entity/CookingPotBlockEntity.java:486)
- [ ] `CookingPotBlockEntity.setRemoved(0)`: its class isn't translated (it extends `SyncedBlockEntity`) (vectorwing/farmersdelight/common/block/entity/CookingPotBlockEntity.java:490)
- [ ] `CookingPotBlockEntity.getUpdateTag(1)`: its class isn't translated (it extends `SyncedBlockEntity`) (vectorwing/farmersdelight/common/block/entity/CookingPotBlockEntity.java:494)
- [ ] `CookingPotBlockEntity.applyImplicitComponents(1)`: its class isn't translated (it extends `SyncedBlockEntity`) (vectorwing/farmersdelight/common/block/entity/CookingPotBlockEntity.java:517)
- [ ] `CookingPotBlockEntity.collectImplicitComponents(1)`: its class isn't translated (it extends `SyncedBlockEntity`) (vectorwing/farmersdelight/common/block/entity/CookingPotBlockEntity.java:524)
- [ ] `CookingPotBlockEntity.createHandler(0)`: its body couldn't be read: an anonymous class (vectorwing/farmersdelight/common/block/entity/CookingPotBlockEntity.java:536)
- [ ] `CookingPotBlockEntity.createIntArray(0)`: its body couldn't be read: an anonymous class (vectorwing/farmersdelight/common/block/entity/CookingPotBlockEntity.java:555)
- [ ] `CookingPotBlockEntity.getScreenOpeningData(1)`: its class isn't translated (it extends `SyncedBlockEntity`) (vectorwing/farmersdelight/common/block/entity/CookingPotBlockEntity.java:581)

### `CuttingBoardBlockEntity`: it extends `SyncedBlockEntity`

- [ ] `CuttingBoardBlockEntity.CuttingBoardBlockEntity(2)`: its class isn't translated (it extends `SyncedBlockEntity`) (vectorwing/farmersdelight/common/block/entity/CuttingBoardBlockEntity.java:59)
- [ ] `CuttingBoardBlockEntity.init(0)`: it uses Fabric's `ItemStorage`.SIDED (vectorwing/farmersdelight/common/block/entity/CuttingBoardBlockEntity.java:64)
- [ ] `CuttingBoardBlockEntity.getStorage(1)`: its class isn't translated (it extends `SyncedBlockEntity`) (vectorwing/farmersdelight/common/block/entity/CuttingBoardBlockEntity.java:68)
- [ ] `CuttingBoardBlockEntity.loadAdditional(1)`: its class isn't translated (it extends `SyncedBlockEntity`) (vectorwing/farmersdelight/common/block/entity/CuttingBoardBlockEntity.java:72)
- [ ] `CuttingBoardBlockEntity.saveAdditional(1)`: its class isn't translated (it extends `SyncedBlockEntity`) (vectorwing/farmersdelight/common/block/entity/CuttingBoardBlockEntity.java:78)
- [ ] `CuttingBoardBlockEntity.preRemoveSideEffects(2)`: its class isn't translated (it extends `SyncedBlockEntity`) (vectorwing/farmersdelight/common/block/entity/CuttingBoardBlockEntity.java:84)
- [ ] `CuttingBoardBlockEntity.processStoredItemUsingTool(2)`: its class isn't translated (it extends `SyncedBlockEntity`) (vectorwing/farmersdelight/common/block/entity/CuttingBoardBlockEntity.java:92)
- [ ] `CuttingBoardBlockEntity.getMatchingRecipe(3)`: its class isn't translated (it extends `SyncedBlockEntity`) (vectorwing/farmersdelight/common/block/entity/CuttingBoardBlockEntity.java:145)
- [ ] `CuttingBoardBlockEntity.spawnCuttingParticles(3)`: its class isn't translated (it extends `SyncedBlockEntity`) (vectorwing/farmersdelight/common/block/entity/CuttingBoardBlockEntity.java:163)
- [ ] `CuttingBoardBlockEntity.playProcessingSound(3)`: its class isn't translated (it extends `SyncedBlockEntity`) (vectorwing/farmersdelight/common/block/entity/CuttingBoardBlockEntity.java:169)
- [ ] `CuttingBoardBlockEntity.playSound(3)`: its class isn't translated (it extends `SyncedBlockEntity`) (vectorwing/farmersdelight/common/block/entity/CuttingBoardBlockEntity.java:185)
- [ ] `CuttingBoardBlockEntity.canAddItem(1)`: its class isn't translated (it extends `SyncedBlockEntity`) (vectorwing/farmersdelight/common/block/entity/CuttingBoardBlockEntity.java:201)
- [ ] `CuttingBoardBlockEntity.addItem(1)`: its class isn't translated (it extends `SyncedBlockEntity`) (vectorwing/farmersdelight/common/block/entity/CuttingBoardBlockEntity.java:207)
- [ ] `CuttingBoardBlockEntity.removeItem(0)`: its class isn't translated (it extends `SyncedBlockEntity`) (vectorwing/farmersdelight/common/block/entity/CuttingBoardBlockEntity.java:211)
- [ ] `CuttingBoardBlockEntity.carveToolOnBoard(1)`: its class isn't translated (it extends `SyncedBlockEntity`) (vectorwing/farmersdelight/common/block/entity/CuttingBoardBlockEntity.java:216)
- [ ] `CuttingBoardBlockEntity.getInventory(0)`: its class isn't translated (it extends `SyncedBlockEntity`) (vectorwing/farmersdelight/common/block/entity/CuttingBoardBlockEntity.java:226)
- [ ] `CuttingBoardBlockEntity.getStoredItem(0)`: its class isn't translated (it extends `SyncedBlockEntity`) (vectorwing/farmersdelight/common/block/entity/CuttingBoardBlockEntity.java:230)
- [ ] `CuttingBoardBlockEntity.getMaxStackSize(0)`: its class isn't translated (it extends `SyncedBlockEntity`) (vectorwing/farmersdelight/common/block/entity/CuttingBoardBlockEntity.java:234)
- [ ] `CuttingBoardBlockEntity.isEmpty(0)`: its class isn't translated (it extends `SyncedBlockEntity`) (vectorwing/farmersdelight/common/block/entity/CuttingBoardBlockEntity.java:238)
- [ ] `CuttingBoardBlockEntity.isItemCarvingBoard(0)`: its class isn't translated (it extends `SyncedBlockEntity`) (vectorwing/farmersdelight/common/block/entity/CuttingBoardBlockEntity.java:242)
- [ ] `CuttingBoardBlockEntity.setRemoved(0)`: its class isn't translated (it extends `SyncedBlockEntity`) (vectorwing/farmersdelight/common/block/entity/CuttingBoardBlockEntity.java:246)
- [ ] `CuttingBoardBlockEntity.createHandler(0)`: its body couldn't be read: an anonymous class (vectorwing/farmersdelight/common/block/entity/CuttingBoardBlockEntity.java:250)

### `CuttingBoardDispenseBehavior`: it extends Minecraft's `OptionalDispenseItemBehavior`

- [ ] `CuttingBoardDispenseBehavior.DISPENSE_ITEM_BEHAVIOR_HASH_MAP`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/block/entity/dispenser/CuttingBoardDispenseBehavior.java:19)
- [ ] `CuttingBoardDispenseBehavior.INSTANCE`: it uses CuttingBoardDispenseBehavior, which isn't translated (it extends Minecraft's `OptionalDispenseItemBehavior`) (vectorwing/farmersdelight/common/block/entity/dispenser/CuttingBoardDispenseBehavior.java:20)
- [ ] `CuttingBoardDispenseBehavior.registerBehaviour(2)`: it uses CuttingBoardDispenseBehavior, which isn't translated (it extends Minecraft's `OptionalDispenseItemBehavior`) (vectorwing/farmersdelight/common/block/entity/dispenser/CuttingBoardDispenseBehavior.java:22)
- [ ] `CuttingBoardDispenseBehavior.dispense(2)`: its class isn't translated (it extends Minecraft's `OptionalDispenseItemBehavior`) (vectorwing/farmersdelight/common/block/entity/dispenser/CuttingBoardDispenseBehavior.java:27)
- [ ] `CuttingBoardDispenseBehavior.tryDispenseStackOnCuttingBoard(2)`: its class isn't translated (it extends Minecraft's `OptionalDispenseItemBehavior`) (vectorwing/farmersdelight/common/block/entity/dispenser/CuttingBoardDispenseBehavior.java:37)

### `HeatableBlockEntity`: it's an interface

- [ ] `HeatableBlockEntity.isHeated(2)`: its class isn't translated (it's an interface) (vectorwing/farmersdelight/common/block/entity/HeatableBlockEntity.java:10)
- [ ] `HeatableBlockEntity.requiresDirectHeat(0)`: its class isn't translated (it's an interface) (vectorwing/farmersdelight/common/block/entity/HeatableBlockEntity.java:30)

### `BasketInvWrapper`: it extends `InvWrapper` (vectorwing.farmersdelight.refabricated.inventory)

- [ ] `BasketInvWrapper.BasketInvWrapper(1)`: its class isn't translated (it extends `InvWrapper` (vectorwing.farmersdelight.refabricated.inventory)) (vectorwing/farmersdelight/common/block/entity/inventory/BasketInvWrapper.java:10)
- [ ] `BasketInvWrapper.insertItem(3)`: its class isn't translated (it extends `InvWrapper` (vectorwing.farmersdelight.refabricated.inventory)) (vectorwing/farmersdelight/common/block/entity/inventory/BasketInvWrapper.java:15)

### `CookingPotItemHandler`: it implements `ItemHandler` (vectorwing.farmersdelight.refabricated.inventory)

- [x] `CookingPotItemHandler.SLOTS_INPUT` (vectorwing/farmersdelight/common/block/entity/inventory/CookingPotItemHandler.java:20)
- [x] `CookingPotItemHandler.SLOT_CONTAINER_INPUT` (vectorwing/farmersdelight/common/block/entity/inventory/CookingPotItemHandler.java:21)
- [x] `CookingPotItemHandler.SLOT_MEAL_OUTPUT` (vectorwing/farmersdelight/common/block/entity/inventory/CookingPotItemHandler.java:22)
- [ ] `CookingPotItemHandler.CookingPotItemHandler(2)`: its class isn't translated (it implements `ItemHandler` (vectorwing.farmersdelight.refabricated.inventory)) (vectorwing/farmersdelight/common/block/entity/inventory/CookingPotItemHandler.java:26)
- [ ] `CookingPotItemHandler.isItemValid(2)`: its class isn't translated (it implements `ItemHandler` (vectorwing.farmersdelight.refabricated.inventory)) (vectorwing/farmersdelight/common/block/entity/inventory/CookingPotItemHandler.java:31)
- [ ] `CookingPotItemHandler.getSlotCount(0)`: its class isn't translated (it implements `ItemHandler` (vectorwing.farmersdelight.refabricated.inventory)) (vectorwing/farmersdelight/common/block/entity/inventory/CookingPotItemHandler.java:36)
- [ ] `CookingPotItemHandler.getSlot(1)`: its class isn't translated (it implements `ItemHandler` (vectorwing.farmersdelight.refabricated.inventory)) (vectorwing/farmersdelight/common/block/entity/inventory/CookingPotItemHandler.java:41)
- [ ] `CookingPotItemHandler.getStackInSlot(1)`: its class isn't translated (it implements `ItemHandler` (vectorwing.farmersdelight.refabricated.inventory)) (vectorwing/farmersdelight/common/block/entity/inventory/CookingPotItemHandler.java:45)
- [ ] `CookingPotItemHandler.setStackInSlot(2)`: its class isn't translated (it implements `ItemHandler` (vectorwing.farmersdelight.refabricated.inventory)) (vectorwing/farmersdelight/common/block/entity/inventory/CookingPotItemHandler.java:50)
- [ ] `CookingPotItemHandler.insertItem(3)`: its class isn't translated (it implements `ItemHandler` (vectorwing.farmersdelight.refabricated.inventory)) (vectorwing/farmersdelight/common/block/entity/inventory/CookingPotItemHandler.java:55)
- [ ] `CookingPotItemHandler.extractItem(3)`: its class isn't translated (it implements `ItemHandler` (vectorwing.farmersdelight.refabricated.inventory)) (vectorwing/farmersdelight/common/block/entity/inventory/CookingPotItemHandler.java:64)
- [ ] `CookingPotItemHandler.getSlotLimit(1)`: its class isn't translated (it implements `ItemHandler` (vectorwing.farmersdelight.refabricated.inventory)) (vectorwing/farmersdelight/common/block/entity/inventory/CookingPotItemHandler.java:73)
- [ ] `CookingPotItemHandler.insert(3)`: its class isn't translated (it implements `ItemHandler` (vectorwing.farmersdelight.refabricated.inventory)) (vectorwing/farmersdelight/common/block/entity/inventory/CookingPotItemHandler.java:78)
- [ ] `CookingPotItemHandler.extract(3)`: its class isn't translated (it implements `ItemHandler` (vectorwing.farmersdelight.refabricated.inventory)) (vectorwing/farmersdelight/common/block/entity/inventory/CookingPotItemHandler.java:94)
- [ ] `CookingPotItemHandler.iterator(0)`: its class isn't translated (it implements `ItemHandler` (vectorwing.farmersdelight.refabricated.inventory)) (vectorwing/farmersdelight/common/block/entity/inventory/CookingPotItemHandler.java:110)
- [ ] `CookingPotItemHandler.getInsertableSlotsFor(1)`: its class isn't translated (it implements `ItemHandler` (vectorwing.farmersdelight.refabricated.inventory)) (vectorwing/farmersdelight/common/block/entity/inventory/CookingPotItemHandler.java:116)
- [ ] `CookingPotItemHandler.getSlotsContaining(1)`: its class isn't translated (it implements `ItemHandler` (vectorwing.farmersdelight.refabricated.inventory)) (vectorwing/farmersdelight/common/block/entity/inventory/CookingPotItemHandler.java:126)

### `SkilletBlockEntity`: it extends `SyncedBlockEntity`

- [ ] `SkilletBlockEntity.SkilletBlockEntity(2)`: its class isn't translated (it extends `SyncedBlockEntity`) (vectorwing/farmersdelight/common/block/entity/SkilletBlockEntity.java:52)
- [ ] `SkilletBlockEntity.cookingTick(4)`: it uses Minecraft's `ServerLevel` (vectorwing/farmersdelight/common/block/entity/SkilletBlockEntity.java:56)
- [ ] `SkilletBlockEntity.animationTick(4)`: it uses Minecraft's `Level` (vectorwing/farmersdelight/common/block/entity/SkilletBlockEntity.java:75)
- [ ] `SkilletBlockEntity.cookAndOutputItems(2)`: its class isn't translated (it extends `SyncedBlockEntity`) (vectorwing/farmersdelight/common/block/entity/SkilletBlockEntity.java:98)
- [ ] `SkilletBlockEntity.isCooking(0)`: its class isn't translated (it extends `SyncedBlockEntity`) (vectorwing/farmersdelight/common/block/entity/SkilletBlockEntity.java:121)
- [ ] `SkilletBlockEntity.isHeated(0)`: its class isn't translated (it extends `SyncedBlockEntity`) (vectorwing/farmersdelight/common/block/entity/SkilletBlockEntity.java:125)
- [ ] `SkilletBlockEntity.getMatchingRecipe(2)`: its class isn't translated (it extends `SyncedBlockEntity`) (vectorwing/farmersdelight/common/block/entity/SkilletBlockEntity.java:129)
- [ ] `SkilletBlockEntity.loadAdditional(1)`: its class isn't translated (it extends `SyncedBlockEntity`) (vectorwing/farmersdelight/common/block/entity/SkilletBlockEntity.java:133)
- [ ] `SkilletBlockEntity.saveAdditional(1)`: its class isn't translated (it extends `SyncedBlockEntity`) (vectorwing/farmersdelight/common/block/entity/SkilletBlockEntity.java:142)
- [ ] `SkilletBlockEntity.preRemoveSideEffects(2)`: its class isn't translated (it extends `SyncedBlockEntity`) (vectorwing/farmersdelight/common/block/entity/SkilletBlockEntity.java:150)
- [ ] `SkilletBlockEntity.getSkilletAsItem(0)`: its class isn't translated (it extends `SyncedBlockEntity`) (vectorwing/farmersdelight/common/block/entity/SkilletBlockEntity.java:158)
- [ ] `SkilletBlockEntity.applyImplicitComponents(1)`: its class isn't translated (it extends `SyncedBlockEntity`) (vectorwing/farmersdelight/common/block/entity/SkilletBlockEntity.java:162)
- [ ] `SkilletBlockEntity.addItemToCook(3)`: its class isn't translated (it extends `SyncedBlockEntity`) (vectorwing/farmersdelight/common/block/entity/SkilletBlockEntity.java:178)
- [ ] `SkilletBlockEntity.removeItem(0)`: its class isn't translated (it extends `SyncedBlockEntity`) (vectorwing/farmersdelight/common/block/entity/SkilletBlockEntity.java:214)
- [ ] `SkilletBlockEntity.getInventory(0)`: its class isn't translated (it extends `SyncedBlockEntity`) (vectorwing/farmersdelight/common/block/entity/SkilletBlockEntity.java:218)
- [ ] `SkilletBlockEntity.getStoredStack(0)`: its class isn't translated (it extends `SyncedBlockEntity`) (vectorwing/farmersdelight/common/block/entity/SkilletBlockEntity.java:222)
- [ ] `SkilletBlockEntity.hasStoredStack(0)`: its class isn't translated (it extends `SyncedBlockEntity`) (vectorwing/farmersdelight/common/block/entity/SkilletBlockEntity.java:226)
- [ ] `SkilletBlockEntity.createHandler(0)`: its body couldn't be read: an anonymous class (vectorwing/farmersdelight/common/block/entity/SkilletBlockEntity.java:230)
- [ ] `SkilletBlockEntity.setRemoved(0)`: its class isn't translated (it extends `SyncedBlockEntity`) (vectorwing/farmersdelight/common/block/entity/SkilletBlockEntity.java:239)

### `StoveBlockEntity`: it extends `AbstractStoveBlockEntity`

- [ ] `StoveBlockEntity.StoveBlockEntity(2)`: its class isn't translated (it extends `AbstractStoveBlockEntity`) (vectorwing/farmersdelight/common/block/entity/StoveBlockEntity.java:15)
- [ ] `StoveBlockEntity.particleTick(4)`: it uses Minecraft's `Level` (vectorwing/farmersdelight/common/block/entity/StoveBlockEntity.java:19)
- [ ] `StoveBlockEntity.addSmokeParticles(0)`: its body couldn't be read: an assert (vectorwing/farmersdelight/common/block/entity/StoveBlockEntity.java:25)
- [ ] `StoveBlockEntity.getInventorySlotCount(0)`: its class isn't translated (it extends `AbstractStoveBlockEntity`) (vectorwing/farmersdelight/common/block/entity/StoveBlockEntity.java:48)
- [ ] `StoveBlockEntity.getStoveItemOffset(1)`: its class isn't translated (it extends `AbstractStoveBlockEntity`) (vectorwing/farmersdelight/common/block/entity/StoveBlockEntity.java:53)

### `SyncedBlockEntity`: it extends Minecraft's `BlockEntity`

- [ ] `SyncedBlockEntity.SyncedBlockEntity(3)`: its class isn't translated (it extends Minecraft's `BlockEntity`) (vectorwing/farmersdelight/common/block/entity/SyncedBlockEntity.java:13)
- [ ] `SyncedBlockEntity.getUpdatePacket(0)`: its class isn't translated (it extends Minecraft's `BlockEntity`) (vectorwing/farmersdelight/common/block/entity/SyncedBlockEntity.java:17)
- [ ] `SyncedBlockEntity.getUpdateTag(1)`: its class isn't translated (it extends Minecraft's `BlockEntity`) (vectorwing/farmersdelight/common/block/entity/SyncedBlockEntity.java:21)
- [ ] `SyncedBlockEntity.inventoryChanged(0)`: its class isn't translated (it extends Minecraft's `BlockEntity`) (vectorwing/farmersdelight/common/block/entity/SyncedBlockEntity.java:25)

### `FeastBlock`: it extends Minecraft's `Block`

- [ ] `FeastBlock.FACING`: it uses Minecraft's `Property` (vectorwing/farmersdelight/common/block/FeastBlock.java:43)
- [ ] `FeastBlock.SERVINGS`: it uses Minecraft's `IntegerProperty` (vectorwing/farmersdelight/common/block/FeastBlock.java:44)
- [ ] `FeastBlock.SHAPES`: it uses Minecraft's `VoxelShape` (vectorwing/farmersdelight/common/block/FeastBlock.java:48)
- [ ] `FeastBlock.FeastBlock(4)`: its class isn't translated (it extends Minecraft's `Block`) (vectorwing/farmersdelight/common/block/FeastBlock.java:56)
- [ ] `FeastBlock.FeastBlock(4)`: its class isn't translated (it extends Minecraft's `Block`) (vectorwing/farmersdelight/common/block/FeastBlock.java:67)
- [ ] `FeastBlock.FeastBlock(3)`: its class isn't translated (it extends Minecraft's `Block`) (vectorwing/farmersdelight/common/block/FeastBlock.java:78)
- [ ] `FeastBlock.getServingsProperty(0)`: its class isn't translated (it extends Minecraft's `Block`) (vectorwing/farmersdelight/common/block/FeastBlock.java:82)
- [ ] `FeastBlock.getMaxServings(0)`: its class isn't translated (it extends Minecraft's `Block`) (vectorwing/farmersdelight/common/block/FeastBlock.java:86)
- [ ] `FeastBlock.getServingItem(1)`: its class isn't translated (it extends Minecraft's `Block`) (vectorwing/farmersdelight/common/block/FeastBlock.java:90)
- [ ] `FeastBlock.getShape(4)`: its class isn't translated (it extends Minecraft's `Block`) (vectorwing/farmersdelight/common/block/FeastBlock.java:94)
- [ ] `FeastBlock.useItemOn(7)`: its class isn't translated (it extends Minecraft's `Block`) (vectorwing/farmersdelight/common/block/FeastBlock.java:98)
- [ ] `FeastBlock.takeServing(5)`: its class isn't translated (it extends Minecraft's `Block`) (vectorwing/farmersdelight/common/block/FeastBlock.java:104)
- [ ] `FeastBlock.getStateForPlacement(1)`: its class isn't translated (it extends Minecraft's `Block`) (vectorwing/farmersdelight/common/block/FeastBlock.java:145)
- [ ] `FeastBlock.updateShape(8)`: its class isn't translated (it extends Minecraft's `Block`) (vectorwing/farmersdelight/common/block/FeastBlock.java:149)
- [ ] `FeastBlock.canSurvive(3)`: its class isn't translated (it extends Minecraft's `Block`) (vectorwing/farmersdelight/common/block/FeastBlock.java:164)
- [ ] `FeastBlock.createBlockStateDefinition(1)`: its class isn't translated (it extends Minecraft's `Block`) (vectorwing/farmersdelight/common/block/FeastBlock.java:168)
- [ ] `FeastBlock.getAnalogOutputSignal(4)`: its class isn't translated (it extends Minecraft's `Block`) (vectorwing/farmersdelight/common/block/FeastBlock.java:172)
- [ ] `FeastBlock.hasAnalogOutputSignal(1)`: its class isn't translated (it extends Minecraft's `Block`) (vectorwing/farmersdelight/common/block/FeastBlock.java:176)
- [ ] `FeastBlock.isPathfindable(2)`: its class isn't translated (it extends Minecraft's `Block`) (vectorwing/farmersdelight/common/block/FeastBlock.java:180)

### `GleamingSaladBlock`: it extends `FeastBlock`

- [ ] `GleamingSaladBlock.SHAPE`: it uses Minecraft's `VoxelShape` (vectorwing/farmersdelight/common/block/GleamingSaladBlock.java:21)
- [ ] `GleamingSaladBlock.GleamingSaladBlock(3)`: its class isn't translated (it extends `FeastBlock`) (vectorwing/farmersdelight/common/block/GleamingSaladBlock.java:23)
- [ ] `GleamingSaladBlock.GleamingSaladBlock(3)`: its class isn't translated (it extends `FeastBlock`) (vectorwing/farmersdelight/common/block/GleamingSaladBlock.java:27)
- [ ] `GleamingSaladBlock.getShape(4)`: its class isn't translated (it extends `FeastBlock`) (vectorwing/farmersdelight/common/block/GleamingSaladBlock.java:31)
- [ ] `GleamingSaladBlock.animateTick(4)`: its class isn't translated (it extends `FeastBlock`) (vectorwing/farmersdelight/common/block/GleamingSaladBlock.java:36)

### `HangingTomatoBlock`: it extends `TomatoBlock`

- [ ] `HangingTomatoBlock.HangingTomatoBlock(1)`: its class isn't translated (it extends `TomatoBlock`) (vectorwing/farmersdelight/common/block/HangingTomatoBlock.java:20)
- [ ] `HangingTomatoBlock.tick(4)`: its class isn't translated (it extends `TomatoBlock`) (vectorwing/farmersdelight/common/block/HangingTomatoBlock.java:25)
- [ ] `HangingTomatoBlock.onRemove(5)`: its class isn't translated (it extends `TomatoBlock`) (vectorwing/farmersdelight/common/block/HangingTomatoBlock.java:32)
- [ ] `HangingTomatoBlock.placeRope(2)`: it uses Minecraft's `LevelAccessor` (vectorwing/farmersdelight/common/block/HangingTomatoBlock.java:39)
- [ ] `HangingTomatoBlock.createBlockStateDefinition(1)`: its class isn't translated (it extends `TomatoBlock`) (vectorwing/farmersdelight/common/block/HangingTomatoBlock.java:51)

### `MushroomColonyBlock`: it extends Minecraft's `VegetationBlock`

- [x] `MushroomColonyBlock.PLACING_LIGHT_LEVEL` (vectorwing/farmersdelight/common/block/MushroomColonyBlock.java:41)
- [ ] `MushroomColonyBlock.COLONY_AGE`: it uses Minecraft's `IntegerProperty` (vectorwing/farmersdelight/common/block/MushroomColonyBlock.java:43)
- [ ] `MushroomColonyBlock.SHAPE_BY_AGE`: it uses Minecraft's `VoxelShape` (vectorwing/farmersdelight/common/block/MushroomColonyBlock.java:44)
- [ ] `MushroomColonyBlock.MushroomColonyBlock(2)`: its class isn't translated (it extends Minecraft's `VegetationBlock`) (vectorwing/farmersdelight/common/block/MushroomColonyBlock.java:51)
- [ ] `MushroomColonyBlock.useItemOn(7)`: its class isn't translated (it extends Minecraft's `VegetationBlock`) (vectorwing/farmersdelight/common/block/MushroomColonyBlock.java:57)
- [ ] `MushroomColonyBlock.getShape(4)`: its class isn't translated (it extends Minecraft's `VegetationBlock`) (vectorwing/farmersdelight/common/block/MushroomColonyBlock.java:97)
- [ ] `MushroomColonyBlock.getAgeProperty(0)`: its class isn't translated (it extends Minecraft's `VegetationBlock`) (vectorwing/farmersdelight/common/block/MushroomColonyBlock.java:101)
- [ ] `MushroomColonyBlock.mayPlaceOn(3)`: its class isn't translated (it extends Minecraft's `VegetationBlock`) (vectorwing/farmersdelight/common/block/MushroomColonyBlock.java:105)
- [ ] `MushroomColonyBlock.canSurvive(3)`: its class isn't translated (it extends Minecraft's `VegetationBlock`) (vectorwing/farmersdelight/common/block/MushroomColonyBlock.java:109)
- [ ] `MushroomColonyBlock.getMaxAge(0)`: its class isn't translated (it extends Minecraft's `VegetationBlock`) (vectorwing/farmersdelight/common/block/MushroomColonyBlock.java:117)
- [ ] `MushroomColonyBlock.randomTick(4)`: its class isn't translated (it extends Minecraft's `VegetationBlock`) (vectorwing/farmersdelight/common/block/MushroomColonyBlock.java:121)
- [ ] `MushroomColonyBlock.getCloneItemStack(4)`: its class isn't translated (it extends Minecraft's `VegetationBlock`) (vectorwing/farmersdelight/common/block/MushroomColonyBlock.java:129)
- [ ] `MushroomColonyBlock.createBlockStateDefinition(1)`: its class isn't translated (it extends Minecraft's `VegetationBlock`) (vectorwing/farmersdelight/common/block/MushroomColonyBlock.java:133)
- [ ] `MushroomColonyBlock.isValidBonemealTarget(4)`: its class isn't translated (it extends Minecraft's `VegetationBlock`) (vectorwing/farmersdelight/common/block/MushroomColonyBlock.java:137)
- [ ] `MushroomColonyBlock.isBonemealSuccess(5)`: its class isn't translated (it extends Minecraft's `VegetationBlock`) (vectorwing/farmersdelight/common/block/MushroomColonyBlock.java:141)
- [ ] `MushroomColonyBlock.getBonemealAgeIncrease(1)`: its class isn't translated (it extends Minecraft's `VegetationBlock`) (vectorwing/farmersdelight/common/block/MushroomColonyBlock.java:145)
- [ ] `MushroomColonyBlock.performBonemeal(5)`: its class isn't translated (it extends Minecraft's `VegetationBlock`) (vectorwing/farmersdelight/common/block/MushroomColonyBlock.java:149)

### `OnionBlock`: it extends Minecraft's `CropBlock`

- [ ] `OnionBlock.SHAPE_BY_AGE`: it uses Minecraft's `VoxelShape` (vectorwing/farmersdelight/common/block/OnionBlock.java:15)
- [ ] `OnionBlock.OnionBlock(1)`: its class isn't translated (it extends Minecraft's `CropBlock`) (vectorwing/farmersdelight/common/block/OnionBlock.java:26)
- [ ] `OnionBlock.getBaseSeedId(0)`: its class isn't translated (it extends Minecraft's `CropBlock`) (vectorwing/farmersdelight/common/block/OnionBlock.java:30)
- [ ] `OnionBlock.getShape(4)`: its class isn't translated (it extends Minecraft's `CropBlock`) (vectorwing/farmersdelight/common/block/OnionBlock.java:34)

### `OrganicCompostBlock`: it extends Minecraft's `Block`

- [ ] `OrganicCompostBlock.COMPOSTING`: it uses Minecraft's `IntegerProperty` (vectorwing/farmersdelight/common/block/OrganicCompostBlock.java:21)
- [ ] `OrganicCompostBlock.OrganicCompostBlock(1)`: its class isn't translated (it extends Minecraft's `Block`) (vectorwing/farmersdelight/common/block/OrganicCompostBlock.java:23)
- [ ] `OrganicCompostBlock.isRandomlyTicking(1)`: its class isn't translated (it extends Minecraft's `Block`) (vectorwing/farmersdelight/common/block/OrganicCompostBlock.java:28)
- [ ] `OrganicCompostBlock.createBlockStateDefinition(1)`: its class isn't translated (it extends Minecraft's `Block`) (vectorwing/farmersdelight/common/block/OrganicCompostBlock.java:32)
- [ ] `OrganicCompostBlock.getMaxCompostingStage(0)`: its class isn't translated (it extends Minecraft's `Block`) (vectorwing/farmersdelight/common/block/OrganicCompostBlock.java:37)
- [ ] `OrganicCompostBlock.randomTick(4)`: its class isn't translated (it extends Minecraft's `Block`) (vectorwing/farmersdelight/common/block/OrganicCompostBlock.java:41)
- [ ] `OrganicCompostBlock.hasAnalogOutputSignal(1)`: its class isn't translated (it extends Minecraft's `Block`) (vectorwing/farmersdelight/common/block/OrganicCompostBlock.java:75)
- [ ] `OrganicCompostBlock.getAnalogOutputSignal(4)`: its class isn't translated (it extends Minecraft's `Block`) (vectorwing/farmersdelight/common/block/OrganicCompostBlock.java:79)
- [ ] `OrganicCompostBlock.animateTick(4)`: its class isn't translated (it extends Minecraft's `Block`) (vectorwing/farmersdelight/common/block/OrganicCompostBlock.java:83)

### `PieBlock`: it extends Minecraft's `Block`

- [ ] `PieBlock.FACING`: it uses Minecraft's `Property` (vectorwing/farmersdelight/common/block/PieBlock.java:49)
- [ ] `PieBlock.BITES`: it uses Minecraft's `IntegerProperty` (vectorwing/farmersdelight/common/block/PieBlock.java:50)
- [ ] `PieBlock.SHAPES`: it uses Minecraft's `VoxelShape` (vectorwing/farmersdelight/common/block/PieBlock.java:51)
- [ ] `PieBlock.ROTATED_SHAPES`: it uses Minecraft's `VoxelShape` (vectorwing/farmersdelight/common/block/PieBlock.java:57)
- [ ] `PieBlock.PieBlock(2)`: its class isn't translated (it extends Minecraft's `Block`) (vectorwing/farmersdelight/common/block/PieBlock.java:60)
- [ ] `PieBlock.getPieSliceItem(0)`: its class isn't translated (it extends Minecraft's `Block`) (vectorwing/farmersdelight/common/block/PieBlock.java:66)
- [ ] `PieBlock.getMaxBites(0)`: its class isn't translated (it extends Minecraft's `Block`) (vectorwing/farmersdelight/common/block/PieBlock.java:70)
- [ ] `PieBlock.getShape(4)`: its class isn't translated (it extends Minecraft's `Block`) (vectorwing/farmersdelight/common/block/PieBlock.java:74)
- [ ] `PieBlock.buildShapes(0)`: it uses Minecraft's `VoxelShape` (vectorwing/farmersdelight/common/block/PieBlock.java:78)
- [ ] `PieBlock.getStateForPlacement(1)`: its class isn't translated (it extends Minecraft's `Block`) (vectorwing/farmersdelight/common/block/PieBlock.java:92)
- [ ] `PieBlock.useItemOn(7)`: its class isn't translated (it extends Minecraft's `Block`) (vectorwing/farmersdelight/common/block/PieBlock.java:96)
- [ ] `PieBlock.useWithoutItem(5)`: its class isn't translated (it extends Minecraft's `Block`) (vectorwing/farmersdelight/common/block/PieBlock.java:102)
- [ ] `PieBlock.consumeBite(4)`: its class isn't translated (it extends Minecraft's `Block`) (vectorwing/farmersdelight/common/block/PieBlock.java:116)
- [ ] `PieBlock.cutSlice(5)`: its class isn't translated (it extends Minecraft's `Block`) (vectorwing/farmersdelight/common/block/PieBlock.java:148)
- [ ] `PieBlock.updateShape(8)`: its class isn't translated (it extends Minecraft's `Block`) (vectorwing/farmersdelight/common/block/PieBlock.java:171)
- [ ] `PieBlock.canSurvive(3)`: its class isn't translated (it extends Minecraft's `Block`) (vectorwing/farmersdelight/common/block/PieBlock.java:186)
- [ ] `PieBlock.createBlockStateDefinition(1)`: its class isn't translated (it extends Minecraft's `Block`) (vectorwing/farmersdelight/common/block/PieBlock.java:190)
- [ ] `PieBlock.getAnalogOutputSignal(4)`: its class isn't translated (it extends Minecraft's `Block`) (vectorwing/farmersdelight/common/block/PieBlock.java:194)
- [ ] `PieBlock.hasAnalogOutputSignal(1)`: its class isn't translated (it extends Minecraft's `Block`) (vectorwing/farmersdelight/common/block/PieBlock.java:198)
- [ ] `PieBlock.isPathfindable(2)`: its class isn't translated (it extends Minecraft's `Block`) (vectorwing/farmersdelight/common/block/PieBlock.java:202)

### `RiceBaleBlock`: it extends Minecraft's `Block`

- [ ] `RiceBaleBlock.FACING`: it uses Minecraft's `Property` (vectorwing/farmersdelight/common/block/RiceBaleBlock.java:20)
- [ ] `RiceBaleBlock.RiceBaleBlock(1)`: its class isn't translated (it extends Minecraft's `Block`) (vectorwing/farmersdelight/common/block/RiceBaleBlock.java:22)
- [ ] `RiceBaleBlock.fallOn(5)`: its class isn't translated (it extends Minecraft's `Block`) (vectorwing/farmersdelight/common/block/RiceBaleBlock.java:28)
- [ ] `RiceBaleBlock.getStateForPlacement(1)`: its class isn't translated (it extends Minecraft's `Block`) (vectorwing/farmersdelight/common/block/RiceBaleBlock.java:32)
- [ ] `RiceBaleBlock.getFireSpreadSpeed(4)`: its class isn't translated (it extends Minecraft's `Block`) (vectorwing/farmersdelight/common/block/RiceBaleBlock.java:36)
- [ ] `RiceBaleBlock.getFlammability(4)`: its class isn't translated (it extends Minecraft's `Block`) (vectorwing/farmersdelight/common/block/RiceBaleBlock.java:40)
- [ ] `RiceBaleBlock.createBlockStateDefinition(1)`: its class isn't translated (it extends Minecraft's `Block`) (vectorwing/farmersdelight/common/block/RiceBaleBlock.java:44)
- [ ] `RiceBaleBlock.rotate(2)`: its class isn't translated (it extends Minecraft's `Block`) (vectorwing/farmersdelight/common/block/RiceBaleBlock.java:48)
- [ ] `RiceBaleBlock.mirror(2)`: its class isn't translated (it extends Minecraft's `Block`) (vectorwing/farmersdelight/common/block/RiceBaleBlock.java:52)

### `RiceBlock`: it extends Minecraft's `VegetationBlock`

- [ ] `RiceBlock.AGE`: it uses Minecraft's `IntegerProperty` (vectorwing/farmersdelight/common/block/RiceBlock.java:41)
- [ ] `RiceBlock.SUPPORTING`: it uses Minecraft's `BooleanProperty` (vectorwing/farmersdelight/common/block/RiceBlock.java:42)
- [ ] `RiceBlock.SHAPE_BY_AGE`: it uses Minecraft's `VoxelShape` (vectorwing/farmersdelight/common/block/RiceBlock.java:43)
- [ ] `RiceBlock.RiceBlock(1)`: its class isn't translated (it extends Minecraft's `VegetationBlock`) (vectorwing/farmersdelight/common/block/RiceBlock.java:50)
- [ ] `RiceBlock.randomTick(4)`: its class isn't translated (it extends Minecraft's `VegetationBlock`) (vectorwing/farmersdelight/common/block/RiceBlock.java:55)
- [ ] `RiceBlock.getShape(4)`: its class isn't translated (it extends Minecraft's `VegetationBlock`) (vectorwing/farmersdelight/common/block/RiceBlock.java:77)
- [ ] `RiceBlock.canSurvive(3)`: its class isn't translated (it extends Minecraft's `VegetationBlock`) (vectorwing/farmersdelight/common/block/RiceBlock.java:81)
- [ ] `RiceBlock.mayPlaceOn(3)`: its class isn't translated (it extends Minecraft's `VegetationBlock`) (vectorwing/farmersdelight/common/block/RiceBlock.java:86)
- [ ] `RiceBlock.getAgeProperty(0)`: its class isn't translated (it extends Minecraft's `VegetationBlock`) (vectorwing/farmersdelight/common/block/RiceBlock.java:90)
- [ ] `RiceBlock.getAge(1)`: its class isn't translated (it extends Minecraft's `VegetationBlock`) (vectorwing/farmersdelight/common/block/RiceBlock.java:94)
- [ ] `RiceBlock.getMaxAge(0)`: its class isn't translated (it extends Minecraft's `VegetationBlock`) (vectorwing/farmersdelight/common/block/RiceBlock.java:98)
- [ ] `RiceBlock.getCloneItemStack(4)`: its class isn't translated (it extends Minecraft's `VegetationBlock`) (vectorwing/farmersdelight/common/block/RiceBlock.java:102)
- [ ] `RiceBlock.withAge(1)`: its class isn't translated (it extends Minecraft's `VegetationBlock`) (vectorwing/farmersdelight/common/block/RiceBlock.java:106)
- [ ] `RiceBlock.createBlockStateDefinition(1)`: its class isn't translated (it extends Minecraft's `VegetationBlock`) (vectorwing/farmersdelight/common/block/RiceBlock.java:110)
- [ ] `RiceBlock.updateShape(8)`: its class isn't translated (it extends Minecraft's `VegetationBlock`) (vectorwing/farmersdelight/common/block/RiceBlock.java:114)
- [ ] `RiceBlock.isSupportingRiceUpper(1)`: its class isn't translated (it extends Minecraft's `VegetationBlock`) (vectorwing/farmersdelight/common/block/RiceBlock.java:135)
- [ ] `RiceBlock.getStateForPlacement(1)`: its class isn't translated (it extends Minecraft's `VegetationBlock`) (vectorwing/farmersdelight/common/block/RiceBlock.java:139)
- [ ] `RiceBlock.isValidBonemealTarget(4)`: its class isn't translated (it extends Minecraft's `VegetationBlock`) (vectorwing/farmersdelight/common/block/RiceBlock.java:144)
- [ ] `RiceBlock.isBonemealSuccess(5)`: its class isn't translated (it extends Minecraft's `VegetationBlock`) (vectorwing/farmersdelight/common/block/RiceBlock.java:149)
- [ ] `RiceBlock.getBonemealAgeIncrease(1)`: its class isn't translated (it extends Minecraft's `VegetationBlock`) (vectorwing/farmersdelight/common/block/RiceBlock.java:153)
- [ ] `RiceBlock.performBonemeal(5)`: its class isn't translated (it extends Minecraft's `VegetationBlock`) (vectorwing/farmersdelight/common/block/RiceBlock.java:157)
- [ ] `RiceBlock.getFluidState(1)`: its class isn't translated (it extends Minecraft's `VegetationBlock`) (vectorwing/farmersdelight/common/block/RiceBlock.java:179)
- [ ] `RiceBlock.canPlaceLiquid(5)`: its class isn't translated (it extends Minecraft's `VegetationBlock`) (vectorwing/farmersdelight/common/block/RiceBlock.java:183)
- [ ] `RiceBlock.placeLiquid(4)`: its class isn't translated (it extends Minecraft's `VegetationBlock`) (vectorwing/farmersdelight/common/block/RiceBlock.java:187)

### `RicePaniclesBlock`: it extends Minecraft's `CropBlock`

- [ ] `RicePaniclesBlock.RICE_AGE`: it uses Minecraft's `IntegerProperty` (vectorwing/farmersdelight/common/block/RicePaniclesBlock.java:22)
- [ ] `RicePaniclesBlock.SHAPE_BY_AGE`: it uses Minecraft's `VoxelShape` (vectorwing/farmersdelight/common/block/RicePaniclesBlock.java:23)
- [ ] `RicePaniclesBlock.RicePaniclesBlock(1)`: its class isn't translated (it extends Minecraft's `CropBlock`) (vectorwing/farmersdelight/common/block/RicePaniclesBlock.java:30)
- [ ] `RicePaniclesBlock.getAgeProperty(0)`: its class isn't translated (it extends Minecraft's `CropBlock`) (vectorwing/farmersdelight/common/block/RicePaniclesBlock.java:34)
- [ ] `RicePaniclesBlock.getShape(4)`: its class isn't translated (it extends Minecraft's `CropBlock`) (vectorwing/farmersdelight/common/block/RicePaniclesBlock.java:38)
- [ ] `RicePaniclesBlock.getMaxAge(0)`: its class isn't translated (it extends Minecraft's `CropBlock`) (vectorwing/farmersdelight/common/block/RicePaniclesBlock.java:42)
- [ ] `RicePaniclesBlock.getBaseSeedId(0)`: its class isn't translated (it extends Minecraft's `CropBlock`) (vectorwing/farmersdelight/common/block/RicePaniclesBlock.java:46)
- [ ] `RicePaniclesBlock.createBlockStateDefinition(1)`: its class isn't translated (it extends Minecraft's `CropBlock`) (vectorwing/farmersdelight/common/block/RicePaniclesBlock.java:50)
- [ ] `RicePaniclesBlock.getBonemealAgeIncrease(1)`: its class isn't translated (it extends Minecraft's `CropBlock`) (vectorwing/farmersdelight/common/block/RicePaniclesBlock.java:54)
- [ ] `RicePaniclesBlock.mayPlaceOn(3)`: its class isn't translated (it extends Minecraft's `CropBlock`) (vectorwing/farmersdelight/common/block/RicePaniclesBlock.java:58)
- [ ] `RicePaniclesBlock.canSurvive(3)`: its class isn't translated (it extends Minecraft's `CropBlock`) (vectorwing/farmersdelight/common/block/RicePaniclesBlock.java:62)

### `RiceRollMedleyBlock`: it extends `FeastBlock`

- [ ] `RiceRollMedleyBlock.ROLL_SERVINGS`: it uses Minecraft's `IntegerProperty` (vectorwing/farmersdelight/common/block/RiceRollMedleyBlock.java:26)
- [ ] `RiceRollMedleyBlock.SHAPE_NORTH_SOUTH`: it uses Minecraft's `VoxelShape` (vectorwing/farmersdelight/common/block/RiceRollMedleyBlock.java:27)
- [ ] `RiceRollMedleyBlock.SHAPE_EAST_WEST`: it uses Minecraft's `VoxelShape` (vectorwing/farmersdelight/common/block/RiceRollMedleyBlock.java:30)
- [ ] `RiceRollMedleyBlock.RiceRollMedleyBlock(1)`: its class isn't translated (it extends `FeastBlock`) (vectorwing/farmersdelight/common/block/RiceRollMedleyBlock.java:46)
- [ ] `RiceRollMedleyBlock.getServingsProperty(0)`: its class isn't translated (it extends `FeastBlock`) (vectorwing/farmersdelight/common/block/RiceRollMedleyBlock.java:50)
- [ ] `RiceRollMedleyBlock.getMaxServings(0)`: its class isn't translated (it extends `FeastBlock`) (vectorwing/farmersdelight/common/block/RiceRollMedleyBlock.java:55)
- [ ] `RiceRollMedleyBlock.getServingItem(1)`: its class isn't translated (it extends `FeastBlock`) (vectorwing/farmersdelight/common/block/RiceRollMedleyBlock.java:60)
- [ ] `RiceRollMedleyBlock.getShape(4)`: its class isn't translated (it extends `FeastBlock`) (vectorwing/farmersdelight/common/block/RiceRollMedleyBlock.java:65)
- [ ] `RiceRollMedleyBlock.createBlockStateDefinition(1)`: its class isn't translated (it extends `FeastBlock`) (vectorwing/farmersdelight/common/block/RiceRollMedleyBlock.java:70)

### `RichSoilBlock`: it extends Minecraft's `Block`

- [ ] `RichSoilBlock.RichSoilBlock(1)`: its class isn't translated (it extends Minecraft's `Block`) (vectorwing/farmersdelight/common/block/RichSoilBlock.java:21)
- [ ] `RichSoilBlock.init(0)`: it uses ModBlocks.RICH_SOIL, which isn't translated (it uses Minecraft's `Block`) (vectorwing/farmersdelight/common/block/RichSoilBlock.java:25)
- [ ] `RichSoilBlock.randomTick(4)`: its class isn't translated (it extends Minecraft's `Block`) (vectorwing/farmersdelight/common/block/RichSoilBlock.java:29)
- [ ] `RichSoilBlock.tryBoostingPlantsAboveAndBelow(3)`: it uses Minecraft's `ServerLevel` (vectorwing/farmersdelight/common/block/RichSoilBlock.java:37)
- [ ] `RichSoilBlock.boostPlant(4)`: it uses Minecraft's `BlockState` (vectorwing/farmersdelight/common/block/RichSoilBlock.java:51)
- [ ] `RichSoilBlock.convertMushroomToColony(3)`: its class isn't translated (it extends Minecraft's `Block`) (vectorwing/farmersdelight/common/block/RichSoilBlock.java:69)

### `RichSoilFarmlandBlock`: it extends Minecraft's `FarmlandBlock`

- [ ] `RichSoilFarmlandBlock.RichSoilFarmlandBlock(1)`: its class isn't translated (it extends Minecraft's `FarmlandBlock`) (vectorwing/farmersdelight/common/block/RichSoilFarmlandBlock.java:29)
- [ ] `RichSoilFarmlandBlock.isNearWater(2)`: it uses Minecraft's `LevelReader` (vectorwing/farmersdelight/common/block/RichSoilFarmlandBlock.java:33)
- [ ] `RichSoilFarmlandBlock.turnToRichSoil(4)`: it uses Minecraft's `Entity` (vectorwing/farmersdelight/common/block/RichSoilFarmlandBlock.java:43)
- [ ] `RichSoilFarmlandBlock.canSurvive(3)`: its class isn't translated (it extends Minecraft's `FarmlandBlock`) (vectorwing/farmersdelight/common/block/RichSoilFarmlandBlock.java:48)
- [ ] `RichSoilFarmlandBlock.tick(4)`: its class isn't translated (it extends Minecraft's `FarmlandBlock`) (vectorwing/farmersdelight/common/block/RichSoilFarmlandBlock.java:53)
- [ ] `RichSoilFarmlandBlock.randomTick(4)`: its class isn't translated (it extends Minecraft's `FarmlandBlock`) (vectorwing/farmersdelight/common/block/RichSoilFarmlandBlock.java:59)
- [ ] `RichSoilFarmlandBlock.getStateForPlacement(1)`: its class isn't translated (it extends Minecraft's `FarmlandBlock`) (vectorwing/farmersdelight/common/block/RichSoilFarmlandBlock.java:88)
- [ ] `RichSoilFarmlandBlock.fallOn(5)`: its class isn't translated (it extends Minecraft's `FarmlandBlock`) (vectorwing/farmersdelight/common/block/RichSoilFarmlandBlock.java:94)

### `RopeBlock`: it extends Minecraft's `IronBarsBlock`

- [ ] `RopeBlock.TIED_TO_BELL`: it uses Minecraft's `BooleanProperty` (vectorwing/farmersdelight/common/block/RopeBlock.java:40)
- [ ] `RopeBlock.LOWER_SUPPORT_AABB`: it uses Minecraft's `VoxelShape` (vectorwing/farmersdelight/common/block/RopeBlock.java:41)
- [ ] `RopeBlock.RopeBlock(1)`: its class isn't translated (it extends Minecraft's `IronBarsBlock`) (vectorwing/farmersdelight/common/block/RopeBlock.java:43)
- [ ] `RopeBlock.isPathfindable(2)`: its class isn't translated (it extends Minecraft's `IronBarsBlock`) (vectorwing/farmersdelight/common/block/RopeBlock.java:56)
- [ ] `RopeBlock.useItemOn(7)`: its class isn't translated (it extends Minecraft's `IronBarsBlock`) (vectorwing/farmersdelight/common/block/RopeBlock.java:60)
- [ ] `RopeBlock.useWithoutItem(5)`: its class isn't translated (it extends Minecraft's `IronBarsBlock`) (vectorwing/farmersdelight/common/block/RopeBlock.java:66)
- [ ] `RopeBlock.getStateForPlacement(1)`: its class isn't translated (it extends Minecraft's `IronBarsBlock`) (vectorwing/farmersdelight/common/block/RopeBlock.java:105)
- [ ] `RopeBlock.getStateWithConnections(4)`: it uses Minecraft's `BlockState` (vectorwing/farmersdelight/common/block/RopeBlock.java:109)
- [ ] `RopeBlock.tieToAnythingValid(2)`: it uses Minecraft's `BlockState` (vectorwing/farmersdelight/common/block/RopeBlock.java:146)
- [ ] `RopeBlock.tieToRopeAndWalls(1)`: it uses Minecraft's `BlockState` (vectorwing/farmersdelight/common/block/RopeBlock.java:150)
- [ ] `RopeBlock.getCollisionShape(4)`: its class isn't translated (it extends Minecraft's `IronBarsBlock`) (vectorwing/farmersdelight/common/block/RopeBlock.java:154)
- [ ] `RopeBlock.getBlockSupportShape(3)`: its class isn't translated (it extends Minecraft's `IronBarsBlock`) (vectorwing/farmersdelight/common/block/RopeBlock.java:158)
- [ ] `RopeBlock.canBeReplaced(2)`: its class isn't translated (it extends Minecraft's `IronBarsBlock`) (vectorwing/farmersdelight/common/block/RopeBlock.java:162)
- [ ] `RopeBlock.updateShape(8)`: its class isn't translated (it extends Minecraft's `IronBarsBlock`) (vectorwing/farmersdelight/common/block/RopeBlock.java:166)
- [ ] `RopeBlock.createBlockStateDefinition(1)`: its class isn't translated (it extends Minecraft's `IronBarsBlock`) (vectorwing/farmersdelight/common/block/RopeBlock.java:196)

### `RopeFenceBlock`: it extends Minecraft's `CrossCollisionBlock`

- [ ] `RopeFenceBlock.POST`: it uses Minecraft's `VoxelShape` (vectorwing/farmersdelight/common/block/RopeFenceBlock.java:33)
- [ ] `RopeFenceBlock.RopeFenceBlock(1)`: its class isn't translated (it extends Minecraft's `CrossCollisionBlock`) (vectorwing/farmersdelight/common/block/RopeFenceBlock.java:35)
- [ ] `RopeFenceBlock.useItemOn(7)`: its class isn't translated (it extends Minecraft's `CrossCollisionBlock`) (vectorwing/farmersdelight/common/block/RopeFenceBlock.java:45)
- [ ] `RopeFenceBlock.getVisualShape(4)`: its class isn't translated (it extends Minecraft's `CrossCollisionBlock`) (vectorwing/farmersdelight/common/block/RopeFenceBlock.java:56)
- [ ] `RopeFenceBlock.getOcclusionShape(1)`: its class isn't translated (it extends Minecraft's `CrossCollisionBlock`) (vectorwing/farmersdelight/common/block/RopeFenceBlock.java:60)
- [ ] `RopeFenceBlock.connectsTo(3)`: its class isn't translated (it extends Minecraft's `CrossCollisionBlock`) (vectorwing/farmersdelight/common/block/RopeFenceBlock.java:64)
- [ ] `RopeFenceBlock.isSameFence(1)`: its class isn't translated (it extends Minecraft's `CrossCollisionBlock`) (vectorwing/farmersdelight/common/block/RopeFenceBlock.java:69)
- [ ] `RopeFenceBlock.getStateForPlacement(1)`: its class isn't translated (it extends Minecraft's `CrossCollisionBlock`) (vectorwing/farmersdelight/common/block/RopeFenceBlock.java:73)
- [ ] `RopeFenceBlock.updateShape(8)`: its class isn't translated (it extends Minecraft's `CrossCollisionBlock`) (vectorwing/farmersdelight/common/block/RopeFenceBlock.java:93)
- [ ] `RopeFenceBlock.createBlockStateDefinition(1)`: its class isn't translated (it extends Minecraft's `CrossCollisionBlock`) (vectorwing/farmersdelight/common/block/RopeFenceBlock.java:117)

### `RopeFenceGateBlock`: it extends Minecraft's `FenceGateBlock`

- [ ] `RopeFenceGateBlock.Z_SHAPE`: it uses Minecraft's `VoxelShape` (vectorwing/farmersdelight/common/block/RopeFenceGateBlock.java:21)
- [ ] `RopeFenceGateBlock.X_SHAPE`: it uses Minecraft's `VoxelShape` (vectorwing/farmersdelight/common/block/RopeFenceGateBlock.java:22)
- [ ] `RopeFenceGateBlock.Z_COLLISION_SHAPE`: it uses Minecraft's `VoxelShape` (vectorwing/farmersdelight/common/block/RopeFenceGateBlock.java:23)
- [ ] `RopeFenceGateBlock.X_COLLISION_SHAPE`: it uses Minecraft's `VoxelShape` (vectorwing/farmersdelight/common/block/RopeFenceGateBlock.java:24)
- [ ] `RopeFenceGateBlock.RopeFenceGateBlock(1)`: its class isn't translated (it extends Minecraft's `FenceGateBlock`) (vectorwing/farmersdelight/common/block/RopeFenceGateBlock.java:26)
- [ ] `RopeFenceGateBlock.updateShape(8)`: its class isn't translated (it extends Minecraft's `FenceGateBlock`) (vectorwing/farmersdelight/common/block/RopeFenceGateBlock.java:30)
- [ ] `RopeFenceGateBlock.isWall(1)`: its class isn't translated (it extends Minecraft's `FenceGateBlock`) (vectorwing/farmersdelight/common/block/RopeFenceGateBlock.java:49)
- [ ] `RopeFenceGateBlock.getShape(4)`: its class isn't translated (it extends Minecraft's `FenceGateBlock`) (vectorwing/farmersdelight/common/block/RopeFenceGateBlock.java:53)
- [ ] `RopeFenceGateBlock.getCollisionShape(4)`: its class isn't translated (it extends Minecraft's `FenceGateBlock`) (vectorwing/farmersdelight/common/block/RopeFenceGateBlock.java:57)

### `RotatedFeastBlock`: it extends `FeastBlock`

- [ ] `RotatedFeastBlock.RotatedFeastBlock(5)`: its class isn't translated (it extends `FeastBlock`) (vectorwing/farmersdelight/common/block/RotatedFeastBlock.java:19)
- [ ] `RotatedFeastBlock.RotatedFeastBlock(5)`: its class isn't translated (it extends `FeastBlock`) (vectorwing/farmersdelight/common/block/RotatedFeastBlock.java:28)
- [ ] `RotatedFeastBlock.getShape(4)`: its class isn't translated (it extends `FeastBlock`) (vectorwing/farmersdelight/common/block/RotatedFeastBlock.java:37)

### `SafetyNetBlock`: it extends Minecraft's `Block`

- [ ] `SafetyNetBlock.WATERLOGGED`: it uses Minecraft's `BooleanProperty` (vectorwing/farmersdelight/common/block/SafetyNetBlock.java:27)
- [ ] `SafetyNetBlock.SHAPE`: it uses Minecraft's `VoxelShape` (vectorwing/farmersdelight/common/block/SafetyNetBlock.java:28)
- [ ] `SafetyNetBlock.SafetyNetBlock(1)`: its class isn't translated (it extends Minecraft's `Block`) (vectorwing/farmersdelight/common/block/SafetyNetBlock.java:30)
- [ ] `SafetyNetBlock.createBlockStateDefinition(1)`: its class isn't translated (it extends Minecraft's `Block`) (vectorwing/farmersdelight/common/block/SafetyNetBlock.java:35)
- [ ] `SafetyNetBlock.getStateForPlacement(1)`: its class isn't translated (it extends Minecraft's `Block`) (vectorwing/farmersdelight/common/block/SafetyNetBlock.java:39)
- [ ] `SafetyNetBlock.updateShape(8)`: its class isn't translated (it extends Minecraft's `Block`) (vectorwing/farmersdelight/common/block/SafetyNetBlock.java:44)
- [ ] `SafetyNetBlock.getFluidState(1)`: its class isn't translated (it extends Minecraft's `Block`) (vectorwing/farmersdelight/common/block/SafetyNetBlock.java:61)
- [ ] `SafetyNetBlock.getShape(4)`: its class isn't translated (it extends Minecraft's `Block`) (vectorwing/farmersdelight/common/block/SafetyNetBlock.java:65)
- [ ] `SafetyNetBlock.fallOn(5)`: its class isn't translated (it extends Minecraft's `Block`) (vectorwing/farmersdelight/common/block/SafetyNetBlock.java:69)

### `SandyShrubBlock`: it extends Minecraft's `VegetationBlock`

- [ ] `SandyShrubBlock.SHAPE`: it uses Minecraft's `VoxelShape` (vectorwing/farmersdelight/common/block/SandyShrubBlock.java:24)
- [ ] `SandyShrubBlock.SandyShrubBlock(1)`: its class isn't translated (it extends Minecraft's `VegetationBlock`) (vectorwing/farmersdelight/common/block/SandyShrubBlock.java:26)
- [ ] `SandyShrubBlock.getShape(4)`: its class isn't translated (it extends Minecraft's `VegetationBlock`) (vectorwing/farmersdelight/common/block/SandyShrubBlock.java:30)
- [ ] `SandyShrubBlock.mayPlaceOn(3)`: its class isn't translated (it extends Minecraft's `VegetationBlock`) (vectorwing/farmersdelight/common/block/SandyShrubBlock.java:34)
- [ ] `SandyShrubBlock.isValidBonemealTarget(4)`: its class isn't translated (it extends Minecraft's `VegetationBlock`) (vectorwing/farmersdelight/common/block/SandyShrubBlock.java:38)
- [ ] `SandyShrubBlock.isBonemealSuccess(5)`: its class isn't translated (it extends Minecraft's `VegetationBlock`) (vectorwing/farmersdelight/common/block/SandyShrubBlock.java:42)
- [ ] `SandyShrubBlock.performBonemeal(5)`: its class isn't translated (it extends Minecraft's `VegetationBlock`) (vectorwing/farmersdelight/common/block/SandyShrubBlock.java:46)

### `SkilletBlock`: it extends Minecraft's `BaseEntityBlock`

- [x] `SkilletBlock.MINIMUM_COOKING_TIME` (vectorwing/farmersdelight/common/block/SkilletBlock.java:48)
- [ ] `SkilletBlock.FACING`: it uses Minecraft's `Property` (vectorwing/farmersdelight/common/block/SkilletBlock.java:49)
- [ ] `SkilletBlock.SUPPORT`: it uses Minecraft's `BooleanProperty` (vectorwing/farmersdelight/common/block/SkilletBlock.java:50)
- [ ] `SkilletBlock.WATERLOGGED`: it uses Minecraft's `BooleanProperty` (vectorwing/farmersdelight/common/block/SkilletBlock.java:51)
- [ ] `SkilletBlock.SHAPE`: it uses Minecraft's `VoxelShape` (vectorwing/farmersdelight/common/block/SkilletBlock.java:52)
- [ ] `SkilletBlock.SHAPE_WITH_TRAY`: it uses Minecraft's `VoxelShape` (vectorwing/farmersdelight/common/block/SkilletBlock.java:53)
- [ ] `SkilletBlock.SkilletBlock(1)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (vectorwing/farmersdelight/common/block/SkilletBlock.java:55)
- [ ] `SkilletBlock.useItemOn(7)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (vectorwing/farmersdelight/common/block/SkilletBlock.java:63)
- [ ] `SkilletBlock.getRenderShape(1)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (vectorwing/farmersdelight/common/block/SkilletBlock.java:98)
- [ ] `SkilletBlock.getShape(4)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (vectorwing/farmersdelight/common/block/SkilletBlock.java:102)
- [ ] `SkilletBlock.getCollisionShape(4)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (vectorwing/farmersdelight/common/block/SkilletBlock.java:106)
- [ ] `SkilletBlock.getStateForPlacement(1)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (vectorwing/farmersdelight/common/block/SkilletBlock.java:110)
- [ ] `SkilletBlock.updateShape(8)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (vectorwing/farmersdelight/common/block/SkilletBlock.java:118)
- [ ] `SkilletBlock.getCloneItemStack(4)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (vectorwing/farmersdelight/common/block/SkilletBlock.java:135)
- [ ] `SkilletBlock.createBlockStateDefinition(1)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (vectorwing/farmersdelight/common/block/SkilletBlock.java:141)
- [ ] `SkilletBlock.animateTick(4)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (vectorwing/farmersdelight/common/block/SkilletBlock.java:145)
- [ ] `SkilletBlock.getFluidState(1)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (vectorwing/farmersdelight/common/block/SkilletBlock.java:156)
- [ ] `SkilletBlock.newBlockEntity(2)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (vectorwing/farmersdelight/common/block/SkilletBlock.java:160)
- [ ] `SkilletBlock.getTicker(3)`: it's generic (vectorwing/farmersdelight/common/block/SkilletBlock.java:164)
- [ ] `SkilletBlock.getTrayState(2)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (vectorwing/farmersdelight/common/block/SkilletBlock.java:174)
- [x] `SkilletBlock.getSkilletCookingTime(2)` (vectorwing/farmersdelight/common/block/SkilletBlock.java:178)

### `StandingCanvasSignBlock`: it extends Minecraft's `StandingSignBlock`

- [ ] `StandingCanvasSignBlock.StandingCanvasSignBlock(2)`: its class isn't translated (it extends Minecraft's `StandingSignBlock`) (vectorwing/farmersdelight/common/block/StandingCanvasSignBlock.java:13)
- [ ] `StandingCanvasSignBlock.getBackgroundColor(0)`: its class isn't translated (it extends Minecraft's `StandingSignBlock`) (vectorwing/farmersdelight/common/block/StandingCanvasSignBlock.java:18)

### `CanvasSign`: it's an interface

- [ ] `CanvasSign.getBackgroundColor(0)`: it has no body (vectorwing/farmersdelight/common/block/state/CanvasSign.java:8)
- [ ] `CanvasSign.isDarkBackground(0)`: its class isn't translated (it's an interface) (vectorwing/farmersdelight/common/block/state/CanvasSign.java:10)

### `CookingPotSupport`: its constants carry values

- [ ] `CookingPotSupport.CookingPotSupport(1)`: its class isn't translated (its constants carry values) (vectorwing/farmersdelight/common/block/state/CookingPotSupport.java:13)
- [ ] `CookingPotSupport.toString(0)`: its class isn't translated (its constants carry values) (vectorwing/farmersdelight/common/block/state/CookingPotSupport.java:17)
- [ ] `CookingPotSupport.getSerializedName(0)`: its class isn't translated (its constants carry values) (vectorwing/farmersdelight/common/block/state/CookingPotSupport.java:22)

### `StoveBlock`: it extends `AbstractStoveBlock`

- [ ] `StoveBlock.StoveBlock(1)`: its class isn't translated (it extends `AbstractStoveBlock`) (vectorwing/farmersdelight/common/block/StoveBlock.java:22)
- [ ] `StoveBlock.newBlockEntity(2)`: its class isn't translated (it extends `AbstractStoveBlock`) (vectorwing/farmersdelight/common/block/StoveBlock.java:26)
- [ ] `StoveBlock.getTicker(3)`: it's generic (vectorwing/farmersdelight/common/block/StoveBlock.java:30)
- [ ] `StoveBlock.animateTick(4)`: its class isn't translated (it extends `AbstractStoveBlock`) (vectorwing/farmersdelight/common/block/StoveBlock.java:40)

### `StrawBaleBlock`: it extends Minecraft's `HayBlock`

- [ ] `StrawBaleBlock.StrawBaleBlock(1)`: its class isn't translated (it extends Minecraft's `HayBlock`) (vectorwing/farmersdelight/common/block/StrawBaleBlock.java:12)
- [ ] `StrawBaleBlock.getFireSpreadSpeed(4)`: its class isn't translated (it extends Minecraft's `HayBlock`) (vectorwing/farmersdelight/common/block/StrawBaleBlock.java:17)
- [ ] `StrawBaleBlock.getFlammability(4)`: its class isn't translated (it extends Minecraft's `HayBlock`) (vectorwing/farmersdelight/common/block/StrawBaleBlock.java:21)

### `TatamiBlock`: it extends Minecraft's `Block`

- [ ] `TatamiBlock.FACING`: it uses Minecraft's `Property` (vectorwing/farmersdelight/common/block/TatamiBlock.java:25)
- [ ] `TatamiBlock.PAIRED`: it uses Minecraft's `BooleanProperty` (vectorwing/farmersdelight/common/block/TatamiBlock.java:26)
- [ ] `TatamiBlock.TatamiBlock(1)`: its class isn't translated (it extends Minecraft's `Block`) (vectorwing/farmersdelight/common/block/TatamiBlock.java:28)
- [ ] `TatamiBlock.getStateForPlacement(1)`: its class isn't translated (it extends Minecraft's `Block`) (vectorwing/farmersdelight/common/block/TatamiBlock.java:35)
- [ ] `TatamiBlock.setPlacedBy(5)`: its class isn't translated (it extends Minecraft's `Block`) (vectorwing/farmersdelight/common/block/TatamiBlock.java:47)
- [ ] `TatamiBlock.updateShape(8)`: its class isn't translated (it extends Minecraft's `Block`) (vectorwing/farmersdelight/common/block/TatamiBlock.java:66)
- [ ] `TatamiBlock.createBlockStateDefinition(1)`: its class isn't translated (it extends Minecraft's `Block`) (vectorwing/farmersdelight/common/block/TatamiBlock.java:81)
- [ ] `TatamiBlock.rotate(2)`: its class isn't translated (it extends Minecraft's `Block`) (vectorwing/farmersdelight/common/block/TatamiBlock.java:85)
- [ ] `TatamiBlock.mirror(2)`: its class isn't translated (it extends Minecraft's `Block`) (vectorwing/farmersdelight/common/block/TatamiBlock.java:89)

### `TatamiHalfMatBlock`: it extends Minecraft's `HorizontalDirectionalBlock`

- [ ] `TatamiHalfMatBlock.SHAPE`: it uses Minecraft's `VoxelShape` (vectorwing/farmersdelight/common/block/TatamiHalfMatBlock.java:21)
- [ ] `TatamiHalfMatBlock.TatamiHalfMatBlock(1)`: its class isn't translated (it extends Minecraft's `HorizontalDirectionalBlock`) (vectorwing/farmersdelight/common/block/TatamiHalfMatBlock.java:23)
- [ ] `TatamiHalfMatBlock.getShape(4)`: its class isn't translated (it extends Minecraft's `HorizontalDirectionalBlock`) (vectorwing/farmersdelight/common/block/TatamiHalfMatBlock.java:27)
- [ ] `TatamiHalfMatBlock.updateShape(8)`: its class isn't translated (it extends Minecraft's `HorizontalDirectionalBlock`) (vectorwing/farmersdelight/common/block/TatamiHalfMatBlock.java:31)
- [ ] `TatamiHalfMatBlock.canSurvive(3)`: its class isn't translated (it extends Minecraft's `HorizontalDirectionalBlock`) (vectorwing/farmersdelight/common/block/TatamiHalfMatBlock.java:46)
- [ ] `TatamiHalfMatBlock.getStateForPlacement(1)`: its class isn't translated (it extends Minecraft's `HorizontalDirectionalBlock`) (vectorwing/farmersdelight/common/block/TatamiHalfMatBlock.java:50)
- [ ] `TatamiHalfMatBlock.createBlockStateDefinition(1)`: its class isn't translated (it extends Minecraft's `HorizontalDirectionalBlock`) (vectorwing/farmersdelight/common/block/TatamiHalfMatBlock.java:54)

### `TatamiMatBlock`: it extends Minecraft's `HorizontalDirectionalBlock`

- [ ] `TatamiMatBlock.PART`: it uses Minecraft's `EnumProperty` (vectorwing/farmersdelight/common/block/TatamiMatBlock.java:30)
- [ ] `TatamiMatBlock.SHAPE`: it uses Minecraft's `VoxelShape` (vectorwing/farmersdelight/common/block/TatamiMatBlock.java:31)
- [ ] `TatamiMatBlock.TatamiMatBlock(1)`: its class isn't translated (it extends Minecraft's `HorizontalDirectionalBlock`) (vectorwing/farmersdelight/common/block/TatamiMatBlock.java:33)
- [ ] `TatamiMatBlock.getDirectionToOther(2)`: it uses Minecraft's `Direction` (vectorwing/farmersdelight/common/block/TatamiMatBlock.java:38)
- [ ] `TatamiMatBlock.createBlockStateDefinition(1)`: its class isn't translated (it extends Minecraft's `HorizontalDirectionalBlock`) (vectorwing/farmersdelight/common/block/TatamiMatBlock.java:42)
- [ ] `TatamiMatBlock.getShape(4)`: its class isn't translated (it extends Minecraft's `HorizontalDirectionalBlock`) (vectorwing/farmersdelight/common/block/TatamiMatBlock.java:46)
- [ ] `TatamiMatBlock.getRenderShape(1)`: its class isn't translated (it extends Minecraft's `HorizontalDirectionalBlock`) (vectorwing/farmersdelight/common/block/TatamiMatBlock.java:50)
- [ ] `TatamiMatBlock.updateShape(8)`: its class isn't translated (it extends Minecraft's `HorizontalDirectionalBlock`) (vectorwing/farmersdelight/common/block/TatamiMatBlock.java:54)
- [ ] `TatamiMatBlock.canSurvive(3)`: its class isn't translated (it extends Minecraft's `HorizontalDirectionalBlock`) (vectorwing/farmersdelight/common/block/TatamiMatBlock.java:75)
- [ ] `TatamiMatBlock.playerWillDestroy(4)`: its class isn't translated (it extends Minecraft's `HorizontalDirectionalBlock`) (vectorwing/farmersdelight/common/block/TatamiMatBlock.java:79)
- [ ] `TatamiMatBlock.setPlacedBy(5)`: its class isn't translated (it extends Minecraft's `HorizontalDirectionalBlock`) (vectorwing/farmersdelight/common/block/TatamiMatBlock.java:95)
- [ ] `TatamiMatBlock.getStateForPlacement(1)`: its class isn't translated (it extends Minecraft's `HorizontalDirectionalBlock`) (vectorwing/farmersdelight/common/block/TatamiMatBlock.java:105)

### `TomatoBlock`: it extends Minecraft's `CropBlock`

- [ ] `TomatoBlock.VINE_AGE`: it uses Minecraft's `IntegerProperty` (vectorwing/farmersdelight/common/block/TomatoBlock.java:46)
- [ ] `TomatoBlock.ROPELOGGED`: it uses Minecraft's `BooleanProperty` (vectorwing/farmersdelight/common/block/TomatoBlock.java:47)
- [ ] `TomatoBlock.SHAPE`: it uses Minecraft's `VoxelShape` (vectorwing/farmersdelight/common/block/TomatoBlock.java:48)
- [ ] `TomatoBlock.TomatoBlock(1)`: its class isn't translated (it extends Minecraft's `CropBlock`) (vectorwing/farmersdelight/common/block/TomatoBlock.java:50)
- [ ] `TomatoBlock.TomatoBlock(2)`: its class isn't translated (it extends Minecraft's `CropBlock`) (vectorwing/farmersdelight/common/block/TomatoBlock.java:57)
- [ ] `TomatoBlock.useItemOn(7)`: its class isn't translated (it extends Minecraft's `CropBlock`) (vectorwing/farmersdelight/common/block/TomatoBlock.java:61)
- [ ] `TomatoBlock.useWithoutItem(5)`: its class isn't translated (it extends Minecraft's `CropBlock`) (vectorwing/farmersdelight/common/block/TomatoBlock.java:71)
- [ ] `TomatoBlock.isRandomlyTicking(1)`: its class isn't translated (it extends Minecraft's `CropBlock`) (vectorwing/farmersdelight/common/block/TomatoBlock.java:89)
- [ ] `TomatoBlock.tick(4)`: its class isn't translated (it extends Minecraft's `CropBlock`) (vectorwing/farmersdelight/common/block/TomatoBlock.java:93)
- [ ] `TomatoBlock.randomTick(4)`: its class isn't translated (it extends Minecraft's `CropBlock`) (vectorwing/farmersdelight/common/block/TomatoBlock.java:102)
- [ ] `TomatoBlock.getStateForAge(1)`: its class isn't translated (it extends Minecraft's `CropBlock`) (vectorwing/farmersdelight/common/block/TomatoBlock.java:122)
- [ ] `TomatoBlock.getAgeProperty(0)`: its class isn't translated (it extends Minecraft's `CropBlock`) (vectorwing/farmersdelight/common/block/TomatoBlock.java:126)
- [ ] `TomatoBlock.getShape(4)`: its class isn't translated (it extends Minecraft's `CropBlock`) (vectorwing/farmersdelight/common/block/TomatoBlock.java:130)
- [ ] `TomatoBlock.getMaxAge(0)`: its class isn't translated (it extends Minecraft's `CropBlock`) (vectorwing/farmersdelight/common/block/TomatoBlock.java:134)
- [ ] `TomatoBlock.getBaseSeedId(0)`: its class isn't translated (it extends Minecraft's `CropBlock`) (vectorwing/farmersdelight/common/block/TomatoBlock.java:138)
- [ ] `TomatoBlock.createBlockStateDefinition(1)`: its class isn't translated (it extends Minecraft's `CropBlock`) (vectorwing/farmersdelight/common/block/TomatoBlock.java:142)
- [ ] `TomatoBlock.canClimbBlock(1)`: its class isn't translated (it extends Minecraft's `CropBlock`) (vectorwing/farmersdelight/common/block/TomatoBlock.java:146)
- [ ] `TomatoBlock.getClimbingState(1)`: its class isn't translated (it extends Minecraft's `CropBlock`) (vectorwing/farmersdelight/common/block/TomatoBlock.java:150)
- [ ] `TomatoBlock.climbRopeAbove(2)`: its class isn't translated (it extends Minecraft's `CropBlock`) (vectorwing/farmersdelight/common/block/TomatoBlock.java:154)
- [ ] `TomatoBlock.getBonemealAgeIncrease(1)`: its class isn't translated (it extends Minecraft's `CropBlock`) (vectorwing/farmersdelight/common/block/TomatoBlock.java:171)
- [ ] `TomatoBlock.isValidBonemealTarget(4)`: its class isn't translated (it extends Minecraft's `CropBlock`) (vectorwing/farmersdelight/common/block/TomatoBlock.java:175)
- [ ] `TomatoBlock.performBonemeal(5)`: its class isn't translated (it extends Minecraft's `CropBlock`) (vectorwing/farmersdelight/common/block/TomatoBlock.java:201)
- [ ] `TomatoBlock.canSurvive(3)`: its class isn't translated (it extends Minecraft's `CropBlock`) (vectorwing/farmersdelight/common/block/TomatoBlock.java:222)
- [ ] `TomatoBlock.hasGoodCropConditions(2)`: its class isn't translated (it extends Minecraft's `CropBlock`) (vectorwing/farmersdelight/common/block/TomatoBlock.java:228)
- [ ] `TomatoBlock.updateShape(8)`: its class isn't translated (it extends Minecraft's `CropBlock`) (vectorwing/farmersdelight/common/block/TomatoBlock.java:232)
- [ ] `TomatoBlock.playerDestroy(6)`: its class isn't translated (it extends Minecraft's `CropBlock`) (vectorwing/farmersdelight/common/block/TomatoBlock.java:249)
- [ ] `TomatoBlock.destroyAndPlaceRope(2)`: it uses Minecraft's `Level` (vectorwing/farmersdelight/common/block/TomatoBlock.java:257)
- [ ] `TomatoBlock.getGrowthSpeed(3)`: its class isn't translated (it extends Minecraft's `CropBlock`) (vectorwing/farmersdelight/common/block/TomatoBlock.java:264)

### `TomatoVineBlock`: it extends `TomatoBlock`

- [ ] `TomatoVineBlock.TomatoVineBlock(1)`: its class isn't translated (it extends `TomatoBlock`) (vectorwing/farmersdelight/common/block/TomatoVineBlock.java:14)
- [ ] `TomatoVineBlock.TomatoVineBlock(2)`: its class isn't translated (it extends `TomatoBlock`) (vectorwing/farmersdelight/common/block/TomatoVineBlock.java:18)
- [ ] `TomatoVineBlock.attemptRopeClimb(3)`: its class isn't translated (it extends `TomatoBlock`) (vectorwing/farmersdelight/common/block/TomatoVineBlock.java:22)

### `WallCanvasSignBlock`: it extends Minecraft's `WallSignBlock`

- [ ] `WallCanvasSignBlock.WallCanvasSignBlock(2)`: its class isn't translated (it extends Minecraft's `WallSignBlock`) (vectorwing/farmersdelight/common/block/WallCanvasSignBlock.java:13)
- [ ] `WallCanvasSignBlock.getBackgroundColor(0)`: its class isn't translated (it extends Minecraft's `WallSignBlock`) (vectorwing/farmersdelight/common/block/WallCanvasSignBlock.java:18)

### `WallHangingCanvasSignBlock`: it extends Minecraft's `WallHangingSignBlock`

- [ ] `WallHangingCanvasSignBlock.WallHangingCanvasSignBlock(2)`: its class isn't translated (it extends Minecraft's `WallHangingSignBlock`) (vectorwing/farmersdelight/common/block/WallHangingCanvasSignBlock.java:13)
- [ ] `WallHangingCanvasSignBlock.getBackgroundColor(0)`: its class isn't translated (it extends Minecraft's `WallHangingSignBlock`) (vectorwing/farmersdelight/common/block/WallHangingCanvasSignBlock.java:18)

### `WildCropBlock`: it extends Minecraft's `FlowerBlock`

- [ ] `WildCropBlock.SHAPE`: it uses Minecraft's `VoxelShape` (vectorwing/farmersdelight/common/block/WildCropBlock.java:25)
- [ ] `WildCropBlock.WildCropBlock(3)`: its class isn't translated (it extends Minecraft's `FlowerBlock`) (vectorwing/farmersdelight/common/block/WildCropBlock.java:27)
- [ ] `WildCropBlock.getShape(4)`: its class isn't translated (it extends Minecraft's `FlowerBlock`) (vectorwing/farmersdelight/common/block/WildCropBlock.java:32)
- [ ] `WildCropBlock.mayPlaceOn(3)`: its class isn't translated (it extends Minecraft's `FlowerBlock`) (vectorwing/farmersdelight/common/block/WildCropBlock.java:36)
- [ ] `WildCropBlock.canBeReplaced(2)`: its class isn't translated (it extends Minecraft's `FlowerBlock`) (vectorwing/farmersdelight/common/block/WildCropBlock.java:40)
- [ ] `WildCropBlock.getFireSpreadSpeed(4)`: its class isn't translated (it extends Minecraft's `FlowerBlock`) (vectorwing/farmersdelight/common/block/WildCropBlock.java:44)
- [ ] `WildCropBlock.getFlammability(4)`: its class isn't translated (it extends Minecraft's `FlowerBlock`) (vectorwing/farmersdelight/common/block/WildCropBlock.java:48)
- [ ] `WildCropBlock.isValidBonemealTarget(4)`: its class isn't translated (it extends Minecraft's `FlowerBlock`) (vectorwing/farmersdelight/common/block/WildCropBlock.java:52)
- [ ] `WildCropBlock.isBonemealSuccess(5)`: its class isn't translated (it extends Minecraft's `FlowerBlock`) (vectorwing/farmersdelight/common/block/WildCropBlock.java:56)
- [ ] `WildCropBlock.performBonemeal(5)`: its class isn't translated (it extends Minecraft's `FlowerBlock`) (vectorwing/farmersdelight/common/block/WildCropBlock.java:60)

### `WildRiceBlock`: it extends Minecraft's `DoublePlantBlock`

- [ ] `WildRiceBlock.WATERLOGGED`: it uses Minecraft's `BooleanProperty` (vectorwing/farmersdelight/common/block/WildRiceBlock.java:37)
- [ ] `WildRiceBlock.WildRiceBlock(1)`: its class isn't translated (it extends Minecraft's `DoublePlantBlock`) (vectorwing/farmersdelight/common/block/WildRiceBlock.java:39)
- [ ] `WildRiceBlock.createBlockStateDefinition(1)`: its class isn't translated (it extends Minecraft's `DoublePlantBlock`) (vectorwing/farmersdelight/common/block/WildRiceBlock.java:44)
- [ ] `WildRiceBlock.canSurvive(3)`: its class isn't translated (it extends Minecraft's `DoublePlantBlock`) (vectorwing/farmersdelight/common/block/WildRiceBlock.java:48)
- [ ] `WildRiceBlock.mayPlaceOn(3)`: its class isn't translated (it extends Minecraft's `DoublePlantBlock`) (vectorwing/farmersdelight/common/block/WildRiceBlock.java:59)
- [ ] `WildRiceBlock.canBeReplaced(2)`: its class isn't translated (it extends Minecraft's `DoublePlantBlock`) (vectorwing/farmersdelight/common/block/WildRiceBlock.java:63)
- [ ] `WildRiceBlock.setPlacedBy(5)`: its class isn't translated (it extends Minecraft's `DoublePlantBlock`) (vectorwing/farmersdelight/common/block/WildRiceBlock.java:67)
- [ ] `WildRiceBlock.updateShape(8)`: its class isn't translated (it extends Minecraft's `DoublePlantBlock`) (vectorwing/farmersdelight/common/block/WildRiceBlock.java:71)
- [ ] `WildRiceBlock.getStateForPlacement(1)`: its class isn't translated (it extends Minecraft's `DoublePlantBlock`) (vectorwing/farmersdelight/common/block/WildRiceBlock.java:96)
- [ ] `WildRiceBlock.canPlaceLiquid(5)`: its class isn't translated (it extends Minecraft's `DoublePlantBlock`) (vectorwing/farmersdelight/common/block/WildRiceBlock.java:107)
- [ ] `WildRiceBlock.getFluidState(1)`: its class isn't translated (it extends Minecraft's `DoublePlantBlock`) (vectorwing/farmersdelight/common/block/WildRiceBlock.java:111)
- [ ] `WildRiceBlock.isValidBonemealTarget(4)`: its class isn't translated (it extends Minecraft's `DoublePlantBlock`) (vectorwing/farmersdelight/common/block/WildRiceBlock.java:115)
- [ ] `WildRiceBlock.isBonemealSuccess(5)`: its class isn't translated (it extends Minecraft's `DoublePlantBlock`) (vectorwing/farmersdelight/common/block/WildRiceBlock.java:119)
- [ ] `WildRiceBlock.performBonemeal(5)`: its class isn't translated (it extends Minecraft's `DoublePlantBlock`) (vectorwing/farmersdelight/common/block/WildRiceBlock.java:123)

### `CommonSetup`

- [ ] `CommonSetup.init(0)`: it calls CommonSetup.registerDispenserBehaviors, which isn't translated (vectorwing/farmersdelight/common/CommonSetup.java:8)
- [ ] `CommonSetup.registerDispenserBehaviors(0)`: it uses ModItems.ROTTEN_TOMATO, which isn't translated (it uses Minecraft's `Item`) (vectorwing/farmersdelight/common/CommonSetup.java:13)
- [x] `CommonSetup.registerItemSetAdditions(0)` (vectorwing/farmersdelight/common/CommonSetup.java:17)

### `Configuration`

- [ ] `Configuration.COMMON_CONFIG`: it uses ModConfigHolder, which isn't translated (it uses Minecraft's `Component`) (vectorwing/farmersdelight/common/Configuration.java:13)
- [ ] `Configuration.CLIENT_CONFIG`: it uses ModConfigHolder, which isn't translated (it uses Minecraft's `Component`) (vectorwing/farmersdelight/common/Configuration.java:14)
- [x] `Configuration.CATEGORY_SETTINGS` (vectorwing/farmersdelight/common/Configuration.java:15)
- [ ] `Configuration.ENABLE_ROPE_REELING`: a function kept in a variable or field (only parameters can be) (vectorwing/farmersdelight/common/Configuration.java:16)
- [ ] `Configuration.CANVAS_SIGN_DARK_BACKGROUND_LIST`: a function kept in a variable or field (only parameters can be) (vectorwing/farmersdelight/common/Configuration.java:17)
- [x] `Configuration.CATEGORY_FARMING` (vectorwing/farmersdelight/common/Configuration.java:18)
- [ ] `Configuration.RICH_SOIL_BOOST_CHANCE`: a function kept in a variable or field (only parameters can be) (vectorwing/farmersdelight/common/Configuration.java:19)
- [ ] `Configuration.ENABLE_TOMATO_VINE_CLIMBING_TAGGED_ROPES`: a function kept in a variable or field (only parameters can be) (vectorwing/farmersdelight/common/Configuration.java:20)
- [ ] `Configuration.DEFAULT_TOMATO_VINE_ROPE`: a function kept in a variable or field (only parameters can be) (vectorwing/farmersdelight/common/Configuration.java:21)
- [x] `Configuration.CATEGORY_CRAFTING` (vectorwing/farmersdelight/common/Configuration.java:22)
- [ ] `Configuration.ENABLE_COOKING_POT_RECIPE_BOOK`: a function kept in a variable or field (only parameters can be) (vectorwing/farmersdelight/common/Configuration.java:23)
- [ ] `Configuration.ENABLE_VANILLA_CROP_CRATES`: a function kept in a variable or field (only parameters can be) (vectorwing/farmersdelight/common/Configuration.java:24)
- [ ] `Configuration.CUTTING_BOARD_FORTUNE_BONUS`: a function kept in a variable or field (only parameters can be) (vectorwing/farmersdelight/common/Configuration.java:25)
- [x] `Configuration.CATEGORY_OVERRIDES` (vectorwing/farmersdelight/common/Configuration.java:26)
- [ ] `Configuration.ENABLE_VANILLA_STRAW_BED_RENAME`: a function kept in a variable or field (only parameters can be) (vectorwing/farmersdelight/common/Configuration.java:27)
- [ ] `Configuration.ENABLE_VANILLA_SOUP_EXTRA_EFFECTS`: a function kept in a variable or field (only parameters can be) (vectorwing/farmersdelight/common/Configuration.java:28)
- [ ] `Configuration.ENABLE_RABBIT_STEW_BUFF`: a function kept in a variable or field (only parameters can be) (vectorwing/farmersdelight/common/Configuration.java:29)
- [ ] `Configuration.ENABLE_PUMPKIN_PIE_SNEAK_TO_PLACE`: a function kept in a variable or field (only parameters can be) (vectorwing/farmersdelight/common/Configuration.java:30)
- [ ] `Configuration.ENABLE_DISPENSER_TOOLS_CUTTING_BOARD`: a function kept in a variable or field (only parameters can be) (vectorwing/farmersdelight/common/Configuration.java:31)
- [x] `Configuration.CATEGORY_OVERRIDES_STACK_SIZE` (vectorwing/farmersdelight/common/Configuration.java:32)
- [ ] `Configuration.ENABLE_STACKABLE_SOUP_ITEMS`: a function kept in a variable or field (only parameters can be) (vectorwing/farmersdelight/common/Configuration.java:33)
- [ ] `Configuration.SOUP_ITEM_LIST`: a function kept in a variable or field (only parameters can be) (vectorwing/farmersdelight/common/Configuration.java:34)
- [x] `Configuration.CATEGORY_WORLD` (vectorwing/farmersdelight/common/Configuration.java:35)
- [ ] `Configuration.GENERATE_FD_CHEST_LOOT`: a function kept in a variable or field (only parameters can be) (vectorwing/farmersdelight/common/Configuration.java:36)
- [ ] `Configuration.GENERATE_VILLAGE_COMPOST_HEAPS`: a function kept in a variable or field (only parameters can be) (vectorwing/farmersdelight/common/Configuration.java:37)
- [ ] `Configuration.GENERATE_VILLAGE_FARM_FD_CROPS`: a function kept in a variable or field (only parameters can be) (vectorwing/farmersdelight/common/Configuration.java:38)
- [x] `Configuration.CATEGORY_DEBUG` (vectorwing/farmersdelight/common/Configuration.java:39)
- [ ] `Configuration.ENABLE_TOMATO_ROPE_PERMANENCE`: a function kept in a variable or field (only parameters can be) (vectorwing/farmersdelight/common/Configuration.java:40)
- [x] `Configuration.CATEGORY_CLIENT` (vectorwing/farmersdelight/common/Configuration.java:41)
- [ ] `Configuration.ENABLE_NOURISHMENT_HUNGER_OVERLAY`: a function kept in a variable or field (only parameters can be) (vectorwing/farmersdelight/common/Configuration.java:42)
- [ ] `Configuration.ENABLE_COMFORT_HEALTH_OVERLAY`: a function kept in a variable or field (only parameters can be) (vectorwing/farmersdelight/common/Configuration.java:43)
- [ ] `Configuration.ENABLE_FOOD_EFFECT_TOOLTIP`: a function kept in a variable or field (only parameters can be) (vectorwing/farmersdelight/common/Configuration.java:44)
- [ ] `Configuration.ENABLE_RECIPE_BOOK_COOKING_POT`: a function kept in a variable or field (only parameters can be) (vectorwing/farmersdelight/common/Configuration.java:46)
- [ ] `Configuration.VANILLA_SOUP_EXTRA_EFFECTS`: a function kept in a variable or field (only parameters can be) (vectorwing/farmersdelight/common/Configuration.java:48)
- [ ] `Configuration.RABBIT_STEW_BUFF`: a function kept in a variable or field (only parameters can be) (vectorwing/farmersdelight/common/Configuration.java:50)
- [ ] `Configuration.DISPENSER_TOOLS_CUTTING_BOARD`: a function kept in a variable or field (only parameters can be) (vectorwing/farmersdelight/common/Configuration.java:52)
- [ ] `Configuration.NOURISHMENT_HUNGER_OVERLAY`: a function kept in a variable or field (only parameters can be) (vectorwing/farmersdelight/common/Configuration.java:54)
- [ ] `Configuration.COMFORT_HEALTH_OVERLAY`: a function kept in a variable or field (only parameters can be) (vectorwing/farmersdelight/common/Configuration.java:56)
- [ ] `Configuration.FOOD_EFFECT_TOOLTIP`: a function kept in a variable or field (only parameters can be) (vectorwing/farmersdelight/common/Configuration.java:58)
- [x] `Configuration.touch(0)` (vectorwing/farmersdelight/common/Configuration.java:60)

### `VanillaCrateEnabledCondition`: it implements Fabric's `ResourceCondition`

- [ ] `VanillaCrateEnabledCondition.INSTANCE`: it uses VanillaCrateEnabledCondition, which isn't translated (it implements Fabric's `ResourceCondition`) (vectorwing/farmersdelight/common/crafting/condition/VanillaCrateEnabledCondition.java:13)
- [ ] `VanillaCrateEnabledCondition.CODEC`: it uses Minecraft's `MapCodec` (vectorwing/farmersdelight/common/crafting/condition/VanillaCrateEnabledCondition.java:14)
- [ ] `VanillaCrateEnabledCondition.TYPE`: it uses Fabric's `ResourceConditionType` (vectorwing/farmersdelight/common/crafting/condition/VanillaCrateEnabledCondition.java:15)
- [ ] `VanillaCrateEnabledCondition.init(0)`: it uses VanillaCrateEnabledCondition.TYPE, which isn't translated (it uses Fabric's `ResourceConditionType`) (vectorwing/farmersdelight/common/crafting/condition/VanillaCrateEnabledCondition.java:19)
- [ ] `VanillaCrateEnabledCondition.getType(0)`: its class isn't translated (it implements Fabric's `ResourceCondition`) (vectorwing/farmersdelight/common/crafting/condition/VanillaCrateEnabledCondition.java:23)
- [ ] `VanillaCrateEnabledCondition.test(1)`: its class isn't translated (it implements Fabric's `ResourceCondition`) (vectorwing/farmersdelight/common/crafting/condition/VanillaCrateEnabledCondition.java:27)

### `CookingPotBookCategory`: its constants carry values

- [ ] `CookingPotBookCategory.CODEC`: it uses Minecraft's `Codec` (vectorwing/farmersdelight/common/crafting/CookingPotBookCategory.java:18)
- [ ] `CookingPotBookCategory.BY_ID`: it uses CookingPotBookCategory, which isn't translated (its constants carry values) (vectorwing/farmersdelight/common/crafting/CookingPotBookCategory.java:31)
- [ ] `CookingPotBookCategory.STREAM_CODEC`: it uses Minecraft's `StreamCodec` (vectorwing/farmersdelight/common/crafting/CookingPotBookCategory.java:32)
- [ ] `CookingPotBookCategory.CookingPotBookCategory(1)`: its class isn't translated (its constants carry values) (vectorwing/farmersdelight/common/crafting/CookingPotBookCategory.java:35)
- [ ] `CookingPotBookCategory.findByName(1)`: it uses CookingPotBookCategory, which isn't translated (its constants carry values) (vectorwing/farmersdelight/common/crafting/CookingPotBookCategory.java:39)
- [ ] `CookingPotBookCategory.toString(0)`: its class isn't translated (its constants carry values) (vectorwing/farmersdelight/common/crafting/CookingPotBookCategory.java:49)

### `CookingPotRecipe`: it implements Minecraft's `Recipe`

- [x] `CookingPotRecipe.INPUT_SLOTS` (vectorwing/farmersdelight/common/crafting/CookingPotRecipe.java:37)
- [ ] `CookingPotRecipe.CookingPotRecipe(7)`: its class isn't translated (it implements Minecraft's `Recipe`) (vectorwing/farmersdelight/common/crafting/CookingPotRecipe.java:47)
- [ ] `CookingPotRecipe.input(0)`: its class isn't translated (it implements Minecraft's `Recipe`) (vectorwing/farmersdelight/common/crafting/CookingPotRecipe.java:65)
- [ ] `CookingPotRecipe.result(0)`: its class isn't translated (it implements Minecraft's `Recipe`) (vectorwing/farmersdelight/common/crafting/CookingPotRecipe.java:69)
- [ ] `CookingPotRecipe.category(0)`: its class isn't translated (it implements Minecraft's `Recipe`) (vectorwing/farmersdelight/common/crafting/CookingPotRecipe.java:73)
- [ ] `CookingPotRecipe.container(0)`: its class isn't translated (it implements Minecraft's `Recipe`) (vectorwing/farmersdelight/common/crafting/CookingPotRecipe.java:77)
- [ ] `CookingPotRecipe.containerOverride(0)`: its class isn't translated (it implements Minecraft's `Recipe`) (vectorwing/farmersdelight/common/crafting/CookingPotRecipe.java:93)
- [ ] `CookingPotRecipe.assemble(1)`: its class isn't translated (it implements Minecraft's `Recipe`) (vectorwing/farmersdelight/common/crafting/CookingPotRecipe.java:97)
- [ ] `CookingPotRecipe.showNotification(0)`: its class isn't translated (it implements Minecraft's `Recipe`) (vectorwing/farmersdelight/common/crafting/CookingPotRecipe.java:101)
- [ ] `CookingPotRecipe.group(0)`: its class isn't translated (it implements Minecraft's `Recipe`) (vectorwing/farmersdelight/common/crafting/CookingPotRecipe.java:105)
- [ ] `CookingPotRecipe.getSerializer(0)`: its class isn't translated (it implements Minecraft's `Recipe`) (vectorwing/farmersdelight/common/crafting/CookingPotRecipe.java:109)
- [ ] `CookingPotRecipe.getType(0)`: its class isn't translated (it implements Minecraft's `Recipe`) (vectorwing/farmersdelight/common/crafting/CookingPotRecipe.java:113)
- [ ] `CookingPotRecipe.display(0)`: its class isn't translated (it implements Minecraft's `Recipe`) (vectorwing/farmersdelight/common/crafting/CookingPotRecipe.java:117)
- [ ] `CookingPotRecipe.placementInfo(0)`: its class isn't translated (it implements Minecraft's `Recipe`) (vectorwing/farmersdelight/common/crafting/CookingPotRecipe.java:132)
- [ ] `CookingPotRecipe.recipeBookCategory(0)`: its class isn't translated (it implements Minecraft's `Recipe`) (vectorwing/farmersdelight/common/crafting/CookingPotRecipe.java:140)
- [ ] `CookingPotRecipe.getExperience(0)`: its class isn't translated (it implements Minecraft's `Recipe`) (vectorwing/farmersdelight/common/crafting/CookingPotRecipe.java:148)
- [ ] `CookingPotRecipe.getCookTime(0)`: its class isn't translated (it implements Minecraft's `Recipe`) (vectorwing/farmersdelight/common/crafting/CookingPotRecipe.java:152)
- [ ] `CookingPotRecipe.matches(2)`: its class isn't translated (it implements Minecraft's `Recipe`) (vectorwing/farmersdelight/common/crafting/CookingPotRecipe.java:156)
- [ ] `CookingPotRecipe.equals(1)`: its class isn't translated (it implements Minecraft's `Recipe`) (vectorwing/farmersdelight/common/crafting/CookingPotRecipe.java:160)
- [ ] `CookingPotRecipe.hashCode(0)`: its class isn't translated (it implements Minecraft's `Recipe`) (vectorwing/farmersdelight/common/crafting/CookingPotRecipe.java:186)

### `CookingPotRecipe.Serializer`

- [ ] `CookingPotRecipe.Serializer.CODEC`: it uses Minecraft's `MapCodec` (vectorwing/farmersdelight/common/crafting/CookingPotRecipe.java:198)
- [ ] `CookingPotRecipe.Serializer.STREAM_CODEC`: it uses Minecraft's `StreamCodec` (vectorwing/farmersdelight/common/crafting/CookingPotRecipe.java:210)
- [ ] `CookingPotRecipe.Serializer.codec(0)`: it uses Minecraft's `MapCodec` (vectorwing/farmersdelight/common/crafting/CookingPotRecipe.java:214)
- [ ] `CookingPotRecipe.Serializer.streamCodec(0)`: it uses Minecraft's `StreamCodec` (vectorwing/farmersdelight/common/crafting/CookingPotRecipe.java:218)
- [ ] `CookingPotRecipe.Serializer.fromNetwork(1)`: it uses CookingPotRecipe, which isn't translated (it implements Minecraft's `Recipe`) (vectorwing/farmersdelight/common/crafting/CookingPotRecipe.java:222)
- [ ] `CookingPotRecipe.Serializer.toNetwork(2)`: it uses Minecraft's `RegistryFriendlyByteBuf` (vectorwing/farmersdelight/common/crafting/CookingPotRecipe.java:233)

### `CuttingBoardRecipe`: it implements Minecraft's `Recipe`

- [x] `CuttingBoardRecipe.MAX_RESULTS` (vectorwing/farmersdelight/common/crafting/CuttingBoardRecipe.java:35)
- [ ] `CuttingBoardRecipe.CuttingBoardRecipe(5)`: its class isn't translated (it implements Minecraft's `Recipe`) (vectorwing/farmersdelight/common/crafting/CuttingBoardRecipe.java:42)
- [ ] `CuttingBoardRecipe.CuttingBoardRecipe(5)`: its class isn't translated (it implements Minecraft's `Recipe`) (vectorwing/farmersdelight/common/crafting/CuttingBoardRecipe.java:50)
- [ ] `CuttingBoardRecipe.matches(2)`: its class isn't translated (it implements Minecraft's `Recipe`) (vectorwing/farmersdelight/common/crafting/CuttingBoardRecipe.java:54)
- [ ] `CuttingBoardRecipe.assemble(1)`: its class isn't translated (it implements Minecraft's `Recipe`) (vectorwing/farmersdelight/common/crafting/CuttingBoardRecipe.java:58)
- [ ] `CuttingBoardRecipe.isSpecial(0)`: its class isn't translated (it implements Minecraft's `Recipe`) (vectorwing/farmersdelight/common/crafting/CuttingBoardRecipe.java:62)
- [ ] `CuttingBoardRecipe.showNotification(0)`: its class isn't translated (it implements Minecraft's `Recipe`) (vectorwing/farmersdelight/common/crafting/CuttingBoardRecipe.java:66)
- [ ] `CuttingBoardRecipe.group(0)`: its class isn't translated (it implements Minecraft's `Recipe`) (vectorwing/farmersdelight/common/crafting/CuttingBoardRecipe.java:70)
- [ ] `CuttingBoardRecipe.getInput(0)`: its class isn't translated (it implements Minecraft's `Recipe`) (vectorwing/farmersdelight/common/crafting/CuttingBoardRecipe.java:74)
- [ ] `CuttingBoardRecipe.getTool(0)`: its class isn't translated (it implements Minecraft's `Recipe`) (vectorwing/farmersdelight/common/crafting/CuttingBoardRecipe.java:78)
- [ ] `CuttingBoardRecipe.getResults(0)`: its class isn't translated (it implements Minecraft's `Recipe`) (vectorwing/farmersdelight/common/crafting/CuttingBoardRecipe.java:82)
- [ ] `CuttingBoardRecipe.getRollableResults(0)`: its class isn't translated (it implements Minecraft's `Recipe`) (vectorwing/farmersdelight/common/crafting/CuttingBoardRecipe.java:86)
- [ ] `CuttingBoardRecipe.rollResults(2)`: its class isn't translated (it implements Minecraft's `Recipe`) (vectorwing/farmersdelight/common/crafting/CuttingBoardRecipe.java:90)
- [ ] `CuttingBoardRecipe.getSoundEvent(0)`: its class isn't translated (it implements Minecraft's `Recipe`) (vectorwing/farmersdelight/common/crafting/CuttingBoardRecipe.java:103)
- [ ] `CuttingBoardRecipe.getSoundEventHolder(0)`: its class isn't translated (it implements Minecraft's `Recipe`) (vectorwing/farmersdelight/common/crafting/CuttingBoardRecipe.java:107)
- [ ] `CuttingBoardRecipe.getSerializer(0)`: its class isn't translated (it implements Minecraft's `Recipe`) (vectorwing/farmersdelight/common/crafting/CuttingBoardRecipe.java:111)
- [ ] `CuttingBoardRecipe.getType(0)`: its class isn't translated (it implements Minecraft's `Recipe`) (vectorwing/farmersdelight/common/crafting/CuttingBoardRecipe.java:115)
- [ ] `CuttingBoardRecipe.placementInfo(0)`: its class isn't translated (it implements Minecraft's `Recipe`) (vectorwing/farmersdelight/common/crafting/CuttingBoardRecipe.java:119)
- [ ] `CuttingBoardRecipe.recipeBookCategory(0)`: its class isn't translated (it implements Minecraft's `Recipe`) (vectorwing/farmersdelight/common/crafting/CuttingBoardRecipe.java:123)
- [ ] `CuttingBoardRecipe.equals(1)`: its class isn't translated (it implements Minecraft's `Recipe`) (vectorwing/farmersdelight/common/crafting/CuttingBoardRecipe.java:127)
- [ ] `CuttingBoardRecipe.hashCode(0)`: its class isn't translated (it implements Minecraft's `Recipe`) (vectorwing/farmersdelight/common/crafting/CuttingBoardRecipe.java:149)

### `CuttingBoardRecipe.Serializer`

- [ ] `CuttingBoardRecipe.Serializer.STREAM_CODEC`: it uses Minecraft's `StreamCodec` (vectorwing/farmersdelight/common/crafting/CuttingBoardRecipe.java:155)
- [ ] `CuttingBoardRecipe.Serializer.CODEC`: it uses Minecraft's `MapCodec` (vectorwing/farmersdelight/common/crafting/CuttingBoardRecipe.java:158)
- [ ] `CuttingBoardRecipe.Serializer.fromNetwork(1)`: it uses CuttingBoardRecipe, which isn't translated (it implements Minecraft's `Recipe`) (vectorwing/farmersdelight/common/crafting/CuttingBoardRecipe.java:188)
- [ ] `CuttingBoardRecipe.Serializer.toNetwork(2)`: it uses Minecraft's `RegistryFriendlyByteBuf` (vectorwing/farmersdelight/common/crafting/CuttingBoardRecipe.java:197)
- [ ] `CuttingBoardRecipe.Serializer.codec(0)`: it uses Minecraft's `MapCodec` (vectorwing/farmersdelight/common/crafting/CuttingBoardRecipe.java:205)
- [ ] `CuttingBoardRecipe.Serializer.streamCodec(0)`: it uses Minecraft's `StreamCodec` (vectorwing/farmersdelight/common/crafting/CuttingBoardRecipe.java:209)

### `CuttingBoardRecipeInput`: it implements Minecraft's `RecipeInput`

- [ ] `CuttingBoardRecipeInput.CuttingBoardRecipeInput(2)`: its class isn't translated (it implements Minecraft's `RecipeInput`) (vectorwing/farmersdelight/common/crafting/CuttingBoardRecipeInput.java:7)
- [ ] `CuttingBoardRecipeInput.getItem(1)`: its class isn't translated (it implements Minecraft's `RecipeInput`) (vectorwing/farmersdelight/common/crafting/CuttingBoardRecipeInput.java:8)
- [ ] `CuttingBoardRecipeInput.size(0)`: its class isn't translated (it implements Minecraft's `RecipeInput`) (vectorwing/farmersdelight/common/crafting/CuttingBoardRecipeInput.java:16)
- [ ] `CuttingBoardRecipeInput.item(0)`: its class isn't translated (it implements Minecraft's `RecipeInput`) (vectorwing/farmersdelight/common/crafting/CuttingBoardRecipeInput.java:7)
- [ ] `CuttingBoardRecipeInput.tool(0)`: its class isn't translated (it implements Minecraft's `RecipeInput`) (vectorwing/farmersdelight/common/crafting/CuttingBoardRecipeInput.java:7)

### `DoughRecipe`: it extends Minecraft's `CustomRecipe`

- [ ] `DoughRecipe.INSTANCE`: it uses DoughRecipe, which isn't translated (it extends Minecraft's `CustomRecipe`) (vectorwing/farmersdelight/common/crafting/DoughRecipe.java:19)
- [ ] `DoughRecipe.MAP_CODEC`: it uses Minecraft's `MapCodec` (vectorwing/farmersdelight/common/crafting/DoughRecipe.java:20)
- [ ] `DoughRecipe.STREAM_CODEC`: it uses Minecraft's `StreamCodec` (vectorwing/farmersdelight/common/crafting/DoughRecipe.java:21)
- [ ] `DoughRecipe.SERIALIZER`: it uses Minecraft's `RecipeSerializer` (vectorwing/farmersdelight/common/crafting/DoughRecipe.java:22)
- [ ] `DoughRecipe.matches(2)`: its class isn't translated (it extends Minecraft's `CustomRecipe`) (vectorwing/farmersdelight/common/crafting/DoughRecipe.java:24)
- [ ] `DoughRecipe.assemble(1)`: its class isn't translated (it extends Minecraft's `CustomRecipe`) (vectorwing/farmersdelight/common/crafting/DoughRecipe.java:50)
- [ ] `DoughRecipe.getRemainingItems(1)`: its class isn't translated (it extends Minecraft's `CustomRecipe`) (vectorwing/farmersdelight/common/crafting/DoughRecipe.java:54)
- [ ] `DoughRecipe.getSerializer(0)`: its class isn't translated (it extends Minecraft's `CustomRecipe`) (vectorwing/farmersdelight/common/crafting/DoughRecipe.java:67)

### `FoodServingRecipe`: it extends Minecraft's `CustomRecipe`

- [ ] `FoodServingRecipe.INSTANCE`: it uses FoodServingRecipe, which isn't translated (it extends Minecraft's `CustomRecipe`) (vectorwing/farmersdelight/common/crafting/FoodServingRecipe.java:17)
- [ ] `FoodServingRecipe.MAP_CODEC`: it uses Minecraft's `MapCodec` (vectorwing/farmersdelight/common/crafting/FoodServingRecipe.java:18)
- [ ] `FoodServingRecipe.STREAM_CODEC`: it uses Minecraft's `StreamCodec` (vectorwing/farmersdelight/common/crafting/FoodServingRecipe.java:19)
- [ ] `FoodServingRecipe.SERIALIZER`: it uses Minecraft's `RecipeSerializer` (vectorwing/farmersdelight/common/crafting/FoodServingRecipe.java:20)
- [ ] `FoodServingRecipe.matches(2)`: its class isn't translated (it extends Minecraft's `CustomRecipe`) (vectorwing/farmersdelight/common/crafting/FoodServingRecipe.java:22)
- [ ] `FoodServingRecipe.assemble(1)`: its class isn't translated (it extends Minecraft's `CustomRecipe`) (vectorwing/farmersdelight/common/crafting/FoodServingRecipe.java:50)
- [ ] `FoodServingRecipe.getRemainingItems(1)`: its class isn't translated (it extends Minecraft's `CustomRecipe`) (vectorwing/farmersdelight/common/crafting/FoodServingRecipe.java:63)
- [ ] `FoodServingRecipe.getSerializer(0)`: its class isn't translated (it extends Minecraft's `CustomRecipe`) (vectorwing/farmersdelight/common/crafting/FoodServingRecipe.java:82)

### `ChanceResult`: it uses Minecraft's `ItemStackTemplate`

- [ ] `ChanceResult.EMPTY`: it uses ChanceResult, which isn't translated (it uses Minecraft's `ItemStackTemplate`) (vectorwing/farmersdelight/common/crafting/ingredient/ChanceResult.java:14)
- [ ] `ChanceResult.CODEC`: it uses Minecraft's `Codec` (vectorwing/farmersdelight/common/crafting/ingredient/ChanceResult.java:15)
- [ ] `ChanceResult.STREAM_CODEC`: it uses Minecraft's `StreamCodec` (vectorwing/farmersdelight/common/crafting/ingredient/ChanceResult.java:21)
- [ ] `ChanceResult.ChanceResult(2)`: its class isn't translated (it uses Minecraft's `ItemStackTemplate`) (vectorwing/farmersdelight/common/crafting/ingredient/ChanceResult.java:13)
- [ ] `ChanceResult.rollOutput(2)`: its class isn't translated (it uses Minecraft's `ItemStackTemplate`) (vectorwing/farmersdelight/common/crafting/ingredient/ChanceResult.java:25)
- [ ] `ChanceResult.stack(0)`: its class isn't translated (it uses Minecraft's `ItemStackTemplate`) (vectorwing/farmersdelight/common/crafting/ingredient/ChanceResult.java:13)
- [ ] `ChanceResult.chance(0)`: its class isn't translated (it uses Minecraft's `ItemStackTemplate`) (vectorwing/farmersdelight/common/crafting/ingredient/ChanceResult.java:13)

### `ItemAbilityIngredient`: it implements Fabric's `CustomIngredient`

- [ ] `ItemAbilityIngredient.SERIALIZER`: it uses Serializer, which isn't translated (it implements Fabric's `CustomIngredientSerializer`) (vectorwing/farmersdelight/common/crafting/ingredient/ItemAbilityIngredient.java:22)
- [x] `ItemAbilityIngredient.SERIALIZER_ID` (vectorwing/farmersdelight/common/crafting/ingredient/ItemAbilityIngredient.java:23)
- [ ] `ItemAbilityIngredient.ItemAbilityIngredient(1)`: its class isn't translated (it implements Fabric's `CustomIngredient`) (vectorwing/farmersdelight/common/crafting/ingredient/ItemAbilityIngredient.java:27)
- [ ] `ItemAbilityIngredient.touch(0)`: it uses ItemAbilityIngredient.SERIALIZER, which isn't translated (it uses Serializer, which isn't translated (it implements Fabric's `CustomIngredientSerializer`)) (vectorwing/farmersdelight/common/crafting/ingredient/ItemAbilityIngredient.java:31)
- [ ] `ItemAbilityIngredient.dissolve(0)`: its class isn't translated (it implements Fabric's `CustomIngredient`) (vectorwing/farmersdelight/common/crafting/ingredient/ItemAbilityIngredient.java:35)
- [ ] `ItemAbilityIngredient.test(1)`: its class isn't translated (it implements Fabric's `CustomIngredient`) (vectorwing/farmersdelight/common/crafting/ingredient/ItemAbilityIngredient.java:45)
- [ ] `ItemAbilityIngredient.items(0)`: its class isn't translated (it implements Fabric's `CustomIngredient`) (vectorwing/farmersdelight/common/crafting/ingredient/ItemAbilityIngredient.java:49)
- [ ] `ItemAbilityIngredient.getItemAbility(0)`: its class isn't translated (it implements Fabric's `CustomIngredient`) (vectorwing/farmersdelight/common/crafting/ingredient/ItemAbilityIngredient.java:54)
- [ ] `ItemAbilityIngredient.requiresTesting(0)`: its class isn't translated (it implements Fabric's `CustomIngredient`) (vectorwing/farmersdelight/common/crafting/ingredient/ItemAbilityIngredient.java:58)
- [ ] `ItemAbilityIngredient.getSerializer(0)`: its class isn't translated (it implements Fabric's `CustomIngredient`) (vectorwing/farmersdelight/common/crafting/ingredient/ItemAbilityIngredient.java:62)

### `ItemAbilityIngredient.Serializer`: it implements Fabric's `CustomIngredientSerializer`

- [ ] `ItemAbilityIngredient.Serializer.CODEC`: it uses Minecraft's `MapCodec` (vectorwing/farmersdelight/common/crafting/ingredient/ItemAbilityIngredient.java:67)
- [ ] `ItemAbilityIngredient.Serializer.STREAM_CODEC`: it uses Minecraft's `StreamCodec` (vectorwing/farmersdelight/common/crafting/ingredient/ItemAbilityIngredient.java:70)
- [ ] `ItemAbilityIngredient.Serializer.getIdentifier(0)`: its class isn't translated (it implements Fabric's `CustomIngredientSerializer`) (vectorwing/farmersdelight/common/crafting/ingredient/ItemAbilityIngredient.java:74)
- [ ] `ItemAbilityIngredient.Serializer.getCodec(0)`: its class isn't translated (it implements Fabric's `CustomIngredientSerializer`) (vectorwing/farmersdelight/common/crafting/ingredient/ItemAbilityIngredient.java:78)
- [ ] `ItemAbilityIngredient.Serializer.getStreamCodec(0)`: its class isn't translated (it implements Fabric's `CustomIngredientSerializer`) (vectorwing/farmersdelight/common/crafting/ingredient/ItemAbilityIngredient.java:82)

### `ComfortEffect`: it extends Minecraft's `MobEffect`

- [ ] `ComfortEffect.ComfortEffect(0)`: its class isn't translated (it extends Minecraft's `MobEffect`) (vectorwing/farmersdelight/common/effect/ComfortEffect.java:11)
- [ ] `ComfortEffect.applyEffectTick(3)`: its class isn't translated (it extends Minecraft's `MobEffect`) (vectorwing/farmersdelight/common/effect/ComfortEffect.java:16)
- [ ] `ComfortEffect.shouldApplyEffectTickThisTick(2)`: its class isn't translated (it extends Minecraft's `MobEffect`) (vectorwing/farmersdelight/common/effect/ComfortEffect.java:30)

### `NourishmentEffect`: it extends Minecraft's `MobEffect`

- [ ] `NourishmentEffect.NourishmentEffect(0)`: its class isn't translated (it extends Minecraft's `MobEffect`) (vectorwing/farmersdelight/common/effect/NourishmentEffect.java:13)
- [ ] `NourishmentEffect.applyEffectTick(3)`: its class isn't translated (it extends Minecraft's `MobEffect`) (vectorwing/farmersdelight/common/effect/NourishmentEffect.java:17)
- [ ] `NourishmentEffect.shouldApplyEffectTickThisTick(2)`: its class isn't translated (it extends Minecraft's `MobEffect`) (vectorwing/farmersdelight/common/effect/NourishmentEffect.java:35)

### `RottenTomatoEntity`: it extends Minecraft's `ThrowableItemProjectile`

- [ ] `RottenTomatoEntity.RottenTomatoEntity(2)`: its class isn't translated (it extends Minecraft's `ThrowableItemProjectile`) (vectorwing/farmersdelight/common/entity/RottenTomatoEntity.java:20)
- [ ] `RottenTomatoEntity.RottenTomatoEntity(2)`: its class isn't translated (it extends Minecraft's `ThrowableItemProjectile`) (vectorwing/farmersdelight/common/entity/RottenTomatoEntity.java:24)
- [ ] `RottenTomatoEntity.RottenTomatoEntity(4)`: its class isn't translated (it extends Minecraft's `ThrowableItemProjectile`) (vectorwing/farmersdelight/common/entity/RottenTomatoEntity.java:28)
- [ ] `RottenTomatoEntity.getDefaultItem(0)`: its class isn't translated (it extends Minecraft's `ThrowableItemProjectile`) (vectorwing/farmersdelight/common/entity/RottenTomatoEntity.java:32)
- [ ] `RottenTomatoEntity.handleEntityEvent(1)`: its class isn't translated (it extends Minecraft's `ThrowableItemProjectile`) (vectorwing/farmersdelight/common/entity/RottenTomatoEntity.java:36)
- [ ] `RottenTomatoEntity.onHitEntity(1)`: its class isn't translated (it extends Minecraft's `ThrowableItemProjectile`) (vectorwing/farmersdelight/common/entity/RottenTomatoEntity.java:56)
- [ ] `RottenTomatoEntity.onHit(1)`: its class isn't translated (it extends Minecraft's `ThrowableItemProjectile`) (vectorwing/farmersdelight/common/entity/RottenTomatoEntity.java:63)

### `EnumParameters`


### `CommonEvents`

- [ ] `CommonEvents.onItemUseFinished(3)`: it uses Minecraft's `Level` (vectorwing/farmersdelight/common/event/CommonEvents.java:8)
- [ ] `CommonEvents.handleVanillaSoupEffects(3)`: it uses Minecraft's `Level` (vectorwing/farmersdelight/common/event/CommonEvents.java:13)

### `CommonModBusEvents`

- [ ] `CommonModBusEvents.init(0)`: it uses Fabric's `DefaultItemComponentEvents`.MODIFY (vectorwing/farmersdelight/common/event/CommonModBusEvents.java:24)
- [ ] `CommonModBusEvents.onModifyDefaultComponents(1)`: it uses Fabric's `ModifyContext` (vectorwing/farmersdelight/common/event/CommonModBusEvents.java:28)
- [ ] `CommonModBusEvents.addVanillaSoupEffect(3)`: it uses Minecraft's `Consumable` (vectorwing/farmersdelight/common/event/CommonModBusEvents.java:58)

### `FoodValues`

- [x] `FoodValues.BRIEF_DURATION` (vectorwing/farmersdelight/common/FoodValues.java:20)
- [x] `FoodValues.SHORT_DURATION` (vectorwing/farmersdelight/common/FoodValues.java:21)
- [x] `FoodValues.MEDIUM_DURATION` (vectorwing/farmersdelight/common/FoodValues.java:22)
- [x] `FoodValues.LONG_DURATION` (vectorwing/farmersdelight/common/FoodValues.java:23)
- [ ] `FoodValues.CABBAGE`: it uses Minecraft's `FoodProperties` (vectorwing/farmersdelight/common/FoodValues.java:24)
- [ ] `FoodValues.TOMATO`: it uses Minecraft's `FoodProperties` (vectorwing/farmersdelight/common/FoodValues.java:25)
- [ ] `FoodValues.ONION`: it uses Minecraft's `FoodProperties` (vectorwing/farmersdelight/common/FoodValues.java:26)
- [ ] `FoodValues.APPLE_CIDER`: it uses Minecraft's `FoodProperties` (vectorwing/farmersdelight/common/FoodValues.java:27)
- [ ] `FoodValues.FRIED_EGG`: it uses Minecraft's `FoodProperties` (vectorwing/farmersdelight/common/FoodValues.java:28)
- [ ] `FoodValues.TOMATO_SAUCE`: it uses Minecraft's `FoodProperties` (vectorwing/farmersdelight/common/FoodValues.java:29)
- [ ] `FoodValues.WHEAT_DOUGH`: it uses Minecraft's `FoodProperties` (vectorwing/farmersdelight/common/FoodValues.java:30)
- [ ] `FoodValues.RAW_PASTA`: it uses Minecraft's `FoodProperties` (vectorwing/farmersdelight/common/FoodValues.java:31)
- [ ] `FoodValues.PIE_CRUST`: it uses Minecraft's `FoodProperties` (vectorwing/farmersdelight/common/FoodValues.java:32)
- [ ] `FoodValues.PUMPKIN_SLICE`: it uses Minecraft's `FoodProperties` (vectorwing/farmersdelight/common/FoodValues.java:33)
- [ ] `FoodValues.CABBAGE_LEAF`: it uses Minecraft's `FoodProperties` (vectorwing/farmersdelight/common/FoodValues.java:34)
- [ ] `FoodValues.MINCED_BEEF`: it uses Minecraft's `FoodProperties` (vectorwing/farmersdelight/common/FoodValues.java:35)
- [ ] `FoodValues.BEEF_PATTY`: it uses Minecraft's `FoodProperties` (vectorwing/farmersdelight/common/FoodValues.java:36)
- [ ] `FoodValues.CHICKEN_CUTS`: it uses Minecraft's `FoodProperties` (vectorwing/farmersdelight/common/FoodValues.java:37)
- [ ] `FoodValues.COOKED_CHICKEN_CUTS`: it uses Minecraft's `FoodProperties` (vectorwing/farmersdelight/common/FoodValues.java:38)
- [ ] `FoodValues.BACON`: it uses Minecraft's `FoodProperties` (vectorwing/farmersdelight/common/FoodValues.java:39)
- [ ] `FoodValues.COOKED_BACON`: it uses Minecraft's `FoodProperties` (vectorwing/farmersdelight/common/FoodValues.java:40)
- [ ] `FoodValues.COD_SLICE`: it uses Minecraft's `FoodProperties` (vectorwing/farmersdelight/common/FoodValues.java:41)
- [ ] `FoodValues.COOKED_COD_SLICE`: it uses Minecraft's `FoodProperties` (vectorwing/farmersdelight/common/FoodValues.java:42)
- [ ] `FoodValues.SALMON_SLICE`: it uses Minecraft's `FoodProperties` (vectorwing/farmersdelight/common/FoodValues.java:43)
- [ ] `FoodValues.COOKED_SALMON_SLICE`: it uses Minecraft's `FoodProperties` (vectorwing/farmersdelight/common/FoodValues.java:44)
- [ ] `FoodValues.MUTTON_CHOPS`: it uses Minecraft's `FoodProperties` (vectorwing/farmersdelight/common/FoodValues.java:45)
- [ ] `FoodValues.COOKED_MUTTON_CHOPS`: it uses Minecraft's `FoodProperties` (vectorwing/farmersdelight/common/FoodValues.java:46)
- [ ] `FoodValues.HAM`: it uses Minecraft's `FoodProperties` (vectorwing/farmersdelight/common/FoodValues.java:47)
- [ ] `FoodValues.SMOKED_HAM`: it uses Minecraft's `FoodProperties` (vectorwing/farmersdelight/common/FoodValues.java:48)
- [ ] `FoodValues.POPSICLE`: it uses Minecraft's `FoodProperties` (vectorwing/farmersdelight/common/FoodValues.java:49)
- [ ] `FoodValues.COOKIES`: it uses Minecraft's `FoodProperties` (vectorwing/farmersdelight/common/FoodValues.java:50)
- [ ] `FoodValues.CAKE_SLICE`: it uses Minecraft's `FoodProperties` (vectorwing/farmersdelight/common/FoodValues.java:51)
- [ ] `FoodValues.PIE_SLICE`: it uses Minecraft's `FoodProperties` (vectorwing/farmersdelight/common/FoodValues.java:52)
- [ ] `FoodValues.FRUIT_SALAD`: it uses Minecraft's `FoodProperties` (vectorwing/farmersdelight/common/FoodValues.java:53)
- [ ] `FoodValues.GLOW_BERRY_CUSTARD`: it uses Minecraft's `FoodProperties` (vectorwing/farmersdelight/common/FoodValues.java:54)
- [ ] `FoodValues.MIXED_SALAD`: it uses Minecraft's `FoodProperties` (vectorwing/farmersdelight/common/FoodValues.java:55)
- [ ] `FoodValues.NETHER_SALAD`: it uses Minecraft's `FoodProperties` (vectorwing/farmersdelight/common/FoodValues.java:56)
- [ ] `FoodValues.BARBECUE_STICK`: it uses Minecraft's `FoodProperties` (vectorwing/farmersdelight/common/FoodValues.java:57)
- [ ] `FoodValues.EGG_SANDWICH`: it uses Minecraft's `FoodProperties` (vectorwing/farmersdelight/common/FoodValues.java:58)
- [ ] `FoodValues.CHICKEN_SANDWICH`: it uses Minecraft's `FoodProperties` (vectorwing/farmersdelight/common/FoodValues.java:59)
- [ ] `FoodValues.HAMBURGER`: it uses Minecraft's `FoodProperties` (vectorwing/farmersdelight/common/FoodValues.java:60)
- [ ] `FoodValues.BACON_SANDWICH`: it uses Minecraft's `FoodProperties` (vectorwing/farmersdelight/common/FoodValues.java:61)
- [ ] `FoodValues.MUTTON_WRAP`: it uses Minecraft's `FoodProperties` (vectorwing/farmersdelight/common/FoodValues.java:62)
- [ ] `FoodValues.DUMPLINGS`: it uses Minecraft's `FoodProperties` (vectorwing/farmersdelight/common/FoodValues.java:63)
- [ ] `FoodValues.STUFFED_POTATO`: it uses Minecraft's `FoodProperties` (vectorwing/farmersdelight/common/FoodValues.java:64)
- [ ] `FoodValues.CABBAGE_ROLLS`: it uses Minecraft's `FoodProperties` (vectorwing/farmersdelight/common/FoodValues.java:65)
- [ ] `FoodValues.SALMON_ROLL`: it uses Minecraft's `FoodProperties` (vectorwing/farmersdelight/common/FoodValues.java:66)
- [ ] `FoodValues.COD_ROLL`: it uses Minecraft's `FoodProperties` (vectorwing/farmersdelight/common/FoodValues.java:67)
- [ ] `FoodValues.KELP_ROLL`: it uses Minecraft's `FoodProperties` (vectorwing/farmersdelight/common/FoodValues.java:68)
- [ ] `FoodValues.KELP_ROLL_SLICE`: it uses Minecraft's `FoodProperties` (vectorwing/farmersdelight/common/FoodValues.java:69)
- [ ] `FoodValues.COOKED_RICE`: it uses Minecraft's `FoodProperties` (vectorwing/farmersdelight/common/FoodValues.java:70)
- [ ] `FoodValues.BONE_BROTH`: it uses Minecraft's `FoodProperties` (vectorwing/farmersdelight/common/FoodValues.java:71)
- [ ] `FoodValues.BEEF_STEW`: it uses Minecraft's `FoodProperties` (vectorwing/farmersdelight/common/FoodValues.java:72)
- [ ] `FoodValues.VEGETABLE_SOUP`: it uses Minecraft's `FoodProperties` (vectorwing/farmersdelight/common/FoodValues.java:73)
- [ ] `FoodValues.FISH_STEW`: it uses Minecraft's `FoodProperties` (vectorwing/farmersdelight/common/FoodValues.java:74)
- [ ] `FoodValues.CHICKEN_SOUP`: it uses Minecraft's `FoodProperties` (vectorwing/farmersdelight/common/FoodValues.java:75)
- [ ] `FoodValues.FRIED_RICE`: it uses Minecraft's `FoodProperties` (vectorwing/farmersdelight/common/FoodValues.java:76)
- [ ] `FoodValues.PUMPKIN_SOUP`: it uses Minecraft's `FoodProperties` (vectorwing/farmersdelight/common/FoodValues.java:77)
- [ ] `FoodValues.BAKED_COD_STEW`: it uses Minecraft's `FoodProperties` (vectorwing/farmersdelight/common/FoodValues.java:78)
- [ ] `FoodValues.NOODLE_SOUP`: it uses Minecraft's `FoodProperties` (vectorwing/farmersdelight/common/FoodValues.java:79)
- [ ] `FoodValues.BACON_AND_EGGS`: it uses Minecraft's `FoodProperties` (vectorwing/farmersdelight/common/FoodValues.java:80)
- [ ] `FoodValues.RATATOUILLE`: it uses Minecraft's `FoodProperties` (vectorwing/farmersdelight/common/FoodValues.java:81)
- [ ] `FoodValues.STEAK_AND_POTATOES`: it uses Minecraft's `FoodProperties` (vectorwing/farmersdelight/common/FoodValues.java:82)
- [ ] `FoodValues.PASTA_WITH_MEATBALLS`: it uses Minecraft's `FoodProperties` (vectorwing/farmersdelight/common/FoodValues.java:83)
- [ ] `FoodValues.PASTA_WITH_MUTTON_CHOP`: it uses Minecraft's `FoodProperties` (vectorwing/farmersdelight/common/FoodValues.java:84)
- [ ] `FoodValues.MUSHROOM_RICE`: it uses Minecraft's `FoodProperties` (vectorwing/farmersdelight/common/FoodValues.java:85)
- [ ] `FoodValues.ROASTED_MUTTON_CHOPS`: it uses Minecraft's `FoodProperties` (vectorwing/farmersdelight/common/FoodValues.java:86)
- [ ] `FoodValues.VEGETABLE_NOODLES`: it uses Minecraft's `FoodProperties` (vectorwing/farmersdelight/common/FoodValues.java:87)
- [ ] `FoodValues.SQUID_INK_PASTA`: it uses Minecraft's `FoodProperties` (vectorwing/farmersdelight/common/FoodValues.java:88)
- [ ] `FoodValues.GRILLED_SALMON`: it uses Minecraft's `FoodProperties` (vectorwing/farmersdelight/common/FoodValues.java:89)
- [ ] `FoodValues.ROAST_CHICKEN`: it uses Minecraft's `FoodProperties` (vectorwing/farmersdelight/common/FoodValues.java:90)
- [ ] `FoodValues.STUFFED_PUMPKIN`: it uses Minecraft's `FoodProperties` (vectorwing/farmersdelight/common/FoodValues.java:91)
- [ ] `FoodValues.HONEY_GLAZED_HAM`: it uses Minecraft's `FoodProperties` (vectorwing/farmersdelight/common/FoodValues.java:92)
- [ ] `FoodValues.SHEPHERDS_PIE`: it uses Minecraft's `FoodProperties` (vectorwing/farmersdelight/common/FoodValues.java:93)
- [ ] `FoodValues.GLEAMING_SALAD`: it uses Minecraft's `FoodProperties` (vectorwing/farmersdelight/common/FoodValues.java:94)
- [ ] `FoodValues.DOG_FOOD`: it uses Minecraft's `FoodProperties` (vectorwing/farmersdelight/common/FoodValues.java:95)
- [ ] `FoodValues.RABBIT_STEW_BUFF`: it uses Minecraft's `FoodProperties` (vectorwing/farmersdelight/common/FoodValues.java:96)
- [ ] `FoodValues.nourishment(1)`: it uses Minecraft's `MobEffectInstance` (vectorwing/farmersdelight/common/FoodValues.java:98)

### `FoodValues.ConsumableValues`

- [ ] `FoodValues.ConsumableValues.FAST_FOOD`: it uses Minecraft's `Consumable` (vectorwing/farmersdelight/common/FoodValues.java:103)
- [ ] `FoodValues.ConsumableValues.HUNGER_INDUCING_FOOD`: it uses Minecraft's `Consumable` (vectorwing/farmersdelight/common/FoodValues.java:104)
- [ ] `FoodValues.ConsumableValues.NOURISHMENT_BRIEF_DURATION`: it uses Minecraft's `Consumable` (vectorwing/farmersdelight/common/FoodValues.java:107)
- [ ] `FoodValues.ConsumableValues.NOURISHMENT_SHORT_DURATION`: it uses Minecraft's `Consumable` (vectorwing/farmersdelight/common/FoodValues.java:110)
- [ ] `FoodValues.ConsumableValues.NOURISHMENT_MEDIUM_DURATION`: it uses Minecraft's `Consumable` (vectorwing/farmersdelight/common/FoodValues.java:113)
- [ ] `FoodValues.ConsumableValues.NOURISHMENT_LONG_DURATION`: it uses Minecraft's `Consumable` (vectorwing/farmersdelight/common/FoodValues.java:116)
- [ ] `FoodValues.ConsumableValues.BONE_BROTH`: it uses Minecraft's `Consumable` (vectorwing/farmersdelight/common/FoodValues.java:119)
- [ ] `FoodValues.ConsumableValues.HOT_COCOA`: it uses Minecraft's `Consumable` (vectorwing/farmersdelight/common/FoodValues.java:122)
- [ ] `FoodValues.ConsumableValues.MILK_BOTTLE`: it uses Minecraft's `Consumable` (vectorwing/farmersdelight/common/FoodValues.java:125)
- [ ] `FoodValues.ConsumableValues.MELON_JUICE`: it uses Minecraft's `Consumable` (vectorwing/farmersdelight/common/FoodValues.java:128)
- [ ] `FoodValues.ConsumableValues.MELON_POPSICLE`: it uses Minecraft's `Consumable` (vectorwing/farmersdelight/common/FoodValues.java:129)
- [ ] `FoodValues.ConsumableValues.APPLE_CIDER`: it uses Minecraft's `Consumable` (vectorwing/farmersdelight/common/FoodValues.java:130)
- [ ] `FoodValues.ConsumableValues.CAKE_SLICE`: it uses Minecraft's `Consumable` (vectorwing/farmersdelight/common/FoodValues.java:133)
- [ ] `FoodValues.ConsumableValues.PIE_SLICE`: it uses Minecraft's `Consumable` (vectorwing/farmersdelight/common/FoodValues.java:136)
- [ ] `FoodValues.ConsumableValues.CHICKEN_CUTS`: it uses Minecraft's `Consumable` (vectorwing/farmersdelight/common/FoodValues.java:139)
- [ ] `FoodValues.ConsumableValues.GLOW_BERRY_CUSTARD`: it uses Minecraft's `Consumable` (vectorwing/farmersdelight/common/FoodValues.java:143)
- [ ] `FoodValues.ConsumableValues.FRUIT_SALAD`: it uses Minecraft's `Consumable` (vectorwing/farmersdelight/common/FoodValues.java:146)
- [ ] `FoodValues.ConsumableValues.NETHER_SALAD`: it uses Minecraft's `Consumable` (vectorwing/farmersdelight/common/FoodValues.java:149)
- [ ] `FoodValues.ConsumableValues.MIXED_SALAD`: it uses Minecraft's `Consumable` (vectorwing/farmersdelight/common/FoodValues.java:152)
- [ ] `FoodValues.ConsumableValues.KELP_ROLL`: it uses Minecraft's `Consumable` (vectorwing/farmersdelight/common/FoodValues.java:155)
- [ ] `FoodValues.ConsumableValues.VANILLA_SOUP_EFFECTS`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/FoodValues.java:156)

### `ExtinguishConsumeEffect`: it implements Minecraft's `ConsumeEffect`

- [ ] `ExtinguishConsumeEffect.CODEC`: it uses Minecraft's `MapCodec` (vectorwing/farmersdelight/common/item/component/consumable/ExtinguishConsumeEffect.java:14)
- [ ] `ExtinguishConsumeEffect.STREAM_CODEC`: it uses Minecraft's `StreamCodec` (vectorwing/farmersdelight/common/item/component/consumable/ExtinguishConsumeEffect.java:15)
- [ ] `ExtinguishConsumeEffect.ExtinguishConsumeEffect(0)`: its class isn't translated (it implements Minecraft's `ConsumeEffect`) (vectorwing/farmersdelight/common/item/component/consumable/ExtinguishConsumeEffect.java:13)
- [ ] `ExtinguishConsumeEffect.getType(0)`: its class isn't translated (it implements Minecraft's `ConsumeEffect`) (vectorwing/farmersdelight/common/item/component/consumable/ExtinguishConsumeEffect.java:17)
- [ ] `ExtinguishConsumeEffect.apply(3)`: its class isn't translated (it implements Minecraft's `ConsumeEffect`) (vectorwing/farmersdelight/common/item/component/consumable/ExtinguishConsumeEffect.java:21)

### `HealConsumeEffect`: it implements Minecraft's `ConsumeEffect`

- [ ] `HealConsumeEffect.CODEC`: it uses Minecraft's `MapCodec` (vectorwing/farmersdelight/common/item/component/consumable/HealConsumeEffect.java:17)
- [ ] `HealConsumeEffect.STREAM_CODEC`: it uses Minecraft's `StreamCodec` (vectorwing/farmersdelight/common/item/component/consumable/HealConsumeEffect.java:20)
- [ ] `HealConsumeEffect.HealConsumeEffect(1)`: its class isn't translated (it implements Minecraft's `ConsumeEffect`) (vectorwing/farmersdelight/common/item/component/consumable/HealConsumeEffect.java:16)
- [ ] `HealConsumeEffect.getType(0)`: its class isn't translated (it implements Minecraft's `ConsumeEffect`) (vectorwing/farmersdelight/common/item/component/consumable/HealConsumeEffect.java:24)
- [ ] `HealConsumeEffect.apply(3)`: its class isn't translated (it implements Minecraft's `ConsumeEffect`) (vectorwing/farmersdelight/common/item/component/consumable/HealConsumeEffect.java:28)
- [ ] `HealConsumeEffect.amount(0)`: its class isn't translated (it implements Minecraft's `ConsumeEffect`) (vectorwing/farmersdelight/common/item/component/consumable/HealConsumeEffect.java:16)

### `RemoveRandomStatusEffectsConsumeEffect`: it implements Minecraft's `ConsumeEffect`

- [ ] `RemoveRandomStatusEffectsConsumeEffect.CODEC`: it uses Minecraft's `MapCodec` (vectorwing/farmersdelight/common/item/component/consumable/RemoveRandomStatusEffectsConsumeEffect.java:25)
- [ ] `RemoveRandomStatusEffectsConsumeEffect.STREAM_CODEC`: it uses Minecraft's `StreamCodec` (vectorwing/farmersdelight/common/item/component/consumable/RemoveRandomStatusEffectsConsumeEffect.java:32)
- [ ] `RemoveRandomStatusEffectsConsumeEffect.RemoveRandomStatusEffectsConsumeEffect(2)`: its class isn't translated (it implements Minecraft's `ConsumeEffect`) (vectorwing/farmersdelight/common/item/component/consumable/RemoveRandomStatusEffectsConsumeEffect.java:24)
- [ ] `RemoveRandomStatusEffectsConsumeEffect.RemoveRandomStatusEffectsConsumeEffect(1)`: its class isn't translated (it implements Minecraft's `ConsumeEffect`) (vectorwing/farmersdelight/common/item/component/consumable/RemoveRandomStatusEffectsConsumeEffect.java:40)
- [ ] `RemoveRandomStatusEffectsConsumeEffect.getType(0)`: its class isn't translated (it implements Minecraft's `ConsumeEffect`) (vectorwing/farmersdelight/common/item/component/consumable/RemoveRandomStatusEffectsConsumeEffect.java:44)
- [ ] `RemoveRandomStatusEffectsConsumeEffect.apply(3)`: its class isn't translated (it implements Minecraft's `ConsumeEffect`) (vectorwing/farmersdelight/common/item/component/consumable/RemoveRandomStatusEffectsConsumeEffect.java:48)
- [ ] `RemoveRandomStatusEffectsConsumeEffect.excluded(0)`: its class isn't translated (it implements Minecraft's `ConsumeEffect`) (vectorwing/farmersdelight/common/item/component/consumable/RemoveRandomStatusEffectsConsumeEffect.java:24)
- [ ] `RemoveRandomStatusEffectsConsumeEffect.harmfulOnly(0)`: its class isn't translated (it implements Minecraft's `ConsumeEffect`) (vectorwing/farmersdelight/common/item/component/consumable/RemoveRandomStatusEffectsConsumeEffect.java:24)

### `ItemStackWrapper`: it uses Minecraft's `ItemStack`

- [ ] `ItemStackWrapper.EMPTY`: it uses ItemStackWrapper, which isn't translated (it uses Minecraft's `ItemStack`) (vectorwing/farmersdelight/common/item/component/ItemStackWrapper.java:9)
- [ ] `ItemStackWrapper.CODEC`: it uses Minecraft's `Codec` (vectorwing/farmersdelight/common/item/component/ItemStackWrapper.java:10)
- [ ] `ItemStackWrapper.STREAM_CODEC`: it uses Minecraft's `StreamCodec` (vectorwing/farmersdelight/common/item/component/ItemStackWrapper.java:11)
- [ ] `ItemStackWrapper.ItemStackWrapper(1)`: its class isn't translated (it uses Minecraft's `ItemStack`) (vectorwing/farmersdelight/common/item/component/ItemStackWrapper.java:16)
- [ ] `ItemStackWrapper.getStack(0)`: its class isn't translated (it uses Minecraft's `ItemStack`) (vectorwing/farmersdelight/common/item/component/ItemStackWrapper.java:21)
- [ ] `ItemStackWrapper.hashCode(0)`: its class isn't translated (it uses Minecraft's `ItemStack`) (vectorwing/farmersdelight/common/item/component/ItemStackWrapper.java:25)
- [ ] `ItemStackWrapper.equals(1)`: its class isn't translated (it uses Minecraft's `ItemStack`) (vectorwing/farmersdelight/common/item/component/ItemStackWrapper.java:30)

### `ConsumableItem`: it extends Minecraft's `Item`

- [ ] `ConsumableItem.ConsumableItem(1)`: its class isn't translated (it extends Minecraft's `Item`) (vectorwing/farmersdelight/common/item/ConsumableItem.java:30)
- [ ] `ConsumableItem.ConsumableItem(2)`: its class isn't translated (it extends Minecraft's `Item`) (vectorwing/farmersdelight/common/item/ConsumableItem.java:36)
- [ ] `ConsumableItem.ConsumableItem(3)`: its class isn't translated (it extends Minecraft's `Item`) (vectorwing/farmersdelight/common/item/ConsumableItem.java:42)
- [ ] `ConsumableItem.finishUsingItem(3)`: its class isn't translated (it extends Minecraft's `Item`) (vectorwing/farmersdelight/common/item/ConsumableItem.java:48)
- [ ] `ConsumableItem.affectConsumer(3)`: its class isn't translated (it extends Minecraft's `Item`) (vectorwing/farmersdelight/common/item/ConsumableItem.java:83)
- [ ] `ConsumableItem.appendHoverText(5)`: its class isn't translated (it extends Minecraft's `Item`) (vectorwing/farmersdelight/common/item/ConsumableItem.java:86)

### `CookingPotItem`: it extends Minecraft's `BlockItem`

- [ ] `CookingPotItem.BAR_COLOR`: it calls Minecraft's `ARGB`.colorFromFloat (vectorwing/farmersdelight/common/item/CookingPotItem.java:14)
- [ ] `CookingPotItem.CookingPotItem(2)`: its class isn't translated (it extends Minecraft's `BlockItem`) (vectorwing/farmersdelight/common/item/CookingPotItem.java:16)
- [ ] `CookingPotItem.isBarVisible(1)`: its class isn't translated (it extends Minecraft's `BlockItem`) (vectorwing/farmersdelight/common/item/CookingPotItem.java:20)
- [ ] `CookingPotItem.getBarWidth(1)`: its class isn't translated (it extends Minecraft's `BlockItem`) (vectorwing/farmersdelight/common/item/CookingPotItem.java:24)
- [ ] `CookingPotItem.getBarColor(1)`: its class isn't translated (it extends Minecraft's `BlockItem`) (vectorwing/farmersdelight/common/item/CookingPotItem.java:28)
- [ ] `CookingPotItem.getTooltipImage(1)`: its class isn't translated (it extends Minecraft's `BlockItem`) (vectorwing/farmersdelight/common/item/CookingPotItem.java:32)
- [ ] `CookingPotItem.getServingCount(1)`: it calls CookingPotBlockEntity.getMealFromItem, which isn't translated (vectorwing/farmersdelight/common/item/CookingPotItem.java:37)

### `DogFoodItem`: it extends `ConsumableItem`

- [ ] `DogFoodItem.EFFECTS`: it uses Minecraft's `MobEffectInstance` (vectorwing/farmersdelight/common/item/DogFoodItem.java:40)
- [ ] `DogFoodItem.DogFoodItem(1)`: its class isn't translated (it extends `ConsumableItem`) (vectorwing/farmersdelight/common/item/DogFoodItem.java:46)
- [ ] `DogFoodItem.init(0)`: it uses Fabric's `UseEntityCallback`.EVENT (vectorwing/farmersdelight/common/item/DogFoodItem.java:50)
- [ ] `DogFoodItem.appendHoverText(5)`: its class isn't translated (it extends `ConsumableItem`) (vectorwing/farmersdelight/common/item/DogFoodItem.java:54)
- [ ] `DogFoodItem.interactLivingEntity(4)`: its class isn't translated (it extends `ConsumableItem`) (vectorwing/farmersdelight/common/item/DogFoodItem.java:78)

### `DogFoodItem.DogFoodEvent`

- [ ] `DogFoodItem.DogFoodEvent.onDogFoodApplied(5)`: it uses Minecraft's `InteractionResult` (vectorwing/farmersdelight/common/item/DogFoodItem.java:83)

### `BackstabbingEnchantment`

- [ ] `BackstabbingEnchantment.isLookingBehindTarget(2)`: it uses Minecraft's `LivingEntity` (vectorwing/farmersdelight/common/item/enchantment/BackstabbingEnchantment.java:17)
- [x] `BackstabbingEnchantment.getBackstabbingDamagePerLevel(2)` (vectorwing/farmersdelight/common/item/enchantment/BackstabbingEnchantment.java:28)

### `BackstabbingEnchantment.BackstabbingEvent`

- [ ] `BackstabbingEnchantment.BackstabbingEvent.onKnifeBackstab(3)`: it uses Minecraft's `LivingEntity` (vectorwing/farmersdelight/common/item/enchantment/BackstabbingEnchantment.java:34)

### `FuelBlockItem`: it extends Minecraft's `BlockItem`

- [ ] `FuelBlockItem.FuelBlockItem(2)`: its class isn't translated (it extends Minecraft's `BlockItem`) (vectorwing/farmersdelight/common/item/FuelBlockItem.java:18)
- [ ] `FuelBlockItem.FuelBlockItem(3)`: its class isn't translated (it extends Minecraft's `BlockItem`) (vectorwing/farmersdelight/common/item/FuelBlockItem.java:22)
- [ ] `FuelBlockItem.getBurnTime(2)`: its class isn't translated (it extends Minecraft's `BlockItem`) (vectorwing/farmersdelight/common/item/FuelBlockItem.java:33)

### `FuelItem`: it extends Minecraft's `Item`

- [ ] `FuelItem.FuelItem(1)`: its class isn't translated (it extends Minecraft's `Item`) (vectorwing/farmersdelight/common/item/FuelItem.java:17)
- [ ] `FuelItem.FuelItem(2)`: its class isn't translated (it extends Minecraft's `Item`) (vectorwing/farmersdelight/common/item/FuelItem.java:21)
- [ ] `FuelItem.getBurnTime(2)`: its class isn't translated (it extends Minecraft's `Item`) (vectorwing/farmersdelight/common/item/FuelItem.java:31)

### `HorseFeedItem`: it extends Minecraft's `Item`

- [ ] `HorseFeedItem.EFFECTS`: it uses Minecraft's `MobEffectInstance` (vectorwing/farmersdelight/common/item/HorseFeedItem.java:40)
- [ ] `HorseFeedItem.HorseFeedItem(1)`: its class isn't translated (it extends Minecraft's `Item`) (vectorwing/farmersdelight/common/item/HorseFeedItem.java:44)
- [ ] `HorseFeedItem.init(0)`: it uses Fabric's `UseEntityCallback`.EVENT (vectorwing/farmersdelight/common/item/HorseFeedItem.java:48)
- [ ] `HorseFeedItem.appendHoverText(5)`: its class isn't translated (it extends Minecraft's `Item`) (vectorwing/farmersdelight/common/item/HorseFeedItem.java:52)
- [ ] `HorseFeedItem.interactLivingEntity(4)`: its class isn't translated (it extends Minecraft's `Item`) (vectorwing/farmersdelight/common/item/HorseFeedItem.java:75)

### `HorseFeedItem.HorseFeedEvent`

- [ ] `HorseFeedItem.HorseFeedEvent.onHorseFeedApplied(5)`: it uses Minecraft's `InteractionResult` (vectorwing/farmersdelight/common/item/HorseFeedItem.java:80)

### `KnifeItem`: it extends Minecraft's `Item`

- [ ] `KnifeItem.KNIFE_DIG`: it uses ItemAbility, which isn't translated (it implements Minecraft's `StringRepresentable`) (vectorwing/farmersdelight/common/item/KnifeItem.java:44)
- [ ] `KnifeItem.KNIFE_HARVEST`: it uses ItemAbility, which isn't translated (it implements Minecraft's `StringRepresentable`) (vectorwing/farmersdelight/common/item/KnifeItem.java:45)
- [ ] `KnifeItem.KNIFE_ACTIONS`: it uses ItemAbility, which isn't translated (it implements Minecraft's `StringRepresentable`) (vectorwing/farmersdelight/common/item/KnifeItem.java:46)
- [ ] `KnifeItem.KnifeItem(1)`: its class isn't translated (it extends Minecraft's `Item`) (vectorwing/farmersdelight/common/item/KnifeItem.java:48)
- [ ] `KnifeItem.init(0)`: it uses Fabric's `UseBlockCallback`.EVENT (vectorwing/farmersdelight/common/item/KnifeItem.java:52)
- [ ] `KnifeItem.createAttributes(3)`: it uses Minecraft's `ItemAttributeModifiers` (vectorwing/farmersdelight/common/item/KnifeItem.java:56)
- [ ] `KnifeItem.canBeEnchantedWith(3)`: its class isn't translated (it extends Minecraft's `Item`) (vectorwing/farmersdelight/common/item/KnifeItem.java:67)
- [ ] `KnifeItem.useOn(1)`: its class isn't translated (it extends Minecraft's `Item`) (vectorwing/farmersdelight/common/item/KnifeItem.java:71)

### `KnifeItem.KnifeEvents`

- [ ] `KnifeItem.KnifeEvents.onKnifeKnockback(2)`: it uses Minecraft's `LivingEntity` (vectorwing/farmersdelight/common/item/KnifeItem.java:104)
- [ ] `KnifeItem.KnifeEvents.onCakeInteraction(4)`: it uses Minecraft's `InteractionResult` (vectorwing/farmersdelight/common/item/KnifeItem.java:114)

### `MushroomColonyItem`: it extends Minecraft's `BlockItem`

- [ ] `MushroomColonyItem.MushroomColonyItem(2)`: its class isn't translated (it extends Minecraft's `BlockItem`) (vectorwing/farmersdelight/common/item/MushroomColonyItem.java:12)
- [ ] `MushroomColonyItem.getPlacementState(1)`: its class isn't translated (it extends Minecraft's `BlockItem`) (vectorwing/farmersdelight/common/item/MushroomColonyItem.java:16)

### `PlaceableItem`: it extends Minecraft's `BlockItem`

- [ ] `PlaceableItem.PlaceableItem(2)`: its class isn't translated (it extends Minecraft's `BlockItem`) (vectorwing/farmersdelight/common/item/PlaceableItem.java:15)
- [ ] `PlaceableItem.appendHoverText(5)`: its class isn't translated (it extends Minecraft's `BlockItem`) (vectorwing/farmersdelight/common/item/PlaceableItem.java:19)

### `RiceItem`: it extends Minecraft's `BlockItem`

- [ ] `RiceItem.RiceItem(2)`: its class isn't translated (it extends Minecraft's `BlockItem`) (vectorwing/farmersdelight/common/item/RiceItem.java:16)
- [ ] `RiceItem.useOn(1)`: its class isn't translated (it extends Minecraft's `BlockItem`) (vectorwing/farmersdelight/common/item/RiceItem.java:20)

### `RopeItem`: it extends Minecraft's `BlockItem`

- [ ] `RopeItem.RopeItem(2)`: its class isn't translated (it extends Minecraft's `BlockItem`) (vectorwing/farmersdelight/common/item/RopeItem.java:17)
- [ ] `RopeItem.updatePlacementContext(1)`: its class isn't translated (it extends Minecraft's `BlockItem`) (vectorwing/farmersdelight/common/item/RopeItem.java:21)
- [ ] `RopeItem.mustSurvive(0)`: its class isn't translated (it extends Minecraft's `BlockItem`) (vectorwing/farmersdelight/common/item/RopeItem.java:62)

### `RottenTomatoItem`: it extends Minecraft's `Item`

- [ ] `RottenTomatoItem.RottenTomatoItem(1)`: its class isn't translated (it extends Minecraft's `Item`) (vectorwing/farmersdelight/common/item/RottenTomatoItem.java:20)
- [ ] `RottenTomatoItem.use(3)`: its class isn't translated (it extends Minecraft's `Item`) (vectorwing/farmersdelight/common/item/RottenTomatoItem.java:24)
- [ ] `RottenTomatoItem.asProjectile(4)`: its class isn't translated (it extends Minecraft's `Item`) (vectorwing/farmersdelight/common/item/RottenTomatoItem.java:51)

### `SkilletItem`: it extends Minecraft's `BlockItem`

- [x] `SkilletItem.FLIP_TIME` (vectorwing/farmersdelight/common/item/SkilletItem.java:60)
- [ ] `SkilletItem.SKILLET_MATERIAL`: it uses Minecraft's `ToolMaterial` (vectorwing/farmersdelight/common/item/SkilletItem.java:61)
- [x] `SkilletItem.FD_ATTACK_KNOCKBACK_UUID` (vectorwing/farmersdelight/common/item/SkilletItem.java:62)
- [ ] `SkilletItem.SkilletItem(2)`: its class isn't translated (it extends Minecraft's `BlockItem`) (vectorwing/farmersdelight/common/item/SkilletItem.java:64)
- [ ] `SkilletItem.createAttributes(3)`: it uses Minecraft's `ItemAttributeModifiers` (vectorwing/farmersdelight/common/item/SkilletItem.java:68)
- [ ] `SkilletItem.allowComponentsUpdateAnimation(4)`: its class isn't translated (it extends Minecraft's `BlockItem`) (vectorwing/farmersdelight/common/item/SkilletItem.java:80)
- [ ] `SkilletItem.compareComponents(3)`: it uses Minecraft's `DataComponentType` (vectorwing/farmersdelight/common/item/SkilletItem.java:88)
- [ ] `SkilletItem.postHurtEnemy(3)`: its class isn't translated (it extends Minecraft's `BlockItem`) (vectorwing/farmersdelight/common/item/SkilletItem.java:92)
- [ ] `SkilletItem.isPlayerNearHeatSource(2)`: it uses Minecraft's `LevelReader` (vectorwing/farmersdelight/common/item/SkilletItem.java:96)
- [ ] `SkilletItem.appendHoverText(5)`: its class isn't translated (it extends Minecraft's `BlockItem`) (vectorwing/farmersdelight/common/item/SkilletItem.java:112)
- [ ] `SkilletItem.getUseDuration(2)`: its class isn't translated (it extends Minecraft's `BlockItem`) (vectorwing/farmersdelight/common/item/SkilletItem.java:116)
- [ ] `SkilletItem.use(3)`: its class isn't translated (it extends Minecraft's `BlockItem`) (vectorwing/farmersdelight/common/item/SkilletItem.java:122)
- [ ] `SkilletItem.onUseTick(4)`: its class isn't translated (it extends Minecraft's `BlockItem`) (vectorwing/farmersdelight/common/item/SkilletItem.java:163)
- [ ] `SkilletItem.releaseUsing(4)`: its class isn't translated (it extends Minecraft's `BlockItem`) (vectorwing/farmersdelight/common/item/SkilletItem.java:182)
- [ ] `SkilletItem.finishUsingItem(3)`: its class isn't translated (it extends Minecraft's `BlockItem`) (vectorwing/farmersdelight/common/item/SkilletItem.java:198)
- [ ] `SkilletItem.getBarWidth(1)`: its class isn't translated (it extends Minecraft's `BlockItem`) (vectorwing/farmersdelight/common/item/SkilletItem.java:227)
- [x] `SkilletItem.getClientPlayerHack(0)` in game_side.rs (vectorwing/farmersdelight/common/item/SkilletItem.java:242)
- [ ] `SkilletItem.getBarColor(1)`: its class isn't translated (it extends Minecraft's `BlockItem`) (vectorwing/farmersdelight/common/item/SkilletItem.java:247)
- [ ] `SkilletItem.isBarVisible(1)`: its class isn't translated (it extends Minecraft's `BlockItem`) (vectorwing/farmersdelight/common/item/SkilletItem.java:251)
- [ ] `SkilletItem.getCookingRecipe(2)`: it uses Minecraft's `RecipeHolder` (vectorwing/farmersdelight/common/item/SkilletItem.java:255)
- [ ] `SkilletItem.mineBlock(5)`: its class isn't translated (it extends Minecraft's `BlockItem`) (vectorwing/farmersdelight/common/item/SkilletItem.java:259)
- [ ] `SkilletItem.place(1)`: its class isn't translated (it extends Minecraft's `BlockItem`) (vectorwing/farmersdelight/common/item/SkilletItem.java:267)
- [ ] `SkilletItem.canBeEnchantedWith(3)`: its class isn't translated (it extends Minecraft's `BlockItem`) (vectorwing/farmersdelight/common/item/SkilletItem.java:272)

### `SkilletItem.SkilletEvents`

- [x] `SkilletItem.SkilletEvents.attackPower` (vectorwing/farmersdelight/common/item/SkilletItem.java:277)
- [ ] `SkilletItem.SkilletEvents.playSkilletAttackSound(2)`: it uses Minecraft's `LivingEntity` (vectorwing/farmersdelight/common/item/SkilletItem.java:279)

### `CopySkilletFunction`: it extends Minecraft's `LootItemConditionalFunction`

- [x] `CopySkilletFunction.ID` (vectorwing/farmersdelight/common/loot/function/CopySkilletFunction.java:18)
- [ ] `CopySkilletFunction.CODEC`: it uses Minecraft's `MapCodec` (vectorwing/farmersdelight/common/loot/function/CopySkilletFunction.java:19)
- [ ] `CopySkilletFunction.CopySkilletFunction(1)`: its class isn't translated (it extends Minecraft's `LootItemConditionalFunction`) (vectorwing/farmersdelight/common/loot/function/CopySkilletFunction.java:23)
- [ ] `CopySkilletFunction.builder(0)`: it uses Minecraft's `Builder` (vectorwing/farmersdelight/common/loot/function/CopySkilletFunction.java:27)
- [ ] `CopySkilletFunction.run(2)`: its class isn't translated (it extends Minecraft's `LootItemConditionalFunction`) (vectorwing/farmersdelight/common/loot/function/CopySkilletFunction.java:31)
- [ ] `CopySkilletFunction.codec(0)`: its class isn't translated (it extends Minecraft's `LootItemConditionalFunction`) (vectorwing/farmersdelight/common/loot/function/CopySkilletFunction.java:40)

### `SmokerCookFunction`: it extends Minecraft's `LootItemConditionalFunction`

- [x] `SmokerCookFunction.ID` (vectorwing/farmersdelight/common/loot/function/SmokerCookFunction.java:18)
- [ ] `SmokerCookFunction.CODEC`: it uses Minecraft's `MapCodec` (vectorwing/farmersdelight/common/loot/function/SmokerCookFunction.java:19)
- [ ] `SmokerCookFunction.SmokerCookFunction(1)`: its class isn't translated (it extends Minecraft's `LootItemConditionalFunction`) (vectorwing/farmersdelight/common/loot/function/SmokerCookFunction.java:23)
- [ ] `SmokerCookFunction.run(2)`: its class isn't translated (it extends Minecraft's `LootItemConditionalFunction`) (vectorwing/farmersdelight/common/loot/function/SmokerCookFunction.java:27)
- [ ] `SmokerCookFunction.codec(0)`: its class isn't translated (it extends Minecraft's `LootItemConditionalFunction`) (vectorwing/farmersdelight/common/loot/function/SmokerCookFunction.java:44)

### `AddItemModifier`: it extends `LootModifier` (vectorwing.farmersdelight.refabricated)

- [ ] `AddItemModifier.AddItemModifier(3)`: its class isn't translated (it extends `LootModifier` (vectorwing.farmersdelight.refabricated)) (vectorwing/farmersdelight/common/loot/modifier/AddItemModifier.java:15)
- [ ] `AddItemModifier.doApply(2)`: its class isn't translated (it extends `LootModifier` (vectorwing.farmersdelight.refabricated)) (vectorwing/farmersdelight/common/loot/modifier/AddItemModifier.java:21)

### `FDAddTableLootModifier`: it extends `LootModifier` (vectorwing.farmersdelight.refabricated)

- [ ] `FDAddTableLootModifier.FDAddTableLootModifier(2)`: its class isn't translated (it extends `LootModifier` (vectorwing.farmersdelight.refabricated)) (vectorwing/farmersdelight/common/loot/modifier/FDAddTableLootModifier.java:16)
- [ ] `FDAddTableLootModifier.doApply(2)`: its class isn't translated (it extends `LootModifier` (vectorwing.farmersdelight.refabricated)) (vectorwing/farmersdelight/common/loot/modifier/FDAddTableLootModifier.java:21)
- [ ] `FDAddTableLootModifier.codec(0)`: its class isn't translated (it extends `LootModifier` (vectorwing.farmersdelight.refabricated)) (vectorwing/farmersdelight/common/loot/modifier/FDAddTableLootModifier.java:29)

### `PastrySlicingModifier`: it extends `LootModifier` (vectorwing.farmersdelight.refabricated)

- [x] `PastrySlicingModifier.MAX_CAKE_BITES` (vectorwing/farmersdelight/common/loot/modifier/PastrySlicingModifier.java:17)
- [x] `PastrySlicingModifier.MAX_PIE_BITES` (vectorwing/farmersdelight/common/loot/modifier/PastrySlicingModifier.java:18)
- [ ] `PastrySlicingModifier.PastrySlicingModifier(2)`: its class isn't translated (it extends `LootModifier` (vectorwing.farmersdelight.refabricated)) (vectorwing/farmersdelight/common/loot/modifier/PastrySlicingModifier.java:21)
- [ ] `PastrySlicingModifier.doApply(2)`: its class isn't translated (it extends `LootModifier` (vectorwing.farmersdelight.refabricated)) (vectorwing/farmersdelight/common/loot/modifier/PastrySlicingModifier.java:26)

### `ReplaceItemModifier`: it extends `LootModifier` (vectorwing.farmersdelight.refabricated)

- [ ] `ReplaceItemModifier.ReplaceItemModifier(4)`: its class isn't translated (it extends `LootModifier` (vectorwing.farmersdelight.refabricated)) (vectorwing/farmersdelight/common/loot/modifier/ReplaceItemModifier.java:16)
- [ ] `ReplaceItemModifier.doApply(2)`: its class isn't translated (it extends `LootModifier` (vectorwing.farmersdelight.refabricated)) (vectorwing/farmersdelight/common/loot/modifier/ReplaceItemModifier.java:23)
- [ ] `CampfireBaleMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (vectorwing/farmersdelight/common/loot/modifier/ReplaceItemModifier.java:12)
- [ ] `CuttingBoardDispenserMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (vectorwing/farmersdelight/common/loot/modifier/ReplaceItemModifier.java:24)
- [ ] `ItemStackComponentizationFixMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (vectorwing/farmersdelight/common/loot/modifier/ReplaceItemModifier.java:14)
- [ ] `V1460Mixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (vectorwing/farmersdelight/common/loot/modifier/ReplaceItemModifier.java:18)
- [ ] `V3818_3Mixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (vectorwing/farmersdelight/common/loot/modifier/ReplaceItemModifier.java:16)
- [ ] `HideBlockBreakProgressMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (vectorwing/farmersdelight/common/loot/modifier/ReplaceItemModifier.java:13)
- [ ] `NourishmentAlwaysEatMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (vectorwing/farmersdelight/common/loot/modifier/ReplaceItemModifier.java:11)
- [ ] `PlacePumpkinPieMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (vectorwing/farmersdelight/common/loot/modifier/ReplaceItemModifier.java:18)
- [ ] `AbstractRecipeBookScreenAccessor`: a Mixin class: its handlers are the injections, in the bundle's helpers (vectorwing/farmersdelight/common/loot/modifier/ReplaceItemModifier.java:9)
- [ ] `CuboidItemModelWrapperAccessor`: a Mixin class: its handlers are the injections, in the bundle's helpers (vectorwing/farmersdelight/common/loot/modifier/ReplaceItemModifier.java:10)

### `FarmersDelightMixinPlugin`: it implements `IMixinConfigPlugin` (org.spongepowered.asm.mixin.extensibility)

- [ ] `FarmersDelightMixinPlugin.onLoad(1)`: its class isn't translated (it implements `IMixinConfigPlugin` (org.spongepowered.asm.mixin.extensibility)) (vectorwing/farmersdelight/common/mixin/refabricated/FarmersDelightMixinPlugin.java:12)
- [ ] `FarmersDelightMixinPlugin.getRefMapperConfig(0)`: its class isn't translated (it implements `IMixinConfigPlugin` (org.spongepowered.asm.mixin.extensibility)) (vectorwing/farmersdelight/common/mixin/refabricated/FarmersDelightMixinPlugin.java:15)
- [ ] `FarmersDelightMixinPlugin.shouldApplyMixin(2)`: its class isn't translated (it implements `IMixinConfigPlugin` (org.spongepowered.asm.mixin.extensibility)) (vectorwing/farmersdelight/common/mixin/refabricated/FarmersDelightMixinPlugin.java:19)
- [ ] `FarmersDelightMixinPlugin.acceptTargets(2)`: its class isn't translated (it implements `IMixinConfigPlugin` (org.spongepowered.asm.mixin.extensibility)) (vectorwing/farmersdelight/common/mixin/refabricated/FarmersDelightMixinPlugin.java:23)
- [ ] `FarmersDelightMixinPlugin.getMixins(0)`: its class isn't translated (it implements `IMixinConfigPlugin` (org.spongepowered.asm.mixin.extensibility)) (vectorwing/farmersdelight/common/mixin/refabricated/FarmersDelightMixinPlugin.java:26)
- [ ] `FarmersDelightMixinPlugin.preApply(4)`: its class isn't translated (it implements `IMixinConfigPlugin` (org.spongepowered.asm.mixin.extensibility)) (vectorwing/farmersdelight/common/mixin/refabricated/FarmersDelightMixinPlugin.java:36)
- [ ] `FarmersDelightMixinPlugin.postApply(4)`: its class isn't translated (it implements `IMixinConfigPlugin` (org.spongepowered.asm.mixin.extensibility)) (vectorwing/farmersdelight/common/mixin/refabricated/FarmersDelightMixinPlugin.java:39)
- [ ] `FenceGateBlockMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (vectorwing/farmersdelight/common/mixin/refabricated/FarmersDelightMixinPlugin.java:12)
- [ ] `FoodDataAccessor`: a Mixin class: its handlers are the injections, in the bundle's helpers (vectorwing/farmersdelight/common/mixin/refabricated/FarmersDelightMixinPlugin.java:8)
- [ ] `GameRulesTypeAccessor`: a Mixin class: its handlers are the injections, in the bundle's helpers (vectorwing/farmersdelight/common/mixin/refabricated/FarmersDelightMixinPlugin.java:7)
- [ ] `GhostSlotsInvoker`: a Mixin class: its handlers are the injections, in the bundle's helpers (vectorwing/farmersdelight/common/mixin/refabricated/FarmersDelightMixinPlugin.java:11)
- [ ] `GuiGraphicsExtractorAccessor`: a Mixin class: its handlers are the injections, in the bundle's helpers (vectorwing/farmersdelight/common/mixin/refabricated/FarmersDelightMixinPlugin.java:10)
- [ ] `HangingSignEditScreenMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (vectorwing/farmersdelight/common/mixin/refabricated/FarmersDelightMixinPlugin.java:18)
- [ ] `HudMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (vectorwing/farmersdelight/common/mixin/refabricated/FarmersDelightMixinPlugin.java:15)
- [ ] `HumanoidMobRendererMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (vectorwing/farmersdelight/common/mixin/refabricated/FarmersDelightMixinPlugin.java:15)
- [ ] `HumanoidModelMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (vectorwing/farmersdelight/common/mixin/refabricated/FarmersDelightMixinPlugin.java:14)
- [ ] `HumanoidRenderStateMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (vectorwing/farmersdelight/common/mixin/refabricated/FarmersDelightMixinPlugin.java:9)
- [ ] `ItemStackMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (vectorwing/farmersdelight/common/mixin/refabricated/FarmersDelightMixinPlugin.java:13)
- [ ] `LaunchpadFenceGateBlockMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (vectorwing/farmersdelight/common/mixin/refabricated/FarmersDelightMixinPlugin.java:13)
- [ ] `LevelMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (vectorwing/farmersdelight/common/mixin/refabricated/FarmersDelightMixinPlugin.java:14)
- [ ] `LivingEntityMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (vectorwing/farmersdelight/common/mixin/refabricated/FarmersDelightMixinPlugin.java:17)
- [ ] `MinecraftServerMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (vectorwing/farmersdelight/common/mixin/refabricated/FarmersDelightMixinPlugin.java:16)
- [ ] `PlayerMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (vectorwing/farmersdelight/common/mixin/refabricated/FarmersDelightMixinPlugin.java:19)
- [ ] `RecipeBookSettingsMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (vectorwing/farmersdelight/common/mixin/refabricated/FarmersDelightMixinPlugin.java:21)
- [ ] `RecipeBookSettingsTypeSettingsInvoker`: a Mixin class: its handlers are the injections, in the bundle's helpers (vectorwing/farmersdelight/common/mixin/refabricated/FarmersDelightMixinPlugin.java:9)
- [ ] `RecipeBookTypeMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (vectorwing/farmersdelight/common/mixin/refabricated/FarmersDelightMixinPlugin.java:7)
- [ ] `RecipeManagerMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (vectorwing/farmersdelight/common/mixin/refabricated/FarmersDelightMixinPlugin.java:26)
- [ ] `SearchRecipeBookCategoryMixin`: its constants carry values (vectorwing/farmersdelight/common/mixin/refabricated/FarmersDelightMixinPlugin.java:10)
- [ ] `ServerRecipeBookMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (vectorwing/farmersdelight/common/mixin/refabricated/FarmersDelightMixinPlugin.java:15)
- [ ] `SignBlockEntityMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (vectorwing/farmersdelight/common/mixin/refabricated/FarmersDelightMixinPlugin.java:16)
- [ ] `SignEditScreenMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (vectorwing/farmersdelight/common/mixin/refabricated/FarmersDelightMixinPlugin.java:18)
- [ ] `RopeFenceConnectionMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (vectorwing/farmersdelight/common/mixin/refabricated/FarmersDelightMixinPlugin.java:13)
- [ ] `VillagersTargetRichSoilMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (vectorwing/farmersdelight/common/mixin/refabricated/FarmersDelightMixinPlugin.java:16)

### `ModNetworking`

- [ ] `ModNetworking.init(0)`: it calls Fabric's `PayloadTypeRegistry`.clientboundPlay (vectorwing/farmersdelight/common/network/ModNetworking.java:14)
- [ ] `ModNetworking.initClient(0)`: it uses SendRecipeBookValuesPayload.TYPE, which isn't translated (it uses Minecraft's `Type`) (vectorwing/farmersdelight/common/network/ModNetworking.java:22)

### `ModNetworking.ClientPayloadHandler`

- [ ] `ModNetworking.ClientPayloadHandler.handleRichSoilBoostParticles(2)`: it uses RichSoilBoostParticlesPayload, which isn't translated (it implements Minecraft's `CustomPacketPayload`) (vectorwing/farmersdelight/common/network/ModNetworking.java:29)

### `FlipSkilletPayload`: it implements Minecraft's `CustomPacketPayload`

- [x] `FlipSkilletPayload.ID` (vectorwing/farmersdelight/common/network/payload/FlipSkilletPayload.java:16)
- [ ] `FlipSkilletPayload.INSTANCE`: it uses FlipSkilletPayload, which isn't translated (it implements Minecraft's `CustomPacketPayload`) (vectorwing/farmersdelight/common/network/payload/FlipSkilletPayload.java:17)
- [ ] `FlipSkilletPayload.TYPE`: it uses Minecraft's `Type` (vectorwing/farmersdelight/common/network/payload/FlipSkilletPayload.java:18)
- [ ] `FlipSkilletPayload.STREAM_CODEC`: it uses Minecraft's `StreamCodec` (vectorwing/farmersdelight/common/network/payload/FlipSkilletPayload.java:19)
- [ ] `FlipSkilletPayload.type(0)`: its class isn't translated (it implements Minecraft's `CustomPacketPayload`) (vectorwing/farmersdelight/common/network/payload/FlipSkilletPayload.java:21)
- [ ] `FlipSkilletPayload.handle(2)`: its class isn't translated (it implements Minecraft's `CustomPacketPayload`) (vectorwing/farmersdelight/common/network/payload/FlipSkilletPayload.java:25)

### `RichSoilBoostParticlesPayload`: it implements Minecraft's `CustomPacketPayload`

- [x] `RichSoilBoostParticlesPayload.ID` (vectorwing/farmersdelight/common/network/payload/RichSoilBoostParticlesPayload.java:12)
- [ ] `RichSoilBoostParticlesPayload.TYPE`: it uses Minecraft's `Type` (vectorwing/farmersdelight/common/network/payload/RichSoilBoostParticlesPayload.java:13)
- [ ] `RichSoilBoostParticlesPayload.STREAM_CODEC`: it uses Minecraft's `StreamCodec` (vectorwing/farmersdelight/common/network/payload/RichSoilBoostParticlesPayload.java:14)
- [ ] `RichSoilBoostParticlesPayload.RichSoilBoostParticlesPayload(1)`: its class isn't translated (it implements Minecraft's `CustomPacketPayload`) (vectorwing/farmersdelight/common/network/payload/RichSoilBoostParticlesPayload.java:11)
- [ ] `RichSoilBoostParticlesPayload.type(0)`: its class isn't translated (it implements Minecraft's `CustomPacketPayload`) (vectorwing/farmersdelight/common/network/payload/RichSoilBoostParticlesPayload.java:18)
- [ ] `RichSoilBoostParticlesPayload.pos(0)`: its class isn't translated (it implements Minecraft's `CustomPacketPayload`) (vectorwing/farmersdelight/common/network/payload/RichSoilBoostParticlesPayload.java:11)

### `SendNaturalRegenerationValuePayload`: it implements Minecraft's `CustomPacketPayload`

- [x] `SendNaturalRegenerationValuePayload.ID` (vectorwing/farmersdelight/common/network/payload/SendNaturalRegenerationValuePayload.java:14)
- [ ] `SendNaturalRegenerationValuePayload.TYPE`: it uses Minecraft's `Type` (vectorwing/farmersdelight/common/network/payload/SendNaturalRegenerationValuePayload.java:15)
- [ ] `SendNaturalRegenerationValuePayload.STREAM_CODEC`: it uses Minecraft's `StreamCodec` (vectorwing/farmersdelight/common/network/payload/SendNaturalRegenerationValuePayload.java:16)
- [ ] `SendNaturalRegenerationValuePayload.SendNaturalRegenerationValuePayload(1)`: its class isn't translated (it implements Minecraft's `CustomPacketPayload`) (vectorwing/farmersdelight/common/network/payload/SendNaturalRegenerationValuePayload.java:20)
- [ ] `SendNaturalRegenerationValuePayload.write(2)`: it uses Minecraft's `RegistryFriendlyByteBuf` (vectorwing/farmersdelight/common/network/payload/SendNaturalRegenerationValuePayload.java:24)
- [ ] `SendNaturalRegenerationValuePayload.type(0)`: its class isn't translated (it implements Minecraft's `CustomPacketPayload`) (vectorwing/farmersdelight/common/network/payload/SendNaturalRegenerationValuePayload.java:28)
- [ ] `SendNaturalRegenerationValuePayload.handle(0)`: its class isn't translated (it implements Minecraft's `CustomPacketPayload`) (vectorwing/farmersdelight/common/network/payload/SendNaturalRegenerationValuePayload.java:32)
- [ ] `SendNaturalRegenerationValuePayload.value(0)`: its class isn't translated (it implements Minecraft's `CustomPacketPayload`) (vectorwing/farmersdelight/common/network/payload/SendNaturalRegenerationValuePayload.java:13)

### `SendRecipeBookValuesPayload`: it implements Minecraft's `CustomPacketPayload`

- [x] `SendRecipeBookValuesPayload.ID` (vectorwing/farmersdelight/common/network/payload/SendRecipeBookValuesPayload.java:15)
- [ ] `SendRecipeBookValuesPayload.TYPE`: it uses Minecraft's `Type` (vectorwing/farmersdelight/common/network/payload/SendRecipeBookValuesPayload.java:16)
- [ ] `SendRecipeBookValuesPayload.STREAM_CODEC`: it uses Minecraft's `StreamCodec` (vectorwing/farmersdelight/common/network/payload/SendRecipeBookValuesPayload.java:17)
- [ ] `SendRecipeBookValuesPayload.SendRecipeBookValuesPayload(2)`: its class isn't translated (it implements Minecraft's `CustomPacketPayload`) (vectorwing/farmersdelight/common/network/payload/SendRecipeBookValuesPayload.java:14)
- [ ] `SendRecipeBookValuesPayload.SendRecipeBookValuesPayload(1)`: its class isn't translated (it implements Minecraft's `CustomPacketPayload`) (vectorwing/farmersdelight/common/network/payload/SendRecipeBookValuesPayload.java:21)
- [ ] `SendRecipeBookValuesPayload.write(2)`: it uses Minecraft's `RegistryFriendlyByteBuf` (vectorwing/farmersdelight/common/network/payload/SendRecipeBookValuesPayload.java:25)
- [ ] `SendRecipeBookValuesPayload.type(0)`: its class isn't translated (it implements Minecraft's `CustomPacketPayload`) (vectorwing/farmersdelight/common/network/payload/SendRecipeBookValuesPayload.java:30)
- [ ] `SendRecipeBookValuesPayload.handle(0)`: its class isn't translated (it implements Minecraft's `CustomPacketPayload`) (vectorwing/farmersdelight/common/network/payload/SendRecipeBookValuesPayload.java:34)
- [ ] `SendRecipeBookValuesPayload.open(0)`: its class isn't translated (it implements Minecraft's `CustomPacketPayload`) (vectorwing/farmersdelight/common/network/payload/SendRecipeBookValuesPayload.java:14)
- [ ] `SendRecipeBookValuesPayload.filtering(0)`: its class isn't translated (it implements Minecraft's `CustomPacketPayload`) (vectorwing/farmersdelight/common/network/payload/SendRecipeBookValuesPayload.java:14)

### `ModBlockIds`

- [ ] `ModBlockIds.CANVAS_WALL_SIGN`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/common/references/ModBlockIds.java:10)
- [ ] `ModBlockIds.DYED_CANVAS_WALL_SIGN`: it uses Minecraft's `ColorCollection` (vectorwing/farmersdelight/common/references/ModBlockIds.java:11)
- [ ] `ModBlockIds.HANGING_CANVAS_WALL_SIGN`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/common/references/ModBlockIds.java:12)
- [ ] `ModBlockIds.DYED_HANGING_CANVAS_WALL_SIGN`: it uses Minecraft's `ColorCollection` (vectorwing/farmersdelight/common/references/ModBlockIds.java:13)
- [ ] `ModBlockIds.PUMPKIN_PIE`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/common/references/ModBlockIds.java:14)
- [ ] `ModBlockIds.BUDDING_TOMATO_CROP`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/common/references/ModBlockIds.java:15)
- [ ] `ModBlockIds.TOMATO_CROP_ON_ROPE`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/common/references/ModBlockIds.java:16)
- [ ] `ModBlockIds.createSimpleColored(1)`: it uses Minecraft's `ColorCollection` (vectorwing/farmersdelight/common/references/ModBlockIds.java:18)
- [ ] `ModBlockIds.key(1)`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/common/references/ModBlockIds.java:22)

### `ModBlockItemIds`

- [ ] `ModBlockItemIds.STOVE`: it uses Minecraft's `BlockItemId` (vectorwing/farmersdelight/common/references/ModBlockItemIds.java:9)
- [ ] `ModBlockItemIds.COOKING_POT`: it uses Minecraft's `BlockItemId` (vectorwing/farmersdelight/common/references/ModBlockItemIds.java:10)
- [ ] `ModBlockItemIds.SKILLET`: it uses Minecraft's `BlockItemId` (vectorwing/farmersdelight/common/references/ModBlockItemIds.java:11)
- [ ] `ModBlockItemIds.WOODEN_BASKET`: it uses Minecraft's `BlockItemId` (vectorwing/farmersdelight/common/references/ModBlockItemIds.java:12)
- [ ] `ModBlockItemIds.BAMBOO_BASKET`: it uses Minecraft's `BlockItemId` (vectorwing/farmersdelight/common/references/ModBlockItemIds.java:13)
- [ ] `ModBlockItemIds.CUTTING_BOARD`: it uses Minecraft's `BlockItemId` (vectorwing/farmersdelight/common/references/ModBlockItemIds.java:14)
- [ ] `ModBlockItemIds.CARROT_CRATE`: it uses Minecraft's `BlockItemId` (vectorwing/farmersdelight/common/references/ModBlockItemIds.java:15)
- [ ] `ModBlockItemIds.POTATO_CRATE`: it uses Minecraft's `BlockItemId` (vectorwing/farmersdelight/common/references/ModBlockItemIds.java:16)
- [ ] `ModBlockItemIds.BEETROOT_CRATE`: it uses Minecraft's `BlockItemId` (vectorwing/farmersdelight/common/references/ModBlockItemIds.java:17)
- [ ] `ModBlockItemIds.CABBAGE_CRATE`: it uses Minecraft's `BlockItemId` (vectorwing/farmersdelight/common/references/ModBlockItemIds.java:18)
- [ ] `ModBlockItemIds.TOMATO_CRATE`: it uses Minecraft's `BlockItemId` (vectorwing/farmersdelight/common/references/ModBlockItemIds.java:19)
- [ ] `ModBlockItemIds.ONION_CRATE`: it uses Minecraft's `BlockItemId` (vectorwing/farmersdelight/common/references/ModBlockItemIds.java:20)
- [ ] `ModBlockItemIds.RICE_BALE`: it uses Minecraft's `BlockItemId` (vectorwing/farmersdelight/common/references/ModBlockItemIds.java:21)
- [ ] `ModBlockItemIds.RICE_BAG`: it uses Minecraft's `BlockItemId` (vectorwing/farmersdelight/common/references/ModBlockItemIds.java:22)
- [ ] `ModBlockItemIds.STRAW_BALE`: it uses Minecraft's `BlockItemId` (vectorwing/farmersdelight/common/references/ModBlockItemIds.java:23)
- [ ] `ModBlockItemIds.ROPE`: it uses Minecraft's `BlockItemId` (vectorwing/farmersdelight/common/references/ModBlockItemIds.java:24)
- [ ] `ModBlockItemIds.SAFETY_NET`: it uses Minecraft's `BlockItemId` (vectorwing/farmersdelight/common/references/ModBlockItemIds.java:25)
- [ ] `ModBlockItemIds.ROPE_FENCE`: it uses Minecraft's `BlockItemId` (vectorwing/farmersdelight/common/references/ModBlockItemIds.java:26)
- [ ] `ModBlockItemIds.ROPE_FENCE_GATE`: it uses Minecraft's `BlockItemId` (vectorwing/farmersdelight/common/references/ModBlockItemIds.java:27)
- [ ] `ModBlockItemIds.OAK_CABINET`: it uses Minecraft's `BlockItemId` (vectorwing/farmersdelight/common/references/ModBlockItemIds.java:28)
- [ ] `ModBlockItemIds.SPRUCE_CABINET`: it uses Minecraft's `BlockItemId` (vectorwing/farmersdelight/common/references/ModBlockItemIds.java:29)
- [ ] `ModBlockItemIds.BIRCH_CABINET`: it uses Minecraft's `BlockItemId` (vectorwing/farmersdelight/common/references/ModBlockItemIds.java:30)
- [ ] `ModBlockItemIds.JUNGLE_CABINET`: it uses Minecraft's `BlockItemId` (vectorwing/farmersdelight/common/references/ModBlockItemIds.java:31)
- [ ] `ModBlockItemIds.ACACIA_CABINET`: it uses Minecraft's `BlockItemId` (vectorwing/farmersdelight/common/references/ModBlockItemIds.java:32)
- [ ] `ModBlockItemIds.DARK_OAK_CABINET`: it uses Minecraft's `BlockItemId` (vectorwing/farmersdelight/common/references/ModBlockItemIds.java:33)
- [ ] `ModBlockItemIds.MANGROVE_CABINET`: it uses Minecraft's `BlockItemId` (vectorwing/farmersdelight/common/references/ModBlockItemIds.java:34)
- [ ] `ModBlockItemIds.CHERRY_CABINET`: it uses Minecraft's `BlockItemId` (vectorwing/farmersdelight/common/references/ModBlockItemIds.java:35)
- [ ] `ModBlockItemIds.BAMBOO_CABINET`: it uses Minecraft's `BlockItemId` (vectorwing/farmersdelight/common/references/ModBlockItemIds.java:36)
- [ ] `ModBlockItemIds.PALE_OAK_CABINET`: it uses Minecraft's `BlockItemId` (vectorwing/farmersdelight/common/references/ModBlockItemIds.java:37)
- [ ] `ModBlockItemIds.POPLAR_CABINET`: it uses Minecraft's `BlockItemId` (vectorwing/farmersdelight/common/references/ModBlockItemIds.java:38)
- [ ] `ModBlockItemIds.CRIMSON_CABINET`: it uses Minecraft's `BlockItemId` (vectorwing/farmersdelight/common/references/ModBlockItemIds.java:39)
- [ ] `ModBlockItemIds.WARPED_CABINET`: it uses Minecraft's `BlockItemId` (vectorwing/farmersdelight/common/references/ModBlockItemIds.java:40)
- [ ] `ModBlockItemIds.CANVAS_RUG`: it uses Minecraft's `BlockItemId` (vectorwing/farmersdelight/common/references/ModBlockItemIds.java:41)
- [ ] `ModBlockItemIds.TATAMI`: it uses Minecraft's `BlockItemId` (vectorwing/farmersdelight/common/references/ModBlockItemIds.java:42)
- [ ] `ModBlockItemIds.FULL_TATAMI_MAT`: it uses Minecraft's `BlockItemId` (vectorwing/farmersdelight/common/references/ModBlockItemIds.java:43)
- [ ] `ModBlockItemIds.HALF_TATAMI_MAT`: it uses Minecraft's `BlockItemId` (vectorwing/farmersdelight/common/references/ModBlockItemIds.java:44)
- [ ] `ModBlockItemIds.CANVAS_SIGN`: it uses Minecraft's `BlockItemId` (vectorwing/farmersdelight/common/references/ModBlockItemIds.java:45)
- [ ] `ModBlockItemIds.DYED_CANVAS_SIGN`: it uses Minecraft's `ColorCollection` (vectorwing/farmersdelight/common/references/ModBlockItemIds.java:46)
- [ ] `ModBlockItemIds.HANGING_CANVAS_SIGN`: it uses Minecraft's `BlockItemId` (vectorwing/farmersdelight/common/references/ModBlockItemIds.java:47)
- [ ] `ModBlockItemIds.DYED_HANGING_CANVAS_SIGN`: it uses Minecraft's `ColorCollection` (vectorwing/farmersdelight/common/references/ModBlockItemIds.java:48)
- [ ] `ModBlockItemIds.BROWN_MUSHROOM_COLONY`: it uses Minecraft's `BlockItemId` (vectorwing/farmersdelight/common/references/ModBlockItemIds.java:49)
- [ ] `ModBlockItemIds.RED_MUSHROOM_COLONY`: it uses Minecraft's `BlockItemId` (vectorwing/farmersdelight/common/references/ModBlockItemIds.java:50)
- [ ] `ModBlockItemIds.ORGANIC_COMPOST`: it uses Minecraft's `BlockItemId` (vectorwing/farmersdelight/common/references/ModBlockItemIds.java:51)
- [ ] `ModBlockItemIds.RICH_SOIL`: it uses Minecraft's `BlockItemId` (vectorwing/farmersdelight/common/references/ModBlockItemIds.java:52)
- [ ] `ModBlockItemIds.RICH_SOIL_FARMLAND`: it uses Minecraft's `BlockItemId` (vectorwing/farmersdelight/common/references/ModBlockItemIds.java:53)
- [ ] `ModBlockItemIds.APPLE_PIE`: it uses Minecraft's `BlockItemId` (vectorwing/farmersdelight/common/references/ModBlockItemIds.java:54)
- [ ] `ModBlockItemIds.SWEET_BERRY_CHEESECAKE`: it uses Minecraft's `BlockItemId` (vectorwing/farmersdelight/common/references/ModBlockItemIds.java:55)
- [ ] `ModBlockItemIds.CHOCOLATE_PIE`: it uses Minecraft's `BlockItemId` (vectorwing/farmersdelight/common/references/ModBlockItemIds.java:56)
- [ ] `ModBlockItemIds.SANDY_SHRUB`: it uses Minecraft's `BlockItemId` (vectorwing/farmersdelight/common/references/ModBlockItemIds.java:57)
- [ ] `ModBlockItemIds.WILD_CABBAGES`: it uses Minecraft's `BlockItemId` (vectorwing/farmersdelight/common/references/ModBlockItemIds.java:58)
- [ ] `ModBlockItemIds.WILD_ONIONS`: it uses Minecraft's `BlockItemId` (vectorwing/farmersdelight/common/references/ModBlockItemIds.java:59)
- [ ] `ModBlockItemIds.WILD_TOMATOES`: it uses Minecraft's `BlockItemId` (vectorwing/farmersdelight/common/references/ModBlockItemIds.java:60)
- [ ] `ModBlockItemIds.WILD_CARROTS`: it uses Minecraft's `BlockItemId` (vectorwing/farmersdelight/common/references/ModBlockItemIds.java:61)
- [ ] `ModBlockItemIds.WILD_POTATOES`: it uses Minecraft's `BlockItemId` (vectorwing/farmersdelight/common/references/ModBlockItemIds.java:62)
- [ ] `ModBlockItemIds.WILD_BEETROOTS`: it uses Minecraft's `BlockItemId` (vectorwing/farmersdelight/common/references/ModBlockItemIds.java:63)
- [ ] `ModBlockItemIds.WILD_RICE`: it uses Minecraft's `BlockItemId` (vectorwing/farmersdelight/common/references/ModBlockItemIds.java:64)
- [ ] `ModBlockItemIds.CABBAGE_CROP`: it uses Minecraft's `BlockItemId` (vectorwing/farmersdelight/common/references/ModBlockItemIds.java:65)
- [ ] `ModBlockItemIds.ONION_CROP`: it uses Minecraft's `BlockItemId` (vectorwing/farmersdelight/common/references/ModBlockItemIds.java:66)
- [ ] `ModBlockItemIds.TOMATO_CROP`: it uses Minecraft's `BlockItemId` (vectorwing/farmersdelight/common/references/ModBlockItemIds.java:67)
- [ ] `ModBlockItemIds.RICE_CROP`: it uses Minecraft's `BlockItemId` (vectorwing/farmersdelight/common/references/ModBlockItemIds.java:68)
- [ ] `ModBlockItemIds.RICE_CROP_PANICLES`: it uses Minecraft's `BlockItemId` (vectorwing/farmersdelight/common/references/ModBlockItemIds.java:69)
- [ ] `ModBlockItemIds.ROAST_CHICKEN_BLOCK`: it uses Minecraft's `BlockItemId` (vectorwing/farmersdelight/common/references/ModBlockItemIds.java:70)
- [ ] `ModBlockItemIds.STUFFED_PUMPKIN_BLOCK`: it uses Minecraft's `BlockItemId` (vectorwing/farmersdelight/common/references/ModBlockItemIds.java:71)
- [ ] `ModBlockItemIds.HONEY_GLAZED_HAM_BLOCK`: it uses Minecraft's `BlockItemId` (vectorwing/farmersdelight/common/references/ModBlockItemIds.java:72)
- [ ] `ModBlockItemIds.SHEPHERDS_PIE_BLOCK`: it uses Minecraft's `BlockItemId` (vectorwing/farmersdelight/common/references/ModBlockItemIds.java:73)
- [ ] `ModBlockItemIds.GLEAMING_SALAD_BLOCK`: it uses Minecraft's `BlockItemId` (vectorwing/farmersdelight/common/references/ModBlockItemIds.java:74)
- [ ] `ModBlockItemIds.RICE_ROLL_MEDLEY_BLOCK`: it uses Minecraft's `BlockItemId` (vectorwing/farmersdelight/common/references/ModBlockItemIds.java:75)
- [ ] `ModBlockItemIds.STRAW_BED`: it uses Minecraft's `BlockItemId` (vectorwing/farmersdelight/common/references/ModBlockItemIds.java:76)
- [ ] `ModBlockItemIds.createSimpleColored(1)`: it uses Minecraft's `ColorCollection` (vectorwing/farmersdelight/common/references/ModBlockItemIds.java:78)
- [ ] `ModBlockItemIds.id(2)`: it uses Minecraft's `BlockItemId` (vectorwing/farmersdelight/common/references/ModBlockItemIds.java:82)
- [ ] `ModBlockItemIds.id(1)`: it uses Minecraft's `BlockItemId` (vectorwing/farmersdelight/common/references/ModBlockItemIds.java:88)

### `ModEntityTypeIds`

- [ ] `ModEntityTypeIds.ROTTEN_TOMATO`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/common/references/ModEntityTypeIds.java:9)
- [ ] `ModEntityTypeIds.key(1)`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/common/references/ModEntityTypeIds.java:11)

### `ModItemIds`

- [ ] `ModItemIds.FLINT_KNIFE`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/common/references/ModItemIds.java:9)
- [ ] `ModItemIds.COPPER_KNIFE`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/common/references/ModItemIds.java:10)
- [ ] `ModItemIds.IRON_KNIFE`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/common/references/ModItemIds.java:11)
- [ ] `ModItemIds.DIAMOND_KNIFE`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/common/references/ModItemIds.java:12)
- [ ] `ModItemIds.NETHERITE_KNIFE`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/common/references/ModItemIds.java:13)
- [ ] `ModItemIds.GOLDEN_KNIFE`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/common/references/ModItemIds.java:14)
- [ ] `ModItemIds.STRAW`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/common/references/ModItemIds.java:15)
- [ ] `ModItemIds.CANVAS`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/common/references/ModItemIds.java:16)
- [ ] `ModItemIds.TREE_BARK`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/common/references/ModItemIds.java:17)
- [ ] `ModItemIds.CABBAGE`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/common/references/ModItemIds.java:18)
- [ ] `ModItemIds.TOMATO`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/common/references/ModItemIds.java:19)
- [ ] `ModItemIds.ROTTEN_TOMATO`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/common/references/ModItemIds.java:20)
- [ ] `ModItemIds.FRIED_EGG`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/common/references/ModItemIds.java:21)
- [ ] `ModItemIds.MILK_BOTTLE`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/common/references/ModItemIds.java:22)
- [ ] `ModItemIds.HOT_COCOA`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/common/references/ModItemIds.java:23)
- [ ] `ModItemIds.APPLE_CIDER`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/common/references/ModItemIds.java:24)
- [ ] `ModItemIds.MELON_JUICE`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/common/references/ModItemIds.java:25)
- [ ] `ModItemIds.TOMATO_SAUCE`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/common/references/ModItemIds.java:26)
- [ ] `ModItemIds.WHEAT_DOUGH`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/common/references/ModItemIds.java:27)
- [ ] `ModItemIds.RAW_PASTA`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/common/references/ModItemIds.java:28)
- [ ] `ModItemIds.PUMPKIN_SLICE`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/common/references/ModItemIds.java:29)
- [ ] `ModItemIds.CABBAGE_LEAF`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/common/references/ModItemIds.java:30)
- [ ] `ModItemIds.MINCED_BEEF`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/common/references/ModItemIds.java:31)
- [ ] `ModItemIds.BEEF_PATTY`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/common/references/ModItemIds.java:32)
- [ ] `ModItemIds.CHICKEN_CUTS`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/common/references/ModItemIds.java:33)
- [ ] `ModItemIds.COOKED_CHICKEN_CUTS`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/common/references/ModItemIds.java:34)
- [ ] `ModItemIds.BACON`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/common/references/ModItemIds.java:35)
- [ ] `ModItemIds.COOKED_BACON`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/common/references/ModItemIds.java:36)
- [ ] `ModItemIds.COD_SLICE`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/common/references/ModItemIds.java:37)
- [ ] `ModItemIds.COOKED_COD_SLICE`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/common/references/ModItemIds.java:38)
- [ ] `ModItemIds.SALMON_SLICE`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/common/references/ModItemIds.java:39)
- [ ] `ModItemIds.COOKED_SALMON_SLICE`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/common/references/ModItemIds.java:40)
- [ ] `ModItemIds.MUTTON_CHOPS`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/common/references/ModItemIds.java:41)
- [ ] `ModItemIds.COOKED_MUTTON_CHOPS`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/common/references/ModItemIds.java:42)
- [ ] `ModItemIds.HAM`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/common/references/ModItemIds.java:43)
- [ ] `ModItemIds.SMOKED_HAM`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/common/references/ModItemIds.java:44)
- [ ] `ModItemIds.PIE_CRUST`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/common/references/ModItemIds.java:45)
- [ ] `ModItemIds.CAKE_SLICE`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/common/references/ModItemIds.java:46)
- [ ] `ModItemIds.APPLE_PIE_SLICE`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/common/references/ModItemIds.java:47)
- [ ] `ModItemIds.SWEET_BERRY_CHEESECAKE_SLICE`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/common/references/ModItemIds.java:48)
- [ ] `ModItemIds.CHOCOLATE_PIE_SLICE`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/common/references/ModItemIds.java:49)
- [ ] `ModItemIds.PUMPKIN_PIE_SLICE`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/common/references/ModItemIds.java:50)
- [ ] `ModItemIds.SWEET_BERRY_COOKIE`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/common/references/ModItemIds.java:51)
- [ ] `ModItemIds.HONEY_COOKIE`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/common/references/ModItemIds.java:52)
- [ ] `ModItemIds.MELON_POPSICLE`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/common/references/ModItemIds.java:53)
- [ ] `ModItemIds.GLOW_BERRY_CUSTARD`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/common/references/ModItemIds.java:54)
- [ ] `ModItemIds.FRUIT_SALAD`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/common/references/ModItemIds.java:55)
- [ ] `ModItemIds.MIXED_SALAD`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/common/references/ModItemIds.java:56)
- [ ] `ModItemIds.NETHER_SALAD`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/common/references/ModItemIds.java:57)
- [ ] `ModItemIds.BARBECUE_STICK`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/common/references/ModItemIds.java:58)
- [ ] `ModItemIds.EGG_SANDWICH`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/common/references/ModItemIds.java:59)
- [ ] `ModItemIds.CHICKEN_SANDWICH`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/common/references/ModItemIds.java:60)
- [ ] `ModItemIds.HAMBURGER`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/common/references/ModItemIds.java:61)
- [ ] `ModItemIds.BACON_SANDWICH`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/common/references/ModItemIds.java:62)
- [ ] `ModItemIds.MUTTON_WRAP`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/common/references/ModItemIds.java:63)
- [ ] `ModItemIds.DUMPLINGS`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/common/references/ModItemIds.java:64)
- [ ] `ModItemIds.STUFFED_POTATO`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/common/references/ModItemIds.java:65)
- [ ] `ModItemIds.CABBAGE_ROLLS`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/common/references/ModItemIds.java:66)
- [ ] `ModItemIds.SALMON_ROLL`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/common/references/ModItemIds.java:67)
- [ ] `ModItemIds.COD_ROLL`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/common/references/ModItemIds.java:68)
- [ ] `ModItemIds.KELP_ROLL`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/common/references/ModItemIds.java:69)
- [ ] `ModItemIds.KELP_ROLL_SLICE`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/common/references/ModItemIds.java:70)
- [ ] `ModItemIds.COOKED_RICE`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/common/references/ModItemIds.java:71)
- [ ] `ModItemIds.BONE_BROTH`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/common/references/ModItemIds.java:72)
- [ ] `ModItemIds.BEEF_STEW`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/common/references/ModItemIds.java:73)
- [ ] `ModItemIds.CHICKEN_SOUP`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/common/references/ModItemIds.java:74)
- [ ] `ModItemIds.VEGETABLE_SOUP`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/common/references/ModItemIds.java:75)
- [ ] `ModItemIds.FISH_STEW`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/common/references/ModItemIds.java:76)
- [ ] `ModItemIds.FRIED_RICE`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/common/references/ModItemIds.java:77)
- [ ] `ModItemIds.PUMPKIN_SOUP`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/common/references/ModItemIds.java:78)
- [ ] `ModItemIds.BAKED_COD_STEW`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/common/references/ModItemIds.java:79)
- [ ] `ModItemIds.NOODLE_SOUP`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/common/references/ModItemIds.java:80)
- [ ] `ModItemIds.ONION_SOUP`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/common/references/ModItemIds.java:81)
- [ ] `ModItemIds.BACON_AND_EGGS`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/common/references/ModItemIds.java:82)
- [ ] `ModItemIds.PASTA_WITH_MEATBALLS`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/common/references/ModItemIds.java:83)
- [ ] `ModItemIds.PASTA_WITH_MUTTON_CHOP`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/common/references/ModItemIds.java:84)
- [ ] `ModItemIds.MUSHROOM_RICE`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/common/references/ModItemIds.java:85)
- [ ] `ModItemIds.ROASTED_MUTTON_CHOPS`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/common/references/ModItemIds.java:86)
- [ ] `ModItemIds.VEGETABLE_NOODLES`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/common/references/ModItemIds.java:87)
- [ ] `ModItemIds.STEAK_AND_POTATOES`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/common/references/ModItemIds.java:88)
- [ ] `ModItemIds.RATATOUILLE`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/common/references/ModItemIds.java:89)
- [ ] `ModItemIds.SQUID_INK_PASTA`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/common/references/ModItemIds.java:90)
- [ ] `ModItemIds.GRILLED_SALMON`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/common/references/ModItemIds.java:91)
- [ ] `ModItemIds.ROAST_CHICKEN`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/common/references/ModItemIds.java:92)
- [ ] `ModItemIds.STUFFED_PUMPKIN`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/common/references/ModItemIds.java:93)
- [ ] `ModItemIds.HONEY_GLAZED_HAM`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/common/references/ModItemIds.java:94)
- [ ] `ModItemIds.SHEPHERDS_PIE`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/common/references/ModItemIds.java:95)
- [ ] `ModItemIds.GLEAMING_SALAD`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/common/references/ModItemIds.java:96)
- [ ] `ModItemIds.DOG_FOOD`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/common/references/ModItemIds.java:97)
- [ ] `ModItemIds.HORSE_FEED`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/common/references/ModItemIds.java:98)
- [ ] `ModItemIds.DEBUG_PUMPKIN_PIE`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/common/references/ModItemIds.java:99)
- [ ] `ModItemIds.key(1)`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/common/references/ModItemIds.java:101)

### `ModAdvancements`

- [ ] `ModAdvancements.USE_CUTTING_BOARD`: it uses CuttingBoardTrigger, which isn't translated (it extends Minecraft's `SimpleCriterionTrigger`) (vectorwing/farmersdelight/common/registry/ModAdvancements.java:8)
- [x] `ModAdvancements.touch(0)` (vectorwing/farmersdelight/common/registry/ModAdvancements.java:10)

### `ModBiomeFeatures`

- [ ] `ModBiomeFeatures.IN_ORDER`: it uses Minecraft's `MapCodec` (vectorwing/farmersdelight/common/registry/ModBiomeFeatures.java:10)
- [ ] `ModBiomeFeatures.WILD_RICE`: it uses Minecraft's `MapCodec` (vectorwing/farmersdelight/common/registry/ModBiomeFeatures.java:11)
- [x] `ModBiomeFeatures.touch(0)` (vectorwing/farmersdelight/common/registry/ModBiomeFeatures.java:13)

### `ModBiomeModifiers`

- [ ] `ModBiomeModifiers.modFeature(1)`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/common/registry/ModBiomeModifiers.java:20)
- [ ] `ModBiomeModifiers.init(0)`: it creates a FDBiomeSelector, which isn't translated (it implements `Predicate` (java.util.function)) (vectorwing/farmersdelight/common/registry/ModBiomeModifiers.java:24)

### `ModBiomeModifiers.FDBiomeSelector`: it implements `Predicate` (java.util.function)

- [ ] `ModBiomeModifiers.FDBiomeSelector.FDBiomeSelector(4)`: its class isn't translated (it implements `Predicate` (java.util.function)) (vectorwing/farmersdelight/common/registry/ModBiomeModifiers.java:74)
- [ ] `ModBiomeModifiers.FDBiomeSelector.FDBiomeSelector(1)`: its class isn't translated (it implements `Predicate` (java.util.function)) (vectorwing/farmersdelight/common/registry/ModBiomeModifiers.java:76)
- [ ] `ModBiomeModifiers.FDBiomeSelector.test(1)`: its class isn't translated (it implements `Predicate` (java.util.function)) (vectorwing/farmersdelight/common/registry/ModBiomeModifiers.java:80)
- [ ] `ModBiomeModifiers.FDBiomeSelector.minTemperature(0)`: its class isn't translated (it implements `Predicate` (java.util.function)) (vectorwing/farmersdelight/common/registry/ModBiomeModifiers.java:74)
- [ ] `ModBiomeModifiers.FDBiomeSelector.maxTemperature(0)`: its class isn't translated (it implements `Predicate` (java.util.function)) (vectorwing/farmersdelight/common/registry/ModBiomeModifiers.java:74)
- [ ] `ModBiomeModifiers.FDBiomeSelector.allowed(0)`: its class isn't translated (it implements `Predicate` (java.util.function)) (vectorwing/farmersdelight/common/registry/ModBiomeModifiers.java:74)
- [ ] `ModBiomeModifiers.FDBiomeSelector.denied(0)`: its class isn't translated (it implements `Predicate` (java.util.function)) (vectorwing/farmersdelight/common/registry/ModBiomeModifiers.java:74)

### `ModBlockEntityTypes`

- [ ] `ModBlockEntityTypes.STOVE`: it uses Minecraft's `BlockEntityType` (vectorwing/farmersdelight/common/registry/ModBlockEntityTypes.java:17)
- [ ] `ModBlockEntityTypes.COOKING_POT`: it uses Minecraft's `BlockEntityType` (vectorwing/farmersdelight/common/registry/ModBlockEntityTypes.java:20)
- [ ] `ModBlockEntityTypes.BASKET`: it uses Minecraft's `BlockEntityType` (vectorwing/farmersdelight/common/registry/ModBlockEntityTypes.java:23)
- [ ] `ModBlockEntityTypes.CUTTING_BOARD`: it uses Minecraft's `BlockEntityType` (vectorwing/farmersdelight/common/registry/ModBlockEntityTypes.java:27)
- [ ] `ModBlockEntityTypes.SKILLET`: it uses Minecraft's `BlockEntityType` (vectorwing/farmersdelight/common/registry/ModBlockEntityTypes.java:30)
- [ ] `ModBlockEntityTypes.CABINET`: it uses Minecraft's `BlockEntityType` (vectorwing/farmersdelight/common/registry/ModBlockEntityTypes.java:33)
- [ ] `ModBlockEntityTypes.addSupportedBlocks(2)`: it uses Minecraft's `BlockEntityType` (vectorwing/farmersdelight/common/registry/ModBlockEntityTypes.java:56)
- [ ] `ModBlockEntityTypes.touch(0)`: it calls ModBlockEntityTypes.addSupportedBlocks, which isn't translated (vectorwing/farmersdelight/common/registry/ModBlockEntityTypes.java:63)

### `ModBlocks`

- [ ] `ModBlocks.STOVE`: it uses Minecraft's `Block` (vectorwing/farmersdelight/common/registry/ModBlocks.java:72)
- [ ] `ModBlocks.COOKING_POT`: it uses Minecraft's `Block` (vectorwing/farmersdelight/common/registry/ModBlocks.java:75)
- [ ] `ModBlocks.SKILLET`: it uses Minecraft's `Block` (vectorwing/farmersdelight/common/registry/ModBlocks.java:78)
- [ ] `ModBlocks.WOODEN_BASKET`: it uses Minecraft's `Block` (vectorwing/farmersdelight/common/registry/ModBlocks.java:81)
- [ ] `ModBlocks.BAMBOO_BASKET`: it uses Minecraft's `Block` (vectorwing/farmersdelight/common/registry/ModBlocks.java:84)
- [ ] `ModBlocks.CUTTING_BOARD`: it uses Minecraft's `Block` (vectorwing/farmersdelight/common/registry/ModBlocks.java:87)
- [ ] `ModBlocks.BASKET`: it uses Minecraft's `Block` (vectorwing/farmersdelight/common/registry/ModBlocks.java:91)
- [ ] `ModBlocks.CARROT_CRATE`: it uses Minecraft's `Block` (vectorwing/farmersdelight/common/registry/ModBlocks.java:92)
- [ ] `ModBlocks.POTATO_CRATE`: it uses Minecraft's `Block` (vectorwing/farmersdelight/common/registry/ModBlocks.java:95)
- [ ] `ModBlocks.BEETROOT_CRATE`: it uses Minecraft's `Block` (vectorwing/farmersdelight/common/registry/ModBlocks.java:98)
- [ ] `ModBlocks.CABBAGE_CRATE`: it uses Minecraft's `Block` (vectorwing/farmersdelight/common/registry/ModBlocks.java:101)
- [ ] `ModBlocks.TOMATO_CRATE`: it uses Minecraft's `Block` (vectorwing/farmersdelight/common/registry/ModBlocks.java:104)
- [ ] `ModBlocks.ONION_CRATE`: it uses Minecraft's `Block` (vectorwing/farmersdelight/common/registry/ModBlocks.java:107)
- [ ] `ModBlocks.RICE_BALE`: it uses Minecraft's `Block` (vectorwing/farmersdelight/common/registry/ModBlocks.java:110)
- [ ] `ModBlocks.RICE_BAG`: it uses Minecraft's `Block` (vectorwing/farmersdelight/common/registry/ModBlocks.java:111)
- [ ] `ModBlocks.STRAW_BALE`: it uses Minecraft's `Block` (vectorwing/farmersdelight/common/registry/ModBlocks.java:112)
- [ ] `ModBlocks.ROPE`: it uses Minecraft's `Block` (vectorwing/farmersdelight/common/registry/ModBlocks.java:113)
- [ ] `ModBlocks.SAFETY_NET`: it uses Minecraft's `Block` (vectorwing/farmersdelight/common/registry/ModBlocks.java:118)
- [ ] `ModBlocks.ROPE_FENCE`: it uses Minecraft's `Block` (vectorwing/farmersdelight/common/registry/ModBlocks.java:123)
- [ ] `ModBlocks.ROPE_FENCE_GATE`: it uses Minecraft's `Block` (vectorwing/farmersdelight/common/registry/ModBlocks.java:126)
- [ ] `ModBlocks.OAK_CABINET`: it uses Minecraft's `Block` (vectorwing/farmersdelight/common/registry/ModBlocks.java:129)
- [ ] `ModBlocks.SPRUCE_CABINET`: it uses Minecraft's `Block` (vectorwing/farmersdelight/common/registry/ModBlocks.java:130)
- [ ] `ModBlocks.BIRCH_CABINET`: it uses Minecraft's `Block` (vectorwing/farmersdelight/common/registry/ModBlocks.java:131)
- [ ] `ModBlocks.JUNGLE_CABINET`: it uses Minecraft's `Block` (vectorwing/farmersdelight/common/registry/ModBlocks.java:132)
- [ ] `ModBlocks.ACACIA_CABINET`: it uses Minecraft's `Block` (vectorwing/farmersdelight/common/registry/ModBlocks.java:133)
- [ ] `ModBlocks.DARK_OAK_CABINET`: it uses Minecraft's `Block` (vectorwing/farmersdelight/common/registry/ModBlocks.java:134)
- [ ] `ModBlocks.MANGROVE_CABINET`: it uses Minecraft's `Block` (vectorwing/farmersdelight/common/registry/ModBlocks.java:135)
- [ ] `ModBlocks.CHERRY_CABINET`: it uses Minecraft's `Block` (vectorwing/farmersdelight/common/registry/ModBlocks.java:136)
- [ ] `ModBlocks.BAMBOO_CABINET`: it uses Minecraft's `Block` (vectorwing/farmersdelight/common/registry/ModBlocks.java:139)
- [ ] `ModBlocks.PALE_OAK_CABINET`: it uses Minecraft's `Block` (vectorwing/farmersdelight/common/registry/ModBlocks.java:142)
- [ ] `ModBlocks.POPLAR_CABINET`: it uses Minecraft's `Block` (vectorwing/farmersdelight/common/registry/ModBlocks.java:143)
- [ ] `ModBlocks.CRIMSON_CABINET`: it uses Minecraft's `Block` (vectorwing/farmersdelight/common/registry/ModBlocks.java:144)
- [ ] `ModBlocks.WARPED_CABINET`: it uses Minecraft's `Block` (vectorwing/farmersdelight/common/registry/ModBlocks.java:147)
- [ ] `ModBlocks.CANVAS_RUG`: it uses Minecraft's `Block` (vectorwing/farmersdelight/common/registry/ModBlocks.java:150)
- [ ] `ModBlocks.TATAMI`: it uses Minecraft's `Block` (vectorwing/farmersdelight/common/registry/ModBlocks.java:153)
- [ ] `ModBlocks.FULL_TATAMI_MAT`: it uses Minecraft's `Block` (vectorwing/farmersdelight/common/registry/ModBlocks.java:154)
- [ ] `ModBlocks.HALF_TATAMI_MAT`: it uses Minecraft's `Block` (vectorwing/farmersdelight/common/registry/ModBlocks.java:157)
- [ ] `ModBlocks.CANVAS_SIGN`: it uses Minecraft's `Block` (vectorwing/farmersdelight/common/registry/ModBlocks.java:162)
- [ ] `ModBlocks.WHITE_CANVAS_SIGN`: it uses Minecraft's `Block` (vectorwing/farmersdelight/common/registry/ModBlocks.java:163)
- [ ] `ModBlocks.ORANGE_CANVAS_SIGN`: it uses Minecraft's `Block` (vectorwing/farmersdelight/common/registry/ModBlocks.java:164)
- [ ] `ModBlocks.MAGENTA_CANVAS_SIGN`: it uses Minecraft's `Block` (vectorwing/farmersdelight/common/registry/ModBlocks.java:165)
- [ ] `ModBlocks.LIGHT_BLUE_CANVAS_SIGN`: it uses Minecraft's `Block` (vectorwing/farmersdelight/common/registry/ModBlocks.java:166)
- [ ] `ModBlocks.YELLOW_CANVAS_SIGN`: it uses Minecraft's `Block` (vectorwing/farmersdelight/common/registry/ModBlocks.java:167)
- [ ] `ModBlocks.LIME_CANVAS_SIGN`: it uses Minecraft's `Block` (vectorwing/farmersdelight/common/registry/ModBlocks.java:168)
- [ ] `ModBlocks.PINK_CANVAS_SIGN`: it uses Minecraft's `Block` (vectorwing/farmersdelight/common/registry/ModBlocks.java:169)
- [ ] `ModBlocks.GRAY_CANVAS_SIGN`: it uses Minecraft's `Block` (vectorwing/farmersdelight/common/registry/ModBlocks.java:170)
- [ ] `ModBlocks.LIGHT_GRAY_CANVAS_SIGN`: it uses Minecraft's `Block` (vectorwing/farmersdelight/common/registry/ModBlocks.java:171)
- [ ] `ModBlocks.CYAN_CANVAS_SIGN`: it uses Minecraft's `Block` (vectorwing/farmersdelight/common/registry/ModBlocks.java:172)
- [ ] `ModBlocks.PURPLE_CANVAS_SIGN`: it uses Minecraft's `Block` (vectorwing/farmersdelight/common/registry/ModBlocks.java:173)
- [ ] `ModBlocks.BLUE_CANVAS_SIGN`: it uses Minecraft's `Block` (vectorwing/farmersdelight/common/registry/ModBlocks.java:174)
- [ ] `ModBlocks.BROWN_CANVAS_SIGN`: it uses Minecraft's `Block` (vectorwing/farmersdelight/common/registry/ModBlocks.java:175)
- [ ] `ModBlocks.GREEN_CANVAS_SIGN`: it uses Minecraft's `Block` (vectorwing/farmersdelight/common/registry/ModBlocks.java:176)
- [ ] `ModBlocks.RED_CANVAS_SIGN`: it uses Minecraft's `Block` (vectorwing/farmersdelight/common/registry/ModBlocks.java:177)
- [ ] `ModBlocks.BLACK_CANVAS_SIGN`: it uses Minecraft's `Block` (vectorwing/farmersdelight/common/registry/ModBlocks.java:178)
- [ ] `ModBlocks.CANVAS_WALL_SIGN`: it uses Minecraft's `Block` (vectorwing/farmersdelight/common/registry/ModBlocks.java:179)
- [ ] `ModBlocks.WHITE_CANVAS_WALL_SIGN`: it uses Minecraft's `Block` (vectorwing/farmersdelight/common/registry/ModBlocks.java:180)
- [ ] `ModBlocks.ORANGE_CANVAS_WALL_SIGN`: it uses Minecraft's `Block` (vectorwing/farmersdelight/common/registry/ModBlocks.java:181)
- [ ] `ModBlocks.MAGENTA_CANVAS_WALL_SIGN`: it uses Minecraft's `Block` (vectorwing/farmersdelight/common/registry/ModBlocks.java:184)
- [ ] `ModBlocks.LIGHT_BLUE_CANVAS_WALL_SIGN`: it uses Minecraft's `Block` (vectorwing/farmersdelight/common/registry/ModBlocks.java:187)
- [ ] `ModBlocks.YELLOW_CANVAS_WALL_SIGN`: it uses Minecraft's `Block` (vectorwing/farmersdelight/common/registry/ModBlocks.java:190)
- [ ] `ModBlocks.LIME_CANVAS_WALL_SIGN`: it uses Minecraft's `Block` (vectorwing/farmersdelight/common/registry/ModBlocks.java:193)
- [ ] `ModBlocks.PINK_CANVAS_WALL_SIGN`: it uses Minecraft's `Block` (vectorwing/farmersdelight/common/registry/ModBlocks.java:194)
- [ ] `ModBlocks.GRAY_CANVAS_WALL_SIGN`: it uses Minecraft's `Block` (vectorwing/farmersdelight/common/registry/ModBlocks.java:195)
- [ ] `ModBlocks.LIGHT_GRAY_CANVAS_WALL_SIGN`: it uses Minecraft's `Block` (vectorwing/farmersdelight/common/registry/ModBlocks.java:196)
- [ ] `ModBlocks.CYAN_CANVAS_WALL_SIGN`: it uses Minecraft's `Block` (vectorwing/farmersdelight/common/registry/ModBlocks.java:199)
- [ ] `ModBlocks.PURPLE_CANVAS_WALL_SIGN`: it uses Minecraft's `Block` (vectorwing/farmersdelight/common/registry/ModBlocks.java:200)
- [ ] `ModBlocks.BLUE_CANVAS_WALL_SIGN`: it uses Minecraft's `Block` (vectorwing/farmersdelight/common/registry/ModBlocks.java:203)
- [ ] `ModBlocks.BROWN_CANVAS_WALL_SIGN`: it uses Minecraft's `Block` (vectorwing/farmersdelight/common/registry/ModBlocks.java:204)
- [ ] `ModBlocks.GREEN_CANVAS_WALL_SIGN`: it uses Minecraft's `Block` (vectorwing/farmersdelight/common/registry/ModBlocks.java:205)
- [ ] `ModBlocks.RED_CANVAS_WALL_SIGN`: it uses Minecraft's `Block` (vectorwing/farmersdelight/common/registry/ModBlocks.java:206)
- [ ] `ModBlocks.BLACK_CANVAS_WALL_SIGN`: it uses Minecraft's `Block` (vectorwing/farmersdelight/common/registry/ModBlocks.java:207)
- [ ] `ModBlocks.HANGING_CANVAS_SIGN`: it uses Minecraft's `Block` (vectorwing/farmersdelight/common/registry/ModBlocks.java:208)
- [ ] `ModBlocks.WHITE_HANGING_CANVAS_SIGN`: it uses Minecraft's `Block` (vectorwing/farmersdelight/common/registry/ModBlocks.java:209)
- [ ] `ModBlocks.ORANGE_HANGING_CANVAS_SIGN`: it uses Minecraft's `Block` (vectorwing/farmersdelight/common/registry/ModBlocks.java:212)
- [ ] `ModBlocks.MAGENTA_HANGING_CANVAS_SIGN`: it uses Minecraft's `Block` (vectorwing/farmersdelight/common/registry/ModBlocks.java:215)
- [ ] `ModBlocks.LIGHT_BLUE_HANGING_CANVAS_SIGN`: it uses Minecraft's `Block` (vectorwing/farmersdelight/common/registry/ModBlocks.java:218)
- [ ] `ModBlocks.YELLOW_HANGING_CANVAS_SIGN`: it uses Minecraft's `Block` (vectorwing/farmersdelight/common/registry/ModBlocks.java:221)
- [ ] `ModBlocks.LIME_HANGING_CANVAS_SIGN`: it uses Minecraft's `Block` (vectorwing/farmersdelight/common/registry/ModBlocks.java:224)
- [ ] `ModBlocks.PINK_HANGING_CANVAS_SIGN`: it uses Minecraft's `Block` (vectorwing/farmersdelight/common/registry/ModBlocks.java:227)
- [ ] `ModBlocks.GRAY_HANGING_CANVAS_SIGN`: it uses Minecraft's `Block` (vectorwing/farmersdelight/common/registry/ModBlocks.java:230)
- [ ] `ModBlocks.LIGHT_GRAY_HANGING_CANVAS_SIGN`: it uses Minecraft's `Block` (vectorwing/farmersdelight/common/registry/ModBlocks.java:233)
- [ ] `ModBlocks.CYAN_HANGING_CANVAS_SIGN`: it uses Minecraft's `Block` (vectorwing/farmersdelight/common/registry/ModBlocks.java:236)
- [ ] `ModBlocks.PURPLE_HANGING_CANVAS_SIGN`: it uses Minecraft's `Block` (vectorwing/farmersdelight/common/registry/ModBlocks.java:239)
- [ ] `ModBlocks.BLUE_HANGING_CANVAS_SIGN`: it uses Minecraft's `Block` (vectorwing/farmersdelight/common/registry/ModBlocks.java:242)
- [ ] `ModBlocks.BROWN_HANGING_CANVAS_SIGN`: it uses Minecraft's `Block` (vectorwing/farmersdelight/common/registry/ModBlocks.java:245)
- [ ] `ModBlocks.GREEN_HANGING_CANVAS_SIGN`: it uses Minecraft's `Block` (vectorwing/farmersdelight/common/registry/ModBlocks.java:248)
- [ ] `ModBlocks.RED_HANGING_CANVAS_SIGN`: it uses Minecraft's `Block` (vectorwing/farmersdelight/common/registry/ModBlocks.java:251)
- [ ] `ModBlocks.BLACK_HANGING_CANVAS_SIGN`: it uses Minecraft's `Block` (vectorwing/farmersdelight/common/registry/ModBlocks.java:252)
- [ ] `ModBlocks.HANGING_CANVAS_WALL_SIGN`: it uses Minecraft's `Block` (vectorwing/farmersdelight/common/registry/ModBlocks.java:255)
- [ ] `ModBlocks.WHITE_HANGING_CANVAS_WALL_SIGN`: it uses Minecraft's `Block` (vectorwing/farmersdelight/common/registry/ModBlocks.java:256)
- [ ] `ModBlocks.ORANGE_HANGING_CANVAS_WALL_SIGN`: it uses Minecraft's `Block` (vectorwing/farmersdelight/common/registry/ModBlocks.java:259)
- [ ] `ModBlocks.MAGENTA_HANGING_CANVAS_WALL_SIGN`: it uses Minecraft's `Block` (vectorwing/farmersdelight/common/registry/ModBlocks.java:262)
- [ ] `ModBlocks.LIGHT_BLUE_HANGING_CANVAS_WALL_SIGN`: it uses Minecraft's `Block` (vectorwing/farmersdelight/common/registry/ModBlocks.java:265)
- [ ] `ModBlocks.YELLOW_HANGING_CANVAS_WALL_SIGN`: it uses Minecraft's `Block` (vectorwing/farmersdelight/common/registry/ModBlocks.java:268)
- [ ] `ModBlocks.LIME_HANGING_CANVAS_WALL_SIGN`: it uses Minecraft's `Block` (vectorwing/farmersdelight/common/registry/ModBlocks.java:271)
- [ ] `ModBlocks.PINK_HANGING_CANVAS_WALL_SIGN`: it uses Minecraft's `Block` (vectorwing/farmersdelight/common/registry/ModBlocks.java:274)
- [ ] `ModBlocks.GRAY_HANGING_CANVAS_WALL_SIGN`: it uses Minecraft's `Block` (vectorwing/farmersdelight/common/registry/ModBlocks.java:277)
- [ ] `ModBlocks.LIGHT_GRAY_HANGING_CANVAS_WALL_SIGN`: it uses Minecraft's `Block` (vectorwing/farmersdelight/common/registry/ModBlocks.java:280)
- [ ] `ModBlocks.CYAN_HANGING_CANVAS_WALL_SIGN`: it uses Minecraft's `Block` (vectorwing/farmersdelight/common/registry/ModBlocks.java:283)
- [ ] `ModBlocks.PURPLE_HANGING_CANVAS_WALL_SIGN`: it uses Minecraft's `Block` (vectorwing/farmersdelight/common/registry/ModBlocks.java:286)
- [ ] `ModBlocks.BLUE_HANGING_CANVAS_WALL_SIGN`: it uses Minecraft's `Block` (vectorwing/farmersdelight/common/registry/ModBlocks.java:289)
- [ ] `ModBlocks.BROWN_HANGING_CANVAS_WALL_SIGN`: it uses Minecraft's `Block` (vectorwing/farmersdelight/common/registry/ModBlocks.java:292)
- [ ] `ModBlocks.GREEN_HANGING_CANVAS_WALL_SIGN`: it uses Minecraft's `Block` (vectorwing/farmersdelight/common/registry/ModBlocks.java:295)
- [ ] `ModBlocks.RED_HANGING_CANVAS_WALL_SIGN`: it uses Minecraft's `Block` (vectorwing/farmersdelight/common/registry/ModBlocks.java:298)
- [ ] `ModBlocks.BLACK_HANGING_CANVAS_WALL_SIGN`: it uses Minecraft's `Block` (vectorwing/farmersdelight/common/registry/ModBlocks.java:301)
- [ ] `ModBlocks.BROWN_MUSHROOM_COLONY`: it uses Minecraft's `Block` (vectorwing/farmersdelight/common/registry/ModBlocks.java:304)
- [ ] `ModBlocks.RED_MUSHROOM_COLONY`: it uses Minecraft's `Block` (vectorwing/farmersdelight/common/registry/ModBlocks.java:309)
- [ ] `ModBlocks.ORGANIC_COMPOST`: it uses Minecraft's `Block` (vectorwing/farmersdelight/common/registry/ModBlocks.java:314)
- [ ] `ModBlocks.RICH_SOIL`: it uses Minecraft's `Block` (vectorwing/farmersdelight/common/registry/ModBlocks.java:317)
- [ ] `ModBlocks.RICH_SOIL_FARMLAND`: it uses Minecraft's `Block` (vectorwing/farmersdelight/common/registry/ModBlocks.java:318)
- [ ] `ModBlocks.APPLE_PIE`: it uses Minecraft's `Block` (vectorwing/farmersdelight/common/registry/ModBlocks.java:321)
- [ ] `ModBlocks.SWEET_BERRY_CHEESECAKE`: it uses Minecraft's `Block` (vectorwing/farmersdelight/common/registry/ModBlocks.java:324)
- [ ] `ModBlocks.CHOCOLATE_PIE`: it uses Minecraft's `Block` (vectorwing/farmersdelight/common/registry/ModBlocks.java:329)
- [ ] `ModBlocks.PUMPKIN_PIE`: it uses Minecraft's `Block` (vectorwing/farmersdelight/common/registry/ModBlocks.java:332)
- [ ] `ModBlocks.SANDY_SHRUB`: it uses Minecraft's `Block` (vectorwing/farmersdelight/common/registry/ModBlocks.java:335)
- [ ] `ModBlocks.WILD_CABBAGES`: it uses Minecraft's `Block` (vectorwing/farmersdelight/common/registry/ModBlocks.java:336)
- [ ] `ModBlocks.WILD_ONIONS`: it uses Minecraft's `Block` (vectorwing/farmersdelight/common/registry/ModBlocks.java:339)
- [ ] `ModBlocks.WILD_TOMATOES`: it uses Minecraft's `Block` (vectorwing/farmersdelight/common/registry/ModBlocks.java:342)
- [ ] `ModBlocks.WILD_CARROTS`: it uses Minecraft's `Block` (vectorwing/farmersdelight/common/registry/ModBlocks.java:345)
- [ ] `ModBlocks.WILD_POTATOES`: it uses Minecraft's `Block` (vectorwing/farmersdelight/common/registry/ModBlocks.java:348)
- [ ] `ModBlocks.WILD_BEETROOTS`: it uses Minecraft's `Block` (vectorwing/farmersdelight/common/registry/ModBlocks.java:351)
- [ ] `ModBlocks.WILD_RICE`: it uses Minecraft's `Block` (vectorwing/farmersdelight/common/registry/ModBlocks.java:354)
- [ ] `ModBlocks.CABBAGE_CROP`: it uses Minecraft's `Block` (vectorwing/farmersdelight/common/registry/ModBlocks.java:355)
- [ ] `ModBlocks.ONION_CROP`: it uses Minecraft's `Block` (vectorwing/farmersdelight/common/registry/ModBlocks.java:356)
- [ ] `ModBlocks.BUDDING_TOMATO_CROP`: it uses Minecraft's `Block` (vectorwing/farmersdelight/common/registry/ModBlocks.java:357)
- [ ] `ModBlocks.TOMATO_CROP`: it uses Minecraft's `Block` (vectorwing/farmersdelight/common/registry/ModBlocks.java:360)
- [ ] `ModBlocks.TOMATO_CROP_ON_ROPE`: it uses Minecraft's `Block` (vectorwing/farmersdelight/common/registry/ModBlocks.java:363)
- [ ] `ModBlocks.RICE_CROP`: it uses Minecraft's `Block` (vectorwing/farmersdelight/common/registry/ModBlocks.java:366)
- [ ] `ModBlocks.RICE_CROP_PANICLES`: it uses Minecraft's `Block` (vectorwing/farmersdelight/common/registry/ModBlocks.java:369)
- [ ] `ModBlocks.ROAST_CHICKEN_BLOCK`: it uses Minecraft's `Block` (vectorwing/farmersdelight/common/registry/ModBlocks.java:372)
- [ ] `ModBlocks.STUFFED_PUMPKIN_BLOCK`: it uses Minecraft's `Block` (vectorwing/farmersdelight/common/registry/ModBlocks.java:377)
- [ ] `ModBlocks.HONEY_GLAZED_HAM_BLOCK`: it uses Minecraft's `Block` (vectorwing/farmersdelight/common/registry/ModBlocks.java:380)
- [ ] `ModBlocks.SHEPHERDS_PIE_BLOCK`: it uses Minecraft's `Block` (vectorwing/farmersdelight/common/registry/ModBlocks.java:385)
- [ ] `ModBlocks.GLEAMING_SALAD_BLOCK`: it uses Minecraft's `Block` (vectorwing/farmersdelight/common/registry/ModBlocks.java:390)
- [ ] `ModBlocks.RICE_ROLL_MEDLEY_BLOCK`: it uses Minecraft's `Block` (vectorwing/farmersdelight/common/registry/ModBlocks.java:395)
- [ ] `ModBlocks.STRAW_BED`: it uses Minecraft's `Block` (vectorwing/farmersdelight/common/registry/ModBlocks.java:396)
- [ ] `ModBlocks.litBlockEmission(1)`: it uses Minecraft's `BlockState` (vectorwing/farmersdelight/common/registry/ModBlocks.java:402)
- [ ] `ModBlocks.glowingFeastBlockEmission(0)`: it uses Minecraft's `BlockState` (vectorwing/farmersdelight/common/registry/ModBlocks.java:406)
- [ ] `ModBlocks.lootTableKey(1)`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/common/registry/ModBlocks.java:410)
- [ ] `ModBlocks.regCanvasSign(2)`: it uses Minecraft's `Block` (vectorwing/farmersdelight/common/registry/ModBlocks.java:414)
- [ ] `ModBlocks.regWallCanvasSign(2)`: it uses Minecraft's `Block` (vectorwing/farmersdelight/common/registry/ModBlocks.java:418)
- [ ] `ModBlocks.regHangingCanvasSign(2)`: it uses Minecraft's `Block` (vectorwing/farmersdelight/common/registry/ModBlocks.java:427)
- [ ] `ModBlocks.regWallHangingCanvasSign(2)`: it uses Minecraft's `Block` (vectorwing/farmersdelight/common/registry/ModBlocks.java:431)
- [ ] `ModBlocks.regBlock(3)`: it uses Minecraft's `Block` (vectorwing/farmersdelight/common/registry/ModBlocks.java:440)
- [ ] `ModBlocks.regBlock(3)`: it uses Minecraft's `Block` (vectorwing/farmersdelight/common/registry/ModBlocks.java:445)
- [ ] `ModBlocks.feastProperties(0)`: it calls Minecraft's `Properties`.of (vectorwing/farmersdelight/common/registry/ModBlocks.java:449)
- [x] `ModBlocks.touch(0)` (vectorwing/farmersdelight/common/registry/ModBlocks.java:453)

### `ModChestLootTables`

- [ ] `ModChestLootTables.ABANDONED_MINESHAFT`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/common/registry/ModChestLootTables.java:9)
- [ ] `ModChestLootTables.BASTION_HOGLIN_STABLE`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/common/registry/ModChestLootTables.java:10)
- [ ] `ModChestLootTables.BASTION_TREASURE`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/common/registry/ModChestLootTables.java:11)
- [ ] `ModChestLootTables.END_CITY_TREASURE`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/common/registry/ModChestLootTables.java:12)
- [ ] `ModChestLootTables.PILLAGER_OUTPOST`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/common/registry/ModChestLootTables.java:13)
- [ ] `ModChestLootTables.RUINED_PORTAL`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/common/registry/ModChestLootTables.java:14)
- [ ] `ModChestLootTables.SHIPWRECK_SUPPLY`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/common/registry/ModChestLootTables.java:15)
- [ ] `ModChestLootTables.SIMPLE_DUNGEON`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/common/registry/ModChestLootTables.java:16)
- [ ] `ModChestLootTables.VILLAGE_BUTCHER`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/common/registry/ModChestLootTables.java:17)
- [ ] `ModChestLootTables.VILLAGE_DESERT_HOUSE`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/common/registry/ModChestLootTables.java:18)
- [ ] `ModChestLootTables.VILLAGE_PLAINS_HOUSE`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/common/registry/ModChestLootTables.java:19)
- [ ] `ModChestLootTables.VILLAGE_SAVANNA_HOUSE`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/common/registry/ModChestLootTables.java:20)
- [ ] `ModChestLootTables.VILLAGE_SNOWY_HOUSE`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/common/registry/ModChestLootTables.java:21)
- [ ] `ModChestLootTables.VILLAGE_TAIGA_HOUSE`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/common/registry/ModChestLootTables.java:22)
- [ ] `ModChestLootTables.register(1)`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/common/registry/ModChestLootTables.java:24)

### `ModConsumeEffectTypes`

- [ ] `ModConsumeEffectTypes.REMOVE_RANDOM_EFFECTS`: it uses Minecraft's `Type` (vectorwing/farmersdelight/common/registry/ModConsumeEffectTypes.java:11)
- [ ] `ModConsumeEffectTypes.HEAL`: it uses Minecraft's `Type` (vectorwing/farmersdelight/common/registry/ModConsumeEffectTypes.java:14)
- [ ] `ModConsumeEffectTypes.EXTINGUISH`: it uses Minecraft's `Type` (vectorwing/farmersdelight/common/registry/ModConsumeEffectTypes.java:17)
- [x] `ModConsumeEffectTypes.touch(0)` (vectorwing/farmersdelight/common/registry/ModConsumeEffectTypes.java:21)

### `ModCreativeTabs`

- [ ] `ModCreativeTabs.TAB_FARMERS_DELIGHT`: it uses Minecraft's `CreativeModeTab` (vectorwing/farmersdelight/common/registry/ModCreativeTabs.java:12)
- [x] `ModCreativeTabs.touch(0)` (vectorwing/farmersdelight/common/registry/ModCreativeTabs.java:21)

### `ModDamageTypes`

- [ ] `ModDamageTypes.STOVE_BURN`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/common/registry/ModDamageTypes.java:14)
- [ ] `ModDamageTypes.getSimpleDamageSource(2)`: it uses Minecraft's `DamageSource` (vectorwing/farmersdelight/common/registry/ModDamageTypes.java:18)
- [ ] `ModDamageTypes.bootstrapDamageTypes(1)`: it uses Minecraft's `BootstrapContext` (vectorwing/farmersdelight/common/registry/ModDamageTypes.java:22)

### `ModDataComponents`

- [ ] `ModDataComponents.MEAL`: it uses Minecraft's `DataComponentType` (vectorwing/farmersdelight/common/registry/ModDataComponents.java:14)
- [ ] `ModDataComponents.CONTAINER`: it uses Minecraft's `DataComponentType` (vectorwing/farmersdelight/common/registry/ModDataComponents.java:17)
- [ ] `ModDataComponents.COOKING_TIME_LENGTH`: it uses Minecraft's `DataComponentType` (vectorwing/farmersdelight/common/registry/ModDataComponents.java:20)
- [ ] `ModDataComponents.SKILLET_INGREDIENT`: it uses Minecraft's `DataComponentType` (vectorwing/farmersdelight/common/registry/ModDataComponents.java:23)
- [ ] `ModDataComponents.SKILLET_FLIP_TIMESTAMP`: it uses Minecraft's `DataComponentType` (vectorwing/farmersdelight/common/registry/ModDataComponents.java:26)
- [ ] `ModDataComponents.SKILLET_FLIPPED`: it uses Minecraft's `DataComponentType` (vectorwing/farmersdelight/common/registry/ModDataComponents.java:29)
- [ ] `ModDataComponents.BACKSTABBING`: it uses Minecraft's `DataComponentType` (vectorwing/farmersdelight/common/registry/ModDataComponents.java:32)
- [x] `ModDataComponents.touch(0)` (vectorwing/farmersdelight/common/registry/ModDataComponents.java:36)

### `ModEffects`

- [ ] `ModEffects.NOURISHMENT`: it uses Minecraft's `Holder` (vectorwing/farmersdelight/common/registry/ModEffects.java:12)
- [ ] `ModEffects.COMFORT`: it uses Minecraft's `Holder` (vectorwing/farmersdelight/common/registry/ModEffects.java:15)
- [x] `ModEffects.register(0)` (vectorwing/farmersdelight/common/registry/ModEffects.java:17)
- [x] `ModEffects.touch(0)` (vectorwing/farmersdelight/common/registry/ModEffects.java:20)

### `ModEntityTypes`

- [ ] `ModEntityTypes.ROTTEN_TOMATO`: it uses Minecraft's `EntityType` (vectorwing/farmersdelight/common/registry/ModEntityTypes.java:15)
- [x] `ModEntityTypes.touch(0)` (vectorwing/farmersdelight/common/registry/ModEntityTypes.java:24)

### `ModIngredientTypes`


### `ModItems`

- [ ] `ModItems.CREATIVE_TAB_ITEMS`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:63)
- [ ] `ModItems.STOVE`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:64)
- [ ] `ModItems.COOKING_POT`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:65)
- [ ] `ModItems.SKILLET`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:68)
- [ ] `ModItems.CUTTING_BOARD`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:81)
- [ ] `ModItems.WOODEN_BASKET`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:82)
- [ ] `ModItems.BAMBOO_BASKET`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:83)
- [ ] `ModItems.BASKET`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:85)
- [ ] `ModItems.CARROT_CRATE`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:86)
- [ ] `ModItems.POTATO_CRATE`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:89)
- [ ] `ModItems.BEETROOT_CRATE`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:92)
- [ ] `ModItems.CABBAGE_CRATE`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:95)
- [ ] `ModItems.TOMATO_CRATE`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:98)
- [ ] `ModItems.ONION_CRATE`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:101)
- [ ] `ModItems.RICE_BALE`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:102)
- [ ] `ModItems.RICE_BAG`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:103)
- [ ] `ModItems.STRAW_BALE`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:104)
- [ ] `ModItems.SAFETY_NET`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:105)
- [ ] `ModItems.OAK_CABINET`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:106)
- [ ] `ModItems.SPRUCE_CABINET`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:107)
- [ ] `ModItems.BIRCH_CABINET`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:110)
- [ ] `ModItems.JUNGLE_CABINET`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:111)
- [ ] `ModItems.ACACIA_CABINET`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:114)
- [ ] `ModItems.DARK_OAK_CABINET`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:117)
- [ ] `ModItems.MANGROVE_CABINET`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:120)
- [ ] `ModItems.CHERRY_CABINET`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:123)
- [ ] `ModItems.BAMBOO_CABINET`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:126)
- [ ] `ModItems.PALE_OAK_CABINET`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:129)
- [ ] `ModItems.POPLAR_CABINET`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:132)
- [ ] `ModItems.CRIMSON_CABINET`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:135)
- [ ] `ModItems.WARPED_CABINET`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:138)
- [ ] `ModItems.TATAMI`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:141)
- [ ] `ModItems.FULL_TATAMI_MAT`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:142)
- [ ] `ModItems.HALF_TATAMI_MAT`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:145)
- [ ] `ModItems.CANVAS_RUG`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:148)
- [ ] `ModItems.ROPE_FENCE`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:149)
- [ ] `ModItems.ROPE_FENCE_GATE`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:150)
- [ ] `ModItems.ORGANIC_COMPOST`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:153)
- [ ] `ModItems.RICH_SOIL`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:156)
- [ ] `ModItems.RICH_SOIL_FARMLAND`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:157)
- [ ] `ModItems.ROPE`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:160)
- [ ] `ModItems.CANVAS_SIGN`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:161)
- [ ] `ModItems.HANGING_CANVAS_SIGN`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:166)
- [ ] `ModItems.WHITE_CANVAS_SIGN`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:171)
- [ ] `ModItems.WHITE_HANGING_CANVAS_SIGN`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:176)
- [ ] `ModItems.LIGHT_GRAY_CANVAS_SIGN`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:181)
- [ ] `ModItems.LIGHT_GRAY_HANGING_CANVAS_SIGN`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:188)
- [ ] `ModItems.GRAY_CANVAS_SIGN`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:193)
- [ ] `ModItems.GRAY_HANGING_CANVAS_SIGN`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:198)
- [ ] `ModItems.BLACK_CANVAS_SIGN`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:203)
- [ ] `ModItems.BLACK_HANGING_CANVAS_SIGN`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:208)
- [ ] `ModItems.BROWN_CANVAS_SIGN`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:213)
- [ ] `ModItems.BROWN_HANGING_CANVAS_SIGN`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:218)
- [ ] `ModItems.RED_CANVAS_SIGN`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:223)
- [ ] `ModItems.RED_HANGING_CANVAS_SIGN`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:228)
- [ ] `ModItems.ORANGE_CANVAS_SIGN`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:233)
- [ ] `ModItems.ORANGE_HANGING_CANVAS_SIGN`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:238)
- [ ] `ModItems.YELLOW_CANVAS_SIGN`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:243)
- [ ] `ModItems.YELLOW_HANGING_CANVAS_SIGN`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:248)
- [ ] `ModItems.LIME_CANVAS_SIGN`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:253)
- [ ] `ModItems.LIME_HANGING_CANVAS_SIGN`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:258)
- [ ] `ModItems.GREEN_CANVAS_SIGN`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:263)
- [ ] `ModItems.GREEN_HANGING_CANVAS_SIGN`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:268)
- [ ] `ModItems.CYAN_CANVAS_SIGN`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:273)
- [ ] `ModItems.CYAN_HANGING_CANVAS_SIGN`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:278)
- [ ] `ModItems.LIGHT_BLUE_CANVAS_SIGN`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:283)
- [ ] `ModItems.LIGHT_BLUE_HANGING_CANVAS_SIGN`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:290)
- [ ] `ModItems.BLUE_CANVAS_SIGN`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:295)
- [ ] `ModItems.BLUE_HANGING_CANVAS_SIGN`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:300)
- [ ] `ModItems.PURPLE_CANVAS_SIGN`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:305)
- [ ] `ModItems.PURPLE_HANGING_CANVAS_SIGN`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:310)
- [ ] `ModItems.MAGENTA_CANVAS_SIGN`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:315)
- [ ] `ModItems.MAGENTA_HANGING_CANVAS_SIGN`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:320)
- [ ] `ModItems.PINK_CANVAS_SIGN`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:325)
- [ ] `ModItems.PINK_HANGING_CANVAS_SIGN`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:330)
- [ ] `ModItems.FLINT_KNIFE`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:335)
- [ ] `ModItems.COPPER_KNIFE`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:336)
- [ ] `ModItems.IRON_KNIFE`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:337)
- [ ] `ModItems.DIAMOND_KNIFE`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:338)
- [ ] `ModItems.NETHERITE_KNIFE`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:339)
- [ ] `ModItems.GOLDEN_KNIFE`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:342)
- [ ] `ModItems.STRAW`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:343)
- [ ] `ModItems.CANVAS`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:344)
- [ ] `ModItems.TREE_BARK`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:345)
- [ ] `ModItems.SANDY_SHRUB`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:346)
- [ ] `ModItems.WILD_CABBAGES`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:347)
- [ ] `ModItems.WILD_ONIONS`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:350)
- [ ] `ModItems.WILD_TOMATOES`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:351)
- [ ] `ModItems.WILD_CARROTS`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:354)
- [ ] `ModItems.WILD_POTATOES`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:357)
- [ ] `ModItems.WILD_BEETROOTS`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:360)
- [ ] `ModItems.WILD_RICE`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:363)
- [ ] `ModItems.BROWN_MUSHROOM_COLONY`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:366)
- [ ] `ModItems.RED_MUSHROOM_COLONY`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:369)
- [ ] `ModItems.CABBAGE`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:372)
- [ ] `ModItems.TOMATO`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:375)
- [ ] `ModItems.ONION`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:378)
- [ ] `ModItems.RICE_PANICLE`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:384)
- [ ] `ModItems.RICE`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:385)
- [ ] `ModItems.CABBAGE_SEEDS`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:388)
- [ ] `ModItems.TOMATO_SEEDS`: its value couldn't be read: an anonymous class (vectorwing/farmersdelight/common/registry/ModItems.java:391)
- [ ] `ModItems.ROTTEN_TOMATO`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:399)
- [ ] `ModItems.FRIED_EGG`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:400)
- [ ] `ModItems.MILK_BOTTLE`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:401)
- [ ] `ModItems.HOT_COCOA`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:404)
- [ ] `ModItems.APPLE_CIDER`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:407)
- [ ] `ModItems.MELON_JUICE`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:412)
- [ ] `ModItems.TOMATO_SAUCE`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:415)
- [ ] `ModItems.WHEAT_DOUGH`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:418)
- [ ] `ModItems.RAW_PASTA`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:421)
- [ ] `ModItems.PUMPKIN_SLICE`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:424)
- [ ] `ModItems.CABBAGE_LEAF`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:425)
- [ ] `ModItems.MINCED_BEEF`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:428)
- [ ] `ModItems.BEEF_PATTY`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:431)
- [ ] `ModItems.CHICKEN_CUTS`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:434)
- [ ] `ModItems.COOKED_CHICKEN_CUTS`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:437)
- [ ] `ModItems.BACON`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:440)
- [ ] `ModItems.COOKED_BACON`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:441)
- [ ] `ModItems.COD_SLICE`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:444)
- [ ] `ModItems.COOKED_COD_SLICE`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:447)
- [ ] `ModItems.SALMON_SLICE`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:450)
- [ ] `ModItems.COOKED_SALMON_SLICE`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:453)
- [ ] `ModItems.MUTTON_CHOPS`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:456)
- [ ] `ModItems.COOKED_MUTTON_CHOPS`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:459)
- [ ] `ModItems.HAM`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:462)
- [ ] `ModItems.SMOKED_HAM`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:463)
- [ ] `ModItems.PIE_CRUST`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:464)
- [ ] `ModItems.APPLE_PIE`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:465)
- [ ] `ModItems.SWEET_BERRY_CHEESECAKE`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:466)
- [ ] `ModItems.CHOCOLATE_PIE`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:469)
- [ ] `ModItems.CAKE_SLICE`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:472)
- [ ] `ModItems.APPLE_PIE_SLICE`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:475)
- [ ] `ModItems.SWEET_BERRY_CHEESECAKE_SLICE`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:478)
- [ ] `ModItems.CHOCOLATE_PIE_SLICE`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:481)
- [ ] `ModItems.PUMPKIN_PIE_SLICE`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:484)
- [ ] `ModItems.SWEET_BERRY_COOKIE`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:487)
- [ ] `ModItems.HONEY_COOKIE`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:490)
- [ ] `ModItems.MELON_POPSICLE`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:493)
- [ ] `ModItems.GLOW_BERRY_CUSTARD`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:496)
- [ ] `ModItems.FRUIT_SALAD`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:501)
- [ ] `ModItems.MIXED_SALAD`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:504)
- [ ] `ModItems.NETHER_SALAD`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:507)
- [ ] `ModItems.BARBECUE_STICK`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:510)
- [ ] `ModItems.EGG_SANDWICH`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:511)
- [ ] `ModItems.CHICKEN_SANDWICH`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:512)
- [ ] `ModItems.HAMBURGER`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:515)
- [ ] `ModItems.BACON_SANDWICH`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:516)
- [ ] `ModItems.MUTTON_WRAP`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:517)
- [ ] `ModItems.DUMPLINGS`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:518)
- [ ] `ModItems.STUFFED_POTATO`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:519)
- [ ] `ModItems.CABBAGE_ROLLS`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:520)
- [ ] `ModItems.SALMON_ROLL`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:521)
- [ ] `ModItems.COD_ROLL`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:522)
- [ ] `ModItems.KELP_ROLL`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:523)
- [ ] `ModItems.KELP_ROLL_SLICE`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:526)
- [ ] `ModItems.COOKED_RICE`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:529)
- [ ] `ModItems.BONE_BROTH`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:534)
- [ ] `ModItems.BEEF_STEW`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:537)
- [ ] `ModItems.CHICKEN_SOUP`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:542)
- [ ] `ModItems.VEGETABLE_SOUP`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:547)
- [ ] `ModItems.FISH_STEW`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:552)
- [ ] `ModItems.FRIED_RICE`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:557)
- [ ] `ModItems.PUMPKIN_SOUP`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:562)
- [ ] `ModItems.BAKED_COD_STEW`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:567)
- [ ] `ModItems.NOODLE_SOUP`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:572)
- [ ] `ModItems.ONION_SOUP`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:577)
- [ ] `ModItems.BACON_AND_EGGS`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:582)
- [ ] `ModItems.PASTA_WITH_MEATBALLS`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:587)
- [ ] `ModItems.PASTA_WITH_MUTTON_CHOP`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:592)
- [ ] `ModItems.MUSHROOM_RICE`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:597)
- [ ] `ModItems.ROASTED_MUTTON_CHOPS`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:602)
- [ ] `ModItems.VEGETABLE_NOODLES`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:607)
- [ ] `ModItems.STEAK_AND_POTATOES`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:612)
- [ ] `ModItems.RATATOUILLE`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:617)
- [ ] `ModItems.SQUID_INK_PASTA`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:622)
- [ ] `ModItems.GRILLED_SALMON`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:627)
- [ ] `ModItems.ROAST_CHICKEN_BLOCK`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:632)
- [ ] `ModItems.ROAST_CHICKEN`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:635)
- [ ] `ModItems.STUFFED_PUMPKIN_BLOCK`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:640)
- [ ] `ModItems.STUFFED_PUMPKIN`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:643)
- [ ] `ModItems.HONEY_GLAZED_HAM_BLOCK`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:648)
- [ ] `ModItems.HONEY_GLAZED_HAM`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:651)
- [ ] `ModItems.SHEPHERDS_PIE_BLOCK`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:656)
- [ ] `ModItems.SHEPHERDS_PIE`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:659)
- [ ] `ModItems.GLEAMING_SALAD_BLOCK`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:664)
- [ ] `ModItems.GLEAMING_SALAD`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:667)
- [ ] `ModItems.RICE_ROLL_MEDLEY_BLOCK`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:672)
- [ ] `ModItems.DOG_FOOD`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:675)
- [ ] `ModItems.HORSE_FEED`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:676)
- [ ] `ModItems.STRAW_BED`: it uses Minecraft's `Item` (vectorwing/farmersdelight/common/registry/ModItems.java:677)
- [ ] `ModItems.DEBUG_PUMPKIN_PIE`: its value couldn't be read: an anonymous class (vectorwing/farmersdelight/common/registry/ModItems.java:680)
- [ ] `ModItems.registerWithTab(3)`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/common/registry/ModItems.java:689)
- [ ] `ModItems.registerHidden(3)`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/common/registry/ModItems.java:696)
- [ ] `ModItems.registerFuelWithTab(3)`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/common/registry/ModItems.java:701)
- [ ] `ModItems.registerBlockWithTab(4)`: it uses Minecraft's `BlockItemId` (vectorwing/farmersdelight/common/registry/ModItems.java:708)
- [ ] `ModItems.registerItemNameBlockWithTab(4)`: it uses Minecraft's `BlockItemId` (vectorwing/farmersdelight/common/registry/ModItems.java:716)
- [ ] `ModItems.registerFuelBlockWithTab(4)`: it uses Minecraft's `BlockItemId` (vectorwing/farmersdelight/common/registry/ModItems.java:723)
- [x] `ModItems.basicItem(0)` (vectorwing/farmersdelight/common/registry/ModItems.java:731)
- [ ] `ModItems.knifeItem(1)`: it uses Minecraft's `ToolMaterial` (vectorwing/farmersdelight/common/registry/ModItems.java:735)
- [ ] `ModItems.foodItem(1)`: it uses Minecraft's `FoodProperties` (vectorwing/farmersdelight/common/registry/ModItems.java:758)
- [ ] `ModItems.foodItem(2)`: it uses Minecraft's `FoodProperties` (vectorwing/farmersdelight/common/registry/ModItems.java:762)
- [ ] `ModItems.bowlFoodItem(1)`: it uses Minecraft's `FoodProperties` (vectorwing/farmersdelight/common/registry/ModItems.java:766)
- [ ] `ModItems.bowlFoodItem(2)`: it uses Minecraft's `FoodProperties` (vectorwing/farmersdelight/common/registry/ModItems.java:770)
- [ ] `ModItems.drinkItem(0)`: it calls ModItems.drinkItem, which isn't translated (vectorwing/farmersdelight/common/registry/ModItems.java:778)
- [ ] `ModItems.drinkItem(1)`: it uses Minecraft's `Consumable` (vectorwing/farmersdelight/common/registry/ModItems.java:782)
- [x] `ModItems.touch(0)` (vectorwing/farmersdelight/common/registry/ModItems.java:789)

### `ModLootFunctions`

- [ ] `ModLootFunctions.COPY_SKILLET`: it uses Minecraft's `MapCodec` (vectorwing/farmersdelight/common/registry/ModLootFunctions.java:10)
- [ ] `ModLootFunctions.SMOKER_COOK`: it uses Minecraft's `MapCodec` (vectorwing/farmersdelight/common/registry/ModLootFunctions.java:11)
- [x] `ModLootFunctions.touch(0)` (vectorwing/farmersdelight/common/registry/ModLootFunctions.java:13)

### `ModMenuTypes`

- [ ] `ModMenuTypes.COOKING_POT`: it uses Minecraft's `MenuType` (vectorwing/farmersdelight/common/registry/ModMenuTypes.java:11)
- [x] `ModMenuTypes.touch(0)` (vectorwing/farmersdelight/common/registry/ModMenuTypes.java:15)

### `ModParticleTypes`

- [ ] `ModParticleTypes.STAR`: it uses Minecraft's `SimpleParticleType` (vectorwing/farmersdelight/common/registry/ModParticleTypes.java:9)
- [ ] `ModParticleTypes.STEAM`: it uses Minecraft's `SimpleParticleType` (vectorwing/farmersdelight/common/registry/ModParticleTypes.java:10)
- [ ] `ModParticleTypes.SPARKLE`: it uses Minecraft's `SimpleParticleType` (vectorwing/farmersdelight/common/registry/ModParticleTypes.java:11)
- [x] `ModParticleTypes.touch(0)` (vectorwing/farmersdelight/common/registry/ModParticleTypes.java:13)

### `ModPlacementModifiers`

- [ ] `ModPlacementModifiers.BIOME_TAG`: it uses Minecraft's `MapCodec` (vectorwing/farmersdelight/common/registry/ModPlacementModifiers.java:9)
- [x] `ModPlacementModifiers.touch(0)` (vectorwing/farmersdelight/common/registry/ModPlacementModifiers.java:11)

### `ModRecipeBookCategories`

- [ ] `ModRecipeBookCategories.COOKING_MEALS`: it uses Minecraft's `RecipeBookCategory` (vectorwing/farmersdelight/common/registry/ModRecipeBookCategories.java:8)
- [ ] `ModRecipeBookCategories.COOKING_DRINKS`: it uses Minecraft's `RecipeBookCategory` (vectorwing/farmersdelight/common/registry/ModRecipeBookCategories.java:9)
- [ ] `ModRecipeBookCategories.COOKING_MISC`: it uses Minecraft's `RecipeBookCategory` (vectorwing/farmersdelight/common/registry/ModRecipeBookCategories.java:10)
- [ ] `ModRecipeBookCategories.CUTTING`: it uses Minecraft's `RecipeBookCategory` (vectorwing/farmersdelight/common/registry/ModRecipeBookCategories.java:11)
- [x] `ModRecipeBookCategories.touch(0)` (vectorwing/farmersdelight/common/registry/ModRecipeBookCategories.java:13)

### `ModRecipeDisplays`

- [ ] `ModRecipeDisplays.COOKING`: it uses Minecraft's `Type` (vectorwing/farmersdelight/common/registry/ModRecipeDisplays.java:9)
- [x] `ModRecipeDisplays.touch(0)` (vectorwing/farmersdelight/common/registry/ModRecipeDisplays.java:11)

### `ModRecipePropertySets`

- [ ] `ModRecipePropertySets.COOKING_POT_INPUT_ONE`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/common/registry/ModRecipePropertySets.java:8)
- [ ] `ModRecipePropertySets.COOKING_POT_INPUT_TWO`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/common/registry/ModRecipePropertySets.java:11)
- [ ] `ModRecipePropertySets.COOKING_POT_INPUT_THREE`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/common/registry/ModRecipePropertySets.java:14)
- [ ] `ModRecipePropertySets.COOKING_POT_INPUT_FOUR`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/common/registry/ModRecipePropertySets.java:17)
- [ ] `ModRecipePropertySets.COOKING_POT_INPUT_FIVE`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/common/registry/ModRecipePropertySets.java:20)
- [ ] `ModRecipePropertySets.COOKING_POT_INPUT_SIX`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/common/registry/ModRecipePropertySets.java:23)
- [ ] `ModRecipePropertySets.COOKING_POT_CONTAINER`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/common/registry/ModRecipePropertySets.java:26)
- [ ] `ModRecipePropertySets.CUTTING_BOARD_INPUT`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/common/registry/ModRecipePropertySets.java:29)
- [ ] `ModRecipePropertySets.CUTTING_BOARD_TOOL`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/common/registry/ModRecipePropertySets.java:32)

### `ModRecipeSerializers`

- [ ] `ModRecipeSerializers.COOKING`: it uses Minecraft's `RecipeSerializer` (vectorwing/farmersdelight/common/registry/ModRecipeSerializers.java:12)
- [ ] `ModRecipeSerializers.CUTTING`: it uses Minecraft's `RecipeSerializer` (vectorwing/farmersdelight/common/registry/ModRecipeSerializers.java:15)
- [ ] `ModRecipeSerializers.FOOD_SERVING`: it uses Minecraft's `RecipeSerializer` (vectorwing/farmersdelight/common/registry/ModRecipeSerializers.java:18)
- [ ] `ModRecipeSerializers.DOUGH`: it uses Minecraft's `RecipeSerializer` (vectorwing/farmersdelight/common/registry/ModRecipeSerializers.java:21)
- [x] `ModRecipeSerializers.touch(0)` (vectorwing/farmersdelight/common/registry/ModRecipeSerializers.java:23)

### `ModRecipeTypes`

- [ ] `ModRecipeTypes.COOKING`: it uses Minecraft's `RecipeType` (vectorwing/farmersdelight/common/registry/ModRecipeTypes.java:11)
- [ ] `ModRecipeTypes.CUTTING`: it uses Minecraft's `RecipeType` (vectorwing/farmersdelight/common/registry/ModRecipeTypes.java:12)
- [ ] `ModRecipeTypes.registerRecipeType(1)`: its body couldn't be read: an anonymous class (vectorwing/farmersdelight/common/registry/ModRecipeTypes.java:14)
- [x] `ModRecipeTypes.touch(0)` (vectorwing/farmersdelight/common/registry/ModRecipeTypes.java:23)

### `ModSounds`

- [ ] `ModSounds.BLOCK_STOVE_CRACKLE`: it uses Minecraft's `SoundEvent` (vectorwing/farmersdelight/common/registry/ModSounds.java:9)
- [ ] `ModSounds.BLOCK_COOKING_POT_BOIL`: it uses Minecraft's `SoundEvent` (vectorwing/farmersdelight/common/registry/ModSounds.java:12)
- [ ] `ModSounds.BLOCK_COOKING_POT_BOIL_SOUP`: it uses Minecraft's `SoundEvent` (vectorwing/farmersdelight/common/registry/ModSounds.java:15)
- [ ] `ModSounds.BLOCK_CUTTING_BOARD_PLACE`: it uses Minecraft's `SoundEvent` (vectorwing/farmersdelight/common/registry/ModSounds.java:19)
- [ ] `ModSounds.BLOCK_CUTTING_BOARD_REMOVE`: it uses Minecraft's `SoundEvent` (vectorwing/farmersdelight/common/registry/ModSounds.java:23)
- [ ] `ModSounds.BLOCK_CUTTING_BOARD_CARVE`: it uses Minecraft's `SoundEvent` (vectorwing/farmersdelight/common/registry/ModSounds.java:27)
- [ ] `ModSounds.BLOCK_CUTTING_BOARD_KNIFE`: it uses Minecraft's `SoundEvent` (vectorwing/farmersdelight/common/registry/ModSounds.java:31)
- [ ] `ModSounds.BLOCK_ROPE_FENCE_GATE_OPEN`: it uses Minecraft's `SoundEvent` (vectorwing/farmersdelight/common/registry/ModSounds.java:35)
- [ ] `ModSounds.BLOCK_ROPE_FENCE_GATE_CLOSE`: it uses Minecraft's `SoundEvent` (vectorwing/farmersdelight/common/registry/ModSounds.java:38)
- [ ] `ModSounds.BLOCK_CABINET_OPEN`: it uses Minecraft's `SoundEvent` (vectorwing/farmersdelight/common/registry/ModSounds.java:42)
- [ ] `ModSounds.BLOCK_CABINET_CLOSE`: it uses Minecraft's `SoundEvent` (vectorwing/farmersdelight/common/registry/ModSounds.java:45)
- [ ] `ModSounds.BLOCK_SKILLET_SIZZLE`: it uses Minecraft's `SoundEvent` (vectorwing/farmersdelight/common/registry/ModSounds.java:48)
- [ ] `ModSounds.BLOCK_SKILLET_ADD_FOOD`: it uses Minecraft's `SoundEvent` (vectorwing/farmersdelight/common/registry/ModSounds.java:51)
- [ ] `ModSounds.ITEM_SKILLET_ATTACK_STRONG`: it uses Minecraft's `SoundEvent` (vectorwing/farmersdelight/common/registry/ModSounds.java:54)
- [ ] `ModSounds.ITEM_SKILLET_ATTACK_WEAK`: it uses Minecraft's `SoundEvent` (vectorwing/farmersdelight/common/registry/ModSounds.java:57)
- [ ] `ModSounds.BLOCK_TOMATOES_PICK_TOMATOES`: it uses Minecraft's `SoundEvent` (vectorwing/farmersdelight/common/registry/ModSounds.java:60)
- [ ] `ModSounds.BLOCK_FOOD_TAKE_PORTION`: it uses Minecraft's `SoundEvent` (vectorwing/farmersdelight/common/registry/ModSounds.java:64)
- [ ] `ModSounds.BLOCK_FOOD_SLICE`: it uses Minecraft's `SoundEvent` (vectorwing/farmersdelight/common/registry/ModSounds.java:67)
- [ ] `ModSounds.ENTITY_ROTTEN_TOMATO_THROW`: it uses Minecraft's `SoundEvent` (vectorwing/farmersdelight/common/registry/ModSounds.java:70)
- [ ] `ModSounds.ENTITY_ROTTEN_TOMATO_HIT`: it uses Minecraft's `SoundEvent` (vectorwing/farmersdelight/common/registry/ModSounds.java:73)
- [x] `ModSounds.touch(0)` (vectorwing/farmersdelight/common/registry/ModSounds.java:77)

### `ModToolMaterials`

- [ ] `ModToolMaterials.FLINT`: it uses Minecraft's `ToolMaterial` (vectorwing/farmersdelight/common/registry/ModToolMaterials.java:8)

### `RegistryAliases`

- [ ] `RegistryAliases.addRegistryAliases(0)`: it calls RegistryAliases.addBlockAlias, which isn't translated (vectorwing/farmersdelight/common/registry/RegistryAliases.java:8)
- [ ] `RegistryAliases.addBlockEntityAlias(2)`: it uses Minecraft's `BuiltInRegistries`.BLOCK_ENTITY_TYPE (vectorwing/farmersdelight/common/registry/RegistryAliases.java:15)
- [ ] `RegistryAliases.addBlockAlias(2)`: it uses Minecraft's `BuiltInRegistries`.BLOCK (vectorwing/farmersdelight/common/registry/RegistryAliases.java:19)
- [ ] `RegistryAliases.addItemAlias(2)`: it uses Minecraft's `BuiltInRegistries`.ITEM (vectorwing/farmersdelight/common/registry/RegistryAliases.java:23)

### `CommonTags`

- [ ] `CommonTags.commonBlockTag(1)`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/CommonTags.java:10)
- [ ] `CommonTags.commonItemTag(1)`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/CommonTags.java:14)

### `CommonTags.Blocks`

- [ ] `CommonTags.Blocks.MINEABLE_WITH_KNIFE`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/CommonTags.java:19)
- [ ] `CommonTags.Blocks.STORAGE_BLOCKS_CARROT`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/CommonTags.java:20)
- [ ] `CommonTags.Blocks.STORAGE_BLOCKS_POTATO`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/CommonTags.java:21)
- [ ] `CommonTags.Blocks.STORAGE_BLOCKS_BEETROOT`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/CommonTags.java:22)
- [ ] `CommonTags.Blocks.STORAGE_BLOCKS_CABBAGE`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/CommonTags.java:23)
- [ ] `CommonTags.Blocks.STORAGE_BLOCKS_TOMATO`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/CommonTags.java:24)
- [ ] `CommonTags.Blocks.STORAGE_BLOCKS_ONION`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/CommonTags.java:25)
- [ ] `CommonTags.Blocks.STORAGE_BLOCKS_RICE`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/CommonTags.java:26)
- [ ] `CommonTags.Blocks.STORAGE_BLOCKS_RICE_PANICLE`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/CommonTags.java:27)
- [ ] `CommonTags.Blocks.STORAGE_BLOCKS_STRAW`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/CommonTags.java:28)

### `CommonTags.Items`

- [ ] `CommonTags.Items.CROPS_CABBAGE`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/CommonTags.java:32)
- [ ] `CommonTags.Items.CROPS_TOMATO`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/CommonTags.java:33)
- [ ] `CommonTags.Items.CROPS_ONION`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/CommonTags.java:34)
- [ ] `CommonTags.Items.CROPS_RICE`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/CommonTags.java:35)
- [ ] `CommonTags.Items.CROPS_GRAIN`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/CommonTags.java:36)
- [ ] `CommonTags.Items.FOODS_CABBAGE`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/CommonTags.java:37)
- [ ] `CommonTags.Items.FOODS_TOMATO`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/CommonTags.java:38)
- [ ] `CommonTags.Items.FOODS_ONION`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/CommonTags.java:39)
- [ ] `CommonTags.Items.FOODS_LEAFY_GREEN`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/CommonTags.java:40)
- [ ] `CommonTags.Items.FOODS_DOUGH`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/CommonTags.java:41)
- [ ] `CommonTags.Items.FOODS_DOUGH_WHEAT`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/CommonTags.java:42)
- [ ] `CommonTags.Items.FOODS_PASTA`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/CommonTags.java:43)
- [ ] `CommonTags.Items.FOODS_RAW_BACON`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/CommonTags.java:44)
- [ ] `CommonTags.Items.FOODS_RAW_BEEF`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/CommonTags.java:45)
- [ ] `CommonTags.Items.FOODS_RAW_CHICKEN`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/CommonTags.java:46)
- [ ] `CommonTags.Items.FOODS_RAW_PORK`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/CommonTags.java:47)
- [ ] `CommonTags.Items.FOODS_RAW_MUTTON`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/CommonTags.java:48)
- [ ] `CommonTags.Items.FOODS_SAFE_RAW_FISH`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/CommonTags.java:49)
- [ ] `CommonTags.Items.FOODS_RAW_COD`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/CommonTags.java:50)
- [ ] `CommonTags.Items.FOODS_RAW_SALMON`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/CommonTags.java:51)
- [ ] `CommonTags.Items.FOODS_COOKED_BACON`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/CommonTags.java:52)
- [ ] `CommonTags.Items.FOODS_COOKED_BEEF`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/CommonTags.java:53)
- [ ] `CommonTags.Items.FOODS_COOKED_CHICKEN`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/CommonTags.java:54)
- [ ] `CommonTags.Items.FOODS_COOKED_PORK`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/CommonTags.java:55)
- [ ] `CommonTags.Items.FOODS_COOKED_MUTTON`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/CommonTags.java:56)
- [ ] `CommonTags.Items.FOODS_COOKED_EGG`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/CommonTags.java:57)
- [ ] `CommonTags.Items.FOODS_COOKED_COD`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/CommonTags.java:58)
- [ ] `CommonTags.Items.FOODS_COOKED_SALMON`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/CommonTags.java:59)
- [ ] `CommonTags.Items.TOOLS_KNIFE`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/CommonTags.java:60)
- [ ] `CommonTags.Items.STORAGE_BLOCKS_CARROT`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/CommonTags.java:61)
- [ ] `CommonTags.Items.STORAGE_BLOCKS_POTATO`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/CommonTags.java:62)
- [ ] `CommonTags.Items.STORAGE_BLOCKS_BEETROOT`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/CommonTags.java:63)
- [ ] `CommonTags.Items.STORAGE_BLOCKS_CABBAGE`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/CommonTags.java:64)
- [ ] `CommonTags.Items.STORAGE_BLOCKS_TOMATO`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/CommonTags.java:65)
- [ ] `CommonTags.Items.STORAGE_BLOCKS_ONION`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/CommonTags.java:66)
- [ ] `CommonTags.Items.STORAGE_BLOCKS_RICE`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/CommonTags.java:67)
- [ ] `CommonTags.Items.STORAGE_BLOCKS_RICE_PANICLE`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/CommonTags.java:68)
- [ ] `CommonTags.Items.STORAGE_BLOCKS_STRAW`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/CommonTags.java:69)

### `CompatibilityTags`

- [x] `CompatibilityTags.CREATE` (vectorwing/farmersdelight/common/tag/CompatibilityTags.java:10)
- [ ] `CompatibilityTags.CREATE_BRITTLE`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/CompatibilityTags.java:11)
- [ ] `CompatibilityTags.CREATE_FAN_TRANSPARENT`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/CompatibilityTags.java:12)
- [ ] `CompatibilityTags.CREATE_PASSIVE_BOILER_HEATERS`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/CompatibilityTags.java:13)
- [ ] `CompatibilityTags.CREATE_UPRIGHT_ON_BELT`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/CompatibilityTags.java:14)
- [x] `CompatibilityTags.CREATE_CA` (vectorwing/farmersdelight/common/tag/CompatibilityTags.java:15)
- [ ] `CompatibilityTags.CREATE_CA_PLANT_FOODS`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/CompatibilityTags.java:16)
- [ ] `CompatibilityTags.CREATE_CA_PLANTS`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/CompatibilityTags.java:17)
- [x] `CompatibilityTags.SABLE` (vectorwing/farmersdelight/common/tag/CompatibilityTags.java:18)
- [ ] `CompatibilityTags.SABLE_SUPER_LIGHT`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/CompatibilityTags.java:19)
- [ ] `CompatibilityTags.SABLE_LIGHT`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/CompatibilityTags.java:20)
- [x] `CompatibilityTags.ORIGINS` (vectorwing/farmersdelight/common/tag/CompatibilityTags.java:21)
- [ ] `CompatibilityTags.ORIGINS_MEAT`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/CompatibilityTags.java:22)
- [x] `CompatibilityTags.SERENE_SEASONS` (vectorwing/farmersdelight/common/tag/CompatibilityTags.java:23)
- [ ] `CompatibilityTags.SERENE_SEASONS_AUTUMN_CROPS_BLOCK`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/CompatibilityTags.java:24)
- [ ] `CompatibilityTags.SERENE_SEASONS_SPRING_CROPS_BLOCK`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/CompatibilityTags.java:25)
- [ ] `CompatibilityTags.SERENE_SEASONS_SUMMER_CROPS_BLOCK`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/CompatibilityTags.java:26)
- [ ] `CompatibilityTags.SERENE_SEASONS_WINTER_CROPS_BLOCK`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/CompatibilityTags.java:27)
- [ ] `CompatibilityTags.SERENE_SEASONS_UNBREAKABLE_FERTILE_CROPS`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/CompatibilityTags.java:28)
- [ ] `CompatibilityTags.SERENE_SEASONS_AUTUMN_CROPS`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/CompatibilityTags.java:29)
- [ ] `CompatibilityTags.SERENE_SEASONS_SPRING_CROPS`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/CompatibilityTags.java:30)
- [ ] `CompatibilityTags.SERENE_SEASONS_SUMMER_CROPS`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/CompatibilityTags.java:31)
- [ ] `CompatibilityTags.SERENE_SEASONS_WINTER_CROPS`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/CompatibilityTags.java:32)
- [x] `CompatibilityTags.TINKERS_CONSTRUCT` (vectorwing/farmersdelight/common/tag/CompatibilityTags.java:33)
- [ ] `CompatibilityTags.TINKERS_CONSTRUCT_SEEDS`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/CompatibilityTags.java:34)
- [ ] `CompatibilityTags.externalItemTag(2)`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/CompatibilityTags.java:36)
- [ ] `CompatibilityTags.externalBlockTag(2)`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/CompatibilityTags.java:40)

### `ConventionalTags`

- [ ] `ConventionalTags.MINEABLE_WITH_KNIFE`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/ConventionalTags.java:10)
- [ ] `ConventionalTags.CROPS`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/ConventionalTags.java:11)
- [ ] `ConventionalTags.CROPS_CABBAGE`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/ConventionalTags.java:12)
- [ ] `ConventionalTags.CROPS_ONION`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/ConventionalTags.java:13)
- [ ] `ConventionalTags.CROPS_RICE`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/ConventionalTags.java:14)
- [ ] `ConventionalTags.CROPS_TOMATO`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/ConventionalTags.java:15)
- [ ] `ConventionalTags.FOODS_BERRIES`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/ConventionalTags.java:16)
- [ ] `ConventionalTags.FOODS_BREADS`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/ConventionalTags.java:17)
- [ ] `ConventionalTags.FOODS_BREADS_WHEAT`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/ConventionalTags.java:18)
- [ ] `ConventionalTags.FOODS_COOKED_FISHES`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/ConventionalTags.java:19)
- [ ] `ConventionalTags.FOODS_COOKED_FISHES_COD`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/ConventionalTags.java:20)
- [ ] `ConventionalTags.FOODS_COOKED_FISHES_SALMON`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/ConventionalTags.java:21)
- [ ] `ConventionalTags.FOODS_COOKED_MEATS`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/ConventionalTags.java:22)
- [ ] `ConventionalTags.FOODS_COOKED_MEATS_COOKED_BACON`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/ConventionalTags.java:23)
- [ ] `ConventionalTags.FOODS_COOKED_MEATS_COOKED_BEEF`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/ConventionalTags.java:24)
- [ ] `ConventionalTags.FOODS_COOKED_MEATS_COOKED_CHICKEN`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/ConventionalTags.java:25)
- [ ] `ConventionalTags.FOODS_COOKED_MEATS_COOKED_PORK`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/ConventionalTags.java:26)
- [ ] `ConventionalTags.FOODS_COOKED_MEATS_COOKED_MUTTON`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/ConventionalTags.java:27)
- [ ] `ConventionalTags.FOODS_COOKED_MEATS_COOKED_EGGS`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/ConventionalTags.java:28)
- [ ] `ConventionalTags.FOODS_DOUGHS`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/ConventionalTags.java:29)
- [ ] `ConventionalTags.FOODS_DOUGHS_WHEAT`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/ConventionalTags.java:30)
- [ ] `ConventionalTags.FOODS_PASTAS`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/ConventionalTags.java:31)
- [ ] `ConventionalTags.FOODS_PASTA_RAW_PASTAS`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/ConventionalTags.java:32)
- [ ] `ConventionalTags.FOODS_RAW_FISHES`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/ConventionalTags.java:33)
- [ ] `ConventionalTags.FOODS_RAW_FISHES_COD`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/ConventionalTags.java:34)
- [ ] `ConventionalTags.FOODS_RAW_FISHES_SALMON`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/ConventionalTags.java:35)
- [ ] `ConventionalTags.FOODS_RAW_FISHES_TROPICAL`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/ConventionalTags.java:36)
- [ ] `ConventionalTags.FOODS_RAW_MEATS`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/ConventionalTags.java:37)
- [ ] `ConventionalTags.FOODS_RAW_MEATS_RAW_BACON`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/ConventionalTags.java:38)
- [ ] `ConventionalTags.FOODS_RAW_MEATS_RAW_BEEF`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/ConventionalTags.java:39)
- [ ] `ConventionalTags.FOODS_RAW_MEATS_RAW_CHICKEN`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/ConventionalTags.java:40)
- [ ] `ConventionalTags.FOODS_RAW_MEATS_RAW_PORK`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/ConventionalTags.java:41)
- [ ] `ConventionalTags.FOODS_RAW_MEATS_RAW_MUTTON`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/ConventionalTags.java:42)
- [ ] `ConventionalTags.FOODS_VEGETABLES`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/ConventionalTags.java:43)
- [ ] `ConventionalTags.FOODS_VEGETABLES_BEETROOT`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/ConventionalTags.java:44)
- [ ] `ConventionalTags.FOODS_VEGETABLES_CARROT`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/ConventionalTags.java:45)
- [ ] `ConventionalTags.FOODS_VEGETABLES_ONION`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/ConventionalTags.java:46)
- [ ] `ConventionalTags.FOODS_VEGETABLES_POTATO`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/ConventionalTags.java:47)
- [ ] `ConventionalTags.FOODS_VEGETABLES_TOMATO`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/ConventionalTags.java:48)
- [ ] `ConventionalTags.GRAINS`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/ConventionalTags.java:49)
- [ ] `ConventionalTags.GRAINS_WHEATS`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/ConventionalTags.java:50)
- [ ] `ConventionalTags.GRAINS_RICES`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/ConventionalTags.java:51)
- [ ] `ConventionalTags.MILKS`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/ConventionalTags.java:52)
- [ ] `ConventionalTags.MILK_BUCKET`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/ConventionalTags.java:53)
- [ ] `ConventionalTags.MILK_BOTTLE`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/ConventionalTags.java:54)
- [ ] `ConventionalTags.SALAD_INGREDIENTS`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/ConventionalTags.java:55)
- [ ] `ConventionalTags.SALAD_INGREDIENTS_CABBAGE`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/ConventionalTags.java:56)
- [ ] `ConventionalTags.SEEDS`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/ConventionalTags.java:57)
- [ ] `ConventionalTags.SEEDS_CABBAGE`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/ConventionalTags.java:58)
- [ ] `ConventionalTags.SEEDS_RICE`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/ConventionalTags.java:59)
- [ ] `ConventionalTags.SEEDS_TOMATO`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/ConventionalTags.java:60)
- [ ] `ConventionalTags.TOOLS_KNIVES`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/ConventionalTags.java:61)
- [ ] `ConventionalTags.DRINKS_MILK`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/ConventionalTags.java:62)
- [ ] `ConventionalTags.cBlockTag(1)`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/ConventionalTags.java:64)
- [ ] `ConventionalTags.cItemTag(1)`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/ConventionalTags.java:68)

### `ModTags`

- [ ] `ModTags.CAMPFIRE_SIGNAL_SMOKE`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/ModTags.java:15)
- [ ] `ModTags.COMPOST_ACTIVATORS`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/ModTags.java:17)
- [ ] `ModTags.DROPS_CAKE_SLICE`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/ModTags.java:19)
- [ ] `ModTags.HEAT_CONDUCTORS`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/ModTags.java:21)
- [ ] `ModTags.HEAT_SOURCES`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/ModTags.java:23)
- [ ] `ModTags.TRAY_HEAT_SOURCES`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/ModTags.java:25)
- [ ] `ModTags.MUSHROOM_COLONY_GROWABLE_ON`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/ModTags.java:27)
- [ ] `ModTags.MINEABLE_WITH_KNIFE`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/ModTags.java:29)
- [ ] `ModTags.TERRAIN`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/ModTags.java:31)
- [ ] `ModTags.STRAW_BLOCKS`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/ModTags.java:33)
- [ ] `ModTags.WILD_CROPS`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/ModTags.java:35)
- [ ] `ModTags.CABINETS`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/ModTags.java:37)
- [ ] `ModTags.WOODEN_CABINETS`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/ModTags.java:39)
- [ ] `ModTags.MUSHROOM_COLONIES`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/ModTags.java:41)
- [ ] `ModTags.ROPES`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/ModTags.java:43)
- [ ] `ModTags.UNAFFECTED_BY_RICH_SOIL`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/ModTags.java:45)
- [ ] `ModTags.MEALS`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/ModTags.java:47)
- [ ] `ModTags.DRINKS`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/ModTags.java:49)
- [ ] `ModTags.FEASTS`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/ModTags.java:51)
- [ ] `ModTags.WILD_CROPS_ITEM`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/ModTags.java:53)
- [ ] `ModTags.STRAW_HARVESTERS`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/ModTags.java:55)
- [ ] `ModTags.KNIVES`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/ModTags.java:57)
- [ ] `ModTags.CANVAS_SIGNS`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/ModTags.java:59)
- [ ] `ModTags.HANGING_CANVAS_SIGNS`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/ModTags.java:61)
- [ ] `ModTags.WOODEN_CABINET_ITEMS`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/ModTags.java:63)
- [ ] `ModTags.CABINET_ITEMS`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/ModTags.java:65)
- [ ] `ModTags.MUSHROOM_COLONY_ITEMS`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/ModTags.java:67)
- [ ] `ModTags.SERVING_CONTAINERS`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/ModTags.java:69)
- [ ] `ModTags.FLAT_ON_CUTTING_BOARD`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/ModTags.java:71)
- [ ] `ModTags.DOG_FOOD_USERS`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/ModTags.java:73)
- [ ] `ModTags.HORSE_FEED_USERS`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/ModTags.java:75)
- [ ] `ModTags.HORSE_FEED_TEMPTED`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/ModTags.java:77)
- [ ] `ModTags.HAS_BROWN_MUSHROOM_COLONY`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/ModTags.java:79)
- [ ] `ModTags.HAS_RED_MUSHROOM_COLONY`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/ModTags.java:81)
- [ ] `ModTags.HAS_WILD_CABBAGE`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/ModTags.java:83)
- [ ] `ModTags.HAS_WILD_BEETROOTS`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/ModTags.java:85)
- [ ] `ModTags.WILD_CARROTS_WHITELIST`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/ModTags.java:87)
- [ ] `ModTags.WILD_CARROTS_BLACKLIST`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/ModTags.java:89)
- [ ] `ModTags.WILD_ONIONS_WHITELIST`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/ModTags.java:91)
- [ ] `ModTags.WILD_ONIONS_BLACKLIST`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/ModTags.java:93)
- [ ] `ModTags.WILD_POTATOES_WHITELIST`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/ModTags.java:95)
- [ ] `ModTags.WILD_POTATOES_BLACKLIST`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/ModTags.java:97)
- [ ] `ModTags.WILD_RICE_WHITELIST`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/ModTags.java:99)
- [ ] `ModTags.WILD_RICE_BLACKLIST`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/ModTags.java:101)
- [ ] `ModTags.WILD_TOMATOES_WHITELIST`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/ModTags.java:103)
- [ ] `ModTags.WILD_TOMATOES_BLACKLIST`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/ModTags.java:105)
- [ ] `ModTags.DROPS_LEATHER`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/ModTags.java:107)
- [ ] `ModTags.HOT_COCOA_IGNORED`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/ModTags.java:109)
- [ ] `ModTags.MILK_BOTTLE_IGNORED`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/ModTags.java:111)
- [ ] `ModTags.modItemTag(1)`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/ModTags.java:113)
- [ ] `ModTags.modBlockTag(1)`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/ModTags.java:117)
- [ ] `ModTags.modEntityTag(1)`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/ModTags.java:121)
- [ ] `ModTags.modBiomeTag(1)`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/ModTags.java:125)
- [ ] `ModTags.modEffectTag(1)`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/ModTags.java:129)

### `ModTags.Blocks`

- [ ] `ModTags.Blocks.CAMPFIRE_SIGNAL_SMOKE`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/ModTags.java:134)
- [ ] `ModTags.Blocks.COMPOST_ACTIVATORS`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/ModTags.java:135)
- [ ] `ModTags.Blocks.DROPS_CAKE_SLICE`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/ModTags.java:136)
- [ ] `ModTags.Blocks.HEAT_CONDUCTORS`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/ModTags.java:137)
- [ ] `ModTags.Blocks.HEAT_SOURCES`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/ModTags.java:138)
- [ ] `ModTags.Blocks.TRAY_HEAT_SOURCES`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/ModTags.java:139)
- [ ] `ModTags.Blocks.MUSHROOM_COLONY_GROWABLE_ON`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/ModTags.java:141)
- [ ] `ModTags.Blocks.PLANTED_FROM_BELOW`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/ModTags.java:142)
- [ ] `ModTags.Blocks.FEASTS`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/ModTags.java:143)
- [ ] `ModTags.Blocks.PIES`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/ModTags.java:144)
- [ ] `ModTags.Blocks.STRAW_BLOCKS`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/ModTags.java:145)
- [ ] `ModTags.Blocks.TERRAIN`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/ModTags.java:146)
- [ ] `ModTags.Blocks.UNAFFECTED_BY_RICH_SOIL`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/ModTags.java:147)
- [ ] `ModTags.Blocks.CABINETS`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/ModTags.java:148)
- [ ] `ModTags.Blocks.CABINETS_WOODEN`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/ModTags.java:149)
- [ ] `ModTags.Blocks.MUSHROOM_COLONIES`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/ModTags.java:150)
- [ ] `ModTags.Blocks.MINEABLE_WITH_KNIFE`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/ModTags.java:151)
- [ ] `ModTags.Blocks.ROPES`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/ModTags.java:152)
- [ ] `ModTags.Blocks.WILD_CROPS`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/ModTags.java:153)

### `ModTags.EntityTypes`

- [ ] `ModTags.EntityTypes.DOG_FOOD_USERS`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/ModTags.java:157)
- [ ] `ModTags.EntityTypes.HORSE_FEED_USERS`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/ModTags.java:158)
- [ ] `ModTags.EntityTypes.HORSE_FEED_TEMPTED`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/ModTags.java:159)

### `ModTags.Items`

- [ ] `ModTags.Items.KNIFE_ENCHANTABLE`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/ModTags.java:163)
- [ ] `ModTags.Items.SNACKS`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/ModTags.java:164)
- [ ] `ModTags.Items.MEALS`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/ModTags.java:165)
- [ ] `ModTags.Items.DRINKS`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/ModTags.java:166)
- [ ] `ModTags.Items.SWEETS`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/ModTags.java:167)
- [ ] `ModTags.Items.FEASTS`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/ModTags.java:168)
- [ ] `ModTags.Items.PIES`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/ModTags.java:169)
- [ ] `ModTags.Items.FLAT_ON_CUTTING_BOARD`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/ModTags.java:170)
- [ ] `ModTags.Items.SERVING_CONTAINERS`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/ModTags.java:171)
- [ ] `ModTags.Items.STRAW_HARVESTERS`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/ModTags.java:172)
- [ ] `ModTags.Items.CABINETS`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/ModTags.java:173)
- [ ] `ModTags.Items.CABINETS_WOODEN`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/ModTags.java:174)
- [ ] `ModTags.Items.CANVAS_SIGNS`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/ModTags.java:175)
- [ ] `ModTags.Items.HANGING_CANVAS_SIGNS`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/ModTags.java:176)
- [ ] `ModTags.Items.KNIVES`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/ModTags.java:177)
- [ ] `ModTags.Items.MUSHROOM_COLONIES`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/ModTags.java:178)
- [ ] `ModTags.Items.WILD_CROPS`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/common/tag/ModTags.java:179)

### `ClientRenderUtils`

- [x] `ClientRenderUtils.isCursorInsideBounds(6)` (vectorwing/farmersdelight/common/utility/ClientRenderUtils.java:7)
- [x] `ClientRenderUtils.getClientPlayerHack(0)` in game_side.rs (vectorwing/farmersdelight/common/utility/ClientRenderUtils.java:11)

### `ItemUtils`

- [ ] `ItemUtils.isValidTool(3)`: it uses ItemAbility, which isn't translated (it implements Minecraft's `StringRepresentable`) (vectorwing/farmersdelight/common/utility/ItemUtils.java:27)
- [ ] `ItemUtils.isKnife(1)`: it calls ItemUtils.isValidTool, which isn't translated (vectorwing/farmersdelight/common/utility/ItemUtils.java:31)
- [ ] `ItemUtils.dropItems(3)`: it uses Minecraft's `Level` (vectorwing/farmersdelight/common/utility/ItemUtils.java:35)
- [ ] `ItemUtils.clearItems(1)`: it uses ItemStackHandler, which isn't translated (it extends `FabricWrappedInventory`) (vectorwing/farmersdelight/common/utility/ItemUtils.java:41)
- [ ] `ItemUtils.doesInventoryHaveItems(1)`: it uses ItemHandler, which isn't translated (it's an interface) (vectorwing/farmersdelight/common/utility/ItemUtils.java:47)
- [ ] `ItemUtils.spawnItemEntity(8)`: it uses Minecraft's `Level` (vectorwing/farmersdelight/common/utility/ItemUtils.java:57)
- [ ] `ItemUtils.getValidatedEnchantmentLevel(3)`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/common/utility/ItemUtils.java:63)
- [ ] `ItemUtils.getEnchantmentLevel(2)`: it uses Minecraft's `DataComponentGetter` (vectorwing/farmersdelight/common/utility/ItemUtils.java:68)

### `ListUtils`

- [ ] `ListUtils.mapArrayIndexSet(3)`: it's generic (vectorwing/farmersdelight/common/utility/ListUtils.java:7)

### `MathUtils`

- [ ] `MathUtils.RAND`: it creates `Random` (java.util) (vectorwing/farmersdelight/common/utility/MathUtils.java:10)
- [ ] `MathUtils.calcRedstoneFromItemHandler(1)`: it uses ItemStackHandler, which isn't translated (it extends `FabricWrappedInventory`) (vectorwing/farmersdelight/common/utility/MathUtils.java:12)

### `RecipeUtils`

- [x] `RecipeUtils.FDLocation(1)` (vectorwing/farmersdelight/common/utility/RecipeUtils.java:6)

### `ShapeUtils`

- [ ] `ShapeUtils.PLATED_SHAPE_CACHE`: it uses Minecraft's `VoxelShape` (vectorwing/farmersdelight/common/utility/ShapeUtils.java:18)
- [ ] `ShapeUtils.ROTATED_SHAPE_CACHE`: it uses Minecraft's `VoxelShape` (vectorwing/farmersdelight/common/utility/ShapeUtils.java:19)
- [ ] `ShapeUtils.rotateY(2)`: it uses Minecraft's `VoxelShape` (vectorwing/farmersdelight/common/utility/ShapeUtils.java:21)
- [ ] `ShapeUtils.blockBox(6)`: it uses Minecraft's `VoxelShape` (vectorwing/farmersdelight/common/utility/ShapeUtils.java:62)
- [ ] `ShapeUtils.getShapesRotatedFromNorth(1)`: it uses Minecraft's `Direction` (vectorwing/farmersdelight/common/utility/ShapeUtils.java:66)
- [ ] `ShapeUtils.buildPlatedFoodShapes(2)`: it uses Minecraft's `VoxelShape` (vectorwing/farmersdelight/common/utility/ShapeUtils.java:75)
- [ ] `ShapeUtils.buildRotatedFoodShapes(1)`: it uses Minecraft's `VoxelShape` (vectorwing/farmersdelight/common/utility/ShapeUtils.java:95)
- [ ] `ShapeUtils.mergeShapes(1)`: it uses Minecraft's `VoxelShape` (vectorwing/farmersdelight/common/utility/ShapeUtils.java:111)

### `ShapeUtils.RotationAmount`


### `TextUtils`

- [ ] `TextUtils.PLACEABLE`: it uses Minecraft's `MutableComponent` (vectorwing/farmersdelight/common/utility/TextUtils.java:26)
- [ ] `TextUtils.PLACEABLE_SNEAKING`: it uses Minecraft's `MutableComponent` (vectorwing/farmersdelight/common/utility/TextUtils.java:27)
- [ ] `TextUtils.DEBUG_ITEM`: it uses Minecraft's `MutableComponent` (vectorwing/farmersdelight/common/utility/TextUtils.java:28)
- [ ] `TextUtils.getTranslation(2)`: it uses `Object` (vectorwing/farmersdelight/common/utility/TextUtils.java:30)
- [ ] `TextUtils.getTextWithType(3)`: it uses `Object` (vectorwing/farmersdelight/common/utility/TextUtils.java:34)
- [ ] `TextUtils.block(2)`: it uses `Object` (vectorwing/farmersdelight/common/utility/TextUtils.java:38)
- [ ] `TextUtils.item(2)`: it uses `Object` (vectorwing/farmersdelight/common/utility/TextUtils.java:42)
- [ ] `TextUtils.advancement(2)`: it uses `Object` (vectorwing/farmersdelight/common/utility/TextUtils.java:46)
- [ ] `TextUtils.container(2)`: it uses `Object` (vectorwing/farmersdelight/common/utility/TextUtils.java:50)
- [ ] `TextUtils.JEI(2)`: it uses `Object` (vectorwing/farmersdelight/common/utility/TextUtils.java:54)
- [ ] `TextUtils.tooltip(2)`: it uses `Object` (vectorwing/farmersdelight/common/utility/TextUtils.java:58)
- [ ] `TextUtils.subtitleKey(2)`: it uses `Object` (vectorwing/farmersdelight/common/utility/TextUtils.java:62)
- [ ] `TextUtils.addFoodEffectTooltip(4)`: it uses Minecraft's `Consumable` (vectorwing/farmersdelight/common/utility/TextUtils.java:66)

### `InOrderFeature`: it implements Minecraft's `Feature`

- [ ] `InOrderFeature.CODEC`: it uses Minecraft's `MapCodec` (vectorwing/farmersdelight/common/world/feature/InOrderFeature.java:15)
- [ ] `InOrderFeature.InOrderFeature(1)`: its class isn't translated (it implements Minecraft's `Feature`) (vectorwing/farmersdelight/common/world/feature/InOrderFeature.java:14)
- [ ] `InOrderFeature.codec(0)`: its class isn't translated (it implements Minecraft's `Feature`) (vectorwing/farmersdelight/common/world/feature/InOrderFeature.java:19)
- [ ] `InOrderFeature.place(4)`: its class isn't translated (it implements Minecraft's `Feature`) (vectorwing/farmersdelight/common/world/feature/InOrderFeature.java:23)
- [ ] `InOrderFeature.features(0)`: its class isn't translated (it implements Minecraft's `Feature`) (vectorwing/farmersdelight/common/world/feature/InOrderFeature.java:14)

### `WildRiceFeature`: it implements Minecraft's `Feature`

- [ ] `WildRiceFeature.CODEC`: it uses Minecraft's `MapCodec` (vectorwing/farmersdelight/common/world/feature/WildRiceFeature.java:17)
- [ ] `WildRiceFeature.codec(0)`: its class isn't translated (it implements Minecraft's `Feature`) (vectorwing/farmersdelight/common/world/feature/WildRiceFeature.java:19)
- [ ] `WildRiceFeature.place(4)`: its class isn't translated (it implements Minecraft's `Feature`) (vectorwing/farmersdelight/common/world/feature/WildRiceFeature.java:23)

### `BiomeTagFilter`: it implements Minecraft's `PlacementFilter`

- [ ] `BiomeTagFilter.CODEC`: it uses Minecraft's `MapCodec` (vectorwing/farmersdelight/common/world/filter/BiomeTagFilter.java:16)
- [ ] `BiomeTagFilter.BiomeTagFilter(1)`: its class isn't translated (it implements Minecraft's `PlacementFilter`) (vectorwing/farmersdelight/common/world/filter/BiomeTagFilter.java:21)
- [ ] `BiomeTagFilter.biomeIsInTag(1)`: it uses BiomeTagFilter, which isn't translated (it implements Minecraft's `PlacementFilter`) (vectorwing/farmersdelight/common/world/filter/BiomeTagFilter.java:25)
- [ ] `BiomeTagFilter.modify(4)`: its class isn't translated (it implements Minecraft's `PlacementFilter`) (vectorwing/farmersdelight/common/world/filter/BiomeTagFilter.java:29)
- [ ] `BiomeTagFilter.shouldPlace(3)`: its class isn't translated (it implements Minecraft's `PlacementFilter`) (vectorwing/farmersdelight/common/world/filter/BiomeTagFilter.java:33)
- [ ] `BiomeTagFilter.codec(0)`: its class isn't translated (it implements Minecraft's `PlacementFilter`) (vectorwing/farmersdelight/common/world/filter/BiomeTagFilter.java:38)

### `AddFeaturesByFilterBiomeModifier`: it uses Minecraft's `HolderSet`

- [ ] `AddFeaturesByFilterBiomeModifier.AddFeaturesByFilterBiomeModifier(6)`: its class isn't translated (it uses Minecraft's `HolderSet`) (vectorwing/farmersdelight/common/world/modifier/AddFeaturesByFilterBiomeModifier.java:10)
- [ ] `AddFeaturesByFilterBiomeModifier.allowedBiomes(0)`: its class isn't translated (it uses Minecraft's `HolderSet`) (vectorwing/farmersdelight/common/world/modifier/AddFeaturesByFilterBiomeModifier.java:10)
- [ ] `AddFeaturesByFilterBiomeModifier.deniedBiomes(0)`: its class isn't translated (it uses Minecraft's `HolderSet`) (vectorwing/farmersdelight/common/world/modifier/AddFeaturesByFilterBiomeModifier.java:10)
- [ ] `AddFeaturesByFilterBiomeModifier.minimumTemperature(0)`: its class isn't translated (it uses Minecraft's `HolderSet`) (vectorwing/farmersdelight/common/world/modifier/AddFeaturesByFilterBiomeModifier.java:10)
- [ ] `AddFeaturesByFilterBiomeModifier.maximumTemperature(0)`: its class isn't translated (it uses Minecraft's `HolderSet`) (vectorwing/farmersdelight/common/world/modifier/AddFeaturesByFilterBiomeModifier.java:10)
- [ ] `AddFeaturesByFilterBiomeModifier.features(0)`: its class isn't translated (it uses Minecraft's `HolderSet`) (vectorwing/farmersdelight/common/world/modifier/AddFeaturesByFilterBiomeModifier.java:10)
- [ ] `AddFeaturesByFilterBiomeModifier.step(0)`: its class isn't translated (it uses Minecraft's `HolderSet`) (vectorwing/farmersdelight/common/world/modifier/AddFeaturesByFilterBiomeModifier.java:10)

### `VillageStructures`

- [ ] `VillageStructures.init(0)`: it uses Fabric's `ServerLifecycleEvents`.SERVER_STARTING (vectorwing/farmersdelight/common/world/VillageStructures.java:28)
- [ ] `VillageStructures.addNewVillageBuilding(1)`: it uses Minecraft's `MinecraftServer` (vectorwing/farmersdelight/common/world/VillageStructures.java:32)
- [ ] `VillageStructures.addBuildingToPool(5)`: it uses Minecraft's `Registry` (vectorwing/farmersdelight/common/world/VillageStructures.java:84)
- [ ] `VillageStructures.addNewRuleToProcessorList(3)`: it uses Minecraft's `StructureProcessor` (vectorwing/farmersdelight/common/world/VillageStructures.java:107)

### `WildCropGeneration`

- [ ] `WildCropGeneration.SANDY_SHRUB`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/common/world/WildCropGeneration.java:11)
- [ ] `WildCropGeneration.WILD_CABBAGES`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/common/world/WildCropGeneration.java:12)
- [ ] `WildCropGeneration.WILD_ONIONS`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/common/world/WildCropGeneration.java:13)
- [ ] `WildCropGeneration.WILD_TOMATOES`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/common/world/WildCropGeneration.java:14)
- [ ] `WildCropGeneration.WILD_CARROTS`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/common/world/WildCropGeneration.java:15)
- [ ] `WildCropGeneration.WILD_POTATOES`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/common/world/WildCropGeneration.java:16)
- [ ] `WildCropGeneration.WILD_BEETROOTS`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/common/world/WildCropGeneration.java:17)
- [ ] `WildCropGeneration.WILD_RICE`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/common/world/WildCropGeneration.java:20)
- [ ] `WildCropGeneration.BROWN_MUSHROOM_COLONIES`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/common/world/WildCropGeneration.java:21)
- [ ] `WildCropGeneration.RED_MUSHROOM_COLONIES`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/common/world/WildCropGeneration.java:24)
- [ ] `WildCropGeneration.PATCH_SANDY_SHRUB`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/common/world/WildCropGeneration.java:27)
- [ ] `WildCropGeneration.PATCH_WILD_CABBAGES`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/common/world/WildCropGeneration.java:30)
- [ ] `WildCropGeneration.PATCH_WILD_ONIONS`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/common/world/WildCropGeneration.java:33)
- [ ] `WildCropGeneration.PATCH_WILD_TOMATOES`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/common/world/WildCropGeneration.java:36)
- [ ] `WildCropGeneration.PATCH_WILD_CARROTS`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/common/world/WildCropGeneration.java:39)
- [ ] `WildCropGeneration.PATCH_WILD_POTATOES`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/common/world/WildCropGeneration.java:42)
- [ ] `WildCropGeneration.PATCH_WILD_BEETROOTS`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/common/world/WildCropGeneration.java:45)
- [ ] `WildCropGeneration.PATCH_WILD_RICE`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/common/world/WildCropGeneration.java:48)
- [ ] `WildCropGeneration.PATCH_BROWN_MUSHROOM_COLONIES`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/common/world/WildCropGeneration.java:51)
- [ ] `WildCropGeneration.PATCH_RED_MUSHROOM_COLONIES`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/common/world/WildCropGeneration.java:54)
- [x] `WildCropGeneration.load(0)` (vectorwing/farmersdelight/common/world/WildCropGeneration.java:58)
- [ ] `WildCropGeneration.bootstrapConfiguredFeatures(1)`: it uses Minecraft's `BootstrapContext` (vectorwing/farmersdelight/common/world/WildCropGeneration.java:61)
- [ ] `WildCropGeneration.bootstrapPlacedFeatures(1)`: it uses Minecraft's `BootstrapContext` (vectorwing/farmersdelight/common/world/WildCropGeneration.java:64)

### `FDAdvancementGenerator`: it extends Fabric's `FabricAdvancementProvider`

- [ ] `FDAdvancementGenerator.FDAdvancementGenerator(2)`: its class isn't translated (it extends Fabric's `FabricAdvancementProvider`) (vectorwing/farmersdelight/data/advancement/FDAdvancementGenerator.java:42)
- [ ] `FDAdvancementGenerator.generateAdvancement(2)`: its class isn't translated (it extends Fabric's `FabricAdvancementProvider`) (vectorwing/farmersdelight/data/advancement/FDAdvancementGenerator.java:46)
- [ ] `FDAdvancementGenerator.placedBlock(1)`: its class isn't translated (it extends Fabric's `FabricAdvancementProvider`) (vectorwing/farmersdelight/data/advancement/FDAdvancementGenerator.java:276)
- [ ] `FDAdvancementGenerator.usedItem(2)`: it uses Minecraft's `Criterion` (vectorwing/farmersdelight/data/advancement/FDAdvancementGenerator.java:280)
- [ ] `FDAdvancementGenerator.getAdvancement(7)`: it uses Minecraft's `Builder` (vectorwing/farmersdelight/data/advancement/FDAdvancementGenerator.java:284)
- [ ] `FDAdvancementGenerator.getNameId(1)`: its class isn't translated (it extends Fabric's `FabricAdvancementProvider`) (vectorwing/farmersdelight/data/advancement/FDAdvancementGenerator.java:294)

### `BlockStates`: it extends Fabric's `FabricModelProvider`

- [x] `BlockStates.DEFAULT_ANGLE_OFFSET` (vectorwing/farmersdelight/data/BlockStates.java:8)
- [ ] `BlockStates.BlockStates(1)`: its class isn't translated (it extends Fabric's `FabricModelProvider`) (vectorwing/farmersdelight/data/BlockStates.java:10)

### `BlockTags`: it extends Fabric's `BlockTagsProvider`

- [ ] `BlockTags.BlockTags(2)`: its class isn't translated (it extends Fabric's `BlockTagsProvider`) (vectorwing/farmersdelight/data/BlockTags.java:27)
- [ ] `BlockTags.addTags(1)`: its class isn't translated (it extends Fabric's `BlockTagsProvider`) (vectorwing/farmersdelight/data/BlockTags.java:31)
- [ ] `BlockTags.registerBlockMineables(0)`: its class isn't translated (it extends Fabric's `BlockTagsProvider`) (vectorwing/farmersdelight/data/BlockTags.java:41)
- [ ] `BlockTags.registerMinecraftTags(0)`: its class isn't translated (it extends Fabric's `BlockTagsProvider`) (vectorwing/farmersdelight/data/BlockTags.java:105)
- [ ] `BlockTags.registerNeoForgeTags(0)`: its class isn't translated (it extends Fabric's `BlockTagsProvider`) (vectorwing/farmersdelight/data/BlockTags.java:169)
- [ ] `BlockTags.registerCommonTags(0)`: its class isn't translated (it extends Fabric's `BlockTagsProvider`) (vectorwing/farmersdelight/data/BlockTags.java:187)
- [ ] `BlockTags.registerModTags(0)`: its class isn't translated (it extends Fabric's `BlockTagsProvider`) (vectorwing/farmersdelight/data/BlockTags.java:200)
- [ ] `BlockTags.registerRefabricatedTags(0)`: its class isn't translated (it extends Fabric's `BlockTagsProvider`) (vectorwing/farmersdelight/data/BlockTags.java:297)
- [ ] `BlockTags.registerCompatibilityTags(0)`: its class isn't translated (it extends Fabric's `BlockTagsProvider`) (vectorwing/farmersdelight/data/BlockTags.java:305)
- [ ] `BlockTags.tagBuilder(1)`: its class isn't translated (it extends Fabric's `BlockTagsProvider`) (vectorwing/farmersdelight/data/BlockTags.java:329)

### `BlockTags.RefabricatedTagBuilder`: it uses Minecraft's `BlockItemTagAppender`

- [ ] `BlockTags.RefabricatedTagBuilder.RefabricatedTagBuilder(1)`: its class isn't translated (it uses Minecraft's `BlockItemTagAppender`) (vectorwing/farmersdelight/data/BlockTags.java:337)
- [ ] `BlockTags.RefabricatedTagBuilder.addOptionalTag(1)`: its class isn't translated (it uses Minecraft's `BlockItemTagAppender`) (vectorwing/farmersdelight/data/BlockTags.java:342)
- [ ] `BlockTags.RefabricatedTagBuilder.add(1)`: its class isn't translated (it uses Minecraft's `BlockItemTagAppender`) (vectorwing/farmersdelight/data/BlockTags.java:347)
- [ ] `BlockTags.RefabricatedTagBuilder.add(1)`: its class isn't translated (it uses Minecraft's `BlockItemTagAppender`) (vectorwing/farmersdelight/data/BlockTags.java:352)
- [ ] `BlockTags.RefabricatedTagBuilder.addColorKey(1)`: its class isn't translated (it uses Minecraft's `BlockItemTagAppender`) (vectorwing/farmersdelight/data/BlockTags.java:358)
- [ ] `BlockTags.RefabricatedTagBuilder.addColorId(1)`: its class isn't translated (it uses Minecraft's `BlockItemTagAppender`) (vectorwing/farmersdelight/data/BlockTags.java:363)
- [ ] `BlockTags.RefabricatedTagBuilder.addOptional(1)`: its class isn't translated (it uses Minecraft's `BlockItemTagAppender`) (vectorwing/farmersdelight/data/BlockTags.java:368)
- [ ] `BlockTags.RefabricatedTagBuilder.addTags(1)`: its class isn't translated (it uses Minecraft's `BlockItemTagAppender`) (vectorwing/farmersdelight/data/BlockTags.java:373)
- [ ] `BlockTags.RefabricatedTagBuilder.addTag(1)`: its class isn't translated (it uses Minecraft's `BlockItemTagAppender`) (vectorwing/farmersdelight/data/BlockTags.java:380)

### `CookingPotRecipeBuilder`: it implements Minecraft's `RecipeBuilder`

- [ ] `CookingPotRecipeBuilder.CookingPotRecipeBuilder(6)`: its class isn't translated (it implements Minecraft's `RecipeBuilder`) (vectorwing/farmersdelight/data/builder/CookingPotRecipeBuilder.java:44)
- [ ] `CookingPotRecipeBuilder.CookingPotRecipeBuilder(5)`: its class isn't translated (it implements Minecraft's `RecipeBuilder`) (vectorwing/farmersdelight/data/builder/CookingPotRecipeBuilder.java:48)
- [ ] `CookingPotRecipeBuilder.cookingPotRecipe(5)`: it uses CookingPotRecipeBuilder, which isn't translated (it implements Minecraft's `RecipeBuilder`) (vectorwing/farmersdelight/data/builder/CookingPotRecipeBuilder.java:58)
- [ ] `CookingPotRecipeBuilder.cookingPotRecipe(6)`: it uses CookingPotRecipeBuilder, which isn't translated (it implements Minecraft's `RecipeBuilder`) (vectorwing/farmersdelight/data/builder/CookingPotRecipeBuilder.java:62)
- [ ] `CookingPotRecipeBuilder.addIngredient(1)`: its class isn't translated (it implements Minecraft's `RecipeBuilder`) (vectorwing/farmersdelight/data/builder/CookingPotRecipeBuilder.java:68)
- [ ] `CookingPotRecipeBuilder.addIngredient(1)`: its class isn't translated (it implements Minecraft's `RecipeBuilder`) (vectorwing/farmersdelight/data/builder/CookingPotRecipeBuilder.java:72)
- [ ] `CookingPotRecipeBuilder.addIngredient(2)`: its class isn't translated (it implements Minecraft's `RecipeBuilder`) (vectorwing/farmersdelight/data/builder/CookingPotRecipeBuilder.java:76)
- [ ] `CookingPotRecipeBuilder.addIngredient(1)`: its class isn't translated (it implements Minecraft's `RecipeBuilder`) (vectorwing/farmersdelight/data/builder/CookingPotRecipeBuilder.java:84)
- [ ] `CookingPotRecipeBuilder.addIngredient(2)`: its class isn't translated (it implements Minecraft's `RecipeBuilder`) (vectorwing/farmersdelight/data/builder/CookingPotRecipeBuilder.java:88)
- [ ] `CookingPotRecipeBuilder.group(1)`: its class isn't translated (it implements Minecraft's `RecipeBuilder`) (vectorwing/farmersdelight/data/builder/CookingPotRecipeBuilder.java:96)
- [ ] `CookingPotRecipeBuilder.defaultId(0)`: its class isn't translated (it implements Minecraft's `RecipeBuilder`) (vectorwing/farmersdelight/data/builder/CookingPotRecipeBuilder.java:100)
- [ ] `CookingPotRecipeBuilder.setRecipeBookCategory(1)`: its class isn't translated (it implements Minecraft's `RecipeBuilder`) (vectorwing/farmersdelight/data/builder/CookingPotRecipeBuilder.java:104)
- [ ] `CookingPotRecipeBuilder.getResult(0)`: its class isn't translated (it implements Minecraft's `RecipeBuilder`) (vectorwing/farmersdelight/data/builder/CookingPotRecipeBuilder.java:109)
- [ ] `CookingPotRecipeBuilder.unlockedBy(2)`: its class isn't translated (it implements Minecraft's `RecipeBuilder`) (vectorwing/farmersdelight/data/builder/CookingPotRecipeBuilder.java:113)
- [ ] `CookingPotRecipeBuilder.unlockedByItems(2)`: its class isn't translated (it implements Minecraft's `RecipeBuilder`) (vectorwing/farmersdelight/data/builder/CookingPotRecipeBuilder.java:118)
- [ ] `CookingPotRecipeBuilder.unlockedByAnyIngredient(1)`: its class isn't translated (it implements Minecraft's `RecipeBuilder`) (vectorwing/farmersdelight/data/builder/CookingPotRecipeBuilder.java:122)
- [ ] `CookingPotRecipeBuilder.setNamespace(1)`: its class isn't translated (it implements Minecraft's `RecipeBuilder`) (vectorwing/farmersdelight/data/builder/CookingPotRecipeBuilder.java:127)
- [ ] `CookingPotRecipeBuilder.getDefaultRecipeId(1)`: it uses Minecraft's `ItemLike` (vectorwing/farmersdelight/data/builder/CookingPotRecipeBuilder.java:132)
- [ ] `CookingPotRecipeBuilder.saveToFD(1)`: its class isn't translated (it implements Minecraft's `RecipeBuilder`) (vectorwing/farmersdelight/data/builder/CookingPotRecipeBuilder.java:136)
- [ ] `CookingPotRecipeBuilder.save(1)`: its class isn't translated (it implements Minecraft's `RecipeBuilder`) (vectorwing/farmersdelight/data/builder/CookingPotRecipeBuilder.java:140)
- [ ] `CookingPotRecipeBuilder.build(2)`: its class isn't translated (it implements Minecraft's `RecipeBuilder`) (vectorwing/farmersdelight/data/builder/CookingPotRecipeBuilder.java:149)
- [ ] `CookingPotRecipeBuilder.build(2)`: its class isn't translated (it implements Minecraft's `RecipeBuilder`) (vectorwing/farmersdelight/data/builder/CookingPotRecipeBuilder.java:158)
- [ ] `CookingPotRecipeBuilder.save(2)`: its class isn't translated (it implements Minecraft's `RecipeBuilder`) (vectorwing/farmersdelight/data/builder/CookingPotRecipeBuilder.java:162)
- [ ] `CookingPotRecipeBuilder.save(2)`: its class isn't translated (it implements Minecraft's `RecipeBuilder`) (vectorwing/farmersdelight/data/builder/CookingPotRecipeBuilder.java:166)

### `CuttingBoardRecipeBuilder`: it implements Minecraft's `RecipeBuilder`

- [ ] `CuttingBoardRecipeBuilder.CuttingBoardRecipeBuilder(5)`: its class isn't translated (it implements Minecraft's `RecipeBuilder`) (vectorwing/farmersdelight/data/builder/CuttingBoardRecipeBuilder.java:31)
- [ ] `CuttingBoardRecipeBuilder.cuttingRecipe(4)`: it uses CuttingBoardRecipeBuilder, which isn't translated (it implements Minecraft's `RecipeBuilder`) (vectorwing/farmersdelight/data/builder/CuttingBoardRecipeBuilder.java:38)
- [ ] `CuttingBoardRecipeBuilder.cuttingRecipe(5)`: it uses CuttingBoardRecipeBuilder, which isn't translated (it implements Minecraft's `RecipeBuilder`) (vectorwing/farmersdelight/data/builder/CuttingBoardRecipeBuilder.java:42)
- [ ] `CuttingBoardRecipeBuilder.cuttingRecipe(3)`: it uses CuttingBoardRecipeBuilder, which isn't translated (it implements Minecraft's `RecipeBuilder`) (vectorwing/farmersdelight/data/builder/CuttingBoardRecipeBuilder.java:46)
- [ ] `CuttingBoardRecipeBuilder.addResult(1)`: its class isn't translated (it implements Minecraft's `RecipeBuilder`) (vectorwing/farmersdelight/data/builder/CuttingBoardRecipeBuilder.java:50)
- [ ] `CuttingBoardRecipeBuilder.addResult(2)`: its class isn't translated (it implements Minecraft's `RecipeBuilder`) (vectorwing/farmersdelight/data/builder/CuttingBoardRecipeBuilder.java:54)
- [ ] `CuttingBoardRecipeBuilder.addResultWithChance(2)`: its class isn't translated (it implements Minecraft's `RecipeBuilder`) (vectorwing/farmersdelight/data/builder/CuttingBoardRecipeBuilder.java:59)
- [ ] `CuttingBoardRecipeBuilder.addResultWithChance(3)`: its class isn't translated (it implements Minecraft's `RecipeBuilder`) (vectorwing/farmersdelight/data/builder/CuttingBoardRecipeBuilder.java:63)
- [ ] `CuttingBoardRecipeBuilder.addSound(1)`: its class isn't translated (it implements Minecraft's `RecipeBuilder`) (vectorwing/farmersdelight/data/builder/CuttingBoardRecipeBuilder.java:68)
- [ ] `CuttingBoardRecipeBuilder.addSound(1)`: its class isn't translated (it implements Minecraft's `RecipeBuilder`) (vectorwing/farmersdelight/data/builder/CuttingBoardRecipeBuilder.java:73)
- [ ] `CuttingBoardRecipeBuilder.unlockedBy(2)`: its class isn't translated (it implements Minecraft's `RecipeBuilder`) (vectorwing/farmersdelight/data/builder/CuttingBoardRecipeBuilder.java:78)
- [ ] `CuttingBoardRecipeBuilder.setNamespace(1)`: its class isn't translated (it implements Minecraft's `RecipeBuilder`) (vectorwing/farmersdelight/data/builder/CuttingBoardRecipeBuilder.java:82)
- [ ] `CuttingBoardRecipeBuilder.salvaging(0)`: its class isn't translated (it implements Minecraft's `RecipeBuilder`) (vectorwing/farmersdelight/data/builder/CuttingBoardRecipeBuilder.java:87)
- [ ] `CuttingBoardRecipeBuilder.group(1)`: its class isn't translated (it implements Minecraft's `RecipeBuilder`) (vectorwing/farmersdelight/data/builder/CuttingBoardRecipeBuilder.java:92)
- [ ] `CuttingBoardRecipeBuilder.defaultId(0)`: its class isn't translated (it implements Minecraft's `RecipeBuilder`) (vectorwing/farmersdelight/data/builder/CuttingBoardRecipeBuilder.java:96)
- [ ] `CuttingBoardRecipeBuilder.getResult(0)`: its class isn't translated (it implements Minecraft's `RecipeBuilder`) (vectorwing/farmersdelight/data/builder/CuttingBoardRecipeBuilder.java:100)
- [ ] `CuttingBoardRecipeBuilder.getDefaultRecipeId(1)`: it uses Minecraft's `ItemLike` (vectorwing/farmersdelight/data/builder/CuttingBoardRecipeBuilder.java:104)
- [ ] `CuttingBoardRecipeBuilder.saveToFD(1)`: its class isn't translated (it implements Minecraft's `RecipeBuilder`) (vectorwing/farmersdelight/data/builder/CuttingBoardRecipeBuilder.java:108)
- [ ] `CuttingBoardRecipeBuilder.save(1)`: its class isn't translated (it implements Minecraft's `RecipeBuilder`) (vectorwing/farmersdelight/data/builder/CuttingBoardRecipeBuilder.java:112)
- [ ] `CuttingBoardRecipeBuilder.build(2)`: its class isn't translated (it implements Minecraft's `RecipeBuilder`) (vectorwing/farmersdelight/data/builder/CuttingBoardRecipeBuilder.java:121)
- [ ] `CuttingBoardRecipeBuilder.build(2)`: its class isn't translated (it implements Minecraft's `RecipeBuilder`) (vectorwing/farmersdelight/data/builder/CuttingBoardRecipeBuilder.java:130)
- [ ] `CuttingBoardRecipeBuilder.save(2)`: its class isn't translated (it implements Minecraft's `RecipeBuilder`) (vectorwing/farmersdelight/data/builder/CuttingBoardRecipeBuilder.java:134)
- [ ] `CuttingBoardRecipeBuilder.save(2)`: its class isn't translated (it implements Minecraft's `RecipeBuilder`) (vectorwing/farmersdelight/data/builder/CuttingBoardRecipeBuilder.java:138)

### `CuttingRecipeFolder`: its constants carry values

- [ ] `CuttingRecipeFolder.CuttingRecipeFolder(1)`: its class isn't translated (its constants carry values) (vectorwing/farmersdelight/data/builder/CuttingRecipeFolder.java:12)
- [ ] `CuttingRecipeFolder.toString(0)`: its class isn't translated (its constants carry values) (vectorwing/farmersdelight/data/builder/CuttingRecipeFolder.java:16)
- [ ] `CuttingRecipeFolder.getSerializedName(0)`: its class isn't translated (its constants carry values) (vectorwing/farmersdelight/data/builder/CuttingRecipeFolder.java:21)

### `DamageTypeTags`: it extends Fabric's `FabricTagsProvider`

- [ ] `DamageTypeTags.DamageTypeTags(2)`: its class isn't translated (it extends Fabric's `FabricTagsProvider`) (vectorwing/farmersdelight/data/DamageTypeTags.java:12)
- [ ] `DamageTypeTags.addTags(1)`: its class isn't translated (it extends Fabric's `FabricTagsProvider`) (vectorwing/farmersdelight/data/DamageTypeTags.java:16)

### `DataGenerators`: it implements Fabric's `DataGeneratorEntrypoint`

- [ ] `DataGenerators.onInitializeDataGenerator(1)`: its class isn't translated (it implements Fabric's `DataGeneratorEntrypoint`) (vectorwing/farmersdelight/data/DataGenerators.java:28)
- [ ] `DataGenerators.buildRegistry(1)`: its class isn't translated (it implements Fabric's `DataGeneratorEntrypoint`) (vectorwing/farmersdelight/data/DataGenerators.java:47)

### `DataGenerators.DynamicRegistryProvider`: it extends Fabric's `FabricDynamicRegistryProvider`

- [ ] `DataGenerators.DynamicRegistryProvider.DynamicRegistryProvider(2)`: its class isn't translated (it extends Fabric's `FabricDynamicRegistryProvider`) (vectorwing/farmersdelight/data/DataGenerators.java:55)
- [ ] `DataGenerators.DynamicRegistryProvider.configure(2)`: its class isn't translated (it extends Fabric's `FabricDynamicRegistryProvider`) (vectorwing/farmersdelight/data/DataGenerators.java:59)
- [ ] `DataGenerators.DynamicRegistryProvider.createContext(2)`: its body couldn't be read: an anonymous class (vectorwing/farmersdelight/data/DataGenerators.java:66)
- [ ] `DataGenerators.DynamicRegistryProvider.getName(0)`: its class isn't translated (it extends Fabric's `FabricDynamicRegistryProvider`) (vectorwing/farmersdelight/data/DataGenerators.java:82)

### `DataMaps`


### `EnchantmentTags`: it extends Fabric's `FabricTagsProvider`

- [ ] `EnchantmentTags.EnchantmentTags(2)`: its class isn't translated (it extends Fabric's `FabricTagsProvider`) (vectorwing/farmersdelight/data/EnchantmentTags.java:11)
- [ ] `EnchantmentTags.addTags(1)`: its class isn't translated (it extends Fabric's `FabricTagsProvider`) (vectorwing/farmersdelight/data/EnchantmentTags.java:15)

### `EntityTags`: it extends Fabric's `EntityTypeTagsProvider`

- [ ] `EntityTags.EntityTags(2)`: its class isn't translated (it extends Fabric's `EntityTypeTagsProvider`) (vectorwing/farmersdelight/data/EntityTags.java:13)
- [ ] `EntityTags.addTags(1)`: its class isn't translated (it extends Fabric's `EntityTypeTagsProvider`) (vectorwing/farmersdelight/data/EntityTags.java:17)

### `ItemModels`: it extends Fabric's `FabricModelProvider`

- [ ] `ItemModels.ItemModels(1)`: its class isn't translated (it extends Fabric's `FabricModelProvider`) (vectorwing/farmersdelight/data/ItemModels.java:17)
- [ ] `ItemModels.generateBlockStateModels(1)`: its class isn't translated (it extends Fabric's `FabricModelProvider`) (vectorwing/farmersdelight/data/ItemModels.java:21)
- [ ] `ItemModels.generateItemModels(1)`: its class isn't translated (it extends Fabric's `FabricModelProvider`) (vectorwing/farmersdelight/data/ItemModels.java:58)
- [ ] `ItemModels.sign(3)`: it uses Minecraft's `BlockModelGenerators` (vectorwing/farmersdelight/data/ItemModels.java:61)
- [ ] `ItemModels.hangingSign(3)`: it uses Minecraft's `BlockModelGenerators` (vectorwing/farmersdelight/data/ItemModels.java:83)

### `ItemTags`: it extends Fabric's `ItemTagsProvider`

- [ ] `ItemTags.ItemTags(3)`: its class isn't translated (it extends Fabric's `ItemTagsProvider`) (vectorwing/farmersdelight/data/ItemTags.java:29)
- [ ] `ItemTags.addTags(1)`: its class isn't translated (it extends Fabric's `ItemTagsProvider`) (vectorwing/farmersdelight/data/ItemTags.java:33)
- [ ] `ItemTags.registerMinecraftTags(0)`: its class isn't translated (it extends Fabric's `ItemTagsProvider`) (vectorwing/farmersdelight/data/ItemTags.java:44)
- [ ] `ItemTags.registerModTags(0)`: its class isn't translated (it extends Fabric's `ItemTagsProvider`) (vectorwing/farmersdelight/data/ItemTags.java:86)
- [ ] `ItemTags.registerRefabricatedTags(0)`: its class isn't translated (it extends Fabric's `ItemTagsProvider`) (vectorwing/farmersdelight/data/ItemTags.java:180)
- [ ] `ItemTags.registerConventionalTags(0)`: its class isn't translated (it extends Fabric's `ItemTagsProvider`) (vectorwing/farmersdelight/data/ItemTags.java:184)
- [ ] `ItemTags.registerCommonTags(0)`: its class isn't translated (it extends Fabric's `ItemTagsProvider`) (vectorwing/farmersdelight/data/ItemTags.java:257)
- [ ] `ItemTags.registerCompatibilityTags(0)`: its class isn't translated (it extends Fabric's `ItemTagsProvider`) (vectorwing/farmersdelight/data/ItemTags.java:305)
- [ ] `ItemTags.tagBuilder(1)`: its class isn't translated (it extends Fabric's `ItemTagsProvider`) (vectorwing/farmersdelight/data/ItemTags.java:343)

### `ItemTags.RefabricatedTagBuilder`: it uses Minecraft's `BlockItemTagAppender`

- [ ] `ItemTags.RefabricatedTagBuilder.RefabricatedTagBuilder(1)`: its class isn't translated (it uses Minecraft's `BlockItemTagAppender`) (vectorwing/farmersdelight/data/ItemTags.java:351)
- [ ] `ItemTags.RefabricatedTagBuilder.addOptionalTag(1)`: its class isn't translated (it uses Minecraft's `BlockItemTagAppender`) (vectorwing/farmersdelight/data/ItemTags.java:356)
- [ ] `ItemTags.RefabricatedTagBuilder.add(1)`: its class isn't translated (it uses Minecraft's `BlockItemTagAppender`) (vectorwing/farmersdelight/data/ItemTags.java:361)
- [ ] `ItemTags.RefabricatedTagBuilder.add(1)`: its class isn't translated (it uses Minecraft's `BlockItemTagAppender`) (vectorwing/farmersdelight/data/ItemTags.java:366)
- [ ] `ItemTags.RefabricatedTagBuilder.addColorKey(1)`: its class isn't translated (it uses Minecraft's `BlockItemTagAppender`) (vectorwing/farmersdelight/data/ItemTags.java:372)
- [ ] `ItemTags.RefabricatedTagBuilder.addColorId(1)`: its class isn't translated (it uses Minecraft's `BlockItemTagAppender`) (vectorwing/farmersdelight/data/ItemTags.java:377)
- [ ] `ItemTags.RefabricatedTagBuilder.addOptional(1)`: its class isn't translated (it uses Minecraft's `BlockItemTagAppender`) (vectorwing/farmersdelight/data/ItemTags.java:382)
- [ ] `ItemTags.RefabricatedTagBuilder.addTags(1)`: its class isn't translated (it uses Minecraft's `BlockItemTagAppender`) (vectorwing/farmersdelight/data/ItemTags.java:387)
- [ ] `ItemTags.RefabricatedTagBuilder.addTag(1)`: its class isn't translated (it uses Minecraft's `BlockItemTagAppender`) (vectorwing/farmersdelight/data/ItemTags.java:394)

### `FDBlockLoot`: it extends Fabric's `FabricBlockLootSubProvider`

- [ ] `FDBlockLoot.FDBlockLoot(2)`: its class isn't translated (it extends Fabric's `FabricBlockLootSubProvider`) (vectorwing/farmersdelight/data/loot/FDBlockLoot.java:56)
- [ ] `FDBlockLoot.generate(0)`: its class isn't translated (it extends Fabric's `FabricBlockLootSubProvider`) (vectorwing/farmersdelight/data/loot/FDBlockLoot.java:61)
- [ ] `FDBlockLoot.hasBlockStateProperties(2)`: its class isn't translated (it extends Fabric's `FabricBlockLootSubProvider`) (vectorwing/farmersdelight/data/loot/FDBlockLoot.java:464)
- [ ] `FDBlockLoot.dropNamedContainer(1)`: its class isn't translated (it extends Fabric's `FabricBlockLootSubProvider`) (vectorwing/farmersdelight/data/loot/FDBlockLoot.java:470)
- [ ] `FDBlockLoot.mushroomColony(2)`: its class isn't translated (it extends Fabric's `FabricBlockLootSubProvider`) (vectorwing/farmersdelight/data/loot/FDBlockLoot.java:474)
- [ ] `FDBlockLoot.createCropDrops(4)`: its class isn't translated (it extends Fabric's `FabricBlockLootSubProvider`) (vectorwing/farmersdelight/data/loot/FDBlockLoot.java:547)
- [ ] `FDBlockLoot.createSeedlessCropDrops(3)`: its class isn't translated (it extends Fabric's `FabricBlockLootSubProvider`) (vectorwing/farmersdelight/data/loot/FDBlockLoot.java:573)
- [ ] `FDBlockLoot.wildCrop(4)`: its class isn't translated (it extends Fabric's `FabricBlockLootSubProvider`) (vectorwing/farmersdelight/data/loot/FDBlockLoot.java:593)
- [ ] `FDBlockLoot.wildCropNoSeeds(3)`: its class isn't translated (it extends Fabric's `FabricBlockLootSubProvider`) (vectorwing/farmersdelight/data/loot/FDBlockLoot.java:621)
- [ ] `FDBlockLoot.platedFood(2)`: its class isn't translated (it extends Fabric's `FabricBlockLootSubProvider`) (vectorwing/farmersdelight/data/loot/FDBlockLoot.java:642)
- [ ] `FDBlockLoot.platedFoodExtraDrop(3)`: its class isn't translated (it extends Fabric's `FabricBlockLootSubProvider`) (vectorwing/farmersdelight/data/loot/FDBlockLoot.java:667)

### `FDChestLoot`: it extends Fabric's `SimpleFabricLootTableSubProvider`

- [ ] `FDChestLoot.FDChestLoot(2)`: its class isn't translated (it extends Fabric's `SimpleFabricLootTableSubProvider`) (vectorwing/farmersdelight/data/loot/FDChestLoot.java:31)
- [ ] `FDChestLoot.generate(1)`: its class isn't translated (it extends Fabric's `SimpleFabricLootTableSubProvider`) (vectorwing/farmersdelight/data/loot/FDChestLoot.java:36)
- [ ] `FDChestLoot.getName(0)`: its class isn't translated (it extends Fabric's `SimpleFabricLootTableSubProvider`) (vectorwing/farmersdelight/data/loot/FDChestLoot.java:390)
- [ ] `FDChestLoot.run(0)`: its class isn't translated (it extends Fabric's `SimpleFabricLootTableSubProvider`) (vectorwing/farmersdelight/data/loot/FDChestLoot.java:394)

### `ModEnchantments`

- [ ] `ModEnchantments.BACKSTABBING`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/data/ModEnchantments.java:18)
- [ ] `ModEnchantments.bootstrap(1)`: it uses Minecraft's `BootstrapContext` (vectorwing/farmersdelight/data/ModEnchantments.java:20)
- [ ] `ModEnchantments.register(3)`: it uses Minecraft's `BootstrapContext` (vectorwing/farmersdelight/data/ModEnchantments.java:40)
- [ ] `ModEnchantments.key(1)`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/data/ModEnchantments.java:44)

### `Recipes`: it extends Fabric's `FabricRecipeProvider`

- [ ] `Recipes.recipeContext`: it uses Minecraft's `BootstrapContext` (vectorwing/farmersdelight/data/Recipes.java:18)
- [ ] `Recipes.Recipes(2)`: its class isn't translated (it extends Fabric's `FabricRecipeProvider`) (vectorwing/farmersdelight/data/Recipes.java:20)
- [ ] `Recipes.createRecipeProvider(3)`: its body couldn't be read: an anonymous class (vectorwing/farmersdelight/data/Recipes.java:24)
- [ ] `Recipes.getName(0)`: its class isn't translated (it extends Fabric's `FabricRecipeProvider`) (vectorwing/farmersdelight/data/Recipes.java:40)

### `CookingRecipes`

- [x] `CookingRecipes.FAST_COOKING` (vectorwing/farmersdelight/data/recipe/CookingRecipes.java:22)
- [x] `CookingRecipes.NORMAL_COOKING` (vectorwing/farmersdelight/data/recipe/CookingRecipes.java:23)
- [x] `CookingRecipes.SLOW_COOKING` (vectorwing/farmersdelight/data/recipe/CookingRecipes.java:24)
- [x] `CookingRecipes.SMALL_EXP` (vectorwing/farmersdelight/data/recipe/CookingRecipes.java:25)
- [x] `CookingRecipes.MEDIUM_EXP` (vectorwing/farmersdelight/data/recipe/CookingRecipes.java:26)
- [x] `CookingRecipes.LARGE_EXP` (vectorwing/farmersdelight/data/recipe/CookingRecipes.java:27)
- [ ] `CookingRecipes.register(2)`: it uses Minecraft's `Provider` (vectorwing/farmersdelight/data/recipe/CookingRecipes.java:29)
- [ ] `CookingRecipes.cookMiscellaneous(2)`: it uses Minecraft's `HolderGetter` (vectorwing/farmersdelight/data/recipe/CookingRecipes.java:36)
- [ ] `CookingRecipes.cookMinecraftSoups(2)`: it uses Minecraft's `HolderGetter` (vectorwing/farmersdelight/data/recipe/CookingRecipes.java:76)
- [ ] `CookingRecipes.cookMeals(2)`: it uses Minecraft's `HolderGetter` (vectorwing/farmersdelight/data/recipe/CookingRecipes.java:100)

### `CraftingRecipes`

- [ ] `CraftingRecipes.register(3)`: it uses Minecraft's `Provider` (vectorwing/farmersdelight/data/recipe/CraftingRecipes.java:36)
- [ ] `CraftingRecipes.canvasSignDyeing(5)`: it uses Minecraft's `HolderGetter` (vectorwing/farmersdelight/data/recipe/CraftingRecipes.java:50)
- [ ] `CraftingRecipes.recipesVanillaAlternatives(2)`: it uses Minecraft's `HolderGetter` (vectorwing/farmersdelight/data/recipe/CraftingRecipes.java:67)
- [ ] `CraftingRecipes.recipesCanvasSigns(2)`: it uses Minecraft's `HolderGetter` (vectorwing/farmersdelight/data/recipe/CraftingRecipes.java:152)
- [ ] `CraftingRecipes.recipesBlocks(3)`: it uses Minecraft's `HolderGetter` (vectorwing/farmersdelight/data/recipe/CraftingRecipes.java:245)
- [ ] `CraftingRecipes.recipesTools(2)`: it uses Minecraft's `HolderGetter` (vectorwing/farmersdelight/data/recipe/CraftingRecipes.java:579)
- [ ] `CraftingRecipes.recipesMaterials(2)`: it uses Minecraft's `HolderGetter` (vectorwing/farmersdelight/data/recipe/CraftingRecipes.java:632)
- [ ] `CraftingRecipes.recipesFoodstuffs(2)`: it uses Minecraft's `HolderGetter` (vectorwing/farmersdelight/data/recipe/CraftingRecipes.java:685)
- [ ] `CraftingRecipes.recipesFoodBlocks(2)`: it uses Minecraft's `HolderGetter` (vectorwing/farmersdelight/data/recipe/CraftingRecipes.java:770)
- [ ] `CraftingRecipes.recipesCraftedMeals(2)`: it uses Minecraft's `HolderGetter` (vectorwing/farmersdelight/data/recipe/CraftingRecipes.java:844)
- [ ] `CraftingRecipes.fdLocation(1)`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/data/recipe/CraftingRecipes.java:1026)

### `CuttingRecipes`

- [ ] `CuttingRecipes.register(2)`: it uses Minecraft's `Provider` (vectorwing/farmersdelight/data/recipe/CuttingRecipes.java:26)
- [ ] `CuttingRecipes.cuttingAnimalItems(2)`: it uses Minecraft's `HolderGetter` (vectorwing/farmersdelight/data/recipe/CuttingRecipes.java:40)
- [ ] `CuttingRecipes.cuttingVegetables(2)`: it uses Minecraft's `HolderGetter` (vectorwing/farmersdelight/data/recipe/CuttingRecipes.java:112)
- [ ] `CuttingRecipes.cuttingFoods(2)`: it uses Minecraft's `HolderGetter` (vectorwing/farmersdelight/data/recipe/CuttingRecipes.java:152)
- [ ] `CuttingRecipes.cuttingFlowers(2)`: it uses Minecraft's `HolderGetter` (vectorwing/farmersdelight/data/recipe/CuttingRecipes.java:201)
- [ ] `CuttingRecipes.salvagingMinerals(1)`: it uses Minecraft's `RecipeOutput` (vectorwing/farmersdelight/data/recipe/CuttingRecipes.java:328)
- [ ] `CuttingRecipes.strippingWood(2)`: it uses Minecraft's `HolderGetter` (vectorwing/farmersdelight/data/recipe/CuttingRecipes.java:351)
- [ ] `CuttingRecipes.salvagingWoodenFurniture(2)`: it uses Minecraft's `HolderGetter` (vectorwing/farmersdelight/data/recipe/CuttingRecipes.java:384)
- [ ] `CuttingRecipes.diggingSediments(2)`: it uses Minecraft's `HolderGetter` (vectorwing/farmersdelight/data/recipe/CuttingRecipes.java:600)
- [ ] `CuttingRecipes.salvagingUsingShears(2)`: it uses Minecraft's `HolderGetter` (vectorwing/farmersdelight/data/recipe/CuttingRecipes.java:608)
- [ ] `CuttingRecipes.salvagingBlockFromVehicle(2)`: it uses Minecraft's `HolderGetter` (vectorwing/farmersdelight/data/recipe/CuttingRecipes.java:624)
- [ ] `CuttingRecipes.salvagePlankFromFurniture(5)`: it uses Minecraft's `HolderGetter` (vectorwing/farmersdelight/data/recipe/CuttingRecipes.java:689)
- [ ] `CuttingRecipes.stripLogForBark(4)`: it uses Minecraft's `HolderGetter` (vectorwing/farmersdelight/data/recipe/CuttingRecipes.java:694)
- [ ] `CuttingRecipes.matchesTool(3)`: it uses Minecraft's `Ingredient` (vectorwing/farmersdelight/data/recipe/CuttingRecipes.java:701)
- [x] `CuttingRecipes.salvagingRecipe(1)` (vectorwing/farmersdelight/data/recipe/CuttingRecipes.java:707)

### `SmeltingRecipes`

- [ ] `SmeltingRecipes.register(2)`: it uses Minecraft's `Provider` (vectorwing/farmersdelight/data/recipe/SmeltingRecipes.java:23)
- [ ] `SmeltingRecipes.foodSmeltingRecipes(5)`: it uses Minecraft's `ItemLike` (vectorwing/farmersdelight/data/recipe/SmeltingRecipes.java:75)
- [ ] `SmeltingRecipes.foodSmeltingRecipes(6)`: it uses Minecraft's `HolderGetter` (vectorwing/farmersdelight/data/recipe/SmeltingRecipes.java:88)

### `SoundDefinitions`: it extends Fabric's `FabricSoundsProvider`

- [ ] `SoundDefinitions.SoundDefinitions(2)`: its class isn't translated (it extends Fabric's `FabricSoundsProvider`) (vectorwing/farmersdelight/data/SoundDefinitions.java:19)
- [ ] `SoundDefinitions.configure(2)`: its class isn't translated (it extends Fabric's `FabricSoundsProvider`) (vectorwing/farmersdelight/data/SoundDefinitions.java:23)
- [ ] `SoundDefinitions.generateNewSoundWithSubtitle(4)`: its class isn't translated (it extends Fabric's `FabricSoundsProvider`) (vectorwing/farmersdelight/data/SoundDefinitions.java:48)
- [ ] `SoundDefinitions.generateNewSound(5)`: its class isn't translated (it extends Fabric's `FabricSoundsProvider`) (vectorwing/farmersdelight/data/SoundDefinitions.java:52)
- [ ] `SoundDefinitions.generateNewSoundCustomSubtitle(5)`: its class isn't translated (it extends Fabric's `FabricSoundsProvider`) (vectorwing/farmersdelight/data/SoundDefinitions.java:57)
- [ ] `SoundDefinitions.generateExistingSoundWithSubtitle(3)`: its class isn't translated (it extends Fabric's `FabricSoundsProvider`) (vectorwing/farmersdelight/data/SoundDefinitions.java:72)
- [ ] `SoundDefinitions.generateExistingSound(4)`: its class isn't translated (it extends Fabric's `FabricSoundsProvider`) (vectorwing/farmersdelight/data/SoundDefinitions.java:76)
- [ ] `SoundDefinitions.getName(0)`: its class isn't translated (it extends Fabric's `FabricSoundsProvider`) (vectorwing/farmersdelight/data/SoundDefinitions.java:85)

### `StructureUpdater`: it implements Minecraft's `DataProvider`

- [ ] `StructureUpdater.StructureUpdater(3)`: its class isn't translated (it implements Minecraft's `DataProvider`) (vectorwing/farmersdelight/data/tools/StructureUpdater.java:39)
- [ ] `StructureUpdater.run(1)`: its class isn't translated (it implements Minecraft's `DataProvider`) (vectorwing/farmersdelight/data/tools/StructureUpdater.java:48)
- [ ] `StructureUpdater.process(3)`: its class isn't translated (it implements Minecraft's `DataProvider`) (vectorwing/farmersdelight/data/tools/StructureUpdater.java:58)
- [ ] `StructureUpdater.writeNBTTo(3)`: its class isn't translated (it implements Minecraft's `DataProvider`) (vectorwing/farmersdelight/data/tools/StructureUpdater.java:71)
- [ ] `StructureUpdater.updateNBT(1)`: it uses Minecraft's `CompoundTag` (vectorwing/farmersdelight/data/tools/StructureUpdater.java:79)
- [ ] `StructureUpdater.getName(0)`: its class isn't translated (it implements Minecraft's `DataProvider`) (vectorwing/farmersdelight/data/tools/StructureUpdater.java:87)

### `VillagerTags`: it extends Fabric's `FabricTagsProvider`

- [ ] `VillagerTags.VillagerTags(2)`: its class isn't translated (it extends Fabric's `FabricTagsProvider`) (vectorwing/farmersdelight/data/VillagerTags.java:13)
- [ ] `VillagerTags.addTags(1)`: its class isn't translated (it extends Fabric's `FabricTagsProvider`) (vectorwing/farmersdelight/data/VillagerTags.java:17)

### `VillagerTrades`: it extends Fabric's `FabricDynamicRegistryProvider`

- [ ] `VillagerTrades.FARMER_1_ONION_EMERALD`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/data/VillagerTrades.java:24)
- [ ] `VillagerTrades.FARMER_1_TOMATO_EMERALD`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/data/VillagerTrades.java:25)
- [ ] `VillagerTrades.FARMER_2_CABBAGE_EMERALD`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/data/VillagerTrades.java:26)
- [ ] `VillagerTrades.FARMER_2_RICE_EMERALD`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/data/VillagerTrades.java:27)
- [ ] `VillagerTrades.WANDERING_TRADER_CABBAGE_EMERALD`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/data/VillagerTrades.java:28)
- [ ] `VillagerTrades.WANDERING_TRADER_TOMATO_EMERALD`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/data/VillagerTrades.java:29)
- [ ] `VillagerTrades.WANDERING_TRADER_RICE_EMERALD`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/data/VillagerTrades.java:30)
- [ ] `VillagerTrades.WANDERING_TRADER_ONION_EMERALD`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/data/VillagerTrades.java:31)
- [ ] `VillagerTrades.VillagerTrades(2)`: its class isn't translated (it extends Fabric's `FabricDynamicRegistryProvider`) (vectorwing/farmersdelight/data/VillagerTrades.java:33)
- [ ] `VillagerTrades.init(1)`: it uses Minecraft's `BootstrapContext` (vectorwing/farmersdelight/data/VillagerTrades.java:37)
- [ ] `VillagerTrades.onVillagerTrades(1)`: it uses Minecraft's `BootstrapContext` (vectorwing/farmersdelight/data/VillagerTrades.java:42)
- [ ] `VillagerTrades.onWandererTrades(1)`: it uses Minecraft's `BootstrapContext` (vectorwing/farmersdelight/data/VillagerTrades.java:49)
- [ ] `VillagerTrades.emeraldsForItemsTrade(4)`: it uses Minecraft's `VillagerTrade` (vectorwing/farmersdelight/data/VillagerTrades.java:56)
- [ ] `VillagerTrades.itemForEmeraldTrade(3)`: it uses Minecraft's `VillagerTrade` (vectorwing/farmersdelight/data/VillagerTrades.java:60)
- [ ] `VillagerTrades.resourceKey(1)`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/data/VillagerTrades.java:64)
- [ ] `VillagerTrades.configure(2)`: its body couldn't be read: an anonymous class (vectorwing/farmersdelight/data/VillagerTrades.java:68)
- [ ] `VillagerTrades.getName(0)`: its class isn't translated (it extends Fabric's `FabricDynamicRegistryProvider`) (vectorwing/farmersdelight/data/VillagerTrades.java:84)

### `FarmersDelight`: it implements Fabric's `ModInitializer`

- [x] `FarmersDelight.MODID` (vectorwing/farmersdelight/FarmersDelight.java:53)
- [x] `FarmersDelight.LOGGER` (vectorwing/farmersdelight/FarmersDelight.java:54)
- [x] `FarmersDelight.id(1)` (vectorwing/farmersdelight/FarmersDelight.java:56)
- [ ] `FarmersDelight.onInitialize(0)`: its class isn't translated (it implements Fabric's `ModInitializer`) (vectorwing/farmersdelight/FarmersDelight.java:60)

### `CookingRecipeCategory`: it implements `IRecipeCategory` (mezz.jei.api.recipe.category)

- [ ] `CookingRecipeCategory.CookingRecipeCategory(1)`: its class isn't translated (it implements `IRecipeCategory` (mezz.jei.api.recipe.category)) (vectorwing/farmersdelight/integration/jei/category/CookingRecipeCategory.java:39)
- [ ] `CookingRecipeCategory.getRecipeType(0)`: its class isn't translated (it implements `IRecipeCategory` (mezz.jei.api.recipe.category)) (vectorwing/farmersdelight/integration/jei/category/CookingRecipeCategory.java:50)
- [ ] `CookingRecipeCategory.getTitle(0)`: its class isn't translated (it implements `IRecipeCategory` (mezz.jei.api.recipe.category)) (vectorwing/farmersdelight/integration/jei/category/CookingRecipeCategory.java:54)
- [ ] `CookingRecipeCategory.getWidth(0)`: its class isn't translated (it implements `IRecipeCategory` (mezz.jei.api.recipe.category)) (vectorwing/farmersdelight/integration/jei/category/CookingRecipeCategory.java:58)
- [ ] `CookingRecipeCategory.getHeight(0)`: its class isn't translated (it implements `IRecipeCategory` (mezz.jei.api.recipe.category)) (vectorwing/farmersdelight/integration/jei/category/CookingRecipeCategory.java:62)
- [ ] `CookingRecipeCategory.getIcon(0)`: its class isn't translated (it implements `IRecipeCategory` (mezz.jei.api.recipe.category)) (vectorwing/farmersdelight/integration/jei/category/CookingRecipeCategory.java:66)
- [ ] `CookingRecipeCategory.setRecipe(3)`: its class isn't translated (it implements `IRecipeCategory` (mezz.jei.api.recipe.category)) (vectorwing/farmersdelight/integration/jei/category/CookingRecipeCategory.java:70)
- [ ] `CookingRecipeCategory.draw(5)`: its class isn't translated (it implements `IRecipeCategory` (mezz.jei.api.recipe.category)) (vectorwing/farmersdelight/integration/jei/category/CookingRecipeCategory.java:94)
- [ ] `CookingRecipeCategory.getTooltip(5)`: its class isn't translated (it implements `IRecipeCategory` (mezz.jei.api.recipe.category)) (vectorwing/farmersdelight/integration/jei/category/CookingRecipeCategory.java:104)

### `CuttingRecipeCategory`: it implements `IRecipeCategory` (mezz.jei.api.recipe.category)

- [x] `CuttingRecipeCategory.OUTPUT_GRID_X` (vectorwing/farmersdelight/integration/jei/category/CuttingRecipeCategory.java:28)
- [x] `CuttingRecipeCategory.OUTPUT_GRID_Y` (vectorwing/farmersdelight/integration/jei/category/CuttingRecipeCategory.java:29)
- [ ] `CuttingRecipeCategory.CuttingRecipeCategory(1)`: its class isn't translated (it implements `IRecipeCategory` (mezz.jei.api.recipe.category)) (vectorwing/farmersdelight/integration/jei/category/CuttingRecipeCategory.java:36)
- [ ] `CuttingRecipeCategory.getRecipeType(0)`: its class isn't translated (it implements `IRecipeCategory` (mezz.jei.api.recipe.category)) (vectorwing/farmersdelight/integration/jei/category/CuttingRecipeCategory.java:44)
- [ ] `CuttingRecipeCategory.getTitle(0)`: its class isn't translated (it implements `IRecipeCategory` (mezz.jei.api.recipe.category)) (vectorwing/farmersdelight/integration/jei/category/CuttingRecipeCategory.java:48)
- [ ] `CuttingRecipeCategory.getWidth(0)`: its class isn't translated (it implements `IRecipeCategory` (mezz.jei.api.recipe.category)) (vectorwing/farmersdelight/integration/jei/category/CuttingRecipeCategory.java:52)
- [ ] `CuttingRecipeCategory.getHeight(0)`: its class isn't translated (it implements `IRecipeCategory` (mezz.jei.api.recipe.category)) (vectorwing/farmersdelight/integration/jei/category/CuttingRecipeCategory.java:56)
- [ ] `CuttingRecipeCategory.getIcon(0)`: its class isn't translated (it implements `IRecipeCategory` (mezz.jei.api.recipe.category)) (vectorwing/farmersdelight/integration/jei/category/CuttingRecipeCategory.java:60)
- [ ] `CuttingRecipeCategory.setRecipe(3)`: its class isn't translated (it implements `IRecipeCategory` (mezz.jei.api.recipe.category)) (vectorwing/farmersdelight/integration/jei/category/CuttingRecipeCategory.java:64)
- [ ] `CuttingRecipeCategory.draw(5)`: its class isn't translated (it implements `IRecipeCategory` (mezz.jei.api.recipe.category)) (vectorwing/farmersdelight/integration/jei/category/CuttingRecipeCategory.java:88)

### `DecompositionRecipeCategory`: it implements `IRecipeCategory` (mezz.jei.api.recipe.category)

- [x] `DecompositionRecipeCategory.UID` (vectorwing/farmersdelight/integration/jei/category/DecompositionRecipeCategory.java:31)
- [x] `DecompositionRecipeCategory.slotSize` (vectorwing/farmersdelight/integration/jei/category/DecompositionRecipeCategory.java:32)
- [ ] `DecompositionRecipeCategory.DecompositionRecipeCategory(1)`: its class isn't translated (it implements `IRecipeCategory` (mezz.jei.api.recipe.category)) (vectorwing/farmersdelight/integration/jei/category/DecompositionRecipeCategory.java:40)
- [ ] `DecompositionRecipeCategory.getRecipeType(0)`: its class isn't translated (it implements `IRecipeCategory` (mezz.jei.api.recipe.category)) (vectorwing/farmersdelight/integration/jei/category/DecompositionRecipeCategory.java:49)
- [ ] `DecompositionRecipeCategory.getTitle(0)`: its class isn't translated (it implements `IRecipeCategory` (mezz.jei.api.recipe.category)) (vectorwing/farmersdelight/integration/jei/category/DecompositionRecipeCategory.java:53)
- [ ] `DecompositionRecipeCategory.getWidth(0)`: its class isn't translated (it implements `IRecipeCategory` (mezz.jei.api.recipe.category)) (vectorwing/farmersdelight/integration/jei/category/DecompositionRecipeCategory.java:57)
- [ ] `DecompositionRecipeCategory.getHeight(0)`: its class isn't translated (it implements `IRecipeCategory` (mezz.jei.api.recipe.category)) (vectorwing/farmersdelight/integration/jei/category/DecompositionRecipeCategory.java:61)
- [ ] `DecompositionRecipeCategory.getIcon(0)`: its class isn't translated (it implements `IRecipeCategory` (mezz.jei.api.recipe.category)) (vectorwing/farmersdelight/integration/jei/category/DecompositionRecipeCategory.java:65)
- [ ] `DecompositionRecipeCategory.setRecipe(3)`: its class isn't translated (it implements `IRecipeCategory` (mezz.jei.api.recipe.category)) (vectorwing/farmersdelight/integration/jei/category/DecompositionRecipeCategory.java:69)
- [ ] `DecompositionRecipeCategory.draw(5)`: its class isn't translated (it implements `IRecipeCategory` (mezz.jei.api.recipe.category)) (vectorwing/farmersdelight/integration/jei/category/DecompositionRecipeCategory.java:77)
- [ ] `DecompositionRecipeCategory.getTooltip(5)`: its class isn't translated (it implements `IRecipeCategory` (mezz.jei.api.recipe.category)) (vectorwing/farmersdelight/integration/jei/category/DecompositionRecipeCategory.java:82)
- [ ] `DecompositionRecipeCategory.translateKey(1)`: it calls TextUtils.JEI, which isn't translated (vectorwing/farmersdelight/integration/jei/category/DecompositionRecipeCategory.java:96)

### `FDRecipes`: it uses Fabric's `SynchronizedRecipes`

- [ ] `FDRecipes.FDRecipes(0)`: its class isn't translated (it uses Fabric's `SynchronizedRecipes`) (vectorwing/farmersdelight/integration/jei/FDRecipes.java:15)
- [ ] `FDRecipes.getCookingPotRecipes(0)`: its class isn't translated (it uses Fabric's `SynchronizedRecipes`) (vectorwing/farmersdelight/integration/jei/FDRecipes.java:25)
- [ ] `FDRecipes.getCuttingBoardRecipes(0)`: its class isn't translated (it uses Fabric's `SynchronizedRecipes`) (vectorwing/farmersdelight/integration/jei/FDRecipes.java:29)

### `FDRecipeTypes`

- [ ] `FDRecipeTypes.COOKING`: it uses `IRecipeType` (mezz.jei.api.recipe.types) (vectorwing/farmersdelight/integration/jei/FDRecipeTypes.java:11)
- [ ] `FDRecipeTypes.CUTTING`: it uses `IRecipeType` (mezz.jei.api.recipe.types) (vectorwing/farmersdelight/integration/jei/FDRecipeTypes.java:12)
- [ ] `FDRecipeTypes.DECOMPOSITION`: it uses `IRecipeType` (mezz.jei.api.recipe.types) (vectorwing/farmersdelight/integration/jei/FDRecipeTypes.java:13)

### `JEIPlugin`: it implements `IModPlugin` (mezz.jei.api)

- [x] `JEIPlugin.ID` (vectorwing/farmersdelight/integration/jei/JEIPlugin.java:35)
- [ ] `JEIPlugin.registerCategories(1)`: its class isn't translated (it implements `IModPlugin` (mezz.jei.api)) (vectorwing/farmersdelight/integration/jei/JEIPlugin.java:37)
- [ ] `JEIPlugin.registerRecipes(1)`: its class isn't translated (it implements `IModPlugin` (mezz.jei.api)) (vectorwing/farmersdelight/integration/jei/JEIPlugin.java:43)
- [ ] `JEIPlugin.registerRecipeCatalysts(1)`: its class isn't translated (it implements `IModPlugin` (mezz.jei.api)) (vectorwing/farmersdelight/integration/jei/JEIPlugin.java:112)
- [ ] `JEIPlugin.registerGuiHandlers(1)`: its class isn't translated (it implements `IModPlugin` (mezz.jei.api)) (vectorwing/farmersdelight/integration/jei/JEIPlugin.java:120)
- [ ] `JEIPlugin.registerRecipeTransferHandlers(1)`: its class isn't translated (it implements `IModPlugin` (mezz.jei.api)) (vectorwing/farmersdelight/integration/jei/JEIPlugin.java:124)
- [ ] `JEIPlugin.getPluginUid(0)`: its class isn't translated (it implements `IModPlugin` (mezz.jei.api)) (vectorwing/farmersdelight/integration/jei/JEIPlugin.java:128)

### `DecompositionDummy`


### `DoughRecipeMaker`

- [ ] `DoughRecipeMaker.createRecipe(0)`: it uses Minecraft's `RecipeHolder` (vectorwing/farmersdelight/integration/jei/resource/DoughRecipeMaker.java:21)

### `LaunchpadClientEvents`

- [ ] `LaunchpadClientEvents.register(1)`: it uses `RegisterGuiLayersEvent` (net.neoforged.neoforge.client.event) (vectorwing/farmersdelight/integration/launchpad/LaunchpadClientEvents.java:12)

### `LaunchpadEvents`

- [ ] `LaunchpadEvents.getGrowthSpeed(3)`: it uses Minecraft's `BlockState` (vectorwing/farmersdelight/integration/launchpad/LaunchpadEvents.java:11)

### `ModMenuCompat`: it implements `ModMenuApi` (com.terraformersmc.modmenu.api)

- [ ] `ModMenuCompat.getModConfigScreenFactory(0)`: done another way in Horizon: the mod's page in Options > Mods (its settings buttons) (vectorwing/farmersdelight/integration/modmenu/ModMenuCompat.java:11)

### `CookingPotCategory`: it implements `DisplayCategory` (me.shedaniel.rei.api.client.registry.display)

- [x] `CookingPotCategory.TITLE` in game_side.rs (vectorwing/farmersdelight/integration/rei/client/categories/CookingPotCategory.java:28)
- [x] `CookingPotCategory.BACKGROUND` (vectorwing/farmersdelight/integration/rei/client/categories/CookingPotCategory.java:29)
- [ ] `CookingPotCategory.getCategoryIdentifier(0)`: its class isn't translated (it implements `DisplayCategory` (me.shedaniel.rei.api.client.registry.display)) (vectorwing/farmersdelight/integration/rei/client/categories/CookingPotCategory.java:31)
- [ ] `CookingPotCategory.getTitle(0)`: its class isn't translated (it implements `DisplayCategory` (me.shedaniel.rei.api.client.registry.display)) (vectorwing/farmersdelight/integration/rei/client/categories/CookingPotCategory.java:35)
- [ ] `CookingPotCategory.getIcon(0)`: its class isn't translated (it implements `DisplayCategory` (me.shedaniel.rei.api.client.registry.display)) (vectorwing/farmersdelight/integration/rei/client/categories/CookingPotCategory.java:39)
- [ ] `CookingPotCategory.getDisplayHeight(0)`: its class isn't translated (it implements `DisplayCategory` (me.shedaniel.rei.api.client.registry.display)) (vectorwing/farmersdelight/integration/rei/client/categories/CookingPotCategory.java:43)
- [ ] `CookingPotCategory.getDisplayWidth(1)`: its class isn't translated (it implements `DisplayCategory` (me.shedaniel.rei.api.client.registry.display)) (vectorwing/farmersdelight/integration/rei/client/categories/CookingPotCategory.java:47)
- [ ] `CookingPotCategory.getDisplayRenderer(1)`: its body couldn't be read: an anonymous class (vectorwing/farmersdelight/integration/rei/client/categories/CookingPotCategory.java:51)
- [ ] `CookingPotCategory.setupDisplay(2)`: its class isn't translated (it implements `DisplayCategory` (me.shedaniel.rei.api.client.registry.display)) (vectorwing/farmersdelight/integration/rei/client/categories/CookingPotCategory.java:63)
- [ ] `CookingPotCategory.getTooltipStrings(1)`: its class isn't translated (it implements `DisplayCategory` (me.shedaniel.rei.api.client.registry.display)) (vectorwing/farmersdelight/integration/rei/client/categories/CookingPotCategory.java:107)

### `CuttingCategory`: it implements `DisplayCategory` (me.shedaniel.rei.api.client.registry.display)

- [x] `CuttingCategory.BACKGROUND` (vectorwing/farmersdelight/integration/rei/client/categories/CuttingCategory.java:26)
- [x] `CuttingCategory.OUTPUT_GRID_X` (vectorwing/farmersdelight/integration/rei/client/categories/CuttingCategory.java:27)
- [x] `CuttingCategory.OUTPUT_GRID_Y` (vectorwing/farmersdelight/integration/rei/client/categories/CuttingCategory.java:28)
- [ ] `CuttingCategory.getCategoryIdentifier(0)`: its class isn't translated (it implements `DisplayCategory` (me.shedaniel.rei.api.client.registry.display)) (vectorwing/farmersdelight/integration/rei/client/categories/CuttingCategory.java:30)
- [ ] `CuttingCategory.getTitle(0)`: its class isn't translated (it implements `DisplayCategory` (me.shedaniel.rei.api.client.registry.display)) (vectorwing/farmersdelight/integration/rei/client/categories/CuttingCategory.java:34)
- [ ] `CuttingCategory.getIcon(0)`: its class isn't translated (it implements `DisplayCategory` (me.shedaniel.rei.api.client.registry.display)) (vectorwing/farmersdelight/integration/rei/client/categories/CuttingCategory.java:38)
- [ ] `CuttingCategory.getDisplayHeight(0)`: its class isn't translated (it implements `DisplayCategory` (me.shedaniel.rei.api.client.registry.display)) (vectorwing/farmersdelight/integration/rei/client/categories/CuttingCategory.java:42)
- [ ] `CuttingCategory.getDisplayWidth(1)`: its class isn't translated (it implements `DisplayCategory` (me.shedaniel.rei.api.client.registry.display)) (vectorwing/farmersdelight/integration/rei/client/categories/CuttingCategory.java:46)
- [ ] `CuttingCategory.setupDisplay(2)`: its class isn't translated (it implements `DisplayCategory` (me.shedaniel.rei.api.client.registry.display)) (vectorwing/farmersdelight/integration/rei/client/categories/CuttingCategory.java:50)

### `DecompositionCategory`: it implements `DisplayCategory` (me.shedaniel.rei.api.client.registry.display)

- [x] `DecompositionCategory.BACKGROUND` (vectorwing/farmersdelight/integration/rei/client/categories/DecompositionCategory.java:25)
- [x] `DecompositionCategory.OUTPUT_GRID_X` (vectorwing/farmersdelight/integration/rei/client/categories/DecompositionCategory.java:26)
- [x] `DecompositionCategory.OUTPUT_GRID_Y` (vectorwing/farmersdelight/integration/rei/client/categories/DecompositionCategory.java:27)
- [ ] `DecompositionCategory.getCategoryIdentifier(0)`: its class isn't translated (it implements `DisplayCategory` (me.shedaniel.rei.api.client.registry.display)) (vectorwing/farmersdelight/integration/rei/client/categories/DecompositionCategory.java:29)
- [ ] `DecompositionCategory.getTitle(0)`: its class isn't translated (it implements `DisplayCategory` (me.shedaniel.rei.api.client.registry.display)) (vectorwing/farmersdelight/integration/rei/client/categories/DecompositionCategory.java:33)
- [ ] `DecompositionCategory.getIcon(0)`: its class isn't translated (it implements `DisplayCategory` (me.shedaniel.rei.api.client.registry.display)) (vectorwing/farmersdelight/integration/rei/client/categories/DecompositionCategory.java:37)
- [ ] `DecompositionCategory.getDisplayHeight(0)`: its class isn't translated (it implements `DisplayCategory` (me.shedaniel.rei.api.client.registry.display)) (vectorwing/farmersdelight/integration/rei/client/categories/DecompositionCategory.java:41)
- [ ] `DecompositionCategory.getDisplayWidth(1)`: its class isn't translated (it implements `DisplayCategory` (me.shedaniel.rei.api.client.registry.display)) (vectorwing/farmersdelight/integration/rei/client/categories/DecompositionCategory.java:45)
- [ ] `DecompositionCategory.setupDisplay(2)`: its class isn't translated (it implements `DisplayCategory` (me.shedaniel.rei.api.client.registry.display)) (vectorwing/farmersdelight/integration/rei/client/categories/DecompositionCategory.java:49)
- [ ] `DecompositionCategory.translateKey(1)`: a translated text with a computed key (vectorwing/farmersdelight/integration/rei/client/categories/DecompositionCategory.java:63)
- [ ] `DecompositionCategory.getFixedDisplaysPerPage(0)`: its class isn't translated (it implements `DisplayCategory` (me.shedaniel.rei.api.client.registry.display)) (vectorwing/farmersdelight/integration/rei/client/categories/DecompositionCategory.java:67)

### `ClientREIPlugin`: it implements `REIClientPlugin` (me.shedaniel.rei.api.client.plugins)

- [ ] `ClientREIPlugin.registerDisplays(1)`: its class isn't translated (it implements `REIClientPlugin` (me.shedaniel.rei.api.client.plugins)) (vectorwing/farmersdelight/integration/rei/client/ClientREIPlugin.java:31)
- [ ] `ClientREIPlugin.registerScreens(1)`: its class isn't translated (it implements `REIClientPlugin` (me.shedaniel.rei.api.client.plugins)) (vectorwing/farmersdelight/integration/rei/client/ClientREIPlugin.java:112)
- [ ] `ClientREIPlugin.registerCategories(1)`: its class isn't translated (it implements `REIClientPlugin` (me.shedaniel.rei.api.client.plugins)) (vectorwing/farmersdelight/integration/rei/client/ClientREIPlugin.java:116)
- [ ] `ClientREIPlugin.registerTransferHandlers(1)`: its class isn't translated (it implements `REIClientPlugin` (me.shedaniel.rei.api.client.plugins)) (vectorwing/farmersdelight/integration/rei/client/ClientREIPlugin.java:122)

### `ClientsidedCookingPotDisplay`: it extends `BasicDisplay` (me.shedaniel.rei.api.common.display.basic)

- [ ] `ClientsidedCookingPotDisplay.SERIALIZER`: it uses `DisplaySerializer` (me.shedaniel.rei.api.common.display) (vectorwing/farmersdelight/integration/rei/client/display/ClientsidedCookingPotDisplay.java:27)
- [ ] `ClientsidedCookingPotDisplay.ClientsidedCookingPotDisplay(2)`: its class isn't translated (it extends `BasicDisplay` (me.shedaniel.rei.api.common.display.basic)) (vectorwing/farmersdelight/integration/rei/client/display/ClientsidedCookingPotDisplay.java:56)
- [ ] `ClientsidedCookingPotDisplay.ClientsidedCookingPotDisplay(6)`: its class isn't translated (it extends `BasicDisplay` (me.shedaniel.rei.api.common.display.basic)) (vectorwing/farmersdelight/integration/rei/client/display/ClientsidedCookingPotDisplay.java:67)
- [ ] `ClientsidedCookingPotDisplay.getOutputContainer(0)`: its class isn't translated (it extends `BasicDisplay` (me.shedaniel.rei.api.common.display.basic)) (vectorwing/farmersdelight/integration/rei/client/display/ClientsidedCookingPotDisplay.java:82)
- [ ] `ClientsidedCookingPotDisplay.getCookTime(0)`: its class isn't translated (it extends `BasicDisplay` (me.shedaniel.rei.api.common.display.basic)) (vectorwing/farmersdelight/integration/rei/client/display/ClientsidedCookingPotDisplay.java:86)
- [ ] `ClientsidedCookingPotDisplay.getExperience(0)`: its class isn't translated (it extends `BasicDisplay` (me.shedaniel.rei.api.common.display.basic)) (vectorwing/farmersdelight/integration/rei/client/display/ClientsidedCookingPotDisplay.java:90)
- [ ] `ClientsidedCookingPotDisplay.isShapeless(0)`: its class isn't translated (it extends `BasicDisplay` (me.shedaniel.rei.api.common.display.basic)) (vectorwing/farmersdelight/integration/rei/client/display/ClientsidedCookingPotDisplay.java:94)
- [ ] `ClientsidedCookingPotDisplay.getWidth(0)`: its class isn't translated (it extends `BasicDisplay` (me.shedaniel.rei.api.common.display.basic)) (vectorwing/farmersdelight/integration/rei/client/display/ClientsidedCookingPotDisplay.java:98)
- [ ] `ClientsidedCookingPotDisplay.getHeight(0)`: its class isn't translated (it extends `BasicDisplay` (me.shedaniel.rei.api.common.display.basic)) (vectorwing/farmersdelight/integration/rei/client/display/ClientsidedCookingPotDisplay.java:102)
- [ ] `ClientsidedCookingPotDisplay.getCategoryIdentifier(0)`: its class isn't translated (it extends `BasicDisplay` (me.shedaniel.rei.api.common.display.basic)) (vectorwing/farmersdelight/integration/rei/client/display/ClientsidedCookingPotDisplay.java:106)
- [ ] `ClientsidedCookingPotDisplay.getSerializer(0)`: its class isn't translated (it extends `BasicDisplay` (me.shedaniel.rei.api.common.display.basic)) (vectorwing/farmersdelight/integration/rei/client/display/ClientsidedCookingPotDisplay.java:110)
- [ ] `ClientsidedCookingPotDisplay.recipeDisplayId(0)`: its class isn't translated (it extends `BasicDisplay` (me.shedaniel.rei.api.common.display.basic)) (vectorwing/farmersdelight/integration/rei/client/display/ClientsidedCookingPotDisplay.java:114)

### `CommonREIPlugin`: it implements `REICommonPlugin` (me.shedaniel.rei.api.common.plugins)

- [ ] `CommonREIPlugin.registerDisplaySerializer(1)`: its class isn't translated (it implements `REICommonPlugin` (me.shedaniel.rei.api.common.plugins)) (vectorwing/farmersdelight/integration/rei/CommonREIPlugin.java:28)
- [ ] `CommonREIPlugin.registerDisplays(1)`: its class isn't translated (it implements `REICommonPlugin` (me.shedaniel.rei.api.common.plugins)) (vectorwing/farmersdelight/integration/rei/CommonREIPlugin.java:34)

### `CookingPotDisplay`: it extends `BasicDisplay` (me.shedaniel.rei.api.common.display.basic)

- [ ] `CookingPotDisplay.SERIALIZER`: it uses `DisplaySerializer` (me.shedaniel.rei.api.common.display) (vectorwing/farmersdelight/integration/rei/display/CookingPotDisplay.java:26)
- [ ] `CookingPotDisplay.CookingPotDisplay(1)`: its class isn't translated (it extends `BasicDisplay` (me.shedaniel.rei.api.common.display.basic)) (vectorwing/farmersdelight/integration/rei/display/CookingPotDisplay.java:55)
- [ ] `CookingPotDisplay.CookingPotDisplay(6)`: its class isn't translated (it extends `BasicDisplay` (me.shedaniel.rei.api.common.display.basic)) (vectorwing/farmersdelight/integration/rei/display/CookingPotDisplay.java:66)
- [ ] `CookingPotDisplay.getOutputContainer(0)`: its class isn't translated (it extends `BasicDisplay` (me.shedaniel.rei.api.common.display.basic)) (vectorwing/farmersdelight/integration/rei/display/CookingPotDisplay.java:75)
- [ ] `CookingPotDisplay.getCookTime(0)`: its class isn't translated (it extends `BasicDisplay` (me.shedaniel.rei.api.common.display.basic)) (vectorwing/farmersdelight/integration/rei/display/CookingPotDisplay.java:79)
- [ ] `CookingPotDisplay.getExperience(0)`: its class isn't translated (it extends `BasicDisplay` (me.shedaniel.rei.api.common.display.basic)) (vectorwing/farmersdelight/integration/rei/display/CookingPotDisplay.java:83)
- [ ] `CookingPotDisplay.getCategoryIdentifier(0)`: its class isn't translated (it extends `BasicDisplay` (me.shedaniel.rei.api.common.display.basic)) (vectorwing/farmersdelight/integration/rei/display/CookingPotDisplay.java:87)
- [ ] `CookingPotDisplay.getSerializer(0)`: its class isn't translated (it extends `BasicDisplay` (me.shedaniel.rei.api.common.display.basic)) (vectorwing/farmersdelight/integration/rei/display/CookingPotDisplay.java:91)
- [ ] `CookingPotDisplay.isShapeless(0)`: its class isn't translated (it extends `BasicDisplay` (me.shedaniel.rei.api.common.display.basic)) (vectorwing/farmersdelight/integration/rei/display/CookingPotDisplay.java:95)
- [ ] `CookingPotDisplay.getWidth(0)`: its class isn't translated (it extends `BasicDisplay` (me.shedaniel.rei.api.common.display.basic)) (vectorwing/farmersdelight/integration/rei/display/CookingPotDisplay.java:99)
- [ ] `CookingPotDisplay.getHeight(0)`: its class isn't translated (it extends `BasicDisplay` (me.shedaniel.rei.api.common.display.basic)) (vectorwing/farmersdelight/integration/rei/display/CookingPotDisplay.java:103)

### `CuttingDisplay`: it extends `BasicDisplay` (me.shedaniel.rei.api.common.display.basic)

- [ ] `CuttingDisplay.CHANCE_RESULTS_STREAM_CODEC`: it uses Minecraft's `StreamCodec` (vectorwing/farmersdelight/integration/rei/display/CuttingDisplay.java:27)
- [ ] `CuttingDisplay.SERIALIZER`: it uses `DisplaySerializer` (me.shedaniel.rei.api.common.display) (vectorwing/farmersdelight/integration/rei/display/CuttingDisplay.java:30)
- [ ] `CuttingDisplay.CuttingDisplay(1)`: its class isn't translated (it extends `BasicDisplay` (me.shedaniel.rei.api.common.display.basic)) (vectorwing/farmersdelight/integration/rei/display/CuttingDisplay.java:56)
- [ ] `CuttingDisplay.CuttingDisplay(5)`: its class isn't translated (it extends `BasicDisplay` (me.shedaniel.rei.api.common.display.basic)) (vectorwing/farmersdelight/integration/rei/display/CuttingDisplay.java:70)
- [ ] `CuttingDisplay.getTool(0)`: its class isn't translated (it extends `BasicDisplay` (me.shedaniel.rei.api.common.display.basic)) (vectorwing/farmersdelight/integration/rei/display/CuttingDisplay.java:82)
- [ ] `CuttingDisplay.getRollableOutputs(0)`: its class isn't translated (it extends `BasicDisplay` (me.shedaniel.rei.api.common.display.basic)) (vectorwing/farmersdelight/integration/rei/display/CuttingDisplay.java:86)
- [ ] `CuttingDisplay.getCategoryIdentifier(0)`: its class isn't translated (it extends `BasicDisplay` (me.shedaniel.rei.api.common.display.basic)) (vectorwing/farmersdelight/integration/rei/display/CuttingDisplay.java:90)
- [ ] `CuttingDisplay.getSerializer(0)`: its class isn't translated (it extends `BasicDisplay` (me.shedaniel.rei.api.common.display.basic)) (vectorwing/farmersdelight/integration/rei/display/CuttingDisplay.java:94)
- [ ] `CuttingDisplay.isShapeless(0)`: its class isn't translated (it extends `BasicDisplay` (me.shedaniel.rei.api.common.display.basic)) (vectorwing/farmersdelight/integration/rei/display/CuttingDisplay.java:98)
- [ ] `CuttingDisplay.getWidth(0)`: its class isn't translated (it extends `BasicDisplay` (me.shedaniel.rei.api.common.display.basic)) (vectorwing/farmersdelight/integration/rei/display/CuttingDisplay.java:102)
- [ ] `CuttingDisplay.getHeight(0)`: its class isn't translated (it extends `BasicDisplay` (me.shedaniel.rei.api.common.display.basic)) (vectorwing/farmersdelight/integration/rei/display/CuttingDisplay.java:106)

### `DecompositionDisplay`: it extends `BasicDisplay` (me.shedaniel.rei.api.common.display.basic)

- [ ] `DecompositionDisplay.SERIALIZER`: it uses `DisplaySerializer` (me.shedaniel.rei.api.common.display) (vectorwing/farmersdelight/integration/rei/display/DecompositionDisplay.java:17)
- [ ] `DecompositionDisplay.DecompositionDisplay(0)`: its class isn't translated (it extends `BasicDisplay` (me.shedaniel.rei.api.common.display.basic)) (vectorwing/farmersdelight/integration/rei/display/DecompositionDisplay.java:21)
- [ ] `DecompositionDisplay.getCategoryIdentifier(0)`: its class isn't translated (it extends `BasicDisplay` (me.shedaniel.rei.api.common.display.basic)) (vectorwing/farmersdelight/integration/rei/display/DecompositionDisplay.java:25)
- [ ] `DecompositionDisplay.getSerializer(0)`: its class isn't translated (it extends `BasicDisplay` (me.shedaniel.rei.api.common.display.basic)) (vectorwing/farmersdelight/integration/rei/display/DecompositionDisplay.java:29)

### `REICategoryIdentifiers`

- [ ] `REICategoryIdentifiers.COOKING`: it uses `CategoryIdentifier` (me.shedaniel.rei.api.common.category) (vectorwing/farmersdelight/integration/rei/REICategoryIdentifiers.java:10)
- [ ] `REICategoryIdentifiers.CUTTING`: it uses `CategoryIdentifier` (me.shedaniel.rei.api.common.category) (vectorwing/farmersdelight/integration/rei/REICategoryIdentifiers.java:11)
- [ ] `REICategoryIdentifiers.DECOMPOSITION`: it uses `CategoryIdentifier` (me.shedaniel.rei.api.common.category) (vectorwing/farmersdelight/integration/rei/REICategoryIdentifiers.java:12)

### `ServerREIPlugin`: it implements `REICommonPlugin` (me.shedaniel.rei.api.common.plugins)

- [ ] `ServerREIPlugin.registerDisplaySerializer(1)`: its class isn't translated (it implements `REICommonPlugin` (me.shedaniel.rei.api.common.plugins)) (vectorwing/farmersdelight/integration/rei/ServerREIPlugin.java:9)

### `CookingPotClientRecipe`: it implements `ReliableClientRecipe` (cc.cassian.rrv.api.recipe)

- [ ] `CookingPotClientRecipe.CookingPotClientRecipe(6)`: its class isn't translated (it implements `ReliableClientRecipe` (cc.cassian.rrv.api.recipe)) (vectorwing/farmersdelight/integration/rrv/cooking_pot/CookingPotClientRecipe.java:32)
- [ ] `CookingPotClientRecipe.getType(0)`: its class isn't translated (it implements `ReliableClientRecipe` (cc.cassian.rrv.api.recipe)) (vectorwing/farmersdelight/integration/rrv/cooking_pot/CookingPotClientRecipe.java:45)
- [ ] `CookingPotClientRecipe.getId(0)`: its class isn't translated (it implements `ReliableClientRecipe` (cc.cassian.rrv.api.recipe)) (vectorwing/farmersdelight/integration/rrv/cooking_pot/CookingPotClientRecipe.java:49)
- [ ] `CookingPotClientRecipe.bindSlots(1)`: its class isn't translated (it implements `ReliableClientRecipe` (cc.cassian.rrv.api.recipe)) (vectorwing/farmersdelight/integration/rrv/cooking_pot/CookingPotClientRecipe.java:53)
- [ ] `CookingPotClientRecipe.renderRecipe(6)`: its class isn't translated (it implements `ReliableClientRecipe` (cc.cassian.rrv.api.recipe)) (vectorwing/farmersdelight/integration/rrv/cooking_pot/CookingPotClientRecipe.java:62)
- [ ] `CookingPotClientRecipe.getIngredients(0)`: its class isn't translated (it implements `ReliableClientRecipe` (cc.cassian.rrv.api.recipe)) (vectorwing/farmersdelight/integration/rrv/cooking_pot/CookingPotClientRecipe.java:88)
- [ ] `CookingPotClientRecipe.getResults(0)`: its class isn't translated (it implements `ReliableClientRecipe` (cc.cassian.rrv.api.recipe)) (vectorwing/farmersdelight/integration/rrv/cooking_pot/CookingPotClientRecipe.java:92)
- [ ] `CookingPotClientRecipe.supportsItemTransfer(0)`: its class isn't translated (it implements `ReliableClientRecipe` (cc.cassian.rrv.api.recipe)) (vectorwing/farmersdelight/integration/rrv/cooking_pot/CookingPotClientRecipe.java:96)
- [ ] `CookingPotClientRecipe.mapRecipeItems(2)`: its class isn't translated (it implements `ReliableClientRecipe` (cc.cassian.rrv.api.recipe)) (vectorwing/farmersdelight/integration/rrv/cooking_pot/CookingPotClientRecipe.java:100)
- [ ] `CookingPotClientRecipe.getTransferClasses(0)`: its class isn't translated (it implements `ReliableClientRecipe` (cc.cassian.rrv.api.recipe)) (vectorwing/farmersdelight/integration/rrv/cooking_pot/CookingPotClientRecipe.java:108)
- [ ] `CookingPotClientRecipe.canTransferToScreen(1)`: its class isn't translated (it implements `ReliableClientRecipe` (cc.cassian.rrv.api.recipe)) (vectorwing/farmersdelight/integration/rrv/cooking_pot/CookingPotClientRecipe.java:112)

### `CookingPotClientRecipeType`: it implements `ReliableClientRecipeType` (cc.cassian.rrv.api.recipe)

- [ ] `CookingPotClientRecipeType.INSTANCE`: it uses CookingPotClientRecipeType, which isn't translated (it implements `ReliableClientRecipeType` (cc.cassian.rrv.api.recipe)) (vectorwing/farmersdelight/integration/rrv/cooking_pot/CookingPotClientRecipeType.java:15)
- [ ] `CookingPotClientRecipeType.getDisplayName(0)`: its class isn't translated (it implements `ReliableClientRecipeType` (cc.cassian.rrv.api.recipe)) (vectorwing/farmersdelight/integration/rrv/cooking_pot/CookingPotClientRecipeType.java:17)
- [ ] `CookingPotClientRecipeType.getId(0)`: its class isn't translated (it implements `ReliableClientRecipeType` (cc.cassian.rrv.api.recipe)) (vectorwing/farmersdelight/integration/rrv/cooking_pot/CookingPotClientRecipeType.java:21)
- [ ] `CookingPotClientRecipeType.getIcon(0)`: its class isn't translated (it implements `ReliableClientRecipeType` (cc.cassian.rrv.api.recipe)) (vectorwing/farmersdelight/integration/rrv/cooking_pot/CookingPotClientRecipeType.java:25)
- [ ] `CookingPotClientRecipeType.getDisplayWidth(0)`: its class isn't translated (it implements `ReliableClientRecipeType` (cc.cassian.rrv.api.recipe)) (vectorwing/farmersdelight/integration/rrv/cooking_pot/CookingPotClientRecipeType.java:29)
- [ ] `CookingPotClientRecipeType.getDisplayHeight(0)`: its class isn't translated (it implements `ReliableClientRecipeType` (cc.cassian.rrv.api.recipe)) (vectorwing/farmersdelight/integration/rrv/cooking_pot/CookingPotClientRecipeType.java:33)
- [ ] `CookingPotClientRecipeType.getGuiTexture(0)`: its class isn't translated (it implements `ReliableClientRecipeType` (cc.cassian.rrv.api.recipe)) (vectorwing/farmersdelight/integration/rrv/cooking_pot/CookingPotClientRecipeType.java:37)
- [ ] `CookingPotClientRecipeType.getSlotCount(0)`: its class isn't translated (it implements `ReliableClientRecipeType` (cc.cassian.rrv.api.recipe)) (vectorwing/farmersdelight/integration/rrv/cooking_pot/CookingPotClientRecipeType.java:41)
- [ ] `CookingPotClientRecipeType.placeSlots(1)`: its class isn't translated (it implements `ReliableClientRecipeType` (cc.cassian.rrv.api.recipe)) (vectorwing/farmersdelight/integration/rrv/cooking_pot/CookingPotClientRecipeType.java:45)
- [ ] `CookingPotClientRecipeType.getCraftReferences(0)`: its class isn't translated (it implements `ReliableClientRecipeType` (cc.cassian.rrv.api.recipe)) (vectorwing/farmersdelight/integration/rrv/cooking_pot/CookingPotClientRecipeType.java:56)

### `CuttingClientRecipe`: it implements `ReliableClientRecipe` (cc.cassian.rrv.api.recipe)

- [ ] `CuttingClientRecipe.CuttingClientRecipe(5)`: its class isn't translated (it implements `ReliableClientRecipe` (cc.cassian.rrv.api.recipe)) (vectorwing/farmersdelight/integration/rrv/cutting/CuttingClientRecipe.java:33)
- [ ] `CuttingClientRecipe.getType(0)`: its class isn't translated (it implements `ReliableClientRecipe` (cc.cassian.rrv.api.recipe)) (vectorwing/farmersdelight/integration/rrv/cutting/CuttingClientRecipe.java:43)
- [ ] `CuttingClientRecipe.getId(0)`: its class isn't translated (it implements `ReliableClientRecipe` (cc.cassian.rrv.api.recipe)) (vectorwing/farmersdelight/integration/rrv/cutting/CuttingClientRecipe.java:47)
- [ ] `CuttingClientRecipe.bindSlots(1)`: its class isn't translated (it implements `ReliableClientRecipe` (cc.cassian.rrv.api.recipe)) (vectorwing/farmersdelight/integration/rrv/cutting/CuttingClientRecipe.java:51)
- [ ] `CuttingClientRecipe.getIngredients(0)`: its class isn't translated (it implements `ReliableClientRecipe` (cc.cassian.rrv.api.recipe)) (vectorwing/farmersdelight/integration/rrv/cutting/CuttingClientRecipe.java:95)
- [ ] `CuttingClientRecipe.getResults(0)`: its class isn't translated (it implements `ReliableClientRecipe` (cc.cassian.rrv.api.recipe)) (vectorwing/farmersdelight/integration/rrv/cutting/CuttingClientRecipe.java:99)

### `CuttingClientRecipe.CuttingSlotRenderer`: it's an interface

- [ ] `CuttingClientRecipe.CuttingSlotRenderer.render(4)`: it has no body (vectorwing/farmersdelight/integration/rrv/cutting/CuttingClientRecipe.java:112)

### `CuttingClientRecipeType`: it implements `ReliableClientRecipeType` (cc.cassian.rrv.api.recipe)

- [ ] `CuttingClientRecipeType.INSTANCE`: it uses CuttingClientRecipeType, which isn't translated (it implements `ReliableClientRecipeType` (cc.cassian.rrv.api.recipe)) (vectorwing/farmersdelight/integration/rrv/cutting/CuttingClientRecipeType.java:15)
- [ ] `CuttingClientRecipeType.getDisplayName(0)`: its class isn't translated (it implements `ReliableClientRecipeType` (cc.cassian.rrv.api.recipe)) (vectorwing/farmersdelight/integration/rrv/cutting/CuttingClientRecipeType.java:17)
- [ ] `CuttingClientRecipeType.getId(0)`: its class isn't translated (it implements `ReliableClientRecipeType` (cc.cassian.rrv.api.recipe)) (vectorwing/farmersdelight/integration/rrv/cutting/CuttingClientRecipeType.java:21)
- [ ] `CuttingClientRecipeType.getIcon(0)`: its class isn't translated (it implements `ReliableClientRecipeType` (cc.cassian.rrv.api.recipe)) (vectorwing/farmersdelight/integration/rrv/cutting/CuttingClientRecipeType.java:25)
- [ ] `CuttingClientRecipeType.getDisplayWidth(0)`: its class isn't translated (it implements `ReliableClientRecipeType` (cc.cassian.rrv.api.recipe)) (vectorwing/farmersdelight/integration/rrv/cutting/CuttingClientRecipeType.java:29)
- [ ] `CuttingClientRecipeType.getDisplayHeight(0)`: its class isn't translated (it implements `ReliableClientRecipeType` (cc.cassian.rrv.api.recipe)) (vectorwing/farmersdelight/integration/rrv/cutting/CuttingClientRecipeType.java:33)
- [ ] `CuttingClientRecipeType.getGuiTexture(0)`: its class isn't translated (it implements `ReliableClientRecipeType` (cc.cassian.rrv.api.recipe)) (vectorwing/farmersdelight/integration/rrv/cutting/CuttingClientRecipeType.java:37)
- [ ] `CuttingClientRecipeType.getSlotCount(0)`: its class isn't translated (it implements `ReliableClientRecipeType` (cc.cassian.rrv.api.recipe)) (vectorwing/farmersdelight/integration/rrv/cutting/CuttingClientRecipeType.java:41)
- [ ] `CuttingClientRecipeType.placeSlots(1)`: its class isn't translated (it implements `ReliableClientRecipeType` (cc.cassian.rrv.api.recipe)) (vectorwing/farmersdelight/integration/rrv/cutting/CuttingClientRecipeType.java:45)
- [ ] `CuttingClientRecipeType.getCraftReferences(0)`: its class isn't translated (it implements `ReliableClientRecipeType` (cc.cassian.rrv.api.recipe)) (vectorwing/farmersdelight/integration/rrv/cutting/CuttingClientRecipeType.java:54)

### `DecompositionClientRecipe`: it implements `ReliableClientRecipe` (cc.cassian.rrv.api.recipe)

- [ ] `DecompositionClientRecipe.activators`: it uses `SlotContent` (cc.cassian.rrv.common.recipe.inventory) (vectorwing/farmersdelight/integration/rrv/decomposition/DecompositionClientRecipe.java:25)
- [ ] `DecompositionClientRecipe.LIGHT_TOOLTIP`: it uses Minecraft's `MutableComponent` (vectorwing/farmersdelight/integration/rrv/decomposition/DecompositionClientRecipe.java:37)
- [ ] `DecompositionClientRecipe.FLUID_TOOLTIP`: it uses Minecraft's `MutableComponent` (vectorwing/farmersdelight/integration/rrv/decomposition/DecompositionClientRecipe.java:38)
- [ ] `DecompositionClientRecipe.ACCELERATORS_TOOLTIP`: it uses Minecraft's `MutableComponent` (vectorwing/farmersdelight/integration/rrv/decomposition/DecompositionClientRecipe.java:39)
- [ ] `DecompositionClientRecipe.DecompositionClientRecipe(2)`: its class isn't translated (it implements `ReliableClientRecipe` (cc.cassian.rrv.api.recipe)) (vectorwing/farmersdelight/integration/rrv/decomposition/DecompositionClientRecipe.java:41)
- [ ] `DecompositionClientRecipe.getId(0)`: its class isn't translated (it implements `ReliableClientRecipe` (cc.cassian.rrv.api.recipe)) (vectorwing/farmersdelight/integration/rrv/decomposition/DecompositionClientRecipe.java:46)
- [ ] `DecompositionClientRecipe.getType(0)`: its class isn't translated (it implements `ReliableClientRecipe` (cc.cassian.rrv.api.recipe)) (vectorwing/farmersdelight/integration/rrv/decomposition/DecompositionClientRecipe.java:50)
- [ ] `DecompositionClientRecipe.bindSlots(1)`: its class isn't translated (it implements `ReliableClientRecipe` (cc.cassian.rrv.api.recipe)) (vectorwing/farmersdelight/integration/rrv/decomposition/DecompositionClientRecipe.java:54)
- [ ] `DecompositionClientRecipe.renderRecipe(6)`: its class isn't translated (it implements `ReliableClientRecipe` (cc.cassian.rrv.api.recipe)) (vectorwing/farmersdelight/integration/rrv/decomposition/DecompositionClientRecipe.java:60)
- [ ] `DecompositionClientRecipe.createTooltip(1)`: it calls TextUtils.JEI, which isn't translated (vectorwing/farmersdelight/integration/rrv/decomposition/DecompositionClientRecipe.java:76)
- [ ] `DecompositionClientRecipe.getIngredients(0)`: its class isn't translated (it implements `ReliableClientRecipe` (cc.cassian.rrv.api.recipe)) (vectorwing/farmersdelight/integration/rrv/decomposition/DecompositionClientRecipe.java:80)
- [ ] `DecompositionClientRecipe.getResults(0)`: its class isn't translated (it implements `ReliableClientRecipe` (cc.cassian.rrv.api.recipe)) (vectorwing/farmersdelight/integration/rrv/decomposition/DecompositionClientRecipe.java:84)

### `DecompositionClientRecipeType`: it implements `ReliableClientRecipeType` (cc.cassian.rrv.api.recipe)

- [ ] `DecompositionClientRecipeType.INSTANCE`: it uses DecompositionClientRecipeType, which isn't translated (it implements `ReliableClientRecipeType` (cc.cassian.rrv.api.recipe)) (vectorwing/farmersdelight/integration/rrv/decomposition/DecompositionClientRecipeType.java:13)
- [ ] `DecompositionClientRecipeType.getDisplayName(0)`: its class isn't translated (it implements `ReliableClientRecipeType` (cc.cassian.rrv.api.recipe)) (vectorwing/farmersdelight/integration/rrv/decomposition/DecompositionClientRecipeType.java:15)
- [ ] `DecompositionClientRecipeType.getDisplayWidth(0)`: its class isn't translated (it implements `ReliableClientRecipeType` (cc.cassian.rrv.api.recipe)) (vectorwing/farmersdelight/integration/rrv/decomposition/DecompositionClientRecipeType.java:19)
- [ ] `DecompositionClientRecipeType.getDisplayHeight(0)`: its class isn't translated (it implements `ReliableClientRecipeType` (cc.cassian.rrv.api.recipe)) (vectorwing/farmersdelight/integration/rrv/decomposition/DecompositionClientRecipeType.java:23)
- [ ] `DecompositionClientRecipeType.getGuiTexture(0)`: its class isn't translated (it implements `ReliableClientRecipeType` (cc.cassian.rrv.api.recipe)) (vectorwing/farmersdelight/integration/rrv/decomposition/DecompositionClientRecipeType.java:27)
- [ ] `DecompositionClientRecipeType.getSlotCount(0)`: its class isn't translated (it implements `ReliableClientRecipeType` (cc.cassian.rrv.api.recipe)) (vectorwing/farmersdelight/integration/rrv/decomposition/DecompositionClientRecipeType.java:31)
- [ ] `DecompositionClientRecipeType.placeSlots(1)`: its class isn't translated (it implements `ReliableClientRecipeType` (cc.cassian.rrv.api.recipe)) (vectorwing/farmersdelight/integration/rrv/decomposition/DecompositionClientRecipeType.java:35)
- [ ] `DecompositionClientRecipeType.getId(0)`: its class isn't translated (it implements `ReliableClientRecipeType` (cc.cassian.rrv.api.recipe)) (vectorwing/farmersdelight/integration/rrv/decomposition/DecompositionClientRecipeType.java:41)
- [ ] `DecompositionClientRecipeType.getIcon(0)`: its class isn't translated (it implements `ReliableClientRecipeType` (cc.cassian.rrv.api.recipe)) (vectorwing/farmersdelight/integration/rrv/decomposition/DecompositionClientRecipeType.java:45)

### `RRVPlugin`: it implements `ReliableRecipeViewerClientPlugin` (cc.cassian.rrv.api)

- [ ] `RRVPlugin.onIntegrationInitialize(0)`: its class isn't translated (it implements `ReliableRecipeViewerClientPlugin` (cc.cassian.rrv.api)) (vectorwing/farmersdelight/integration/rrv/RRVPlugin.java:23)

### `ReplaceOnRemoval`: it's an interface

- [ ] `ReplaceOnRemoval.onRemove(5)`: it has no body (vectorwing/farmersdelight/refabricated/block/ReplaceOnRemoval.java:8)

### `CanItemPerformAbility`: it implements Minecraft's `LootItemCondition`

- [ ] `CanItemPerformAbility.CODEC`: it uses Minecraft's `MapCodec` (vectorwing/farmersdelight/refabricated/CanItemPerformAbility.java:14)
- [ ] `CanItemPerformAbility.CanItemPerformAbility(1)`: its class isn't translated (it implements Minecraft's `LootItemCondition`) (vectorwing/farmersdelight/refabricated/CanItemPerformAbility.java:13)
- [ ] `CanItemPerformAbility.init(0)`: it calls RegUtils.regLootCond, which isn't translated (vectorwing/farmersdelight/refabricated/CanItemPerformAbility.java:18)
- [ ] `CanItemPerformAbility.canItemPerformAbility(1)`: it uses Minecraft's `Builder` (vectorwing/farmersdelight/refabricated/CanItemPerformAbility.java:22)
- [ ] `CanItemPerformAbility.test(1)`: its class isn't translated (it implements Minecraft's `LootItemCondition`) (vectorwing/farmersdelight/refabricated/CanItemPerformAbility.java:26)
- [ ] `CanItemPerformAbility.getReferencedContextParams(0)`: its class isn't translated (it implements Minecraft's `LootItemCondition`) (vectorwing/farmersdelight/refabricated/CanItemPerformAbility.java:31)
- [ ] `CanItemPerformAbility.codec(0)`: its class isn't translated (it implements Minecraft's `LootItemCondition`) (vectorwing/farmersdelight/refabricated/CanItemPerformAbility.java:35)
- [ ] `CanItemPerformAbility.ability(0)`: its class isn't translated (it implements Minecraft's `LootItemCondition`) (vectorwing/farmersdelight/refabricated/CanItemPerformAbility.java:13)

### `CompostableHelper`

- [x] `CompostableHelper.COMPOSTABLES` (vectorwing/farmersdelight/refabricated/CompostableHelper.java:18)
- [ ] `CompostableHelper.apply(0)`: JSON's asMap (vectorwing/farmersdelight/refabricated/CompostableHelper.java:20)

### `RefabricatedMobEffectTags`: it extends Fabric's `FabricTagsProvider`

- [ ] `RefabricatedMobEffectTags.RefabricatedMobEffectTags(2)`: its class isn't translated (it extends Fabric's `FabricTagsProvider`) (vectorwing/farmersdelight/refabricated/data/RefabricatedMobEffectTags.java:12)
- [ ] `RefabricatedMobEffectTags.addTags(1)`: its class isn't translated (it extends Fabric's `FabricTagsProvider`) (vectorwing/farmersdelight/refabricated/data/RefabricatedMobEffectTags.java:16)

### `SkilletEntityRenderState`: it's an interface

- [ ] `SkilletEntityRenderState.fdrf$getSkilletFlipTimestamp(0)`: it has no body (vectorwing/farmersdelight/refabricated/duck/SkilletEntityRenderState.java:4)
- [ ] `SkilletEntityRenderState.fdrf$setSkilletFlipTimestamp(1)`: it has no body (vectorwing/farmersdelight/refabricated/duck/SkilletEntityRenderState.java:6)

### `FDRecipeBookTypes`

- [ ] `FDRecipeBookTypes.COOKING`: it uses Minecraft's `RecipeBookType` (vectorwing/farmersdelight/refabricated/FDRecipeBookTypes.java:6)

### `FDRefabricatedTags`

- [ ] `FDRefabricatedTags.modBiomeTag(1)`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/refabricated/FDRefabricatedTags.java:13)
- [ ] `FDRefabricatedTags.modBlockTag(1)`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/refabricated/FDRefabricatedTags.java:17)
- [ ] `FDRefabricatedTags.modEntityTag(1)`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/refabricated/FDRefabricatedTags.java:21)
- [ ] `FDRefabricatedTags.modItemTag(1)`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/refabricated/FDRefabricatedTags.java:25)
- [ ] `FDRefabricatedTags.modEffectTag(1)`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/refabricated/FDRefabricatedTags.java:29)

### `FDRefabricatedTags.Biomes`

- [ ] `FDRefabricatedTags.Biomes.HAS_BROWN_MUSHROOM_COLONY`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/refabricated/FDRefabricatedTags.java:34)
- [ ] `FDRefabricatedTags.Biomes.HAS_RED_MUSHROOM_COLONY`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/refabricated/FDRefabricatedTags.java:35)
- [ ] `FDRefabricatedTags.Biomes.HAS_WILD_CABBAGE`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/refabricated/FDRefabricatedTags.java:36)
- [ ] `FDRefabricatedTags.Biomes.HAS_WILD_BEETROOTS`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/refabricated/FDRefabricatedTags.java:37)
- [ ] `FDRefabricatedTags.Biomes.WILD_CARROTS_WHITELIST`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/refabricated/FDRefabricatedTags.java:38)
- [ ] `FDRefabricatedTags.Biomes.WILD_CARROTS_BLACKLIST`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/refabricated/FDRefabricatedTags.java:39)
- [ ] `FDRefabricatedTags.Biomes.WILD_ONIONS_WHITELIST`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/refabricated/FDRefabricatedTags.java:40)
- [ ] `FDRefabricatedTags.Biomes.WILD_ONIONS_BLACKLIST`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/refabricated/FDRefabricatedTags.java:41)
- [ ] `FDRefabricatedTags.Biomes.WILD_POTATOES_WHITELIST`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/refabricated/FDRefabricatedTags.java:42)
- [ ] `FDRefabricatedTags.Biomes.WILD_POTATOES_BLACKLIST`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/refabricated/FDRefabricatedTags.java:43)
- [ ] `FDRefabricatedTags.Biomes.WILD_RICE_WHITELIST`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/refabricated/FDRefabricatedTags.java:44)
- [ ] `FDRefabricatedTags.Biomes.WILD_RICE_BLACKLIST`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/refabricated/FDRefabricatedTags.java:45)
- [ ] `FDRefabricatedTags.Biomes.WILD_TOMATOES_WHITELIST`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/refabricated/FDRefabricatedTags.java:46)
- [ ] `FDRefabricatedTags.Biomes.WILD_TOMATOES_BLACKLIST`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/refabricated/FDRefabricatedTags.java:47)

### `FDRefabricatedTags.Blocks`

- [ ] `FDRefabricatedTags.Blocks.KNIFE_INSTANTLY_MINES`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/refabricated/FDRefabricatedTags.java:51)
- [ ] `FDRefabricatedTags.Blocks.GROWS_MUSHROOM_COLONIES`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/refabricated/FDRefabricatedTags.java:52)
- [ ] `FDRefabricatedTags.Blocks.GROWS_WILD_CROPS`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/refabricated/FDRefabricatedTags.java:53)

### `FDRefabricatedTags.EntityTypes`

- [ ] `FDRefabricatedTags.EntityTypes.DROPS_LEATHER`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/refabricated/FDRefabricatedTags.java:57)

### `FDRefabricatedTags.Items`

- [ ] `FDRefabricatedTags.Items.FLINT_TOOL_MATERIALS`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/refabricated/FDRefabricatedTags.java:61)

### `FDRefabricatedTags.MobEffects`

- [ ] `FDRefabricatedTags.MobEffects.MILK_BOTTLE_IGNORED`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/refabricated/FDRefabricatedTags.java:65)
- [ ] `FDRefabricatedTags.MobEffects.HOT_COCOA_IGNORED`: it uses Minecraft's `TagKey` (vectorwing/farmersdelight/refabricated/FDRefabricatedTags.java:66)

### `FabricWrappedInventory`: it implements `ItemHandler`

- [ ] `FabricWrappedInventory.FabricWrappedInventory(1)`: its class isn't translated (it implements `ItemHandler`) (vectorwing/farmersdelight/refabricated/inventory/FabricWrappedInventory.java:18)
- [ ] `FabricWrappedInventory.getSlot(1)`: its class isn't translated (it implements `ItemHandler`) (vectorwing/farmersdelight/refabricated/inventory/FabricWrappedInventory.java:26)
- [ ] `FabricWrappedInventory.insert(3)`: its class isn't translated (it implements `ItemHandler`) (vectorwing/farmersdelight/refabricated/inventory/FabricWrappedInventory.java:30)
- [ ] `FabricWrappedInventory.extract(3)`: its class isn't translated (it implements `ItemHandler`) (vectorwing/farmersdelight/refabricated/inventory/FabricWrappedInventory.java:53)
- [ ] `FabricWrappedInventory.iterator(0)`: its class isn't translated (it implements `ItemHandler`) (vectorwing/farmersdelight/refabricated/inventory/FabricWrappedInventory.java:76)

### `InvWrapper`: it extends `FabricWrappedInventory`

- [ ] `InvWrapper.InvWrapper(1)`: its class isn't translated (it extends `FabricWrappedInventory`) (vectorwing/farmersdelight/refabricated/inventory/InvWrapper.java:10)
- [ ] `InvWrapper.equals(1)`: its class isn't translated (it extends `FabricWrappedInventory`) (vectorwing/farmersdelight/refabricated/inventory/InvWrapper.java:15)
- [ ] `InvWrapper.getInv(0)`: its class isn't translated (it extends `FabricWrappedInventory`) (vectorwing/farmersdelight/refabricated/inventory/InvWrapper.java:24)
- [ ] `InvWrapper.getSlotCount(0)`: its class isn't translated (it extends `FabricWrappedInventory`) (vectorwing/farmersdelight/refabricated/inventory/InvWrapper.java:28)
- [ ] `InvWrapper.getSlotLimit(1)`: its class isn't translated (it extends `FabricWrappedInventory`) (vectorwing/farmersdelight/refabricated/inventory/InvWrapper.java:33)
- [ ] `InvWrapper.getStackInSlot(1)`: its class isn't translated (it extends `FabricWrappedInventory`) (vectorwing/farmersdelight/refabricated/inventory/InvWrapper.java:38)
- [ ] `InvWrapper.insertItem(3)`: its class isn't translated (it extends `FabricWrappedInventory`) (vectorwing/farmersdelight/refabricated/inventory/InvWrapper.java:43)
- [ ] `InvWrapper.extractItem(3)`: its class isn't translated (it extends `FabricWrappedInventory`) (vectorwing/farmersdelight/refabricated/inventory/InvWrapper.java:107)
- [ ] `InvWrapper.setStackInSlot(2)`: its class isn't translated (it extends `FabricWrappedInventory`) (vectorwing/farmersdelight/refabricated/inventory/InvWrapper.java:134)
- [ ] `InvWrapper.isItemValid(2)`: its class isn't translated (it extends `FabricWrappedInventory`) (vectorwing/farmersdelight/refabricated/inventory/InvWrapper.java:139)

### `ItemHandler`: it's an interface

- [ ] `ItemHandler.getSlotCount(0)`: it has no body (vectorwing/farmersdelight/refabricated/inventory/ItemHandler.java:10)
- [ ] `ItemHandler.getSlotLimit(1)`: it has no body (vectorwing/farmersdelight/refabricated/inventory/ItemHandler.java:12)
- [ ] `ItemHandler.getStackInSlot(1)`: it has no body (vectorwing/farmersdelight/refabricated/inventory/ItemHandler.java:14)
- [ ] `ItemHandler.insertItem(3)`: it has no body (vectorwing/farmersdelight/refabricated/inventory/ItemHandler.java:16)
- [ ] `ItemHandler.extractItem(3)`: it has no body (vectorwing/farmersdelight/refabricated/inventory/ItemHandler.java:18)
- [ ] `ItemHandler.setStackInSlot(2)`: it has no body (vectorwing/farmersdelight/refabricated/inventory/ItemHandler.java:20)
- [ ] `ItemHandler.isItemValid(2)`: it has no body (vectorwing/farmersdelight/refabricated/inventory/ItemHandler.java:22)
- [ ] `ItemHandler.getInputSlotIndexes(0)`: its class isn't translated (it's an interface) (vectorwing/farmersdelight/refabricated/inventory/ItemHandler.java:24)
- [ ] `ItemHandler.commitModifiedStacks(0)`: its class isn't translated (it's an interface) (vectorwing/farmersdelight/refabricated/inventory/ItemHandler.java:28)
- [ ] `ItemHandler.removeItem(2)`: its class isn't translated (it's an interface) (vectorwing/farmersdelight/refabricated/inventory/ItemHandler.java:32)
- [ ] `ItemHandler.insertSlot(4)`: its class isn't translated (it's an interface) (vectorwing/farmersdelight/refabricated/inventory/ItemHandler.java:37)
- [ ] `ItemHandler.extractSlot(4)`: its class isn't translated (it's an interface) (vectorwing/farmersdelight/refabricated/inventory/ItemHandler.java:42)

### `ItemHandlerSlot`: it extends Minecraft's `Slot`

- [ ] `ItemHandlerSlot.EMPTY_INVENTORY`: it uses Minecraft's `Container` (vectorwing/farmersdelight/refabricated/inventory/ItemHandlerSlot.java:9)
- [ ] `ItemHandlerSlot.ItemHandlerSlot(4)`: its class isn't translated (it extends Minecraft's `Slot`) (vectorwing/farmersdelight/refabricated/inventory/ItemHandlerSlot.java:12)
- [ ] `ItemHandlerSlot.mayPlace(1)`: its class isn't translated (it extends Minecraft's `Slot`) (vectorwing/farmersdelight/refabricated/inventory/ItemHandlerSlot.java:17)
- [ ] `ItemHandlerSlot.getItem(0)`: its class isn't translated (it extends Minecraft's `Slot`) (vectorwing/farmersdelight/refabricated/inventory/ItemHandlerSlot.java:21)
- [ ] `ItemHandlerSlot.set(1)`: its class isn't translated (it extends Minecraft's `Slot`) (vectorwing/farmersdelight/refabricated/inventory/ItemHandlerSlot.java:25)
- [ ] `ItemHandlerSlot.remove(1)`: its class isn't translated (it extends Minecraft's `Slot`) (vectorwing/farmersdelight/refabricated/inventory/ItemHandlerSlot.java:30)
- [ ] `ItemHandlerSlot.getMaxStackSize(0)`: its class isn't translated (it extends Minecraft's `Slot`) (vectorwing/farmersdelight/refabricated/inventory/ItemHandlerSlot.java:36)
- [ ] `ItemHandlerSlot.getItemHandler(0)`: its class isn't translated (it extends Minecraft's `Slot`) (vectorwing/farmersdelight/refabricated/inventory/ItemHandlerSlot.java:40)

### `ItemHandlerStackWrapper`: it extends Fabric's `SingleStackStorage`

- [ ] `ItemHandlerStackWrapper.ItemHandlerStackWrapper(2)`: its class isn't translated (it extends Fabric's `SingleStackStorage`) (vectorwing/farmersdelight/refabricated/inventory/ItemHandlerStackWrapper.java:11)
- [ ] `ItemHandlerStackWrapper.getStack(0)`: its class isn't translated (it extends Fabric's `SingleStackStorage`) (vectorwing/farmersdelight/refabricated/inventory/ItemHandlerStackWrapper.java:16)
- [ ] `ItemHandlerStackWrapper.setStack(1)`: its class isn't translated (it extends Fabric's `SingleStackStorage`) (vectorwing/farmersdelight/refabricated/inventory/ItemHandlerStackWrapper.java:20)
- [ ] `ItemHandlerStackWrapper.getCapacity(1)`: its class isn't translated (it extends Fabric's `SingleStackStorage`) (vectorwing/farmersdelight/refabricated/inventory/ItemHandlerStackWrapper.java:24)

### `ItemStackHandler`: it extends `FabricWrappedInventory`

- [ ] `ItemStackHandler.ItemStackHandler(0)`: its class isn't translated (it extends `FabricWrappedInventory`) (vectorwing/farmersdelight/refabricated/inventory/ItemStackHandler.java:12)
- [ ] `ItemStackHandler.ItemStackHandler(1)`: its class isn't translated (it extends `FabricWrappedInventory`) (vectorwing/farmersdelight/refabricated/inventory/ItemStackHandler.java:16)
- [ ] `ItemStackHandler.getSlotCount(0)`: its class isn't translated (it extends `FabricWrappedInventory`) (vectorwing/farmersdelight/refabricated/inventory/ItemStackHandler.java:21)
- [ ] `ItemStackHandler.getSlotLimit(1)`: its class isn't translated (it extends `FabricWrappedInventory`) (vectorwing/farmersdelight/refabricated/inventory/ItemStackHandler.java:26)
- [ ] `ItemStackHandler.getStackLimit(2)`: its class isn't translated (it extends `FabricWrappedInventory`) (vectorwing/farmersdelight/refabricated/inventory/ItemStackHandler.java:31)
- [ ] `ItemStackHandler.isItemValid(2)`: its class isn't translated (it extends `FabricWrappedInventory`) (vectorwing/farmersdelight/refabricated/inventory/ItemStackHandler.java:35)
- [ ] `ItemStackHandler.getStackInSlot(1)`: its class isn't translated (it extends `FabricWrappedInventory`) (vectorwing/farmersdelight/refabricated/inventory/ItemStackHandler.java:40)
- [ ] `ItemStackHandler.insertItem(3)`: its class isn't translated (it extends `FabricWrappedInventory`) (vectorwing/farmersdelight/refabricated/inventory/ItemStackHandler.java:45)
- [ ] `ItemStackHandler.extractItem(3)`: its class isn't translated (it extends `FabricWrappedInventory`) (vectorwing/farmersdelight/refabricated/inventory/ItemStackHandler.java:83)
- [ ] `ItemStackHandler.setStackInSlot(2)`: its class isn't translated (it extends `FabricWrappedInventory`) (vectorwing/farmersdelight/refabricated/inventory/ItemStackHandler.java:113)
- [ ] `ItemStackHandler.serialize(1)`: its class isn't translated (it extends `FabricWrappedInventory`) (vectorwing/farmersdelight/refabricated/inventory/ItemStackHandler.java:119)
- [ ] `ItemStackHandler.deserialize(1)`: its class isn't translated (it extends `FabricWrappedInventory`) (vectorwing/farmersdelight/refabricated/inventory/ItemStackHandler.java:123)
- [ ] `ItemStackHandler.onContentsChanged(1)`: its class isn't translated (it extends `FabricWrappedInventory`) (vectorwing/farmersdelight/refabricated/inventory/ItemStackHandler.java:128)
- [ ] `ItemStackHandler.removeItem(3)`: its class isn't translated (it extends `FabricWrappedInventory`) (vectorwing/farmersdelight/refabricated/inventory/ItemStackHandler.java:131)

### `RecipeWrapper`: it implements Minecraft's `RecipeInput`

- [ ] `RecipeWrapper.RecipeWrapper(1)`: its class isn't translated (it implements Minecraft's `RecipeInput`) (vectorwing/farmersdelight/refabricated/inventory/RecipeWrapper.java:13)
- [ ] `RecipeWrapper.stackedContents(0)`: its class isn't translated (it implements Minecraft's `RecipeInput`) (vectorwing/farmersdelight/refabricated/inventory/RecipeWrapper.java:31)
- [ ] `RecipeWrapper.ingredientAmount(0)`: its class isn't translated (it implements Minecraft's `RecipeInput`) (vectorwing/farmersdelight/refabricated/inventory/RecipeWrapper.java:35)
- [ ] `RecipeWrapper.getItem(1)`: its class isn't translated (it implements Minecraft's `RecipeInput`) (vectorwing/farmersdelight/refabricated/inventory/RecipeWrapper.java:39)
- [ ] `RecipeWrapper.size(0)`: its class isn't translated (it implements Minecraft's `RecipeInput`) (vectorwing/farmersdelight/refabricated/inventory/RecipeWrapper.java:43)

### `ItemAbility`: it implements Minecraft's `StringRepresentable`

- [ ] `ItemAbility.CODEC`: it uses Minecraft's `Codec` (vectorwing/farmersdelight/refabricated/ItemAbility.java:27)
- [ ] `ItemAbility.getSerializedName(0)`: its class isn't translated (it implements Minecraft's `StringRepresentable`) (vectorwing/farmersdelight/refabricated/ItemAbility.java:29)
- [ ] `ItemAbility.canPerformAction(1)`: its class isn't translated (it implements Minecraft's `StringRepresentable`) (vectorwing/farmersdelight/refabricated/ItemAbility.java:33)
- [ ] `ItemAbility.canPerformAction(1)`: its class isn't translated (it implements Minecraft's `StringRepresentable`) (vectorwing/farmersdelight/refabricated/ItemAbility.java:37)

### `LootModificationEvents`

- [ ] `LootModificationEvents.BLOCKS_CAKE`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/refabricated/LootModificationEvents.java:54)
- [ ] `LootModificationEvents.BLOCKS_PUMPKIN`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/refabricated/LootModificationEvents.java:55)
- [ ] `LootModificationEvents.BLOCKS_SHORT_GRASS`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/refabricated/LootModificationEvents.java:56)
- [ ] `LootModificationEvents.BLOCKS_TALL_GRASS`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/refabricated/LootModificationEvents.java:57)
- [ ] `LootModificationEvents.ENTITIES_CAVE_SPIDER`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/refabricated/LootModificationEvents.java:58)
- [ ] `LootModificationEvents.ENTITIES_CHICKEN`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/refabricated/LootModificationEvents.java:59)
- [ ] `LootModificationEvents.ENTITIES_HOGLIN`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/refabricated/LootModificationEvents.java:60)
- [ ] `LootModificationEvents.ENTITIES_PIG`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/refabricated/LootModificationEvents.java:61)
- [ ] `LootModificationEvents.ENTITIES_RABBIT`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/refabricated/LootModificationEvents.java:62)
- [ ] `LootModificationEvents.ENTITIES_SHULKER`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/refabricated/LootModificationEvents.java:63)
- [ ] `LootModificationEvents.ENTITIES_SPIDER`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/refabricated/LootModificationEvents.java:64)
- [ ] `LootModificationEvents.BLOCKS_WHEAT`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/refabricated/LootModificationEvents.java:65)
- [ ] `LootModificationEvents.BLOCKS_APPLE_PIE`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/refabricated/LootModificationEvents.java:66)
- [ ] `LootModificationEvents.BLOCKS_CHOCOLATE_PIE`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/refabricated/LootModificationEvents.java:67)
- [ ] `LootModificationEvents.BLOCKS_PUMPKIN_PIE`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/refabricated/LootModificationEvents.java:68)
- [ ] `LootModificationEvents.BLOCKS_RICE_PANICLES`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/refabricated/LootModificationEvents.java:69)
- [ ] `LootModificationEvents.BLOCKS_SANDY_SHRUB`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/refabricated/LootModificationEvents.java:70)
- [ ] `LootModificationEvents.BLOCKS_SWEET_BERRY_CHEESECAKE`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/refabricated/LootModificationEvents.java:71)
- [ ] `LootModificationEvents.FD_ABANDONED_MINESHAFT`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/refabricated/LootModificationEvents.java:72)
- [ ] `LootModificationEvents.FD_BASTION_HOGLIN_STABLE`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/refabricated/LootModificationEvents.java:73)
- [ ] `LootModificationEvents.FD_BASTION_TREASURE`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/refabricated/LootModificationEvents.java:74)
- [ ] `LootModificationEvents.FD_END_CITY_TREASURE`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/refabricated/LootModificationEvents.java:75)
- [ ] `LootModificationEvents.FD_PILLAGER_OUTPOST`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/refabricated/LootModificationEvents.java:76)
- [ ] `LootModificationEvents.FD_RUINED_PORTAL`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/refabricated/LootModificationEvents.java:77)
- [ ] `LootModificationEvents.FD_SHIPWRECK_SUPPLY`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/refabricated/LootModificationEvents.java:78)
- [ ] `LootModificationEvents.FD_SIMPLE_DUNGEON`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/refabricated/LootModificationEvents.java:79)
- [ ] `LootModificationEvents.FD_VILLAGE_BUTCHER`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/refabricated/LootModificationEvents.java:80)
- [ ] `LootModificationEvents.FD_VILLAGE_DESERT_HOUSE`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/refabricated/LootModificationEvents.java:81)
- [ ] `LootModificationEvents.FD_VILLAGE_PLAINS_HOUSE`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/refabricated/LootModificationEvents.java:82)
- [ ] `LootModificationEvents.FD_VILLAGE_SAVANNA_HOUSE`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/refabricated/LootModificationEvents.java:83)
- [ ] `LootModificationEvents.FD_VILLAGE_SNOWY_HOUSE`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/refabricated/LootModificationEvents.java:84)
- [ ] `LootModificationEvents.FD_VILLAGE_TAIGA_HOUSE`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/refabricated/LootModificationEvents.java:85)
- [ ] `LootModificationEvents.init(0)`: it uses Fabric's `LootTableEvents`.MODIFY (vectorwing/farmersdelight/refabricated/LootModificationEvents.java:87)
- [ ] `LootModificationEvents.modifyDrops(3)`: it uses Minecraft's `Holder` (vectorwing/farmersdelight/refabricated/LootModificationEvents.java:92)
- [ ] `LootModificationEvents.modifyTable(4)`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/refabricated/LootModificationEvents.java:110)
- [ ] `LootModificationEvents.chestLoot(4)`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/refabricated/LootModificationEvents.java:117)
- [ ] `LootModificationEvents.lootTableReference(2)`: it uses Minecraft's `LootPoolEntryContainer` (vectorwing/farmersdelight/refabricated/LootModificationEvents.java:177)
- [ ] `LootModificationEvents.scavengingLoot(4)`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/refabricated/LootModificationEvents.java:181)
- [ ] `LootModificationEvents.slicingLoot(4)`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/refabricated/LootModificationEvents.java:451)
- [ ] `LootModificationEvents.pastrySlicing(6)`: it uses Minecraft's `Builder` (vectorwing/farmersdelight/refabricated/LootModificationEvents.java:475)
- [ ] `LootModificationEvents.straw(4)`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/refabricated/LootModificationEvents.java:508)
- [ ] `LootModificationEvents.strawChance02(2)`: it uses Minecraft's `Builder` (vectorwing/farmersdelight/refabricated/LootModificationEvents.java:562)
- [ ] `LootModificationEvents.strawChance03(2)`: it uses Minecraft's `Builder` (vectorwing/farmersdelight/refabricated/LootModificationEvents.java:580)
- [ ] `LootModificationEvents.vanillaKey(1)`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/refabricated/LootModificationEvents.java:598)
- [ ] `LootModificationEvents.key(1)`: it uses Minecraft's `ResourceKey` (vectorwing/farmersdelight/refabricated/LootModificationEvents.java:602)

### `LootModifier`

- [ ] `LootModifier.LootModifier(1)`: it uses Minecraft's `LootItemCondition` (vectorwing/farmersdelight/refabricated/LootModifier.java:12)
- [ ] `LootModifier.LootModifier(2)`: it uses Minecraft's `LootItemCondition` (vectorwing/farmersdelight/refabricated/LootModifier.java:15)
- [ ] `LootModifier.doApply(2)`: it has no body (vectorwing/farmersdelight/refabricated/LootModifier.java:18)

### `ConfigBuilder`: a function kept in a variable or field (only parameters can be)

- [x] `ConfigBuilder.LOGGER` (vectorwing/farmersdelight/refabricated/mlconfigs/ConfigBuilder.java:19)
- [x] `ConfigBuilder.YACL` (vectorwing/farmersdelight/refabricated/mlconfigs/ConfigBuilder.java:20)
- [x] `ConfigBuilder.CLOTH_CONFIG` (vectorwing/farmersdelight/refabricated/mlconfigs/ConfigBuilder.java:21)
- [ ] `ConfigBuilder.STRING_CHECK`: it uses `Object` (vectorwing/farmersdelight/refabricated/mlconfigs/ConfigBuilder.java:29)
- [ ] `ConfigBuilder.LIST_STRING_CHECK`: it uses `Object` (vectorwing/farmersdelight/refabricated/mlconfigs/ConfigBuilder.java:30)
- [ ] `ConfigBuilder.create(2)`: it uses ConfigBuilder, which isn't translated (a function kept in a variable or field (only parameters can be)) (vectorwing/farmersdelight/refabricated/mlconfigs/ConfigBuilder.java:32)
- [ ] `ConfigBuilder.create(2)`: it uses ConfigBuilder, which isn't translated (a function kept in a variable or field (only parameters can be)) (vectorwing/farmersdelight/refabricated/mlconfigs/ConfigBuilder.java:36)
- [ ] `ConfigBuilder.ConfigBuilder(2)`: its class isn't translated (a function kept in a variable or field (only parameters can be)) (vectorwing/farmersdelight/refabricated/mlconfigs/ConfigBuilder.java:40)
- [ ] `ConfigBuilder.build(0)`: it has no body (vectorwing/farmersdelight/refabricated/mlconfigs/ConfigBuilder.java:45)
- [ ] `ConfigBuilder.getName(0)`: its class isn't translated (a function kept in a variable or field (only parameters can be)) (vectorwing/farmersdelight/refabricated/mlconfigs/ConfigBuilder.java:47)
- [ ] `ConfigBuilder.push(1)`: it has no body (vectorwing/farmersdelight/refabricated/mlconfigs/ConfigBuilder.java:51)
- [ ] `ConfigBuilder.pop(0)`: it has no body (vectorwing/farmersdelight/refabricated/mlconfigs/ConfigBuilder.java:53)
- [ ] `ConfigBuilder.setWriteJsons(0)`: it's generic (vectorwing/farmersdelight/refabricated/mlconfigs/ConfigBuilder.java:55)
- [ ] `ConfigBuilder.define(2)`: it has no body (vectorwing/farmersdelight/refabricated/mlconfigs/ConfigBuilder.java:60)
- [ ] `ConfigBuilder.define(4)`: it has no body (vectorwing/farmersdelight/refabricated/mlconfigs/ConfigBuilder.java:62)
- [ ] `ConfigBuilder.define(4)`: it has no body (vectorwing/farmersdelight/refabricated/mlconfigs/ConfigBuilder.java:64)
- [ ] `ConfigBuilder.define(4)`: it has no body (vectorwing/farmersdelight/refabricated/mlconfigs/ConfigBuilder.java:66)
- [ ] `ConfigBuilder.defineColor(2)`: it has no body (vectorwing/farmersdelight/refabricated/mlconfigs/ConfigBuilder.java:68)
- [ ] `ConfigBuilder.define(3)`: it has no body (vectorwing/farmersdelight/refabricated/mlconfigs/ConfigBuilder.java:70)
- [ ] `ConfigBuilder.define(2)`: it has no body (vectorwing/farmersdelight/refabricated/mlconfigs/ConfigBuilder.java:72)
- [ ] `ConfigBuilder.define(2)`: it has no body (vectorwing/farmersdelight/refabricated/mlconfigs/ConfigBuilder.java:76)
- [ ] `ConfigBuilder.currentCategory(0)`: it has no body (vectorwing/farmersdelight/refabricated/mlconfigs/ConfigBuilder.java:80)
- [ ] `ConfigBuilder.define(3)`: it has no body (vectorwing/farmersdelight/refabricated/mlconfigs/ConfigBuilder.java:82)
- [ ] `ConfigBuilder.define(2)`: it has no body (vectorwing/farmersdelight/refabricated/mlconfigs/ConfigBuilder.java:84)
- [ ] `ConfigBuilder.defineObject(3)`: it has no body (vectorwing/farmersdelight/refabricated/mlconfigs/ConfigBuilder.java:86)
- [ ] `ConfigBuilder.defineObjectList(3)`: it's generic (vectorwing/farmersdelight/refabricated/mlconfigs/ConfigBuilder.java:88)
- [ ] `ConfigBuilder.defineMap(2)`: its class isn't translated (a function kept in a variable or field (only parameters can be)) (vectorwing/farmersdelight/refabricated/mlconfigs/ConfigBuilder.java:92)
- [ ] `ConfigBuilder.defineIDMap(2)`: its class isn't translated (a function kept in a variable or field (only parameters can be)) (vectorwing/farmersdelight/refabricated/mlconfigs/ConfigBuilder.java:96)
- [ ] `ConfigBuilder.defineJson(2)`: it has no body (vectorwing/farmersdelight/refabricated/mlconfigs/ConfigBuilder.java:100)
- [ ] `ConfigBuilder.defineJson(2)`: it has no body (vectorwing/farmersdelight/refabricated/mlconfigs/ConfigBuilder.java:102)
- [ ] `ConfigBuilder.define(2)`: it has no body (vectorwing/farmersdelight/refabricated/mlconfigs/ConfigBuilder.java:104)
- [ ] `ConfigBuilder.description(1)`: its class isn't translated (a function kept in a variable or field (only parameters can be)) (vectorwing/farmersdelight/refabricated/mlconfigs/ConfigBuilder.java:108)
- [ ] `ConfigBuilder.tooltip(1)`: its class isn't translated (a function kept in a variable or field (only parameters can be)) (vectorwing/farmersdelight/refabricated/mlconfigs/ConfigBuilder.java:112)
- [ ] `ConfigBuilder.tooltipKey(1)`: its class isn't translated (a function kept in a variable or field (only parameters can be)) (vectorwing/farmersdelight/refabricated/mlconfigs/ConfigBuilder.java:116)
- [ ] `ConfigBuilder.translationKey(1)`: its class isn't translated (a function kept in a variable or field (only parameters can be)) (vectorwing/farmersdelight/refabricated/mlconfigs/ConfigBuilder.java:120)
- [ ] `ConfigBuilder.comment(1)`: its class isn't translated (a function kept in a variable or field (only parameters can be)) (vectorwing/farmersdelight/refabricated/mlconfigs/ConfigBuilder.java:124)
- [ ] `ConfigBuilder.onChange(1)`: its class isn't translated (a function kept in a variable or field (only parameters can be)) (vectorwing/farmersdelight/refabricated/mlconfigs/ConfigBuilder.java:135)
- [ ] `ConfigBuilder.worldReload(0)`: it has no body (vectorwing/farmersdelight/refabricated/mlconfigs/ConfigBuilder.java:140)
- [ ] `ConfigBuilder.gameRestart(0)`: it has no body (vectorwing/farmersdelight/refabricated/mlconfigs/ConfigBuilder.java:142)
- [ ] `ConfigBuilder.maybeAddTranslationString(1)`: its class isn't translated (a function kept in a variable or field (only parameters can be)) (vectorwing/farmersdelight/refabricated/mlconfigs/ConfigBuilder.java:144)

### `ConfigBuilder.IdentifierConfigValue`: it implements `Supplier` (com.google.common.base)

- [ ] `ConfigBuilder.IdentifierConfigValue.IdentifierConfigValue(3)`: its class isn't translated (it implements `Supplier` (com.google.common.base)) (vectorwing/farmersdelight/refabricated/mlconfigs/ConfigBuilder.java:162)
- [ ] `ConfigBuilder.IdentifierConfigValue.get(0)`: its class isn't translated (it implements `Supplier` (com.google.common.base)) (vectorwing/farmersdelight/refabricated/mlconfigs/ConfigBuilder.java:166)

### `ConfigType`

- [ ] `ConfigType.isSynced(0)`: `this` used as a value (vectorwing/farmersdelight/refabricated/mlconfigs/ConfigType.java:8)
- [ ] `ConfigType.getDefaultName(0)`: `this` used as a value (vectorwing/farmersdelight/refabricated/mlconfigs/ConfigType.java:12)

### `ConfigBuilderImpl`: it extends `ConfigBuilder` (vectorwing.farmersdelight.refabricated.mlconfigs)

- [ ] `ConfigBuilderImpl.create(2)`: it uses ConfigBuilder, which isn't translated (a function kept in a variable or field (only parameters can be)) (vectorwing/farmersdelight/refabricated/mlconfigs/fabric/ConfigBuilderImpl.java:31)
- [ ] `ConfigBuilderImpl.ConfigBuilderImpl(2)`: its class isn't translated (it extends `ConfigBuilder` (vectorwing.farmersdelight.refabricated.mlconfigs)) (vectorwing/farmersdelight/refabricated/mlconfigs/fabric/ConfigBuilderImpl.java:35)
- [ ] `ConfigBuilderImpl.build(0)`: its body couldn't be read: an assert (vectorwing/farmersdelight/refabricated/mlconfigs/fabric/ConfigBuilderImpl.java:40)
- [ ] `ConfigBuilderImpl.currentCategory(0)`: its class isn't translated (it extends `ConfigBuilder` (vectorwing.farmersdelight.refabricated.mlconfigs)) (vectorwing/farmersdelight/refabricated/mlconfigs/fabric/ConfigBuilderImpl.java:45)
- [ ] `ConfigBuilderImpl.push(1)`: its class isn't translated (it extends `ConfigBuilder` (vectorwing.farmersdelight.refabricated.mlconfigs)) (vectorwing/farmersdelight/refabricated/mlconfigs/fabric/ConfigBuilderImpl.java:50)
- [ ] `ConfigBuilderImpl.pop(0)`: its body couldn't be read: an assert (vectorwing/farmersdelight/refabricated/mlconfigs/fabric/ConfigBuilderImpl.java:57)
- [ ] `ConfigBuilderImpl.doAddConfig(2)`: its class isn't translated (it extends `ConfigBuilder` (vectorwing.farmersdelight.refabricated.mlconfigs)) (vectorwing/farmersdelight/refabricated/mlconfigs/fabric/ConfigBuilderImpl.java:63)
- [ ] `ConfigBuilderImpl.define(2)`: its class isn't translated (it extends `ConfigBuilder` (vectorwing.farmersdelight.refabricated.mlconfigs)) (vectorwing/farmersdelight/refabricated/mlconfigs/fabric/ConfigBuilderImpl.java:77)
- [ ] `ConfigBuilderImpl.define(4)`: its class isn't translated (it extends `ConfigBuilder` (vectorwing.farmersdelight.refabricated.mlconfigs)) (vectorwing/farmersdelight/refabricated/mlconfigs/fabric/ConfigBuilderImpl.java:84)
- [ ] `ConfigBuilderImpl.define(4)`: its class isn't translated (it extends `ConfigBuilder` (vectorwing.farmersdelight.refabricated.mlconfigs)) (vectorwing/farmersdelight/refabricated/mlconfigs/fabric/ConfigBuilderImpl.java:91)
- [ ] `ConfigBuilderImpl.define(4)`: its class isn't translated (it extends `ConfigBuilder` (vectorwing.farmersdelight.refabricated.mlconfigs)) (vectorwing/farmersdelight/refabricated/mlconfigs/fabric/ConfigBuilderImpl.java:98)
- [ ] `ConfigBuilderImpl.defineColor(2)`: its class isn't translated (it extends `ConfigBuilder` (vectorwing.farmersdelight.refabricated.mlconfigs)) (vectorwing/farmersdelight/refabricated/mlconfigs/fabric/ConfigBuilderImpl.java:105)
- [ ] `ConfigBuilderImpl.define(3)`: its class isn't translated (it extends `ConfigBuilder` (vectorwing.farmersdelight.refabricated.mlconfigs)) (vectorwing/farmersdelight/refabricated/mlconfigs/fabric/ConfigBuilderImpl.java:112)
- [ ] `ConfigBuilderImpl.define(3)`: its class isn't translated (it extends `ConfigBuilder` (vectorwing.farmersdelight.refabricated.mlconfigs)) (vectorwing/farmersdelight/refabricated/mlconfigs/fabric/ConfigBuilderImpl.java:119)
- [ ] `ConfigBuilderImpl.define(2)`: its class isn't translated (it extends `ConfigBuilder` (vectorwing.farmersdelight.refabricated.mlconfigs)) (vectorwing/farmersdelight/refabricated/mlconfigs/fabric/ConfigBuilderImpl.java:126)
- [ ] `ConfigBuilderImpl.defineJson(2)`: its class isn't translated (it extends `ConfigBuilder` (vectorwing.farmersdelight.refabricated.mlconfigs)) (vectorwing/farmersdelight/refabricated/mlconfigs/fabric/ConfigBuilderImpl.java:133)
- [ ] `ConfigBuilderImpl.defineJson(2)`: its class isn't translated (it extends `ConfigBuilder` (vectorwing.farmersdelight.refabricated.mlconfigs)) (vectorwing/farmersdelight/refabricated/mlconfigs/fabric/ConfigBuilderImpl.java:140)
- [ ] `ConfigBuilderImpl.defineObject(3)`: it's generic (vectorwing/farmersdelight/refabricated/mlconfigs/fabric/ConfigBuilderImpl.java:147)
- [ ] `ConfigBuilderImpl.maybeAddTranslationString(1)`: its class isn't translated (it extends `ConfigBuilder` (vectorwing.farmersdelight.refabricated.mlconfigs)) (vectorwing/farmersdelight/refabricated/mlconfigs/fabric/ConfigBuilderImpl.java:154)
- [ ] `ConfigBuilderImpl.gameRestart(0)`: its class isn't translated (it extends `ConfigBuilder` (vectorwing.farmersdelight.refabricated.mlconfigs)) (vectorwing/farmersdelight/refabricated/mlconfigs/fabric/ConfigBuilderImpl.java:160)
- [ ] `ConfigBuilderImpl.worldReload(0)`: its class isn't translated (it extends `ConfigBuilder` (vectorwing.farmersdelight.refabricated.mlconfigs)) (vectorwing/farmersdelight/refabricated/mlconfigs/fabric/ConfigBuilderImpl.java:165)
- [ ] `ConfigBuilderImpl.comment(1)`: its class isn't translated (it extends `ConfigBuilder` (vectorwing.farmersdelight.refabricated.mlconfigs)) (vectorwing/farmersdelight/refabricated/mlconfigs/fabric/ConfigBuilderImpl.java:170)

### `ConfigEntry`

- [x] `ConfigEntry.ConfigEntry(1)` (vectorwing/farmersdelight/refabricated/mlconfigs/fabric/ConfigEntry.java:8)
- [ ] `ConfigEntry.loadFromJson(1)`: it has no body (vectorwing/farmersdelight/refabricated/mlconfigs/fabric/ConfigEntry.java:12)
- [ ] `ConfigEntry.saveToJson(1)`: it has no body (vectorwing/farmersdelight/refabricated/mlconfigs/fabric/ConfigEntry.java:14)
- [x] `ConfigEntry.getName(0)` (vectorwing/farmersdelight/refabricated/mlconfigs/fabric/ConfigEntry.java:16)

### `ConfigSubCategory`: it extends `ConfigEntry`

- [ ] `ConfigSubCategory.ConfigSubCategory(1)`: its class isn't translated (it extends `ConfigEntry`) (vectorwing/farmersdelight/refabricated/mlconfigs/fabric/ConfigSubCategory.java:11)
- [ ] `ConfigSubCategory.addEntry(1)`: its class isn't translated (it extends `ConfigEntry`) (vectorwing/farmersdelight/refabricated/mlconfigs/fabric/ConfigSubCategory.java:15)
- [ ] `ConfigSubCategory.getEntries(0)`: its class isn't translated (it extends `ConfigEntry`) (vectorwing/farmersdelight/refabricated/mlconfigs/fabric/ConfigSubCategory.java:19)
- [ ] `ConfigSubCategory.loadFromJson(1)`: its class isn't translated (it extends `ConfigEntry`) (vectorwing/farmersdelight/refabricated/mlconfigs/fabric/ConfigSubCategory.java:23)
- [ ] `ConfigSubCategory.saveToJson(1)`: its class isn't translated (it extends `ConfigEntry`) (vectorwing/farmersdelight/refabricated/mlconfigs/fabric/ConfigSubCategory.java:34)

### `FabricConfigHolder`: it extends `ModConfigHolder` (vectorwing.farmersdelight.refabricated.mlconfigs)

- [x] `FabricConfigHolder.GSON` (vectorwing/farmersdelight/refabricated/mlconfigs/fabric/FabricConfigHolder.java:34)
- [ ] `FabricConfigHolder.loadAllConfigs(0)`: it calls FabricConfigHolder.getTrackedSpecs, which isn't in the mod's source (vectorwing/farmersdelight/refabricated/mlconfigs/fabric/FabricConfigHolder.java:39)
- [ ] `FabricConfigHolder.FabricConfigHolder(4)`: its class isn't translated (it extends `ModConfigHolder` (vectorwing.farmersdelight.refabricated.mlconfigs)) (vectorwing/farmersdelight/refabricated/mlconfigs/fabric/FabricConfigHolder.java:46)
- [ ] `FabricConfigHolder.getMainEntry(0)`: its class isn't translated (it extends `ModConfigHolder` (vectorwing.farmersdelight.refabricated.mlconfigs)) (vectorwing/farmersdelight/refabricated/mlconfigs/fabric/FabricConfigHolder.java:55)
- [ ] `FabricConfigHolder.isLoaded(0)`: its class isn't translated (it extends `ModConfigHolder` (vectorwing.farmersdelight.refabricated.mlconfigs)) (vectorwing/farmersdelight/refabricated/mlconfigs/fabric/FabricConfigHolder.java:59)
- [ ] `FabricConfigHolder.forceLoad(0)`: its class isn't translated (it extends `ModConfigHolder` (vectorwing.farmersdelight.refabricated.mlconfigs)) (vectorwing/farmersdelight/refabricated/mlconfigs/fabric/FabricConfigHolder.java:64)
- [ ] `FabricConfigHolder.saveConfig(0)`: its class isn't translated (it extends `ModConfigHolder` (vectorwing.farmersdelight.refabricated.mlconfigs)) (vectorwing/farmersdelight/refabricated/mlconfigs/fabric/FabricConfigHolder.java:94)
- [ ] `FabricConfigHolder.makeScreen(2)`: its class isn't translated (it extends `ModConfigHolder` (vectorwing.farmersdelight.refabricated.mlconfigs)) (vectorwing/farmersdelight/refabricated/mlconfigs/fabric/FabricConfigHolder.java:112)
- [ ] `FabricConfigHolder.hasConfigScreen(0)`: its class isn't translated (it extends `ModConfigHolder` (vectorwing.farmersdelight.refabricated.mlconfigs)) (vectorwing/farmersdelight/refabricated/mlconfigs/fabric/FabricConfigHolder.java:122)
- [ ] `FabricConfigHolder.loadFromBytes(1)`: its class isn't translated (it extends `ModConfigHolder` (vectorwing.farmersdelight.refabricated.mlconfigs)) (vectorwing/farmersdelight/refabricated/mlconfigs/fabric/FabricConfigHolder.java:127)
- [ ] `FabricConfigHolder.onPlayerLoggedIn(3)`: its class isn't translated (it extends `ModConfigHolder` (vectorwing.farmersdelight.refabricated.mlconfigs)) (vectorwing/farmersdelight/refabricated/mlconfigs/fabric/FabricConfigHolder.java:139)

### `FabricConfigListScreen`: it extends Minecraft's `Screen`

- [ ] `FabricConfigListScreen.FabricConfigListScreen(6)`: its class isn't translated (it extends Minecraft's `Screen`) (vectorwing/farmersdelight/refabricated/mlconfigs/fabric/FabricConfigListScreen.java:34)
- [ ] `FabricConfigListScreen.init(0)`: its class isn't translated (it extends Minecraft's `Screen`) (vectorwing/farmersdelight/refabricated/mlconfigs/fabric/FabricConfigListScreen.java:44)
- [ ] `FabricConfigListScreen.addExtraButtons(0)`: its class isn't translated (it extends Minecraft's `Screen`) (vectorwing/farmersdelight/refabricated/mlconfigs/fabric/FabricConfigListScreen.java:50)
- [ ] `FabricConfigListScreen.removed(0)`: its class isn't translated (it extends Minecraft's `Screen`) (vectorwing/farmersdelight/refabricated/mlconfigs/fabric/FabricConfigListScreen.java:58)
- [ ] `FabricConfigListScreen.extractRenderState(4)`: its class isn't translated (it extends Minecraft's `Screen`) (vectorwing/farmersdelight/refabricated/mlconfigs/fabric/FabricConfigListScreen.java:61)
- [ ] `FabricConfigListScreen.isMouseWithin(6)`: its class isn't translated (it extends Minecraft's `Screen`) (vectorwing/farmersdelight/refabricated/mlconfigs/fabric/FabricConfigListScreen.java:75)
- [ ] `FabricConfigListScreen.mouseClicked(2)`: its class isn't translated (it extends Minecraft's `Screen`) (vectorwing/farmersdelight/refabricated/mlconfigs/fabric/FabricConfigListScreen.java:79)
- [ ] `FabricConfigListScreen.onClose(0)`: its class isn't translated (it extends Minecraft's `Screen`) (vectorwing/farmersdelight/refabricated/mlconfigs/fabric/FabricConfigListScreen.java:88)

### `FabricConfigListScreen.ConfigButton`: it extends Minecraft's `Entry`

- [ ] `FabricConfigListScreen.ConfigButton.ConfigButton(1)`: its class isn't translated (it extends Minecraft's `Entry`) (vectorwing/farmersdelight/refabricated/mlconfigs/fabric/FabricConfigListScreen.java:95)
- [ ] `FabricConfigListScreen.ConfigButton.ConfigButton(3)`: its class isn't translated (it extends Minecraft's `Entry`) (vectorwing/farmersdelight/refabricated/mlconfigs/fabric/FabricConfigListScreen.java:99)
- [ ] `FabricConfigListScreen.ConfigButton.extractContent(5)`: its class isn't translated (it extends Minecraft's `Entry`) (vectorwing/farmersdelight/refabricated/mlconfigs/fabric/FabricConfigListScreen.java:110)
- [ ] `FabricConfigListScreen.ConfigButton.children(0)`: its class isn't translated (it extends Minecraft's `Entry`) (vectorwing/farmersdelight/refabricated/mlconfigs/fabric/FabricConfigListScreen.java:117)
- [ ] `FabricConfigListScreen.ConfigButton.narratables(0)`: its class isn't translated (it extends Minecraft's `Entry`) (vectorwing/farmersdelight/refabricated/mlconfigs/fabric/FabricConfigListScreen.java:121)

### `FabricConfigListScreen.ConfigList`: it extends Minecraft's `ContainerObjectSelectionList`

- [ ] `FabricConfigListScreen.ConfigList.ConfigList(6)`: its body couldn't be read: an inner class creation (vectorwing/farmersdelight/refabricated/mlconfigs/fabric/FabricConfigListScreen.java:127)
- [ ] `FabricConfigListScreen.ConfigList.getRowWidth(0)`: its class isn't translated (it extends Minecraft's `ContainerObjectSelectionList`) (vectorwing/farmersdelight/refabricated/mlconfigs/fabric/FabricConfigListScreen.java:136)
- [ ] `FabricConfigListScreen.ConfigList.scrollBarX(0)`: its class isn't translated (it extends Minecraft's `ContainerObjectSelectionList`) (vectorwing/farmersdelight/refabricated/mlconfigs/fabric/FabricConfigListScreen.java:140)

### `BoolConfigValue`: it extends `ConfigValue`

- [ ] `BoolConfigValue.BoolConfigValue(2)`: its class isn't translated (it extends `ConfigValue`) (vectorwing/farmersdelight/refabricated/mlconfigs/fabric/values/BoolConfigValue.java:7)
- [ ] `BoolConfigValue.isValid(1)`: its class isn't translated (it extends `ConfigValue`) (vectorwing/farmersdelight/refabricated/mlconfigs/fabric/values/BoolConfigValue.java:11)
- [ ] `BoolConfigValue.loadFromJson(1)`: its class isn't translated (it extends `ConfigValue`) (vectorwing/farmersdelight/refabricated/mlconfigs/fabric/values/BoolConfigValue.java:15)
- [ ] `BoolConfigValue.saveToJson(1)`: its class isn't translated (it extends `ConfigValue`) (vectorwing/farmersdelight/refabricated/mlconfigs/fabric/values/BoolConfigValue.java:34)

### `ColorConfigValue`: it extends `IntConfigValue`

- [ ] `ColorConfigValue.CODEC`: it uses Minecraft's `Codec` (vectorwing/farmersdelight/refabricated/mlconfigs/fabric/values/ColorConfigValue.java:16)
- [ ] `ColorConfigValue.ColorConfigValue(2)`: its class isn't translated (it extends `IntConfigValue`) (vectorwing/farmersdelight/refabricated/mlconfigs/fabric/values/ColorConfigValue.java:23)
- [ ] `ColorConfigValue.loadFromJson(1)`: its class isn't translated (it extends `IntConfigValue`) (vectorwing/farmersdelight/refabricated/mlconfigs/fabric/values/ColorConfigValue.java:27)
- [ ] `ColorConfigValue.saveToJson(1)`: its class isn't translated (it extends `IntConfigValue`) (vectorwing/farmersdelight/refabricated/mlconfigs/fabric/values/ColorConfigValue.java:48)
- [x] `ColorConfigValue.formatString(1)` (vectorwing/farmersdelight/refabricated/mlconfigs/fabric/values/ColorConfigValue.java:57)
- [ ] `ColorConfigValue.isValidStringOrError(1)`: it uses Minecraft's `DataResult` (vectorwing/farmersdelight/refabricated/mlconfigs/fabric/values/ColorConfigValue.java:61)

### `ConfigValue`: it's generic

- [ ] `ConfigValue.ConfigValue(2)`: its class isn't translated (it's generic) (vectorwing/farmersdelight/refabricated/mlconfigs/fabric/values/ConfigValue.java:15)
- [ ] `ConfigValue.getDefaultValue(0)`: its class isn't translated (it's generic) (vectorwing/farmersdelight/refabricated/mlconfigs/fabric/values/ConfigValue.java:23)
- [ ] `ConfigValue.isValid(1)`: it has no body (vectorwing/farmersdelight/refabricated/mlconfigs/fabric/values/ConfigValue.java:27)
- [ ] `ConfigValue.set(1)`: its class isn't translated (it's generic) (vectorwing/farmersdelight/refabricated/mlconfigs/fabric/values/ConfigValue.java:29)
- [ ] `ConfigValue.get(0)`: its class isn't translated (it's generic) (vectorwing/farmersdelight/refabricated/mlconfigs/fabric/values/ConfigValue.java:33)
- [ ] `ConfigValue.setDescriptionKey(1)`: its class isn't translated (it's generic) (vectorwing/farmersdelight/refabricated/mlconfigs/fabric/values/ConfigValue.java:38)
- [ ] `ConfigValue.setTranslationKey(1)`: its class isn't translated (it's generic) (vectorwing/farmersdelight/refabricated/mlconfigs/fabric/values/ConfigValue.java:42)
- [ ] `ConfigValue.getTranslation(0)`: its class isn't translated (it's generic) (vectorwing/farmersdelight/refabricated/mlconfigs/fabric/values/ConfigValue.java:46)
- [ ] `ConfigValue.getDescription(0)`: its class isn't translated (it's generic) (vectorwing/farmersdelight/refabricated/mlconfigs/fabric/values/ConfigValue.java:50)

### `DoubleConfigValue`: it extends `ConfigValue`

- [ ] `DoubleConfigValue.DoubleConfigValue(4)`: its class isn't translated (it extends `ConfigValue`) (vectorwing/farmersdelight/refabricated/mlconfigs/fabric/values/DoubleConfigValue.java:12)
- [ ] `DoubleConfigValue.isValid(1)`: its class isn't translated (it extends `ConfigValue`) (vectorwing/farmersdelight/refabricated/mlconfigs/fabric/values/DoubleConfigValue.java:19)
- [ ] `DoubleConfigValue.loadFromJson(1)`: its class isn't translated (it extends `ConfigValue`) (vectorwing/farmersdelight/refabricated/mlconfigs/fabric/values/DoubleConfigValue.java:23)
- [ ] `DoubleConfigValue.saveToJson(1)`: its class isn't translated (it extends `ConfigValue`) (vectorwing/farmersdelight/refabricated/mlconfigs/fabric/values/DoubleConfigValue.java:42)
- [ ] `DoubleConfigValue.getMax(0)`: its class isn't translated (it extends `ConfigValue`) (vectorwing/farmersdelight/refabricated/mlconfigs/fabric/values/DoubleConfigValue.java:51)
- [ ] `DoubleConfigValue.getMin(0)`: its class isn't translated (it extends `ConfigValue`) (vectorwing/farmersdelight/refabricated/mlconfigs/fabric/values/DoubleConfigValue.java:55)

### `EnumConfigValue`: it's generic

- [ ] `EnumConfigValue.EnumConfigValue(2)`: its class isn't translated (it's generic) (vectorwing/farmersdelight/refabricated/mlconfigs/fabric/values/EnumConfigValue.java:9)
- [ ] `EnumConfigValue.isValid(1)`: its class isn't translated (it's generic) (vectorwing/farmersdelight/refabricated/mlconfigs/fabric/values/EnumConfigValue.java:14)
- [ ] `EnumConfigValue.getEnumClass(0)`: its class isn't translated (it's generic) (vectorwing/farmersdelight/refabricated/mlconfigs/fabric/values/EnumConfigValue.java:18)
- [ ] `EnumConfigValue.loadFromJson(1)`: its class isn't translated (it's generic) (vectorwing/farmersdelight/refabricated/mlconfigs/fabric/values/EnumConfigValue.java:22)
- [ ] `EnumConfigValue.saveToJson(1)`: its class isn't translated (it's generic) (vectorwing/farmersdelight/refabricated/mlconfigs/fabric/values/EnumConfigValue.java:49)

### `FloatConfigValue`: it extends `ConfigValue`

- [ ] `FloatConfigValue.FloatConfigValue(4)`: its class isn't translated (it extends `ConfigValue`) (vectorwing/farmersdelight/refabricated/mlconfigs/fabric/values/FloatConfigValue.java:12)
- [ ] `FloatConfigValue.isValid(1)`: its class isn't translated (it extends `ConfigValue`) (vectorwing/farmersdelight/refabricated/mlconfigs/fabric/values/FloatConfigValue.java:19)
- [ ] `FloatConfigValue.loadFromJson(1)`: its class isn't translated (it extends `ConfigValue`) (vectorwing/farmersdelight/refabricated/mlconfigs/fabric/values/FloatConfigValue.java:23)
- [ ] `FloatConfigValue.saveToJson(1)`: its class isn't translated (it extends `ConfigValue`) (vectorwing/farmersdelight/refabricated/mlconfigs/fabric/values/FloatConfigValue.java:42)
- [ ] `FloatConfigValue.getMax(0)`: its class isn't translated (it extends `ConfigValue`) (vectorwing/farmersdelight/refabricated/mlconfigs/fabric/values/FloatConfigValue.java:51)
- [ ] `FloatConfigValue.getMin(0)`: its class isn't translated (it extends `ConfigValue`) (vectorwing/farmersdelight/refabricated/mlconfigs/fabric/values/FloatConfigValue.java:55)

### `IntConfigValue`: it extends `ConfigValue`

- [ ] `IntConfigValue.IntConfigValue(4)`: its class isn't translated (it extends `ConfigValue`) (vectorwing/farmersdelight/refabricated/mlconfigs/fabric/values/IntConfigValue.java:12)
- [ ] `IntConfigValue.isValid(1)`: its class isn't translated (it extends `ConfigValue`) (vectorwing/farmersdelight/refabricated/mlconfigs/fabric/values/IntConfigValue.java:19)
- [ ] `IntConfigValue.loadFromJson(1)`: its class isn't translated (it extends `ConfigValue`) (vectorwing/farmersdelight/refabricated/mlconfigs/fabric/values/IntConfigValue.java:23)
- [ ] `IntConfigValue.saveToJson(1)`: its class isn't translated (it extends `ConfigValue`) (vectorwing/farmersdelight/refabricated/mlconfigs/fabric/values/IntConfigValue.java:42)
- [ ] `IntConfigValue.getMax(0)`: its class isn't translated (it extends `ConfigValue`) (vectorwing/farmersdelight/refabricated/mlconfigs/fabric/values/IntConfigValue.java:51)
- [ ] `IntConfigValue.getMin(0)`: its class isn't translated (it extends `ConfigValue`) (vectorwing/farmersdelight/refabricated/mlconfigs/fabric/values/IntConfigValue.java:55)

### `JsonConfigValue`: it extends `ConfigValue`

- [ ] `JsonConfigValue.JsonConfigValue(2)`: its class isn't translated (it extends `ConfigValue`) (vectorwing/farmersdelight/refabricated/mlconfigs/fabric/values/JsonConfigValue.java:12)
- [ ] `JsonConfigValue.isValid(1)`: its class isn't translated (it extends `ConfigValue`) (vectorwing/farmersdelight/refabricated/mlconfigs/fabric/values/JsonConfigValue.java:17)
- [ ] `JsonConfigValue.loadFromJson(1)`: its class isn't translated (it extends `ConfigValue`) (vectorwing/farmersdelight/refabricated/mlconfigs/fabric/values/JsonConfigValue.java:21)
- [ ] `JsonConfigValue.getDefaultValue(0)`: its class isn't translated (it extends `ConfigValue`) (vectorwing/farmersdelight/refabricated/mlconfigs/fabric/values/JsonConfigValue.java:40)
- [ ] `JsonConfigValue.saveToJson(1)`: its class isn't translated (it extends `ConfigValue`) (vectorwing/farmersdelight/refabricated/mlconfigs/fabric/values/JsonConfigValue.java:44)

### `ListStringConfigValue`: it's generic

- [ ] `ListStringConfigValue.ListStringConfigValue(3)`: its class isn't translated (it's generic) (vectorwing/farmersdelight/refabricated/mlconfigs/fabric/values/ListStringConfigValue.java:14)
- [ ] `ListStringConfigValue.isValid(1)`: its class isn't translated (it's generic) (vectorwing/farmersdelight/refabricated/mlconfigs/fabric/values/ListStringConfigValue.java:19)
- [ ] `ListStringConfigValue.loadFromJson(1)`: its class isn't translated (it's generic) (vectorwing/farmersdelight/refabricated/mlconfigs/fabric/values/ListStringConfigValue.java:23)
- [ ] `ListStringConfigValue.saveToJson(1)`: its class isn't translated (it's generic) (vectorwing/farmersdelight/refabricated/mlconfigs/fabric/values/ListStringConfigValue.java:52)

### `ObjectConfigValue`: it's generic

- [ ] `ObjectConfigValue.ObjectConfigValue(3)`: its class isn't translated (it's generic) (vectorwing/farmersdelight/refabricated/mlconfigs/fabric/values/ObjectConfigValue.java:18)
- [ ] `ObjectConfigValue.isValid(1)`: its class isn't translated (it's generic) (vectorwing/farmersdelight/refabricated/mlconfigs/fabric/values/ObjectConfigValue.java:24)
- [ ] `ObjectConfigValue.loadFromJson(1)`: its class isn't translated (it's generic) (vectorwing/farmersdelight/refabricated/mlconfigs/fabric/values/ObjectConfigValue.java:29)
- [ ] `ObjectConfigValue.getDefaultValue(0)`: its class isn't translated (it's generic) (vectorwing/farmersdelight/refabricated/mlconfigs/fabric/values/ObjectConfigValue.java:52)
- [ ] `ObjectConfigValue.saveToJson(1)`: its class isn't translated (it's generic) (vectorwing/farmersdelight/refabricated/mlconfigs/fabric/values/ObjectConfigValue.java:57)

### `StringConfigValue`: it extends `ConfigValue`

- [ ] `StringConfigValue.StringConfigValue(3)`: its class isn't translated (it extends `ConfigValue`) (vectorwing/farmersdelight/refabricated/mlconfigs/fabric/values/StringConfigValue.java:11)
- [ ] `StringConfigValue.isValid(1)`: its class isn't translated (it extends `ConfigValue`) (vectorwing/farmersdelight/refabricated/mlconfigs/fabric/values/StringConfigValue.java:17)
- [ ] `StringConfigValue.loadFromJson(1)`: its class isn't translated (it extends `ConfigValue`) (vectorwing/farmersdelight/refabricated/mlconfigs/fabric/values/StringConfigValue.java:21)
- [ ] `StringConfigValue.saveToJson(1)`: its class isn't translated (it extends `ConfigValue`) (vectorwing/farmersdelight/refabricated/mlconfigs/fabric/values/StringConfigValue.java:40)

### `ClothConfigCompat`

- [ ] `ClothConfigCompat.makeScreen(2)`: it uses Minecraft's `Screen` (vectorwing/farmersdelight/refabricated/mlconfigs/integration/cloth_config/ClothConfigCompat.java:39)
- [ ] `ClothConfigCompat.makeScreen(3)`: it uses Minecraft's `Screen` (vectorwing/farmersdelight/refabricated/mlconfigs/integration/cloth_config/ClothConfigCompat.java:44)
- [ ] `ClothConfigCompat.addEntriesRecursive(3)`: it uses ConfigBuilder, which isn't translated (a function kept in a variable or field (only parameters can be)) (vectorwing/farmersdelight/refabricated/mlconfigs/integration/cloth_config/ClothConfigCompat.java:74)
- [ ] `ClothConfigCompat.buildEntry(2)`: it uses `AbstractConfigListEntry` (me.shedaniel.clothconfig2.api) (vectorwing/farmersdelight/refabricated/mlconfigs/integration/cloth_config/ClothConfigCompat.java:86)
- [ ] `ClothConfigCompat.addEnum(2)`: it's generic (vectorwing/farmersdelight/refabricated/mlconfigs/integration/cloth_config/ClothConfigCompat.java:182)

### `YACLCompat`

- [ ] `YACLCompat.DOUBLE_FORMATTER`: it uses `ValueFormatter` (dev.isxander.yacl3.api.controller) (vectorwing/farmersdelight/refabricated/mlconfigs/integration/yacl/YACLCompat.java:42)
- [ ] `YACLCompat.FLOAT_FORMATTER`: it uses `ValueFormatter` (dev.isxander.yacl3.api.controller) (vectorwing/farmersdelight/refabricated/mlconfigs/integration/yacl/YACLCompat.java:43)
- [ ] `YACLCompat.makeScreen(2)`: it uses Minecraft's `Screen` (vectorwing/farmersdelight/refabricated/mlconfigs/integration/yacl/YACLCompat.java:45)
- [ ] `YACLCompat.makeScreen(3)`: it uses Minecraft's `Screen` (vectorwing/farmersdelight/refabricated/mlconfigs/integration/yacl/YACLCompat.java:49)
- [ ] `YACLCompat.addEntriesRecursive(3)`: it uses `Builder` (dev.isxander.yacl3.api.Option) (vectorwing/farmersdelight/refabricated/mlconfigs/integration/yacl/YACLCompat.java:89)
- [ ] `YACLCompat.buildEntry(1)`: it uses `Option` (dev.isxander.yacl3.api) (vectorwing/farmersdelight/refabricated/mlconfigs/integration/yacl/YACLCompat.java:114)
- [ ] `YACLCompat.addEnum(1)`: it's generic (vectorwing/farmersdelight/refabricated/mlconfigs/integration/yacl/YACLCompat.java:221)

### `ModConfigHolder`: it uses Minecraft's `Component`

- [ ] `ModConfigHolder.CONFIG_STORAGE`: it uses ModConfigHolder, which isn't translated (it uses Minecraft's `Component`) (vectorwing/farmersdelight/refabricated/mlconfigs/ModConfigHolder.java:20)
- [ ] `ModConfigHolder.addTrackedSpec(1)`: it uses ModConfigHolder, which isn't translated (it uses Minecraft's `Component`) (vectorwing/farmersdelight/refabricated/mlconfigs/ModConfigHolder.java:28)
- [ ] `ModConfigHolder.getTrackedSpecs(0)`: it uses ModConfigHolder, which isn't translated (it uses Minecraft's `Component`) (vectorwing/farmersdelight/refabricated/mlconfigs/ModConfigHolder.java:35)
- [ ] `ModConfigHolder.getConfigSpec(1)`: it uses ModConfigHolder, which isn't translated (it uses Minecraft's `Component`) (vectorwing/farmersdelight/refabricated/mlconfigs/ModConfigHolder.java:39)
- [ ] `ModConfigHolder.ModConfigHolder(5)`: its class isn't translated (it uses Minecraft's `Component`) (vectorwing/farmersdelight/refabricated/mlconfigs/ModConfigHolder.java:43)
- [ ] `ModConfigHolder.getReadableName(1)`: String.capitalize with 0 values (vectorwing/farmersdelight/refabricated/mlconfigs/ModConfigHolder.java:53)
- [ ] `ModConfigHolder.getReadableName(0)`: its class isn't translated (it uses Minecraft's `Component`) (vectorwing/farmersdelight/refabricated/mlconfigs/ModConfigHolder.java:57)
- [ ] `ModConfigHolder.onRefresh(0)`: its class isn't translated (it uses Minecraft's `Component`) (vectorwing/farmersdelight/refabricated/mlconfigs/ModConfigHolder.java:61)
- [ ] `ModConfigHolder.isLoaded(0)`: its class isn't translated (it uses Minecraft's `Component`) (vectorwing/farmersdelight/refabricated/mlconfigs/ModConfigHolder.java:67)
- [ ] `ModConfigHolder.forceLoad(0)`: it has no body (vectorwing/farmersdelight/refabricated/mlconfigs/ModConfigHolder.java:71)
- [ ] `ModConfigHolder.getConfigType(0)`: its class isn't translated (it uses Minecraft's `Component`) (vectorwing/farmersdelight/refabricated/mlconfigs/ModConfigHolder.java:73)
- [ ] `ModConfigHolder.getModId(0)`: its class isn't translated (it uses Minecraft's `Component`) (vectorwing/farmersdelight/refabricated/mlconfigs/ModConfigHolder.java:77)
- [ ] `ModConfigHolder.getId(0)`: its class isn't translated (it uses Minecraft's `Component`) (vectorwing/farmersdelight/refabricated/mlconfigs/ModConfigHolder.java:81)
- [ ] `ModConfigHolder.isSynced(0)`: its class isn't translated (it uses Minecraft's `Component`) (vectorwing/farmersdelight/refabricated/mlconfigs/ModConfigHolder.java:85)
- [ ] `ModConfigHolder.getFileName(0)`: its class isn't translated (it uses Minecraft's `Component`) (vectorwing/farmersdelight/refabricated/mlconfigs/ModConfigHolder.java:89)
- [ ] `ModConfigHolder.getFullPath(0)`: its class isn't translated (it uses Minecraft's `Component`) (vectorwing/farmersdelight/refabricated/mlconfigs/ModConfigHolder.java:93)
- [ ] `ModConfigHolder.loadFromBytes(1)`: it has no body (vectorwing/farmersdelight/refabricated/mlconfigs/ModConfigHolder.java:97)
- [ ] `ModConfigHolder.makeScreen(1)`: its class isn't translated (it uses Minecraft's `Component`) (vectorwing/farmersdelight/refabricated/mlconfigs/ModConfigHolder.java:99)
- [ ] `ModConfigHolder.makeScreen(2)`: it has no body (vectorwing/farmersdelight/refabricated/mlconfigs/ModConfigHolder.java:104)
- [ ] `ModConfigHolder.hasConfigScreen(0)`: it has no body (vectorwing/farmersdelight/refabricated/mlconfigs/ModConfigHolder.java:107)
- [ ] `ModConfigHolder.syncConfigsToPlayer(1)`: its class isn't translated (it uses Minecraft's `Component`) (vectorwing/farmersdelight/refabricated/mlconfigs/ModConfigHolder.java:109)
- [ ] `ModConfigHolder.sendSyncedConfigsToAllPlayers(0)`: its class isn't translated (it uses Minecraft's `Component`) (vectorwing/farmersdelight/refabricated/mlconfigs/ModConfigHolder.java:112)

### `ModConfigHolder.ConfigLoadingException`: it extends `RuntimeException`

- [ ] `ModConfigHolder.ConfigLoadingException.ConfigLoadingException(2)`: its class isn't translated (it extends `RuntimeException`) (vectorwing/farmersdelight/refabricated/mlconfigs/ModConfigHolder.java:116)

### `RegUtils`

- [ ] `RegUtils.register(3)`: it's generic (vectorwing/farmersdelight/refabricated/RegUtils.java:33)
- [ ] `RegUtils.regEntity(2)`: it's generic (vectorwing/farmersdelight/refabricated/RegUtils.java:39)
- [ ] `RegUtils.regMenu(2)`: it's generic (vectorwing/farmersdelight/refabricated/RegUtils.java:45)
- [ ] `RegUtils.regPlacementMod(2)`: it's generic (vectorwing/farmersdelight/refabricated/RegUtils.java:49)
- [ ] `RegUtils.regRecipeSerializer(2)`: it's generic (vectorwing/farmersdelight/refabricated/RegUtils.java:55)
- [ ] `RegUtils.regRecipe(2)`: it's generic (vectorwing/farmersdelight/refabricated/RegUtils.java:59)
- [ ] `RegUtils.regParticle(2)`: it's generic (vectorwing/farmersdelight/refabricated/RegUtils.java:63)
- [ ] `RegUtils.regSound(2)`: it's generic (vectorwing/farmersdelight/refabricated/RegUtils.java:67)
- [ ] `RegUtils.regFeature(2)`: it's generic (vectorwing/farmersdelight/refabricated/RegUtils.java:71)
- [ ] `RegUtils.regBlockEntity(2)`: it's generic (vectorwing/farmersdelight/refabricated/RegUtils.java:77)
- [ ] `RegUtils.regTab(2)`: it's generic (vectorwing/farmersdelight/refabricated/RegUtils.java:81)
- [ ] `RegUtils.regComponent(2)`: it's generic (vectorwing/farmersdelight/refabricated/RegUtils.java:85)
- [ ] `RegUtils.regComponent(2)`: it's generic (vectorwing/farmersdelight/refabricated/RegUtils.java:89)
- [ ] `RegUtils.regEnchComponent(2)`: it's generic (vectorwing/farmersdelight/refabricated/RegUtils.java:95)
- [ ] `RegUtils.regEffect(2)`: it's generic (vectorwing/farmersdelight/refabricated/RegUtils.java:101)
- [ ] `RegUtils.regLootFunc(2)`: it's generic (vectorwing/farmersdelight/refabricated/RegUtils.java:105)
- [ ] `RegUtils.regItem(2)`: it's generic (vectorwing/farmersdelight/refabricated/RegUtils.java:111)
- [ ] `RegUtils.regBlock(2)`: it's generic (vectorwing/farmersdelight/refabricated/RegUtils.java:117)
- [ ] `RegUtils.regTrigger(2)`: it's generic (vectorwing/farmersdelight/refabricated/RegUtils.java:123)
- [ ] `RegUtils.regLootCond(2)`: it's generic (vectorwing/farmersdelight/refabricated/RegUtils.java:127)
- [ ] `RegUtils.regRecipeBookCategory(2)`: it's generic (vectorwing/farmersdelight/refabricated/RegUtils.java:131)
- [ ] `RegUtils.regConsumeEffectType(2)`: it's generic (vectorwing/farmersdelight/refabricated/RegUtils.java:135)
- [ ] `RegUtils.regRecipeDisplay(2)`: it's generic (vectorwing/farmersdelight/refabricated/RegUtils.java:139)
