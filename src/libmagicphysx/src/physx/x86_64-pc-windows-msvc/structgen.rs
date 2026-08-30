#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxVec2 {
    pub x: f32,
    pub y: f32,
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxVec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxExtendedVec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxVec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxQuat {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxMat33 {
    pub column0: PxVec3,
    pub column1: PxVec3,
    pub column2: PxVec3,
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxMat44 {
    pub column0: PxVec4,
    pub column1: PxVec4,
    pub column2: PxVec4,
    pub column3: PxVec4,
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxTransform {
    pub q: PxQuat,
    pub p: PxVec3,
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxAllocator {
    pub structgen_pad0: [u8; 1],
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxRawAllocator {
    pub structgen_pad0: [u8; 1],
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxUserAllocated {
    pub structgen_pad0: [u8; 1],
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxTempAllocator {
    pub structgen_pad0: [u8; 1],
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxBitAndByte {
    pub structgen_pad0: [u8; 1],
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxBitMap {
    pub structgen_pad0: [u8; 16],
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxVec3Padded {
    pub structgen_pad0: [u8; 12],
    pub padding: u32,
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxTransformPadded {
    pub structgen_pad0: [u8; 28],
    pub padding: u32,
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxBounds3 {
    pub minimum: PxVec3,
    pub maximum: PxVec3,
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxBroadcastingErrorCallback {
    pub structgen_pad0: [u8; 160],
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxFPUGuard {
    pub structgen_pad0: [u8; 32],
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxSIMDGuard {
    pub structgen_pad0: [u8; 8],
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxPlane {
    pub n: PxVec3,
    pub d: f32,
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct Interpolation {
    pub structgen_pad0: [u8; 1],
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxMutexImpl {
    pub structgen_pad0: [u8; 1],
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxReadWriteLock {
    pub structgen_pad0: [u8; 8],
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxProfileScoped {
    pub mCallback: *mut PxProfilerCallback,
    pub mEventName: *const std::ffi::c_char,
    pub mProfilerData: *mut std::ffi::c_void,
    pub mContextId: u64,
    pub mDetached: bool,
    pub structgen_pad0: [u8; 7],
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxSListEntry {
    pub structgen_pad0: [u8; 16],
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxSListImpl {
    pub structgen_pad0: [u8; 1],
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxSyncImpl {
    pub structgen_pad0: [u8; 1],
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxThreadImpl {
    pub structgen_pad0: [u8; 1],
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxCounterFrequencyToTensOfNanos {
    pub mNumerator: u64,
    pub mDenominator: u64,
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxTime {
    pub structgen_pad0: [u8; 8],
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxBoundedData {
    pub data: *const std::ffi::c_void,
    pub stride: u32,
    pub count: u32,
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxDebugPoint {
    pub pos: PxVec3,
    pub color: u32,
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxDebugLine {
    pub pos0: PxVec3,
    pub color0: u32,
    pub pos1: PxVec3,
    pub color1: u32,
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxDebugTriangle {
    pub pos0: PxVec3,
    pub color0: u32,
    pub pos1: PxVec3,
    pub color1: u32,
    pub pos2: PxVec3,
    pub color2: u32,
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxDebugText {
    pub position: PxVec3,
    pub size: f32,
    pub color: u32,
    pub structgen_pad0: [u8; 4],
    pub string: *const std::ffi::c_char,
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxTolerancesScale {
    pub length: f32,
    pub speed: f32,
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxGeometry {
    pub structgen_pad0: [u8; 4],
    pub mTypePadding: f32,
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxBoxGeometry {
    pub structgen_pad0: [u8; 8],
    pub halfExtents: PxVec3,
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxCapsuleGeometry {
    pub structgen_pad0: [u8; 8],
    pub radius: f32,
    pub halfHeight: f32,
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxHullPolygon {
    pub mPlane: [f32; 4],
    pub mNbVerts: u16,
    pub mIndexBase: u16,
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxMeshScale {
    pub scale: PxVec3,
    pub rotation: PxQuat,
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxConvexMeshGeometry {
    pub structgen_pad0: [u8; 8],
    pub scale: PxMeshScale,
    pub structgen_pad1: [u8; 4],
    pub convexMesh: *mut PxConvexMesh,
    pub meshFlags: PxConvexMeshGeometryFlags,
    pub structgen_pad2: [u8; 7],
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxSphereGeometry {
    pub structgen_pad0: [u8; 8],
    pub radius: f32,
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxPlaneGeometry {
    pub structgen_pad0: [u8; 8],
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxTriangleMeshGeometry {
    pub structgen_pad0: [u8; 8],
    pub scale: PxMeshScale,
    pub meshFlags: PxMeshGeometryFlags,
    pub structgen_pad1: [u8; 3],
    pub triangleMesh: *mut PxTriangleMesh,
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxHeightFieldGeometry {
    pub structgen_pad0: [u8; 8],
    pub heightField: *mut PxHeightField,
    pub heightScale: f32,
    pub rowScale: f32,
    pub columnScale: f32,
    pub heightFieldFlags: PxMeshGeometryFlags,
    pub structgen_pad1: [u8; 3],
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxFilterData {
    pub word0: u32,
    pub word1: u32,
    pub word2: u32,
    pub word3: u32,
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxParticleSystemGeometry {
    pub structgen_pad0: [u8; 8],
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxTetrahedronMeshGeometry {
    pub structgen_pad0: [u8; 8],
    pub tetrahedronMesh: *mut PxTetrahedronMesh,
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxQueryHit {
    pub faceIndex: u32,
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxLocationHit {
    pub structgen_pad0: [u8; 4],
    pub flags: PxHitFlags,
    pub structgen_pad1: [u8; 2],
    pub position: PxVec3,
    pub normal: PxVec3,
    pub distance: f32,
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxGeomRaycastHit {
    pub structgen_pad0: [u8; 36],
    pub u: f32,
    pub v: f32,
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxGeomOverlapHit {
    pub structgen_pad0: [u8; 4],
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxGeomSweepHit {
    pub structgen_pad0: [u8; 36],
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxGeomIndexPair {
    pub id0: u32,
    pub id1: u32,
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxGeomIndexClosePair {
    pub structgen_pad0: [u8; 8],
    pub distance: f32,
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxQueryThreadContext {
    pub structgen_pad0: [u8; 1],
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxCustomGeometry {
    pub structgen_pad0: [u8; 16],
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxConvexCoreGeometry {
    pub structgen_pad0: [u8; 40],
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxGeometryHolder {
    pub structgen_pad0: [u8; 56],
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxGeometryQuery {
    pub structgen_pad0: [u8; 1],
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxHeightFieldSample {
    pub height: i16,
    pub materialIndex0: PxBitAndByte,
    pub materialIndex1: PxBitAndByte,
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxHeightFieldDesc {
    pub nbRows: u32,
    pub nbColumns: u32,
    pub format: PxHeightFieldFormat,
    pub structgen_pad0: [u8; 4],
    pub samples: PxBoundedData,
    pub convexEdgeThreshold: f32,
    pub flags: PxHeightFieldFlags,
    pub structgen_pad1: [u8; 2],
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxMeshQuery {
    pub structgen_pad0: [u8; 1],
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxSimpleTriangleMesh {
    pub points: PxBoundedData,
    pub triangles: PxBoundedData,
    pub flags: PxMeshFlags,
    pub structgen_pad0: [u8; 6],
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxTriangle {
    pub verts: [PxVec3; 3],
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxTrianglePadded {
    pub structgen_pad0: [u8; 36],
    pub padding: u32,
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxTetrahedron {
    pub verts: [PxVec3; 4],
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxDeformableVolumeCollisionData {
    pub structgen_pad0: [u8; 1],
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxTetrahedronMeshData {
    pub structgen_pad0: [u8; 1],
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxDeformableVolumeSimulationData {
    pub structgen_pad0: [u8; 1],
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct Px1DConstraintFlag {
    pub structgen_pad0: [u8; 1],
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct Px1DConstraint {
    pub linear0: PxVec3,
    pub geometricError: f32,
    pub angular0: PxVec3,
    pub velocityTarget: f32,
    pub linear1: PxVec3,
    pub minImpulse: f32,
    pub angular1: PxVec3,
    pub maxImpulse: f32,
    pub structgen_pad0: [u8; 8],
    pub flags: u16,
    pub solveHint: u16,
    pub pad: u32,
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxConstraintInvMassScale {
    pub linear0: f32,
    pub angular0: f32,
    pub linear1: f32,
    pub angular1: f32,
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxContactPoint {
    pub normal: PxVec3,
    pub separation: f32,
    pub point: PxVec3,
    pub maxImpulse: f32,
    pub targetVel: PxVec3,
    pub staticFriction: f32,
    pub materialFlags: u8,
    pub structgen_pad0: [u8; 3],
    pub internalFaceIndex1: u32,
    pub dynamicFriction: f32,
    pub restitution: f32,
    pub damping: f32,
    pub structgen_pad1: [u8; 12],
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxSolverBody {
    pub linearVelocity: PxVec3,
    pub maxSolverNormalProgress: u16,
    pub maxSolverFrictionProgress: u16,
    pub angularState: PxVec3,
    pub solverProgress: u32,
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxSolverBodyData {
    pub linearVelocity: PxVec3,
    pub invMass: f32,
    pub angularVelocity: PxVec3,
    pub reportThreshold: f32,
    pub sqrtInvInertia: PxMat33,
    pub penBiasClamp: f32,
    pub nodeIndex: u32,
    pub maxContactImpulse: f32,
    pub body2World: PxTransform,
    pub pad: u32,
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxConstraintBatchHeader {
    pub startIndex: u32,
    pub stride: u16,
    pub constraintType: u16,
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxSolverConstraintDesc {
    pub structgen_pad0: [u8; 16],
    pub bodyADataIndex: u32,
    pub bodyBDataIndex: u32,
    pub linkIndexA: u32,
    pub linkIndexB: u32,
    pub constraint: *mut u8,
    pub writeBack: *mut std::ffi::c_void,
    pub writeBackFriction: *mut std::ffi::c_void,
    pub progressA: u16,
    pub progressB: u16,
    pub structgen_pad1: [u8; 4],
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxSolverConstraintPrepDescBase {
    pub invMassScales: PxConstraintInvMassScale,
    pub desc: *mut PxSolverConstraintDesc,
    pub body0: *const PxSolverBody,
    pub body1: *const PxSolverBody,
    pub data0: *const PxSolverBodyData,
    pub data1: *const PxSolverBodyData,
    pub bodyFrame0: PxTransform,
    pub bodyFrame1: PxTransform,
    pub bodyState0: BodyState,
    pub bodyState1: BodyState,
    pub structgen_pad0: [u8; 8],
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxSolverConstraintPrepDesc {
    pub structgen_pad0: [u8; 128],
    pub rows: *mut Px1DConstraint,
    pub numRows: u32,
    pub linBreakForce: f32,
    pub angBreakForce: f32,
    pub minResponseThreshold: f32,
    pub disablePreprocessing: bool,
    pub improvedSlerp: bool,
    pub driveLimitsAreForces: bool,
    pub extendedLimits: bool,
    pub disableConstraint: bool,
    pub structgen_pad1: [u8; 3],
    pub body0WorldOffset: PxVec3Padded,
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxSolverContactDesc {
    pub structgen_pad0: [u8; 128],
    pub frictionPtr: *mut u8,
    pub contacts: *const PxContactPoint,
    pub numContacts: u32,
    pub frictionCount: u8,
    pub hasMaxImpulse: bool,
    pub disableStrongFriction: bool,
    pub hasForceThresholds: bool,
    pub restDistance: f32,
    pub maxCCDSeparation: f32,
    pub contactForces: *mut f32,
    pub startFrictionPatchIndex: u32,
    pub numFrictionPatches: u32,
    pub startContactPatchIndex: u32,
    pub numContactPatches: u16,
    pub axisConstraintCount: u16,
    pub offsetSlop: f32,
    pub structgen_pad1: [u8; 4],
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxArticulationLimit {
    pub low: f32,
    pub high: f32,
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxPerformanceEnvelope {
    pub maxEffort: f32,
    pub maxActuatorVelocity: f32,
    pub velocityDependentResistance: f32,
    pub speedEffortGradient: f32,
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxJointFrictionParams {
    pub staticFrictionEffort: f32,
    pub dynamicFrictionEffort: f32,
    pub viscousFrictionCoefficient: f32,
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxArticulationDrive {
    pub stiffness: f32,
    pub damping: f32,
    pub maxForce: f32,
    pub envelope: PxPerformanceEnvelope,
    pub driveType: PxArticulationDriveType,
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxTGSSolverBodyVel {
    pub linearVelocity: PxVec3,
    pub maxDynamicPartition: u16,
    pub nbStaticInteractions: u16,
    pub angularVelocity: PxVec3,
    pub partitionMask: u32,
    pub deltaAngDt: PxVec3,
    pub maxAngVel: f32,
    pub deltaLinDt: PxVec3,
    pub lockFlags: u16,
    pub isKinematic: bool,
    pub pad: u8,
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxTGSSolverBodyTxInertia {
    pub deltaBody2WorldQ: PxQuat,
    pub body2WorldP: PxVec3,
    pub sqrtInvInertia: PxMat33,
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxTGSSolverBodyData {
    pub originalLinearVelocity: PxVec3,
    pub maxContactImpulse: f32,
    pub originalAngularVelocity: PxVec3,
    pub penBiasClamp: f32,
    pub invMass: f32,
    pub nodeIndex: u32,
    pub reportThreshold: f32,
    pub pad: u32,
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxTGSSolverConstraintPrepDescBase {
    pub invMassScales: PxConstraintInvMassScale,
    pub desc: *mut PxSolverConstraintDesc,
    pub body0: *const PxTGSSolverBodyVel,
    pub body1: *const PxTGSSolverBodyVel,
    pub body0TxI: *const PxTGSSolverBodyTxInertia,
    pub body1TxI: *const PxTGSSolverBodyTxInertia,
    pub bodyData0: *const PxTGSSolverBodyData,
    pub bodyData1: *const PxTGSSolverBodyData,
    pub bodyFrame0: PxTransform,
    pub bodyFrame1: PxTransform,
    pub structgen_pad0: [u8; 16],
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxTGSSolverConstraintPrepDesc {
    pub structgen_pad0: [u8; 144],
    pub rows: *mut Px1DConstraint,
    pub numRows: u32,
    pub linBreakForce: f32,
    pub angBreakForce: f32,
    pub minResponseThreshold: f32,
    pub disablePreprocessing: bool,
    pub improvedSlerp: bool,
    pub driveLimitsAreForces: bool,
    pub extendedLimits: bool,
    pub disableConstraint: bool,
    pub structgen_pad1: [u8; 3],
    pub body0WorldOffset: PxVec3Padded,
    pub cA2w: PxVec3Padded,
    pub cB2w: PxVec3Padded,
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxTGSSolverContactDesc {
    pub structgen_pad0: [u8; 144],
    pub frictionPtr: *mut u8,
    pub contacts: *const PxContactPoint,
    pub numContacts: u32,
    pub frictionCount: u8,
    pub hasMaxImpulse: bool,
    pub disableStrongFriction: bool,
    pub hasForceThresholds: bool,
    pub restDistance: f32,
    pub maxCCDSeparation: f32,
    pub contactForces: *mut f32,
    pub startFrictionPatchIndex: u32,
    pub numFrictionPatches: u32,
    pub startContactPatchIndex: u32,
    pub numContactPatches: u16,
    pub axisConstraintCount: u16,
    pub maxImpulse: f32,
    pub torsionalPatchRadius: f32,
    pub minTorsionalPatchRadius: f32,
    pub offsetSlop: f32,
    pub structgen_pad1: [u8; 8],
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxArticulationTendonLimit {
    pub lowLimit: f32,
    pub highLimit: f32,
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxSpatialForce {
    pub force: PxVec3,
    pub pad0: f32,
    pub torque: PxVec3,
    pub pad1: f32,
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxSpatialVelocity {
    pub linear: PxVec3,
    pub pad0: f32,
    pub angular: PxVec3,
    pub pad1: f32,
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxArticulationRootLinkData {
    pub transform: PxTransform,
    pub worldLinVel: PxVec3,
    pub worldAngVel: PxVec3,
    pub worldLinAccel: PxVec3,
    pub worldAngAccel: PxVec3,
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxArticulationCache {
    pub externalForces: *mut PxSpatialForce,
    pub denseJacobian: *mut f32,
    pub massMatrix: *mut f32,
    pub coriolisForce: *mut f32,
    pub gravityCompensationForce: *mut f32,
    pub centroidalMomentumMatrix: *mut f32,
    pub centroidalMomentumBias: *mut f32,
    pub jointVelocity: *mut f32,
    pub jointAcceleration: *mut f32,
    pub jointPosition: *mut f32,
    pub jointForce: *mut f32,
    pub jointTargetPositions: *mut f32,
    pub jointTargetVelocities: *mut f32,
    pub linkVelocity: *mut PxSpatialVelocity,
    pub linkAcceleration: *mut PxSpatialVelocity,
    pub linkIncomingJointForce: *mut PxSpatialForce,
    pub linkForce: *mut PxVec3,
    pub linkTorque: *mut PxVec3,
    pub rootLinkData: *mut PxArticulationRootLinkData,
    pub coefficientMatrix: *mut f32,
    pub lambda: *mut f32,
    pub scratchMemory: *mut std::ffi::c_void,
    pub scratchAllocator: *mut std::ffi::c_void,
    pub version: u32,
    pub structgen_pad0: [u8; 4],
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxNodeIndex {
    pub structgen_pad0: [u8; 8],
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxConstraintShaderTable {
    pub solverPrep: *mut std::ffi::c_void,
    pub visualize: *mut std::ffi::c_void,
    pub flag: PxConstraintFlag,
    pub structgen_pad0: [u8; 4],
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxContactPatch {
    pub mMassModification: PxConstraintInvMassScale,
    pub normal: PxVec3,
    pub restitution: f32,
    pub dynamicFriction: f32,
    pub staticFriction: f32,
    pub damping: f32,
    pub startContactIndex: u16,
    pub nbContacts: u8,
    pub materialFlags: u8,
    pub internalFlags: u16,
    pub materialIndex0: u16,
    pub materialIndex1: u16,
    pub pad: [u16; 5],
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxContact {
    pub contact: PxVec3,
    pub separation: f32,
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxExtendedContact {
    pub structgen_pad0: [u8; 16],
    pub targetVelocity: PxVec3,
    pub maxImpulse: f32,
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxModifiableContact {
    pub structgen_pad0: [u8; 32],
    pub normal: PxVec3,
    pub restitution: f32,
    pub materialFlags: u32,
    pub materialIndex0: u16,
    pub materialIndex1: u16,
    pub staticFriction: f32,
    pub dynamicFriction: f32,
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxContactStreamIterator {
    pub zero: PxVec3,
    pub structgen_pad0: [u8; 4],
    pub patch: *const PxContactPatch,
    pub contact: *const PxContact,
    pub faceIndice: *const u32,
    pub totalPatches: u32,
    pub totalContacts: u32,
    pub nextContactIndex: u32,
    pub nextPatchIndex: u32,
    pub contactPatchHeaderSize: u32,
    pub contactPointSize: u32,
    pub mStreamFormat: StreamFormat,
    pub forceNoResponse: u32,
    pub pointStepped: bool,
    pub structgen_pad1: [u8; 3],
    pub hasFaceIndices: u32,
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxFrictionPatch {
    pub anchorPositions: [PxVec3; 2],
    pub anchorImpulses: [PxVec3; 2],
    pub anchorCount: u32,
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxFrictionAnchorStreamIterator {
    pub structgen_pad0: [u8; 32],
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxGpuContactPair {
    pub contactPatches: *mut u8,
    pub contactPoints: *mut u8,
    pub contactForces: *mut f32,
    pub frictionPatches: *mut u8,
    pub transformCacheRef0: u32,
    pub transformCacheRef1: u32,
    pub nodeIndex0: PxNodeIndex,
    pub nodeIndex1: PxNodeIndex,
    pub actor0: *mut PxActor,
    pub actor1: *mut PxActor,
    pub nbContacts: u16,
    pub nbPatches: u16,
    pub structgen_pad0: [u8; 4],
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxContactSet {
    pub structgen_pad0: [u8; 16],
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxContactModifyPair {
    pub actor: [*const PxRigidActor; 2],
    pub shape: [*const PxShape; 2],
    pub transform: [PxTransform; 2],
    pub contacts: PxContactSet,
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxDiffuseParticleParams {
    pub threshold: f32,
    pub lifetime: f32,
    pub airDrag: f32,
    pub bubbleDrag: f32,
    pub buoyancy: f32,
    pub kineticEnergyWeight: f32,
    pub pressureWeight: f32,
    pub divergenceWeight: f32,
    pub collisionDecay: f32,
    pub useAccurateVelocity: bool,
    pub structgen_pad0: [u8; 3],
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxDeformableAttachmentData {
    pub actor: [*mut PxActor; 2],
    pub type_: [PxDeformableAttachmentTargetType; 2],
    pub structgen_pad0: [u8; 64],
    pub pose: [PxTransform; 2],
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxDeformableElementFilterData {
    pub actor: [*mut PxActor; 2],
    pub structgen_pad0: [u8; 64],
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxActorShape {
    pub actor: *mut PxRigidActor,
    pub shape: *mut PxShape,
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxRaycastHit {
    pub structgen_pad0: [u8; 64],
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxOverlapHit {
    pub structgen_pad0: [u8; 24],
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxSweepHit {
    pub structgen_pad0: [u8; 56],
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxRaycastCallback {
    pub structgen_pad0: [u8; 8],
    pub block: PxRaycastHit,
    pub hasBlock: bool,
    pub structgen_pad1: [u8; 7],
    pub touches: *mut PxRaycastHit,
    pub maxNbTouches: u32,
    pub nbTouches: u32,
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxOverlapCallback {
    pub structgen_pad0: [u8; 8],
    pub block: PxOverlapHit,
    pub hasBlock: bool,
    pub structgen_pad1: [u8; 7],
    pub touches: *mut PxOverlapHit,
    pub maxNbTouches: u32,
    pub nbTouches: u32,
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxSweepCallback {
    pub structgen_pad0: [u8; 8],
    pub block: PxSweepHit,
    pub hasBlock: bool,
    pub structgen_pad1: [u8; 7],
    pub touches: *mut PxSweepHit,
    pub maxNbTouches: u32,
    pub nbTouches: u32,
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxRaycastBuffer {
    pub structgen_pad0: [u8; 8],
    pub block: PxRaycastHit,
    pub hasBlock: bool,
    pub structgen_pad1: [u8; 7],
    pub touches: *mut PxRaycastHit,
    pub maxNbTouches: u32,
    pub nbTouches: u32,
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxOverlapBuffer {
    pub structgen_pad0: [u8; 8],
    pub block: PxOverlapHit,
    pub hasBlock: bool,
    pub structgen_pad1: [u8; 7],
    pub touches: *mut PxOverlapHit,
    pub maxNbTouches: u32,
    pub nbTouches: u32,
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxSweepBuffer {
    pub structgen_pad0: [u8; 8],
    pub block: PxSweepHit,
    pub hasBlock: bool,
    pub structgen_pad1: [u8; 7],
    pub touches: *mut PxSweepHit,
    pub maxNbTouches: u32,
    pub nbTouches: u32,
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxQueryCache {
    pub shape: *mut PxShape,
    pub actor: *mut PxRigidActor,
    pub faceIndex: u32,
    pub structgen_pad0: [u8; 4],
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxQueryFilterData {
    pub data: PxFilterData,
    pub flags: PxQueryFlags,
    pub structgen_pad0: [u8; 2],
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxArticulationGPUAPIMaxCounts {
    pub maxDofs: u32,
    pub maxLinks: u32,
    pub maxFixedTendons: u32,
    pub maxFixedTendonJoints: u32,
    pub maxSpatialTendons: u32,
    pub maxSpatialTendonAttachments: u32,
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxSceneQueryDesc {
    pub staticStructure: PxPruningStructureType,
    pub dynamicStructure: PxPruningStructureType,
    pub dynamicTreeRebuildRateHint: u32,
    pub dynamicTreeSecondaryPruner: PxDynamicTreeSecondaryPruner,
    pub staticBVHBuildStrategy: PxBVHBuildStrategy,
    pub dynamicBVHBuildStrategy: PxBVHBuildStrategy,
    pub staticNbObjectsPerNode: u32,
    pub dynamicNbObjectsPerNode: u32,
    pub sceneQueryUpdateMode: PxSceneQueryUpdateMode,
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxBroadPhaseRegion {
    pub mBounds: PxBounds3,
    pub mUserData: *mut std::ffi::c_void,
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxBroadPhaseRegionInfo {
    pub mRegion: PxBroadPhaseRegion,
    pub mNbStaticObjects: u32,
    pub mNbDynamicObjects: u32,
    pub mActive: bool,
    pub mOverlap: bool,
    pub structgen_pad0: [u8; 6],
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxBroadPhaseCaps {
    pub mMaxNbRegions: u32,
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxGpuBroadPhaseDesc {
    pub gpuBroadPhaseNbBitsShiftX: u8,
    pub gpuBroadPhaseNbBitsShiftY: u8,
    pub gpuBroadPhaseNbBitsShiftZ: u8,
    pub gpuBroadPhaseNbBitsEnvIDX: u8,
    pub gpuBroadPhaseNbBitsEnvIDY: u8,
    pub gpuBroadPhaseNbBitsEnvIDZ: u8,
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxBroadPhaseDesc {
    pub mType: PxBroadPhaseType,
    pub structgen_pad0: [u8; 4],
    pub mContextID: u64,
    pub mContextManager: *mut PxCudaContextManager,
    pub mFoundLostPairsCapacity: u32,
    pub mDiscardStaticVsKinematic: bool,
    pub mDiscardKinematicVsKinematic: bool,
    pub structgen_pad1: [u8; 2],
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxBroadPhaseUpdateData {
    pub mCreated: *const u32,
    pub mNbCreated: u32,
    pub structgen_pad0: [u8; 4],
    pub mUpdated: *const u32,
    pub mNbUpdated: u32,
    pub structgen_pad1: [u8; 4],
    pub mRemoved: *const u32,
    pub mNbRemoved: u32,
    pub structgen_pad2: [u8; 4],
    pub mBounds: *const PxBounds3,
    pub mGroups: *const u32,
    pub mDistances: *const f32,
    pub mCapacity: u32,
    pub structgen_pad3: [u8; 4],
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxBroadPhasePair {
    pub mID0: u32,
    pub mID1: u32,
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxBroadPhaseResults {
    pub mNbCreatedPairs: u32,
    pub structgen_pad0: [u8; 4],
    pub mCreatedPairs: *const PxBroadPhasePair,
    pub mNbDeletedPairs: u32,
    pub structgen_pad1: [u8; 4],
    pub mDeletedPairs: *const PxBroadPhasePair,
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxSceneLimits {
    pub maxNbActors: u32,
    pub maxNbBodies: u32,
    pub maxNbStaticShapes: u32,
    pub maxNbDynamicShapes: u32,
    pub maxNbAggregates: u32,
    pub maxNbConstraints: u32,
    pub maxNbRegions: u32,
    pub maxNbBroadPhaseOverlaps: u32,
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxGpuDynamicsMemoryConfig {
    pub tempBufferCapacity: u64,
    pub maxRigidContactCount: u32,
    pub maxRigidPatchCount: u32,
    pub heapCapacity: u32,
    pub foundLostPairsCapacity: u32,
    pub foundLostAggregatePairsCapacity: u32,
    pub totalAggregatePairsCapacity: u32,
    pub maxDeformableSurfaceContacts: u32,
    pub maxDeformableVolumeContacts: u32,
    pub maxParticleContacts: u32,
    pub collisionStackSize: u32,
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxSceneDesc {
    pub structgen_pad0: [u8; 36],
    pub gravity: PxVec3,
    pub simulationEventCallback: *mut PxSimulationEventCallback,
    pub contactModifyCallback: *mut PxContactModifyCallback,
    pub ccdContactModifyCallback: *mut PxCCDContactModifyCallback,
    pub deformableSurfacePostSolveCallback: *mut PxPostSolveCallback,
    pub deformableVolumePostSolveCallback: *mut PxPostSolveCallback,
    pub filterShaderData: *const std::ffi::c_void,
    pub filterShaderDataSize: u32,
    pub structgen_pad1: [u8; 4],
    pub filterShader: *mut std::ffi::c_void,
    pub filterCallback: *mut PxSimulationFilterCallback,
    pub kineKineFilteringMode: PxPairFilteringMode,
    pub staticKineFilteringMode: PxPairFilteringMode,
    pub broadPhaseType: PxBroadPhaseType,
    pub structgen_pad2: [u8; 4],
    pub broadPhaseCallback: *mut PxBroadPhaseCallback,
    pub gpuBroadPhaseDesc: *mut PxGpuBroadPhaseDesc,
    pub limits: PxSceneLimits,
    pub frictionType: PxFrictionType,
    pub solverType: PxSolverType,
    pub bounceThresholdVelocity: f32,
    pub frictionOffsetThreshold: f32,
    pub frictionCorrelationDistance: f32,
    pub flags: PxSceneFlags,
    pub cpuDispatcher: *mut PxCpuDispatcher,
    pub cudaContextManager: *mut PxCudaContextManager,
    pub userData: *mut std::ffi::c_void,
    pub solverBatchSize: u32,
    pub solverArticulationBatchSize: u32,
    pub nbContactDataBlocks: u32,
    pub maxNbContactDataBlocks: u32,
    pub maxBiasCoefficient: f32,
    pub contactReportStreamBufferSize: u32,
    pub ccdMaxPasses: u32,
    pub ccdThreshold: f32,
    pub ccdMaxSeparation: f32,
    pub wakeCounterResetValue: f32,
    pub sanityBounds: PxBounds3,
    pub gpuDynamicsConfig: PxGpuDynamicsMemoryConfig,
    pub gpuMaxNumPartitions: u32,
    pub gpuMaxNumStaticPartitions: u32,
    pub gpuComputeVersion: u32,
    pub contactPairSlabSize: u32,
    pub sceneQuerySystem: *mut PxSceneQuerySystem,
    pub structgen_pad3: [u8; 8],
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxGpuDynamicsMemoryConfigStatistics {
    pub tempBufferCapacity: u64,
    pub rigidContactCount: u32,
    pub rigidPatchCount: u32,
    pub foundLostPairs: u32,
    pub foundLostAggregatePairs: u32,
    pub totalAggregatePairs: u32,
    pub deformableSurfaceContacts: u32,
    pub deformableVolumeContacts: u32,
    pub softbodyContacts: u32,
    pub particleContacts: u32,
    pub collisionStackSize: u32,
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxSimulationStatistics {
    pub nbActiveConstraints: u32,
    pub nbActiveDynamicBodies: u32,
    pub nbActiveKinematicBodies: u32,
    pub nbStaticBodies: u32,
    pub nbDynamicBodies: u32,
    pub nbKinematicBodies: u32,
    pub nbShapes: [u32; 11],
    pub nbAggregates: u32,
    pub nbArticulations: u32,
    pub nbAxisSolverConstraints: u32,
    pub compressedContactSize: u32,
    pub requiredContactConstraintMemory: u32,
    pub peakConstraintMemory: u32,
    pub nbDiscreteContactPairsTotal: u32,
    pub nbDiscreteContactPairsWithCacheHits: u32,
    pub nbDiscreteContactPairsWithContacts: u32,
    pub nbNewPairs: u32,
    pub nbLostPairs: u32,
    pub nbNewTouches: u32,
    pub nbLostTouches: u32,
    pub nbPartitions: u32,
    pub structgen_pad0: [u8; 4],
    pub gpuMemParticles: u64,
    pub gpuMemDeformableSurfaces: u64,
    pub gpuMemDeformableVolumes: u64,
    pub gpuMemHeap: u64,
    pub gpuMemHeapBroadPhase: u64,
    pub gpuMemHeapNarrowPhase: u64,
    pub gpuMemHeapSolver: u64,
    pub gpuMemHeapArticulation: u64,
    pub gpuMemHeapSimulation: u64,
    pub gpuMemHeapSimulationArticulation: u64,
    pub gpuMemHeapSimulationParticles: u64,
    pub gpuMemHeapSimulationDeformableSurface: u64,
    pub gpuMemHeapSimulationDeformableVolume: u64,
    pub gpuMemHeapParticles: u64,
    pub gpuMemHeapDeformableSurfaces: u64,
    pub gpuMemHeapDeformableVolumes: u64,
    pub gpuMemHeapOther: u64,
    pub gpuDynamicsMemoryConfigStatistics: PxGpuDynamicsMemoryConfigStatistics,
    pub nbBroadPhaseAdds: u32,
    pub nbBroadPhaseRemoves: u32,
    pub nbDiscreteContactPairs: [[u32; 11]; 11],
    pub nbCCDPairs: [[u32; 11]; 11],
    pub nbModifiedContactPairs: [[u32; 11]; 11],
    pub nbTriggerPairs: [[u32; 11]; 11],
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxDominanceGroupPair {
    pub dominance0: u8,
    pub dominance1: u8,
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxSceneReadLock {
    pub structgen_pad0: [u8; 8],
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxSceneWriteLock {
    pub structgen_pad0: [u8; 8],
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxContactPairExtraDataItem {
    pub type_: u8,
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxContactPairVelocity {
    pub structgen_pad0: [u8; 4],
    pub linearVelocity: [PxVec3; 2],
    pub angularVelocity: [PxVec3; 2],
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxContactPairPose {
    pub structgen_pad0: [u8; 4],
    pub globalPose: [PxTransform; 2],
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxContactPairIndex {
    pub structgen_pad0: [u8; 2],
    pub index: u16,
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxContactPairExtraDataIterator {
    pub currPtr: *const u8,
    pub endPtr: *const u8,
    pub preSolverVelocity: *const PxContactPairVelocity,
    pub postSolverVelocity: *const PxContactPairVelocity,
    pub eventPose: *const PxContactPairPose,
    pub contactPairIndex: u32,
    pub structgen_pad0: [u8; 4],
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxContactPairHeader {
    pub actors: [*mut PxActor; 2],
    pub extraDataStream: *const u8,
    pub extraDataStreamSize: u16,
    pub flags: PxContactPairHeaderFlags,
    pub structgen_pad0: [u8; 4],
    pub pairs: *const PxContactPair,
    pub nbPairs: u32,
    pub structgen_pad1: [u8; 4],
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxContactPairPoint {
    pub position: PxVec3,
    pub separation: f32,
    pub normal: PxVec3,
    pub internalFaceIndex0: u32,
    pub impulse: PxVec3,
    pub internalFaceIndex1: u32,
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxContactPairFrictionAnchor {
    pub position: PxVec3,
    pub impulse: PxVec3,
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxContactPair {
    pub shapes: [*mut PxShape; 2],
    pub contactPatches: *const u8,
    pub contactPoints: *const u8,
    pub contactImpulses: *const f32,
    pub frictionPatches: *const u8,
    pub requiredBufferSize: u32,
    pub contactCount: u8,
    pub patchCount: u8,
    pub contactStreamSize: u16,
    pub flags: PxContactPairFlags,
    pub events: PxPairFlags,
    pub internalData: [u32; 2],
    pub structgen_pad0: [u8; 4],
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxTriggerPair {
    pub triggerShape: *mut PxShape,
    pub triggerActor: *mut PxActor,
    pub otherShape: *mut PxShape,
    pub otherActor: *mut PxActor,
    pub status: PxPairFlag,
    pub flags: PxTriggerPairFlags,
    pub structgen_pad0: [u8; 3],
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxConstraintInfo {
    pub constraint: *mut PxConstraint,
    pub externalReference: *mut std::ffi::c_void,
    pub type_: u32,
    pub structgen_pad0: [u8; 4],
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxObstacle {
    pub structgen_pad0: [u8; 8],
    pub mUserData: *mut std::ffi::c_void,
    pub mPos: PxExtendedVec3,
    pub mRot: PxQuat,
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxBoxObstacle {
    pub structgen_pad0: [u8; 56],
    pub mHalfExtents: PxVec3,
    pub structgen_pad1: [u8; 4],
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxCapsuleObstacle {
    pub structgen_pad0: [u8; 56],
    pub mHalfHeight: f32,
    pub mRadius: f32,
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxControllerState {
    pub deltaXP: PxVec3,
    pub structgen_pad0: [u8; 4],
    pub touchedShape: *mut PxShape,
    pub touchedActor: *mut PxRigidActor,
    pub touchedObstacleHandle: u32,
    pub collisionFlags: u32,
    pub standOnAnotherCCT: bool,
    pub standOnObstacle: bool,
    pub isMovingUp: bool,
    pub structgen_pad1: [u8; 5],
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxControllerStats {
    pub nbIterations: u16,
    pub nbFullUpdates: u16,
    pub nbPartialUpdates: u16,
    pub nbTessellation: u16,
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxControllerHit {
    pub controller: *mut PxController,
    pub worldPos: PxExtendedVec3,
    pub worldNormal: PxVec3,
    pub dir: PxVec3,
    pub length: f32,
    pub structgen_pad0: [u8; 4],
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxControllerShapeHit {
    pub structgen_pad0: [u8; 64],
    pub shape: *mut PxShape,
    pub actor: *mut PxRigidActor,
    pub triangleIndex: u32,
    pub structgen_pad1: [u8; 4],
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxControllersHit {
    pub structgen_pad0: [u8; 64],
    pub other: *mut PxController,
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxControllerObstacleHit {
    pub structgen_pad0: [u8; 64],
    pub userData: *const std::ffi::c_void,
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxControllerFilters {
    pub mFilterData: *const PxFilterData,
    pub mFilterCallback: *mut PxQueryFilterCallback,
    pub mFilterFlags: PxQueryFlags,
    pub structgen_pad0: [u8; 6],
    pub mCCTFilterCallback: *mut PxControllerFilterCallback,
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxDim3 {
    pub x: u32,
    pub y: u32,
    pub z: u32,
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxSDFDesc {
    pub sdf: PxBoundedData,
    pub dims: PxDim3,
    pub meshLower: PxVec3,
    pub spacing: f32,
    pub subgridSize: u32,
    pub bitsPerSubgridPixel: PxSdfBitsPerSubgridPixel,
    pub sdfSubgrids3DTexBlockDim: PxDim3,
    pub sdfSubgrids: PxBoundedData,
    pub sdfStartSlots: PxBoundedData,
    pub subgridsMinSdfValue: f32,
    pub subgridsMaxSdfValue: f32,
    pub sdfBounds: PxBounds3,
    pub narrowBandThicknessRelativeToSdfBoundsDiagonal: f32,
    pub numThreadsForSdfConstruction: u32,
    pub baseMesh: PxSimpleTriangleMesh,
    pub sdfBuilder: *mut PxSDFBuilder,
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxConvexMeshDesc {
    pub points: PxBoundedData,
    pub polygons: PxBoundedData,
    pub indices: PxBoundedData,
    pub flags: PxConvexFlags,
    pub vertexLimit: u16,
    pub polygonLimit: u16,
    pub quantizedCount: u16,
    pub sdfDesc: *mut PxSDFDesc,
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxTriangleMeshDesc {
    pub structgen_pad0: [u8; 56],
    pub sdfDesc: *mut PxSDFDesc,
    pub geomEpsilon: f32,
    pub structgen_pad1: [u8; 4],
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxTetrahedronMeshDesc {
    pub structgen_pad0: [u8; 16],
    pub points: PxBoundedData,
    pub tetrahedrons: PxBoundedData,
    pub flags: PxMeshFlags,
    pub tetsPerElement: u16,
    pub structgen_pad1: [u8; 4],
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxDeformableVolumeSimulationDataDesc {
    pub vertexToTet: PxBoundedData,
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxBVH33MidphaseDesc {
    pub meshSizePerformanceTradeOff: f32,
    pub meshCookingHint: PxMeshCookingHint,
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxBVH34MidphaseDesc {
    pub numPrimsPerLeaf: u32,
    pub buildStrategy: PxBVH34BuildStrategy,
    pub quantized: bool,
    pub structgen_pad0: [u8; 3],
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxMidphaseDesc {
    pub structgen_pad0: [u8; 16],
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxBVHDesc {
    pub bounds: PxBoundedData,
    pub enlargement: f32,
    pub numPrimsPerLeaf: u32,
    pub buildStrategy: PxBVHBuildStrategy,
    pub structgen_pad0: [u8; 4],
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxCookingParams {
    pub areaTestEpsilon: f32,
    pub planeTolerance: f32,
    pub convexMeshCookingType: PxConvexMeshCookingType,
    pub suppressTriangleMeshRemapTable: bool,
    pub buildTriangleAdjacencies: bool,
    pub buildGPUData: bool,
    pub structgen_pad0: [u8; 1],
    pub scale: PxTolerancesScale,
    pub meshPreprocessParams: PxMeshPreprocessingFlags,
    pub meshWeldTolerance: f32,
    pub meshAreaMinLimit: f32,
    pub meshEdgeLengthMaxLimit: f32,
    pub midphaseDesc: PxMidphaseDesc,
    pub gaussMapLimit: u32,
    pub maxWeightRatioInTet: f32,
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxSpring {
    pub stiffness: f32,
    pub damping: f32,
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxJointLimitParameters {
    pub restitution: f32,
    pub bounceThreshold: f32,
    pub stiffness: f32,
    pub damping: f32,
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxJointLinearLimit {
    pub structgen_pad0: [u8; 16],
    pub value: f32,
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxJointLinearLimitPair {
    pub structgen_pad0: [u8; 16],
    pub upper: f32,
    pub lower: f32,
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxJointAngularLimitPair {
    pub structgen_pad0: [u8; 16],
    pub upper: f32,
    pub lower: f32,
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxJointLimitCone {
    pub structgen_pad0: [u8; 16],
    pub yAngle: f32,
    pub zAngle: f32,
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxJointLimitPyramid {
    pub structgen_pad0: [u8; 16],
    pub yAngleMin: f32,
    pub yAngleMax: f32,
    pub zAngleMin: f32,
    pub zAngleMax: f32,
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxD6JointDrive {
    pub structgen_pad0: [u8; 8],
    pub forceLimit: f32,
    pub flags: PxD6JointDriveFlags,
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxGroupsMask {
    pub bits0: u16,
    pub bits1: u16,
    pub bits2: u16,
    pub bits3: u16,
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxRigidActorExt {
    pub structgen_pad0: [u8; 1],
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxConvexCoreExt {
    pub structgen_pad0: [u8; 1],
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxMassProperties {
    pub inertiaTensor: PxMat33,
    pub centerOfMass: PxVec3,
    pub mass: f32,
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxRigidBodyExt {
    pub structgen_pad0: [u8; 1],
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxShapeExt {
    pub structgen_pad0: [u8; 1],
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxMeshOverlapUtil {
    pub structgen_pad0: [u8; 1040],
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxSerialization {
    pub structgen_pad0: [u8; 1],
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxStringTableExt {
    pub structgen_pad0: [u8; 1],
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxBroadPhaseExt {
    pub structgen_pad0: [u8; 1],
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxSceneQueryExt {
    pub structgen_pad0: [u8; 1],
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxBatchQueryStatus {
    pub structgen_pad0: [u8; 1],
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxSamplingExt {
    pub structgen_pad0: [u8; 1],
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxTetrahedronMeshExt {
    pub structgen_pad0: [u8; 1],
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxGjkQuery {
    pub structgen_pad0: [u8; 1],
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxCustomGeometryExt {
    pub structgen_pad0: [u8; 1],
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxDeformableSurfaceExt {
    pub structgen_pad0: [u8; 1],
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxRepXObject {
    pub typeName: *const std::ffi::c_char,
    pub serializable: *const std::ffi::c_void,
    pub id: u64,
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxRepXInstantiationArgs {
    pub structgen_pad0: [u8; 8],
    pub cooker: *const PxCookingParams,
    pub stringTable: *mut PxStringTable,
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxVehicleComponentSequence {
    pub structgen_pad0: [u8; 808],
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxVehicleAxleDescription {
    pub nbAxles: u32,
    pub nbWheelsPerAxle: [u32; 20],
    pub axleToWheelIds: [u32; 20],
    pub wheelIdsInAxleOrder: [u32; 20],
    pub nbWheels: u32,
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxVehicleFrame {
    pub lngAxis: PxVehicleAxes,
    pub latAxis: PxVehicleAxes,
    pub vrtAxis: PxVehicleAxes,
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxVehicleScale {
    pub scale: f32,
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxVehicleTireSlipParams {
    pub minLatSlipDenominator: f32,
    pub minPassiveLongSlipDenominator: f32,
    pub minActiveLongSlipDenominator: f32,
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxVehicleTireAxisStickyParams {
    pub thresholdSpeed: f32,
    pub thresholdTime: f32,
    pub damping: f32,
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxVehicleTireStickyParams {
    pub stickyParams: [PxVehicleTireAxisStickyParams; 2],
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxVehiclePvdContext {
    pub attributeHandles: *const PxVehiclePvdAttributeHandles,
    pub structgen_pad0: [u8; 8],
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxVehicleSimulationContext {
    pub gravity: PxVec3,
    pub frame: PxVehicleFrame,
    pub scale: PxVehicleScale,
    pub tireSlipParams: PxVehicleTireSlipParams,
    pub tireStickyParams: PxVehicleTireStickyParams,
    pub thresholdForwardSpeedForWheelAngleIntegration: f32,
    pub structgen_pad0: [u8; 4],
    pub pvdContext: PxVehiclePvdContext,
    pub structgen_pad1: [u8; 8],
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxVehiclePhysXSimulationContext {
    pub structgen_pad0: [u8; 96],
    pub physxUnitCylinderSweepMesh: *const PxConvexMesh,
    pub physxScene: *const PxScene,
    pub physxActorUpdateMode: PxVehiclePhysXActorUpdateMode,
    pub physxActorWakeCounterResetValue: f32,
    pub physxActorWakeCounterThreshold: f32,
    pub structgen_pad1: [u8; 4],
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxVehicleRoadGeometryState {
    pub plane: PxPlane,
    pub friction: f32,
    pub velocity: PxVec3,
    pub hitState: bool,
    pub structgen_pad0: [u8; 3],
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxVehicleRigidBodyState {
    pub pose: PxTransform,
    pub linearVelocity: PxVec3,
    pub angularVelocity: PxVec3,
    pub previousLinearVelocity: PxVec3,
    pub previousAngularVelocity: PxVec3,
    pub externalForce: PxVec3,
    pub externalTorque: PxVec3,
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxVehiclePhysXRoadGeometryQueryState {
    pub actor: *mut PxRigidActor,
    pub shape: *mut PxShape,
    pub material: *mut PxMaterial,
    pub hitPosition: PxVec3,
    pub structgen_pad0: [u8; 4],
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxVehiclePhysXActor {
    pub rigidBody: *mut PxRigidBody,
    pub wheelShapes: [*mut PxShape; 20],
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxVehiclePhysXSteerState {
    pub previousSteerCommand: f32,
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxVehicleVectorN {
    pub structgen_pad0: [u8; 96],
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxVehicleMatrixNN {
    pub mValues: [[f32; 23]; 23],
    pub mSize: u32,
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxVehicleMatrixNNLUSolver {
    pub structgen_pad0: [u8; 2300],
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxVehicleMatrixNGaussSeidelSolver {
    pub structgen_pad0: [u8; 1],
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxVehicleMatrix33Solver {
    pub structgen_pad0: [u8; 1],
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxVehicleCommandValueResponseTable {
    pub commandValue: f32,
    pub structgen_pad0: [u8; 516],
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxVehicleCommandNonLinearResponseParams {
    pub speedResponses: [f32; 128],
    pub nbSpeedResponses: u16,
    pub speedResponsesPerCommandValue: [u16; 8],
    pub nbSpeedResponsesPerCommandValue: [u16; 8],
    pub structgen_pad0: [u8; 2],
    pub commandValues: [f32; 8],
    pub nbCommandValues: u16,
    pub structgen_pad1: [u8; 2],
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxVehicleCommandResponseParams {
    pub nonlinearResponse: PxVehicleCommandNonLinearResponseParams,
    pub wheelResponseMultipliers: [f32; 20],
    pub maxResponse: f32,
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxVehicleBrakeCommandResponseParams {
    pub structgen_pad0: [u8; 668],
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxVehicleCommandState {
    pub brakes: [f32; 2],
    pub nbBrakes: u32,
    pub throttle: f32,
    pub steer: f32,
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxVehicleDirectDriveTransmissionCommandState {
    pub structgen_pad0: [u8; 4],
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxVehicleEngineDriveTransmissionCommandState {
    pub clutch: f32,
    pub targetGear: u32,
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxVehicleTankDriveTransmissionCommandState {
    pub structgen_pad0: [u8; 8],
    pub thrusts: [f32; 2],
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxVehicleDirectDriveThrottleCommandResponseParams {
    pub structgen_pad0: [u8; 668],
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxVehicleClutchCommandResponseParams {
    pub maxResponse: f32,
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxVehicleClutchParams {
    pub accuracyMode: PxVehicleClutchAccuracyMode,
    pub estimateIterations: u32,
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxVehicleEngineParams {
    pub structgen_pad0: [u8; 68],
    pub moi: f32,
    pub peakTorque: f32,
    pub idleOmega: f32,
    pub maxOmega: f32,
    pub dampingRateFullThrottle: f32,
    pub dampingRateZeroThrottleClutchEngaged: f32,
    pub dampingRateZeroThrottleClutchDisengaged: f32,
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxVehicleGearboxParams {
    pub neutralGear: u32,
    pub ratios: [f32; 32],
    pub finalRatio: f32,
    pub nbRatios: u32,
    pub switchTime: f32,
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxVehicleAutoboxParams {
    pub upRatios: [f32; 32],
    pub downRatios: [f32; 32],
    pub latency: f32,
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxVehicleFourWheelDriveDifferentialLegacyParams {
    pub frontWheelIds: [u32; 2],
    pub rearWheelIds: [u32; 2],
    pub frontRearSplit: f32,
    pub frontNegPosSplit: f32,
    pub rearNegPosSplit: f32,
    pub centerBias: f32,
    pub frontBias: f32,
    pub rearBias: f32,
    pub structgen_pad0: [u8; 4],
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxVehicleMultiWheelDriveDifferentialParams {
    pub torqueRatios: [f32; 20],
    pub aveWheelSpeedRatios: [f32; 20],
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxVehicleFourWheelDriveDifferentialParams {
    pub structgen_pad0: [u8; 160],
    pub frontWheelIds: [u32; 2],
    pub rearWheelIds: [u32; 2],
    pub frontBias: f32,
    pub frontTarget: f32,
    pub rearBias: f32,
    pub rearTarget: f32,
    pub centerBias: f32,
    pub centerTarget: f32,
    pub rate: f32,
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxVehicleTankDriveDifferentialParams {
    pub structgen_pad0: [u8; 160],
    pub nbTracks: u32,
    pub thrustIdPerTrack: [u32; 20],
    pub nbWheelsPerTrack: [u32; 20],
    pub trackToWheelIds: [u32; 20],
    pub wheelIdsInTrackOrder: [u32; 20],
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxVehicleClutchCommandResponseState {
    pub normalisedCommandResponse: f32,
    pub commandResponse: f32,
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxVehicleEngineDriveThrottleCommandResponseState {
    pub commandResponse: f32,
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxVehicleEngineState {
    pub rotationSpeed: f32,
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxVehicleGearboxState {
    pub currentGear: u32,
    pub targetGear: u32,
    pub gearSwitchTime: f32,
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxVehicleAutoboxState {
    pub timeSinceLastShift: f32,
    pub activeAutoboxGearShift: bool,
    pub structgen_pad0: [u8; 3],
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxVehicleDifferentialState {
    pub connectedWheels: [u32; 20],
    pub nbConnectedWheels: u32,
    pub torqueRatiosAllWheels: [f32; 20],
    pub aveWheelSpeedContributionAllWheels: [f32; 20],
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxVehicleWheelConstraintGroupState {
    pub nbGroups: u32,
    pub nbWheelsPerGroup: [u32; 20],
    pub groupToWheelIds: [u32; 20],
    pub wheelIdsInGroupOrder: [u32; 20],
    pub wheelMultipliersInGroupOrder: [f32; 20],
    pub nbWheelsInGroups: u32,
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxVehicleClutchSlipState {
    pub clutchSlip: f32,
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxVehicleSteerCommandResponseParams {
    pub structgen_pad0: [u8; 668],
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxVehicleAckermannParams {
    pub wheelIds: [u32; 2],
    pub wheelBase: f32,
    pub trackWidth: f32,
    pub strength: f32,
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxVehicleWheelActuationState {
    pub isBrakeApplied: bool,
    pub isDriveApplied: bool,
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxVehicleWheelRigidBody1dState {
    pub rotationSpeed: f32,
    pub correctedRotationSpeed: f32,
    pub rotationAngle: f32,
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxVehicleWheelLocalPose {
    pub localPose: PxTransform,
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxVehicleWheelParams {
    pub radius: f32,
    pub halfWidth: f32,
    pub mass: f32,
    pub moi: f32,
    pub dampingRate: f32,
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxVehicleTireDirectionState {
    pub directions: [PxVec3; 2],
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxVehicleTireSpeedState {
    pub speedStates: [f32; 2],
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxVehicleTireSlipState {
    pub slips: [f32; 2],
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxVehicleTireGripState {
    pub load: f32,
    pub friction: f32,
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxVehicleTireCamberAngleState {
    pub camberAngle: f32,
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxVehicleTireStickyState {
    pub lowSpeedTime: [f32; 2],
    pub activeStatus: [bool; 2],
    pub structgen_pad0: [u8; 2],
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxVehicleTireForce {
    pub forces: [PxVec3; 2],
    pub torques: [PxVec3; 2],
    pub aligningMoment: f32,
    pub wheelTorque: f32,
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxVehiclePhysXRigidActorParams {
    pub structgen_pad0: [u8; 8],
    pub physxActorName: *const std::ffi::c_char,
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxVehiclePhysXRigidActorShapeParams {
    pub structgen_pad0: [u8; 24],
    pub flags: PxShapeFlags,
    pub structgen_pad1: [u8; 3],
    pub simulationFilterData: PxFilterData,
    pub queryFilterData: PxFilterData,
    pub structgen_pad2: [u8; 4],
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxVehiclePhysXWheelParams {
    pub structgen_pad0: [u8; 8],
    pub wheelParams: *const PxVehicleWheelParams,
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxVehiclePhysXWheelShapeParams {
    pub structgen_pad0: [u8; 8],
    pub flags: PxShapeFlags,
    pub structgen_pad1: [u8; 3],
    pub simulationFilterData: PxFilterData,
    pub queryFilterData: PxFilterData,
    pub structgen_pad2: [u8; 4],
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxVehiclePhysXSuspensionLimitConstraintParams {
    pub restitution: f32,
    pub directionForSuspensionLimitConstraint: DirectionSpecifier,
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxVehiclePhysXConstraintState {
    pub tireActiveStatus: [bool; 2],
    pub structgen_pad0: [u8; 2],
    pub tireLinears: [PxVec3; 2],
    pub tireAngulars: [PxVec3; 2],
    pub tireDamping: [f32; 2],
    pub suspActiveStatus: bool,
    pub structgen_pad1: [u8; 3],
    pub suspLinear: PxVec3,
    pub suspAngular: PxVec3,
    pub suspGeometricError: f32,
    pub restitution: f32,
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxVehiclePhysXConstraints {
    pub constraintStates: [PxVehiclePhysXConstraintState; 20],
    pub constraints: [*mut PxConstraint; 5],
    pub constraintConnectors: [*mut PxVehicleConstraintConnector; 5],
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxVehicleSuspensionParams {
    pub suspensionAttachment: PxTransform,
    pub suspensionTravelDir: PxVec3,
    pub suspensionTravelDist: f32,
    pub wheelAttachment: PxTransform,
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxVehicleSuspensionStateCalculationParams {
    pub suspensionJounceCalculationType: PxVehicleSuspensionJounceCalculationType,
    pub limitSuspensionExpansionVelocity: bool,
    pub structgen_pad0: [u8; 3],
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxVehicleSuspensionComplianceParams {
    pub structgen_pad0: [u8; 160],
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxVehicleSuspensionForceParams {
    pub stiffness: f32,
    pub damping: f32,
    pub sprungMass: f32,
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxVehicleSuspensionForceLegacyParams {
    pub stiffness: f32,
    pub damping: f32,
    pub restDistance: f32,
    pub sprungMass: f32,
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxVehicleAntiRollForceParams {
    pub wheel0: u32,
    pub wheel1: u32,
    pub stiffness: f32,
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxVehicleSuspensionState {
    pub jounce: f32,
    pub jounceSpeed: f32,
    pub separation: f32,
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxVehicleSuspensionComplianceState {
    pub toe: f32,
    pub camber: f32,
    pub tireForceAppPoint: PxVec3,
    pub suspForceAppPoint: PxVec3,
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxVehicleSuspensionForce {
    pub force: PxVec3,
    pub torque: PxVec3,
    pub normalForce: f32,
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxVehicleAntiRollTorque {
    pub antiRollTorque: PxVec3,
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxVehiclePhysXRoadGeometryQueryParams {
    pub defaultFilterData: PxQueryFilterData,
    pub structgen_pad0: [u8; 4],
    pub filterDataEntries: *mut PxQueryFilterData,
    pub filterCallback: *mut PxQueryFilterCallback,
    pub roadGeometryQueryType: PxVehiclePhysXRoadGeometryQueryType,
    pub structgen_pad1: [u8; 4],
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxVehiclePhysXMaterialFriction {
    pub material: *const PxMaterial,
    pub friction: f32,
    pub structgen_pad0: [u8; 4],
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxVehiclePhysXMaterialFrictionParams {
    pub materialFrictions: *mut PxVehiclePhysXMaterialFriction,
    pub nbMaterialFrictions: u32,
    pub defaultFriction: f32,
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxVehicleTireForceParams {
    pub latStiffX: f32,
    pub latStiffY: f32,
    pub longStiff: f32,
    pub camberStiff: f32,
    pub frictionVsSlip: [[f32; 3]; 2],
    pub restLoad: f32,
    pub loadFilter: [[f32; 2]; 2],
}
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxVehicleRigidBodyParams {
    pub mass: f32,
    pub moi: PxVec3,
}
#[cfg(test)]
mod sizes {
    use super::*;
    use std::mem::size_of;
    #[test]
    fn check_sizes() {
        assert_eq!(size_of::<PxVec2>(), 8);
        assert_eq!(size_of::<PxVec3>(), 12);
        assert_eq!(size_of::<PxExtendedVec3>(), 24);
        assert_eq!(size_of::<PxVec4>(), 16);
        assert_eq!(size_of::<PxQuat>(), 16);
        assert_eq!(size_of::<PxMat33>(), 36);
        assert_eq!(size_of::<PxMat44>(), 64);
        assert_eq!(size_of::<PxTransform>(), 28);
        assert_eq!(size_of::<PxAllocator>(), 1);
        assert_eq!(size_of::<PxRawAllocator>(), 1);
        assert_eq!(size_of::<PxUserAllocated>(), 1);
        assert_eq!(size_of::<PxTempAllocator>(), 1);
        assert_eq!(size_of::<PxBitAndByte>(), 1);
        assert_eq!(size_of::<PxBitMap>(), 16);
        assert_eq!(size_of::<PxVec3Padded>(), 16);
        assert_eq!(size_of::<PxTransformPadded>(), 32);
        assert_eq!(size_of::<PxBounds3>(), 24);
        assert_eq!(size_of::<PxBroadcastingErrorCallback>(), 160);
        assert_eq!(size_of::<PxFPUGuard>(), 32);
        assert_eq!(size_of::<PxSIMDGuard>(), 8);
        assert_eq!(size_of::<PxPlane>(), 16);
        assert_eq!(size_of::<Interpolation>(), 1);
        assert_eq!(size_of::<PxMutexImpl>(), 1);
        assert_eq!(size_of::<PxReadWriteLock>(), 8);
        assert_eq!(size_of::<PxProfileScoped>(), 40);
        assert_eq!(size_of::<PxSListEntry>(), 16);
        assert_eq!(size_of::<PxSListImpl>(), 1);
        assert_eq!(size_of::<PxSyncImpl>(), 1);
        assert_eq!(size_of::<PxThreadImpl>(), 1);
        assert_eq!(size_of::<PxCounterFrequencyToTensOfNanos>(), 16);
        assert_eq!(size_of::<PxTime>(), 8);
        assert_eq!(size_of::<PxBoundedData>(), 16);
        assert_eq!(size_of::<PxDebugPoint>(), 16);
        assert_eq!(size_of::<PxDebugLine>(), 32);
        assert_eq!(size_of::<PxDebugTriangle>(), 48);
        assert_eq!(size_of::<PxDebugText>(), 32);
        assert_eq!(size_of::<PxTolerancesScale>(), 8);
        assert_eq!(size_of::<PxGeometry>(), 8);
        assert_eq!(size_of::<PxBoxGeometry>(), 20);
        assert_eq!(size_of::<PxCapsuleGeometry>(), 16);
        assert_eq!(size_of::<PxHullPolygon>(), 20);
        assert_eq!(size_of::<PxMeshScale>(), 28);
        assert_eq!(size_of::<PxConvexMeshGeometry>(), 56);
        assert_eq!(size_of::<PxSphereGeometry>(), 12);
        assert_eq!(size_of::<PxPlaneGeometry>(), 8);
        assert_eq!(size_of::<PxTriangleMeshGeometry>(), 48);
        assert_eq!(size_of::<PxHeightFieldGeometry>(), 32);
        assert_eq!(size_of::<PxFilterData>(), 16);
        assert_eq!(size_of::<PxParticleSystemGeometry>(), 8);
        assert_eq!(size_of::<PxTetrahedronMeshGeometry>(), 16);
        assert_eq!(size_of::<PxQueryHit>(), 4);
        assert_eq!(size_of::<PxLocationHit>(), 36);
        assert_eq!(size_of::<PxGeomRaycastHit>(), 44);
        assert_eq!(size_of::<PxGeomOverlapHit>(), 4);
        assert_eq!(size_of::<PxGeomSweepHit>(), 36);
        assert_eq!(size_of::<PxGeomIndexPair>(), 8);
        assert_eq!(size_of::<PxGeomIndexClosePair>(), 12);
        assert_eq!(size_of::<PxQueryThreadContext>(), 1);
        assert_eq!(size_of::<PxCustomGeometry>(), 16);
        assert_eq!(size_of::<PxConvexCoreGeometry>(), 40);
        assert_eq!(size_of::<PxGeometryHolder>(), 56);
        assert_eq!(size_of::<PxGeometryQuery>(), 1);
        assert_eq!(size_of::<PxHeightFieldSample>(), 4);
        assert_eq!(size_of::<PxHeightFieldDesc>(), 40);
        assert_eq!(size_of::<PxMeshQuery>(), 1);
        assert_eq!(size_of::<PxSimpleTriangleMesh>(), 40);
        assert_eq!(size_of::<PxTriangle>(), 36);
        assert_eq!(size_of::<PxTrianglePadded>(), 40);
        assert_eq!(size_of::<PxTetrahedron>(), 48);
        assert_eq!(size_of::<PxDeformableVolumeCollisionData>(), 1);
        assert_eq!(size_of::<PxTetrahedronMeshData>(), 1);
        assert_eq!(size_of::<PxDeformableVolumeSimulationData>(), 1);
        assert_eq!(size_of::<Px1DConstraintFlag>(), 1);
        assert_eq!(size_of::<Px1DConstraint>(), 80);
        assert_eq!(size_of::<PxConstraintInvMassScale>(), 16);
        assert_eq!(size_of::<PxContactPoint>(), 80);
        assert_eq!(size_of::<PxSolverBody>(), 32);
        assert_eq!(size_of::<PxSolverBodyData>(), 112);
        assert_eq!(size_of::<PxConstraintBatchHeader>(), 8);
        assert_eq!(size_of::<PxSolverConstraintDesc>(), 64);
        assert_eq!(size_of::<PxSolverConstraintPrepDescBase>(), 128);
        assert_eq!(size_of::<PxSolverConstraintPrepDesc>(), 176);
        assert_eq!(size_of::<PxSolverContactDesc>(), 192);
        assert_eq!(size_of::<PxArticulationLimit>(), 8);
        assert_eq!(size_of::<PxPerformanceEnvelope>(), 16);
        assert_eq!(size_of::<PxJointFrictionParams>(), 12);
        assert_eq!(size_of::<PxArticulationDrive>(), 32);
        assert_eq!(size_of::<PxTGSSolverBodyVel>(), 64);
        assert_eq!(size_of::<PxTGSSolverBodyTxInertia>(), 64);
        assert_eq!(size_of::<PxTGSSolverBodyData>(), 48);
        assert_eq!(size_of::<PxTGSSolverConstraintPrepDescBase>(), 144);
        assert_eq!(size_of::<PxTGSSolverConstraintPrepDesc>(), 224);
        assert_eq!(size_of::<PxTGSSolverContactDesc>(), 224);
        assert_eq!(size_of::<PxArticulationTendonLimit>(), 8);
        assert_eq!(size_of::<PxSpatialForce>(), 32);
        assert_eq!(size_of::<PxSpatialVelocity>(), 32);
        assert_eq!(size_of::<PxArticulationRootLinkData>(), 76);
        assert_eq!(size_of::<PxArticulationCache>(), 192);
        assert_eq!(size_of::<PxNodeIndex>(), 8);
        assert_eq!(size_of::<PxConstraintShaderTable>(), 24);
        assert_eq!(size_of::<PxContactPatch>(), 64);
        assert_eq!(size_of::<PxContact>(), 16);
        assert_eq!(size_of::<PxExtendedContact>(), 32);
        assert_eq!(size_of::<PxModifiableContact>(), 64);
        assert_eq!(size_of::<PxContactStreamIterator>(), 80);
        assert_eq!(size_of::<PxFrictionPatch>(), 52);
        assert_eq!(size_of::<PxFrictionAnchorStreamIterator>(), 32);
        assert_eq!(size_of::<PxGpuContactPair>(), 80);
        assert_eq!(size_of::<PxContactSet>(), 16);
        assert_eq!(size_of::<PxContactModifyPair>(), 104);
        assert_eq!(size_of::<PxDiffuseParticleParams>(), 40);
        assert_eq!(size_of::<PxDeformableAttachmentData>(), 144);
        assert_eq!(size_of::<PxDeformableElementFilterData>(), 80);
        assert_eq!(size_of::<PxActorShape>(), 16);
        assert_eq!(size_of::<PxRaycastHit>(), 64);
        assert_eq!(size_of::<PxOverlapHit>(), 24);
        assert_eq!(size_of::<PxSweepHit>(), 56);
        assert_eq!(size_of::<PxRaycastCallback>(), 96);
        assert_eq!(size_of::<PxOverlapCallback>(), 56);
        assert_eq!(size_of::<PxSweepCallback>(), 88);
        assert_eq!(size_of::<PxRaycastBuffer>(), 96);
        assert_eq!(size_of::<PxOverlapBuffer>(), 56);
        assert_eq!(size_of::<PxSweepBuffer>(), 88);
        assert_eq!(size_of::<PxQueryCache>(), 24);
        assert_eq!(size_of::<PxQueryFilterData>(), 20);
        assert_eq!(size_of::<PxArticulationGPUAPIMaxCounts>(), 24);
        assert_eq!(size_of::<PxSceneQueryDesc>(), 36);
        assert_eq!(size_of::<PxBroadPhaseRegion>(), 32);
        assert_eq!(size_of::<PxBroadPhaseRegionInfo>(), 48);
        assert_eq!(size_of::<PxBroadPhaseCaps>(), 4);
        assert_eq!(size_of::<PxGpuBroadPhaseDesc>(), 6);
        assert_eq!(size_of::<PxBroadPhaseDesc>(), 32);
        assert_eq!(size_of::<PxBroadPhaseUpdateData>(), 80);
        assert_eq!(size_of::<PxBroadPhasePair>(), 8);
        assert_eq!(size_of::<PxBroadPhaseResults>(), 32);
        assert_eq!(size_of::<PxSceneLimits>(), 32);
        assert_eq!(size_of::<PxGpuDynamicsMemoryConfig>(), 48);
        assert_eq!(size_of::<PxSceneDesc>(), 376);
        assert_eq!(size_of::<PxGpuDynamicsMemoryConfigStatistics>(), 48);
        assert_eq!(size_of::<PxSimulationStatistics>(), 2256);
        assert_eq!(size_of::<PxDominanceGroupPair>(), 2);
        assert_eq!(size_of::<PxSceneReadLock>(), 8);
        assert_eq!(size_of::<PxSceneWriteLock>(), 8);
        assert_eq!(size_of::<PxContactPairExtraDataItem>(), 1);
        assert_eq!(size_of::<PxContactPairVelocity>(), 52);
        assert_eq!(size_of::<PxContactPairPose>(), 60);
        assert_eq!(size_of::<PxContactPairIndex>(), 4);
        assert_eq!(size_of::<PxContactPairExtraDataIterator>(), 48);
        assert_eq!(size_of::<PxContactPairHeader>(), 48);
        assert_eq!(size_of::<PxContactPairPoint>(), 48);
        assert_eq!(size_of::<PxContactPairFrictionAnchor>(), 24);
        assert_eq!(size_of::<PxContactPair>(), 72);
        assert_eq!(size_of::<PxTriggerPair>(), 40);
        assert_eq!(size_of::<PxConstraintInfo>(), 24);
        assert_eq!(size_of::<PxObstacle>(), 56);
        assert_eq!(size_of::<PxBoxObstacle>(), 72);
        assert_eq!(size_of::<PxCapsuleObstacle>(), 64);
        assert_eq!(size_of::<PxControllerState>(), 48);
        assert_eq!(size_of::<PxControllerStats>(), 8);
        assert_eq!(size_of::<PxControllerHit>(), 64);
        assert_eq!(size_of::<PxControllerShapeHit>(), 88);
        assert_eq!(size_of::<PxControllersHit>(), 72);
        assert_eq!(size_of::<PxControllerObstacleHit>(), 72);
        assert_eq!(size_of::<PxControllerFilters>(), 32);
        assert_eq!(size_of::<PxDim3>(), 12);
        assert_eq!(size_of::<PxSDFDesc>(), 184);
        assert_eq!(size_of::<PxConvexMeshDesc>(), 64);
        assert_eq!(size_of::<PxTriangleMeshDesc>(), 72);
        assert_eq!(size_of::<PxTetrahedronMeshDesc>(), 56);
        assert_eq!(size_of::<PxDeformableVolumeSimulationDataDesc>(), 16);
        assert_eq!(size_of::<PxBVH33MidphaseDesc>(), 8);
        assert_eq!(size_of::<PxBVH34MidphaseDesc>(), 12);
        assert_eq!(size_of::<PxMidphaseDesc>(), 16);
        assert_eq!(size_of::<PxBVHDesc>(), 32);
        assert_eq!(size_of::<PxCookingParams>(), 64);
        assert_eq!(size_of::<PxSpring>(), 8);
        assert_eq!(size_of::<PxJointLimitParameters>(), 16);
        assert_eq!(size_of::<PxJointLinearLimit>(), 20);
        assert_eq!(size_of::<PxJointLinearLimitPair>(), 24);
        assert_eq!(size_of::<PxJointAngularLimitPair>(), 24);
        assert_eq!(size_of::<PxJointLimitCone>(), 24);
        assert_eq!(size_of::<PxJointLimitPyramid>(), 32);
        assert_eq!(size_of::<PxD6JointDrive>(), 16);
        assert_eq!(size_of::<PxGroupsMask>(), 8);
        assert_eq!(size_of::<PxRigidActorExt>(), 1);
        assert_eq!(size_of::<PxConvexCoreExt>(), 1);
        assert_eq!(size_of::<PxMassProperties>(), 52);
        assert_eq!(size_of::<PxRigidBodyExt>(), 1);
        assert_eq!(size_of::<PxShapeExt>(), 1);
        assert_eq!(size_of::<PxMeshOverlapUtil>(), 1040);
        assert_eq!(size_of::<PxSerialization>(), 1);
        assert_eq!(size_of::<PxStringTableExt>(), 1);
        assert_eq!(size_of::<PxBroadPhaseExt>(), 1);
        assert_eq!(size_of::<PxSceneQueryExt>(), 1);
        assert_eq!(size_of::<PxBatchQueryStatus>(), 1);
        assert_eq!(size_of::<PxSamplingExt>(), 1);
        assert_eq!(size_of::<PxTetrahedronMeshExt>(), 1);
        assert_eq!(size_of::<PxGjkQuery>(), 1);
        assert_eq!(size_of::<PxCustomGeometryExt>(), 1);
        assert_eq!(size_of::<PxDeformableSurfaceExt>(), 1);
        assert_eq!(size_of::<PxRepXObject>(), 24);
        assert_eq!(size_of::<PxRepXInstantiationArgs>(), 24);
        assert_eq!(size_of::<PxVehicleComponentSequence>(), 808);
        assert_eq!(size_of::<PxVehicleAxleDescription>(), 248);
        assert_eq!(size_of::<PxVehicleFrame>(), 12);
        assert_eq!(size_of::<PxVehicleScale>(), 4);
        assert_eq!(size_of::<PxVehicleTireSlipParams>(), 12);
        assert_eq!(size_of::<PxVehicleTireAxisStickyParams>(), 12);
        assert_eq!(size_of::<PxVehicleTireStickyParams>(), 24);
        assert_eq!(size_of::<PxVehiclePvdContext>(), 16);
        assert_eq!(size_of::<PxVehicleSimulationContext>(), 96);
        assert_eq!(size_of::<PxVehiclePhysXSimulationContext>(), 128);
        assert_eq!(size_of::<PxVehicleRoadGeometryState>(), 36);
        assert_eq!(size_of::<PxVehicleRigidBodyState>(), 100);
        assert_eq!(size_of::<PxVehiclePhysXRoadGeometryQueryState>(), 40);
        assert_eq!(size_of::<PxVehiclePhysXActor>(), 168);
        assert_eq!(size_of::<PxVehiclePhysXSteerState>(), 4);
        assert_eq!(size_of::<PxVehicleVectorN>(), 96);
        assert_eq!(size_of::<PxVehicleMatrixNN>(), 2120);
        assert_eq!(size_of::<PxVehicleMatrixNNLUSolver>(), 2300);
        assert_eq!(size_of::<PxVehicleMatrixNGaussSeidelSolver>(), 1);
        assert_eq!(size_of::<PxVehicleMatrix33Solver>(), 1);
        assert_eq!(size_of::<PxVehicleCommandValueResponseTable>(), 520);
        assert_eq!(size_of::<PxVehicleCommandNonLinearResponseParams>(), 584);
        assert_eq!(size_of::<PxVehicleCommandResponseParams>(), 668);
        assert_eq!(size_of::<PxVehicleBrakeCommandResponseParams>(), 668);
        assert_eq!(size_of::<PxVehicleCommandState>(), 20);
        assert_eq!(size_of::<PxVehicleDirectDriveTransmissionCommandState>(), 4);
        assert_eq!(size_of::<PxVehicleEngineDriveTransmissionCommandState>(), 8);
        assert_eq!(size_of::<PxVehicleTankDriveTransmissionCommandState>(), 16);
        assert_eq!(size_of::<PxVehicleDirectDriveThrottleCommandResponseParams>(), 668);
        assert_eq!(size_of::<PxVehicleClutchCommandResponseParams>(), 4);
        assert_eq!(size_of::<PxVehicleClutchParams>(), 8);
        assert_eq!(size_of::<PxVehicleEngineParams>(), 96);
        assert_eq!(size_of::<PxVehicleGearboxParams>(), 144);
        assert_eq!(size_of::<PxVehicleAutoboxParams>(), 260);
        assert_eq!(size_of::<PxVehicleFourWheelDriveDifferentialLegacyParams>(), 44);
        assert_eq!(size_of::<PxVehicleMultiWheelDriveDifferentialParams>(), 160);
        assert_eq!(size_of::<PxVehicleFourWheelDriveDifferentialParams>(), 204);
        assert_eq!(size_of::<PxVehicleTankDriveDifferentialParams>(), 484);
        assert_eq!(size_of::<PxVehicleClutchCommandResponseState>(), 8);
        assert_eq!(size_of::<PxVehicleEngineDriveThrottleCommandResponseState>(), 4);
        assert_eq!(size_of::<PxVehicleEngineState>(), 4);
        assert_eq!(size_of::<PxVehicleGearboxState>(), 12);
        assert_eq!(size_of::<PxVehicleAutoboxState>(), 8);
        assert_eq!(size_of::<PxVehicleDifferentialState>(), 244);
        assert_eq!(size_of::<PxVehicleWheelConstraintGroupState>(), 328);
        assert_eq!(size_of::<PxVehicleClutchSlipState>(), 4);
        assert_eq!(size_of::<PxVehicleSteerCommandResponseParams>(), 668);
        assert_eq!(size_of::<PxVehicleAckermannParams>(), 20);
        assert_eq!(size_of::<PxVehicleWheelActuationState>(), 2);
        assert_eq!(size_of::<PxVehicleWheelRigidBody1dState>(), 12);
        assert_eq!(size_of::<PxVehicleWheelLocalPose>(), 28);
        assert_eq!(size_of::<PxVehicleWheelParams>(), 20);
        assert_eq!(size_of::<PxVehicleTireDirectionState>(), 24);
        assert_eq!(size_of::<PxVehicleTireSpeedState>(), 8);
        assert_eq!(size_of::<PxVehicleTireSlipState>(), 8);
        assert_eq!(size_of::<PxVehicleTireGripState>(), 8);
        assert_eq!(size_of::<PxVehicleTireCamberAngleState>(), 4);
        assert_eq!(size_of::<PxVehicleTireStickyState>(), 12);
        assert_eq!(size_of::<PxVehicleTireForce>(), 56);
        assert_eq!(size_of::<PxVehiclePhysXRigidActorParams>(), 16);
        assert_eq!(size_of::<PxVehiclePhysXRigidActorShapeParams>(), 64);
        assert_eq!(size_of::<PxVehiclePhysXWheelParams>(), 16);
        assert_eq!(size_of::<PxVehiclePhysXWheelShapeParams>(), 48);
        assert_eq!(size_of::<PxVehiclePhysXSuspensionLimitConstraintParams>(), 8);
        assert_eq!(size_of::<PxVehiclePhysXConstraintState>(), 96);
        assert_eq!(size_of::<PxVehiclePhysXConstraints>(), 2000);
        assert_eq!(size_of::<PxVehicleSuspensionParams>(), 72);
        assert_eq!(size_of::<PxVehicleSuspensionStateCalculationParams>(), 8);
        assert_eq!(size_of::<PxVehicleSuspensionComplianceParams>(), 160);
        assert_eq!(size_of::<PxVehicleSuspensionForceParams>(), 12);
        assert_eq!(size_of::<PxVehicleSuspensionForceLegacyParams>(), 16);
        assert_eq!(size_of::<PxVehicleAntiRollForceParams>(), 12);
        assert_eq!(size_of::<PxVehicleSuspensionState>(), 12);
        assert_eq!(size_of::<PxVehicleSuspensionComplianceState>(), 32);
        assert_eq!(size_of::<PxVehicleSuspensionForce>(), 28);
        assert_eq!(size_of::<PxVehicleAntiRollTorque>(), 12);
        assert_eq!(size_of::<PxVehiclePhysXRoadGeometryQueryParams>(), 48);
        assert_eq!(size_of::<PxVehiclePhysXMaterialFriction>(), 16);
        assert_eq!(size_of::<PxVehiclePhysXMaterialFrictionParams>(), 16);
        assert_eq!(size_of::<PxVehicleTireForceParams>(), 60);
        assert_eq!(size_of::<PxVehicleRigidBodyParams>(), 16);
    }
}
