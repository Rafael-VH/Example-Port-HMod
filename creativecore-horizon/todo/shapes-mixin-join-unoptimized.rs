// Not ported yet: the Java @Inject(INVOKE) on Shapes.joinUnoptimized (ShapesMixin.joinUnoptimized).
// Why it's off: `horizon_core::geometry::shape::join::VoxelShape::join_unoptimized` has a `?` in its body: horizon-mixins doesn't rewrite such bodies with BEFORE_CALL. A HEAD hook on it, or a BEFORE_CALL on a function it calls, may work instead.
// horizon-mixins kind: BEFORE_CALL on `horizon_core::geometry::shape::join::VoxelShape::join_unoptimized`
// Note: Only whole statements, plain let initializers and block tails can be selected.
// Note: The call it hooks (Shapes.createIndexMerger) isn't mapped to a Rust function yet.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   BooleanOp.AND -> (not mapped yet)
//   BooleanOp.FIRST -> (not mapped yet)
//   BooleanOp.NOT_AND -> (not mapped yet)
//   BooleanOp.NOT_SAME -> (not mapped yet)
//   BooleanOp.ONLY_FIRST -> (not mapped yet)
//   BooleanOp.ONLY_SECOND -> (not mapped yet)
//   BooleanOp.OR -> (not mapped yet)
//   BooleanOp.SAME -> (not mapped yet)
//   BooleanOp.SECOND -> (not mapped yet)
//   BooleanOp.getClass -> (not mapped yet)
//   Shapes.createIndexMerger -> (not mapped yet)
//   Shapes.joinIsNotEmpty -> horizon_core::geometry::shape::join::VoxelShape::join_is_not_empty (suggested, high)
//   Shapes.joinUnoptimized -> horizon_core::geometry::shape::join::VoxelShape::join_unoptimized (suggested, high)
//   VoxelShape.isEmpty -> (not mapped yet)
//   VoxelShape.toAabbs -> (not mapped yet)
//
// Decompiled Java, team/creative/creativecore/mixin/ShapesMixin.java:23:
// | @Inject(
// |    method = "joinUnoptimized(Lnet/minecraft/world/phys/shapes/VoxelShape;Lnet/minecraft/world/phys/shapes/VoxelShape;Lnet/minecraft/world/phys/shapes/BooleanOp;)Lnet/minecraft/world/phys/shapes/VoxelShape;",
// |    at = @At(
// |       value = "INVOKE",
// |       target = "Lnet/minecraft/world/phys/shapes/Shapes;createIndexMerger(ILit/unimi/dsi/fastutil/doubles/DoubleList;Lit/unimi/dsi/fastutil/doubles/DoubleList;ZZ)Lnet/minecraft/world/phys/shapes/IndexMerger;"
// |    ),
// |    cancellable = true,
// |    require = 1
// | )
// | private static void joinUnoptimized(VoxelShape shape1, VoxelShape shape2, BooleanOp operation, CallbackInfoReturnable<VoxelShape> info) {
// |    if (shape1 instanceof BoxesVoxelShape || shape2 instanceof BoxesVoxelShape) {
// |       if (operation == BooleanOp.AND) {
// |          ABBs bbs = new ABBs(shape1);
// |          if (shape2 instanceof BoxesVoxelShape bb2) {
// |             bbs.intersection(bb2.boxes);
// |          } else {
// |             bbs.intersectionVanilla(shape2.toAabbs());
// |          }
// |
// |          info.setReturnValue(BoxesVoxelShape.create(bbs.getBoxes()));
// |       } else if (operation == BooleanOp.NOT_AND) {
// |          ABBs intersection = new ABBs(shape1);
// |          if (shape2 instanceof BoxesVoxelShape bb2) {
// |             intersection.intersection(bb2.boxes);
// |          } else {
// |             intersection.intersectionVanilla(shape2.toAabbs());
// |          }
// |
// |          ABBs bbs = new ABBs(shape1);
// |          bbs.addShape(shape2);
// |          bbs.cutOut(intersection);
// |          info.setReturnValue(BoxesVoxelShape.create(bbs.getBoxes()));
// |       } else if (operation == BooleanOp.OR) {
// |          ABBs bbs = new ABBs(shape1);
// |          if (shape2 instanceof BoxesVoxelShape bb2) {
// |             bbs.addNonOverlapping(bb2.boxes);
// |          } else {
// |             bbs.addNonOverlappingVanilla(shape2.toAabbs());
// |          }
// |
// |          info.setReturnValue(BoxesVoxelShape.create(bbs.getBoxes()));
// |       } else if (operation == BooleanOp.FIRST) {
// |          if (shape1 instanceof BoxesVoxelShape bb1) {
// |             info.setReturnValue(BoxesVoxelShape.create(bb1.boxes));
// |          } else {
// |             info.setReturnValue(BoxesVoxelShape.createVanilla(shape1.toAabbs()));
// |          }
// |       } else if (operation == BooleanOp.SECOND) {
// |          if (shape2 instanceof BoxesVoxelShape bb2) {
// |             info.setReturnValue(BoxesVoxelShape.create(bb2.boxes));
// |          } else {
// |             info.setReturnValue(BoxesVoxelShape.createVanilla(shape2.toAabbs()));
// |          }
// |       } else if (operation == BooleanOp.ONLY_FIRST) {
// |          ABBs bbs = new ABBs(shape1);
// |          if (shape2 instanceof BoxesVoxelShape bb2) {
// |             bbs.cutOut(bb2.boxes);
// |          } else {
// |             bbs.cutOutVanilla(shape2.toAabbs());
// |          }
// |
// |          info.setReturnValue(BoxesVoxelShape.create(bbs.getBoxes()));
// |       } else if (operation == BooleanOp.ONLY_SECOND) {
// |          ABBs bbs = new ABBs(shape2);
// |          if (shape1 instanceof BoxesVoxelShape bb1) {
// |             bbs.cutOut(bb1.boxes);
// |          } else {
// |             bbs.cutOutVanilla(shape1.toAabbs());
// |          }
// |
// |          info.setReturnValue(BoxesVoxelShape.create(bbs.getBoxes()));
// |       } else {
// |          CreativeCore.LOGGER
// |             .warn(
// |                "Apparentely there are more BooleanOp " + operation + "," + operation.getClass().getName() + ", which might result in horrible performance."
// |             );
// |       }
// |    }
// | }
