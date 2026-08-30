/// Error codes
///
/// These error codes are passed to [`PxErrorCallback`]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PxErrorCode {
    NoError = 0,
    /// An informational message.
    DebugInfo = 1,
    /// a warning message for the user to help with debugging
    DebugWarning = 2,
    /// method called with invalid parameter(s)
    InvalidParameter = 4,
    /// method was called at a time when an operation is not possible
    InvalidOperation = 8,
    /// method failed to allocate some memory
    OutOfMemory = 16,
    /// The library failed for some reason.
    /// Possibly you have passed invalid values like NaNs, which are not checked for.
    InternalError = 32,
    /// An unrecoverable error, execution should be halted and log output flushed
    Abort = 64,
    /// The SDK has determined that an operation may result in poor performance.
    PerfWarning = 128,
    /// A bit mask for including all errors
    MaskAll = -1,
}

/// enum for empty constructor tag
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PxEMPTY {
    PxEmpty = 0,
}

/// enum for zero constructor tag for vectors and matrices
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PxZERO {
    PxZero = 0,
}

/// enum for identity constructor flag for quaternions, transforms, and matrices
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PxIDENTITY {
    PxIdentity = 0,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum PxThreadPriority {
    /// High priority
    High = 0,
    /// Above Normal priority
    AboveNormal = 1,
    /// Normal/default priority
    Normal = 2,
    /// Below Normal priority
    BelowNormal = 3,
    /// Low priority.
    Low = 4,
    ForceDword = 4294967295,
}

/// Default color values used for debug rendering.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum PxDebugColor {
    ArgbBlack = 4278190080,
    ArgbRed = 4294901760,
    ArgbGreen = 4278255360,
    ArgbBlue = 4278190335,
    ArgbYellow = 4294967040,
    ArgbMagenta = 4294902015,
    ArgbCyan = 4278255615,
    ArgbWhite = 4294967295,
    ArgbGrey = 4286611584,
    ArgbDarkred = 4287102976,
    ArgbDarkgreen = 4278224896,
    ArgbDarkblue = 4278190216,
}

/// an enumeration of concrete classes inheriting from PxBase
///
/// Enumeration space is reserved for future PhysX core types, PhysXExtensions,
/// PhysXVehicle and Custom application types.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PxConcreteType {
    Undefined = 0,
    Heightfield = 1,
    ConvexMesh = 2,
    /// Will be removed together with deprecated BVH33.
    TriangleMeshBvh33 = 3,
    TriangleMeshBvh34 = 4,
    TetrahedronMesh = 5,
    DeformableVolumeMesh = 6,
    RigidDynamic = 7,
    RigidStatic = 8,
    Shape = 9,
    Material = 10,
    DeformableSurfaceMaterial = 11,
    DeformableVolumeMaterial = 12,
    PbdMaterial = 13,
    Constraint = 14,
    Aggregate = 15,
    ArticulationReducedCoordinate = 16,
    ArticulationLink = 17,
    ArticulationJointReducedCoordinate = 18,
    ArticulationSpatialTendon = 19,
    ArticulationFixedTendon = 20,
    ArticulationAttachment = 21,
    ArticulationTendonJoint = 22,
    ArticulationMimicJoint = 23,
    PruningStructure = 24,
    Bvh = 25,
    DeformableVolume = 26,
    DeformableVolumeState = 27,
    PbdParticlesystem = 28,
    DeformableSurface = 29,
    DeformableAttachment = 30,
    DeformableElementFilter = 31,
    ParticleBuffer = 32,
    ParticleDiffuseBuffer = 33,
    PhysxCoreCount = 34,
    FirstPhysxExtension = 256,
    FirstVehicleExtension = 512,
    FirstUserExtension = 1024,
}

impl From<u16> for PxConcreteType {
    fn from(val: u16) -> Self {
        #[allow(clippy::match_same_arms)]
        match val {
            0 => Self::Undefined,
            1 => Self::Heightfield,
            2 => Self::ConvexMesh,
            3 => Self::TriangleMeshBvh33,
            4 => Self::TriangleMeshBvh34,
            5 => Self::TetrahedronMesh,
            6 => Self::DeformableVolumeMesh,
            7 => Self::RigidDynamic,
            8 => Self::RigidStatic,
            9 => Self::Shape,
            10 => Self::Material,
            11 => Self::DeformableSurfaceMaterial,
            12 => Self::DeformableVolumeMaterial,
            13 => Self::PbdMaterial,
            14 => Self::Constraint,
            15 => Self::Aggregate,
            16 => Self::ArticulationReducedCoordinate,
            17 => Self::ArticulationLink,
            18 => Self::ArticulationJointReducedCoordinate,
            19 => Self::ArticulationSpatialTendon,
            20 => Self::ArticulationFixedTendon,
            21 => Self::ArticulationAttachment,
            22 => Self::ArticulationTendonJoint,
            23 => Self::ArticulationMimicJoint,
            24 => Self::PruningStructure,
            25 => Self::Bvh,
            26 => Self::DeformableVolume,
            27 => Self::DeformableVolumeState,
            28 => Self::PbdParticlesystem,
            29 => Self::DeformableSurface,
            30 => Self::DeformableAttachment,
            31 => Self::DeformableElementFilter,
            32 => Self::ParticleBuffer,
            33 => Self::ParticleDiffuseBuffer,
            34 => Self::PhysxCoreCount,
            256 => Self::FirstPhysxExtension,
            512 => Self::FirstVehicleExtension,
            1024 => Self::FirstUserExtension,
            _ => Self::Undefined,
        }
    }
}

/// Flags for PxBase.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PxBaseFlag {
    OwnsMemory = 1,
    IsReleasable = 2,
}

bitflags::bitflags! {
    /// Flags for [`PxBaseFlag`]
    #[derive(Default)]
    #[repr(transparent)]
    pub struct PxBaseFlags: u16 {
        const OwnsMemory = 1 << 0;
        const IsReleasable = 1 << 1;
    }
}

/// Identifies the type of each heavyweight PxTask object
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PxTaskType {
    /// PxTask will be run on the CPU
    Cpu = 0,
    /// Return code when attempting to find a task that does not exist
    NotPresent = 1,
    /// PxTask execution has been completed
    Completed = 2,
}

/// A geometry type.
///
/// Used to distinguish the type of a ::PxGeometry object.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PxGeometryType {
    Sphere = 0,
    Plane = 1,
    Capsule = 2,
    Box = 3,
    Convexcore = 4,
    Convexmesh = 5,
    Particlesystem = 6,
    Tetrahedronmesh = 7,
    Trianglemesh = 8,
    Heightfield = 9,
    Custom = 10,
    /// internal use only!
    GeometryCount = 11,
    /// internal use only!
    Invalid = -1,
}

/// Geometry-level query flags.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PxGeometryQueryFlag {
    /// Saves/restores SIMD control word for each query (safer but slower). Omit this if you took care of it yourself in your app.
    SimdGuard = 1,
    Default = 1,
}

bitflags::bitflags! {
    /// Flags for [`PxGeometryQueryFlag`]
    #[derive(Default)]
    #[repr(transparent)]
    pub struct PxGeometryQueryFlags: u32 {
        const SimdGuard = 1 << 0;
        const Default = 1 << 0;
    }
}

/// Desired build strategy for bounding-volume hierarchies
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PxBVHBuildStrategy {
    /// Fast build strategy. Fast build speed, good runtime performance in most cases. Recommended for runtime cooking.
    Fast = 0,
    /// Default build strategy. Medium build speed, good runtime performance in all cases.
    Default = 1,
    /// SAH build strategy. Slower builds, slightly improved runtime performance in some cases.
    Sah = 2,
    Last = 3,
}

/// Flags controlling the simulated behavior of the convex mesh geometry.
///
/// Used in ::PxConvexMeshGeometryFlags.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PxConvexMeshGeometryFlag {
    /// Use tighter (but more expensive to compute) bounds around the convex geometry.
    TightBounds = 1,
}

bitflags::bitflags! {
    /// Flags for [`PxConvexMeshGeometryFlag`]
    #[derive(Default)]
    #[repr(transparent)]
    pub struct PxConvexMeshGeometryFlags: u8 {
        const TightBounds = 1 << 0;
    }
}

/// Flags controlling the simulated behavior of the triangle mesh geometry.
///
/// Used in ::PxMeshGeometryFlags.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PxMeshGeometryFlag {
    /// Use tighter (but more expensive to compute) bounds around the triangle mesh geometry.
    TightBounds = 1,
    /// Meshes with this flag set are treated as double-sided.
    /// This flag is currently only used for raycasts and sweeps. It is ignored for overlap queries and has no effect on contact generation, i.e. simulation.
    /// For detailed specifications of this flag for meshes and heightfields please refer to the Geometry Query section of the user guide.
    /// For double-sided collision meshes, consider duplicating their faces with flipped normals.
    DoubleSided = 2,
}

bitflags::bitflags! {
    /// Flags for [`PxMeshGeometryFlag`]
    #[derive(Default)]
    #[repr(transparent)]
    pub struct PxMeshGeometryFlags: u8 {
        const TightBounds = 1 << 0;
        const DoubleSided = 1 << 1;
    }
}

/// Identifies dirty particle buffers that need to be updated in the particle system.
///
/// This flag can be used mark the device user buffers that are dirty and need to be written to the particle system.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PxParticleBufferFlag {
    /// No data specified
    None = 0,
    /// Specifies the position (first 3 floats) and inverse mass (last float) data (array of PxVec4 * number of particles)
    UpdatePosition = 1,
    /// Specifies the velocity (first 3 floats) data (array of PxVec4 * number of particles)
    UpdateVelocity = 2,
    /// Specifies the per-particle phase flag data (array of PxU32 * number of particles)
    UpdatePhase = 4,
    /// Specifies the rest position (first 3 floats) data
    UpdateRestposition = 8,
    /// Specifies the diffuse particle parameter buffer (see PxDiffuseParticleParams)
    UpdateDiffuseParam = 16,
    All = 31,
}

bitflags::bitflags! {
    /// Flags for [`PxParticleBufferFlag`]
    #[derive(Default)]
    #[repr(transparent)]
    pub struct PxParticleBufferFlags: u32 {
        const UpdatePosition = 1 << 0;
        const UpdateVelocity = 1 << 1;
        const UpdatePhase = 1 << 2;
        const UpdateRestposition = 1 << 3;
        const UpdateDiffuseParam = 1 << 4;
        const All = Self::UpdatePosition.bits | Self::UpdateVelocity.bits | Self::UpdatePhase.bits | Self::UpdateRestposition.bits | Self::UpdateDiffuseParam.bits;
    }
}

/// Identifies per-particle behavior for a PxParticleSystem.
///
/// See [`PxPBDParticleSystem::createPhase`]().
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum PxParticlePhaseFlag {
    /// Bits [ 0, 19] represent the particle group for controlling collisions
    ParticlePhaseGroupMask = 1048575,
    /// Bits [20, 23] hold flags about how the particle behave
    ParticlePhaseFlagsMask = 4293918720,
    /// If set this particle will interact with particles of the same group
    ParticlePhaseSelfCollide = 1048576,
    /// If set this particle will ignore collisions with particles closer than the radius in the rest pose, this flag should not be specified unless valid rest positions have been specified using setRestParticles()
    ParticlePhaseSelfCollideFilter = 2097152,
    /// If set this particle will generate fluid density constraints for its overlapping neighbors
    ParticlePhaseFluid = 4194304,
}

bitflags::bitflags! {
    /// Flags for [`PxParticlePhaseFlag`]
    #[derive(Default)]
    #[repr(transparent)]
    pub struct PxParticlePhaseFlags: u32 {
        const ParticlePhaseGroupMask = 0x000fffff;
        const ParticlePhaseFlagsMask = Self::ParticlePhaseSelfCollide.bits | Self::ParticlePhaseSelfCollideFilter.bits | Self::ParticlePhaseFluid.bits;
        const ParticlePhaseSelfCollide = 1 << 20;
        const ParticlePhaseSelfCollideFilter = 1 << 21;
        const ParticlePhaseFluid = 1 << 22;
    }
}

/// Collection of flags describing the actions to take for a collision pair.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PxPairFlag {
    /// Process the contacts of this collision pair in the dynamics solver.
    ///
    /// Only takes effect if the colliding actors are rigid bodies.
    SolveContact = 1,
    /// Call contact modification callback for this collision pair
    ///
    /// Only takes effect if the colliding actors are rigid bodies.
    ModifyContacts = 2,
    /// Call contact report callback or trigger callback when this collision pair starts to be in contact.
    ///
    /// If one of the two collision objects is a trigger shape (see [`PxShapeFlag::eTRIGGER_SHAPE`])
    /// then the trigger callback will get called as soon as the other object enters the trigger volume.
    /// If none of the two collision objects is a trigger shape then the contact report callback will get
    /// called when the actors of this collision pair start to be in contact.
    ///
    /// Only takes effect if the colliding actors are rigid bodies.
    ///
    /// Only takes effect if eDETECT_DISCRETE_CONTACT or eDETECT_CCD_CONTACT is raised
    NotifyTouchFound = 4,
    /// Call contact report callback while this collision pair is in contact
    ///
    /// If none of the two collision objects is a trigger shape then the contact report callback will get
    /// called while the actors of this collision pair are in contact.
    ///
    /// Triggers do not support this event. Persistent trigger contacts need to be tracked separately by observing eNOTIFY_TOUCH_FOUND/eNOTIFY_TOUCH_LOST events.
    ///
    /// Only takes effect if the colliding actors are rigid bodies.
    ///
    /// No report will get sent if the objects in contact are sleeping.
    ///
    /// Only takes effect if eDETECT_DISCRETE_CONTACT or eDETECT_CCD_CONTACT is raised
    ///
    /// If this flag gets enabled while a pair is in touch already, there will be no eNOTIFY_TOUCH_PERSISTS events until the pair loses and regains touch.
    NotifyTouchPersists = 8,
    /// Call contact report callback or trigger callback when this collision pair stops to be in contact
    ///
    /// If one of the two collision objects is a trigger shape (see [`PxShapeFlag::eTRIGGER_SHAPE`])
    /// then the trigger callback will get called as soon as the other object leaves the trigger volume.
    /// If none of the two collision objects is a trigger shape then the contact report callback will get
    /// called when the actors of this collision pair stop to be in contact.
    ///
    /// Only takes effect if the colliding actors are rigid bodies.
    ///
    /// This event will also get triggered if one of the colliding objects gets deleted.
    ///
    /// Only takes effect if eDETECT_DISCRETE_CONTACT or eDETECT_CCD_CONTACT is raised
    NotifyTouchLost = 16,
    /// Call contact report callback when this collision pair is in contact during CCD passes.
    ///
    /// If CCD with multiple passes is enabled, then a fast moving object might bounce on and off the same
    /// object multiple times. Hence, the same pair might be in contact multiple times during a simulation step.
    /// This flag will make sure that all the detected collision during CCD will get reported. For performance
    /// reasons, the system can not always tell whether the contact pair lost touch in one of the previous CCD
    /// passes and thus can also not always tell whether the contact is new or has persisted. eNOTIFY_TOUCH_CCD
    /// just reports when the two collision objects were detected as being in contact during a CCD pass.
    ///
    /// Only takes effect if the colliding actors are rigid bodies.
    ///
    /// Trigger shapes are not supported.
    ///
    /// Only takes effect if eDETECT_CCD_CONTACT is raised
    NotifyTouchCcd = 32,
    /// Call contact report callback when the contact force between the actors of this collision pair exceeds one of the actor-defined force thresholds.
    ///
    /// Only takes effect if the colliding actors are rigid bodies.
    ///
    /// Only takes effect if eDETECT_DISCRETE_CONTACT or eDETECT_CCD_CONTACT is raised
    ///
    /// Only works with PGS solver, and only on CPU.
    NotifyThresholdForceFound = 64,
    /// Call contact report callback when the contact force between the actors of this collision pair continues to exceed one of the actor-defined force thresholds.
    ///
    /// Only takes effect if the colliding actors are rigid bodies.
    ///
    /// If a pair gets re-filtered and this flag has previously been disabled, then the report will not get fired in the same frame even if the force threshold has been reached in the
    /// previous one (unless [`eNOTIFY_THRESHOLD_FORCE_FOUND`] has been set in the previous frame).
    ///
    /// Only takes effect if eDETECT_DISCRETE_CONTACT or eDETECT_CCD_CONTACT is raised
    ///
    /// Only works with PGS solver, and only on CPU.
    NotifyThresholdForcePersists = 128,
    /// Call contact report callback when the contact force between the actors of this collision pair falls below one of the actor-defined force thresholds (includes the case where this collision pair stops being in contact).
    ///
    /// Only takes effect if the colliding actors are rigid bodies.
    ///
    /// If a pair gets re-filtered and this flag has previously been disabled, then the report will not get fired in the same frame even if the force threshold has been reached in the
    /// previous one (unless [`eNOTIFY_THRESHOLD_FORCE_FOUND`] or #eNOTIFY_THRESHOLD_FORCE_PERSISTS has been set in the previous frame).
    ///
    /// Only takes effect if eDETECT_DISCRETE_CONTACT or eDETECT_CCD_CONTACT is raised
    ///
    /// Only works with PGS solver, and only on CPU.
    NotifyThresholdForceLost = 256,
    /// Provide contact points in contact reports for this collision pair.
    ///
    /// Only takes effect if the colliding actors are rigid bodies and if used in combination with the flags eNOTIFY_TOUCH_... or eNOTIFY_THRESHOLD_FORCE_...
    ///
    /// Only takes effect if eDETECT_DISCRETE_CONTACT or eDETECT_CCD_CONTACT is raised
    NotifyContactPoints = 512,
    /// This flag is used to indicate whether this pair generates discrete collision detection contacts.
    ///
    /// Contacts are only responded to if eSOLVE_CONTACT is enabled.
    DetectDiscreteContact = 1024,
    /// This flag is used to indicate whether this pair generates CCD contacts.
    ///
    /// The contacts will only be responded to if eSOLVE_CONTACT is enabled on this pair.
    ///
    /// The scene must have PxSceneFlag::eENABLE_CCD enabled to use this feature.
    ///
    /// Non-static bodies of the pair should have PxRigidBodyFlag::eENABLE_CCD specified for this feature to work correctly.
    ///
    /// This flag is not supported with trigger shapes. However, CCD trigger events can be emulated using non-trigger shapes
    /// and requesting eNOTIFY_TOUCH_FOUND and eNOTIFY_TOUCH_LOST and not raising eSOLVE_CONTACT on the pair.
    DetectCcdContact = 2048,
    /// Provide pre solver velocities in contact reports for this collision pair.
    ///
    /// If the collision pair has contact reports enabled, the velocities of the rigid bodies before contacts have been solved
    /// will be provided in the contact report callback unless the pair lost touch in which case no data will be provided.
    ///
    /// Usually it is not necessary to request these velocities as they will be available by querying the velocity from the provided
    /// PxRigidActor object directly. However, it might be the case that the velocity of a rigid body gets set while the simulation is running
    /// in which case the PxRigidActor would return this new velocity in the contact report callback and not the velocity the simulation used.
    PreSolverVelocity = 4096,
    /// Provide post solver velocities in contact reports for this collision pair.
    ///
    /// If the collision pair has contact reports enabled, the velocities of the rigid bodies after contacts have been solved
    /// will be provided in the contact report callback unless the pair lost touch in which case no data will be provided.
    PostSolverVelocity = 8192,
    /// Provide rigid body poses in contact reports for this collision pair.
    ///
    /// If the collision pair has contact reports enabled, the rigid body poses at the contact event will be provided
    /// in the contact report callback unless the pair lost touch in which case no data will be provided.
    ///
    /// Usually it is not necessary to request these poses as they will be available by querying the pose from the provided
    /// PxRigidActor object directly. However, it might be the case that the pose of a rigid body gets set while the simulation is running
    /// in which case the PxRigidActor would return this new pose in the contact report callback and not the pose the simulation used.
    /// Another use case is related to CCD with multiple passes enabled, A fast moving object might bounce on and off the same
    /// object multiple times. This flag can be used to request the rigid body poses at the time of impact for each such collision event.
    ContactEventPose = 16384,
    /// For internal use only.
    NextFree = 32768,
    /// Provided default flag to do simple contact processing for this collision pair.
    ContactDefault = 1025,
    /// Provided default flag to get commonly used trigger behavior for this collision pair.
    TriggerDefault = 1044,
}

bitflags::bitflags! {
    /// Flags for [`PxPairFlag`]
    #[derive(Default)]
    #[repr(transparent)]
    pub struct PxPairFlags: u16 {
        const SolveContact = 1 << 0;
        const ModifyContacts = 1 << 1;
        const NotifyTouchFound = 1 << 2;
        const NotifyTouchPersists = 1 << 3;
        const NotifyTouchLost = 1 << 4;
        const NotifyTouchCcd = 1 << 5;
        const NotifyThresholdForceFound = 1 << 6;
        const NotifyThresholdForcePersists = 1 << 7;
        const NotifyThresholdForceLost = 1 << 8;
        const NotifyContactPoints = 1 << 9;
        const DetectDiscreteContact = 1 << 10;
        const DetectCcdContact = 1 << 11;
        const PreSolverVelocity = 1 << 12;
        const PostSolverVelocity = 1 << 13;
        const ContactEventPose = 1 << 14;
        const NextFree = 1 << 15;
        const ContactDefault = Self::SolveContact.bits | Self::DetectDiscreteContact.bits;
        const TriggerDefault = Self::NotifyTouchFound.bits | Self::NotifyTouchLost.bits | Self::DetectDiscreteContact.bits;
    }
}

/// Collection of flags describing the filter actions to take for a collision pair.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PxFilterFlag {
    /// Ignore the collision pair as long as the bounding volumes of the pair objects overlap.
    ///
    /// Killed pairs will be ignored by the simulation and won't run through the filter again until one
    /// of the following occurs:
    ///
    /// The bounding volumes of the two objects overlap again (after being separated)
    ///
    /// The user enforces a re-filtering (see [`PxScene::resetFiltering`]())
    Kill = 1,
    /// Ignore the collision pair as long as the bounding volumes of the pair objects overlap or until filtering relevant data changes for one of the collision objects.
    ///
    /// Suppressed pairs will be ignored by the simulation and won't make another filter request until one
    /// of the following occurs:
    ///
    /// Same conditions as for killed pairs (see [`eKILL`])
    ///
    /// The filter data or the filter object attributes change for one of the collision objects
    Suppress = 2,
    /// Invoke the filter callback ([`PxSimulationFilterCallback::pairFound`]()) for this collision pair.
    Callback = 4,
    /// Track this collision pair with the filter callback mechanism.
    ///
    /// When the bounding volumes of the collision pair lose contact, the filter callback [`PxSimulationFilterCallback::pairLost`]()
    /// will be invoked. Furthermore, the filter status of the collision pair can be adjusted through [`PxSimulationFilterCallback::statusChange`]()
    /// once per frame (until a pairLost() notification occurs).
    Notify = 12,
    /// Provided default to get standard behavior:
    ///
    /// The application configure the pair's collision properties once when bounding volume overlap is found and
    /// doesn't get asked again about that pair until overlap status or filter properties changes, or re-filtering is requested.
    ///
    /// No notification is provided when bounding volume overlap is lost
    ///
    /// The pair will not be killed or suppressed, so collision detection will be processed
    Default = 0,
}

bitflags::bitflags! {
    /// Flags for [`PxFilterFlag`]
    #[derive(Default)]
    #[repr(transparent)]
    pub struct PxFilterFlags: u16 {
        const Kill = 1 << 0;
        const Suppress = 1 << 1;
        const Callback = 1 << 2;
        const Notify = Self::Callback.bits;
    }
}

/// Identifies each type of filter object.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PxFilterObjectType {
    /// A static rigid body
    RigidStatic = 0,
    /// A dynamic rigid body
    RigidDynamic = 1,
    /// An articulation
    Articulation = 2,
    /// A deformable surface
    DeformableSurface = 3,
    /// A deformable volume
    DeformableVolume = 4,
    /// A particle system
    Particlesystem = 5,
    /// internal use only!
    MaxTypeCount = 16,
    /// internal use only!
    Undefined = 15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PxFilterObjectFlag {
    Kinematic = 16,
    Trigger = 32,
    CustomGeometry = 64,
    NextFree = 128,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PxPairFilteringMode {
    /// Output pair from BP, potentially send to user callbacks, create regular interaction object.
    ///
    /// Enable contact pair filtering between kinematic/static or kinematic/kinematic rigid bodies.
    ///
    /// By default contacts between these are suppressed (see [`PxFilterFlag::eSUPPRESS`]) and don't get reported to the filter mechanism.
    /// Use this mode if these pairs should go through the filtering pipeline nonetheless.
    ///
    /// This mode is not mutable, and must be set in PxSceneDesc at scene creation.
    Keep = 0,
    /// Output pair from BP, create interaction marker. Can be later switched to regular interaction.
    Suppress = 1,
    /// Don't output pair from BP. Cannot be later switched to regular interaction, needs "resetFiltering" call.
    Kill = 2,
    /// Default is eSUPPRESS for compatibility with previous PhysX versions.
    Default = 1,
}

/// Flags which control the behavior of an actor.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PxActorFlag {
    /// Enable debug renderer for this actor
    Visualization = 1,
    /// Disables scene gravity for this actor
    DisableGravity = 2,
    /// Enables the sending of PxSimulationEventCallback::onWake() and PxSimulationEventCallback::onSleep() notify events
    SendSleepNotifies = 4,
    /// Disables simulation for the actor.
    ///
    /// This is only supported by PxRigidStatic and PxRigidDynamic actors and can be used to reduce the memory footprint when rigid actors are
    /// used for scene queries only.
    ///
    /// Setting this flag will remove all constraints attached to the actor from the scene.
    ///
    /// If this flag is set, the following calls are forbidden:
    ///
    /// PxRigidBody: setLinearVelocity(), setAngularVelocity(), addForce(), addTorque(), clearForce(), clearTorque(), setForceAndTorque()
    ///
    /// PxRigidDynamic: setKinematicTarget(), setWakeCounter(), wakeUp(), putToSleep()
    ///
    /// Sleeping:
    /// Raising this flag will set all velocities and the wake counter to 0, clear all forces, clear the kinematic target, put the actor
    /// to sleep and wake up all touching actors from the previous frame.
    DisableSimulation = 8,
}

bitflags::bitflags! {
    /// Flags for [`PxActorFlag`]
    #[derive(Default)]
    #[repr(transparent)]
    pub struct PxActorFlags: u8 {
        const Visualization = 1 << 0;
        const DisableGravity = 1 << 1;
        const SendSleepNotifies = 1 << 2;
        const DisableSimulation = 1 << 3;
    }
}

/// Identifies each type of actor.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PxActorType {
    /// A static rigid body
    RigidStatic = 0,
    /// A dynamic rigid body
    RigidDynamic = 1,
    /// An articulation link
    ArticulationLink = 2,
    /// A deformable surface
    DeformableSurface = 3,
    /// A deformable volume
    DeformableVolume = 4,
    /// A PBD ParticleSystem
    PbdParticlesystem = 5,
    /// internal use only!
    ActorCount = 6,
    /// internal use only!
    ActorForceDword = 2147483647,
}

/// Flags which control the behaviour of a particle system.
///
/// See [`PxPBDParticleSystem::setParticleFlag`](), #PxPBDParticleSystem::setParticleFlags(), #PxPBDParticleSystem::getParticleFlags()
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PxParticleFlag {
    /// Disables particle self-collision
    DisableSelfCollision = 1,
    /// Disables particle-rigid body collision
    DisableRigidCollision = 2,
    /// Enables full advection of diffuse particles. By default, diffuse particles are advected only by particles in the cell they are contained. This flag enables full neighbourhood generation (more expensive).
    FullDiffuseAdvection = 4,
    /// Enables speculative CCD for particle-rigid body collision.
    EnableSpeculativeCcd = 8,
}

bitflags::bitflags! {
    /// Flags for [`PxParticleFlag`]
    #[derive(Default)]
    #[repr(transparent)]
    pub struct PxParticleFlags: u32 {
        const DisableSelfCollision = 1 << 0;
        const DisableRigidCollision = 1 << 1;
        const FullDiffuseAdvection = 1 << 2;
        const EnableSpeculativeCcd = 1 << 3;
    }
}

/// Collection of flags providing a mechanism to lock motion along a specific axis.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PxParticleLockFlag {
    LockX = 1,
    LockY = 2,
    LockZ = 4,
}

bitflags::bitflags! {
    /// Flags for [`PxParticleLockFlag`]
    #[derive(Default)]
    #[repr(transparent)]
    pub struct PxParticleLockFlags: u8 {
        const LockX = 1 << 0;
        const LockY = 1 << 1;
        const LockZ = 1 << 2;
    }
}

/// Scene query and geometry query behavior flags.
///
/// PxHitFlags are used for 3 different purposes:
///
/// 1) To request hit fields to be filled in by scene queries (such as hit position, normal, face index or UVs).
/// 2) Once query is completed, to indicate which fields are valid (note that a query may produce more valid fields than requested).
/// 3) To specify additional options for the narrow phase and mid-phase intersection routines.
///
/// All these flags apply to both scene queries and geometry queries (PxGeometryQuery).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PxHitFlag {
    /// "position" member of [`PxQueryHit`] is valid
    Position = 1,
    /// "normal" member of [`PxQueryHit`] is valid
    Normal = 2,
    /// "u" and "v" barycentric coordinates of [`PxQueryHit`] are valid. Not applicable to sweep queries.
    Uv = 8,
    /// Performance hint flag for sweeps when it is known upfront there's no initial overlap.
    /// NOTE: using this flag may cause undefined results if shapes are initially overlapping.
    AssumeNoInitialOverlap = 16,
    /// Report any first hit. Used for geometries that contain more than one primitive. For meshes,
    /// if neither eMESH_MULTIPLE nor eANY_HIT is specified, a single closest hit will be reported.
    AnyHit = 32,
    /// Report all hits for meshes rather than just the first. Not applicable to sweep queries.
    MeshMultiple = 64,
    /// Report hits with back faces of mesh triangles. Also report hits for raycast
    /// originating on mesh surface and facing away from the surface normal. Not applicable to sweep queries.
    /// Please refer to the user guide for heightfield-specific differences.
    MeshBothSides = 128,
    /// Use more accurate but slower narrow phase sweep tests.
    /// May provide better compatibility with PhysX 3.2 sweep behavior.
    PreciseSweep = 256,
    /// Report the minimum translation depth, normal and contact point.
    Mtd = 512,
    /// "face index" member of [`PxQueryHit`] is valid
    FaceIndex = 1024,
    Default = 1027,
    /// Only this subset of flags can be modified by pre-filter. Other modifications will be discarded.
    ModifiableFlags = 464,
}

bitflags::bitflags! {
    /// Flags for [`PxHitFlag`]
    #[derive(Default)]
    #[repr(transparent)]
    pub struct PxHitFlags: u16 {
        const Position = 1 << 0;
        const Normal = 1 << 1;
        const Uv = 1 << 3;
        const AssumeNoInitialOverlap = 1 << 4;
        const AnyHit = 1 << 5;
        const MeshMultiple = 1 << 6;
        const MeshBothSides = 1 << 7;
        const PreciseSweep = 1 << 8;
        const Mtd = 1 << 9;
        const FaceIndex = 1 << 10;
        const Default = Self::Position.bits | Self::Normal.bits | Self::FaceIndex.bits;
        const ModifiableFlags = Self::AssumeNoInitialOverlap.bits | Self::MeshMultiple.bits | Self::MeshBothSides.bits | Self::PreciseSweep.bits;
    }
}

/// Enumeration of core types for convex core geometries.
///
/// This enum defines the various cores that can be used as the basis
/// for creating convex core geometries. Each type represents a different
/// fundamental shape that can be extended with a margin to create more
/// complex convex shapes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PxConvexCore {
    Point = 0,
    Segment = 1,
    Box = 2,
    Ellipsoid = 3,
    Cylinder = 4,
    Cone = 5,
    Count = 6,
}

/// Describes the format of height field samples.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PxHeightFieldFormat {
    /// Height field height data is 16 bit signed integers, followed by triangle materials.
    ///
    /// Each sample is 32 bits wide arranged as follows:
    ///
    /// 1) First there is a 16 bit height value.
    /// 2) Next, two one byte material indices, with the high bit of each byte reserved for special use.
    /// (so the material index is only 7 bits).
    /// The high bit of material0 is the tess-flag.
    /// The high bit of material1 is reserved for future use.
    ///
    /// There are zero or more unused bytes before the next sample depending on PxHeightFieldDesc.sampleStride,
    /// where the application may eventually keep its own data.
    ///
    /// This is the only format supported at the moment.
    S16Tm = 1,
}

/// Determines the tessellation of height field cells.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PxHeightFieldTessFlag {
    /// This flag determines which way each quad cell is subdivided.
    ///
    /// The flag lowered indicates subdivision like this: (the 0th vertex is referenced by only one triangle)
    ///
    /// +--+--+--+---> column
    /// | /| /| /|
    /// |/ |/ |/ |
    /// +--+--+--+
    /// | /| /| /|
    /// |/ |/ |/ |
    /// +--+--+--+
    /// |
    /// |
    /// V row
    ///
    /// The flag raised indicates subdivision like this: (the 0th vertex is shared by two triangles)
    ///
    /// +--+--+--+---> column
    /// |
    /// \
    /// |
    /// \
    /// |
    /// \
    /// |
    /// |
    /// \
    /// |
    /// \
    /// |
    /// \
    /// |
    /// +--+--+--+
    /// |
    /// \
    /// |
    /// \
    /// |
    /// \
    /// |
    /// |
    /// \
    /// |
    /// \
    /// |
    /// \
    /// |
    /// +--+--+--+
    /// |
    /// |
    /// V row
    E0ThVertexShared = 1,
}

/// Enum with flag values to be used in PxHeightFieldDesc.flags.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PxHeightFieldFlag {
    /// Disable collisions with height field with boundary edges.
    ///
    /// Raise this flag if several terrain patches are going to be placed adjacent to each other,
    /// to avoid a bump when sliding across.
    ///
    /// This flag is ignored in contact generation with sphere and capsule shapes.
    NoBoundaryEdges = 1,
}

bitflags::bitflags! {
    /// Flags for [`PxHeightFieldFlag`]
    #[derive(Default)]
    #[repr(transparent)]
    pub struct PxHeightFieldFlags: u16 {
        const NoBoundaryEdges = 1 << 0;
    }
}

/// Special material index values for height field samples.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PxHeightFieldMaterial {
    /// A material indicating that the triangle should be treated as a hole in the mesh.
    Hole = 127,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PxMeshMeshQueryFlag {
    /// Report all overlaps
    Default = 0,
    /// Ignore coplanar triangle-triangle overlaps
    DiscardCoplanar = 1,
    /// Reserved flag
    Reserved = 2,
    /// Reserved flag
    Reserved1 = 2,
    /// Reserved flag
    Reserved2 = 4,
    /// Reserved flag
    Reserved3 = 8,
}

bitflags::bitflags! {
    /// Flags for [`PxMeshMeshQueryFlag`]
    #[derive(Default)]
    #[repr(transparent)]
    pub struct PxMeshMeshQueryFlags: u32 {
        const DiscardCoplanar = 1 << 0;
        const Reserved = 1 << 1;
        const Reserved1 = 1 << 1;
        const Reserved2 = 1 << 2;
        const Reserved3 = 1 << 3;
    }
}

/// Enum with flag values to be used in PxSimpleTriangleMesh::flags.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PxMeshFlag {
    /// Specifies if the SDK should flip normals.
    ///
    /// The PhysX libraries assume that the face normal of a triangle with vertices [a,b,c] can be computed as:
    /// edge1 = b-a
    /// edge2 = c-a
    /// face_normal = edge1 x edge2.
    ///
    /// Note: This is the same as a counterclockwise winding in a right handed coordinate system or
    /// alternatively a clockwise winding order in a left handed coordinate system.
    ///
    /// If this does not match the winding order for your triangles, raise the below flag.
    Flipnormals = 1,
    /// Denotes the use of 16-bit vertex indices
    E16BitIndices = 2,
}

bitflags::bitflags! {
    /// Flags for [`PxMeshFlag`]
    #[derive(Default)]
    #[repr(transparent)]
    pub struct PxMeshFlags: u16 {
        const Flipnormals = 1 << 0;
        const E16BitIndices = 1 << 1;
    }
}

/// Mesh midphase structure. This enum is used to select the desired acceleration structure for midphase queries
/// (i.e. raycasts, overlaps, sweeps vs triangle meshes).
///
/// The PxMeshMidPhase::eBVH33 structure is the one used in recent PhysX versions (up to PhysX 3.3). It has great performance and is
/// supported on all platforms. It is deprecated since PhysX 5.x.
///
/// The PxMeshMidPhase::eBVH34 structure is a revisited implementation introduced in PhysX 3.4. It can be significantly faster both
/// in terms of cooking performance and runtime performance.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PxMeshMidPhase {
    /// Use eBVH34 instead. Used to be default midphase mesh structure up to PhysX 3.3
    Bvh33 = 0,
    /// New midphase mesh structure, introduced in PhysX 3.4
    Bvh34 = 1,
    Last = 2,
}

/// Flags for the mesh geometry properties.
///
/// Used in ::PxTriangleMeshFlags.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PxTriangleMeshFlag {
    /// The triangle mesh has 16bits vertex indices.
    E16BitIndices = 2,
    /// The triangle mesh has adjacency information build.
    AdjacencyInfo = 4,
    /// Indicates that this mesh would preferably not be the mesh projected for mesh-mesh collision. This can indicate that the mesh is not well tessellated.
    PreferNoSdfProj = 8,
}

bitflags::bitflags! {
    /// Flags for [`PxTriangleMeshFlag`]
    #[derive(Default)]
    #[repr(transparent)]
    pub struct PxTriangleMeshFlags: u8 {
        const E16BitIndices = 1 << 1;
        const AdjacencyInfo = 1 << 2;
        const PreferNoSdfProj = 1 << 3;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PxTetrahedronMeshFlag {
    /// The tetrahedron mesh has 16bits vertex indices
    E16BitIndices = 2,
}

bitflags::bitflags! {
    /// Flags for [`PxTetrahedronMeshFlag`]
    #[derive(Default)]
    #[repr(transparent)]
    pub struct PxTetrahedronMeshFlags: u8 {
        const E16BitIndices = 1 << 1;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PxAggregateType {
    /// Aggregate will contain various actors of unspecified types
    Generic = 0,
    /// Aggregate will only contain static actors
    Static = 1,
    /// Aggregate will only contain kinematic actors
    Kinematic = 2,
}

/// Constraint row flags
///
/// These flags configure the post-processing of constraint rows and the behavior of the solver while solving constraints
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum Px1DConstraintFlag {
    /// whether the constraint is a spring. Mutually exclusive with eRESTITUTION. If set, eKEEPBIAS is ignored.
    Spring = 1,
    /// whether the constraint is a force or acceleration spring. Only valid if eSPRING is set.
    AccelerationSpring = 2,
    /// whether the restitution model should be applied to generate the target velocity. Mutually exclusive with eSPRING. If restitution causes a bounces, eKEEPBIAS is ignored
    Restitution = 4,
    /// whether to keep the error term when solving for velocity. Ignored if restitution generates bounce, or eSPRING is set.
    Keepbias = 8,
    /// whether to accumulate the force value from this constraint in the force total that is reported for the constraint and tested for breakage
    OutputForce = 16,
    /// whether the constraint has a drive force limit (which will be scaled by dt unless [`PxConstraintFlag::eDRIVE_LIMITS_ARE_FORCES`] is set)
    HasDriveLimit = 32,
    /// whether this is an angular or linear constraint
    AngularConstraint = 64,
}

bitflags::bitflags! {
    /// Flags for [`Px1DConstraintFlag`]
    #[derive(Default)]
    #[repr(transparent)]
    pub struct Px1DConstraintFlags: u16 {
        const Spring = 1 << 0;
        const AccelerationSpring = 1 << 1;
        const Restitution = 1 << 2;
        const Keepbias = 1 << 3;
        const OutputForce = 1 << 4;
        const HasDriveLimit = 1 << 5;
        const AngularConstraint = 1 << 6;
    }
}

/// Constraint type hints which the solver uses to optimize constraint handling
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PxConstraintSolveHint {
    /// no special properties
    None = 0,
    /// a group of acceleration drive constraints with the same stiffness and drive parameters
    Acceleration1 = 256,
    /// temporary special value to identify SLERP drive rows
    SlerpSpring = 258,
    /// a group of acceleration drive constraints with the same stiffness and drive parameters
    Acceleration2 = 512,
    /// a group of acceleration drive constraints with the same stiffness and drive parameters
    Acceleration3 = 768,
    /// for internal purpose only, please do not use.
    RotationalEquality = 1024,
    /// for internal purpose only, please do not use.
    RotationalInequality = 1025,
    /// Mark as equality constraint.
    ///
    /// If a 1D constraint is an equality constraint with [-PX_MAX_FLT, PX_MAX_FLT] force limits and a velocity target equal zero, then this
    /// flag can be raised to allow the solver to internally change the jacobian of this constraint and have it being orthogonalized relative
    /// to other equality constraints in the same PxConstraint (unless PxConstraintFlag::eDISABLE_PREPROCESSING is set). This can improve
    /// the convergence when solving the constraints.
    Equality = 2048,
    /// Mark as inequality constraint.
    ///
    /// If a 1D constraint is an inequality constraint with [0, PX_MAX_FLT] force limits, then this flag can be raised to allow the solver
    /// to internally change the jacobian of this constraint and have it being orthogonalized relative to the equality constraints in the
    /// same PxConstraint (unless PxConstraintFlag::eDISABLE_PREPROCESSING is set). This can improve the convergence when solving the
    /// constraints.
    Inequality = 2049,
}

/// Flags for determining which components of the constraint should be visualized.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PxConstraintVisualizationFlag {
    /// visualize constraint frames
    LocalFrames = 1,
    /// visualize constraint limits
    Limits = 2,
}

/// Flags for determining how PVD should serialize a constraint update
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PxPvdUpdateType {
    /// triggers createPvdInstance call, creates an instance of a constraint
    CreateInstance = 0,
    /// triggers releasePvdInstance call, releases an instance of a constraint
    ReleaseInstance = 1,
    /// triggers updatePvdProperties call, updates all properties of a constraint
    UpdateAllProperties = 2,
    /// triggers simUpdate call, updates all simulation properties of a constraint
    UpdateSimProperties = 3,
}

/// Constraint descriptor used inside the solver
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum ConstraintType {
    /// Defines this pair is a contact constraint
    ContactConstraint = 0,
    /// Defines this pair is a joint constraint
    JointConstraint = 1,
}

/// Data structure used for preparing constraints before solving them
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum BodyState {
    DynamicBody = 1,
    StaticBody = 2,
    KinematicBody = 4,
    Articulation = 8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PxArticulationAxis {
    /// Rotational about eX
    Twist = 0,
    /// Rotational about eY
    Swing1 = 1,
    /// Rotational about eZ
    Swing2 = 2,
    /// Linear in eX
    X = 3,
    /// Linear in eY
    Y = 4,
    /// Linear in eZ
    Z = 5,
    Count = 6,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PxArticulationMotion {
    /// Locked axis, i.e. degree of freedom (DOF)
    Locked = 0,
    /// Limited DOF - set limits of joint DOF together with this flag, see PxArticulationJointReducedCoordinate::setLimitParams
    Limited = 1,
    /// Free DOF
    Free = 2,
}

bitflags::bitflags! {
    /// Flags for [`PxArticulationMotion`]
    #[derive(Default)]
    #[repr(transparent)]
    pub struct PxArticulationMotions: u8 {
        const Limited = 1 << 0;
        const Free = 1 << 1;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PxArticulationJointType {
    /// All joint axes, i.e. degrees of freedom (DOFs) locked
    Fix = 0,
    /// Single linear DOF, e.g. cart on a rail
    Prismatic = 1,
    /// Single rotational DOF, e.g. an elbow joint or a rotational motor, position wrapped at 2pi radians
    Revolute = 2,
    /// Single rotational DOF, e.g. an elbow joint or a rotational motor, position not wrapped
    RevoluteUnwrapped = 3,
    /// Ball and socket joint with two or three DOFs
    Spherical = 4,
    Undefined = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PxArticulationFlag {
    /// Set articulation base to be fixed.
    FixBase = 1,
    /// Limits for drive effort are forces and torques rather than impulses, see PxArticulationDrive::maxForce.
    DriveLimitsAreForces = 2,
    /// Disable collisions between the articulation's links (note that parent/child collisions are disabled internally in either case).
    DisableSelfCollision = 4,
}

bitflags::bitflags! {
    /// Flags for [`PxArticulationFlag`]
    #[derive(Default)]
    #[repr(transparent)]
    pub struct PxArticulationFlags: u8 {
        const FixBase = 1 << 0;
        const DriveLimitsAreForces = 1 << 1;
        const DisableSelfCollision = 1 << 2;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PxArticulationDriveType {
    /// The output of the implicit spring drive controller is a force/torque.
    Force = 0,
    /// The output of the implicit spring drive controller is a joint acceleration (use this to get (spatial)-inertia-invariant behavior of the drive).
    Acceleration = 1,
    None = 2,
}

/// These flags determine what data is read or written to the internal articulation data via cache.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PxArticulationCacheFlag {
    /// The joint velocities, see PxArticulationCache::jointVelocity.
    Velocity = 1,
    /// The joint accelerations, see PxArticulationCache::jointAcceleration.
    Acceleration = 2,
    /// The joint positions, see PxArticulationCache::jointPosition.
    Position = 4,
    /// The joint forces, see PxArticulationCache::jointForce.
    Force = 8,
    /// The link velocities, see PxArticulationCache::linkVelocity. Link velocities cannot be set except for the root link velocity via  PxArticulationCache::rootLinkData.
    LinkVelocity = 16,
    /// The link accelerations, see PxArticulationCache::linkAcceleration.
    LinkAcceleration = 32,
    /// The root link transform, see PxArticulationCache::rootLinkData.
    RootTransform = 64,
    /// The root link velocities (read/write) and accelerations (read), see PxArticulationCache::rootLinkData.
    RootVelocities = 128,
    /// The link incoming joint forces, see PxArticulationCache::linkIncomingJointForce.
    LinkIncomingJointForce = 1024,
    /// The joint target positions, see PxArticulationCache::jointTargetPositions.
    JointTargetPositions = 2048,
    /// The joint target velocities, see PxArticulationCache::jointTargetVelocities.
    JointTargetVelocities = 4096,
    /// The link forces, see PxArticulationCache::linkForce.
    LinkForce = 8192,
    /// The link torques, see PxArticulationCache::linkTorque.
    LinkTorque = 16384,
    All = 247,
}

bitflags::bitflags! {
    /// Flags for [`PxArticulationCacheFlag`]
    #[derive(Default)]
    #[repr(transparent)]
    pub struct PxArticulationCacheFlags: u32 {
        const Velocity = 1 << 0;
        const Acceleration = 1 << 1;
        const Position = 1 << 2;
        const Force = 1 << 3;
        const LinkVelocity = 1 << 4;
        const LinkAcceleration = 1 << 5;
        const RootTransform = 1 << 6;
        const RootVelocities = 1 << 7;
        const LinkIncomingJointForce = 1 << 10;
        const JointTargetPositions = 1 << 11;
        const JointTargetVelocities = 1 << 12;
        const LinkForce = 1 << 13;
        const LinkTorque = 1 << 14;
        const All = Self::Velocity.bits | Self::Acceleration.bits | Self::Position.bits | Self::LinkVelocity.bits | Self::LinkAcceleration.bits | Self::RootTransform.bits | Self::RootVelocities.bits;
    }
}

/// Flag that configures articulation-state updates by PxArticulationReducedCoordinate::updateKinematic.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PxArticulationKinematicFlag {
    /// Raise after any changes to the articulation root or joint positions using non-cache API calls. Updates links' positions and velocities.
    Position = 1,
    /// Raise after velocity-only changes to the articulation root or joints using non-cache API calls. Updates links' velocities.
    Velocity = 2,
}

bitflags::bitflags! {
    /// Flags for [`PxArticulationKinematicFlag`]
    #[derive(Default)]
    #[repr(transparent)]
    pub struct PxArticulationKinematicFlags: u8 {
        const Position = 1 << 0;
        const Velocity = 1 << 1;
    }
}

/// Flags which affect the behavior of PxShapes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PxShapeFlag {
    /// The shape will partake in collision in the physical simulation.
    ///
    /// It is illegal to raise the eSIMULATION_SHAPE and eTRIGGER_SHAPE flags.
    /// In the event that one of these flags is already raised the sdk will reject any
    /// attempt to raise the other.  To raise the eSIMULATION_SHAPE first ensure that
    /// eTRIGGER_SHAPE is already lowered.
    ///
    /// This flag has no effect if simulation is disabled for the corresponding actor (see [`PxActorFlag::eDISABLE_SIMULATION`]).
    SimulationShape = 1,
    /// The shape will partake in scene queries (ray casts, overlap tests, sweeps, ...).
    SceneQueryShape = 2,
    /// The shape is a trigger which can send reports whenever other shapes enter/leave its volume.
    ///
    /// Triangle meshes and heightfields can not be triggers. Shape creation will fail in these cases.
    ///
    /// Shapes marked as triggers do not collide with other objects. If an object should act both
    /// as a trigger shape and a collision shape then create a rigid body with two shapes, one being a
    /// trigger shape and the other a collision shape. 	It is illegal to raise the eTRIGGER_SHAPE and
    /// eSIMULATION_SHAPE flags on a single PxShape instance.  In the event that one of these flags is already
    /// raised the sdk will reject any attempt to raise the other.  To raise the eTRIGGER_SHAPE flag first
    /// ensure that eSIMULATION_SHAPE flag is already lowered.
    ///
    /// Trigger shapes will no longer send notification events for interactions with other trigger shapes.
    ///
    /// Shapes marked as triggers are allowed to participate in scene queries, provided the eSCENE_QUERY_SHAPE flag is set.
    ///
    /// This flag has no effect if simulation is disabled for the corresponding actor (see [`PxActorFlag::eDISABLE_SIMULATION`]).
    TriggerShape = 4,
    /// Enable debug renderer for this shape
    Visualization = 8,
}

bitflags::bitflags! {
    /// Flags for [`PxShapeFlag`]
    #[derive(Default)]
    #[repr(transparent)]
    pub struct PxShapeFlags: u8 {
        const SimulationShape = 1 << 0;
        const SceneQueryShape = 1 << 1;
        const TriggerShape = 1 << 2;
        const Visualization = 1 << 3;
    }
}

/// Parameter to addForce() and addTorque() calls, determines the exact operation that is carried out.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PxForceMode {
    /// parameter has unit of mass * length / time^2, i.e., a force
    Force = 0,
    /// parameter has unit of mass * length / time, i.e., force * time
    Impulse = 1,
    /// parameter has unit of length / time, i.e., the effect is mass independent: a velocity change.
    VelocityChange = 2,
    /// parameter has unit of length/ time^2, i.e., an acceleration. It gets treated just like a force except the mass is not divided out before integration.
    Acceleration = 3,
}

/// Collection of flags describing the behavior of a rigid body.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PxRigidBodyFlag {
    /// Enable kinematic mode for the body.
    Kinematic = 1,
    /// Use the kinematic target transform for scene queries.
    ///
    /// If this flag is raised, then scene queries will treat the kinematic target transform as the current pose
    /// of the body (instead of using the actual pose). Without this flag, the kinematic target will only take
    /// effect with respect to scene queries after a simulation step.
    UseKinematicTargetForSceneQueries = 2,
    /// Enable CCD for the body.
    EnableCcd = 4,
    /// Enabled CCD in swept integration for the actor.
    ///
    /// If this flag is raised and CCD is enabled, CCD interactions will simulate friction. By default, friction is disabled in CCD interactions because
    /// CCD friction has been observed to introduce some simulation artifacts. CCD friction was enabled in previous versions of the SDK. Raising this flag will result in behavior
    /// that is a closer match for previous versions of the SDK.
    ///
    /// This flag requires PxRigidBodyFlag::eENABLE_CCD to be raised to have any effect.
    EnableCcdFriction = 8,
    /// Register a rigid body to dynamically adjust contact offset based on velocity. This can be used to achieve a CCD effect.
    ///
    /// If both eENABLE_CCD and eENABLE_SPECULATIVE_CCD are set on the same body, then angular motions are handled by speculative
    /// contacts (eENABLE_SPECULATIVE_CCD) while linear motions are handled by sweeps (eENABLE_CCD).
    EnableSpeculativeCcd = 16,
    /// Register a rigid body for reporting pose changes by the simulation at an early stage.
    ///
    /// Sometimes it might be advantageous to get access to the new pose of a rigid body as early as possible and
    /// not wait until the call to fetchResults() returns. Setting this flag will schedule the rigid body to get reported
    /// in [`PxSimulationEventCallback::onAdvance`](). Please refer to the documentation of that callback to understand
    /// the behavior and limitations of this functionality.
    EnablePoseIntegrationPreview = 32,
    /// Permit CCD to limit maxContactImpulse. This is useful for use-cases like a destruction system but can cause visual artefacts so is not enabled by default.
    EnableCcdMaxContactImpulse = 64,
    /// Carries over forces/torques between frames, rather than clearing them
    ///
    /// If this flag is raised, forces and torques will carry over between frames. Impulses applied with PxForceMode::eIMPULSE will not be retained.
    ///
    /// Clearing this flag will retain the accelerations for an additional frame before clearing them. To reset the forces immediately for the next frame,
    /// a call to PxRigidBody::clearForce() / PxRigidBody::clearTorque() is needed.
    RetainAccelerations = 128,
    /// Forces kinematic-kinematic pairs notifications for this actor.
    ///
    /// This flag overrides the global scene-level PxPairFilteringMode setting for kinematic actors.
    /// This is equivalent to having PxPairFilteringMode::eKEEP for pairs involving this actor.
    ///
    /// A particular use case is when you have a large amount of kinematic actors, but you are only
    /// interested in interactions between a few of them. In this case it is best to use
    /// PxSceneDesc.kineKineFilteringMode = PxPairFilteringMode::eKILL, and then raise the
    /// eFORCE_KINE_KINE_NOTIFICATIONS flag on the small set of kinematic actors that need
    /// notifications.
    ///
    /// This has no effect if PxRigidBodyFlag::eKINEMATIC is not set.
    ///
    /// Changing this flag at runtime will not have an effect until you remove and re-add the actor to the scene.
    ForceKineKineNotifications = 256,
    /// Forces static-kinematic pairs notifications for this actor.
    ///
    /// Similar to eFORCE_KINE_KINE_NOTIFICATIONS, but for static-kinematic interactions.
    ///
    /// This has no effect if PxRigidBodyFlag::eKINEMATIC is not set.
    ///
    /// Changing this flag at runtime will not have an effect until you remove and re-add the actor to the scene.
    ForceStaticKineNotifications = 512,
    /// Enables computation of gyroscopic forces on the rigid body.
    EnableGyroscopicForces = 1024,
    /// Reserved for internal usage
    Reserved = 32768,
}

bitflags::bitflags! {
    /// Flags for [`PxRigidBodyFlag`]
    #[derive(Default)]
    #[repr(transparent)]
    pub struct PxRigidBodyFlags: u16 {
        const Kinematic = 1 << 0;
        const UseKinematicTargetForSceneQueries = 1 << 1;
        const EnableCcd = 1 << 2;
        const EnableCcdFriction = 1 << 3;
        const EnableSpeculativeCcd = 1 << 4;
        const EnablePoseIntegrationPreview = 1 << 5;
        const EnableCcdMaxContactImpulse = 1 << 6;
        const RetainAccelerations = 1 << 7;
        const ForceKineKineNotifications = 1 << 8;
        const ForceStaticKineNotifications = 1 << 9;
        const EnableGyroscopicForces = 1 << 10;
        const Reserved = 1 << 15;
    }
}

/// constraint flags
///
/// eBROKEN is a read only flag
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PxConstraintFlag {
    /// whether the constraint is broken
    Broken = 1,
    /// whether contacts should be generated between the objects this constraint constrains
    CollisionEnabled = 8,
    /// whether this constraint should be visualized, if constraint visualization is turned on
    Visualization = 16,
    /// Will be removed in a future version and the limits will always be forces. limits for drive strength are forces rather than impulses
    DriveLimitsAreForces = 32,
    /// perform preprocessing for improved accuracy on D6 Slerp Drive (this flag will be removed in a future release when preprocessing is no longer required)
    ImprovedSlerp = 128,
    /// suppress constraint preprocessing, intended for use with rowResponseThreshold. May result in worse solver accuracy for ill-conditioned constraints.
    DisablePreprocessing = 256,
    /// enables extended limit ranges for angular limits (e.g., limit values > PxPi or
    /// <
    /// -PxPi)
    EnableExtendedLimits = 512,
    /// please do not raise this flag as it is for internal use only
    GpuCompatible = 1024,
    /// updates the constraint each frame
    AlwaysUpdate = 2048,
    /// disables the constraint. SolverPrep functions won't be called for this constraint.
    DisableConstraint = 4096,
}

bitflags::bitflags! {
    /// Flags for [`PxConstraintFlag`]
    #[derive(Default)]
    #[repr(transparent)]
    pub struct PxConstraintFlags: u16 {
        const Broken = 1 << 0;
        const CollisionEnabled = 1 << 3;
        const Visualization = 1 << 4;
        const DriveLimitsAreForces = 1 << 5;
        const ImprovedSlerp = 1 << 7;
        const DisablePreprocessing = 1 << 8;
        const EnableExtendedLimits = 1 << 9;
        const GpuCompatible = 1 << 10;
        const AlwaysUpdate = 1 << 11;
        const DisableConstraint = 1 << 12;
    }
}

/// Flags which control the behavior of a material.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PxMaterialFlag {
    /// If this flag is set, friction computations are always skipped between shapes with this material and any other shape.
    DisableFriction = 1,
    /// Whether to use strong friction.
    /// The difference between "normal" and "strong" friction is that the strong friction feature
    /// remembers the "friction error" between simulation steps. The friction is a force trying to
    /// hold objects in place (or slow them down) and this is handled in the solver. But since the
    /// solver is only an approximation, the result of the friction calculation can include a small
    /// "error" - e.g. a box resting on a slope should not move at all if the static friction is in
    /// action, but could slowly glide down the slope because of a small friction error in each
    /// simulation step. The strong friction counter-acts this by remembering the small error and
    /// taking it to account during the next simulation step.
    ///
    /// However, in some cases the strong friction could cause problems, and this is why it is
    /// possible to disable the strong friction feature by setting this flag. One example is
    /// raycast vehicles that are sliding fast across the surface, but still need a precise
    /// steering behavior. It may be a good idea to reenable the strong friction when objects
    /// are coming to a rest, to prevent them from slowly creeping down inclines.
    ///
    /// Note: This flag only has an effect if the PxMaterialFlag::eDISABLE_FRICTION bit is 0.
    DisableStrongFriction = 2,
    /// If this flag is raised in combination with negative restitution, the computed spring-damper output will be interpreted as
    /// acceleration instead of force targets, analog to acceleration spring constraints.
    /// The flag has no effect for non-compliant contacts (i.e., if restitution is nonnegative).
    /// In an interaction between a compliant-force and a compliant-acceleration body the latter will dominate.
    CompliantAccelerationSpring = 16,
}

bitflags::bitflags! {
    /// Flags for [`PxMaterialFlag`]
    #[derive(Default)]
    #[repr(transparent)]
    pub struct PxMaterialFlags: u16 {
        const DisableFriction = 1 << 0;
        const DisableStrongFriction = 1 << 1;
        const CompliantAccelerationSpring = 1 << 4;
    }
}

/// Enumeration that determines the way in which two material properties will be combined to yield a friction or restitution coefficient for a collision.
///
/// When two actors come in contact with each other, they each have materials with various coefficients, but we only need a single set of coefficients for the pair.
///
/// Physics doesn't have any inherent combinations because the coefficients are determined empirically on a case by case
/// basis. However, simulating this with a pairwise lookup table is often impractical.
///
/// For this reason the following combine behaviors are available:
///
/// eAVERAGE
/// eMIN
/// eMULTIPLY
/// eMAX
///
/// The effective combine mode for the pair is maximum(material0.combineMode, material1.combineMode).
///
/// Notes that the restitution coefficient is overloaded if it is negative and represents a spring stiffness for compliant contacts. In the compliant contact case, the following rules apply:
/// If a compliant (restitution
/// <
/// 0) material interacts with a rigid (restitution >= 0) material, the compliant behavior will be chosen independent
/// of combine mode. In all other cases (i.e., also for compliant-compliant interactions) the combine mode is used.
/// For a compliant-compliant interaction with eMULTIPLY combine mode, we multiply the values but keep the sign negative.
/// The material damping follows the same logic, i.e., for the compliant vs non-compliant case, we take the damping value of the compliant material. Otherwise the combine mode is respected.
/// In an interaction between a compliant-force and a compliant-acceleration body the latter will dominate and exclusively determine the collision behavior with its parameters.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PxCombineMode {
    /// Average: (a + b)/2
    Average = 0,
    /// Minimum: minimum(a,b)
    Min = 1,
    /// Multiply: a*b
    Multiply = 2,
    /// Maximum: maximum(a,b)
    Max = 3,
    /// This is not a valid combine mode, it is a sentinel to denote the number of possible values. We assert that the variable's value is smaller than this.
    NValues = 4,
    /// This is not a valid combine mode, it is to assure that the size of the enum type is big enough.
    Pad32 = 2147483647,
}

/// Header for a contact patch where all points share same material and normal
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PxContactPatchFlags {
    /// Indicates this contact stream has face indices.
    HasFaceIndices = 1,
    /// Indicates this contact stream is modifiable.
    Modifiable = 2,
    /// Indicates this contact stream is notify-only (no contact response).
    ForceNoResponse = 4,
    /// Indicates this contact stream has modified mass ratios
    HasModifiedMassRatios = 8,
    /// Indicates this contact stream has target velocities set
    HasTargetVelocity = 16,
    /// Indicates this contact stream has max impulses set
    HasMaxImpulse = 32,
    /// Indicates this contact stream needs patches re-generated. This is required if the application modified either the contact normal or the material properties
    RegeneratePatches = 64,
    CompressedModifiedContact = 128,
}

/// A class to iterate over a compressed contact stream. This supports read-only access to the various contact formats.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum StreamFormat {
    SimpleStream = 0,
    ModifiableStream = 1,
    CompressedModifiableStream = 2,
}

/// Flags to enable or disable special modes of a PxDeformableBody instance
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PxDeformableBodyFlag {
    /// Determines if self collision will be detected and resolved
    DisableSelfCollision = 1,
    /// Enables support for speculative contact generation, see [`PxRigidBodyFlag::eENABLE_SPECULATIVE_CCD`]
    EnableSpeculativeCcd = 2,
    /// Enables support for kinematic motion of the simulation mesh, see [`PxRigidBodyFlag::eKINEMATIC`]
    Kinematic = 4,
}

bitflags::bitflags! {
    /// Flags for [`PxDeformableBodyFlag`]
    #[derive(Default)]
    #[repr(transparent)]
    pub struct PxDeformableBodyFlags: u8 {
        const DisableSelfCollision = 1 << 0;
        const EnableSpeculativeCcd = 1 << 1;
        const Kinematic = 1 << 2;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PxDeformableSurfaceFlag {
    UseAnisotropicModel = 1,
    EnableFlattening = 2,
}

bitflags::bitflags! {
    /// Flags for [`PxDeformableSurfaceFlag`]
    #[derive(Default)]
    #[repr(transparent)]
    pub struct PxDeformableSurfaceFlags: u16 {
        const UseAnisotropicModel = 1 << 0;
        const EnableFlattening = 1 << 1;
    }
}

/// Identifies input and output buffers for PxDeformableSurface.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PxDeformableSurfaceDataFlag {
    None = 0,
    PositionInvmass = 1,
    Velocity = 2,
    RestPosition = 4,
    All = 7,
}

bitflags::bitflags! {
    /// Flags for [`PxDeformableSurfaceDataFlag`]
    #[derive(Default)]
    #[repr(transparent)]
    pub struct PxDeformableSurfaceDataFlags: u32 {
        const PositionInvmass = 1 << 0;
        const Velocity = 1 << 1;
        const RestPosition = 1 << 2;
        const All = Self::PositionInvmass.bits | Self::Velocity.bits | Self::RestPosition.bits;
    }
}

/// Flags to enable or disable special modes of a PxDeformableVolume instance
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PxDeformableVolumeFlag {
    /// Enables computation of a Cauchy stress tensor for every tetrahedron in the simulation mesh. The tensors can be accessed through the deformable volume direct API
    ComputeStressTensor = 1,
    /// Enables partially kinematic motion of the collision and simulation mesh.
    PartiallyKinematic = 2,
}

bitflags::bitflags! {
    /// Flags for [`PxDeformableVolumeFlag`]
    #[derive(Default)]
    #[repr(transparent)]
    pub struct PxDeformableVolumeFlags: u16 {
        const ComputeStressTensor = 1 << 0;
        const PartiallyKinematic = 1 << 1;
    }
}

/// Identifies the buffers of a PxDeformableVolume instance.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PxDeformableVolumeDataFlag {
    None = 0,
    /// The collision mesh's positions
    PositionInvmass = 1,
    /// The simulation mesh's positions and inverse masses
    SimPositionInvmass = 2,
    /// The simulation mesh's velocities
    SimVelocity = 4,
    /// The collision mesh's rest positions
    RestPositionInvmass = 8,
    All = 15,
}

bitflags::bitflags! {
    /// Flags for [`PxDeformableVolumeDataFlag`]
    #[derive(Default)]
    #[repr(transparent)]
    pub struct PxDeformableVolumeDataFlags: u32 {
        const PositionInvmass = 1 << 0;
        const SimPositionInvmass = 1 << 1;
        const SimVelocity = 1 << 2;
        const RestPositionInvmass = 1 << 3;
        const All = Self::PositionInvmass.bits | Self::SimPositionInvmass.bits | Self::SimVelocity.bits | Self::RestPositionInvmass.bits;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PxDeformableVolumeMaterialModel {
    /// Default model. Well suited for high stiffness. Does need tetrahedra with good shapes (no extreme slivers) in the rest pose.
    CoRotational = 0,
    /// Well suited for lower stiffness. Robust to any tetrahedron shape.
    NeoHookean = 1,
}

/// Flags specifying deletion event types.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PxDeletionEventFlag {
    /// The user has called release on an object.
    UserRelease = 1,
    /// The destructor of an object has been called and the memory has been released.
    MemoryRelease = 2,
}

bitflags::bitflags! {
    /// Flags for [`PxDeletionEventFlag`]
    #[derive(Default)]
    #[repr(transparent)]
    pub struct PxDeletionEventFlags: u8 {
        const UserRelease = 1 << 0;
        const MemoryRelease = 1 << 1;
    }
}

/// Identifies the attachment target type for an actor involved in an attachment.
///
/// The target type provides actor related information about what kind of attachment should be created.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PxDeformableAttachmentTargetType {
    /// Attachment to vertex points of deformable mesh.
    Vertex = 0,
    /// Attachment to points on triangles of deformable mesh.
    Triangle = 1,
    /// Attachment to points in tetrahedrons of deformable mesh.
    Tetrahedron = 2,
    /// Attachment to points in rigid actor local frame.
    Rigid = 3,
    /// Attachment to points in global frame.
    World = 4,
    /// Internal use only.
    Undefined = 5,
}

/// Filtering flags for scene queries.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PxQueryFlag {
    /// Traverse static shapes
    Static = 1,
    /// Traverse dynamic shapes
    Dynamic = 2,
    /// Run the pre-intersection-test filter (see [`PxQueryFilterCallback::preFilter`]())
    Prefilter = 4,
    /// Run the post-intersection-test filter (see [`PxQueryFilterCallback::postFilter`]())
    Postfilter = 8,
    /// Abort traversal as soon as any hit is found and return it via callback.block.
    /// Helps query performance. Both eTOUCH and eBLOCK hitTypes are considered hits with this flag.
    AnyHit = 16,
    /// All hits are reported as touching. Overrides eBLOCK returned from user filters with eTOUCH.
    /// This is also an optimization hint that may improve query performance.
    NoBlock = 32,
    /// Run with legacy batch query filter behavior. Raising this flag ensures that
    /// the hardcoded filter equation is neglected. This guarantees that any provided PxQueryFilterCallback
    /// will be utilised, as specified by the ePREFILTER  and ePOSTFILTER flags.
    BatchQueryLegacyBehaviour = 64,
    /// Same as eBATCH_QUERY_LEGACY_BEHAVIOUR, more explicit name making it clearer that this can also be used
    /// with regular/non-batched queries if needed.
    DisableHardcodedFilter = 64,
    /// Reserved for internal use
    Reserved = 32768,
}

bitflags::bitflags! {
    /// Flags for [`PxQueryFlag`]
    #[derive(Default)]
    #[repr(transparent)]
    pub struct PxQueryFlags: u16 {
        const Static = 1 << 0;
        const Dynamic = 1 << 1;
        const Prefilter = 1 << 2;
        const Postfilter = 1 << 3;
        const AnyHit = 1 << 4;
        const NoBlock = 1 << 5;
        const BatchQueryLegacyBehaviour = 1 << 6;
        const DisableHardcodedFilter = 1 << 6;
        const Reserved = 1 << 15;
    }
}

/// Classification of scene query hits (intersections).
///
/// - eNONE: Returning this hit type means that the hit should not be reported.
/// - eBLOCK: For all raycast, sweep and overlap queries the nearest eBLOCK type hit will always be returned in PxHitCallback::block member.
/// - eTOUCH: Whenever a raycast, sweep or overlap query was called with non-zero PxHitCallback::nbTouches and PxHitCallback::touches
/// parameters, eTOUCH type hits that are closer or same distance (touchDistance
/// <
/// = blockDistance condition)
/// as the globally nearest eBLOCK type hit, will be reported.
/// - For example, to record all hits from a raycast query, always return eTOUCH.
///
/// All hits in overlap() queries are treated as if the intersection distance were zero.
/// This means the hits are unsorted and all eTOUCH hits are recorded by the callback even if an eBLOCK overlap hit was encountered.
/// Even though all overlap() blocking hits have zero length, only one (arbitrary) eBLOCK overlap hit is recorded in PxHitCallback::block.
/// All overlap() eTOUCH type hits are reported (zero touchDistance
/// <
/// = zero blockDistance condition).
///
/// For raycast/sweep/overlap calls with zero touch buffer or PxHitCallback::nbTouches member,
/// only the closest hit of type eBLOCK is returned. All eTOUCH hits are discarded.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PxQueryHitType {
    /// the query should ignore this shape
    None = 0,
    /// a hit on the shape touches the intersection geometry of the query but does not block it
    Touch = 1,
    /// a hit on the shape blocks the query (does not block overlap queries)
    Block = 2,
}

/// Collection of flags providing a mechanism to lock motion along/around a specific axis.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PxRigidDynamicLockFlag {
    LockLinearX = 1,
    LockLinearY = 2,
    LockLinearZ = 4,
    LockAngularX = 8,
    LockAngularY = 16,
    LockAngularZ = 32,
}

bitflags::bitflags! {
    /// Flags for [`PxRigidDynamicLockFlag`]
    #[derive(Default)]
    #[repr(transparent)]
    pub struct PxRigidDynamicLockFlags: u8 {
        const LockLinearX = 1 << 0;
        const LockLinearY = 1 << 1;
        const LockLinearZ = 1 << 2;
        const LockAngularX = 1 << 3;
        const LockAngularY = 1 << 4;
        const LockAngularZ = 1 << 5;
    }
}

/// This flag specifies the type of data to get when calling PxDirectGPUAPI::getRigidDynamicData().
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PxRigidDynamicGPUAPIReadType {
    /// Get the global poses. Type: 1 PxTransform per PxRigidDynamic.
    GlobalPose = 0,
    /// Get the linear velocities. Type: 1 PxVec3 per PxRigidDynamic.
    LinearVelocity = 1,
    /// Get the angular velocities. Type: 1 PxVec3 per PxRigidDynamic.
    AngularVelocity = 2,
    /// Get the linear accelerations. Type: 1 PxVec3 per PxRigidDynamic.
    LinearAcceleration = 3,
    /// Get the angular accelerations. Type: 1 PxVec3 per PxRigidDynamic.
    AngularAcceleration = 4,
}

/// This flag specifies the type of data to set when calling PxDirectGPUAPI::setRigidDynamicData().
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PxRigidDynamicGPUAPIWriteType {
    /// Set the global poses. Type: 1 PxTransform per PxRigidDynamic.
    GlobalPose = 0,
    /// Set the linear velocities. Type: 1 PxVec3 per PxRigidDynamic.
    LinearVelocity = 1,
    /// Set the angular velocities. Type: 1 PxVec3 per PxRigidDynamic.
    AngularVelocity = 2,
    /// Set the forces. Will be applied at the center of gravity of the bodies. Type: 1 PxVec3 per PxRigidDynamic.
    Force = 3,
    /// Set the torques. Will be applied at the center of gravity of the bodies. Type: 1 PxVec3 per PxRigidDynamic.
    Torque = 4,
}

/// This flag specifies the type of data to get when calling PxDirectGPUAPI::getArticulationData().
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PxArticulationGPUAPIReadType {
    /// The joint positions. 1 PxReal per dof. Block size per articulation: maxDofs.
    JointPosition = 0,
    /// The joint velocities. 1 PxReal per dof. Block size per articulation: maxDofs.
    JointVelocity = 1,
    /// The joint accelerations. 1 PxReal per dof. Block size per articulation: maxDofs.
    JointAcceleration = 2,
    /// The joint forces or torques applied using setArticulationData. 1 PxReal per dof. Block size per articulation: maxDofs. Not updated by the simulation, will return the values set by PxDirectGPUAPI::setArticulationData().
    JointForce = 3,
    /// The velocity targets applied using setArticulationData. 1 PxReal per dof. Block size per articulation: maxDofs. Not updated by the simulation, will return the values set by PxDirectGPUAPI::setArticulationData().
    JointTargetVelocity = 4,
    /// The position targets applied using setArticulationData. 1 PxReal per dof. Block size per articulation: maxDofs. Not updated by the simulation, will return the values set by PxDirectGPUAPI::setArticulationData().
    JointTargetPosition = 5,
    /// The root link global pose. 1 PxTransform per articulation. Block size per articulation: 1.
    RootGlobalPose = 6,
    /// The root link linear velocity. 1 PxVec3 per articulation. Block size per articulation: 1.
    RootLinearVelocity = 7,
    /// The root link angular velocity. 1 PxVec3 per articulation. Block size per articulation: 1.
    RootAngularVelocity = 8,
    /// The link global pose including root link. 1 PxTransform per link. Block size per articulation: maxLinks.
    LinkGlobalPose = 9,
    /// The link linear velocities including root link. 1 PxVec3 per link. Block size per articulation: maxLinks.
    LinkLinearVelocity = 10,
    /// The link angular velocities including root link. 1 PxVec3 per link. Block size per articulation: maxLinks.
    LinkAngularVelocity = 11,
    /// The link linear accelerations including root link. 1 PxVec3 per link. Block size per articulation: maxLinks.
    LinkLinearAcceleration = 12,
    /// The link angular accelerations including root link. 1 PxVec3 per link. Block size per articulation: maxLinks.
    LinkAngularAcceleration = 13,
    /// The link incoming joint forces including root link. The force is reported in the child joint frame of the link's incoming joint. 2 PxVec3 per link. The first PxVec3 contains the force, and the second PxVec3 contains the torque. Block size per articulation: maxLinks.
    LinkIncomingJointForce = 14,
    /// Fixed tendon data. 1 PxGpuFixedTendonData per fixed tendon. Block size per articulation: maxFixedTendons. Not updated by the simulation, will return the values set by PxDirectGPUAPI::setArticulationData().
    FixedTendon = 15,
    /// Fixed tendon joint data. 1 PxGpuTendonJointCoefficientData per fixed tendon joint. Block size per articulation: maxFixedTendons * maxFixedTendonJoints. Not updated by the simulation, will return the values set by PxDirectGPUAPI::setArticulationData().
    FixedTendonJoint = 16,
    /// Spatial tendon data. 1 PxGpuSpatialTendonData per spatial tendon. Block size per articulation: maxSpatialTendons. Not updated by the simulation, will return the values set by PxDirectGPUAPI::setArticulationData().
    SpatialTendon = 17,
    /// Spatial tendon attachment data. 1 PxGpuTendonAttachmentData per spatial tendon attachment. Block size per articulation: maxSpatialTendons * maxSpatialTendonAttachments. Not updated by the simulation, will return the values set by PxDirectGPUAPI::setArticulationData().
    SpatialTendonAttachment = 18,
}

/// This flag specifies the type of data to set when calling PxDirectGPUAPI::setArticulationData().
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PxArticulationGPUAPIWriteType {
    /// The joint positions. 1 PxReal per dof. Block size per articulation: maxDofs.
    JointPosition = 0,
    /// The joint velocities. 1 PxReal per dof. Block size per articulation: maxDofs.
    JointVelocity = 1,
    /// The applied joint forces or torques. 1 PxReal per dof. Block size per articulation: maxDofs.
    JointForce = 2,
    /// The velocity targets for the joint drives. 1 PxReal per dof. Block size per articulation: maxDofs.
    JointTargetVelocity = 3,
    /// The position targets for the joint drives. 1 PxReal per dof. Block size per articulation: maxDofs.
    JointTargetPosition = 4,
    /// The root link transform. 1 PxTransform per articulation. Block size per articulation: 1.
    RootGlobalPose = 5,
    /// The root link linear velocity. 1 PxVec3 per articulation. Block size per articulation: 1.
    RootLinearVelocity = 6,
    /// The root link angular velocity. 1 PxVec3 per articulation. Block size per articulation: 1.
    RootAngularVelocity = 7,
    /// The forces to apply to links. 1 PxVec3 per link. Block size per articulation: maxLinks.
    LinkForce = 8,
    /// The torques to apply to links. 1 PxVec3 per link. Block size per articulation: maxLinks.
    LinkTorque = 9,
    /// Fixed tendon data. 1 PxGpuFixedTendonData per fixed tendon. Block size per articulation: maxFixedTendons.
    FixedTendon = 10,
    /// Fixed tendon joint data. 1 PxGpuTendonJointCoefficientData per fixed tendon joint. Block size per articulation: maxFixedTendons * maxFixedTendonJoints.
    FixedTendonJoint = 11,
    /// Spatial tendon data. 1 PxGpuSpatialTendonData per spatial tendon. Block size per articulation: maxSpatialTendons.
    SpatialTendon = 12,
    /// Spatial tendon attachment data. 1 PxGpuTendonAttachmentData per spatial tendon attachment. Block size per articulation: maxSpatialTendons * maxSpatialTendonAttachments.
    SpatialTendonAttachment = 13,
}

/// This flag specifies the type of operation to perform when calling PxDirectGPUAPI::computeArticulationData.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PxArticulationGPUAPIComputeType {
    /// Updates the link state for all the articulations specified in the index list. This operation can be performed
    /// by the user to propagate changes made to root transform/root velocities/joint positions/joint velocities to
    /// be reflected in the link transforms/velocities. Performing this operation will clear output values calculated by
    /// the simulation, specifically link accelerations, link incoming joint forces, and joint accelerations. Note
    /// that this is only necessary if the user wants to query link state, otherwise it will be performed automatically
    /// at the start of the next call to simulate(). The data input parameter will be ignored and can be set to NULL for
    /// this operation.
    UpdateKinematic = 0,
    /// Computes the dense Jacobian for the articulation in world space, including the dofs of a potentially floating base.
    /// This is the batched, direct-GPU equivalent of PxArticulationReducedCoordinate::computeDenseJacobian. The output data
    /// buffer is laid out into sequential blocks per articulation, where each block has the size
    /// (6 + maxDofs) * (6 + (maxLinks - 1) * 6) * sizeof(float). maxLinks and maxDofs are the maximum link and dof counts
    /// across all the articulations in the scene, and can be queried by calling PxDirectGPUAPI::getArticulationGPUAPIMaxCounts().
    /// The size of the jacobian can vary by articulation, and will be determined using these formulas:
    /// nCols = (fixedBase ? 0 : 6) + dofCount, nRows = (fixedBase ? 0 : 6) + (linkCount - 1) * 6. The matrix is indexed [nCols * row + column].
    DenseJacobians = 1,
    /// Computes the mass matrices that maps accelerations to forces: forces = M * accelerations on the GPU.
    /// This is the batched, direct-GPU equivalent of PxArticulationReducedCoordinate::computeMassMatrix(). The output buffer is laid
    /// out into sequential blocks per articulation, where each block has the size (maxDofs + 6) * (maxDofs + 6) * sizeof(float).
    /// maxDofs is the maximum dof count across all the articulations in the scene, and can be queried by calling
    /// PxDirectGPUAPI::getArticulationGPUAPIMaxCounts(), The size of the matrix can vary by articulation, and will be dofCount * dofCount
    /// for fixed-base articulations and (dofCount + 6) * (dofCount + 6) for floating-base articulations.
    /// We refer to the documentation of PxArticulationCache and PxArticulationReducedCoordinate::computeMassMatrix() for a more detailed explanation.
    MassMatrices = 2,
    /// Computes the joint dof forces (and root force) required to counteract Coriolis and centrifugal forces for the given articulation pose.
    /// This is the batched, direct-GPU equivalent to PxArticulationReducedCoordinate::computeCoriolisCompensation(). The output data
    /// buffer is laid out into sequential blocks per articulation, where each block has the size (maxDofs + 6) * sizeof(float). maxDofs
    /// is the maximum dof count across all the articulations in the scene, and can be queried by calling
    /// PxDirectGPUAPI::getArticulationGPUAPIMaxCounts(). The size of the output can vary by articulation, and will be dofCount
    /// for fixed-base articulations and (dofCount + 6) for floating-base articulations. We refer to the documentation of
    /// PxArticulationCache and PxArticulationReducedCoordinate::computeCoriolisCompensation() for a more detailed explanation.
    CoriolisAndCentrifugalCompensation = 3,
    /// Computes the forces required to counteract gravitational forces for the given articulation pose. This is the batched,
    /// direct-GPU equivalent of PxArticulationReducedCoordinate::computeGravityCompensation(). The output data buffer is laid out
    /// into sequential blocks per articulation, where each block has the size (maxDofs + 6) * sizeof(float). maxDofs
    /// is the maximum dof count across all the articulations in the scene, and can be queried by calling
    /// PxDirectGPUAPI::getArticulationGPUAPIMaxCounts(). The size of the output can vary by articulation, and will be dofCount
    /// for fixed-base articulations and (dofCount + 6) for floating-base articulations. We refer to the documentation of
    /// PxArticulationCache and PxArticulationReducedCoordinate::computeGravityCompensation() for a more detailed explanation.
    GravityCompensation = 4,
    /// Computes the articulation's center of mass in the world frame for the given articulation pose.
    /// This is the batched, direct-GPU equivalent to PxArticulationReducedCoordinate::computeArticulationCOM(). The output data
    /// buffer is laid out into sequential blocks per articulation, where each block has the size sizeof(float) * 3.
    ArticulationComsWorldFrame = 5,
    /// Computes the articulation's center of mass in the root frame for the given articulation pose.
    /// This is the batched, direct-GPU equivalent to PxArticulationReducedCoordinate::computeArticulationCOM(). The output data
    /// buffer is laid out into sequential blocks per articulation, where each block has the size sizeof(float) * 3.
    ArticulationComsRootFrame = 6,
    /// Computes the centroidal momentum matrix and bias force for a floating-base articulation.
    /// This is the batched, direct-GPU equivalent to PxArticulationReducedCoordinate::computeCentroidalMomentumMatrix(). The data buffer is laid
    /// out into four main blocks. The two first blocks correspond to the input (mass matrix, Coriolis and Centrifugal compensation force),
    /// and the two last blocks correspond to the output (centroidal momentum matrix, bias force). Each block must be organized into sequential
    /// subblocks per articulation. The size of the subblock is (maxDofs + 6) * (maxDofs + 6) * sizeof(float) for the mass matrix,
    /// (maxDofs + 6) * sizeof(float) for the Coriolis and Centrifugal compensation force, 6 * (maxDofs + 6) * sizeof(float) for the centroidal
    /// momentum matrix, and 6 * sizeof(float) for the bias force. maxDofs is the maximum dof count across all the articulations in the scene,
    /// and can be queried by calling PxDirectGPUAPI::getArticulationGPUAPIMaxCounts(). The size of the actual data in each subblock can vary by
    /// articulation, and will depend on the value of dofCount. The dof indices will be according to the low-level indexing, we refer to
    /// the documentation of PxArticulationCache for an explanation.
    CentroidalMomentumMatrices = 7,
}

/// This flag specifies the type of data to get when calling [`PxDirectGPUAPI::getD6JointData`]().
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PxD6JointGPUAPIReadType {
    /// The joint forces applied by the solver.
    ///
    /// The forces are in world space. 1 PxVec3 per joint.
    ///
    /// Replaces calls to PxConstraint::getForce() which will not work properly anymore if direct GPU API is used.
    JointForce = 0,
    /// The joint torques applied by the solver.
    ///
    /// The torques are in world space. 1 PxVec3 per joint.
    ///
    /// Replaces calls to PxConstraint::getForce() which will not work properly anymore if direct GPU API is used.
    JointTorque = 1,
}

/// Pruning structure used to accelerate scene queries.
///
/// eNONE uses a simple data structure that consumes less memory than the alternatives,
/// but generally has slower query performance.
///
/// eDYNAMIC_AABB_TREE usually provides the fastest queries. However there is a
/// constant per-frame management cost associated with this structure. How much work should
/// be done per frame can be tuned via the [`PxSceneQueryDesc::dynamicTreeRebuildRateHint`]
/// parameter.
///
/// eSTATIC_AABB_TREE is typically used for static objects. It is the same as the
/// dynamic AABB tree, without the per-frame overhead. This can be a good choice for static
/// objects, if no static objects are added, moved or removed after the scene has been
/// created. If there is no such guarantee (e.g. when streaming parts of the world in and out),
/// then the dynamic version is a better choice even for static objects.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PxPruningStructureType {
    /// Using a simple data structure
    None = 0,
    /// Using a dynamic AABB tree
    DynamicAabbTree = 1,
    /// Using a static AABB tree
    StaticAabbTree = 2,
    Last = 3,
}

/// Secondary pruning structure used for newly added objects in dynamic trees.
///
/// Dynamic trees (PxPruningStructureType::eDYNAMIC_AABB_TREE) are slowly rebuilt
/// over several frames. A secondary pruning structure holds and manages objects
/// added to the scene while this rebuild is in progress.
///
/// eNONE ignores newly added objects. This means that for a number of frames (roughly
/// defined by PxSceneQueryDesc::dynamicTreeRebuildRateHint) newly added objects will
/// be ignored by scene queries. This can be acceptable when streaming large worlds, e.g.
/// when the objects added at the boundaries of the game world don't immediately need to be
/// visible from scene queries (it would be equivalent to streaming that data in a few frames
/// later). The advantage of this approach is that there is no CPU cost associated with
/// inserting the new objects in the scene query data structures, and no extra runtime cost
/// when performing queries.
///
/// eBUCKET uses a structure similar to PxPruningStructureType::eNONE. Insertion is fast but
/// query cost can be high.
///
/// eINCREMENTAL uses an incremental AABB-tree, with no direct PxPruningStructureType equivalent.
/// Query time is fast but insertion cost can be high.
///
/// eBVH uses a PxBVH structure. This usually offers the best overall performance.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PxDynamicTreeSecondaryPruner {
    /// no secondary pruner, new objects aren't visible to SQ for a few frames
    None = 0,
    /// bucket-based secondary pruner, faster updates, slower query time
    Bucket = 1,
    /// incremental-BVH secondary pruner, faster query time, slower updates
    Incremental = 2,
    /// PxBVH-based secondary pruner, good overall performance
    Bvh = 3,
    Last = 4,
}

/// Scene query update mode
///
/// This enum controls what work is done when the scene query system is updated. The updates traditionally happen when PxScene::fetchResults
/// is called. This function then calls PxSceneQuerySystem::finalizeUpdates, where the update mode is used.
///
/// fetchResults/finalizeUpdates will sync changed bounds during simulation and update the scene query bounds in pruners, this work is mandatory.
///
/// eBUILD_ENABLED_COMMIT_ENABLED does allow to execute the new AABB tree build step during fetchResults/finalizeUpdates, additionally
/// the pruner commit is called where any changes are applied. During commit PhysX refits the dynamic scene query tree and if a new tree
/// was built and the build finished the tree is swapped with current AABB tree.
///
/// eBUILD_ENABLED_COMMIT_DISABLED does allow to execute the new AABB tree build step during fetchResults/finalizeUpdates. Pruner commit
/// is not called, this means that refit will then occur during the first scene query following fetchResults/finalizeUpdates, or may be forced
/// by the method PxScene::flushQueryUpdates() / PxSceneQuerySystemBase::flushUpdates().
///
/// eBUILD_DISABLED_COMMIT_DISABLED no further scene query work is executed. The scene queries update needs to be called manually, see
/// PxScene::sceneQueriesUpdate (see that function's doc for the equivalent PxSceneQuerySystem sequence). It is recommended to call
/// PxScene::sceneQueriesUpdate right after fetchResults/finalizeUpdates as the pruning structures are not updated.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PxSceneQueryUpdateMode {
    /// Both scene query build and commit are executed.
    BuildEnabledCommitEnabled = 0,
    /// Scene query build only is executed.
    BuildEnabledCommitDisabled = 1,
    /// No work is done, no update of scene queries
    BuildDisabledCommitDisabled = 2,
}

/// Built-in enum for default PxScene pruners
///
/// This is passed as a pruner index to various functions in the following APIs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum PxScenePrunerIndex {
    PxScenePrunerStatic = 0,
    PxScenePrunerDynamic = 1,
    PxSceneCompoundPruner = 4294967295,
}

/// Broad phase algorithm used in the simulation
///
/// eSAP is a good generic choice with great performance when many objects are sleeping. Performance
/// can degrade significantly though, when all objects are moving, or when large numbers of objects
/// are added to or removed from the broad phase. This algorithm does not need world bounds to be
/// defined in order to work.
///
/// eMBP is an alternative broad phase algorithm that does not suffer from the same performance
/// issues as eSAP when all objects are moving or when inserting large numbers of objects. However
/// its generic performance when many objects are sleeping might be inferior to eSAP, and it requires
/// users to define world bounds in order to work.
///
/// eABP is a revisited implementation of MBP, which automatically manages broad-phase regions.
/// It offers the convenience of eSAP (no need to define world bounds or regions) and the performance
/// of eMBP when a lot of objects are moving. While eSAP can remain faster when most objects are
/// sleeping and eMBP can remain faster when it uses a large number of properly-defined regions,
/// eABP often gives the best performance on average and the best memory usage.
///
/// ePABP is a parallel implementation of ABP. It can often be the fastest (CPU) broadphase, but it
/// can use more memory than ABP.
///
/// eGPU is a GPU implementation of the incremental sweep and prune approach. Additionally, it uses a ABP-style
/// initial pair generation approach to avoid large spikes when inserting shapes. It not only has the advantage
/// of traditional SAP approch which is good for when many objects are sleeping, but due to being fully parallel,
/// it also is great when lots of shapes are moving or for runtime pair insertion and removal. It can become a
/// performance bottleneck if there are a very large number of shapes roughly projecting to the same values
/// on a given axis. If the scene has a very large number of shapes in an actor, e.g. a humanoid, it is recommended
/// to use an aggregate to represent multi-shape or multi-body actors to minimize stress placed on the broad phase.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PxBroadPhaseType {
    /// 3-axes sweep-and-prune
    Sap = 0,
    /// Multi box pruning
    Mbp = 1,
    /// Automatic box pruning
    Abp = 2,
    /// Parallel automatic box pruning
    Pabp = 3,
    /// GPU broad phase
    Gpu = 4,
    Last = 5,
}

/// Enum for selecting the friction algorithm used for simulation.
///
/// Since only the patch friction model is supported now, the friction type option is obsolete.
///
/// [`PxFrictionType::ePATCH`] is the default friction logic (Couloumb type friction model). Friction gets computed per contact patch.
/// Up to two contact points lying in the contact patch area are selected as friction anchors to which friction impulses are applied. If there
/// are more than two contact points, to select anchors from, the anchors are selected using a heuristic that tries to maximize the distance
/// between the anchors within the contact patch area. For each contact patch, two perpendicular axes of the contact patch plane are selected.
/// A 1D-constraint along each of the two axes is used to implement friction at a friction anchor point. Note that the two axes are processed
/// separately when the PGS solver type is selected. This can lead to asymmetries when transitioning from dynamic to static friction and vice
/// versa in certain edge cases. The TGS solver type, on the other hand, works with the combined impulse along the two axes and as such avoids
/// this potential problem, but this is slightly more computationally expensive. Another difference between TGS and PGS is that TGS applies
/// friction throughout all position and all velocity iterations, while PGS by default applies friction throughout the last 3 position iterations
/// and all velocity iterations (unless [`PxSceneFlag::eENABLE_FRICTION_EVERY_ITERATION`] is used).
///
/// [`PxFrictionType::eFRICTION_COUNT`] is the total number of friction models supported by the SDK.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PxFrictionType {
    /// Select default patch-friction model.
    Patch = 0,
    /// The total number of friction models supported by the SDK.
    FrictionCount = 1,
}

/// Enum for selecting the type of solver used for the simulation.
///
/// [`PxSolverType::ePGS`] selects the iterative sequential impulse solver. This is the same kind of solver used in PhysX 3.4 and earlier releases.
///
/// [`PxSolverType::eTGS`] selects a non linear iterative solver. This kind of solver can lead to improved convergence and handle large mass ratios, long chains and jointed systems better. It is slightly more expensive than the default solver and can introduce more energy to correct joint and contact errors.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PxSolverType {
    /// Projected Gauss-Seidel iterative solver
    Pgs = 0,
    /// Temporal Gauss-Seidel solver
    Tgs = 1,
}

/// flags for configuring properties of the scene
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PxSceneFlag {
    /// Enable Active Actors Notification.
    ///
    /// This flag enables the Active Actor Notification feature for a scene.  This
    /// feature defaults to disabled.  When disabled, the function
    /// PxScene::getActiveActors() will always return a NULL list.
    ///
    /// There may be a performance penalty for enabling the Active Actor Notification, hence this flag should
    /// only be enabled if the application intends to use the feature.
    ///
    /// Default:
    /// False
    EnableActiveActors = 1,
    /// Enables a second broad phase check after integration that makes it possible to prevent objects from tunneling through eachother.
    ///
    /// PxPairFlag::eDETECT_CCD_CONTACT requires this flag to be specified.
    ///
    /// For this feature to be effective for bodies that can move at a significant velocity, the user should raise the flag PxRigidBodyFlag::eENABLE_CCD for them.
    ///
    /// This flag is not mutable, and must be set in PxSceneDesc at scene creation.
    ///
    /// Default:
    /// False
    EnableCcd = 2,
    /// Enables a simplified swept integration strategy, which sacrifices some accuracy for improved performance.
    ///
    /// This simplified swept integration approach makes certain assumptions about the motion of objects that are not made when using a full swept integration.
    /// These assumptions usually hold but there are cases where they could result in incorrect behavior between a set of fast-moving rigid bodies. A key issue is that
    /// fast-moving dynamic objects may tunnel through each-other after a rebound. This will not happen if this mode is disabled. However, this approach will be potentially
    /// faster than a full swept integration because it will perform significantly fewer sweeps in non-trivial scenes involving many fast-moving objects. This approach
    /// should successfully resist objects passing through the static environment.
    ///
    /// PxPairFlag::eDETECT_CCD_CONTACT requires this flag to be specified.
    ///
    /// This scene flag requires eENABLE_CCD to be enabled as well. If it is not, this scene flag will do nothing.
    ///
    /// For this feature to be effective for bodies that can move at a significant velocity, the user should raise the flag PxRigidBodyFlag::eENABLE_CCD for them.
    ///
    /// This flag is not mutable, and must be set in PxSceneDesc at scene creation.
    ///
    /// Default:
    /// False
    DisableCcdResweep = 4,
    /// Enable GJK-based distance collision detection system.
    ///
    /// This flag is not mutable, and must be set in PxSceneDesc at scene creation.
    ///
    /// Default:
    /// true
    EnablePcm = 64,
    /// Disable contact report buffer resize. Once the contact buffer is full, the rest of the contact reports will
    /// not be buffered and sent.
    ///
    /// This flag is not mutable, and must be set in PxSceneDesc at scene creation.
    ///
    /// Default:
    /// false
    DisableContactReportBufferResize = 128,
    /// Disable contact cache.
    ///
    /// Contact caches are used internally to provide faster contact generation. You can disable all contact caches
    /// if memory usage for this feature becomes too high.
    ///
    /// This flag is not mutable, and must be set in PxSceneDesc at scene creation.
    ///
    /// Default:
    /// false
    DisableContactCache = 256,
    /// Require scene-level locking
    ///
    /// When set to true this requires that threads accessing the PxScene use the
    /// multi-threaded lock methods.
    ///
    /// This flag is not mutable, and must be set in PxSceneDesc at scene creation.
    ///
    /// Default:
    /// false
    RequireRwLock = 512,
    /// Enables additional stabilization pass in solver
    ///
    /// When set to true, this enables additional stabilization processing to improve that stability of complex interactions between large numbers of bodies.
    ///
    /// Note that this flag is not mutable and must be set in PxSceneDesc at scene creation. Also, this is an experimental feature which does result in some loss of momentum.
    EnableStabilization = 1024,
    /// Enables average points in contact manifolds
    ///
    /// When set to true, this enables additional contacts to be generated per manifold to represent the average point in a manifold. This can stabilize stacking when only a small
    /// number of solver iterations is used.
    ///
    /// Note that this flag is not mutable and must be set in PxSceneDesc at scene creation.
    EnableAveragePoint = 2048,
    /// Do not report kinematics in list of active actors.
    ///
    /// Since the target pose for kinematics is set by the user, an application can track the activity state directly and use
    /// this flag to avoid that kinematics get added to the list of active actors.
    ///
    /// This flag has only an effect in combination with eENABLE_ACTIVE_ACTORS.
    ///
    /// Default:
    /// false
    ExcludeKinematicsFromActiveActors = 4096,
    /// Do not report kinematics in list of active actors.
    ///
    /// Since the target pose for kinematics is set by the user, an application can track the activity state directly and use
    /// this flag to avoid that kinematics get added to the list of active actors.
    ///
    /// This flag has only an effect in combination with eENABLE_ACTIVE_ACTORS.
    ///
    /// Default:
    /// false
    EnableGpuDynamics = 8192,
    /// Provides improved determinism at the expense of performance.
    ///
    /// By default, PhysX provides limited determinism guarantees. Specifically, PhysX guarantees that the exact scene (same actors created in the same order) and simulated using the same
    /// time-stepping scheme should provide the exact same behaviour.
    ///
    /// However, if additional actors are added to the simulation, this can affect the behaviour of the existing actors in the simulation, even if the set of new actors do not interact with
    /// the existing actors.
    ///
    /// This flag provides an additional level of determinism that guarantees that the simulation will not change if additional actors are added to the simulation, provided those actors do not interfere
    /// with the existing actors in the scene. Determinism is only guaranteed if the actors are inserted in a consistent order each run in a newly-created scene and simulated using a consistent time-stepping
    /// scheme.
    ///
    /// Note that this flag is not mutable and must be set at scene creation.
    ///
    /// Note that enabling this flag can have a negative impact on performance.
    ///
    /// Note that this feature is not currently supported on GPU.
    ///
    /// Default
    /// false
    EnableEnhancedDeterminism = 16384,
    /// Controls processing friction in all solver iterations
    ///
    /// By default, PhysX processes friction only in the final 3 position iterations, and all velocity
    /// iterations. This flag enables friction processing in all position and velocity iterations.
    ///
    /// The default behaviour provides a good trade-off between performance and stability and is aimed
    /// primarily at game development.
    ///
    /// When simulating more complex frictional behaviour, such as grasping of complex geometries with
    /// a robotic manipulator, better results can be achieved by enabling friction in all solver iterations.
    ///
    /// This flag only has effect with the default solver. The TGS solver always performs friction per-iteration.
    EnableFrictionEveryIteration = 32768,
    /// Controls application of gravity and other external forces per TGS solver position iterations
    ///
    /// By default, external forces such as gravity are applied just once at the beginning of each simulate() call. With this
    /// flag enabled the same forces are applied in each sub time step (position iteration) of the TGS solver, leading to greater stability and better solver convergence.
    /// One consequence is that a body in freefall will move a shorter distance over the entire simulation step if the flag is raised.
    ///
    /// Note that raising this flag makes the distance traveled under freefall dependent on the number of solver iterations.
    /// Since solver iterations are determined per-island, bodies assigned to an island with fewer solver iterations will travel a larger distance than bodies assigned to an island with more iterations.
    ///
    /// This feature is only supported for the TGS solver.
    ///
    /// Default
    /// false
    EnableExternalForcesEveryIterationTgs = 65536,
    /// Enables the direct-GPU API. Raising this flag is only allowed if eENABLE_GPU_DYNAMICS is raised and
    /// PxBroadphaseType::eGPU is used.
    ///
    /// This is useful if your application only needs to communicate to the GPU via GPU buffers. Can be significantly
    /// faster.
    ///
    /// Enabling the direct-GPU API will disable the readback of simulation state from GPU to CPU. Simulation outputs
    /// can only be accessed using the direct-GPU API functions in PxDirectGPUAPI (PxDirectGPUAPI::getRigidDynamicData(),
    /// PxDirectGPUAPI::getArticulationData(), PxDirectGPUAPI::copyContactData()), and reading state directly from the actor
    /// is not allowed.
    ///
    /// This flag requires PxSceneFlag::eDISABLE_SLEEPING to be raised.
    ///
    /// This flag is not mutable and must be set in PxSceneDesc at scene creation.
    ///
    /// Default
    /// false
    EnableDirectGpuApi = 131072,
    /// Enables the computation of body accelerations for PxRigidDynamic actors.
    ///
    /// By default PhysX does not compute per-body accelerations for PxRigidDynamic actors (only for articulation links).
    /// This flag tells the system to compute them.
    ///
    /// Retrieve the accelerations using PxRigidBody::getLinearAcceleration() and PxRigidBody::getAngularAcceleration().
    ///
    /// If the flag is not enabled these functions will return valid accelerations for PxArticulationLink objects, but
    /// it will return zero for PxRigidDynamic actors.
    ///
    /// If the flag is enabled, these functions will return valid accelerations for both PxArticulationLink and
    /// PxRigidDynamic objects.
    ///
    /// This flag also enables PxRigidDynamicGPUAPIReadType::eLINEAR_ACCELERATION and PxRigidDynamicGPUAPIReadType::eANGULAR_ACCELERATION
    /// in the direct GPU API.
    ///
    /// This flag is not mutable and must be set in PxSceneDesc at scene creation.
    ///
    /// Default
    /// false
    EnableBodyAccelerations = 262144,
    /// Reorders articulation contact constraints and articulation joint maximum velocity constraints in the solver.
    ///
    /// When this flag is raised, the solver will observe the following order:
    /// - joint friction, joint drive, joint position limit
    /// - link dynamic contact
    /// - link static contact
    /// - joint max velocity
    ///
    /// When the flag is lowered, the solver will observe a modified order:
    /// - link dynamic contact
    /// - joint friction, joint drive, joint position limit
    /// - joint max velocity
    /// - link static contact
    ///
    /// Raising the flag can be useful for certain simulation scenarios such as gripping, where it is desirable for dynamic contact
    /// to be resolved after joint drive but before max joint velocity.
    ///
    /// Raising this flag may have a negative effect on simulation performance.
    ///
    /// A goal of raising this flag is shallower contact penetration. This will in turn result in a reduced force
    /// reported by PxArticulationCache::linkIncomingJointForce.
    ///
    /// Default
    /// false
    SolveArticulationContactLast = 524288,
    /// Disables all sleeping logic in the scene.
    ///
    /// When this flag is raised, no objects will be put to sleep. They will all be treated by the solver as awake.
    /// This is a performance optimization for use cases where sleeping is not desired.
    ///
    /// This flag is automatically enabled when PxSceneFlag::eENABLE_DIRECT_GPU_API is set.
    ///
    /// Default
    /// false
    DisableSleeping = 1048576,
    /// Disables all sleeping logic in the scene.
    ///
    /// When this flag is raised, no objects will be put to sleep. They will all be treated by the solver as awake.
    /// This is a performance optimization for use cases where sleeping is not desired.
    ///
    /// This flag is automatically enabled when PxSceneFlag::eENABLE_DIRECT_GPU_API is set.
    ///
    /// Default
    /// false
    MutableFlags = 4097,
}

bitflags::bitflags! {
    /// Flags for [`PxSceneFlag`]
    #[derive(Default)]
    #[repr(transparent)]
    pub struct PxSceneFlags: u32 {
        const EnableActiveActors = 1 << 0;
        const EnableCcd = 1 << 1;
        const DisableCcdResweep = 1 << 2;
        const EnablePcm = 1 << 6;
        const DisableContactReportBufferResize = 1 << 7;
        const DisableContactCache = 1 << 8;
        const RequireRwLock = 1 << 9;
        const EnableStabilization = 1 << 10;
        const EnableAveragePoint = 1 << 11;
        const ExcludeKinematicsFromActiveActors = 1 << 12;
        const EnableGpuDynamics = 1 << 13;
        const EnableEnhancedDeterminism = 1 << 14;
        const EnableFrictionEveryIteration = 1 << 15;
        const EnableExternalForcesEveryIterationTgs = 1 << 16;
        const EnableDirectGpuApi = 1 << 17;
        const EnableBodyAccelerations = 1 << 18;
        const SolveArticulationContactLast = 1 << 19;
        const DisableSleeping = 1 << 20;
        const MutableFlags = Self::EnableActiveActors.bits | Self::ExcludeKinematicsFromActiveActors.bits;
    }
}

/// Debug visualization parameters.
///
/// [`PxVisualizationParameter::eSCALE`] is the master switch for enabling visualization, please read the corresponding documentation
/// for further details.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PxVisualizationParameter {
    /// This overall visualization scale gets multiplied with the individual scales. Setting to zero ignores all visualizations. Default is 0.
    ///
    /// The below settings permit the debug visualization of various simulation properties.
    /// The setting is either zero, in which case the property is not drawn. Otherwise it is a scaling factor
    /// that determines the size of the visualization widgets.
    ///
    /// Only objects for which visualization is turned on using setFlag(eVISUALIZATION) are visualized (see [`PxActorFlag::eVISUALIZATION`], #PxShapeFlag::eVISUALIZATION, ...).
    /// Default is 0.
    ///
    /// Notes:
    /// - to see any visualization, you have to set PxVisualizationParameter::eSCALE to nonzero first.
    /// - the scale factor has been introduced because it's difficult (if not impossible) to come up with a
    /// good scale for 3D vectors. Normals are normalized and their length is always 1. But it doesn't mean
    /// we should render a line of length 1. Depending on your objects/scene, this might be completely invisible
    /// or extremely huge. That's why the scale factor is here, to let you tune the length until it's ok in
    /// your scene.
    /// - however, things like collision shapes aren't ambiguous. They are clearly defined for example by the
    /// triangles
    /// &
    /// polygons themselves, and there's no point in scaling that. So the visualization widgets
    /// are only scaled when it makes sense.
    ///
    /// Range:
    /// [0, PX_MAX_F32)
    /// Default:
    /// 0
    Scale = 0,
    /// Visualize the world axes.
    WorldAxes = 1,
    /// Visualize a bodies axes.
    BodyAxes = 2,
    /// Visualize a body's mass axes.
    ///
    /// This visualization is also useful for visualizing the sleep state of bodies. Sleeping bodies are drawn in
    /// black, while awake bodies are drawn in white. If the body is sleeping and part of a sleeping group, it is
    /// drawn in red.
    BodyMassAxes = 3,
    /// Visualize the bodies linear velocity.
    BodyLinVelocity = 4,
    /// Visualize the bodies angular velocity.
    BodyAngVelocity = 5,
    /// Visualize contact points. Will enable contact information.
    ContactPoint = 6,
    /// Visualize contact normals. Will enable contact information.
    ContactNormal = 7,
    /// Visualize contact errors. Will enable contact information.
    ContactError = 8,
    /// Visualize Contact impulses. Will enable contact information.
    ContactImpulse = 9,
    /// Visualize Contact forces. Will enable contact information.
    ///
    /// Use eCONTACT_IMPULSE instead.
    ContactForce = 9,
    /// Visualize friction points. Will enable contact information.
    FrictionPoint = 10,
    /// Visualize friction normals. Will enable contact information.
    FrictionNormal = 11,
    /// Visualize friction impulses. Will enable contact information.
    FrictionImpulse = 12,
    /// Visualize actor axes.
    ActorAxes = 13,
    /// Visualize bounds (AABBs in world space)
    CollisionAabbs = 14,
    /// Shape visualization
    CollisionShapes = 15,
    /// Shape axis visualization
    CollisionAxes = 16,
    /// Compound visualization (compound AABBs in world space)
    CollisionCompounds = 17,
    /// Mesh
    /// &
    /// convex face normals
    CollisionFnormals = 18,
    /// Active edges for meshes
    CollisionEdges = 19,
    /// Static pruning structures
    CollisionStatic = 20,
    /// Dynamic pruning structures
    CollisionDynamic = 21,
    /// Joint local axes
    JointLocalFrames = 22,
    /// Joint limits
    JointLimits = 23,
    /// Visualize culling box
    CullBox = 24,
    /// MBP regions
    MbpRegions = 25,
    /// Renders the simulation mesh instead of the collision mesh (only available for tetmeshes)
    ///
    /// Deformable visualization is currently not supported.
    SimulationMesh = 26,
    /// Renders the SDF of a mesh instead of the collision mesh (only available for triangle meshes with SDFs)
    Sdf = 27,
    /// This is not a parameter, it just records the current number of parameters (as maximum(PxVisualizationParameter)+1) for use in loops.
    NumValues = 28,
    /// This is not a parameter, it just records the current number of parameters (as maximum(PxVisualizationParameter)+1) for use in loops.
    ForceDword = 2147483647,
}

/// Different types of rigid body collision pair statistics.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum RbPairStatsType {
    /// Shape pairs processed as discrete contact pairs for the current simulation step.
    DiscreteContactPairs = 0,
    /// Shape pairs processed as swept integration pairs for the current simulation step.
    ///
    /// Counts the pairs for which special CCD (continuous collision detection) work was actually done and NOT the number of pairs which were configured for CCD.
    /// Furthermore, there can be multiple CCD passes and all processed pairs of all passes are summed up, hence the number can be larger than the amount of pairs which have been configured for CCD.
    CcdPairs = 1,
    /// Shape pairs processed with user contact modification enabled for the current simulation step.
    ModifiedContactPairs = 2,
    /// Trigger shape pairs processed for the current simulation step.
    TriggerPairs = 3,
}

/// PVD scene Flags. They are disabled by default, and only works if PxPvdInstrumentationFlag::eDEBUG is set.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PxPvdSceneFlag {
    TransmitContacts = 1,
    /// Transmits contact stream to PVD.
    TransmitScenequeries = 2,
    /// Transmits scene query stream to PVD.
    TransmitConstraints = 4,
}

bitflags::bitflags! {
    /// Flags for [`PxPvdSceneFlag`]
    #[derive(Default)]
    #[repr(transparent)]
    pub struct PxPvdSceneFlags: u8 {
        const TransmitContacts = 1 << 0;
        const TransmitScenequeries = 1 << 1;
        const TransmitConstraints = 1 << 2;
    }
}

/// Identifies each type of actor for retrieving actors from a scene.
///
/// [`PxArticulationLink`] objects are not supported. Use the #PxArticulationReducedCoordinate object to retrieve all its links.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PxActorTypeFlag {
    /// A static rigid body
    RigidStatic = 1,
    /// A dynamic rigid body
    RigidDynamic = 2,
}

bitflags::bitflags! {
    /// Flags for [`PxActorTypeFlag`]
    #[derive(Default)]
    #[repr(transparent)]
    pub struct PxActorTypeFlags: u16 {
        const RigidStatic = 1 << 0;
        const RigidDynamic = 1 << 1;
    }
}

/// Extra data item types for contact pairs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PxContactPairExtraDataType {
    /// see [`PxContactPairVelocity`]
    PreSolverVelocity = 0,
    /// see [`PxContactPairVelocity`]
    PostSolverVelocity = 1,
    /// see [`PxContactPairPose`]
    ContactEventPose = 2,
    /// see [`PxContactPairIndex`]
    ContactPairIndex = 3,
}

/// Collection of flags providing information on contact report pairs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PxContactPairHeaderFlag {
    /// The actor with index 0 has been removed from the scene.
    RemovedActor0 = 1,
    /// The actor with index 1 has been removed from the scene.
    RemovedActor1 = 2,
}

bitflags::bitflags! {
    /// Flags for [`PxContactPairHeaderFlag`]
    #[derive(Default)]
    #[repr(transparent)]
    pub struct PxContactPairHeaderFlags: u16 {
        const RemovedActor0 = 1 << 0;
        const RemovedActor1 = 1 << 1;
    }
}

/// Collection of flags providing information on contact report pairs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PxContactPairFlag {
    /// The shape with index 0 has been removed from the actor/scene.
    RemovedShape0 = 1,
    /// The shape with index 1 has been removed from the actor/scene.
    RemovedShape1 = 2,
    /// First actor pair contact.
    ///
    /// The provided shape pair marks the first contact between the two actors, no other shape pair has been touching prior to the current simulation frame.
    ///
    /// : This info is only available if [`PxPairFlag::eNOTIFY_TOUCH_FOUND`] has been declared for the pair.
    ActorPairHasFirstTouch = 4,
    /// All contact between the actor pair was lost.
    ///
    /// All contact between the two actors has been lost, no shape pairs remain touching after the current simulation frame.
    ActorPairLostTouch = 8,
    /// Internal flag, used by [`PxContactPair`].extractContacts()
    ///
    /// The applied contact impulses are provided for every contact point.
    /// This is the case if [`PxPairFlag::eSOLVE_CONTACT`] has been set for the pair.
    InternalHasImpulses = 16,
    /// Internal flag, used by [`PxContactPair`].extractContacts()
    ///
    /// The provided contact point information is flipped with regards to the shapes of the contact pair. This mainly concerns the order of the internal triangle indices.
    InternalContactsAreFlipped = 32,
}

bitflags::bitflags! {
    /// Flags for [`PxContactPairFlag`]
    #[derive(Default)]
    #[repr(transparent)]
    pub struct PxContactPairFlags: u16 {
        const RemovedShape0 = 1 << 0;
        const RemovedShape1 = 1 << 1;
        const ActorPairHasFirstTouch = 1 << 2;
        const ActorPairLostTouch = 1 << 3;
        const InternalHasImpulses = 1 << 4;
        const InternalContactsAreFlipped = 1 << 5;
    }
}

/// Collection of flags providing information on trigger report pairs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PxTriggerPairFlag {
    /// The trigger shape has been removed from the actor/scene.
    RemovedShapeTrigger = 1,
    /// The shape causing the trigger event has been removed from the actor/scene.
    RemovedShapeOther = 2,
    /// For internal use only.
    NextFree = 4,
}

bitflags::bitflags! {
    /// Flags for [`PxTriggerPairFlag`]
    #[derive(Default)]
    #[repr(transparent)]
    pub struct PxTriggerPairFlags: u8 {
        const RemovedShapeTrigger = 1 << 0;
        const RemovedShapeOther = 1 << 1;
        const NextFree = 1 << 2;
    }
}

/// The type of controller, eg box, sphere or capsule.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PxControllerShapeType {
    /// A box controller.
    Box = 0,
    /// A capsule controller
    Capsule = 1,
    /// A capsule controller
    ForceDword = 2147483647,
}

/// specifies how a CCT interacts with non-walkable parts.
///
/// This is only used when slopeLimit is non zero. It is currently enabled for static actors only, and not supported for spheres or capsules.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PxControllerNonWalkableMode {
    /// Stops character from climbing up non-walkable slopes, but doesn't move it otherwise
    PreventClimbing = 0,
    /// Stops character from climbing up non-walkable slopes, and forces it to slide down those slopes
    PreventClimbingAndForceSliding = 1,
}

/// specifies which sides a character is colliding with.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PxControllerCollisionFlag {
    /// Character is colliding to the sides.
    CollisionSides = 1,
    /// Character has collision above.
    CollisionUp = 2,
    /// Character has collision below.
    CollisionDown = 4,
}

bitflags::bitflags! {
    /// Flags for [`PxControllerCollisionFlag`]
    #[derive(Default)]
    #[repr(transparent)]
    pub struct PxControllerCollisionFlags: u8 {
        const CollisionSides = 1 << 0;
        const CollisionUp = 1 << 1;
        const CollisionDown = 1 << 2;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PxCapsuleClimbingMode {
    /// Standard mode, let the capsule climb over surfaces according to impact normal
    Easy = 0,
    /// Constrained mode, try to limit climbing according to the step offset
    Constrained = 1,
    Last = 2,
}

/// specifies controller behavior
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PxControllerBehaviorFlag {
    /// Controller can ride on touched object (i.e. when this touched object is moving horizontally).
    ///
    /// The CCT vs. CCT case is not supported.
    CctCanRideOnObject = 1,
    /// Controller should slide on touched object
    CctSlide = 2,
    /// Disable all code dealing with controllers riding on objects, let users define it outside of the SDK.
    CctUserDefinedRide = 4,
}

bitflags::bitflags! {
    /// Flags for [`PxControllerBehaviorFlag`]
    #[derive(Default)]
    #[repr(transparent)]
    pub struct PxControllerBehaviorFlags: u8 {
        const CctCanRideOnObject = 1 << 0;
        const CctSlide = 1 << 1;
        const CctUserDefinedRide = 1 << 2;
    }
}

/// specifies debug-rendering flags
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum PxControllerDebugRenderFlag {
    /// Temporal bounding volume around controllers
    TemporalBv = 1,
    /// Cached bounding volume around controllers
    CachedBv = 2,
    /// User-defined obstacles
    Obstacles = 4,
    None = 0,
    All = 4294967295,
}

bitflags::bitflags! {
    /// Flags for [`PxControllerDebugRenderFlag`]
    #[derive(Default)]
    #[repr(transparent)]
    pub struct PxControllerDebugRenderFlags: u32 {
        const TemporalBv = 1 << 0;
        const CachedBv = 1 << 1;
        const Obstacles = 1 << 2;
        const All = Self::TemporalBv.bits | Self::CachedBv.bits | Self::Obstacles.bits;
    }
}

/// Defines the number of bits per subgrid pixel
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PxSdfBitsPerSubgridPixel {
    /// 8 bit per subgrid pixel (values will be stored as normalized integers)
    E8BitPerPixel = 1,
    /// 16 bit per subgrid pixel (values will be stored as normalized integers)
    E16BitPerPixel = 2,
    /// 32 bit per subgrid pixel (values will be stored as floats in world scale units)
    E32BitPerPixel = 4,
}

/// Flags which describe the format and behavior of a convex mesh.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PxConvexFlag {
    /// Denotes the use of 16-bit vertex indices in PxConvexMeshDesc::triangles or PxConvexMeshDesc::polygons.
    /// (otherwise, 32-bit indices are assumed)
    E16BitIndices = 1,
    /// Automatically recomputes the hull from the vertices. If this flag is not set, you must provide the entire geometry manually.
    ///
    /// For the specific algorithm used in hull computation, please see PxConvexMeshCookingType.
    ComputeConvex = 2,
    /// Checks and removes almost zero-area triangles during convex hull computation.
    /// The rejected area size is specified in PxCookingParams::areaTestEpsilon
    ///
    /// This flag is only used in combination with eCOMPUTE_CONVEX.
    CheckZeroAreaTriangles = 4,
    /// Quantizes the input vertices using the k-means clustering
    ///
    /// The input vertices are quantized to PxConvexMeshDesc::quantizedCount
    /// see http://en.wikipedia.org/wiki/K-means_clustering
    QuantizeInput = 8,
    /// Disables the convex mesh validation to speed-up hull creation. Please use separate validation
    /// function in checked/debug builds. Creating a convex mesh with invalid input data without prior validation
    /// may result in undefined behavior.
    DisableMeshValidation = 16,
    /// Enables plane shifting vertex limit algorithm.
    ///
    /// Plane shifting is an alternative algorithm for the case when the computed hull has more vertices
    /// than the specified vertex limit.
    ///
    /// The default algorithm computes the full hull, and an OBB around the input vertices. This OBB is then sliced
    /// with the hull planes until the vertex limit is reached.The default algorithm requires the vertex limit
    /// to be set to at least 8, and typically produces results that are much better quality than are produced
    /// by plane shifting.
    ///
    /// When plane shifting is enabled, the hull computation stops when vertex limit is reached. The hull planes
    /// are then shifted to contain all input vertices, and the new plane intersection points are then used to
    /// generate the final hull with the given vertex limit.Plane shifting may produce sharp edges to vertices
    /// very far away from the input cloud, and does not guarantee that all input vertices are inside the resulting
    /// hull.However, it can be used with a vertex limit as low as 4.
    PlaneShifting = 32,
    /// Inertia tensor computation is faster using SIMD code, but the precision is lower, which may result
    /// in incorrect inertia for very thin hulls.
    FastInertiaComputation = 64,
    /// Convex hull input vertices are shifted to be around origin to provide better computation stability.
    /// It is recommended to provide input vertices around the origin, otherwise use this flag to improve
    /// numerical stability.
    ///
    /// Is used only with eCOMPUTE_CONVEX flag.
    ShiftVertices = 256,
}

bitflags::bitflags! {
    /// Flags for [`PxConvexFlag`]
    #[derive(Default)]
    #[repr(transparent)]
    pub struct PxConvexFlags: u16 {
        const E16BitIndices = 1 << 0;
        const ComputeConvex = 1 << 1;
        const CheckZeroAreaTriangles = 1 << 2;
        const QuantizeInput = 1 << 3;
        const DisableMeshValidation = 1 << 4;
        const PlaneShifting = 1 << 5;
        const FastInertiaComputation = 1 << 6;
        const ShiftVertices = 1 << 8;
    }
}

/// Defines the tetrahedron structure of a mesh.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PxMeshFormat {
    /// Normal tetmesh with arbitrary tetrahedra
    TetMesh = 0,
    /// 5 or 6 tetrahedra in a row will form a hexahedron
    HexMesh = 1,
}

/// This is only used for BVH33 which is deprecated and will be removed in a future version. Use BVH34 instead.
///
/// Enumeration for mesh cooking hints.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PxMeshCookingHint {
    /// Default value. Favors higher quality hierarchy with higher runtime performance over cooking speed.
    SimPerformance = 0,
    /// Enables fast cooking path at the expense of somewhat lower quality hierarchy construction.
    CookingPerformance = 1,
}

/// Desired build strategy for PxMeshMidPhase::eBVH34
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PxBVH34BuildStrategy {
    /// Fast build strategy. Fast build speed, good runtime performance in most cases. Recommended for runtime mesh cooking.
    Fast = 0,
    /// Default build strategy. Medium build speed, good runtime performance in all cases.
    Default = 1,
    /// SAH build strategy. Slower builds, slightly improved runtime performance in some cases.
    Sah = 2,
    Last = 3,
}

/// Result from convex cooking.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PxConvexMeshCookingResult {
    /// Convex mesh cooking succeeded.
    Success = 0,
    /// Convex mesh cooking failed, algorithm couldn't find 4 initial vertices without a small triangle.
    ZeroAreaTestFailed = 1,
    /// Convex mesh cooking succeeded, but the algorithm has reached the 255 polygons limit.
    /// The produced hull does not contain all input vertices. Try to simplify the input vertices
    /// or try to use the eINFLATE_CONVEX or the eQUANTIZE_INPUT flags.
    PolygonsLimitReached = 2,
    /// Something unrecoverable happened. Check the error stream to find out what.
    Failure = 3,
    /// Convex mesh cooking succeeded, but the algorithm could not make the mesh GPU compatible because the
    /// in-sphere radius is more than 100x smaller than the largest extent. Collision detection for any pair involving
    /// this convex mesh will fall back to CPU.
    NonGpuCompatible = 4,
}

/// Enumeration for convex mesh cooking algorithms.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PxConvexMeshCookingType {
    /// The Quickhull algorithm constructs the hull from the given input points. The resulting hull
    /// will only contain a subset of the input points.
    Quickhull = 0,
}

/// Result from triangle mesh cooking
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PxTriangleMeshCookingResult {
    /// Everything is A-OK.
    Success = 0,
    /// A triangle is too large for well-conditioned results. Tessellate the mesh for better behavior, see the user guide section on cooking for more details.
    LargeTriangle = 1,
    /// The mesh cleaning operation removed all triangles, resulting in an empty mesh.
    EmptyMesh = 2,
    /// Something unrecoverable happened. Check the error stream to find out what.
    Failure = 3,
}

/// Enum for the set of mesh pre-processing parameters.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PxMeshPreprocessingFlag {
    /// When set, mesh welding is performed. See PxCookingParams::meshWeldTolerance. Mesh cleaning must be enabled.
    WeldVertices = 1,
    /// When set, mesh cleaning is disabled. This makes cooking faster.
    ///
    /// When mesh cleaning is disabled, mesh welding is also disabled.
    ///
    /// It is recommended to use only meshes that passed during validateTriangleMesh.
    DisableCleanMesh = 2,
    /// When set, active edges are not computed and just enabled for all edges. This makes cooking faster but contact generation slower.
    DisableActiveEdgesPrecompute = 4,
    /// When set, 32-bit indices will always be created regardless of triangle count.
    ///
    /// By default mesh will be created with 16-bit indices for triangle count
    /// <
    /// = 0xFFFF and 32-bit otherwise.
    Force32bitIndices = 8,
    /// When set, a list of triangles will be created for each associated vertex in the mesh.
    EnableVertMapping = 16,
    /// When set, inertia data is calculated for the mesh, assuming unit density.
    EnableInertia = 32,
}

bitflags::bitflags! {
    /// Flags for [`PxMeshPreprocessingFlag`]
    #[derive(Default)]
    #[repr(transparent)]
    pub struct PxMeshPreprocessingFlags: u32 {
        const WeldVertices = 1 << 0;
        const DisableCleanMesh = 1 << 1;
        const DisableActiveEdgesPrecompute = 1 << 2;
        const Force32bitIndices = 1 << 3;
        const EnableVertMapping = 1 << 4;
        const EnableInertia = 1 << 5;
    }
}

/// Unique identifiers for extensions classes which implement a constraint based on PxConstraint.
///
/// Users which want to create their own custom constraint types should choose an ID larger or equal to eNEXT_FREE_ID
/// and not eINVALID_ID.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PxConstraintExtIDs {
    Joint = 0,
    VehicleJoint = 1,
    NextFreeId = 2,
    InvalidId = 2147483647,
}

/// an enumeration of PhysX' built-in joint types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PxJointConcreteType {
    Spherical = 256,
    Revolute = 257,
    Prismatic = 258,
    Fixed = 259,
    Distance = 260,
    D6 = 261,
    Gear = 262,
    RackAndPinion = 263,
    Last = 264,
}

/// an enumeration for specifying one or other of the actors referenced by a joint
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PxJointActorIndex {
    Actor0 = 0,
    Actor1 = 1,
    Count = 2,
}

/// flags for configuring the drive of a PxDistanceJoint
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PxDistanceJointFlag {
    MaxDistanceEnabled = 2,
    MinDistanceEnabled = 4,
    SpringEnabled = 8,
}

bitflags::bitflags! {
    /// Flags for [`PxDistanceJointFlag`]
    #[derive(Default)]
    #[repr(transparent)]
    pub struct PxDistanceJointFlags: u16 {
        const MaxDistanceEnabled = 1 << 1;
        const MinDistanceEnabled = 1 << 2;
        const SpringEnabled = 1 << 3;
    }
}

/// Flags specific to the prismatic joint.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PxPrismaticJointFlag {
    LimitEnabled = 2,
}

bitflags::bitflags! {
    /// Flags for [`PxPrismaticJointFlag`]
    #[derive(Default)]
    #[repr(transparent)]
    pub struct PxPrismaticJointFlags: u16 {
        const LimitEnabled = 1 << 1;
    }
}

/// Flags specific to the Revolute Joint.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PxRevoluteJointFlag {
    /// enable the limit
    LimitEnabled = 1,
    /// enable the drive
    DriveEnabled = 2,
    /// if the existing velocity is beyond the drive velocity, do not add force
    DriveFreespin = 4,
}

bitflags::bitflags! {
    /// Flags for [`PxRevoluteJointFlag`]
    #[derive(Default)]
    #[repr(transparent)]
    pub struct PxRevoluteJointFlags: u16 {
        const LimitEnabled = 1 << 0;
        const DriveEnabled = 1 << 1;
        const DriveFreespin = 1 << 2;
    }
}

/// Flags specific to the spherical joint.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PxSphericalJointFlag {
    /// the cone limit for the spherical joint is enabled
    LimitEnabled = 2,
}

bitflags::bitflags! {
    /// Flags for [`PxSphericalJointFlag`]
    #[derive(Default)]
    #[repr(transparent)]
    pub struct PxSphericalJointFlags: u16 {
        const LimitEnabled = 1 << 1;
    }
}

/// Used to specify one of the degrees of freedom of  a D6 joint.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PxD6Axis {
    /// motion along the X axis
    X = 0,
    /// motion along the Y axis
    Y = 1,
    /// motion along the Z axis
    Z = 2,
    /// motion around the X axis
    Twist = 3,
    /// motion around the Y axis
    Swing1 = 4,
    /// motion around the Z axis
    Swing2 = 5,
    Count = 6,
}

/// Used to specify the range of motions allowed for a degree of freedom in a D6 joint.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PxD6Motion {
    /// The DOF is locked, it does not allow relative motion.
    Locked = 0,
    /// The DOF is limited, it only allows motion within a specific range.
    Limited = 1,
    /// The DOF is free and has its full range of motion.
    Free = 2,
}

/// The configuration to use for driving to the angular component of a target pose or velocity.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PxD6AngularDriveConfig {
    /// The joint tries to reach the angular drive target by separately driving along each angular degree of freedom.
    ///
    /// Each angular degree of freedom can have its own set of drive parameters. The degrees of freedom are covered by a twist and two swing axes.
    /// As a consequence, only the following options are available when setting the drive parameters: PxD6Drive::eSWING1, PxD6Drive::eSWING2,
    /// PxD6Drive::eTWIST (see [`PxD6Joint::setDrive`]()).
    SwingTwist = 0,
    /// The joint tries to reach the angular drive target by following a spherical linear interpolation (SLERP) based path.
    ///
    /// A single set of drive parameters will be used for all angular degrees of freedom and PxD6Drive::eSLERP is the only valid option to set
    /// those parameters (see [`PxD6Joint::setDrive`]()).
    Slerp = 1,
}

/// Used to specify which axes of a D6 joint are driven.
///
/// Each drive is an implicit force-limited damped spring:
///
/// force = spring * (target position - position) + damping * (targetVelocity - velocity)
///
/// Alternatively, the spring may be configured to generate a specified acceleration instead of a force.
///
/// A linear axis is affected by drive only if the corresponding drive flag is set. There are two possible models
/// for angular drive: swing/twist, which may be used to drive one or more angular degrees of freedom, or slerp,
/// which may only be used to drive all three angular degrees simultaneously. Please use [`PxD6AngularDriveConfig`]
/// to configure the angular drive model.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PxD6Drive {
    /// drive along the X-axis
    X = 0,
    /// drive along the Y-axis
    Y = 1,
    /// drive along the Z-axis
    Z = 2,
    /// rotational drive around the X-axis
    ///
    /// Only allowed if the angular drive configuration is set to PxD6AngularDriveConfig::eSWING_TWIST.
    Twist = 3,
    /// rotational drive around the Y-axis
    ///
    /// Only allowed if the angular drive configuration is set to PxD6AngularDriveConfig::eSWING_TWIST.
    Swing1 = 4,
    /// rotational drive around the Z-axis
    ///
    /// Only allowed if the angular drive configuration is set to PxD6AngularDriveConfig::eSWING_TWIST.
    Swing2 = 5,
    /// drive of all three angular degrees along a SLERP-path
    ///
    /// Only allowed if the angular drive configuration is set to PxD6AngularDriveConfig::eSLERP.
    Slerp = 6,
    /// drive of all three angular degrees along a SLERP-path
    ///
    /// Only allowed if the angular drive configuration is set to PxD6AngularDriveConfig::eSLERP.
    Count = 7,
}

impl From<usize> for PxD6Drive {
    fn from(val: usize) -> Self {
        #[allow(clippy::match_same_arms)]
        match val {
            0 => Self::X,
            1 => Self::Y,
            2 => Self::Z,
            3 => Self::Twist,
            4 => Self::Swing1,
            5 => Self::Swing2,
            6 => Self::Slerp,
            7 => Self::Count,
            _ => Self::Count,
        }
    }
}

/// flags for configuring the drive model of a PxD6Joint
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PxD6JointDriveFlag {
    /// drive spring is for the acceleration at the joint (rather than the force)
    Acceleration = 1,
    /// Add drive force/torque to the joint force/torque total.
    ///
    /// If this flag is raised, the force/torque value from this drive constraint will be accumulated
    /// in the force/torque total that is reported for the underlying PxConstraint object. Note that
    /// because the force/torque total changes, the joint break behavior will change too.
    ///
    /// Default:
    /// False
    OutputForce = 2,
}

bitflags::bitflags! {
    /// Flags for [`PxD6JointDriveFlag`]
    #[derive(Default)]
    #[repr(transparent)]
    pub struct PxD6JointDriveFlags: u32 {
        const Acceleration = 1 << 0;
        const OutputForce = 1 << 1;
    }
}

/// Collision filtering operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PxFilterOp {
    PxFilteropAnd = 0,
    PxFilteropOr = 1,
    PxFilteropXor = 2,
    PxFilteropNand = 3,
    PxFilteropNor = 4,
    PxFilteropNxor = 5,
    PxFilteropSwapAnd = 6,
}

/// If a thread ends up waiting for work it will find itself in a spin-wait loop until work becomes available.
/// Three strategies are available to limit wasted cycles.
/// The strategies are as follows:
/// a) wait until a work task signals the end of the spin-wait period.
/// b) yield the thread by providing a hint to reschedule thread execution, thereby allowing other threads to run.
/// c) yield the processor by informing it that it is waiting for work and requesting it to more efficiently use compute resources.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PxDefaultCpuDispatcherWaitForWorkMode {
    WaitForWork = 0,
    YieldThread = 1,
    YieldProcessor = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PxVehicleLimits {
    MaxNbWheels = 20,
    MaxNbAxles = 20,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PxVehicleComponentSequenceLimits {
    MaxNbSubgroups = 16,
    MaxNbComponents = 64,
    MaxNbSubgroupelements = 80,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PxVehicleAxes {
    /// The +x axis
    PosX = 0,
    /// The -x axis
    NegX = 1,
    /// The +y axis
    PosY = 2,
    /// The -y axis
    NegY = 3,
    /// The +z axis
    PosZ = 4,
    /// The -z axis
    NegZ = 5,
    MaxNbAxes = 6,
}

/// Determine whether the PhysX actor associated with a vehicle is to be updated with a velocity change or an acceleration change.
/// A velocity change will be immediately reflected in linear and angular velocity queries against the vehicle.  An acceleration change, on the other hand,
/// will leave the linear and angular velocities unchanged until the next PhysX scene update has applied the acceleration update to the actor's linear and
/// angular velocities.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PxVehiclePhysXActorUpdateMode {
    ApplyVelocity = 0,
    ApplyAcceleration = 1,
}

/// Tires have two important directions for the purposes of tire force computation: longitudinal and lateral.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PxVehicleTireDirectionModes {
    Longitudinal = 0,
    Lateral = 1,
    MaxNbPlanarDirections = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PxVehicleSimulationContextType {
    /// The simulation context inherits from PxVehicleSimulationContext
    Default = 0,
    /// The simulation context inherits from PxVehiclePhysXSimulationContext
    Physx = 1,
}

/// Choose between a potentially more expensive but more accurate solution to the clutch model or a potentially cheaper but less accurate solution.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PxVehicleClutchAccuracyMode {
    Estimate = 0,
    BestPossible = 1,
}

/// Set the direction to apply a constraint impulse when the suspension cannot place the wheel on the ground
/// and simultaneously respect the limits of suspension travel. The choices are to push along the ground normal to resolve the
/// geometric error or to push along the suspension direction. The former choice can be thought of as mimicing a force applied
/// by the tire's contact with the ground, while the latter can be thought of as mimicing a force arising from a suspension limit spring.
/// When the ground normal and the suspension direction are approximately aligned, both do an equivalent job of maintaining the wheel above
/// the ground. When the vehicle is on its side, eSUSPENSION does a better job of keeping the wheels above
/// the ground but comes at the cost of an unnaturally strong torque that can lead to unwanted self-righting behaviour.
/// eROAD_GEOMETRY_NORMAL is a good choice to avoid self-righting behaviour and still do a reasonable job at maintaining
/// the wheel above the ground in the event that the vehicle is tending towards a roll onto its side.
/// eNONE should be chosen if it is desired that no extra impulse is applied when the suspension alone cannot keep the wheels above
/// the ground plane.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum DirectionSpecifier {
    Suspension = 0,
    RoadGeometryNormal = 1,
    None = 2,
}

/// A description of the number of PxConstraintConnector instances per vehicle required to maintain suspension limit
/// and sticky tire instances.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PxVehiclePhysXConstraintLimits {
    NbDofsPerPxconstraint = 12,
    NbDofsPerWheel = 3,
    NbWheelsPerPxconstraint = 4,
    NbConstraintsPerVehicle = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PxVehicleSuspensionJounceCalculationType {
    /// The jounce is calculated using a raycast against the plane of the road geometry state
    Raycast = 0,
    /// The jounce is calculated by sweeping a cylinder against the plane of the road geometry state
    Sweep = 1,
    MaxNb = 2,
}

/// PhysX scene queries may be raycasts or sweeps.
///
/// eNONE will result in no PhysX scene query. This option will not overwrite the associated PxVehicleRoadGeometryState.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PxVehiclePhysXRoadGeometryQueryType {
    /// Info about the road geometry below the wheel is provided by the user
    None = 0,
    /// The road geometry below the wheel is analyzed using a raycast query
    Raycast = 1,
    /// The road geometry below the wheel is analyzed using a sweep query
    Sweep = 2,
    MaxNb = 3,
}

/// types of instrumentation that PVD can do.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PxPvdInstrumentationFlag {
    /// Send debugging information to PVD.
    ///
    /// This information is the actual object data of the rigid statics, shapes,
    /// articulations, etc.  Sending this information has a noticeable impact on
    /// performance and thus this flag should not be set if you want an accurate
    /// performance profile.
    Debug = 1,
    /// Send profile information to PVD.
    ///
    /// This information populates PVD's profile view.  It has (at this time) negligible
    /// cost compared to Debug information and makes PVD *much* more useful so it is quite
    /// highly recommended.
    ///
    /// This flag works together with a PxCreatePhysics parameter.
    /// Using it allows the SDK to send profile events to PVD.
    Profile = 2,
    /// Send memory information to PVD.
    ///
    /// The PVD sdk side hooks into the Foundation memory controller and listens to
    /// allocation/deallocation events.  This has a noticable hit on the first frame,
    /// however, this data is somewhat compressed and the PhysX SDK doesn't allocate much
    /// once it hits a steady state.  This information also has a fairly negligible
    /// impact and thus is also highly recommended.
    ///
    /// This flag works together with a PxCreatePhysics parameter,
    /// trackOutstandingAllocations.  Using both of them together allows users to have
    /// an accurate view of the overall memory usage of the simulation at the cost of
    /// a hashtable lookup per allocation/deallocation.  Again, PhysX makes a best effort
    /// attempt not to allocate or deallocate during simulation so this hashtable lookup
    /// tends to have no effect past the first frame.
    ///
    /// Sending memory information without tracking outstanding allocations means that
    /// PVD will accurate information about the state of the memory system before the
    /// actual connection happened.
    Memory = 4,
    /// Send memory information to PVD.
    ///
    /// The PVD sdk side hooks into the Foundation memory controller and listens to
    /// allocation/deallocation events.  This has a noticable hit on the first frame,
    /// however, this data is somewhat compressed and the PhysX SDK doesn't allocate much
    /// once it hits a steady state.  This information also has a fairly negligible
    /// impact and thus is also highly recommended.
    ///
    /// This flag works together with a PxCreatePhysics parameter,
    /// trackOutstandingAllocations.  Using both of them together allows users to have
    /// an accurate view of the overall memory usage of the simulation at the cost of
    /// a hashtable lookup per allocation/deallocation.  Again, PhysX makes a best effort
    /// attempt not to allocate or deallocate during simulation so this hashtable lookup
    /// tends to have no effect past the first frame.
    ///
    /// Sending memory information without tracking outstanding allocations means that
    /// PVD will accurate information about the state of the memory system before the
    /// actual connection happened.
    All = 7,
}

bitflags::bitflags! {
    /// Flags for [`PxPvdInstrumentationFlag`]
    #[derive(Default)]
    #[repr(transparent)]
    pub struct PxPvdInstrumentationFlags: u8 {
        const Debug = 1 << 0;
        const Profile = 1 << 1;
        const Memory = 1 << 2;
        const All = Self::Debug.bits | Self::Profile.bits | Self::Memory.bits;
    }
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxAllocatorCallback {
    vtable_: *const std::ffi::c_void,
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxFoundation {
    vtable_: *const std::ffi::c_void,
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxInputStream {
    vtable_: *const std::ffi::c_void,
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxInputData {
    vtable_: *const std::ffi::c_void,
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxOutputStream {
    vtable_: *const std::ffi::c_void,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct PxLogTwo {
    _unused: [u8; 0],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct PxUnConst {
    _unused: [u8; 0],
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxErrorCallback {
    vtable_: *const std::ffi::c_void,
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxAllocationListener {
    vtable_: *const std::ffi::c_void,
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxBroadcastingAllocator {
    vtable_: *const std::ffi::c_void,
mAllocator: *mut PxAllocatorCallback,
mError: *mut PxErrorCallback,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct PxHash {
    _unused: [u8; 0],
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxProfilerCallback {
    vtable_: *const std::ffi::c_void,
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxSocket {
    vtable_: *const std::ffi::c_void,
mImpl: *mut std::ffi::c_void,
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxRunnable {
    vtable_: *const std::ffi::c_void,
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxRenderBuffer {
    vtable_: *const std::ffi::c_void,
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxProcessPxBaseCallback {
    vtable_: *const std::ffi::c_void,
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxSerializationContext {
    vtable_: *const std::ffi::c_void,
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxDeserializationContext {
    vtable_: *const std::ffi::c_void,
mExtraDataAddress: *mut u8,
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxSerializationRegistry {
    vtable_: *const std::ffi::c_void,
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxCollection {
    vtable_: *const std::ffi::c_void,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct PxTypeInfo {
    _unused: [u8; 0],
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxBase {
    vtable_: *const std::ffi::c_void,
mConcreteType: u16,
mBaseFlags: PxBaseFlags,
mBuiltInRefCount: u32,
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxRefCounted {
    vtable_: *const std::ffi::c_void,
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxStringTable {
    vtable_: *const std::ffi::c_void,
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxSerializer {
    vtable_: *const std::ffi::c_void,
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxInsertionCallback {
    vtable_: *const std::ffi::c_void,
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxTaskManager {
    vtable_: *const std::ffi::c_void,
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxCpuDispatcher {
    vtable_: *const std::ffi::c_void,
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxBaseTask {
    vtable_: *const std::ffi::c_void,
mContextID: u64,
mTm: *mut PxTaskManager,
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxTask {
    vtable_: *const std::ffi::c_void,
mTaskID: u32,
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxLightCpuTask {
    vtable_: *const std::ffi::c_void,
mCont: *mut PxBaseTask,
mRefCount: i32,
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxReportCallbackBase {
    vtable_: *const std::ffi::c_void,
    pub mCapacity: u32,
    pub mSize: u32,
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxBVH {
    vtable_: *const std::ffi::c_void,
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxConvexMesh {
    vtable_: *const std::ffi::c_void,
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxSimulationFilterCallback {
    vtable_: *const std::ffi::c_void,
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxActor {
    vtable_: *const std::ffi::c_void,
    pub userData: *mut std::ffi::c_void,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct PxCudaContextManager {
    _unused: [u8; 0],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct PxGpuParticleSystem {
    _unused: [u8; 0],
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxParticleSystemCallback {
    vtable_: *const std::ffi::c_void,
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxMultiCallback {
    vtable_: *const std::ffi::c_void,
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxPBDParticleSystem {
    vtable_: *const std::ffi::c_void,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct PxContactBuffer {
    _unused: [u8; 0],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct PxRenderOutput {
    _unused: [u8; 0],
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxHeightField {
    vtable_: *const std::ffi::c_void,
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxTriangleMesh {
    vtable_: *const std::ffi::c_void,
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxBVH33TriangleMesh {
    vtable_: *const std::ffi::c_void,
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxBVH34TriangleMesh {
    vtable_: *const std::ffi::c_void,
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxDeformableVolumeAuxData {
    vtable_: *const std::ffi::c_void,
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxTetrahedronMesh {
    vtable_: *const std::ffi::c_void,
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxDeformableVolumeMesh {
    vtable_: *const std::ffi::c_void,
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxCollisionMeshMappingData {
    vtable_: *const std::ffi::c_void,
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxCollisionTetrahedronMeshData {
    vtable_: *const std::ffi::c_void,
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxSimulationTetrahedronMeshData {
    vtable_: *const std::ffi::c_void,
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxAggregate {
    vtable_: *const std::ffi::c_void,
    pub userData: *mut std::ffi::c_void,
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxConstraintVisualizer {
    vtable_: *const std::ffi::c_void,
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxConstraintConnector {
    vtable_: *const std::ffi::c_void,
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxConstraintAllocator {
    vtable_: *const std::ffi::c_void,
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxArticulationAttachment {
    vtable_: *const std::ffi::c_void,
    pub userData: *mut std::ffi::c_void,
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxArticulationTendonJoint {
    vtable_: *const std::ffi::c_void,
    pub userData: *mut std::ffi::c_void,
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxArticulationTendon {
    vtable_: *const std::ffi::c_void,
    pub userData: *mut std::ffi::c_void,
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxArticulationSpatialTendon {
    vtable_: *const std::ffi::c_void,
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxArticulationFixedTendon {
    vtable_: *const std::ffi::c_void,
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxArticulationMimicJoint {
    vtable_: *const std::ffi::c_void,
    pub userData: *mut std::ffi::c_void,
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxArticulationReducedCoordinate {
    vtable_: *const std::ffi::c_void,
    pub userData: *mut std::ffi::c_void,
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxArticulationJointReducedCoordinate {
    vtable_: *const std::ffi::c_void,
    pub userData: *mut std::ffi::c_void,
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxShape {
    vtable_: *const std::ffi::c_void,
    pub userData: *mut std::ffi::c_void,
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxRigidActor {
    vtable_: *const std::ffi::c_void,
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxRigidBody {
    vtable_: *const std::ffi::c_void,
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxArticulationLink {
    vtable_: *const std::ffi::c_void,
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxConstraint {
    vtable_: *const std::ffi::c_void,
    pub userData: *mut std::ffi::c_void,
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxBaseMaterial {
    vtable_: *const std::ffi::c_void,
    pub userData: *mut std::ffi::c_void,
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxMaterial {
    vtable_: *const std::ffi::c_void,
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxContactModifyCallback {
    vtable_: *const std::ffi::c_void,
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxCCDContactModifyCallback {
    vtable_: *const std::ffi::c_void,
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxDeformableBody {
    vtable_: *const std::ffi::c_void,
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxDeformableSurface {
    vtable_: *const std::ffi::c_void,
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxDeformableMaterial {
    vtable_: *const std::ffi::c_void,
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxDeformableSurfaceMaterial {
    vtable_: *const std::ffi::c_void,
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxDeformableVolume {
    vtable_: *const std::ffi::c_void,
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxDeformableVolumeMaterial {
    vtable_: *const std::ffi::c_void,
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxDeletionListener {
    vtable_: *const std::ffi::c_void,
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxParticleBuffer {
    vtable_: *const std::ffi::c_void,
    pub userData: *mut std::ffi::c_void,
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxParticleAndDiffuseBuffer {
    vtable_: *const std::ffi::c_void,
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxPBDMaterial {
    vtable_: *const std::ffi::c_void,
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxDeformableAttachment {
    vtable_: *const std::ffi::c_void,
    pub userData: *mut std::ffi::c_void,
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxDeformableElementFilter {
    vtable_: *const std::ffi::c_void,
    pub userData: *mut std::ffi::c_void,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct PxOmniPvd {
    _unused: [u8; 0],
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxPhysics {
    vtable_: *const std::ffi::c_void,
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxQueryFilterCallback {
    vtable_: *const std::ffi::c_void,
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxRigidDynamic {
    vtable_: *const std::ffi::c_void,
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxRigidStatic {
    vtable_: *const std::ffi::c_void,
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxDirectGPUAPI {
    vtable_: *const std::ffi::c_void,
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxSceneQuerySystemBase {
    vtable_: *const std::ffi::c_void,
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxSceneSQSystem {
    vtable_: *const std::ffi::c_void,
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxSceneQuerySystem {
    vtable_: *const std::ffi::c_void,
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxBroadPhaseRegions {
    vtable_: *const std::ffi::c_void,
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxBroadPhase {
    vtable_: *const std::ffi::c_void,
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxAABBManager {
    vtable_: *const std::ffi::c_void,
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxPvdSceneClient {
    vtable_: *const std::ffi::c_void,
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxBroadPhaseCallback {
    vtable_: *const std::ffi::c_void,
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxPostSolveCallback {
    vtable_: *const std::ffi::c_void,
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxScene {
    vtable_: *const std::ffi::c_void,
    pub userData: *mut std::ffi::c_void,
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxSimulationEventCallback {
    vtable_: *const std::ffi::c_void,
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxPruningStructure {
    vtable_: *const std::ffi::c_void,
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxObstacleContext {
    vtable_: *const std::ffi::c_void,
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxUserControllerHitReport {
    vtable_: *const std::ffi::c_void,
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxControllerFilterCallback {
    vtable_: *const std::ffi::c_void,
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxControllerDesc {
    vtable_: *const std::ffi::c_void,
    pub position: PxExtendedVec3,
    pub upDirection: PxVec3,
    pub slopeLimit: f32,
    pub invisibleWallHeight: f32,
    pub maxJumpHeight: f32,
    pub contactOffset: f32,
    pub stepOffset: f32,
    pub density: f32,
    pub scaleCoeff: f32,
    pub volumeGrowth: f32,
    pub reportCallback: *mut PxUserControllerHitReport,
    pub behaviorCallback: *mut PxControllerBehaviorCallback,
    pub nonWalkableMode: PxControllerNonWalkableMode,
    pub material: *mut PxMaterial,
    pub registerDeletionListener: bool,
    pub clientID: u8,
    pub userData: *mut std::ffi::c_void,
mType: PxControllerShapeType,
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxController {
    vtable_: *const std::ffi::c_void,
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxBoxControllerDesc {
    vtable_: *const std::ffi::c_void,
    pub halfHeight: f32,
    pub halfSideExtent: f32,
    pub halfForwardExtent: f32,
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxBoxController {
    vtable_: *const std::ffi::c_void,
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxCapsuleControllerDesc {
    vtable_: *const std::ffi::c_void,
    pub radius: f32,
    pub height: f32,
    pub climbingMode: PxCapsuleClimbingMode,
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxCapsuleController {
    vtable_: *const std::ffi::c_void,
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxControllerBehaviorCallback {
    vtable_: *const std::ffi::c_void,
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxControllerManager {
    vtable_: *const std::ffi::c_void,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct PxSDFBuilder {
    _unused: [u8; 0],
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxDefaultMemoryOutputStream {
    vtable_: *const std::ffi::c_void,
mAllocator: *mut PxAllocatorCallback,
mData: *mut u8,
mSize: u64,
mCapacity: u64,
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxDefaultMemoryInputData {
    vtable_: *const std::ffi::c_void,
mSize: u64,
mData: *const u8,
mPos: u64,
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxDefaultFileOutputStream {
    vtable_: *const std::ffi::c_void,
mFile: *mut std::ffi::c_void,
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxDefaultFileInputData {
    vtable_: *const std::ffi::c_void,
mFile: *mut std::ffi::c_void,
mLength: u64,
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxDefaultAllocator {
    vtable_: *const std::ffi::c_void,
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxJoint {
    vtable_: *const std::ffi::c_void,
    pub userData: *mut std::ffi::c_void,
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxDistanceJoint {
    vtable_: *const std::ffi::c_void,
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxFixedJoint {
    vtable_: *const std::ffi::c_void,
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxPrismaticJoint {
    vtable_: *const std::ffi::c_void,
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxRevoluteJoint {
    vtable_: *const std::ffi::c_void,
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxSphericalJoint {
    vtable_: *const std::ffi::c_void,
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxD6Joint {
    vtable_: *const std::ffi::c_void,
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxGearJoint {
    vtable_: *const std::ffi::c_void,
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxRackAndPinionJoint {
    vtable_: *const std::ffi::c_void,
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxDefaultErrorCallback {
    vtable_: *const std::ffi::c_void,
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxDefaultCpuDispatcher {
    vtable_: *const std::ffi::c_void,
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxBatchQueryExt {
    vtable_: *const std::ffi::c_void,
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxCustomSceneQuerySystem {
    vtable_: *const std::ffi::c_void,
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxCustomSceneQuerySystemAdapter {
    vtable_: *const std::ffi::c_void,
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxPoissonSampler {
    vtable_: *const std::ffi::c_void,
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxTriangleMeshPoissonSampler {
    vtable_: *const std::ffi::c_void,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XmlMemoryAllocator {
    _unused: [u8; 0],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XmlWriter {
    _unused: [u8; 0],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XmlReader {
    _unused: [u8; 0],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct MemoryBuffer {
    _unused: [u8; 0],
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxRepXSerializer {
    vtable_: *const std::ffi::c_void,
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxVehicleComponent {
    vtable_: *const std::ffi::c_void,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct PxVehiclePvdAttributeHandles {
    _unused: [u8; 0],
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxVehicleDirectDriveCommandResponseComponent {
    vtable_: *const std::ffi::c_void,
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxVehicleDirectDriveActuationStateComponent {
    vtable_: *const std::ffi::c_void,
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxVehicleDirectDrivetrainComponent {
    vtable_: *const std::ffi::c_void,
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxVehicleEngineDriveCommandResponseComponent {
    vtable_: *const std::ffi::c_void,
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxVehicleMultiWheelDriveDifferentialStateComponent {
    vtable_: *const std::ffi::c_void,
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxVehicleFourWheelDriveDifferentialStateComponent {
    vtable_: *const std::ffi::c_void,
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxVehicleTankDriveDifferentialStateComponent {
    vtable_: *const std::ffi::c_void,
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxVehicleLegacyFourWheelDriveDifferentialStateComponent {
    vtable_: *const std::ffi::c_void,
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxVehicleEngineDriveActuationStateComponent {
    vtable_: *const std::ffi::c_void,
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxVehicleEngineDrivetrainComponent {
    vtable_: *const std::ffi::c_void,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct PxTransform {
    _unused: [u8; 0],
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxVehiclePhysXActorBeginComponent {
    vtable_: *const std::ffi::c_void,
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxVehiclePhysXActorEndComponent {
    vtable_: *const std::ffi::c_void,
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxVehicleConstraintConnector {
    vtable_: *const std::ffi::c_void,
mVehicleConstraintState: *mut PxVehiclePhysXConstraintState,
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxVehiclePhysXConstraintComponent {
    vtable_: *const std::ffi::c_void,
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxVehiclePhysXRoadGeometrySceneQueryComponent {
    vtable_: *const std::ffi::c_void,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct PxVehiclePvdObjectHandles {
    _unused: [u8; 0],
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxVehiclePVDComponent {
    vtable_: *const std::ffi::c_void,
firstTime: bool,
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxVehicleRigidBodyComponent {
    vtable_: *const std::ffi::c_void,
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxVehicleSuspensionComponent {
    vtable_: *const std::ffi::c_void,
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxVehicleLegacySuspensionComponent {
    vtable_: *const std::ffi::c_void,
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxVehicleTireComponent {
    vtable_: *const std::ffi::c_void,
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxVehicleLegacyTireComponent {
    vtable_: *const std::ffi::c_void,
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxVehicleWheelComponent {
    vtable_: *const std::ffi::c_void,
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxPvd {
    vtable_: *const std::ffi::c_void,
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-structs", derive(Debug))]
#[repr(C)]
pub struct PxPvdTransport {
    vtable_: *const std::ffi::c_void,
}
extern "C" {
    pub fn PxAllocatorCallback_delete(self_: *mut PxAllocatorCallback);

    /// Allocates size bytes of memory, which must be 16-byte aligned.
    ///
    /// This method should never return NULL.  If you run out of memory, then
    /// you should terminate the app or take some other appropriate action.
    ///
    /// Threading:
    /// This function should be thread safe as it can be called in the context of the user thread
    /// and physics processing thread(s).
    ///
    /// The allocated block of memory.
    pub fn PxAllocatorCallback_allocate_mut(self_: *mut PxAllocatorCallback, size: usize, typeName: *const std::ffi::c_char, filename: *const std::ffi::c_char, line: i32) -> *mut std::ffi::c_void;

    /// Frees memory previously allocated by allocate().
    ///
    /// Threading:
    /// This function should be thread safe as it can be called in the context of the user thread
    /// and physics processing thread(s).
    pub fn PxAllocatorCallback_deallocate_mut(self_: *mut PxAllocatorCallback, ptr: *mut std::ffi::c_void);

    /// Built-in assert function
    pub fn phys_PxAssert(exp: *const std::ffi::c_char, file: *const std::ffi::c_char, line: i32, ignore: *mut bool);

    /// Destroys the instance it is called on.
    ///
    /// The operation will fail, if there are still modules referencing the foundation object. Release all dependent modules
    /// prior to calling this method.
    pub fn PxFoundation_release_mut(self_: *mut PxFoundation);

    /// retrieves error callback
    pub fn PxFoundation_getErrorCallback_mut(self_: *mut PxFoundation) -> *mut PxErrorCallback;

    /// Sets mask of errors to report.
    pub fn PxFoundation_setErrorLevel_mut(self_: *mut PxFoundation, mask: PxErrorCode);

    /// Retrieves mask of errors to be reported.
    pub fn PxFoundation_getErrorLevel(self_: *const PxFoundation) -> PxErrorCode;

    /// Retrieves the allocator this object was created with.
    pub fn PxFoundation_getAllocatorCallback_mut(self_: *mut PxFoundation) -> *mut PxAllocatorCallback;

    /// Retrieves if allocation names are being passed to allocator callback.
    pub fn PxFoundation_getReportAllocationNames(self_: *const PxFoundation) -> bool;

    /// Set if allocation names are being passed to allocator callback.
    ///
    /// Enabled by default in debug and checked build, disabled by default in profile and release build.
    pub fn PxFoundation_setReportAllocationNames_mut(self_: *mut PxFoundation, value: bool);

    pub fn PxFoundation_registerAllocationListener_mut(self_: *mut PxFoundation, listener: *mut PxAllocationListener);

    pub fn PxFoundation_deregisterAllocationListener_mut(self_: *mut PxFoundation, listener: *mut PxAllocationListener);

    pub fn PxFoundation_registerErrorCallback_mut(self_: *mut PxFoundation, callback: *mut PxErrorCallback);

    pub fn PxFoundation_deregisterErrorCallback_mut(self_: *mut PxFoundation, callback: *mut PxErrorCallback);

    pub fn PxFoundation_error_mut(self_: *mut PxFoundation, c: PxErrorCode, file: *const std::ffi::c_char, line: i32, messageFmt: *const std::ffi::c_char) -> bool;

    pub fn PxFoundation_error_mut_1(self_: *mut PxFoundation, anon_param0: PxErrorCode, file: *const std::ffi::c_char, line: i32, messageFmt: *const std::ffi::c_char, anon_param4: *mut std::ffi::c_char) -> bool;

    /// Creates an instance of the foundation class
    ///
    /// The foundation class is needed to initialize higher level SDKs. There may be only one instance per process.
    /// Calling this method after an instance has been created already will result in an error message and NULL will be
    /// returned.
    ///
    /// Foundation instance on success, NULL if operation failed
    pub fn phys_PxCreateFoundation(version: u32, allocator: *mut PxAllocatorCallback, errorCallback: *mut PxErrorCallback) -> *mut PxFoundation;

    pub fn phys_PxSetFoundationInstance(foundation: *mut PxFoundation);

    pub fn phys_PxGetFoundation() -> *mut PxFoundation;

    /// Similar to PxGetFoundation() except it handles the case if the foundation was not created already.
    ///
    /// Pointer to the foundation if an instance is currently available, otherwise null.
    pub fn phys_PxIsFoundationValid() -> *mut PxFoundation;

    /// Get the callback that will be used for all profiling.
    pub fn phys_PxGetProfilerCallback() -> *mut PxProfilerCallback;

    /// Set the callback that will be used for all profiling.
    pub fn phys_PxSetProfilerCallback(profiler: *mut PxProfilerCallback);

    /// Get the allocator callback
    pub fn phys_PxGetAllocatorCallback() -> *mut PxAllocatorCallback;

    /// Get the broadcasting allocator callback
    pub fn phys_PxGetBroadcastAllocator(reportAllocationNames: *mut bool) -> *mut PxAllocatorCallback;

    /// Get the error callback
    pub fn phys_PxGetErrorCallback() -> *mut PxErrorCallback;

    /// Get the broadcasting error callback
    pub fn phys_PxGetBroadcastError() -> *mut PxErrorCallback;

    /// Get the warn once timestamp
    pub fn phys_PxGetWarnOnceTimeStamp() -> u32;

    /// Decrement the ref count of PxFoundation
    pub fn phys_PxDecFoundationRefCount();

    /// Increment the ref count of PxFoundation
    pub fn phys_PxIncFoundationRefCount();

    /// read from the stream. The number of bytes read may be less than the number requested.
    ///
    /// the number of bytes read from the stream.
    pub fn PxInputStream_read_mut(self_: *mut PxInputStream, dest: *mut std::ffi::c_void, count: u64) -> u64;

    pub fn PxInputStream_delete(self_: *mut PxInputStream);

    /// return the length of the input data
    ///
    /// size in bytes of the input data
    pub fn PxInputData_getLength(self_: *const PxInputData) -> u64;

    /// seek to the given offset from the start of the data.
    pub fn PxInputData_seek_mut(self_: *mut PxInputData, offset: u64);

    /// return the current offset from the start of the data
    ///
    /// the offset to seek to.
    pub fn PxInputData_tell(self_: *const PxInputData) -> u64;

    pub fn PxInputData_delete(self_: *mut PxInputData);

    /// write to the stream. The number of bytes written may be less than the number sent.
    ///
    /// the number of bytes written to the stream by this call.
    pub fn PxOutputStream_write_mut(self_: *mut PxOutputStream, src: *const std::ffi::c_void, count: u64) -> u64;

    pub fn PxOutputStream_delete(self_: *mut PxOutputStream);

    pub fn PxAllocator_new(anon_param0: *const std::ffi::c_char) -> PxAllocator;

    pub fn PxAllocator_allocate(size: usize, file: *const std::ffi::c_char, line: i32, cookie: *mut u32) -> *mut std::ffi::c_void;

    pub fn PxAllocator_deallocate(ptr: *mut std::ffi::c_void, cookie: *mut u32);

    pub fn PxRawAllocator_new(anon_param0: *const std::ffi::c_char) -> PxRawAllocator;

    pub fn PxRawAllocator_allocate(size: usize, anon_param1: *const std::ffi::c_char, anon_param2: i32, cookie: *mut u32) -> *mut std::ffi::c_void;

    pub fn PxRawAllocator_deallocate(ptr: *mut std::ffi::c_void, cookie: *mut u32);

    pub fn PxTempAllocator_new(anon_param0: *const std::ffi::c_char) -> PxTempAllocator;

    pub fn PxTempAllocator_allocate_mut(self_: *mut PxTempAllocator, size: usize, file: *const std::ffi::c_char, line: i32) -> *mut std::ffi::c_void;

    pub fn PxTempAllocator_deallocate_mut(self_: *mut PxTempAllocator, ptr: *mut std::ffi::c_void);

    /// Sets the bytes of the provided buffer to zero.
    ///
    /// Pointer to memory block (same as input)
    pub fn phys_PxMemZero(dest: *mut std::ffi::c_void, count: usize) -> *mut std::ffi::c_void;

    /// Sets the bytes of the provided buffer to the specified value.
    ///
    /// Pointer to memory block (same as input)
    pub fn phys_PxMemSet(dest: *mut std::ffi::c_void, c: i32, count: usize) -> *mut std::ffi::c_void;

    /// Copies the bytes of one memory block to another. The memory blocks must not overlap.
    ///
    /// Use [`PxMemMove`] if memory blocks overlap.
    ///
    /// Pointer to destination memory block
    pub fn phys_PxMemCopy(dest: *mut std::ffi::c_void, src: *const std::ffi::c_void, count: usize) -> *mut std::ffi::c_void;

    /// Copies the bytes of one memory block to another. The memory blocks can overlap.
    ///
    /// Use [`PxMemCopy`] if memory blocks do not overlap.
    ///
    /// Pointer to destination memory block
    pub fn phys_PxMemMove(dest: *mut std::ffi::c_void, src: *const std::ffi::c_void, count: usize) -> *mut std::ffi::c_void;

    /// Mark a specified amount of memory with 0xcd pattern. This is used to check that the meta data
    /// definition for serialized classes is complete in checked builds.
    pub fn phys_PxMarkSerializedMemory(ptr: *mut std::ffi::c_void, byteSize: usize);

    pub fn phys_PxMemoryBarrier();

    /// Returns the index of the highest set bit. Not valid for zero arg.
    pub fn phys_PxHighestSetBitUnsafe(v: u64) -> u32;

    /// Returns the index of the highest set bit. Not valid for zero arg.
    pub fn phys_PxHighestSetBitUnsafe_1(v: u32) -> u32;

    /// Returns the index of the lowest set bit. Undefined for zero arg.
    pub fn phys_PxLowestSetBitUnsafe(v: u64) -> u32;

    /// Returns the index of the lowest set bit. Undefined for zero arg.
    pub fn phys_PxLowestSetBitUnsafe_1(v: u32) -> u32;

    /// Returns the number of leading zeros in v. Returns 32 for v=0.
    pub fn phys_PxCountLeadingZeros(v: u32) -> u32;

    pub fn phys_PxPrefetchLine(ptr: *const std::ffi::c_void, offset: u32);

    pub fn phys_PxPrefetch(ptr: *const std::ffi::c_void, count: u32);

    pub fn phys_PxBitCount(v: u32) -> u32;

    pub fn phys_PxIsPowerOfTwo(x: u32) -> bool;

    pub fn phys_PxNextPowerOfTwo(x: u32) -> u32;

    pub fn phys_PxNextPowerOfTwo_1(x: u64) -> u64;

    /// Return the index of the lowest set bit. Not valid for zero arg.
    pub fn phys_PxLowestSetBit(x: u32) -> u32;

    /// Return the index of the lowest set bit. Not valid for zero arg.
    pub fn phys_PxLowestSetBit_1(x: u64) -> u32;

    /// Return the index of the highest set bit. Not valid for zero arg.
    pub fn phys_PxHighestSetBit(x: u32) -> u32;

    /// Return the index of the highest set bit. Not valid for zero arg.
    pub fn phys_PxHighestSetBit_1(x: u64) -> u32;

    pub fn phys_PxILog2(num: u32) -> u32;

    pub fn PxVec3Padded_new_alloc() -> *mut PxVec3Padded;

    pub fn PxVec3Padded_delete(self_: *mut PxVec3Padded);

    pub fn PxVec3Padded_new_alloc_1(p: *const PxVec3) -> *mut PxVec3Padded;

    pub fn PxVec3Padded_new_alloc_2(f: f32) -> *mut PxVec3Padded;

    pub fn PxVec3Padded_new_alloc_3(_x: f32, _y: f32, _z: f32) -> *mut PxVec3Padded;

    pub fn PxTransformPadded_new() -> PxTransformPadded;

    pub fn PxTransformPadded_new_1(other: *const PxTransform) -> PxTransformPadded;

    pub fn PxTransformPadded_new_2(anon_param0: PxIDENTITY) -> PxTransformPadded;

    pub fn PxTransformPadded_new_3(position: *const PxVec3) -> PxTransformPadded;

    pub fn PxTransformPadded_new_4(orientation: *const PxQuat) -> PxTransformPadded;

    pub fn PxTransformPadded_new_5(p0: *const PxVec3, q0: *const PxQuat) -> PxTransformPadded;

    /// Sets a rotation matrix around the X axis.
    pub fn phys_PxSetRotX(m: *mut PxMat33, angle: f32);

    /// Sets a rotation matrix around the Y axis.
    pub fn phys_PxSetRotY(m: *mut PxMat33, angle: f32);

    /// Sets a rotation matrix around the Z axis.
    pub fn phys_PxSetRotZ(m: *mut PxMat33, angle: f32);

    /// Returns a rotation quaternion around the X axis.
    ///
    /// Quaternion that rotates around the desired axis
    pub fn phys_PxGetRotXQuat(angle: f32) -> PxQuat;

    /// Returns a rotation quaternion around the Y axis.
    ///
    /// Quaternion that rotates around the desired axis
    pub fn phys_PxGetRotYQuat(angle: f32) -> PxQuat;

    /// Returns a rotation quaternion around the Z axis.
    ///
    /// Quaternion that rotates around the desired axis
    pub fn phys_PxGetRotZQuat(angle: f32) -> PxQuat;

    /// Default constructor, not performing any initialization for performance reason.
    ///
    /// Use empty() function below to construct empty bounds.
    pub fn PxBounds3_new() -> PxBounds3;

    /// Construct from two bounding points
    pub fn PxBounds3_new_1(minimum: *const PxVec3, maximum: *const PxVec3) -> PxBounds3;

    /// Return empty bounds.
    pub fn PxBounds3_empty() -> PxBounds3;

    /// returns the AABB containing v0 and v1.
    pub fn PxBounds3_boundsOfPoints(v0: *const PxVec3, v1: *const PxVec3) -> PxBounds3;

    /// returns the AABB from center and extents vectors.
    pub fn PxBounds3_centerExtents(center: *const PxVec3, extent: *const PxVec3) -> PxBounds3;

    /// Construct from center, extent, and (not necessarily orthogonal) basis
    pub fn PxBounds3_basisExtent(center: *const PxVec3, basis: *const PxMat33, extent: *const PxVec3) -> PxBounds3;

    /// Construct from pose and extent
    pub fn PxBounds3_poseExtent(pose: *const PxTransform, extent: *const PxVec3) -> PxBounds3;

    /// gets the transformed bounds of the passed AABB (resulting in a bigger AABB).
    ///
    /// This version is safe to call for empty bounds.
    pub fn PxBounds3_transformSafe(matrix: *const PxMat33, bounds: *const PxBounds3) -> PxBounds3;

    /// gets the transformed bounds of the passed AABB (resulting in a bigger AABB).
    ///
    /// Calling this method for empty bounds leads to undefined behavior. Use [`transformSafe`]() instead.
    pub fn PxBounds3_transformFast(matrix: *const PxMat33, bounds: *const PxBounds3) -> PxBounds3;

    /// gets the transformed bounds of the passed AABB (resulting in a bigger AABB).
    ///
    /// This version is safe to call for empty bounds.
    pub fn PxBounds3_transformSafe_1(transform: *const PxTransform, bounds: *const PxBounds3) -> PxBounds3;

    /// gets the transformed bounds of the passed AABB (resulting in a bigger AABB).
    ///
    /// Calling this method for empty bounds leads to undefined behavior. Use [`transformSafe`]() instead.
    pub fn PxBounds3_transformFast_1(transform: *const PxTransform, bounds: *const PxBounds3) -> PxBounds3;

    /// Sets empty to true
    pub fn PxBounds3_setEmpty_mut(self_: *mut PxBounds3);

    /// Sets the bounds to maximum size [-PX_MAX_BOUNDS_EXTENTS, PX_MAX_BOUNDS_EXTENTS].
    pub fn PxBounds3_setMaximal_mut(self_: *mut PxBounds3);

    /// expands the volume to include v
    pub fn PxBounds3_include_mut(self_: *mut PxBounds3, v: *const PxVec3);

    /// expands the volume to include b.
    pub fn PxBounds3_include_mut_1(self_: *mut PxBounds3, b: *const PxBounds3);

    pub fn PxBounds3_isEmpty(self_: *const PxBounds3) -> bool;

    /// indicates whether the intersection of this and b is empty or not.
    pub fn PxBounds3_intersects(self_: *const PxBounds3, b: *const PxBounds3) -> bool;

    /// computes the 1D-intersection between two AABBs, on a given axis.
    pub fn PxBounds3_intersects1D(self_: *const PxBounds3, a: *const PxBounds3, axis: u32) -> bool;

    /// indicates if these bounds contain v.
    pub fn PxBounds3_contains(self_: *const PxBounds3, v: *const PxVec3) -> bool;

    /// checks a box is inside another box.
    pub fn PxBounds3_isInside(self_: *const PxBounds3, box_: *const PxBounds3) -> bool;

    /// returns the center of this axis aligned box.
    pub fn PxBounds3_getCenter(self_: *const PxBounds3) -> PxVec3;

    /// get component of the box's center along a given axis
    pub fn PxBounds3_getCenter_1(self_: *const PxBounds3, axis: u32) -> f32;

    /// get component of the box's extents along a given axis
    pub fn PxBounds3_getExtents(self_: *const PxBounds3, axis: u32) -> f32;

    /// returns the dimensions (width/height/depth) of this axis aligned box.
    pub fn PxBounds3_getDimensions(self_: *const PxBounds3) -> PxVec3;

    /// returns the extents, which are half of the width/height/depth.
    pub fn PxBounds3_getExtents_1(self_: *const PxBounds3) -> PxVec3;

    /// scales the AABB.
    ///
    /// This version is safe to call for empty bounds.
    pub fn PxBounds3_scaleSafe_mut(self_: *mut PxBounds3, scale: f32);

    /// scales the AABB.
    ///
    /// Calling this method for empty bounds leads to undefined behavior. Use [`scaleSafe`]() instead.
    pub fn PxBounds3_scaleFast_mut(self_: *mut PxBounds3, scale: f32);

    /// fattens the AABB in all 3 dimensions by the given distance.
    ///
    /// This version is safe to call for empty bounds.
    pub fn PxBounds3_fattenSafe_mut(self_: *mut PxBounds3, distance: f32);

    /// fattens the AABB in all 3 dimensions by the given distance.
    ///
    /// Calling this method for empty bounds leads to undefined behavior. Use [`fattenSafe`]() instead.
    pub fn PxBounds3_fattenFast_mut(self_: *mut PxBounds3, distance: f32);

    /// checks that the AABB values are not NaN
    pub fn PxBounds3_isFinite(self_: *const PxBounds3) -> bool;

    /// checks that the AABB values describe a valid configuration.
    pub fn PxBounds3_isValid(self_: *const PxBounds3) -> bool;

    /// Finds the closest point in the box to the point p. If p is contained, this will be p, otherwise it
    /// will be the closest point on the surface of the box.
    pub fn PxBounds3_closestPoint(self_: *const PxBounds3, p: *const PxVec3) -> PxVec3;

    pub fn PxErrorCallback_delete(self_: *mut PxErrorCallback);

    /// Reports an error code.
    pub fn PxErrorCallback_reportError_mut(self_: *mut PxErrorCallback, code: PxErrorCode, message: *const std::ffi::c_char, file: *const std::ffi::c_char, line: i32);

    /// callback when memory is allocated.
    pub fn PxAllocationListener_onAllocation_mut(self_: *mut PxAllocationListener, size: usize, typeName: *const std::ffi::c_char, filename: *const std::ffi::c_char, line: i32, allocatedMemory: *mut std::ffi::c_void);

    /// callback when memory is deallocated.
    pub fn PxAllocationListener_onDeallocation_mut(self_: *mut PxAllocationListener, allocatedMemory: *mut std::ffi::c_void);

    /// The default constructor.
    pub fn PxBroadcastingAllocator_new_alloc(allocator: *mut PxAllocatorCallback, error: *mut PxErrorCallback) -> *mut PxBroadcastingAllocator;

    /// The default constructor.
    pub fn PxBroadcastingAllocator_delete(self_: *mut PxBroadcastingAllocator);

    /// Allocates size bytes of memory, which must be 16-byte aligned.
    ///
    /// This method should never return NULL.  If you run out of memory, then
    /// you should terminate the app or take some other appropriate action.
    ///
    /// Threading:
    /// This function should be thread safe as it can be called in the context of the user thread
    /// and physics processing thread(s).
    ///
    /// The allocated block of memory.
    pub fn PxBroadcastingAllocator_allocate_mut(self_: *mut PxBroadcastingAllocator, size: usize, typeName: *const std::ffi::c_char, filename: *const std::ffi::c_char, line: i32) -> *mut std::ffi::c_void;

    /// Frees memory previously allocated by allocate().
    ///
    /// Threading:
    /// This function should be thread safe as it can be called in the context of the user thread
    /// and physics processing thread(s).
    pub fn PxBroadcastingAllocator_deallocate_mut(self_: *mut PxBroadcastingAllocator, ptr: *mut std::ffi::c_void);

    /// The default constructor.
    pub fn PxBroadcastingErrorCallback_new_alloc(errorCallback: *mut PxErrorCallback) -> *mut PxBroadcastingErrorCallback;

    /// The default destructor.
    pub fn PxBroadcastingErrorCallback_delete(self_: *mut PxBroadcastingErrorCallback);

    /// Reports an error code.
    pub fn PxBroadcastingErrorCallback_reportError_mut(self_: *mut PxBroadcastingErrorCallback, code: PxErrorCode, message: *const std::ffi::c_char, file: *const std::ffi::c_char, line: i32);

    pub fn PxFPUGuard_new_alloc() -> *mut PxFPUGuard;

    pub fn PxFPUGuard_delete(self_: *mut PxFPUGuard);

    pub fn PxSIMDGuard_new_alloc(enable: bool) -> *mut PxSIMDGuard;

    pub fn PxSIMDGuard_delete(self_: *mut PxSIMDGuard);

    /// Enables floating point exceptions for the scalar and SIMD unit
    pub fn phys_PxEnableFPExceptions();

    /// Disables floating point exceptions for the scalar and SIMD unit
    pub fn phys_PxDisableFPExceptions();

    /// Constructor
    pub fn PxPlane_new() -> PxPlane;

    /// Constructor from a normal and a distance
    pub fn PxPlane_new_1(nx: f32, ny: f32, nz: f32, distance: f32) -> PxPlane;

    /// Constructor from a normal and a distance
    pub fn PxPlane_new_2(normal: *const PxVec3, distance: f32) -> PxPlane;

    /// Constructor from a point on the plane and a normal
    pub fn PxPlane_new_3(point: *const PxVec3, normal: *const PxVec3) -> PxPlane;

    /// Constructor from three points
    pub fn PxPlane_new_4(p0: *const PxVec3, p1: *const PxVec3, p2: *const PxVec3) -> PxPlane;

    pub fn PxPlane_distance(self_: *const PxPlane, p: *const PxVec3) -> f32;

    pub fn PxPlane_contains(self_: *const PxPlane, p: *const PxVec3) -> bool;

    /// projects p into the plane
    pub fn PxPlane_project(self_: *const PxPlane, p: *const PxVec3) -> PxVec3;

    /// find an arbitrary point in the plane
    pub fn PxPlane_pointInPlane(self_: *const PxPlane) -> PxVec3;

    /// equivalent plane with unit normal
    pub fn PxPlane_normalize_mut(self_: *mut PxPlane);

    /// transform plane
    pub fn PxPlane_transform(self_: *const PxPlane, pose: *const PxTransform) -> PxPlane;

    /// inverse-transform plane
    pub fn PxPlane_inverseTransform(self_: *const PxPlane, pose: *const PxTransform) -> PxPlane;

    /// finds the shortest rotation between two vectors.
    ///
    /// a rotation about an axis normal to the two vectors which takes one to the other via the shortest path
    pub fn phys_PxShortestRotation(from: *const PxVec3, target: *const PxVec3) -> PxQuat;

    pub fn phys_PxDiagonalize(m: *const PxMat33, axes: *mut PxQuat) -> PxVec3;

    /// creates a transform from the endpoints of a segment, suitable for an actor transform for a PxCapsuleGeometry
    ///
    /// A PxTransform which will transform the vector (1,0,0) to the capsule axis shrunk by the halfHeight
    pub fn phys_PxTransformFromSegment(p0: *const PxVec3, p1: *const PxVec3, halfHeight: *mut f32) -> PxTransform;

    /// creates a transform from a plane equation, suitable for an actor transform for a PxPlaneGeometry
    ///
    /// a PxTransform which will transform the plane PxPlane(1,0,0,0) to the specified plane
    pub fn phys_PxTransformFromPlaneEquation(plane: *const PxPlane) -> PxTransform;

    /// creates a plane equation from a transform, such as the actor transform for a PxPlaneGeometry
    ///
    /// the plane
    pub fn phys_PxPlaneEquationFromTransform(pose: *const PxTransform) -> PxPlane;

    /// Spherical linear interpolation of two quaternions.
    ///
    /// Returns left when t=0, right when t=1 and a linear interpolation of left and right when 0
    /// <
    /// t
    /// <
    /// 1.
    /// Returns angle between -PI and PI in radians
    pub fn phys_PxSlerp(t: f32, left: *const PxQuat, right: *const PxQuat) -> PxQuat;

    /// integrate transform.
    pub fn phys_PxIntegrateTransform(curTrans: *const PxTransform, linvel: *const PxVec3, angvel: *const PxVec3, timeStep: f32, result: *mut PxTransform);

    /// Compute the exponent of a PxVec3
    pub fn phys_PxExp(v: *const PxVec3) -> PxQuat;

    /// computes a oriented bounding box around the scaled basis.
    ///
    /// Bounding box extent.
    pub fn phys_PxOptimizeBoundingBox(basis: *mut PxMat33) -> PxVec3;

    /// return Returns the log of a PxQuat
    pub fn phys_PxLog(q: *const PxQuat) -> PxVec3;

    /// return Returns 0 if v.x is largest element of v, 1 if v.y is largest element, 2 if v.z is largest element.
    pub fn phys_PxLargestAxis(v: *const PxVec3) -> u32;

    /// Compute tan(theta/2) given sin(theta) and cos(theta) as inputs.
    ///
    /// Returns tan(theta/2)
    pub fn phys_PxTanHalf(sin: f32, cos: f32) -> f32;

    /// Compute the closest point on an 2d ellipse to a given 2d point.
    ///
    /// Returns the 2d position on the surface of the ellipse that is closest to point.
    pub fn phys_PxEllipseClamp(point: *const PxVec3, radii: *const PxVec3) -> PxVec3;

    /// Compute from an input quaternion q a pair of quaternions (swing, twist) such that
    /// q = swing * twist
    /// with the caveats that swing.x = twist.y = twist.z = 0.
    pub fn phys_PxSeparateSwingTwist(q: *const PxQuat, swing: *mut PxQuat, twist: *mut PxQuat);

    /// Compute the angle between two non-unit vectors
    ///
    /// Returns the angle (in radians) between the two vector v0 and v1.
    pub fn phys_PxComputeAngle(v0: *const PxVec3, v1: *const PxVec3) -> f32;

    /// Compute two normalized vectors (right and up) that are perpendicular to an input normalized vector (dir).
    pub fn phys_PxComputeBasisVectors(dir: *const PxVec3, right: *mut PxVec3, up: *mut PxVec3);

    /// Compute three normalized vectors (dir, right and up) that are parallel to (dir) and perpendicular to (right, up) the
    /// normalized direction vector (p1 - p0)/||p1 - p0||.
    pub fn phys_PxComputeBasisVectors_1(p0: *const PxVec3, p1: *const PxVec3, dir: *mut PxVec3, right: *mut PxVec3, up: *mut PxVec3);

    /// Compute (i+1)%3
    pub fn phys_PxGetNextIndex3(i: u32) -> u32;

    /// Computes the barycentric coordinates for a point inside a tetrahedron.
    ///
    /// This function calculates the barycentric coordinates of a point p with respect to a tetrahedron defined by vertices a, b, c, and d.
    pub fn phys_PxComputeBarycentric(a: *const PxVec3, b: *const PxVec3, c: *const PxVec3, d: *const PxVec3, p: *const PxVec3, bary: *mut PxVec4);

    /// Computes the barycentric coordinates for a point inside a triangle.
    ///
    /// This function calculates the barycentric coordinates of a point p with respect to a triangle defined by vertices a, b, and c.
    pub fn phys_PxComputeBarycentric_1(a: *const PxVec3, b: *const PxVec3, c: *const PxVec3, p: *const PxVec3, bary: *mut PxVec4);

    /// Computes the barycentric coordinates for a point inside a triangle (deprecated).
    ///
    /// This function is deprecated. Use PxComputeBarycentric instead.
    pub fn phys_computeBarycentric(a: *const PxVec3, b: *const PxVec3, c: *const PxVec3, p: *const PxVec3, bary: *mut PxVec4);

    /// Computes the barycentric coordinates for a point inside a tetrahedron (deprecated).
    ///
    /// This function is deprecated. Use PxComputeBarycentric instead.
    pub fn phys_computeBarycentric_1(a: *const PxVec3, b: *const PxVec3, c: *const PxVec3, d: *const PxVec3, p: *const PxVec3, bary: *mut PxVec4);

    /// Performs linear interpolation between two values.
    ///
    /// The interpolated value
    pub fn phys_PxLerp(a: f32, b: f32, t: f32) -> f32;

    /// Performs bilinear interpolation.
    ///
    /// The interpolated value
    pub fn phys_PxBiLerp(f00: f32, f10: f32, f01: f32, f11: f32, tx: f32, ty: f32) -> f32;

    /// Performs trilinear interpolation.
    ///
    /// The interpolated value
    pub fn phys_PxTriLerp(f000: f32, f100: f32, f010: f32, f110: f32, f001: f32, f101: f32, f011: f32, f111: f32, tx: f32, ty: f32, tz: f32) -> f32;

    /// Computes the 1D index for a 3D grid point.
    ///
    /// The 1D index corresponding to the 3D grid point
    pub fn phys_PxSDFIdx(i: u32, j: u32, k: u32, nbX: u32, nbY: u32) -> u32;

    /// Samples the signed distance field (SDF) at a given local position.
    ///
    /// This function samples the SDF at a given local position within the defined box bounds and calculates the interpolated distance value. It handles grid clamping and ensures that the sampled value is within the tolerance limit.
    ///
    /// The sampled SDF value
    pub fn phys_PxSDFSample(sdf: *const f32, localPos: *const PxVec3, sdfBoxLower: *const PxVec3, sdfBoxHigher: *const PxVec3, sdfDx: f32, invSdfDx: f32, dimX: u32, dimY: u32, dimZ: u32, tolerance: f32) -> f32;

    /// Performs linear interpolation between two values.
    ///
    /// The interpolated value
    ///
    /// Please use corresponding freestanding function outside of Interpolation scope.
    pub fn Interpolation_PxLerp(a: f32, b: f32, t: f32) -> f32;

    /// Performs bilinear interpolation.
    ///
    /// The interpolated value
    ///
    /// Please use corresponding freestanding function outside of Interpolation scope.
    pub fn Interpolation_PxBiLerp(f00: f32, f10: f32, f01: f32, f11: f32, tx: f32, ty: f32) -> f32;

    /// Performs trilinear interpolation.
    ///
    /// The interpolated value
    ///
    /// Please use corresponding freestanding function outside of Interpolation scope.
    pub fn Interpolation_PxTriLerp(f000: f32, f100: f32, f010: f32, f110: f32, f001: f32, f101: f32, f011: f32, f111: f32, tx: f32, ty: f32, tz: f32) -> f32;

    /// Computes the 1D index for a 3D grid point.
    ///
    /// The 1D index corresponding to the 3D grid point
    ///
    /// Please use corresponding freestanding function outside of Interpolation scope.
    pub fn Interpolation_PxSDFIdx(i: u32, j: u32, k: u32, nbX: u32, nbY: u32) -> u32;

    /// Samples the signed distance field (SDF) at a given local position.
    ///
    /// This function samples the SDF at a given local position within the defined box bounds and calculates the interpolated distance value. It handles grid clamping and ensures that the sampled value is within the tolerance limit.
    ///
    /// The sampled SDF value
    ///
    /// Please use corresponding freestanding function outside of Interpolation scope.
    pub fn Interpolation_PxSDFSampleImpl(sdf: *const f32, localPos: *const PxVec3, sdfBoxLower: *const PxVec3, sdfBoxHigher: *const PxVec3, sdfDx: f32, invSdfDx: f32, dimX: u32, dimY: u32, dimZ: u32, tolerance: f32) -> f32;

    /// Samples the signed distance field (SDF) at a given local position with gradient computation (deprecated).
    ///
    /// The sampled SDF value
    ///
    /// Please use PxSDFSample.
    pub fn phys_PxSdfSample(sdf: *const f32, localPos: *const PxVec3, sdfBoxLower: *const PxVec3, sdfBoxHigher: *const PxVec3, sdfDx: f32, invSdfDx: f32, dimX: u32, dimY: u32, dimZ: u32, gradient: *mut PxVec3, tolerance: f32) -> f32;

    /// The constructor for Mutex creates a mutex. It is initially unlocked.
    pub fn PxMutexImpl_new_alloc() -> *mut PxMutexImpl;

    /// The destructor for Mutex deletes the mutex.
    pub fn PxMutexImpl_delete(self_: *mut PxMutexImpl);

    /// Acquire (lock) the mutex. If the mutex is already locked
    /// by another thread, this method blocks until the mutex is
    /// unlocked.
    pub fn PxMutexImpl_lock_mut(self_: *mut PxMutexImpl);

    /// Acquire (lock) the mutex. If the mutex is already locked
    /// by another thread, this method returns false without blocking.
    pub fn PxMutexImpl_trylock_mut(self_: *mut PxMutexImpl) -> bool;

    /// Release (unlock) the mutex.
    pub fn PxMutexImpl_unlock_mut(self_: *mut PxMutexImpl);

    /// Size of this class.
    pub fn PxMutexImpl_getSize() -> u32;

    pub fn PxReadWriteLock_new_alloc() -> *mut PxReadWriteLock;

    pub fn PxReadWriteLock_delete(self_: *mut PxReadWriteLock);

    pub fn PxReadWriteLock_lockReader_mut(self_: *mut PxReadWriteLock, takeLock: bool);

    pub fn PxReadWriteLock_lockWriter_mut(self_: *mut PxReadWriteLock);

    pub fn PxReadWriteLock_unlockReader_mut(self_: *mut PxReadWriteLock);

    pub fn PxReadWriteLock_unlockWriter_mut(self_: *mut PxReadWriteLock);

    /// Mark the beginning of a nested profile block
    ///
    /// Returns implementation-specific profiler data for this event
    pub fn PxProfilerCallback_zoneStart_mut(self_: *mut PxProfilerCallback, eventName: *const std::ffi::c_char, detached: bool, contextId: u64) -> *mut std::ffi::c_void;

    /// Mark the end of a nested profile block
    ///
    /// eventName plus contextId can be used to uniquely match up start and end of a zone.
    pub fn PxProfilerCallback_zoneEnd_mut(self_: *mut PxProfilerCallback, profilerData: *mut std::ffi::c_void, eventName: *const std::ffi::c_char, detached: bool, contextId: u64);

    /// Record integer data to be displayed in the profiler.
    pub fn PxProfilerCallback_recordData_mut(self_: *mut PxProfilerCallback, value: i32, valueName: *const std::ffi::c_char, contextId: u64);

    /// Record float data to be displayed in the profiler.
    pub fn PxProfilerCallback_recordData_mut_1(self_: *mut PxProfilerCallback, value: f32, valueName: *const std::ffi::c_char, contextId: u64);

    /// Record a frame marker to be displayed in the profiler.
    ///
    /// Markers that have identical names will be displayed in the profiler
    /// along with the time between each of the markers. A frame counter will display the frame marker count.
    pub fn PxProfilerCallback_recordFrame_mut(self_: *mut PxProfilerCallback, name: *const std::ffi::c_char, contextId: u64);

    pub fn PxProfileScoped_new_alloc(callback: *mut PxProfilerCallback, eventName: *const std::ffi::c_char, detached: bool, contextId: u64) -> *mut PxProfileScoped;

    pub fn PxProfileScoped_delete(self_: *mut PxProfileScoped);

    pub fn PxSListEntry_new() -> PxSListEntry;

    pub fn PxSListEntry_next_mut(self_: *mut PxSListEntry) -> *mut PxSListEntry;

    pub fn PxSListImpl_new_alloc() -> *mut PxSListImpl;

    pub fn PxSListImpl_delete(self_: *mut PxSListImpl);

    pub fn PxSListImpl_push_mut(self_: *mut PxSListImpl, entry: *mut PxSListEntry);

    pub fn PxSListImpl_pop_mut(self_: *mut PxSListImpl) -> *mut PxSListEntry;

    pub fn PxSListImpl_flush_mut(self_: *mut PxSListImpl) -> *mut PxSListEntry;

    pub fn PxSListImpl_getSize() -> u32;

    pub fn PxSocket_delete(self_: *mut PxSocket);

    /// Opens a network socket for input and/or output
    ///
    /// True if the connection was successful, false otherwise
    pub fn PxSocket_connect_mut(self_: *mut PxSocket, host: *const std::ffi::c_char, port: u16, timeout: u32) -> bool;

    /// Opens a network socket for input and/or output as a server.  Put the connection in listening mode
    pub fn PxSocket_listen_mut(self_: *mut PxSocket, port: u16) -> bool;

    /// Accept a connection on a socket that is in listening mode
    ///
    /// This method only supports a single connection client.  Additional clients
    /// that connect to the listening port will overwrite the existing socket handle.
    ///
    /// whether a connection was established
    pub fn PxSocket_accept_mut(self_: *mut PxSocket, block: bool) -> bool;

    /// Disconnects an open socket
    pub fn PxSocket_disconnect_mut(self_: *mut PxSocket);

    /// Returns whether the socket is currently open (connected) or not.
    ///
    /// True if the socket is connected, false otherwise
    pub fn PxSocket_isConnected(self_: *const PxSocket) -> bool;

    /// Returns the name of the connected host. This is the same as the string
    /// that was supplied to the connect call.
    ///
    /// The name of the connected host
    pub fn PxSocket_getHost(self_: *const PxSocket) -> *const std::ffi::c_char;

    /// Returns the port of the connected host. This is the same as the port
    /// that was supplied to the connect call.
    ///
    /// The port of the connected host
    pub fn PxSocket_getPort(self_: *const PxSocket) -> u16;

    /// Flushes the output stream. Until the stream is flushed, there is no
    /// guarantee that the written data has actually reached the destination
    /// storage. Flush forces all buffered data to be sent to the output.
    ///
    /// flush always blocks. If the socket is in non-blocking mode, this will result
    /// the thread spinning.
    ///
    /// True if the flush was successful, false otherwise
    pub fn PxSocket_flush_mut(self_: *mut PxSocket) -> bool;

    /// Writes data to the output stream.
    ///
    /// Number of bytes actually written. This could be lower than length if the socket is non-blocking.
    pub fn PxSocket_write_mut(self_: *mut PxSocket, data: *const u8, length: u32) -> u32;

    /// Reads data from the output stream.
    ///
    /// Number of bytes actually read. This could be lower than length if the stream end is
    /// encountered or the socket is non-blocking.
    pub fn PxSocket_read_mut(self_: *mut PxSocket, data: *mut u8, length: u32) -> u32;

    /// Sets blocking mode of the socket.
    /// Socket must be connected, otherwise calling this method won't take any effect.
    pub fn PxSocket_setBlocking_mut(self_: *mut PxSocket, blocking: bool);

    /// Returns whether read/write/flush calls to the socket are blocking.
    ///
    /// True if the socket is blocking.
    pub fn PxSocket_isBlocking(self_: *const PxSocket) -> bool;

    pub fn PxSyncImpl_new_alloc() -> *mut PxSyncImpl;

    pub fn PxSyncImpl_delete(self_: *mut PxSyncImpl);

    /// Wait on the object for at most the given number of ms. Returns
    /// true if the object is signaled. Sync::waitForever will block forever
    /// or until the object is signaled.
    pub fn PxSyncImpl_wait_mut(self_: *mut PxSyncImpl, milliseconds: u32) -> bool;

    /// Signal the synchronization object, waking all threads waiting on it
    pub fn PxSyncImpl_set_mut(self_: *mut PxSyncImpl);

    /// Reset the synchronization object
    pub fn PxSyncImpl_reset_mut(self_: *mut PxSyncImpl);

    /// Size of this class.
    pub fn PxSyncImpl_getSize() -> u32;

    pub fn PxRunnable_new_alloc() -> *mut PxRunnable;

    pub fn PxRunnable_delete(self_: *mut PxRunnable);

    pub fn PxRunnable_execute_mut(self_: *mut PxRunnable);

    pub fn PxThreadImpl_getDefaultStackSize() -> u32;

    pub fn PxThreadImpl_getId() -> usize;

    /// Construct (but do not start) the thread object. The OS thread object will not be created
    /// until start() is called. Executes in the context
    /// of the spawning thread.
    pub fn PxThreadImpl_new_alloc() -> *mut PxThreadImpl;

    /// Deallocate all resources associated with the thread. Should be called in the
    /// context of the spawning thread.
    pub fn PxThreadImpl_delete(self_: *mut PxThreadImpl);

    /// Create the OS thread and start it running. Called in the context of the spawning thread.
    /// If an affinity mask has previously been set then it will be applied after the
    /// thread has been created.
    pub fn PxThreadImpl_start_mut(self_: *mut PxThreadImpl, stackSize: u32, r: *mut PxRunnable);

    /// Violently kill the current thread. Blunt instrument, not recommended since
    /// it can leave all kinds of things unreleased (stack, memory, mutexes...) Should
    /// be called in the context of the spawning thread.
    pub fn PxThreadImpl_kill_mut(self_: *mut PxThreadImpl);

    /// Stop the thread. Signals the spawned thread that it should stop, so the
    /// thread should check regularly
    pub fn PxThreadImpl_signalQuit_mut(self_: *mut PxThreadImpl);

    /// Wait for a thread to stop. Should be called in the context of the spawning
    /// thread. Returns false if the thread has not been started.
    pub fn PxThreadImpl_waitForQuit_mut(self_: *mut PxThreadImpl) -> bool;

    /// check whether the thread is signalled to quit. Called in the context of the
    /// spawned thread.
    pub fn PxThreadImpl_quitIsSignalled_mut(self_: *mut PxThreadImpl) -> bool;

    /// Cleanly shut down this thread. Called in the context of the spawned thread.
    pub fn PxThreadImpl_quit_mut(self_: *mut PxThreadImpl);

    /// Change the affinity mask for this thread. The mask is a platform
    /// specific value.
    ///
    /// On Windows, Linux, and Switch platforms, each set mask bit represents
    /// the index of a logical processor that the OS may schedule thread execution on.
    /// Bits outside the range of valid logical processors may be ignored or cause
    /// the function to return an error.
    ///
    /// On Apple platforms, this function has no effect.
    ///
    /// If the thread has not yet been started then the mask is stored
    /// and applied when the thread is started.
    ///
    /// If the thread has already been started then this method	returns the
    /// previous affinity mask on success, otherwise it returns zero.
    pub fn PxThreadImpl_setAffinityMask_mut(self_: *mut PxThreadImpl, mask: u32) -> u32;

    pub fn PxThreadImpl_getPriority(threadId: usize) -> PxThreadPriority;

    /// Set thread priority.
    pub fn PxThreadImpl_setPriority_mut(self_: *mut PxThreadImpl, prio: PxThreadPriority);

    /// set the thread's name
    pub fn PxThreadImpl_setName_mut(self_: *mut PxThreadImpl, name: *const std::ffi::c_char);

    /// Put the current thread to sleep for the given number of milliseconds
    pub fn PxThreadImpl_sleep(ms: u32);

    /// Yield the current thread's slot on the CPU
    pub fn PxThreadImpl_yield();

    /// Inform the processor that we're in a busy wait to give it a chance to do something clever.
    /// yield() yields the thread, while yieldProcessor() aims to yield the processor
    pub fn PxThreadImpl_yieldProcessor();

    /// Return the number of physical cores (does not include hyper-threaded cores), returns 0 on failure
    pub fn PxThreadImpl_getNbPhysicalCores() -> u32;

    /// Size of this class.
    pub fn PxThreadImpl_getSize() -> u32;

    pub fn phys_PxTlsAlloc() -> u32;

    pub fn phys_PxTlsFree(index: u32);

    pub fn phys_PxTlsGet(index: u32) -> *mut std::ffi::c_void;

    pub fn phys_PxTlsGetValue(index: u32) -> usize;

    pub fn phys_PxTlsSet(index: u32, value: *mut std::ffi::c_void) -> u32;

    pub fn phys_PxTlsSetValue(index: u32, value: usize) -> u32;

    pub fn PxCounterFrequencyToTensOfNanos_new(inNum: u64, inDenom: u64) -> PxCounterFrequencyToTensOfNanos;

    pub fn PxCounterFrequencyToTensOfNanos_toTensOfNanos(self_: *const PxCounterFrequencyToTensOfNanos, inCounter: u64) -> u64;

    pub fn PxTime_getBootCounterFrequency() -> *const PxCounterFrequencyToTensOfNanos;

    pub fn PxTime_getCounterFrequency() -> PxCounterFrequencyToTensOfNanos;

    pub fn PxTime_getCurrentCounterValue() -> u64;

    pub fn PxTime_getCurrentTimeInTensOfNanoSeconds() -> u64;

    pub fn PxTime_new() -> PxTime;

    pub fn PxTime_getElapsedSeconds_mut(self_: *mut PxTime) -> f64;

    pub fn PxTime_peekElapsedSeconds_mut(self_: *mut PxTime) -> f64;

    pub fn PxTime_getLastTime(self_: *const PxTime) -> f64;

    pub fn PxBoundedData_new() -> PxBoundedData;

    pub fn PxBoundedData_new_1(data_: *mut std::ffi::c_void, stride_: u32, count_: u32) -> PxBoundedData;

    pub fn PxDebugPoint_new(p: *const PxVec3, c: *const u32) -> PxDebugPoint;

    pub fn PxDebugLine_new(p0: *const PxVec3, p1: *const PxVec3, c: *const u32) -> PxDebugLine;

    pub fn PxDebugTriangle_new(p0: *const PxVec3, p1: *const PxVec3, p2: *const PxVec3, c: *const u32) -> PxDebugTriangle;

    pub fn PxDebugText_new() -> PxDebugText;

    pub fn PxDebugText_new_1(pos: *const PxVec3, sz: *const f32, clr: *const u32, str: *const std::ffi::c_char) -> PxDebugText;

    pub fn PxRenderBuffer_delete(self_: *mut PxRenderBuffer);

    pub fn PxRenderBuffer_getNbPoints(self_: *const PxRenderBuffer) -> u32;

    pub fn PxRenderBuffer_getPoints(self_: *const PxRenderBuffer) -> *const PxDebugPoint;

    pub fn PxRenderBuffer_addPoint_mut(self_: *mut PxRenderBuffer, point: *const PxDebugPoint);

    pub fn PxRenderBuffer_getNbLines(self_: *const PxRenderBuffer) -> u32;

    pub fn PxRenderBuffer_getLines(self_: *const PxRenderBuffer) -> *const PxDebugLine;

    pub fn PxRenderBuffer_addLine_mut(self_: *mut PxRenderBuffer, line: *const PxDebugLine);

    pub fn PxRenderBuffer_reserveLines_mut(self_: *mut PxRenderBuffer, nbLines: u32) -> *mut PxDebugLine;

    pub fn PxRenderBuffer_reservePoints_mut(self_: *mut PxRenderBuffer, nbLines: u32) -> *mut PxDebugPoint;

    pub fn PxRenderBuffer_getNbTriangles(self_: *const PxRenderBuffer) -> u32;

    pub fn PxRenderBuffer_getTriangles(self_: *const PxRenderBuffer) -> *const PxDebugTriangle;

    pub fn PxRenderBuffer_addTriangle_mut(self_: *mut PxRenderBuffer, triangle: *const PxDebugTriangle);

    pub fn PxRenderBuffer_append_mut(self_: *mut PxRenderBuffer, other: *const PxRenderBuffer);

    pub fn PxRenderBuffer_clear_mut(self_: *mut PxRenderBuffer);

    pub fn PxRenderBuffer_shift_mut(self_: *mut PxRenderBuffer, delta: *const PxVec3);

    pub fn PxRenderBuffer_empty(self_: *const PxRenderBuffer) -> bool;

    pub fn PxProcessPxBaseCallback_delete(self_: *mut PxProcessPxBaseCallback);

    pub fn PxProcessPxBaseCallback_process_mut(self_: *mut PxProcessPxBaseCallback, anon_param0: *mut PxBase);

    /// Registers a reference value corresponding to a PxBase object.
    ///
    /// This method is assumed to be called in the implementation of PxSerializer::registerReferences for serialized
    /// references that need to be resolved on deserialization.
    ///
    /// A reference needs to be associated with exactly one PxBase object in either the collection or the
    /// external references collection.
    ///
    /// Different kinds of references are supported and need to be specified. In the most common case
    /// (PX_SERIAL_REF_KIND_PXBASE) the PxBase object matches the reference value (which is the pointer
    /// to the PxBase object). Integer references maybe registered as well (used for internal material
    /// indices with PX_SERIAL_REF_KIND_MATERIAL_IDX). Other kinds could be added with the restriction that
    /// for pointer types the kind value needs to be marked with the PX_SERIAL_REF_KIND_PTR_TYPE_BIT.
    pub fn PxSerializationContext_registerReference_mut(self_: *mut PxSerializationContext, base: *mut PxBase, kind: u32, reference: usize);

    /// Returns the collection that is being serialized.
    pub fn PxSerializationContext_getCollection(self_: *const PxSerializationContext) -> *const PxCollection;

    /// Serializes object data and object extra data.
    ///
    /// This function is assumed to be called within the implementation of PxSerializer::exportData and PxSerializer::exportExtraData.
    pub fn PxSerializationContext_writeData_mut(self_: *mut PxSerializationContext, data: *const std::ffi::c_void, size: u64);

    /// Aligns the serialized data.
    ///
    /// This function is assumed to be called within the implementation of PxSerializer::exportData and PxSerializer::exportExtraData.
    pub fn PxSerializationContext_alignData_mut(self_: *mut PxSerializationContext, alignment: u32);

    /// Helper function to write a name to the extraData if serialization is configured to save names.
    ///
    /// This function is assumed to be called within the implementation of PxSerializer::exportExtraData.
    pub fn PxSerializationContext_writeName_mut(self_: *mut PxSerializationContext, name: *const std::ffi::c_char);

    /// Retrieves a pointer to a deserialized PxBase object given a corresponding deserialized reference value
    ///
    /// This method is assumed to be called in the implementation of PxSerializer::createObject in order
    /// to update reference values on deserialization.
    ///
    /// To update a PxBase reference the corresponding deserialized pointer value needs to be provided in order to retrieve
    /// the location of the corresponding deserialized PxBase object. (PxDeserializationContext::translatePxBase simplifies
    /// this common case).
    ///
    /// For other kinds of references the reverence values need to be updated by deduction given the corresponding PxBase instance.
    ///
    /// PxBase object associated with the reference value
    pub fn PxDeserializationContext_resolveReference(self_: *const PxDeserializationContext, kind: u32, reference: usize) -> *mut PxBase;

    /// Helper function to read a name from the extra data during deserialization.
    ///
    /// This function is assumed to be called within the implementation of PxSerializer::createObject.
    pub fn PxDeserializationContext_readName_mut(self_: *mut PxDeserializationContext, name: *mut *const std::ffi::c_char);

    /// Function to align the extra data stream to a power of 2 alignment
    ///
    /// This function is assumed to be called within the implementation of PxSerializer::createObject.
    pub fn PxDeserializationContext_alignExtraData_mut(self_: *mut PxDeserializationContext, alignment: u32);

    /// Register a serializer for a concrete type
    pub fn PxSerializationRegistry_registerSerializer_mut(self_: *mut PxSerializationRegistry, type_: u16, serializer: *mut PxSerializer);

    /// Unregister a serializer for a concrete type, and retrieves the corresponding serializer object.
    ///
    /// Unregistered serializer corresponding to type, NULL for types for which no serializer has been registered.
    pub fn PxSerializationRegistry_unregisterSerializer_mut(self_: *mut PxSerializationRegistry, type_: u16) -> *mut PxSerializer;

    /// Returns PxSerializer corresponding to type
    ///
    /// Registered PxSerializer object corresponding to type
    pub fn PxSerializationRegistry_getSerializer(self_: *const PxSerializationRegistry, type_: u16) -> *const PxSerializer;

    /// Register a RepX serializer for a concrete type
    ///
    /// Xml serialization is deprecated. An alternative serialization system is provided through USD Physics.
    pub fn PxSerializationRegistry_registerRepXSerializer_mut(self_: *mut PxSerializationRegistry, type_: u16, serializer: *mut PxRepXSerializer);

    /// Unregister a RepX serializer for a concrete type, and retrieves the corresponding serializer object.
    ///
    /// Xml serialization is deprecated. An alternative serialization system is provided through USD Physics.
    ///
    /// Unregistered PxRepxSerializer corresponding to type, NULL for types for which no RepX serializer has been registered.
    pub fn PxSerializationRegistry_unregisterRepXSerializer_mut(self_: *mut PxSerializationRegistry, type_: u16) -> *mut PxRepXSerializer;

    /// Returns RepX serializer given the corresponding type name
    ///
    /// Xml serialization is deprecated. An alternative serialization system is provided through USD Physics.
    ///
    /// Registered PxRepXSerializer object corresponding to type name
    pub fn PxSerializationRegistry_getRepXSerializer(self_: *const PxSerializationRegistry, typeName: *const std::ffi::c_char) -> *mut PxRepXSerializer;

    /// Releases PxSerializationRegistry instance.
    ///
    /// This unregisters all PhysX and PhysXExtension serializers. Make sure to unregister all custom type
    /// serializers before releasing the PxSerializationRegistry.
    pub fn PxSerializationRegistry_release_mut(self_: *mut PxSerializationRegistry);

    /// Adds a PxBase object to the collection.
    ///
    /// Adds a PxBase object to the collection. Optionally a PxSerialObjectId can be provided
    /// in order to resolve dependencies between collections. A PxSerialObjectId value of PX_SERIAL_OBJECT_ID_INVALID
    /// means the object remains without id. Objects can be added regardless of other objects they require. If the object
    /// is already in the collection, the ID will be set if it was PX_SERIAL_OBJECT_ID_INVALID previously, otherwise the
    /// operation fails.
    pub fn PxCollection_add_mut(self_: *mut PxCollection, object: *mut PxBase, id: u64);

    /// Removes a PxBase member object from the collection.
    ///
    /// Object needs to be contained by the collection.
    pub fn PxCollection_remove_mut(self_: *mut PxCollection, object: *mut PxBase);

    /// Returns whether the collection contains a certain PxBase object.
    ///
    /// Whether object is contained.
    pub fn PxCollection_contains(self_: *const PxCollection, object: *mut PxBase) -> bool;

    /// Adds an id to a member PxBase object.
    ///
    /// If the object is already associated with an id within the collection, the id is replaced.
    /// May only be called for objects that are members of the collection. The id needs to be unique
    /// within the collection.
    pub fn PxCollection_addId_mut(self_: *mut PxCollection, object: *mut PxBase, id: u64);

    /// Removes id from a contained PxBase object.
    ///
    /// May only be called for ids that are associated with an object in the collection.
    pub fn PxCollection_removeId_mut(self_: *mut PxCollection, id: u64);

    /// Adds all PxBase objects and their ids of collection to this collection.
    ///
    /// PxBase objects already in this collection are ignored. Object ids need to be conflict
    /// free, i.e. the same object may not have two different ids within the two collections.
    pub fn PxCollection_add_mut_1(self_: *mut PxCollection, collection: *mut PxCollection);

    /// Removes all PxBase objects of collection from this collection.
    ///
    /// PxBase objects not present in this collection are ignored. Ids of objects
    /// which are removed are also removed.
    pub fn PxCollection_remove_mut_1(self_: *mut PxCollection, collection: *mut PxCollection);

    /// Gets number of PxBase objects in this collection.
    ///
    /// Number of objects in this collection
    pub fn PxCollection_getNbObjects(self_: *const PxCollection) -> u32;

    /// Gets the PxBase object of this collection given its index.
    ///
    /// PxBase object at index index
    pub fn PxCollection_getObject(self_: *const PxCollection, index: u32) -> *mut PxBase;

    /// Copies member PxBase pointers to a user specified buffer.
    ///
    /// number of members PxBase objects that have been written to the userBuffer
    pub fn PxCollection_getObjects(self_: *const PxCollection, userBuffer: *mut *mut PxBase, bufferSize: u32, startIndex: u32) -> u32;

    /// Looks for a PxBase object given a PxSerialObjectId value.
    ///
    /// If there is no PxBase object in the collection with the given id, NULL is returned.
    ///
    /// PxBase object with the given id value or NULL
    pub fn PxCollection_find(self_: *const PxCollection, id: u64) -> *mut PxBase;

    /// Gets number of PxSerialObjectId names in this collection.
    ///
    /// Number of PxSerialObjectId names in this collection
    pub fn PxCollection_getNbIds(self_: *const PxCollection) -> u32;

    /// Copies member PxSerialObjectId values to a user specified buffer.
    ///
    /// number of members PxSerialObjectId values that have been written to the userBuffer
    pub fn PxCollection_getIds(self_: *const PxCollection, userBuffer: *mut u64, bufferSize: u32, startIndex: u32) -> u32;

    /// Gets the PxSerialObjectId name of a PxBase object within the collection.
    ///
    /// The PxBase object needs to be a member of the collection.
    ///
    /// PxSerialObjectId name of the object or PX_SERIAL_OBJECT_ID_INVALID if the object is unnamed
    pub fn PxCollection_getId(self_: *const PxCollection, object: *const PxBase) -> u64;

    /// Deletes a collection object.
    ///
    /// This function only deletes the collection object, i.e. the container class. It doesn't delete objects
    /// that are part of the collection.
    pub fn PxCollection_release_mut(self_: *mut PxCollection);

    /// Creates a collection object.
    ///
    /// Objects can only be serialized or deserialized through a collection.
    /// For serialization, users must add objects to the collection and serialize the collection as a whole.
    /// For deserialization, the system gives back a collection of deserialized objects to users.
    ///
    /// The new collection object.
    pub fn phys_PxCreateCollection() -> *mut PxCollection;

    /// Releases the PxBase instance, please check documentation of release in derived class.
    pub fn PxBase_release_mut(self_: *mut PxBase);

    /// Returns string name of dynamic type.
    ///
    /// Class name of most derived type of this object.
    pub fn PxBase_getConcreteTypeName(self_: *const PxBase) -> *const std::ffi::c_char;

    /// Returns concrete type of object.
    ///
    /// PxConcreteType::Enum of serialized object
    pub fn PxBase_getConcreteType(self_: *const PxBase) -> u16;

    /// Set PxBaseFlag
    pub fn PxBase_setBaseFlag_mut(self_: *mut PxBase, flag: PxBaseFlag, value: bool);

    /// Set PxBaseFlags
    pub fn PxBase_setBaseFlags_mut(self_: *mut PxBase, inFlags: PxBaseFlags);

    /// Returns PxBaseFlags
    ///
    /// PxBaseFlags
    pub fn PxBase_getBaseFlags(self_: *const PxBase) -> PxBaseFlags;

    /// Whether the object is subordinate.
    ///
    /// A class is subordinate, if it can only be instantiated in the context of another class.
    ///
    /// Whether the class is subordinate
    pub fn PxBase_isReleasable(self_: *const PxBase) -> bool;

    /// Decrements the reference count of the object and releases it if the new reference count is zero.
    pub fn PxRefCounted_release_mut(self_: *mut PxRefCounted);

    /// Returns the reference count of the object.
    ///
    /// At creation, the reference count of the object is 1. Every other object referencing this object increments the
    /// count by 1.	When the reference count reaches 0, and only then, the object gets destroyed automatically.
    ///
    /// the current reference count.
    pub fn PxRefCounted_getReferenceCount(self_: *const PxRefCounted) -> u32;

    /// Acquires a counted reference to this object.
    ///
    /// This method increases the reference count of the object by 1. Decrement the reference count by calling release()
    pub fn PxRefCounted_acquireReference_mut(self_: *mut PxRefCounted);

    /// constructor sets to default
    pub fn PxTolerancesScale_new(defaultLength: f32, defaultSpeed: f32) -> PxTolerancesScale;

    /// Returns true if the descriptor is valid.
    ///
    /// true if the current settings are valid (returns always true).
    pub fn PxTolerancesScale_isValid(self_: *const PxTolerancesScale) -> bool;

    /// Allocate a new string.
    ///
    /// *Always* a valid null terminated string.  "" is returned if "" or null is passed in.
    pub fn PxStringTable_allocateStr_mut(self_: *mut PxStringTable, inSrc: *const std::ffi::c_char) -> *const std::ffi::c_char;

    /// Release the string table and all the strings associated with it.
    pub fn PxStringTable_release_mut(self_: *mut PxStringTable);

    /// Returns string name of dynamic type.
    ///
    /// Class name of most derived type of this object.
    pub fn PxSerializer_getConcreteTypeName(self_: *const PxSerializer) -> *const std::ffi::c_char;

    /// Adds required objects to the collection.
    ///
    /// This method does not add the required objects recursively, e.g. objects required by required objects.
    pub fn PxSerializer_requiresObjects(self_: *const PxSerializer, anon_param0: *mut PxBase, anon_param1: *mut PxProcessPxBaseCallback);

    /// Whether the object is subordinate.
    ///
    /// A class is subordinate, if it can only be instantiated in the context of another class.
    ///
    /// Whether the class is subordinate
    pub fn PxSerializer_isSubordinate(self_: *const PxSerializer) -> bool;

    /// Exports object's extra data to stream.
    pub fn PxSerializer_exportExtraData(self_: *const PxSerializer, anon_param0: *mut PxBase, anon_param1: *mut PxSerializationContext);

    /// Exports object's data to stream.
    pub fn PxSerializer_exportData(self_: *const PxSerializer, anon_param0: *mut PxBase, anon_param1: *mut PxSerializationContext);

    /// Register references that the object maintains to other objects.
    pub fn PxSerializer_registerReferences(self_: *const PxSerializer, obj: *mut PxBase, s: *mut PxSerializationContext);

    /// Returns size needed to create the class instance.
    ///
    /// sizeof class instance.
    pub fn PxSerializer_getClassSize(self_: *const PxSerializer) -> usize;

    /// Create object at a given address, resolve references and import extra data.
    ///
    /// Created PxBase pointer (needs to be identical to address before increment).
    pub fn PxSerializer_createObject(self_: *const PxSerializer, address: *mut *mut u8, context: *mut PxDeserializationContext) -> *mut PxBase;

    /// *******************************************************************************************************************
    pub fn PxSerializer_delete(self_: *mut PxSerializer);

    /// Builds object (TriangleMesh, Heightfield, ConvexMesh or BVH) from given data in PxPhysics.
    ///
    /// PxBase Created object in PxPhysics.
    pub fn PxInsertionCallback_buildObjectFromData_mut(self_: *mut PxInsertionCallback, type_: PxConcreteType, data: *mut std::ffi::c_void) -> *mut PxBase;

    /// Set the user-provided dispatcher object for CPU tasks
    pub fn PxTaskManager_setCpuDispatcher_mut(self_: *mut PxTaskManager, ref_: *mut PxCpuDispatcher);

    /// Get the user-provided dispatcher object for CPU tasks
    ///
    /// The CPU dispatcher object.
    pub fn PxTaskManager_getCpuDispatcher(self_: *const PxTaskManager) -> *mut PxCpuDispatcher;

    /// Reset any dependencies between Tasks
    ///
    /// Will be called at the start of every frame before tasks are submitted.
    pub fn PxTaskManager_resetDependencies_mut(self_: *mut PxTaskManager);

    /// Called by the owning scene to start the task graph.
    ///
    /// All tasks with ref count of 1 will be dispatched.
    pub fn PxTaskManager_startSimulation_mut(self_: *mut PxTaskManager);

    /// Called by the owning scene at the end of a simulation step.
    pub fn PxTaskManager_stopSimulation_mut(self_: *mut PxTaskManager);

    /// Called by the worker threads to inform the PxTaskManager that a task has completed processing.
    pub fn PxTaskManager_taskCompleted_mut(self_: *mut PxTaskManager, task: *mut PxTask);

    /// Retrieve a task by name
    ///
    /// The ID of the task with that name, or eNOT_PRESENT if not found
    pub fn PxTaskManager_getNamedTask_mut(self_: *mut PxTaskManager, name: *const std::ffi::c_char) -> u32;

    /// Submit a task with a unique name.
    ///
    /// The ID of the task with that name, or eNOT_PRESENT if not found
    pub fn PxTaskManager_submitNamedTask_mut(self_: *mut PxTaskManager, task: *mut PxTask, name: *const std::ffi::c_char, type_: PxTaskType) -> u32;

    /// Submit an unnamed task.
    ///
    /// The ID of the task with that name, or eNOT_PRESENT if not found
    pub fn PxTaskManager_submitUnnamedTask_mut(self_: *mut PxTaskManager, task: *mut PxTask, type_: PxTaskType) -> u32;

    /// Retrieve a task given a task ID
    ///
    /// The task associated with the ID
    pub fn PxTaskManager_getTaskFromID_mut(self_: *mut PxTaskManager, id: u32) -> *mut PxTask;

    /// Release the PxTaskManager object, referenced dispatchers will not be released
    pub fn PxTaskManager_release_mut(self_: *mut PxTaskManager);

    /// Construct a new PxTaskManager instance with the given [optional] dispatchers
    pub fn PxTaskManager_createTaskManager(errorCallback: *mut PxErrorCallback, anon_param1: *mut PxCpuDispatcher) -> *mut PxTaskManager;

    /// Called by the TaskManager when a task is to be queued for execution.
    ///
    /// Upon receiving a task, the dispatcher should schedule the task to run.
    /// After the task has been run, it should call the release() method and
    /// discard its pointer.
    pub fn PxCpuDispatcher_submitTask_mut(self_: *mut PxCpuDispatcher, task: *mut PxBaseTask);

    /// Returns the number of available worker threads for this dispatcher.
    ///
    /// The SDK will use this count to control how many tasks are submitted. By
    /// matching the number of tasks with the number of execution units task
    /// overhead can be reduced.
    pub fn PxCpuDispatcher_getWorkerCount(self_: *const PxCpuDispatcher) -> u32;

    pub fn PxCpuDispatcher_delete(self_: *mut PxCpuDispatcher);

    /// The user-implemented run method where the task's work should be performed
    ///
    /// run() methods must be thread safe, stack friendly (no alloca, etc), and
    /// must never block.
    pub fn PxBaseTask_run_mut(self_: *mut PxBaseTask);

    /// Return a user-provided task name for profiling purposes.
    ///
    /// It does not have to be unique, but unique names are helpful.
    ///
    /// The name of this task
    pub fn PxBaseTask_getName(self_: *const PxBaseTask) -> *const std::ffi::c_char;

    /// Implemented by derived implementation classes
    pub fn PxBaseTask_addReference_mut(self_: *mut PxBaseTask);

    /// Implemented by derived implementation classes
    pub fn PxBaseTask_removeReference_mut(self_: *mut PxBaseTask);

    /// Implemented by derived implementation classes
    pub fn PxBaseTask_getReference(self_: *const PxBaseTask) -> i32;

    /// Implemented by derived implementation classes
    ///
    /// A task may assume in its release() method that the task system no longer holds
    /// references to it - so it may safely run its destructor, recycle itself, etc.
    /// provided no additional user references to the task exist
    pub fn PxBaseTask_release_mut(self_: *mut PxBaseTask);

    /// Tells the scheduler if a task is high priority or not.
    ///
    /// This function is a hint to the scheduler, to let it know that some tasks are
    /// higher priority than others. The scheduler should try to execute high priority
    /// tasks first, but there is no guarantee that it does (some schedulers can ignore
    /// this information).
    ///
    /// True for high priority task, false for regular tasks
    pub fn PxBaseTask_isHighPriority(self_: *const PxBaseTask) -> bool;

    /// Return PxTaskManager to which this task was submitted
    ///
    /// Note, can return NULL if task was not submitted, or has been
    /// completed.
    pub fn PxBaseTask_getTaskManager(self_: *const PxBaseTask) -> *mut PxTaskManager;

    pub fn PxBaseTask_setContextId_mut(self_: *mut PxBaseTask, id: u64);

    pub fn PxBaseTask_getContextId(self_: *const PxBaseTask) -> u64;

    /// Release method implementation
    pub fn PxTask_release_mut(self_: *mut PxTask);

    /// Inform the PxTaskManager this task must finish before the given
    pub fn PxTask_finishBefore_mut(self_: *mut PxTask, taskID: u32);

    /// Inform the PxTaskManager this task cannot start until the given
    pub fn PxTask_startAfter_mut(self_: *mut PxTask, taskID: u32);

    /// Manually increment this task's reference count. The task will
    /// not be allowed to run until removeReference() is called.
    pub fn PxTask_addReference_mut(self_: *mut PxTask);

    /// Manually decrement this task's reference count. If the reference
    /// count reaches zero, the task will be dispatched.
    pub fn PxTask_removeReference_mut(self_: *mut PxTask);

    /// Return the ref-count for this task
    pub fn PxTask_getReference(self_: *const PxTask) -> i32;

    /// Return the unique ID for this task
    pub fn PxTask_getTaskID(self_: *const PxTask) -> u32;

    /// Called by PxTaskManager at submission time for initialization
    ///
    /// Perform simulation step initialization here.
    pub fn PxTask_submitted_mut(self_: *mut PxTask);

    /// Initialize this task and specify the task that will have its ref count decremented on completion.
    ///
    /// Submission is deferred until the task's mRefCount is decremented to zero.
    /// Note that we only use the PxTaskManager to query the appropriate dispatcher.
    pub fn PxLightCpuTask_setContinuation_mut(self_: *mut PxLightCpuTask, tm: *mut PxTaskManager, c: *mut PxBaseTask);

    /// Initialize this task and specify the task that will have its ref count decremented on completion.
    ///
    /// This overload of setContinuation() queries the PxTaskManager from the continuation
    /// task, which cannot be NULL.
    pub fn PxLightCpuTask_setContinuation_mut_1(self_: *mut PxLightCpuTask, c: *mut PxBaseTask);

    /// Retrieves continuation task
    pub fn PxLightCpuTask_getContinuation(self_: *const PxLightCpuTask) -> *mut PxBaseTask;

    /// Manually decrement this task's reference count. If the reference
    /// count reaches zero, the task will be dispatched.
    pub fn PxLightCpuTask_removeReference_mut(self_: *mut PxLightCpuTask);

    /// Return the ref-count for this task
    pub fn PxLightCpuTask_getReference(self_: *const PxLightCpuTask) -> i32;

    /// Manually increment this task's reference count. The task will
    /// not be allowed to run until removeReference() is called.
    pub fn PxLightCpuTask_addReference_mut(self_: *mut PxLightCpuTask);

    /// called by CpuDispatcher after run method has completed
    ///
    /// Decrements the continuation task's reference count, if specified.
    pub fn PxLightCpuTask_release_mut(self_: *mut PxLightCpuTask);

    /// Returns the type of the geometry.
    ///
    /// The type of the object.
    pub fn PxGeometry_getType(self_: *const PxGeometry) -> PxGeometryType;

    /// Constructor to initialize half extents from scalar parameters.
    pub fn PxBoxGeometry_new(hx: f32, hy: f32, hz: f32) -> PxBoxGeometry;

    /// Constructor to initialize half extents from vector parameter.
    pub fn PxBoxGeometry_new_1(halfExtents_: PxVec3) -> PxBoxGeometry;

    /// Returns true if the geometry is valid.
    ///
    /// True if the current settings are valid
    ///
    /// A valid box has a positive extent in each direction (halfExtents.x > 0, halfExtents.y > 0, halfExtents.z > 0).
    /// It is illegal to call PxPhysics::createShape with a box that has zero extent in any direction.
    pub fn PxBoxGeometry_isValid(self_: *const PxBoxGeometry) -> bool;

    pub fn PxReportCallbackBase_new_alloc(capacity: u32) -> *mut PxReportCallbackBase;

    pub fn PxReportCallbackBase_delete(self_: *mut PxReportCallbackBase);

    /// Returns the number of bounds in the BVH.
    ///
    /// You can use [`getBounds`]() to retrieve the bounds.
    ///
    /// These are the user-defined bounds passed to the BVH builder, not the internal bounds around each BVH node.
    ///
    /// Number of bounds in the BVH.
    pub fn PxBVH_getNbBounds(self_: *const PxBVH) -> u32;

    /// Retrieve the read-only bounds in the BVH.
    ///
    /// These are the user-defined bounds passed to the BVH builder, not the internal bounds around each BVH node.
    pub fn PxBVH_getBounds(self_: *const PxBVH) -> *const PxBounds3;

    /// Retrieve the bounds in the BVH.
    ///
    /// These bounds can be modified. Call refit() after modifications are done.
    ///
    /// These are the user-defined bounds passed to the BVH builder, not the internal bounds around each BVH node.
    pub fn PxBVH_getBoundsForModification_mut(self_: *mut PxBVH) -> *mut PxBounds3;

    /// Refit the BVH.
    ///
    /// This function "refits" the tree, i.e. takes the new (leaf) bounding boxes into account and
    /// recomputes all the BVH bounds accordingly. This is an O(n) operation with n = number of bounds in the BVH.
    ///
    /// This works best with minor bounds modifications, i.e. when the bounds remain close to their initial values.
    /// With large modifications the tree quality degrades more and more, and subsequent query performance suffers.
    /// It might be a better strategy to create a brand new BVH if bounds change drastically.
    ///
    /// This function refits the whole tree after an arbitrary number of bounds have potentially been modified by
    /// users (via getBoundsForModification()). If you only have a small number of bounds to update, it might be
    /// more efficient to use setBounds() and partialRefit() instead.
    pub fn PxBVH_refit_mut(self_: *mut PxBVH);

    /// Update single bounds.
    ///
    /// This is an alternative to getBoundsForModification() / refit(). If you only have a small set of bounds to
    /// update, it can be inefficient to call the refit() function, because it refits the whole BVH.
    ///
    /// Instead, one can update individual bounds with this updateBounds() function. It sets the new bounds and
    /// marks the corresponding BVH nodes for partial refit. Once all the individual bounds have been updated,
    /// call partialRefit() to only refit the subset of marked nodes.
    ///
    /// true if success
    pub fn PxBVH_updateBounds_mut(self_: *mut PxBVH, boundsIndex: u32, newBounds: *const PxBounds3) -> bool;

    /// Refits subset of marked nodes.
    ///
    /// This is an alternative to the refit() function, to be called after updateBounds() calls.
    /// See updateBounds() for details.
    pub fn PxBVH_partialRefit_mut(self_: *mut PxBVH);

    pub fn PxBVH_getConcreteTypeName(self_: *const PxBVH) -> *const std::ffi::c_char;

    /// Constructor, initializes to a capsule with passed radius and half height.
    pub fn PxCapsuleGeometry_new(radius_: f32, halfHeight_: f32) -> PxCapsuleGeometry;

    /// Returns true if the geometry is valid.
    ///
    /// True if the current settings are valid.
    ///
    /// A valid capsule has radius > 0, halfHeight >= 0.
    /// It is illegal to call PxPhysics::createShape with a capsule that has zero radius or height.
    pub fn PxCapsuleGeometry_isValid(self_: *const PxCapsuleGeometry) -> bool;

    /// Returns the number of vertices.
    ///
    /// Number of vertices.
    pub fn PxConvexMesh_getNbVertices(self_: *const PxConvexMesh) -> u32;

    /// Returns the vertices.
    ///
    /// Array of vertices.
    pub fn PxConvexMesh_getVertices(self_: *const PxConvexMesh) -> *const PxVec3;

    /// Returns the index buffer.
    ///
    /// Index buffer.
    pub fn PxConvexMesh_getIndexBuffer(self_: *const PxConvexMesh) -> *const u8;

    /// Returns the number of polygons.
    ///
    /// Number of polygons.
    pub fn PxConvexMesh_getNbPolygons(self_: *const PxConvexMesh) -> u32;

    /// Returns the polygon data.
    ///
    /// True if success.
    pub fn PxConvexMesh_getPolygonData(self_: *const PxConvexMesh, index: u32, data: *mut PxHullPolygon) -> bool;

    /// Decrements the reference count of a convex mesh and releases it if the new reference count is zero.
    pub fn PxConvexMesh_release_mut(self_: *mut PxConvexMesh);

    /// Returns the mass properties of the mesh assuming unit density.
    ///
    /// The following relationship holds between mass and volume:
    ///
    /// mass = volume * density
    ///
    /// The mass of a unit density mesh is equal to its volume, so this function returns the volume of the mesh.
    ///
    /// Similarly, to obtain the localInertia of an identically shaped object with a uniform density of d, simply multiply the
    /// localInertia of the unit density mesh by d.
    pub fn PxConvexMesh_getMassInformation(self_: *const PxConvexMesh, mass: *mut f32, localInertia: *mut PxMat33, localCenterOfMass: *mut PxVec3);

    /// Returns the local-space (vertex space) AABB from the convex mesh.
    ///
    /// local-space bounds
    pub fn PxConvexMesh_getLocalBounds(self_: *const PxConvexMesh) -> PxBounds3;

    /// Returns the local-space Signed Distance Field for this mesh if it has one.
    ///
    /// local-space SDF.
    pub fn PxConvexMesh_getSDF(self_: *const PxConvexMesh) -> *const f32;

    pub fn PxConvexMesh_getConcreteTypeName(self_: *const PxConvexMesh) -> *const std::ffi::c_char;

    /// This method decides whether a convex mesh is gpu compatible. If the total number of vertices are more than 64 or any number of vertices in a polygon is more than 32, or
    /// convex hull data was not cooked with GPU data enabled during cooking or was loaded from a serialized collection, the convex hull is incompatible with GPU collision detection. Otherwise
    /// it is compatible.
    ///
    /// True if the convex hull is gpu compatible
    pub fn PxConvexMesh_isGpuCompatible(self_: *const PxConvexMesh) -> bool;

    /// Constructor initializes to identity scale.
    pub fn PxMeshScale_new() -> PxMeshScale;

    /// Constructor from scalar.
    pub fn PxMeshScale_new_1(r: f32) -> PxMeshScale;

    /// Constructor to initialize to arbitrary scale and identity scale rotation.
    pub fn PxMeshScale_new_2(s: *const PxVec3) -> PxMeshScale;

    /// Constructor to initialize to arbitrary scaling.
    pub fn PxMeshScale_new_3(s: *const PxVec3, r: *const PxQuat) -> PxMeshScale;

    /// Returns true if the scaling is an identity transformation.
    pub fn PxMeshScale_isIdentity(self_: *const PxMeshScale) -> bool;

    /// Returns the inverse of this scaling transformation.
    pub fn PxMeshScale_getInverse(self_: *const PxMeshScale) -> PxMeshScale;

    /// Converts this transformation to a 3x3 matrix representation.
    pub fn PxMeshScale_toMat33(self_: *const PxMeshScale) -> PxMat33;

    /// Returns true if combination of negative scale components will cause the triangle normal to flip. The SDK will flip the normals internally.
    pub fn PxMeshScale_hasNegativeDeterminant(self_: *const PxMeshScale) -> bool;

    pub fn PxMeshScale_transform(self_: *const PxMeshScale, v: *const PxVec3) -> PxVec3;

    pub fn PxMeshScale_isValidForTriangleMesh(self_: *const PxMeshScale) -> bool;

    pub fn PxMeshScale_isValidForConvexMesh(self_: *const PxMeshScale) -> bool;

    /// Constructor. By default creates an empty object with a NULL mesh and identity scale.
    pub fn PxConvexMeshGeometry_new(mesh: *mut PxConvexMesh, scaling: *const PxMeshScale, flags: PxConvexMeshGeometryFlags) -> PxConvexMeshGeometry;

    /// Returns true if the geometry is valid.
    ///
    /// True if the current settings are valid for shape creation.
    ///
    /// A valid convex mesh has a positive scale value in each direction (scale.x > 0, scale.y > 0, scale.z > 0).
    /// It is illegal to call PxPhysics::createShape with a convex that has zero extent in any direction.
    pub fn PxConvexMeshGeometry_isValid(self_: *const PxConvexMeshGeometry) -> bool;

    /// Constructor.
    pub fn PxSphereGeometry_new(ir: f32) -> PxSphereGeometry;

    /// Returns true if the geometry is valid.
    ///
    /// True if the current settings are valid
    ///
    /// A valid sphere has radius > 0.
    /// It is illegal to call PxPhysics::createShape with a sphere that has zero radius.
    pub fn PxSphereGeometry_isValid(self_: *const PxSphereGeometry) -> bool;

    /// Constructor.
    pub fn PxPlaneGeometry_new() -> PxPlaneGeometry;

    /// Returns true if the geometry is valid.
    ///
    /// True if the current settings are valid
    pub fn PxPlaneGeometry_isValid(self_: *const PxPlaneGeometry) -> bool;

    /// Constructor. By default creates an empty object with a NULL mesh and identity scale.
    pub fn PxTriangleMeshGeometry_new(mesh: *mut PxTriangleMesh, scaling: *const PxMeshScale, flags: PxMeshGeometryFlags) -> PxTriangleMeshGeometry;

    /// Returns true if the geometry is valid.
    ///
    /// True if the current settings are valid for shape creation.
    ///
    /// A valid triangle mesh has a positive scale value in each direction (scale.scale.x > 0, scale.scale.y > 0, scale.scale.z > 0).
    /// It is illegal to call PxPhysics::createShape with a triangle mesh that has zero extents in any direction.
    pub fn PxTriangleMeshGeometry_isValid(self_: *const PxTriangleMeshGeometry) -> bool;

    /// Constructor.
    pub fn PxHeightFieldGeometry_new(hf: *mut PxHeightField, flags: PxMeshGeometryFlags, heightScale_: f32, rowScale_: f32, columnScale_: f32) -> PxHeightFieldGeometry;

    /// Returns true if the geometry is valid.
    ///
    /// True if the current settings are valid
    ///
    /// A valid height field has a positive scale value in each direction (heightScale > 0, rowScale > 0, columnScale > 0).
    /// It is illegal to call PxPhysics::createShape with a height field that has zero extents in any direction.
    pub fn PxHeightFieldGeometry_isValid(self_: *const PxHeightFieldGeometry) -> bool;

    pub fn PxFilterData_new(anon_param0: PxEMPTY) -> PxFilterData;

    /// Default constructor.
    pub fn PxFilterData_new_1() -> PxFilterData;

    /// Constructor to set filter data initially.
    pub fn PxFilterData_new_2(w0: u32, w1: u32, w2: u32, w3: u32) -> PxFilterData;

    /// (re)sets the structure to the default.
    pub fn PxFilterData_setToDefault_mut(self_: *mut PxFilterData);

    /// Extract filter object type from the filter attributes of a collision pair object
    ///
    /// The type of the collision pair object.
    pub fn phys_PxGetFilterObjectType(attr: u32) -> PxFilterObjectType;

    /// Specifies whether the collision object belongs to a kinematic rigid body
    ///
    /// True if the object belongs to a kinematic rigid body, else false
    pub fn phys_PxFilterObjectIsKinematic(attr: u32) -> bool;

    /// Specifies whether the collision object is a trigger shape
    ///
    /// True if the object is a trigger shape, else false
    pub fn phys_PxFilterObjectIsTrigger(attr: u32) -> bool;

    /// Filter method to specify how a pair of potentially colliding objects should be processed.
    ///
    /// This method gets called when the filter flags returned by the filter shader (see [`PxSimulationFilterShader`])
    /// indicate that the filter callback should be invoked ([`PxFilterFlag::eCALLBACK`] or #PxFilterFlag::eNOTIFY set).
    /// Return the PxFilterFlag flags and set the PxPairFlag flags to define what the simulation should do with the given
    /// collision pair.
    ///
    /// Filter flags defining whether the pair should be discarded, temporarily ignored or processed and whether the pair
    /// should be tracked and send a report on pair deletion through the filter callback
    pub fn PxSimulationFilterCallback_pairFound_mut(self_: *mut PxSimulationFilterCallback, pairID: u64, attributes0: u32, filterData0: PxFilterData, a0: *const PxActor, s0: *const PxShape, attributes1: u32, filterData1: PxFilterData, a1: *const PxActor, s1: *const PxShape, pairFlags: *mut PxPairFlags) -> PxFilterFlags;

    /// Callback to inform that a tracked collision pair is gone.
    ///
    /// This method gets called when a collision pair disappears or gets re-filtered. Only applies to
    /// collision pairs which have been marked as filter callback pairs ([`PxFilterFlag::eNOTIFY`] set in #pairFound()).
    pub fn PxSimulationFilterCallback_pairLost_mut(self_: *mut PxSimulationFilterCallback, pairID: u64, attributes0: u32, filterData0: PxFilterData, attributes1: u32, filterData1: PxFilterData, objectRemoved: bool);

    /// Callback to give the opportunity to change the filter state of a tracked collision pair.
    ///
    /// This method gets called once per simulation step to let the application change the filter and pair
    /// flags of a collision pair that has been reported in [`pairFound`]() and requested callbacks by
    /// setting [`PxFilterFlag::eNOTIFY`]. To request a change of filter status, the target pair has to be
    /// specified by its ID, the new filter and pair flags have to be provided and the method should return true.
    ///
    /// If this method changes the filter status of a collision pair and the pair should keep being tracked
    /// by the filter callbacks then [`PxFilterFlag::eNOTIFY`] has to be set.
    ///
    /// The application is responsible to ensure that this method does not get called for pairs that have been
    /// reported as lost, see [`pairLost`]().
    ///
    /// True if the changes should be applied. In this case the method will get called again. False if
    /// no more status changes should be done in the current simulation step. In that case the provided flags will be discarded.
    pub fn PxSimulationFilterCallback_statusChange_mut(self_: *mut PxSimulationFilterCallback, pairID: *mut u64, pairFlags: *mut PxPairFlags, filterFlags: *mut PxFilterFlags) -> bool;

    /// Deletes the actor.
    ///
    /// Do not keep a reference to the deleted instance.
    ///
    /// If the actor belongs to a [`PxAggregate`] object, it is automatically removed from the aggregate.
    pub fn PxActor_release_mut(self_: *mut PxActor);

    /// Retrieves the type of actor.
    ///
    /// The actor type of the actor.
    pub fn PxActor_getType(self_: *const PxActor) -> PxActorType;

    /// Retrieves the scene which this actor belongs to.
    ///
    /// Owner Scene. NULL if not part of a scene.
    pub fn PxActor_getScene(self_: *const PxActor) -> *mut PxScene;

    /// Sets a name string for the object that can be retrieved with getName().
    ///
    /// This is for debugging and is not used by the SDK. The string is not copied by the SDK,
    /// only the pointer is stored.
    ///
    /// Default:
    /// NULL
    pub fn PxActor_setName_mut(self_: *mut PxActor, name: *const std::ffi::c_char);

    /// Retrieves the name string set with setName().
    ///
    /// Name string associated with object.
    pub fn PxActor_getName(self_: *const PxActor) -> *const std::ffi::c_char;

    /// Retrieves the axis aligned bounding box enclosing the actor.
    ///
    /// It is not allowed to use this method while the simulation is running (except during PxScene::collide(),
    /// in PxContactModifyCallback or in contact report callbacks).
    ///
    /// The actor's bounding box.
    pub fn PxActor_getWorldBounds(self_: *const PxActor, inflation: f32) -> PxBounds3;

    /// Raises or clears a particular actor flag.
    ///
    /// See the list of flags [`PxActorFlag`]
    ///
    /// Sleeping:
    /// Does
    /// NOT
    /// wake the actor up automatically.
    pub fn PxActor_setActorFlag_mut(self_: *mut PxActor, flag: PxActorFlag, value: bool);

    /// Sets the actor flags.
    ///
    /// See the list of flags [`PxActorFlag`]
    pub fn PxActor_setActorFlags_mut(self_: *mut PxActor, inFlags: PxActorFlags);

    /// Reads the PxActor flags.
    ///
    /// See the list of flags [`PxActorFlag`]
    ///
    /// The values of the PxActor flags.
    pub fn PxActor_getActorFlags(self_: *const PxActor) -> PxActorFlags;

    /// Assigns dynamic actors a dominance group identifier.
    ///
    /// PxDominanceGroup is a 5 bit group identifier (legal range from 0 to 31).
    ///
    /// The PxScene::setDominanceGroupPair() lets you set certain behaviors for pairs of dominance groups.
    /// By default every dynamic actor is created in group 0.
    ///
    /// Default:
    /// 0
    ///
    /// Sleeping:
    /// Changing the dominance group does
    /// NOT
    /// wake the actor up automatically.
    pub fn PxActor_setDominanceGroup_mut(self_: *mut PxActor, dominanceGroup: u8);

    /// Retrieves the value set with setDominanceGroup().
    ///
    /// The dominance group of this actor.
    pub fn PxActor_getDominanceGroup(self_: *const PxActor) -> u8;

    /// Sets the owner client of an actor.
    ///
    /// This cannot be done once the actor has been placed into a scene.
    ///
    /// Default:
    /// PX_DEFAULT_CLIENT
    pub fn PxActor_setOwnerClient_mut(self_: *mut PxActor, inClient: u8);

    /// Returns the owner client that was specified at creation time.
    ///
    /// This value cannot be changed once the object is placed into the scene.
    pub fn PxActor_getOwnerClient(self_: *const PxActor) -> u8;

    /// Retrieves the aggregate the actor might be a part of.
    ///
    /// The aggregate the actor is a part of, or NULL if the actor does not belong to an aggregate.
    pub fn PxActor_getAggregate(self_: *const PxActor) -> *mut PxAggregate;

    /// Sets the environment ID for this actor.
    ///
    /// The environment ID is an extra built-in filter group for the GPU broadphase. Actors will only collide with each-other if they have the
    /// same environment ID.
    ///
    /// The default value is PX_INVALID_U32. Actors with this ID will collide with other actors, regardless of which environment they are a part of.
    ///
    /// The environment ID must be set before adding the actor to a scene, and cannot change while the actor is in the scene.
    ///
    /// If it is not PX_INVALID_U32, the environment ID must be smaller than 1
    /// <
    /// <
    /// 24, i.e. the system does not support more than 1
    /// <
    /// <
    /// 24 environments.
    ///
    /// Default:
    /// PX_INVALID_U32
    ///
    /// This is not available for CPU broadphases.
    ///
    /// True if success.
    pub fn PxActor_setEnvironmentID_mut(self_: *mut PxActor, envID: u32) -> bool;

    /// Returns the environment ID for this actor.
    ///
    /// Environment ID for this actor.
    pub fn PxActor_getEnvironmentID(self_: *const PxActor) -> u32;

    /// Destructor
    pub fn PxParticleSystemCallback_delete(self_: *mut PxParticleSystemCallback);

    pub fn PxMultiCallback_new_alloc() -> *mut PxMultiCallback;

    /// Adds a callback
    ///
    /// True if the callback was added
    pub fn PxMultiCallback_addCallback_mut(self_: *mut PxMultiCallback, callback: *mut PxParticleSystemCallback) -> bool;

    /// Removes a callback
    ///
    /// True if the callback was removed
    pub fn PxMultiCallback_removeCallback_mut(self_: *mut PxMultiCallback, callback: *const PxParticleSystemCallback) -> bool;

    pub fn PxMultiCallback_delete(self_: *mut PxMultiCallback);

    pub fn PxPBDParticleSystem_delete(self_: *mut PxPBDParticleSystem);

    /// Sets the solver iteration counts for the body.
    ///
    /// The solver iteration count determines how accurately joints and contacts are resolved.
    /// If you are having trouble with jointed bodies oscillating and behaving erratically, then
    /// setting a higher position iteration count may improve their stability.
    ///
    /// If intersecting bodies are being depenetrated too violently, increase the number of velocity
    /// iterations. More velocity iterations will drive the relative exit velocity of the intersecting
    /// objects closer to the correct value given the restitution.
    ///
    /// Default:
    /// 4 position iterations, 1 velocity iteration
    ///
    /// See [`getSolverIterationCounts`]()
    pub fn PxPBDParticleSystem_setSolverIterationCounts_mut(self_: *mut PxPBDParticleSystem, minPositionIters: u32, minVelocityIters: u32);

    /// Retrieves the solver iteration counts.
    ///
    /// See [`setSolverIterationCounts`]()
    pub fn PxPBDParticleSystem_getSolverIterationCounts(self_: *const PxPBDParticleSystem, minPositionIters: *mut u32, minVelocityIters: *mut u32);

    /// Retrieves the collision filter settings.
    ///
    /// The filter data
    pub fn PxPBDParticleSystem_getSimulationFilterData(self_: *const PxPBDParticleSystem) -> PxFilterData;

    /// Set collision filter settings
    ///
    /// Allows to control with which objects the particle system collides
    pub fn PxPBDParticleSystem_setSimulationFilterData_mut(self_: *mut PxPBDParticleSystem, data: *const PxFilterData);

    /// Set particle flag
    ///
    /// Allows to control self collision etc.
    pub fn PxPBDParticleSystem_setParticleFlag_mut(self_: *mut PxPBDParticleSystem, flag: PxParticleFlag, val: bool);

    /// Set particle flags
    ///
    /// Allows to control self collision etc.
    pub fn PxPBDParticleSystem_setParticleFlags_mut(self_: *mut PxPBDParticleSystem, flags: PxParticleFlags);

    /// Retrieves the particle flags.
    ///
    /// The particle flags
    pub fn PxPBDParticleSystem_getParticleFlags(self_: *const PxPBDParticleSystem) -> PxParticleFlags;

    /// Set the maximal depenetration velocity particles can reach
    ///
    /// Allows to limit the particles' maximal depenetration velocity to avoid that collision responses lead to very high particle velocities
    pub fn PxPBDParticleSystem_setMaxDepenetrationVelocity_mut(self_: *mut PxPBDParticleSystem, maxDepenetrationVelocity: f32);

    /// Retrieves maximal depenetration velocity a particle can have.
    ///
    /// The maximal depenetration velocity
    pub fn PxPBDParticleSystem_getMaxDepenetrationVelocity(self_: *const PxPBDParticleSystem) -> f32;

    /// Set the maximal linear velocity particles can reach.
    ///
    /// Allows to limit the particles' maximal velocity to control the maximal distance a particle can move per frame.
    ///
    /// Default:
    /// PX_MAX_F32
    pub fn PxPBDParticleSystem_setMaxLinearVelocity_mut(self_: *mut PxPBDParticleSystem, maxLinearVelocity: f32);

    /// Retrieves maximal linear velocity a particle can have.
    ///
    /// The maximal linear velocity
    pub fn PxPBDParticleSystem_getMaxLinearVelocity(self_: *const PxPBDParticleSystem) -> f32;

    /// Use setMaxLinearVelocity() instead.
    pub fn PxPBDParticleSystem_setMaxVelocity_mut(self_: *mut PxPBDParticleSystem, maxVelocity: f32);

    /// Use getMaxLinearVelocity() instead.
    pub fn PxPBDParticleSystem_getMaxVelocity(self_: *const PxPBDParticleSystem) -> f32;

    /// Return the cuda context manager
    ///
    /// The cuda context manager
    pub fn PxPBDParticleSystem_getCudaContextManager(self_: *const PxPBDParticleSystem) -> *mut PxCudaContextManager;

    /// Set the rest offset for the collision between particles and rigids or deformable bodies.
    ///
    /// A particle and a rigid or deformable body will come to rest at a distance equal to the sum of their restOffset values.
    pub fn PxPBDParticleSystem_setRestOffset_mut(self_: *mut PxPBDParticleSystem, restOffset: f32);

    /// Return the rest offset
    ///
    /// the rest offset
    ///
    /// See [`setRestOffset`]()
    pub fn PxPBDParticleSystem_getRestOffset(self_: *const PxPBDParticleSystem) -> f32;

    /// Set the contact offset for the collision between particles and rigids or soft bodies
    ///
    /// The contact offset needs to be larger than the rest offset.
    /// Contact constraints are generated for a particle and a rigid or deformable below the distance equal to the sum of their contacOffset values.
    pub fn PxPBDParticleSystem_setContactOffset_mut(self_: *mut PxPBDParticleSystem, contactOffset: f32);

    /// Return the contact offset
    ///
    /// the contact offset
    ///
    /// See [`setContactOffset`]()
    pub fn PxPBDParticleSystem_getContactOffset(self_: *const PxPBDParticleSystem) -> f32;

    /// Set the contact offset for the interactions between particles
    ///
    /// The particle contact offset needs to be larger than the fluid rest offset and larger than the solid rest offset.
    /// Interactions for two particles are computed if their distance is below twice the particleContactOffset value.
    pub fn PxPBDParticleSystem_setParticleContactOffset_mut(self_: *mut PxPBDParticleSystem, particleContactOffset: f32);

    /// Return the particle contact offset
    ///
    /// the particle contact offset
    ///
    /// See [`setParticleContactOffset`]()
    pub fn PxPBDParticleSystem_getParticleContactOffset(self_: *const PxPBDParticleSystem) -> f32;

    /// Set the solid rest offset
    ///
    /// Two solid particles (or a solid and a fluid particle) will come to rest at a distance equal to twice the solidRestOffset value.
    pub fn PxPBDParticleSystem_setSolidRestOffset_mut(self_: *mut PxPBDParticleSystem, solidRestOffset: f32);

    /// Return the solid rest offset
    ///
    /// the solid rest offset
    ///
    /// See [`setSolidRestOffset`]()
    pub fn PxPBDParticleSystem_getSolidRestOffset(self_: *const PxPBDParticleSystem) -> f32;

    /// Reads the particle lock flags.
    ///
    /// See the list of flags [`PxParticleLockFlag`]
    ///
    /// The values of the particle lock flags.
    pub fn PxPBDParticleSystem_getParticleLockFlags(self_: *const PxPBDParticleSystem) -> PxParticleLockFlags;

    /// Raises or clears a particular particle lock flag.
    ///
    /// See the list of flags [`PxParticleLockFlag`]
    ///
    /// Default:
    /// no flags are set
    pub fn PxPBDParticleSystem_setParticleLockFlag_mut(self_: *mut PxPBDParticleSystem, flag: PxParticleLockFlag, value: bool);

    /// Set all particle lock flags.
    pub fn PxPBDParticleSystem_setParticleLockFlags_mut(self_: *mut PxPBDParticleSystem, flags: PxParticleLockFlags);

    /// Creates combined particle flag with particle material and particle phase flags.
    ///
    /// The combined particle group index and phase flags.
    ///
    /// See [`PxParticlePhaseFlag`]
    pub fn PxPBDParticleSystem_createPhase_mut(self_: *mut PxPBDParticleSystem, material: *mut PxPBDMaterial, flags: PxParticlePhaseFlags) -> u32;

    /// Returns number of particle materials referenced by particle phases
    ///
    /// The number of particle materials
    pub fn PxPBDParticleSystem_getNbParticleMaterials(self_: *const PxPBDParticleSystem) -> u32;

    /// Returns particle materials referenced by particle phases
    ///
    /// The particle materials
    pub fn PxPBDParticleSystem_getParticleMaterials(self_: *const PxPBDParticleSystem, userBuffer: *mut *mut PxPBDMaterial, bufferSize: u32, startIndex: u32) -> u32;

    /// Sets a user notify object which receives special simulation events when they occur.
    ///
    /// Do not set the callback while the simulation is running. Calls to this method while the simulation is running will be ignored.
    ///
    /// A call to fetchResultsParticleSystem() on the PxScene will synchronize the work such that the caller knows that all worke done in the callback completed.
    ///
    /// See [`PxParticleSystemCallback`], #getParticleSystemCallback()
    pub fn PxPBDParticleSystem_setParticleSystemCallback_mut(self_: *mut PxPBDParticleSystem, callback: *mut PxParticleSystemCallback);

    /// Retrieves the simulationEventCallback pointer set with setSimulationEventCallback().
    ///
    /// The current user notify pointer. See PxSimulationEventCallback.
    ///
    /// See [`PxParticleSystemCallback`], #setParticleSystemCallback()
    pub fn PxPBDParticleSystem_getParticleSystemCallback(self_: *const PxPBDParticleSystem) -> *mut PxParticleSystemCallback;

    /// Add an existing particle buffer to the particle system.
    ///
    /// See [`PxParticleBuffer`].
    pub fn PxPBDParticleSystem_addParticleBuffer_mut(self_: *mut PxPBDParticleSystem, particleBuffer: *mut PxParticleBuffer);

    /// Remove particle buffer from the particle system.
    ///
    /// See [`PxParticleBuffer`].
    pub fn PxPBDParticleSystem_removeParticleBuffer_mut(self_: *mut PxPBDParticleSystem, particleBuffer: *mut PxParticleBuffer);

    /// Returns the GPU particle system index.
    ///
    /// The GPU index, if the particle system is in a scene and PxSceneFlag::eENABLE_DIRECT_GPU_API is set, or 0xFFFFFFFF otherwise.
    pub fn PxPBDParticleSystem_getGpuParticleSystemIndex_mut(self_: *mut PxPBDParticleSystem) -> u32;

    /// Set wind direction and intensity
    pub fn PxPBDParticleSystem_setWind_mut(self_: *mut PxPBDParticleSystem, wind: *const PxVec3);

    /// Retrieves the wind direction and intensity.
    ///
    /// The wind direction and intensity
    pub fn PxPBDParticleSystem_getWind(self_: *const PxPBDParticleSystem) -> PxVec3;

    /// Set the fluid boundary density scale
    ///
    /// Defines how strong of a contribution the boundary (typically a rigid surface) should have on a fluid particle's density.
    pub fn PxPBDParticleSystem_setFluidBoundaryDensityScale_mut(self_: *mut PxPBDParticleSystem, fluidBoundaryDensityScale: f32);

    /// Return the fluid boundary density scale
    ///
    /// the fluid boundary density scale
    ///
    /// See [`setFluidBoundaryDensityScale`]()
    pub fn PxPBDParticleSystem_getFluidBoundaryDensityScale(self_: *const PxPBDParticleSystem) -> f32;

    /// Set the fluid rest offset
    ///
    /// Two fluid particles will come to rest at a distance equal to twice the fluidRestOffset value.
    pub fn PxPBDParticleSystem_setFluidRestOffset_mut(self_: *mut PxPBDParticleSystem, fluidRestOffset: f32);

    /// Return the fluid rest offset
    ///
    /// the fluid rest offset
    ///
    /// See [`setFluidRestOffset`]()
    pub fn PxPBDParticleSystem_getFluidRestOffset(self_: *const PxPBDParticleSystem) -> f32;

    /// Set the particle system grid size x dimension
    pub fn PxPBDParticleSystem_setGridSizeX_mut(self_: *mut PxPBDParticleSystem, gridSizeX: u32);

    /// Get the particle system grid size x dimension
    ///
    /// [in] the x dimension in the particle grid
    ///
    /// See [`setGridSizeX`]()
    pub fn PxPBDParticleSystem_getGridSizeX(self_: *const PxPBDParticleSystem) -> u32;

    /// Set the particle system grid size y dimension
    pub fn PxPBDParticleSystem_setGridSizeY_mut(self_: *mut PxPBDParticleSystem, gridSizeY: u32);

    /// Get the particle system grid size y dimension
    ///
    /// [in] the y dimension in the particle grid
    ///
    /// See [`setGridSizeY`]()
    pub fn PxPBDParticleSystem_getGridSizeY(self_: *const PxPBDParticleSystem) -> u32;

    /// Set the particle system grid size z dimension
    pub fn PxPBDParticleSystem_setGridSizeZ_mut(self_: *mut PxPBDParticleSystem, gridSizeZ: u32);

    /// Get the particle system grid size z dimension
    ///
    /// [in] the z dimension in the particle grid
    ///
    /// See [`setGridSizeZ`]()
    pub fn PxPBDParticleSystem_getGridSizeZ(self_: *const PxPBDParticleSystem) -> u32;

    pub fn PxPBDParticleSystem_getConcreteTypeName(self_: *const PxPBDParticleSystem) -> *const std::ffi::c_char;

    /// Default constructor.
    ///
    /// Creates an empty object with no particles.
    pub fn PxParticleSystemGeometry_new() -> PxParticleSystemGeometry;

    /// Returns true if the geometry is valid.
    ///
    /// True if the current settings are valid for shape creation.
    pub fn PxParticleSystemGeometry_isValid(self_: *const PxParticleSystemGeometry) -> bool;

    /// Constructor. By default creates an empty object with a NULL mesh and identity scale.
    pub fn PxTetrahedronMeshGeometry_new(mesh: *mut PxTetrahedronMesh) -> PxTetrahedronMeshGeometry;

    /// Returns true if the geometry is valid.
    ///
    /// True if the current settings are valid for shape creation.
    ///
    /// A valid tetrahedron mesh has a positive scale value in each direction (scale.scale.x > 0, scale.scale.y > 0, scale.scale.z > 0).
    /// It is illegal to call PxPhysics::createShape with a tetrahedron mesh that has zero extents in any direction.
    pub fn PxTetrahedronMeshGeometry_isValid(self_: *const PxTetrahedronMeshGeometry) -> bool;

    pub fn PxQueryHit_new() -> PxQueryHit;

    pub fn PxLocationHit_new() -> PxLocationHit;

    /// For raycast hits: true for shapes overlapping with raycast origin.
    ///
    /// For sweep hits: true for shapes overlapping at zero sweep distance.
    pub fn PxLocationHit_hadInitialOverlap(self_: *const PxLocationHit) -> bool;

    pub fn PxGeomRaycastHit_new() -> PxGeomRaycastHit;

    pub fn PxGeomOverlapHit_new() -> PxGeomOverlapHit;

    pub fn PxGeomSweepHit_new() -> PxGeomSweepHit;

    pub fn PxGeomIndexPair_new() -> PxGeomIndexPair;

    pub fn PxGeomIndexPair_new_1(_id0: u32, _id1: u32) -> PxGeomIndexPair;

    pub fn PxGeomIndexClosePair_new() -> PxGeomIndexClosePair;

    pub fn PxGeomIndexClosePair_new_1(_id0: u32, _id1: u32, d: f32) -> PxGeomIndexClosePair;

    /// For internal use
    pub fn PxCustomGeometry_getUniqueID() -> u32;

    /// Default constructor.
    ///
    /// Creates an empty object with a NULL callbacks pointer.
    pub fn PxCustomGeometry_new() -> PxCustomGeometry;

    /// Returns true if the geometry is valid.
    ///
    /// True if the current settings are valid for shape creation.
    pub fn PxCustomGeometry_isValid(self_: *const PxCustomGeometry) -> bool;

    /// Default constructor
    pub fn PxConvexCoreGeometry_new() -> PxConvexCoreGeometry;

    /// Get the type of the core
    ///
    /// The type of the core
    pub fn PxConvexCoreGeometry_getCoreType(self_: *const PxConvexCoreGeometry) -> PxConvexCore;

    /// Get a pointer to the core data.
    ///
    /// A pointer to the core data.
    pub fn PxConvexCoreGeometry_getCoreData(self_: *const PxConvexCoreGeometry) -> *const std::ffi::c_void;

    /// Get the margin of the convex core geometry.
    ///
    /// The margin size.
    pub fn PxConvexCoreGeometry_getMargin(self_: *const PxConvexCoreGeometry) -> f32;

    /// Check if the convex core geometry is valid.
    ///
    /// True if the geometry is valid, false otherwise.
    pub fn PxConvexCoreGeometry_isValid(self_: *const PxConvexCoreGeometry) -> bool;

    pub fn PxGeometryHolder_getType(self_: *const PxGeometryHolder) -> PxGeometryType;

    pub fn PxGeometryHolder_any_mut(self_: *mut PxGeometryHolder) -> *mut PxGeometry;

    pub fn PxGeometryHolder_any(self_: *const PxGeometryHolder) -> *const PxGeometry;

    pub fn PxGeometryHolder_sphere_mut(self_: *mut PxGeometryHolder) -> *mut PxSphereGeometry;

    pub fn PxGeometryHolder_sphere(self_: *const PxGeometryHolder) -> *const PxSphereGeometry;

    pub fn PxGeometryHolder_plane_mut(self_: *mut PxGeometryHolder) -> *mut PxPlaneGeometry;

    pub fn PxGeometryHolder_plane(self_: *const PxGeometryHolder) -> *const PxPlaneGeometry;

    pub fn PxGeometryHolder_capsule_mut(self_: *mut PxGeometryHolder) -> *mut PxCapsuleGeometry;

    pub fn PxGeometryHolder_capsule(self_: *const PxGeometryHolder) -> *const PxCapsuleGeometry;

    pub fn PxGeometryHolder_box_mut(self_: *mut PxGeometryHolder) -> *mut PxBoxGeometry;

    pub fn PxGeometryHolder_box(self_: *const PxGeometryHolder) -> *const PxBoxGeometry;

    pub fn PxGeometryHolder_convexCore_mut(self_: *mut PxGeometryHolder) -> *mut PxConvexCoreGeometry;

    pub fn PxGeometryHolder_convexCore(self_: *const PxGeometryHolder) -> *const PxConvexCoreGeometry;

    pub fn PxGeometryHolder_convexMesh_mut(self_: *mut PxGeometryHolder) -> *mut PxConvexMeshGeometry;

    pub fn PxGeometryHolder_convexMesh(self_: *const PxGeometryHolder) -> *const PxConvexMeshGeometry;

    pub fn PxGeometryHolder_tetMesh_mut(self_: *mut PxGeometryHolder) -> *mut PxTetrahedronMeshGeometry;

    pub fn PxGeometryHolder_tetMesh(self_: *const PxGeometryHolder) -> *const PxTetrahedronMeshGeometry;

    pub fn PxGeometryHolder_triangleMesh_mut(self_: *mut PxGeometryHolder) -> *mut PxTriangleMeshGeometry;

    pub fn PxGeometryHolder_triangleMesh(self_: *const PxGeometryHolder) -> *const PxTriangleMeshGeometry;

    pub fn PxGeometryHolder_heightField_mut(self_: *mut PxGeometryHolder) -> *mut PxHeightFieldGeometry;

    pub fn PxGeometryHolder_heightField(self_: *const PxGeometryHolder) -> *const PxHeightFieldGeometry;

    pub fn PxGeometryHolder_particleSystem_mut(self_: *mut PxGeometryHolder) -> *mut PxParticleSystemGeometry;

    pub fn PxGeometryHolder_particleSystem(self_: *const PxGeometryHolder) -> *const PxParticleSystemGeometry;

    pub fn PxGeometryHolder_custom_mut(self_: *mut PxGeometryHolder) -> *mut PxCustomGeometry;

    pub fn PxGeometryHolder_custom(self_: *const PxGeometryHolder) -> *const PxCustomGeometry;

    pub fn PxGeometryHolder_storeAny_mut(self_: *mut PxGeometryHolder, geometry: *const PxGeometry);

    pub fn PxGeometryHolder_new() -> PxGeometryHolder;

    pub fn PxGeometryHolder_new_1(geometry: *const PxGeometry) -> PxGeometryHolder;

    /// Raycast test against a geometry object.
    ///
    /// All geometry types are supported except PxParticleSystemGeometry and PxTetrahedronMeshGeometry.
    ///
    /// Number of hits between the ray and the geometry object
    pub fn PxGeometryQuery_raycast(origin: *const PxVec3, unitDir: *const PxVec3, geom: *const PxGeometry, pose: *const PxTransform, maxDist: f32, hitFlags: PxHitFlags, maxHits: u32, rayHits: *mut PxGeomRaycastHit, stride: u32, queryFlags: PxGeometryQueryFlags, threadContext: *mut PxQueryThreadContext) -> u32;

    /// Overlap test for two geometry objects.
    ///
    /// All combinations are supported except:
    ///
    /// PxPlaneGeometry vs. {PxPlaneGeometry, PxTriangleMeshGeometry, PxHeightFieldGeometry}
    ///
    /// PxTriangleMeshGeometry vs. PxHeightFieldGeometry
    ///
    /// PxHeightFieldGeometry vs. PxHeightFieldGeometry
    ///
    /// Anything involving PxParticleSystemGeometry or PxTetrahedronMeshGeometry
    ///
    /// True if the two geometry objects overlap
    pub fn PxGeometryQuery_overlap(geom0: *const PxGeometry, pose0: *const PxTransform, geom1: *const PxGeometry, pose1: *const PxTransform, queryFlags: PxGeometryQueryFlags, threadContext: *mut PxQueryThreadContext) -> bool;

    /// Sweep a specified geometry object in space and test for collision with a given object.
    ///
    /// The following combinations are supported.
    ///
    /// PxSphereGeometry vs. {PxSphereGeometry, PxPlaneGeometry, PxCapsuleGeometry, PxBoxGeometry, PxConvexCoreGeometry, PxConvexMeshGeometry, PxTriangleMeshGeometry, PxHeightFieldGeometry}
    ///
    /// PxCapsuleGeometry vs. {PxSphereGeometry, PxPlaneGeometry, PxCapsuleGeometry, PxBoxGeometry, PxConvexCoreGeometry, PxConvexMeshGeometry, PxTriangleMeshGeometry, PxHeightFieldGeometry}
    ///
    /// PxBoxGeometry vs. {PxSphereGeometry, PxPlaneGeometry, PxCapsuleGeometry, PxBoxGeometry, PxConvexCoreGeometry, PxConvexMeshGeometry, PxTriangleMeshGeometry, PxHeightFieldGeometry}
    ///
    /// PxConvexCoreGeometry vs. {PxSphereGeometry, PxPlaneGeometry, PxCapsuleGeometry, PxBoxGeometry, PxConvexCoreGeometry, PxConvexMeshGeometry, PxTriangleMeshGeometry, PxHeightFieldGeometry}
    ///
    /// PxConvexMeshGeometry vs. {PxSphereGeometry, PxPlaneGeometry, PxCapsuleGeometry, PxBoxGeometry, PxConvexCoreGeometry, PxConvexMeshGeometry, PxTriangleMeshGeometry, PxHeightFieldGeometry}
    ///
    /// True if the swept geometry object geom0 hits the object geom1
    pub fn PxGeometryQuery_sweep(unitDir: *const PxVec3, maxDist: f32, geom0: *const PxGeometry, pose0: *const PxTransform, geom1: *const PxGeometry, pose1: *const PxTransform, sweepHit: *mut PxGeomSweepHit, hitFlags: PxHitFlags, inflation: f32, queryFlags: PxGeometryQueryFlags, threadContext: *mut PxQueryThreadContext) -> bool;

    /// Compute minimum translational distance (MTD) between two geometry objects.
    ///
    /// All combinations of geom objects are supported except:
    /// - plane/plane
    /// - plane/mesh
    /// - plane/heightfield
    /// - mesh/mesh
    /// - mesh/heightfield
    /// - heightfield/heightfield
    /// - anything involving PxParticleSystemGeometry, PxTetrahedronMeshGeometry
    ///
    /// The function returns a unit vector ('direction') and a penetration depth ('depth').
    ///
    /// The depenetration vector D = direction * depth should be applied to the first object, to
    /// get out of the second object.
    ///
    /// Returned depth should always be positive or null.
    ///
    /// If objects do not overlap, the function can not compute the MTD and returns false.
    ///
    /// True if the MTD has successfully been computed, i.e. if objects do overlap.
    pub fn PxGeometryQuery_computePenetration(direction: *mut PxVec3, depth: *mut f32, geom0: *const PxGeometry, pose0: *const PxTransform, geom1: *const PxGeometry, pose1: *const PxTransform, queryFlags: PxGeometryQueryFlags) -> bool;

    /// Computes distance between a point and a geometry object.
    ///
    /// Currently supported geometry objects: box, sphere, capsule, convex core, convex mesh, mesh.
    ///
    /// For meshes, only the BVH34 midphase data-structure is supported.
    ///
    /// Square distance between the point and the geom object, or 0.0 if the point is inside the object, or -1.0 if an error occured (geometry type is not supported, or invalid pose)
    pub fn PxGeometryQuery_pointDistance(point: *const PxVec3, geom: *const PxGeometry, pose: *const PxTransform, closestPoint: *mut PxVec3, closestIndex: *mut u32, queryFlags: PxGeometryQueryFlags) -> f32;

    /// computes the bounds for a geometry object
    ///
    /// True if success, false if an error occurred and the bounds were not written out.
    pub fn PxGeometryQuery_computeGeomBounds(bounds: *mut PxBounds3, geom: *const PxGeometry, pose: *const PxTransform, offset: f32, inflation: f32, queryFlags: PxGeometryQueryFlags) -> bool;

    /// Generate collision contacts between a convex geometry and a single triangle
    ///
    /// True if there was collision
    pub fn PxGeometryQuery_generateTriangleContacts(geom: *const PxGeometry, pose: *const PxTransform, triangleVertices: *const PxVec3, triangleIndex: u32, contactDistance: f32, meshContactMargin: f32, toleranceLength: f32, contactBuffer: *mut PxContactBuffer) -> bool;

    /// Checks if provided geometry is valid.
    ///
    /// True if geometry is valid.
    pub fn PxGeometryQuery_isValid(geom: *const PxGeometry) -> bool;

    pub fn PxHeightFieldSample_tessFlag(self_: *const PxHeightFieldSample) -> u8;

    pub fn PxHeightFieldSample_setTessFlag_mut(self_: *mut PxHeightFieldSample);

    pub fn PxHeightFieldSample_clearTessFlag_mut(self_: *mut PxHeightFieldSample);

    /// Decrements the reference count of a height field and releases it if the new reference count is zero.
    pub fn PxHeightField_release_mut(self_: *mut PxHeightField);

    /// Writes out the sample data array.
    ///
    /// The user provides destBufferSize bytes storage at destBuffer.
    /// The data is formatted and arranged as PxHeightFieldDesc.samples.
    ///
    /// The number of bytes written.
    pub fn PxHeightField_saveCells(self_: *const PxHeightField, destBuffer: *mut std::ffi::c_void, destBufferSize: u32) -> u32;

    /// Replaces a rectangular subfield in the sample data array.
    ///
    /// The user provides the description of a rectangular subfield in subfieldDesc.
    /// The data is formatted and arranged as PxHeightFieldDesc.samples.
    ///
    /// True on success, false on failure. Failure can occur due to format mismatch.
    ///
    /// Modified samples are constrained to the same height quantization range as the original heightfield.
    /// Source samples that are out of range of target heightfield will be clipped with no error.
    /// PhysX does not keep a mapping from the heightfield to heightfield shapes that reference it.
    /// Call PxShape::setGeometry on each shape which references the height field, to ensure that internal data structures are updated to reflect the new geometry.
    /// Please note that PxShape::setGeometry does not guarantee correct/continuous behavior when objects are resting on top of old or new geometry.
    pub fn PxHeightField_modifySamples_mut(self_: *mut PxHeightField, startCol: i32, startRow: i32, subfieldDesc: *const PxHeightFieldDesc, shrinkBounds: bool) -> bool;

    /// Retrieves the number of sample rows in the samples array.
    ///
    /// The number of sample rows in the samples array.
    pub fn PxHeightField_getNbRows(self_: *const PxHeightField) -> u32;

    /// Retrieves the number of sample columns in the samples array.
    ///
    /// The number of sample columns in the samples array.
    pub fn PxHeightField_getNbColumns(self_: *const PxHeightField) -> u32;

    /// Retrieves the format of the sample data.
    ///
    /// The format of the sample data.
    pub fn PxHeightField_getFormat(self_: *const PxHeightField) -> PxHeightFieldFormat;

    /// Retrieves the offset in bytes between consecutive samples in the array.
    ///
    /// The offset in bytes between consecutive samples in the array.
    pub fn PxHeightField_getSampleStride(self_: *const PxHeightField) -> u32;

    /// Retrieves the convex edge threshold.
    ///
    /// The convex edge threshold.
    pub fn PxHeightField_getConvexEdgeThreshold(self_: *const PxHeightField) -> f32;

    /// Retrieves the flags bits, combined from values of the enum ::PxHeightFieldFlag.
    ///
    /// The flags bits, combined from values of the enum ::PxHeightFieldFlag.
    pub fn PxHeightField_getFlags(self_: *const PxHeightField) -> PxHeightFieldFlags;

    /// Retrieves the height at the given coordinates in grid space.
    ///
    /// The height at the given coordinates or 0 if the coordinates are out of range.
    pub fn PxHeightField_getHeight(self_: *const PxHeightField, x: f32, z: f32) -> f32;

    /// Returns material table index of given triangle
    ///
    /// This function takes a post cooking triangle index.
    ///
    /// Material table index, or 0xffff if no per-triangle materials are used
    pub fn PxHeightField_getTriangleMaterialIndex(self_: *const PxHeightField, triangleIndex: u32) -> u16;

    /// Returns a triangle face normal for a given triangle index
    ///
    /// This function takes a post cooking triangle index.
    ///
    /// Triangle normal for a given triangle index
    pub fn PxHeightField_getTriangleNormal(self_: *const PxHeightField, triangleIndex: u32) -> PxVec3;

    /// Returns heightfield sample of given row and column
    ///
    /// Heightfield sample
    pub fn PxHeightField_getSample(self_: *const PxHeightField, row: u32, column: u32) -> *const PxHeightFieldSample;

    /// Returns the number of times the heightfield data has been modified
    ///
    /// This method returns the number of times modifySamples has been called on this heightfield, so that code that has
    /// retained state that depends on the heightfield can efficiently determine whether it has been modified.
    ///
    /// the number of times the heightfield sample data has been modified.
    pub fn PxHeightField_getTimestamp(self_: *const PxHeightField) -> u32;

    pub fn PxHeightField_getConcreteTypeName(self_: *const PxHeightField) -> *const std::ffi::c_char;

    /// Constructor sets to default.
    pub fn PxHeightFieldDesc_new() -> PxHeightFieldDesc;

    /// (re)sets the structure to the default.
    pub fn PxHeightFieldDesc_setToDefault_mut(self_: *mut PxHeightFieldDesc);

    /// Returns true if the descriptor is valid.
    ///
    /// True if the current settings are valid.
    pub fn PxHeightFieldDesc_isValid(self_: *const PxHeightFieldDesc) -> bool;

    /// Retrieves triangle data from a triangle ID.
    ///
    /// This function can be used together with [`findOverlapTriangleMesh`]() to retrieve triangle properties.
    ///
    /// This function will flip the triangle normal whenever triGeom.scale.hasNegativeDeterminant() is true.
    pub fn PxMeshQuery_getTriangle(triGeom: *const PxTriangleMeshGeometry, transform: *const PxTransform, triangleIndex: u32, triangle: *mut PxTriangle, vertexIndices: *mut u32, adjacencyIndices: *mut u32);

    /// Retrieves triangle data from a triangle ID.
    ///
    /// This function can be used together with [`findOverlapHeightField`]() to retrieve triangle properties.
    ///
    /// This function will flip the triangle normal whenever triGeom.scale.hasNegativeDeterminant() is true.
    ///
    /// TriangleIndex is an index used in internal format, which does have an index out of the bounds in last row.
    /// To traverse all tri indices in the HF, the following code can be applied:
    /// for (PxU32 row = 0; row
    /// <
    /// (nbRows - 1); row++)
    /// {
    /// for (PxU32 col = 0; col
    /// <
    /// (nbCols - 1); col++)
    /// {
    /// for (PxU32 k = 0; k
    /// <
    /// 2; k++)
    /// {
    /// const PxU32 triIndex = 2 * (row*nbCols + col) + k;
    /// ....
    /// }
    /// }
    /// }
    pub fn PxMeshQuery_getTriangle_1(hfGeom: *const PxHeightFieldGeometry, transform: *const PxTransform, triangleIndex: u32, triangle: *mut PxTriangle, vertexIndices: *mut u32, adjacencyIndices: *mut u32);

    /// Find the mesh triangles which touch the specified geometry object.
    ///
    /// For mesh-vs-mesh overlap tests, please use the specialized function below.
    ///
    /// Returned triangle indices can be used with [`getTriangle`]() to retrieve the triangle properties.
    ///
    /// Number of overlaps found, i.e. number of elements written to the results buffer
    pub fn PxMeshQuery_findOverlapTriangleMesh(geom: *const PxGeometry, geomPose: *const PxTransform, meshGeom: *const PxTriangleMeshGeometry, meshPose: *const PxTransform, results: *mut u32, maxResults: u32, startIndex: u32, overflow: *mut bool, queryFlags: PxGeometryQueryFlags) -> u32;

    /// Find the height field triangles which touch the specified geometry object.
    ///
    /// Returned triangle indices can be used with [`getTriangle`]() to retrieve the triangle properties.
    ///
    /// Number of overlaps found, i.e. number of elements written to the results buffer
    pub fn PxMeshQuery_findOverlapHeightField(geom: *const PxGeometry, geomPose: *const PxTransform, hfGeom: *const PxHeightFieldGeometry, hfPose: *const PxTransform, results: *mut u32, maxResults: u32, startIndex: u32, overflow: *mut bool, queryFlags: PxGeometryQueryFlags) -> u32;

    /// Sweep a specified geometry object in space and test for collision with a set of given triangles.
    ///
    /// This function simply sweeps input geometry against each input triangle, in the order they are given.
    /// This is an O(N) operation with N = number of input triangles. It does not use any particular acceleration structure.
    ///
    /// True if the swept geometry object hits the specified triangles
    ///
    /// Only the following geometry types are currently supported: PxSphereGeometry, PxCapsuleGeometry, PxBoxGeometry
    ///
    /// If a shape from the scene is already overlapping with the query shape in its starting position, the hit is returned unless eASSUME_NO_INITIAL_OVERLAP was specified.
    ///
    /// This function returns a single closest hit across all the input triangles. Multiple hits are not supported.
    ///
    /// Supported hitFlags are PxHitFlag::eDEFAULT, PxHitFlag::eASSUME_NO_INITIAL_OVERLAP, PxHitFlag::ePRECISE_SWEEP, PxHitFlag::eMESH_BOTH_SIDES, PxHitFlag::eANY_HIT.
    ///
    /// ePOSITION is only defined when there is no initial overlap (sweepHit.hadInitialOverlap() == false)
    ///
    /// The returned normal for initially overlapping sweeps is set to -unitDir.
    ///
    /// Otherwise the returned normal is the front normal of the triangle even if PxHitFlag::eMESH_BOTH_SIDES is set.
    ///
    /// The returned PxGeomSweepHit::faceIndex parameter will hold the index of the hit triangle in input array, i.e. the range is [0; triangleCount). For initially overlapping sweeps, this is the index of overlapping triangle.
    ///
    /// The inflation parameter is not compatible with PxHitFlag::ePRECISE_SWEEP.
    pub fn PxMeshQuery_sweep(unitDir: *const PxVec3, distance: f32, geom: *const PxGeometry, pose: *const PxTransform, triangleCount: u32, triangles: *const PxTriangle, sweepHit: *mut PxGeomSweepHit, hitFlags: PxHitFlags, cachedIndex: *const u32, inflation: f32, doubleSided: bool, queryFlags: PxGeometryQueryFlags) -> bool;

    /// constructor sets to default.
    pub fn PxSimpleTriangleMesh_new() -> PxSimpleTriangleMesh;

    /// (re)sets the structure to the default.
    pub fn PxSimpleTriangleMesh_setToDefault_mut(self_: *mut PxSimpleTriangleMesh);

    /// returns true if the current settings are valid
    pub fn PxSimpleTriangleMesh_isValid(self_: *const PxSimpleTriangleMesh) -> bool;

    /// Constructor
    pub fn PxTriangle_new_alloc() -> *mut PxTriangle;

    /// Constructor
    pub fn PxTriangle_new_alloc_1(p0: *const PxVec3, p1: *const PxVec3, p2: *const PxVec3) -> *mut PxTriangle;

    /// Destructor
    pub fn PxTriangle_delete(self_: *mut PxTriangle);

    /// Compute the normal of the Triangle.
    pub fn PxTriangle_normal(self_: *const PxTriangle, _normal: *mut PxVec3);

    /// Compute the unnormalized normal of the triangle.
    pub fn PxTriangle_denormalizedNormal(self_: *const PxTriangle, _normal: *mut PxVec3);

    /// Compute the area of the triangle.
    ///
    /// Area of the triangle.
    pub fn PxTriangle_area(self_: *const PxTriangle) -> f32;

    /// Computes a point on the triangle from u and v barycentric coordinates.
    pub fn PxTriangle_pointFromUV(self_: *const PxTriangle, u: f32, v: f32) -> PxVec3;

    pub fn PxTrianglePadded_new_alloc() -> *mut PxTrianglePadded;

    pub fn PxTrianglePadded_delete(self_: *mut PxTrianglePadded);

    /// Returns the number of vertices.
    ///
    /// number of vertices
    pub fn PxTriangleMesh_getNbVertices(self_: *const PxTriangleMesh) -> u32;

    /// Returns the vertices.
    ///
    /// array of vertices
    pub fn PxTriangleMesh_getVertices(self_: *const PxTriangleMesh) -> *const PxVec3;

    /// Returns all mesh vertices for modification.
    ///
    /// This function will return the vertices of the mesh so that their positions can be changed in place.
    /// After modifying the vertices you must call refitBVH for the refitting to actually take place.
    /// This function maintains the old mesh topology (triangle indices).
    ///
    /// inplace vertex coordinates for each existing mesh vertex.
    ///
    /// It is recommended to use this feature for scene queries only.
    ///
    /// Size of array returned is equal to the number returned by getNbVertices().
    ///
    /// This function operates on cooked vertex indices.
    ///
    /// This means the index mapping and vertex count can be different from what was provided as an input to the cooking routine.
    ///
    /// To achieve unchanged 1-to-1 index mapping with orignal mesh data (before cooking) please use the following cooking flags:
    ///
    /// eWELD_VERTICES = 0, eDISABLE_CLEAN_MESH = 1.
    ///
    /// It is also recommended to make sure that a call to validateTriangleMesh returns true if mesh cleaning is disabled.
    pub fn PxTriangleMesh_getVerticesForModification_mut(self_: *mut PxTriangleMesh) -> *mut PxVec3;

    /// Refits BVH for mesh vertices.
    ///
    /// This function will refit the mesh BVH to correctly enclose the new positions updated by getVerticesForModification.
    /// Mesh BVH will not be reoptimized by this function so significantly different new positions will cause significantly reduced performance.
    ///
    /// New bounds for the entire mesh.
    ///
    /// For PxMeshMidPhase::eBVH34 trees the refit operation is only available on non-quantized trees (see PxBVH34MidphaseDesc::quantized)
    ///
    /// PhysX does not keep a mapping from the mesh to mesh shapes that reference it.
    ///
    /// Call PxShape::setGeometry on each shape which references the mesh, to ensure that internal data structures are updated to reflect the new geometry.
    ///
    /// PxShape::setGeometry does not guarantee correct/continuous behavior when objects are resting on top of old or new geometry.
    ///
    /// It is also recommended to make sure that a call to validateTriangleMesh returns true if mesh cleaning is disabled.
    ///
    /// Active edges information will be lost during refit, the rigid body mesh contact generation might not perform as expected.
    pub fn PxTriangleMesh_refitBVH_mut(self_: *mut PxTriangleMesh) -> PxBounds3;

    /// Returns the number of triangles.
    ///
    /// number of triangles
    pub fn PxTriangleMesh_getNbTriangles(self_: *const PxTriangleMesh) -> u32;

    /// Returns the triangle indices.
    ///
    /// The indices can be 16 or 32bit depending on the number of triangles in the mesh.
    /// Call getTriangleMeshFlags() to know if the indices are 16 or 32 bits.
    ///
    /// The number of indices is the number of triangles * 3.
    ///
    /// array of triangles
    pub fn PxTriangleMesh_getTriangles(self_: *const PxTriangleMesh) -> *const std::ffi::c_void;

    /// Reads the PxTriangleMesh flags.
    ///
    /// See the list of flags [`PxTriangleMeshFlag`]
    ///
    /// The values of the PxTriangleMesh flags.
    pub fn PxTriangleMesh_getTriangleMeshFlags(self_: *const PxTriangleMesh) -> PxTriangleMeshFlags;

    /// Returns the triangle remapping table.
    ///
    /// The triangles are internally sorted according to various criteria. Hence the internal triangle order
    /// does not always match the original (user-defined) order. The remapping table helps finding the old
    /// indices knowing the new ones:
    ///
    /// remapTable[ internalTriangleIndex ] = originalTriangleIndex
    ///
    /// the remapping table (or NULL if 'PxCookingParams::suppressTriangleMeshRemapTable' has been used)
    pub fn PxTriangleMesh_getTrianglesRemap(self_: *const PxTriangleMesh) -> *const u32;

    /// Decrements the reference count of a triangle mesh and releases it if the new reference count is zero.
    pub fn PxTriangleMesh_release_mut(self_: *mut PxTriangleMesh);

    /// Returns material table index of given triangle
    ///
    /// This function takes a post cooking triangle index.
    ///
    /// Material table index, or 0xffff if no per-triangle materials are used
    pub fn PxTriangleMesh_getTriangleMaterialIndex(self_: *const PxTriangleMesh, triangleIndex: u32) -> u16;

    /// Returns the local-space (vertex space) AABB from the triangle mesh.
    ///
    /// local-space bounds
    pub fn PxTriangleMesh_getLocalBounds(self_: *const PxTriangleMesh) -> PxBounds3;

    /// Returns the local-space Signed Distance Field for this mesh if it has one.
    ///
    /// local-space SDF.
    pub fn PxTriangleMesh_getSDF(self_: *const PxTriangleMesh) -> *const f32;

    /// Returns the resolution of the local-space dense SDF.
    pub fn PxTriangleMesh_getSDFDimensions(self_: *const PxTriangleMesh, numX: *mut u32, numY: *mut u32, numZ: *mut u32);

    /// Sets whether this mesh should be preferred for SDF projection.
    ///
    /// By default, meshes are flagged as preferring projection and the decisions on which mesh to project is based on the triangle and vertex
    /// count. The model with the fewer triangles is projected onto the SDF of the more detailed mesh.
    /// If one of the meshes is set to prefer SDF projection (default) and the other is set to not prefer SDF projection, model flagged as
    /// preferring SDF projection will be projected onto the model flagged as not preferring, regardless of the detail of the respective meshes.
    /// Where both models are flagged as preferring no projection, the less detailed model will be projected as before.
    pub fn PxTriangleMesh_setPreferSDFProjection_mut(self_: *mut PxTriangleMesh, preferProjection: bool);

    /// Returns whether this mesh prefers SDF projection.
    ///
    /// whether this mesh prefers SDF projection.
    pub fn PxTriangleMesh_getPreferSDFProjection(self_: *const PxTriangleMesh) -> bool;

    /// Returns the mass properties of the mesh assuming unit density.
    ///
    /// The following relationship holds between mass and volume:
    ///
    /// mass = volume * density
    ///
    /// The mass of a unit density mesh is equal to its volume, so this function returns the volume of the mesh.
    ///
    /// Similarly, to obtain the localInertia of an identically shaped object with a uniform density of d, simply multiply the
    /// localInertia of the unit density mesh by d.
    pub fn PxTriangleMesh_getMassInformation(self_: *const PxTriangleMesh, mass: *mut f32, localInertia: *mut PxMat33, localCenterOfMass: *mut PxVec3);

    /// Constructor
    pub fn PxTetrahedron_new_alloc() -> *mut PxTetrahedron;

    /// Constructor
    pub fn PxTetrahedron_new_alloc_1(p0: *const PxVec3, p1: *const PxVec3, p2: *const PxVec3, p3: *const PxVec3) -> *mut PxTetrahedron;

    /// Destructor
    pub fn PxTetrahedron_delete(self_: *mut PxTetrahedron);

    /// Decrements the reference count of a tetrahedron mesh and releases it if the new reference count is zero.
    pub fn PxDeformableVolumeAuxData_release_mut(self_: *mut PxDeformableVolumeAuxData);

    /// Get the inverse mass of each vertex of the tetrahedron mesh.
    ///
    /// PxReal* A pointer to an array of inverse mass for each vertex of the tetrahedron mesh. Size: number of vertices * sizeof(PxReal).
    pub fn PxDeformableVolumeAuxData_getGridModelInvMass_mut(self_: *mut PxDeformableVolumeAuxData) -> *mut f32;

    /// Returns the number of vertices.
    ///
    /// number of vertices
    pub fn PxTetrahedronMesh_getNbVertices(self_: *const PxTetrahedronMesh) -> u32;

    /// Returns the vertices
    ///
    /// array of vertices
    pub fn PxTetrahedronMesh_getVertices(self_: *const PxTetrahedronMesh) -> *const PxVec3;

    /// Returns the number of tetrahedrons.
    ///
    /// number of tetrahedrons
    pub fn PxTetrahedronMesh_getNbTetrahedrons(self_: *const PxTetrahedronMesh) -> u32;

    /// Returns the tetrahedron indices.
    ///
    /// The indices can be 16 or 32bit depending on the number of tetrahedrons in the mesh.
    /// Call getTetrahedronMeshFlags() to know if the indices are 16 or 32 bits.
    ///
    /// The number of indices is the number of tetrahedrons * 4.
    ///
    /// array of tetrahedrons
    pub fn PxTetrahedronMesh_getTetrahedrons(self_: *const PxTetrahedronMesh) -> *const std::ffi::c_void;

    /// Reads the PxTetrahedronMesh flags.
    ///
    /// See the list of flags [`PxTetrahedronMeshFlags`]
    ///
    /// The values of the PxTetrahedronMesh flags.
    pub fn PxTetrahedronMesh_getTetrahedronMeshFlags(self_: *const PxTetrahedronMesh) -> PxTetrahedronMeshFlags;

    /// Returns the tetrahedra remapping table.
    ///
    /// The tetrahedra are internally sorted according to various criteria. Hence the internal tetrahedron order
    /// does not always match the original (user-defined) order. The remapping table helps finding the old
    /// indices knowing the new ones:
    ///
    /// remapTable[ internalTetrahedronIndex ] = originalTetrahedronIndex
    ///
    /// the remapping table (or NULL if 'PxCookingParams::suppressTriangleMeshRemapTable' has been used)
    pub fn PxTetrahedronMesh_getTetrahedraRemap(self_: *const PxTetrahedronMesh) -> *const u32;

    /// Returns the local-space (vertex space) AABB from the tetrahedron mesh.
    ///
    /// local-space bounds
    pub fn PxTetrahedronMesh_getLocalBounds(self_: *const PxTetrahedronMesh) -> PxBounds3;

    /// Decrements the reference count of a tetrahedron mesh and releases it if the new reference count is zero.
    pub fn PxTetrahedronMesh_release_mut(self_: *mut PxTetrahedronMesh);

    /// Const accecssor to the deformable volume's collision mesh.
    pub fn PxDeformableVolumeMesh_getCollisionMesh(self_: *const PxDeformableVolumeMesh) -> *const PxTetrahedronMesh;

    /// Accecssor to the deformable volume's collision mesh.
    pub fn PxDeformableVolumeMesh_getCollisionMesh_mut(self_: *mut PxDeformableVolumeMesh) -> *mut PxTetrahedronMesh;

    /// Const accessor to the deformable volume's simulation mesh.
    pub fn PxDeformableVolumeMesh_getSimulationMesh(self_: *const PxDeformableVolumeMesh) -> *const PxTetrahedronMesh;

    /// Accecssor to the deformable volume's simulation mesh.
    pub fn PxDeformableVolumeMesh_getSimulationMesh_mut(self_: *mut PxDeformableVolumeMesh) -> *mut PxTetrahedronMesh;

    /// Const accessor to the deformable volume's simulation state.
    pub fn PxDeformableVolumeMesh_getDeformableVolumeAuxData(self_: *const PxDeformableVolumeMesh) -> *const PxDeformableVolumeAuxData;

    /// Accessor to the deformable volume's auxilary data like mass and rest pose information
    pub fn PxDeformableVolumeMesh_getDeformableVolumeAuxData_mut(self_: *mut PxDeformableVolumeMesh) -> *mut PxDeformableVolumeAuxData;

    /// Decrements the reference count of a tetrahedron mesh and releases it if the new reference count is zero.
    pub fn PxDeformableVolumeMesh_release_mut(self_: *mut PxDeformableVolumeMesh);

    pub fn PxCollisionMeshMappingData_release_mut(self_: *mut PxCollisionMeshMappingData);

    pub fn PxCollisionTetrahedronMeshData_getMesh(self_: *const PxCollisionTetrahedronMeshData) -> *const PxTetrahedronMeshData;

    pub fn PxCollisionTetrahedronMeshData_getMesh_mut(self_: *mut PxCollisionTetrahedronMeshData) -> *mut PxTetrahedronMeshData;

    pub fn PxCollisionTetrahedronMeshData_getData(self_: *const PxCollisionTetrahedronMeshData) -> *const PxDeformableVolumeCollisionData;

    pub fn PxCollisionTetrahedronMeshData_getData_mut(self_: *mut PxCollisionTetrahedronMeshData) -> *mut PxDeformableVolumeCollisionData;

    pub fn PxCollisionTetrahedronMeshData_release_mut(self_: *mut PxCollisionTetrahedronMeshData);

    pub fn PxSimulationTetrahedronMeshData_getMesh_mut(self_: *mut PxSimulationTetrahedronMeshData) -> *mut PxTetrahedronMeshData;

    pub fn PxSimulationTetrahedronMeshData_getData_mut(self_: *mut PxSimulationTetrahedronMeshData) -> *mut PxDeformableVolumeSimulationData;

    pub fn PxSimulationTetrahedronMeshData_release_mut(self_: *mut PxSimulationTetrahedronMeshData);

    pub fn phys_PxGetAggregateFilterHint(type_: PxAggregateType, enableSelfCollision: bool) -> u32;

    pub fn phys_PxGetAggregateSelfCollisionBit(hint: u32) -> u32;

    pub fn phys_PxGetAggregateType(hint: u32) -> PxAggregateType;

    /// Deletes the aggregate object.
    ///
    /// Deleting the PxAggregate object does not delete the aggregated actors. If the PxAggregate object
    /// belongs to a scene, the aggregated actors are automatically re-inserted in that scene. If you intend
    /// to delete both the PxAggregate and its actors, it is best to release the actors first, then release
    /// the PxAggregate when it is empty.
    pub fn PxAggregate_release_mut(self_: *mut PxAggregate);

    /// Adds an actor to the aggregate object.
    ///
    /// A warning is output if the total number of actors is reached, or if the incoming actor already belongs
    /// to an aggregate.
    ///
    /// If the aggregate belongs to a scene, adding an actor to the aggregate also adds the actor to that scene.
    ///
    /// If the actor already belongs to a scene, a warning is output and the call is ignored. You need to remove
    /// the actor from the scene first, before adding it to the aggregate.
    ///
    /// When a BVH is provided the actor shapes are grouped together.
    /// The scene query pruning structure inside PhysX SDK will store/update one
    /// bound per actor. The scene queries against such an actor will query actor
    /// bounds and then make a local space query against the provided BVH, which is in actor's local space.
    pub fn PxAggregate_addActor_mut(self_: *mut PxAggregate, actor: *mut PxActor, bvh: *const PxBVH) -> bool;

    /// Removes an actor from the aggregate object.
    ///
    /// A warning is output if the incoming actor does not belong to the aggregate. Otherwise the actor is
    /// removed from the aggregate. If the aggregate belongs to a scene, the actor is reinserted in that
    /// scene. If you intend to delete the actor, it is best to call [`PxActor::release`]() directly. That way
    /// the actor will be automatically removed from its aggregate (if any) and not reinserted in a scene.
    pub fn PxAggregate_removeActor_mut(self_: *mut PxAggregate, actor: *mut PxActor) -> bool;

    /// Adds an articulation to the aggregate object.
    ///
    /// A warning is output if the total number of actors is reached (every articulation link counts as an actor),
    /// or if the incoming articulation already belongs	to an aggregate.
    ///
    /// If the aggregate belongs to a scene, adding an articulation to the aggregate also adds the articulation to that scene.
    ///
    /// If the articulation already belongs to a scene, a warning is output and the call is ignored. You need to remove
    /// the articulation from the scene first, before adding it to the aggregate.
    pub fn PxAggregate_addArticulation_mut(self_: *mut PxAggregate, articulation: *mut PxArticulationReducedCoordinate) -> bool;

    /// Removes an articulation from the aggregate object.
    ///
    /// A warning is output if the incoming articulation does not belong to the aggregate. Otherwise the articulation is
    /// removed from the aggregate. If the aggregate belongs to a scene, the articulation is reinserted in that
    /// scene. If you intend to delete the articulation, it is best to call [`PxArticulationReducedCoordinate::release`]() directly. That way
    /// the articulation will be automatically removed from its aggregate (if any) and not reinserted in a scene.
    pub fn PxAggregate_removeArticulation_mut(self_: *mut PxAggregate, articulation: *mut PxArticulationReducedCoordinate) -> bool;

    /// Returns the number of actors contained in the aggregate.
    ///
    /// You can use [`getActors`]() to retrieve the actor pointers.
    ///
    /// Number of actors contained in the aggregate.
    pub fn PxAggregate_getNbActors(self_: *const PxAggregate) -> u32;

    /// Retrieves max amount of actors that can be contained in the aggregate.
    ///
    /// Max actor size.
    pub fn PxAggregate_getMaxNbActors(self_: *const PxAggregate) -> u32;

    /// Retrieves max amount of shapes that can be contained in the aggregate.
    ///
    /// Max shape size.
    pub fn PxAggregate_getMaxNbShapes(self_: *const PxAggregate) -> u32;

    /// Retrieve all actors contained in the aggregate.
    ///
    /// You can retrieve the number of actor pointers by calling [`getNbActors`]()
    ///
    /// Number of actor pointers written to the buffer.
    pub fn PxAggregate_getActors(self_: *const PxAggregate, userBuffer: *mut *mut PxActor, bufferSize: u32, startIndex: u32) -> u32;

    /// Retrieves the scene which this aggregate belongs to.
    ///
    /// Owner Scene. NULL if not part of a scene.
    pub fn PxAggregate_getScene_mut(self_: *mut PxAggregate) -> *mut PxScene;

    /// Retrieves aggregate's self-collision flag.
    ///
    /// self-collision flag
    pub fn PxAggregate_getSelfCollision(self_: *const PxAggregate) -> bool;

    /// Sets the environment ID for this aggregate.
    ///
    /// The environment ID is an extra built-in filter group for the GPU broadphase. Aggregates will only collide with actors or aggregates that
    /// have the same environment ID.
    ///
    /// The default value is PX_INVALID_U32. Aggregates with this ID will collide with other actors or aggregates, regardless of which environment
    /// they are a part of.
    ///
    /// The environment ID must be set before adding the aggregate to a scene, and cannot change while the aggregate is in the scene.
    ///
    /// If it is not PX_INVALID_U32, the environment ID must be smaller than 1
    /// <
    /// <
    /// 24, i.e. the system does not support more than 1
    /// <
    /// <
    /// 24 environments.
    ///
    /// Aggregated actors must have a default environment ID (PX_INVALID_U32). The environment ID of the aggregate is used in the broadphase, not
    /// the environment IDs from aggregated actors.
    ///
    /// Default:
    /// PX_INVALID_U32
    ///
    /// This is not available for CPU broadphases.
    ///
    /// True if success.
    pub fn PxAggregate_setEnvironmentID_mut(self_: *mut PxAggregate, envID: u32) -> bool;

    /// Returns the environment ID for this aggregate.
    ///
    /// Environment ID for this aggregate.
    pub fn PxAggregate_getEnvironmentID(self_: *const PxAggregate) -> u32;

    pub fn PxAggregate_getConcreteTypeName(self_: *const PxAggregate) -> *const std::ffi::c_char;

    pub fn Px1DConstraintFlag_new() -> Px1DConstraintFlag;

    pub fn PxConstraintInvMassScale_new() -> PxConstraintInvMassScale;

    pub fn PxConstraintInvMassScale_new_1(lin0: f32, ang0: f32, lin1: f32, ang1: f32) -> PxConstraintInvMassScale;

    /// Visualize joint frames
    pub fn PxConstraintVisualizer_visualizeJointFrames_mut(self_: *mut PxConstraintVisualizer, parent: *const PxTransform, child: *const PxTransform);

    /// Visualize joint linear limit
    pub fn PxConstraintVisualizer_visualizeLinearLimit_mut(self_: *mut PxConstraintVisualizer, t0: *const PxTransform, t1: *const PxTransform, value: f32);

    /// Visualize joint angular limit
    pub fn PxConstraintVisualizer_visualizeAngularLimit_mut(self_: *mut PxConstraintVisualizer, t0: *const PxTransform, lower: f32, upper: f32);

    /// Visualize limit cone
    pub fn PxConstraintVisualizer_visualizeLimitCone_mut(self_: *mut PxConstraintVisualizer, t: *const PxTransform, tanQSwingY: f32, tanQSwingZ: f32);

    /// Visualize joint double cone
    pub fn PxConstraintVisualizer_visualizeDoubleCone_mut(self_: *mut PxConstraintVisualizer, t: *const PxTransform, angle: f32);

    /// Visualize line
    pub fn PxConstraintVisualizer_visualizeLine_mut(self_: *mut PxConstraintVisualizer, p0: *const PxVec3, p1: *const PxVec3, color: u32);

    /// Pre-simulation data preparation
    /// when the constraint is marked dirty, this function is called at the start of the simulation
    /// step for the SDK to copy the constraint data block.
    pub fn PxConstraintConnector_prepareData_mut(self_: *mut PxConstraintConnector) -> *mut std::ffi::c_void;

    /// this function is called by the SDK to update OmniPVD's view of it
    pub fn PxConstraintConnector_updateOmniPvdProperties(self_: *const PxConstraintConnector);

    /// Constraint release callback
    ///
    /// When the SDK deletes a PxConstraint object this function is called by the SDK. In general
    /// custom constraints should not be deleted directly by applications: rather, the constraint
    /// should respond to a release() request by calling PxConstraint::release(), then wait for
    /// this call to release its own resources.
    ///
    /// This function is also called when a PxConstraint object is deleted on cleanup due to
    /// destruction of the PxPhysics object.
    pub fn PxConstraintConnector_onConstraintRelease_mut(self_: *mut PxConstraintConnector);

    /// Center-of-mass shift callback
    ///
    /// This function is called by the SDK when the CoM of one of the actors is moved. Since the
    /// API specifies constraint positions relative to actors, and the constraint shader functions
    /// are supplied with coordinates relative to bodies, some synchronization is usually required
    /// when the application moves an object's center of mass.
    pub fn PxConstraintConnector_onComShift_mut(self_: *mut PxConstraintConnector, actor: u32);

    /// Origin shift callback
    ///
    /// This function is called by the SDK when the scene origin gets shifted and allows to adjust
    /// custom data which contains world space transforms.
    ///
    /// If the adjustments affect constraint shader data, it is necessary to call PxConstraint::markDirty()
    /// to make sure that the data gets synced at the beginning of the next simulation step.
    pub fn PxConstraintConnector_onOriginShift_mut(self_: *mut PxConstraintConnector, shift: *const PxVec3);

    /// Fetches external data for a constraint.
    ///
    /// This function is used by the SDK to acquire a reference to the owner of a constraint and a unique
    /// owner type ID. This information will be passed on when a breakable constraint breaks or when
    /// [`PxConstraint::getExternalReference`]() is called.
    ///
    /// Reference to the external object which owns the constraint.
    pub fn PxConstraintConnector_getExternalReference_mut(self_: *mut PxConstraintConnector, typeID: *mut u32) -> *mut std::ffi::c_void;

    /// Obtain a reference to a PxBase interface if the constraint has one.
    ///
    /// If the constraint does not implement the PxBase interface, it should return NULL.
    pub fn PxConstraintConnector_getSerializable_mut(self_: *mut PxConstraintConnector) -> *mut PxBase;

    /// Obtain the shader function pointer used to prep rows for this constraint
    pub fn PxConstraintConnector_getPrep(self_: *const PxConstraintConnector) -> *mut std::ffi::c_void;

    /// Obtain the pointer to the constraint's constant data
    pub fn PxConstraintConnector_getConstantBlock(self_: *const PxConstraintConnector) -> *const std::ffi::c_void;

    /// Let the connector know it has been connected to a constraint.
    pub fn PxConstraintConnector_connectToConstraint_mut(self_: *mut PxConstraintConnector, anon_param0: *mut PxConstraint);

    /// virtual destructor
    pub fn PxConstraintConnector_delete(self_: *mut PxConstraintConnector);

    pub fn PxSolverBody_new() -> PxSolverBody;

    pub fn PxSolverBodyData_projectVelocity(self_: *const PxSolverBodyData, lin: *const PxVec3, ang: *const PxVec3) -> f32;

    pub fn PxSolverConstraintPrepDesc_delete(self_: *mut PxSolverConstraintPrepDesc);

    /// Allocates constraint data. It is the application's responsibility to release this memory after PxSolveConstraints has completed.
    ///
    /// The allocated memory. This address must be 16-byte aligned.
    pub fn PxConstraintAllocator_reserveConstraintData_mut(self_: *mut PxConstraintAllocator, byteSize: u32) -> *mut u8;

    /// Allocates friction data. Friction data can be retained by the application for a given pair and provided as an input to PxSolverContactDesc to improve simulation stability.
    /// It is the application's responsibility to release this memory. If this memory is released, the application should ensure it does not pass pointers to this memory to PxSolverContactDesc.
    ///
    /// The allocated memory. This address must be 4-byte aligned.
    pub fn PxConstraintAllocator_reserveFrictionData_mut(self_: *mut PxConstraintAllocator, byteSize: u32) -> *mut u8;

    pub fn PxConstraintAllocator_delete(self_: *mut PxConstraintAllocator);

    pub fn PxArticulationLimit_new() -> PxArticulationLimit;

    pub fn PxArticulationLimit_new_1(low_: f32, high_: f32) -> PxArticulationLimit;

    pub fn PxPerformanceEnvelope_new(anon_param0: *const PxEMPTY) -> PxPerformanceEnvelope;

    pub fn PxPerformanceEnvelope_new_1(maxEffort_: f32, maxActuatorVelocity_: f32, velocityDependentResistance_: f32, speedEffortGradient_: f32) -> PxPerformanceEnvelope;

    pub fn PxJointFrictionParams_new() -> PxJointFrictionParams;

    pub fn PxJointFrictionParams_new_1(staticFrictionEffort_: f32, dynamicFrictionEffort_: f32, viscousFrictionCoefficient_: f32) -> PxJointFrictionParams;

    pub fn PxArticulationDrive_new(anon_param0: *const PxEMPTY) -> PxArticulationDrive;

    pub fn PxArticulationDrive_new_1() -> PxArticulationDrive;

    pub fn PxArticulationDrive_new_2(stiffness_: f32, damping_: f32, maxForce_: f32, driveType_: PxArticulationDriveType) -> PxArticulationDrive;

    pub fn PxArticulationDrive_new_3(stiffness_: f32, damping_: f32, envelope_: PxPerformanceEnvelope, driveType_: PxArticulationDriveType) -> PxArticulationDrive;

    pub fn PxTGSSolverBodyVel_projectVelocity(self_: *const PxTGSSolverBodyVel, lin: *const PxVec3, ang: *const PxVec3) -> f32;

    pub fn PxTGSSolverBodyData_projectVelocity(self_: *const PxTGSSolverBodyData, linear: *const PxVec3, angular: *const PxVec3) -> f32;

    pub fn PxTGSSolverConstraintPrepDesc_delete(self_: *mut PxTGSSolverConstraintPrepDesc);

    /// Sets the spring rest length for the sub-tendon from the root to this leaf attachment.
    ///
    /// Setting this on non-leaf attachments has no effect.
    ///
    /// This method should not be used after the direct GPU API has been enabled and initialized. See [`PxDirectGPUAPI`] for the details.
    pub fn PxArticulationAttachment_setRestLength_mut(self_: *mut PxArticulationAttachment, restLength: f32);

    /// Gets the spring rest length for the sub-tendon from the root to this leaf attachment.
    ///
    /// The rest length.
    ///
    /// This method should not be used after the direct GPU API has been enabled and initialized. See [`PxDirectGPUAPI`] for the details.
    pub fn PxArticulationAttachment_getRestLength(self_: *const PxArticulationAttachment) -> f32;

    /// Sets the low and high limit on the length of the sub-tendon from the root to this leaf attachment.
    ///
    /// Setting this on non-leaf attachments has no effect.
    ///
    /// This method should not be used after the direct GPU API has been enabled and initialized. See [`PxDirectGPUAPI`] for the details.
    pub fn PxArticulationAttachment_setLimitParameters_mut(self_: *mut PxArticulationAttachment, parameters: *const PxArticulationTendonLimit);

    /// Gets the low and high limit on the length of the sub-tendon from the root to this leaf attachment.
    ///
    /// Struct with the low and high limit.
    ///
    /// This method should not be used after the direct GPU API has been enabled and initialized. See [`PxDirectGPUAPI`] for the details.
    pub fn PxArticulationAttachment_getLimitParameters(self_: *const PxArticulationAttachment) -> PxArticulationTendonLimit;

    /// Sets the attachment's relative offset in the link actor frame.
    ///
    /// This method should not be used after the direct GPU API has been enabled and initialized. See [`PxDirectGPUAPI`] for the details.
    pub fn PxArticulationAttachment_setRelativeOffset_mut(self_: *mut PxArticulationAttachment, offset: *const PxVec3);

    /// Gets the attachment's relative offset in the link actor frame.
    ///
    /// The relative offset in the link actor frame.
    ///
    /// This method should not be used after the direct GPU API has been enabled and initialized. See [`PxDirectGPUAPI`] for the details.
    pub fn PxArticulationAttachment_getRelativeOffset(self_: *const PxArticulationAttachment) -> PxVec3;

    /// Sets the attachment coefficient.
    ///
    /// This method should not be used after the direct GPU API has been enabled and initialized. See [`PxDirectGPUAPI`] for the details.
    pub fn PxArticulationAttachment_setCoefficient_mut(self_: *mut PxArticulationAttachment, coefficient: f32);

    /// Gets the attachment coefficient.
    ///
    /// The scale that the distance between this attachment and its parent is multiplied by when summing up the spatial tendon's length.
    ///
    /// This method should not be used after the direct GPU API has been enabled and initialized. See [`PxDirectGPUAPI`] for the details.
    pub fn PxArticulationAttachment_getCoefficient(self_: *const PxArticulationAttachment) -> f32;

    /// Gets the articulation link.
    ///
    /// The articulation link that this attachment is attached to.
    pub fn PxArticulationAttachment_getLink(self_: *const PxArticulationAttachment) -> *mut PxArticulationLink;

    /// Gets the parent attachment.
    ///
    /// The parent attachment.
    pub fn PxArticulationAttachment_getParent(self_: *const PxArticulationAttachment) -> *mut PxArticulationAttachment;

    /// Indicates that this attachment is a leaf, and thus defines a sub-tendon from the root to this attachment.
    ///
    /// True: This attachment is a leaf and has zero children; False: Not a leaf.
    pub fn PxArticulationAttachment_isLeaf(self_: *const PxArticulationAttachment) -> bool;

    /// Gets the spatial tendon that the attachment is a part of.
    ///
    /// The tendon.
    pub fn PxArticulationAttachment_getTendon(self_: *const PxArticulationAttachment) -> *mut PxArticulationSpatialTendon;

    /// Releases the attachment.
    ///
    /// Releasing the attachment is not allowed while the articulation is in a scene. In order to
    /// release the attachment, remove and then re-add the articulation to the scene.
    pub fn PxArticulationAttachment_release_mut(self_: *mut PxArticulationAttachment);

    /// Returns the string name of the dynamic type.
    ///
    /// The string name.
    pub fn PxArticulationAttachment_getConcreteTypeName(self_: *const PxArticulationAttachment) -> *const std::ffi::c_char;

    /// Sets the tendon joint coefficient.
    ///
    /// RecipCoefficient is commonly expected to be 1/coefficient, but it can be set to different values to tune behavior; for example, zero can be used to
    /// have a joint axis only participate in the length computation of the tendon, but not have any tendon force applied to it.
    ///
    /// This method should not be used after the direct GPU API has been enabled and initialized. See [`PxDirectGPUAPI`] for the details.
    pub fn PxArticulationTendonJoint_setCoefficient_mut(self_: *mut PxArticulationTendonJoint, axis: PxArticulationAxis, coefficient: f32, recipCoefficient: f32);

    /// Gets the tendon joint coefficient.
    ///
    /// This method should not be used after the direct GPU API has been enabled and initialized. See [`PxDirectGPUAPI`] for the details.
    pub fn PxArticulationTendonJoint_getCoefficient(self_: *const PxArticulationTendonJoint, axis: *mut PxArticulationAxis, coefficient: *mut f32, recipCoefficient: *mut f32);

    /// Gets the articulation link.
    ///
    /// The articulation link (and its incoming joint in particular) that this tendon joint is associated with.
    pub fn PxArticulationTendonJoint_getLink(self_: *const PxArticulationTendonJoint) -> *mut PxArticulationLink;

    /// Gets the parent tendon joint.
    ///
    /// The parent tendon joint.
    pub fn PxArticulationTendonJoint_getParent(self_: *const PxArticulationTendonJoint) -> *mut PxArticulationTendonJoint;

    /// Gets the tendon that the joint is a part of.
    ///
    /// The tendon.
    pub fn PxArticulationTendonJoint_getTendon(self_: *const PxArticulationTendonJoint) -> *mut PxArticulationFixedTendon;

    /// Releases a tendon joint.
    ///
    /// Releasing a tendon joint is not allowed while the articulation is in a scene. In order to
    /// release the joint, remove and then re-add the articulation to the scene.
    pub fn PxArticulationTendonJoint_release_mut(self_: *mut PxArticulationTendonJoint);

    /// Returns the string name of the dynamic type.
    ///
    /// The string name.
    pub fn PxArticulationTendonJoint_getConcreteTypeName(self_: *const PxArticulationTendonJoint) -> *const std::ffi::c_char;

    /// Sets the spring stiffness term acting on the tendon length.
    ///
    /// This method should not be used after the direct GPU API has been enabled and initialized. See [`PxDirectGPUAPI`] for the details.
    pub fn PxArticulationTendon_setStiffness_mut(self_: *mut PxArticulationTendon, stiffness: f32);

    /// Gets the spring stiffness of the tendon.
    ///
    /// The spring stiffness.
    ///
    /// This method should not be used after the direct GPU API has been enabled and initialized. See [`PxDirectGPUAPI`] for the details.
    pub fn PxArticulationTendon_getStiffness(self_: *const PxArticulationTendon) -> f32;

    /// Sets the damping term acting both on the tendon length and tendon-length limits.
    ///
    /// This method should not be used after the direct GPU API has been enabled and initialized. See [`PxDirectGPUAPI`] for the details.
    pub fn PxArticulationTendon_setDamping_mut(self_: *mut PxArticulationTendon, damping: f32);

    /// Gets the damping term acting both on the tendon length and tendon-length limits.
    ///
    /// The damping term.
    ///
    /// This method should not be used after the direct GPU API has been enabled and initialized. See [`PxDirectGPUAPI`] for the details.
    pub fn PxArticulationTendon_getDamping(self_: *const PxArticulationTendon) -> f32;

    /// Sets the limit stiffness term acting on the tendon's length limits.
    ///
    /// For spatial tendons, this parameter applies to all its leaf attachments / sub-tendons.
    ///
    /// This method should not be used after the direct GPU API has been enabled and initialized. See [`PxDirectGPUAPI`] for the details.
    pub fn PxArticulationTendon_setLimitStiffness_mut(self_: *mut PxArticulationTendon, stiffness: f32);

    /// Gets the limit stiffness term acting on the tendon's length limits.
    ///
    /// For spatial tendons, this parameter applies to all its leaf attachments / sub-tendons.
    ///
    /// The limit stiffness term.
    ///
    /// This method should not be used after the direct GPU API has been enabled and initialized. See [`PxDirectGPUAPI`] for the details.
    pub fn PxArticulationTendon_getLimitStiffness(self_: *const PxArticulationTendon) -> f32;

    /// Sets the length offset term for the tendon.
    ///
    /// An offset defines an amount to be added to the accumulated length computed for the tendon. It allows the
    /// application to actuate the tendon by shortening or lengthening it.
    ///
    /// This method should not be used after the direct GPU API has been enabled and initialized. See [`PxDirectGPUAPI`] for the details.
    pub fn PxArticulationTendon_setOffset_mut(self_: *mut PxArticulationTendon, offset: f32, autowake: bool);

    /// Gets the length offset term for the tendon.
    ///
    /// The offset term.
    ///
    /// This method should not be used after the direct GPU API has been enabled and initialized. See [`PxDirectGPUAPI`] for the details.
    pub fn PxArticulationTendon_getOffset(self_: *const PxArticulationTendon) -> f32;

    /// Gets the articulation that the tendon is a part of.
    ///
    /// The articulation.
    pub fn PxArticulationTendon_getArticulation(self_: *const PxArticulationTendon) -> *mut PxArticulationReducedCoordinate;

    /// Releases a tendon to remove it from the articulation and free its associated memory.
    ///
    /// When an articulation is released, its attached tendons are automatically released.
    ///
    /// Releasing a tendon is not allowed while the articulation is in a scene. In order to
    /// release the tendon, remove and then re-add the articulation to the scene.
    pub fn PxArticulationTendon_release_mut(self_: *mut PxArticulationTendon);

    /// Creates an articulation attachment and adds it to the list of children in the parent attachment.
    ///
    /// Creating an attachment is not allowed while the articulation is in a scene. In order to
    /// add the attachment, remove and then re-add the articulation to the scene.
    ///
    /// The newly-created attachment if creation was successful, otherwise a null pointer.
    pub fn PxArticulationSpatialTendon_createAttachment_mut(self_: *mut PxArticulationSpatialTendon, parent: *mut PxArticulationAttachment, coefficient: f32, relativeOffset: PxVec3, link: *mut PxArticulationLink) -> *mut PxArticulationAttachment;

    /// Fills a user-provided buffer of attachment pointers with the set of attachments.
    ///
    /// The number of attachments that were filled into the user buffer.
    pub fn PxArticulationSpatialTendon_getAttachments(self_: *const PxArticulationSpatialTendon, userBuffer: *mut *mut PxArticulationAttachment, bufferSize: u32, startIndex: u32) -> u32;

    /// Returns the number of attachments in the tendon.
    ///
    /// The number of attachments.
    pub fn PxArticulationSpatialTendon_getNbAttachments(self_: *const PxArticulationSpatialTendon) -> u32;

    /// Returns the string name of the dynamic type.
    ///
    /// The string name.
    pub fn PxArticulationSpatialTendon_getConcreteTypeName(self_: *const PxArticulationSpatialTendon) -> *const std::ffi::c_char;

    pub fn PxArticulationSpatialTendon_delete(self_: *mut PxArticulationSpatialTendon);

    /// Creates an articulation tendon joint and adds it to the list of children in the parent tendon joint.
    ///
    /// Creating a tendon joint is not allowed while the articulation is in a scene. In order to
    /// add the joint, remove and then re-add the articulation to the scene.
    ///
    /// The newly-created tendon joint if creation was successful, otherwise a null pointer.
    ///
    /// - The axis motion must not be configured as PxArticulationMotion::eLOCKED.
    /// - The axis cannot be part of a fixed joint, i.e. joint configured as PxArticulationJointType::eFIX.
    pub fn PxArticulationFixedTendon_createTendonJoint_mut(self_: *mut PxArticulationFixedTendon, parent: *mut PxArticulationTendonJoint, axis: PxArticulationAxis, coefficient: f32, recipCoefficient: f32, link: *mut PxArticulationLink) -> *mut PxArticulationTendonJoint;

    /// Fills a user-provided buffer of tendon-joint pointers with the set of tendon joints.
    ///
    /// The number of tendon joints filled into the user buffer.
    pub fn PxArticulationFixedTendon_getTendonJoints(self_: *const PxArticulationFixedTendon, userBuffer: *mut *mut PxArticulationTendonJoint, bufferSize: u32, startIndex: u32) -> u32;

    /// Returns the number of tendon joints in the tendon.
    ///
    /// The number of tendon joints.
    pub fn PxArticulationFixedTendon_getNbTendonJoints(self_: *const PxArticulationFixedTendon) -> u32;

    /// Sets the spring rest length of the tendon.
    ///
    /// The accumulated "length" of a fixed tendon is a linear combination of the joint axis positions that the tendon is
    /// associated with, scaled by the respective tendon joints' coefficients. As such, when the joint positions of all
    /// joints are zero, the accumulated length of a fixed tendon is zero.
    ///
    /// The spring of the tendon is not exerting any force on the articulation when the rest length is equal to the
    /// tendon's accumulated length plus the tendon offset.
    ///
    /// This method should not be used after the direct GPU API has been enabled and initialized. See [`PxDirectGPUAPI`] for the details.
    pub fn PxArticulationFixedTendon_setRestLength_mut(self_: *mut PxArticulationFixedTendon, restLength: f32);

    /// Gets the spring rest length of the tendon.
    ///
    /// The spring rest length of the tendon.
    ///
    /// This method should not be used after the direct GPU API has been enabled and initialized. See [`PxDirectGPUAPI`] for the details.
    pub fn PxArticulationFixedTendon_getRestLength(self_: *const PxArticulationFixedTendon) -> f32;

    /// Sets the low and high limit on the length of the tendon.
    ///
    /// The limits, together with the damping and limit stiffness parameters, act on the accumulated length of the tendon.
    ///
    /// This method should not be used after the direct GPU API has been enabled and initialized. See [`PxDirectGPUAPI`] for the details.
    pub fn PxArticulationFixedTendon_setLimitParameters_mut(self_: *mut PxArticulationFixedTendon, parameter: *const PxArticulationTendonLimit);

    /// Gets the low and high limit on the length of the tendon.
    ///
    /// Struct with the low and high limit.
    ///
    /// This method should not be used after the direct GPU API has been enabled and initialized. See [`PxDirectGPUAPI`] for the details.
    pub fn PxArticulationFixedTendon_getLimitParameters(self_: *const PxArticulationFixedTendon) -> PxArticulationTendonLimit;

    /// Returns the string name of the dynamic type.
    ///
    /// The string name.
    pub fn PxArticulationFixedTendon_getConcreteTypeName(self_: *const PxArticulationFixedTendon) -> *const std::ffi::c_char;

    pub fn PxArticulationFixedTendon_delete(self_: *mut PxArticulationFixedTendon);

    /// Releases the mimic joint.
    ///
    /// Releasing a mimic joint is not allowed while the articulation is in a scene. In order to
    /// release a mimic joint, remove and then re-add the articulation to the scene.
    pub fn PxArticulationMimicJoint_release_mut(self_: *mut PxArticulationMimicJoint);

    /// Returns the articulation that this mimic joint is part of.
    ///
    /// A reference to the articulation.
    pub fn PxArticulationMimicJoint_getArticulation(self_: *const PxArticulationMimicJoint) -> *mut PxArticulationReducedCoordinate;

    /// Get the gear of a mimic joint.
    ///
    /// The gear ratio.
    pub fn PxArticulationMimicJoint_getGearRatio(self_: *const PxArticulationMimicJoint) -> f32;

    /// Set the gear ratio of a mimic joint.
    pub fn PxArticulationMimicJoint_setGearRatio_mut(self_: *mut PxArticulationMimicJoint, gearRatio: f32);

    /// Get the offset of a mimic joint.
    ///
    /// The offset.
    pub fn PxArticulationMimicJoint_getOffset(self_: *const PxArticulationMimicJoint) -> f32;

    /// Set the offset of a mimic joint.
    pub fn PxArticulationMimicJoint_setOffset_mut(self_: *mut PxArticulationMimicJoint, offset: f32);

    /// Get the natural frequency of a mimic joint.
    ///
    /// The natural frequency.
    pub fn PxArticulationMimicJoint_getNaturalFrequency(self_: *const PxArticulationMimicJoint) -> f32;

    /// Set the natural frequency of a mimic joint.
    pub fn PxArticulationMimicJoint_setNaturalFrequency_mut(self_: *mut PxArticulationMimicJoint, naturalFrequency: f32);

    /// Get the damping ratio of a mimic joint.
    ///
    /// The damping ratio.
    pub fn PxArticulationMimicJoint_getDampingRatio(self_: *const PxArticulationMimicJoint) -> f32;

    /// Set the damping ratio of a mimic joint.
    pub fn PxArticulationMimicJoint_setDampingRatio_mut(self_: *mut PxArticulationMimicJoint, dampingRatio: f32);

    /// Return the jointA specified in PxArticulationReducedCoordinate::createMimicJoint()
    ///
    /// The jointA specified in PxArticulationReducedCoordinate::createMimicJoint()
    pub fn PxArticulationMimicJoint_getJointA(self_: *const PxArticulationMimicJoint) -> *mut PxArticulationJointReducedCoordinate;

    /// Return the jointB specified in PxArticulationReducedCoordinate::createMimicJoint()
    ///
    /// The jointB specified in PxArticulationReducedCoordinate::createMimicJoint()
    pub fn PxArticulationMimicJoint_getJointB(self_: *const PxArticulationMimicJoint) -> *mut PxArticulationJointReducedCoordinate;

    /// Return the axisA specified in PxArticulationReducedCoordinate::createMimicJoint()
    ///
    /// The axisA specified in PxArticulationReducedCoordinate::createMimicJoint()
    pub fn PxArticulationMimicJoint_getAxisA(self_: *const PxArticulationMimicJoint) -> PxArticulationAxis;

    /// Return the axisB specified in PxArticulationReducedCoordinate::createMimicJoint()
    ///
    /// The axisB specified in PxArticulationReducedCoordinate::createMimicJoint()
    pub fn PxArticulationMimicJoint_getAxisB(self_: *const PxArticulationMimicJoint) -> PxArticulationAxis;

    /// Returns the string name of the dynamic type.
    ///
    /// The string name.
    pub fn PxArticulationMimicJoint_getConcreteTypeName(self_: *const PxArticulationMimicJoint) -> *const std::ffi::c_char;

    pub fn PxArticulationCache_new() -> PxArticulationCache;

    /// Releases an articulation cache.
    pub fn PxArticulationCache_release_mut(self_: *mut PxArticulationCache);

    /// Returns the scene which this articulation belongs to.
    ///
    /// Owner Scene. NULL if not part of a scene.
    pub fn PxArticulationReducedCoordinate_getScene(self_: *const PxArticulationReducedCoordinate) -> *mut PxScene;

    /// Sets the solver iteration counts for the articulation.
    ///
    /// The solver iteration count determines how accurately contacts, drives, and limits are resolved.
    /// Setting a higher position iteration count may therefore help in scenarios where the articulation
    /// is subject to many constraints; for example, a manipulator articulation with drives and joint limits
    /// that is grasping objects, or several such articulations interacting through contacts. Other situations
    /// where higher position iterations may improve simulation fidelity are: large mass ratios within the
    /// articulation or between the articulation and an object in contact with it; or strong drives in the
    /// articulation being used to manipulate a light object.
    ///
    /// If intersecting bodies are being depenetrated too violently, increase the number of velocity
    /// iterations. More velocity iterations will drive the relative exit velocity of the intersecting
    /// objects closer to the correct value given the restitution.
    ///
    /// This call may not be made during simulation.
    pub fn PxArticulationReducedCoordinate_setSolverIterationCounts_mut(self_: *mut PxArticulationReducedCoordinate, minPositionIters: u32, minVelocityIters: u32);

    /// Returns the solver iteration counts.
    pub fn PxArticulationReducedCoordinate_getSolverIterationCounts(self_: *const PxArticulationReducedCoordinate, minPositionIters: *mut u32, minVelocityIters: *mut u32);

    /// Returns true if this articulation is sleeping.
    ///
    /// When an actor does not move for a period of time, it is no longer simulated in order to reduce computational cost. This state
    /// is called sleeping. However, because the object automatically wakes up when it is either touched by an awake object,
    /// or a sleep-affecting property is changed by the user, the entire sleep mechanism should be transparent to the user.
    ///
    /// An articulation can only go to sleep if all links are ready for sleeping. An articulation is guaranteed to be awake
    /// if at least one of the following holds:
    ///
    /// The wake counter of any link in the articulation is positive (see [`setWakeCounter`]()).
    ///
    /// The mass-normalized energy of any link in the articulation is above a threshold (see [`setSleepThreshold`]()).
    ///
    /// A non-zero force or torque has been applied to any joint or link.
    ///
    /// If an articulation is sleeping, the following state is guaranteed:
    ///
    /// The wake counter is zero.
    ///
    /// The linear and angular velocity of all links is zero.
    ///
    /// There is no force update pending.
    ///
    /// When an articulation gets inserted into a scene, it will be considered asleep if all the points above hold, else it will
    /// be treated as awake.
    ///
    /// If an articulation is asleep after the call to [`PxScene::fetchResults`]() returns, it is guaranteed that the poses of the
    /// links were not changed. You can use this information to avoid updating the transforms of associated objects.
    ///
    /// True if the articulation is sleeping.
    ///
    /// This call may only be made on articulations that are in a scene, and may not be made during simulation,
    /// except in a split simulation in-between [`PxScene::fetchCollision`] and #PxScene::advance.
    pub fn PxArticulationReducedCoordinate_isSleeping(self_: *const PxArticulationReducedCoordinate) -> bool;

    /// Sets the mass-normalized energy threshold below which the articulation may go to sleep.
    ///
    /// The articulation will sleep if the energy of each link is below this threshold.
    ///
    /// This call may not be made during simulation.
    ///
    /// Default:
    /// 5e-5f * PxTolerancesScale::speed * PxTolerancesScale::speed;
    pub fn PxArticulationReducedCoordinate_setSleepThreshold_mut(self_: *mut PxArticulationReducedCoordinate, threshold: f32);

    /// Returns the mass-normalized energy below which the articulation may go to sleep.
    ///
    /// The energy threshold for sleeping.
    pub fn PxArticulationReducedCoordinate_getSleepThreshold(self_: *const PxArticulationReducedCoordinate) -> f32;

    /// Sets the mass-normalized kinetic energy threshold below which the articulation may participate in stabilization.
    ///
    /// Articulations whose kinetic energy divided by their mass is above this threshold will not participate in stabilization.
    ///
    /// This value has no effect if PxSceneFlag::eENABLE_STABILIZATION was not enabled on the PxSceneDesc.
    ///
    /// Default:
    /// 5e-6f * PxTolerancesScale::speed * PxTolerancesScale::speed
    ///
    /// This call may not be made during simulation.
    pub fn PxArticulationReducedCoordinate_setStabilizationThreshold_mut(self_: *mut PxArticulationReducedCoordinate, threshold: f32);

    /// Returns the mass-normalized kinetic energy below which the articulation may participate in stabilization.
    ///
    /// Articulations whose kinetic energy divided by their mass is above this threshold will not participate in stabilization.
    ///
    /// The energy threshold for participating in stabilization.
    pub fn PxArticulationReducedCoordinate_getStabilizationThreshold(self_: *const PxArticulationReducedCoordinate) -> f32;

    /// Sets the wake counter for the articulation in seconds.
    ///
    /// - The wake counter value specifies a time threshold used to determine whether an articulation may be put to sleep.
    /// - The articulation will be put to sleep if all links have experienced a mass-normalised energy less than a threshold for at least
    /// a threshold time, as specified by the wake counter.
    /// - Passing in a positive value will wake up the articulation automatically.
    ///
    /// Default:
    /// 0.4s (which corresponds to 20 frames for a time step of 0.02s)
    ///
    /// This call may not be made during simulation, except in a split simulation in-between [`PxScene::fetchCollision`] and #PxScene::advance.
    pub fn PxArticulationReducedCoordinate_setWakeCounter_mut(self_: *mut PxArticulationReducedCoordinate, wakeCounterValue: f32);

    /// Returns the wake counter of the articulation in seconds.
    ///
    /// The wake counter of the articulation in seconds.
    ///
    /// This call may not be made during simulation, except in a split simulation in-between [`PxScene::fetchCollision`] and #PxScene::advance.
    pub fn PxArticulationReducedCoordinate_getWakeCounter(self_: *const PxArticulationReducedCoordinate) -> f32;

    /// Wakes up the articulation if it is sleeping.
    ///
    /// - The articulation will be woken up and might cause other touching objects to wake up as well during the next simulation step.
    /// - This will set the wake counter of the articulation to the value specified in [`PxSceneDesc::wakeCounterResetValue`].
    ///
    /// This call may only be made on articulations that are in a scene, and may not be made during simulation,
    /// except in a split simulation in-between [`PxScene::fetchCollision`] and #PxScene::advance.
    pub fn PxArticulationReducedCoordinate_wakeUp_mut(self_: *mut PxArticulationReducedCoordinate);

    /// Forces the articulation to sleep.
    ///
    /// - The articulation will stay asleep during the next simulation step if not touched by another non-sleeping actor.
    /// - This will set any applied force, the velocity, and the wake counter of all bodies in the articulation to zero.
    ///
    /// This call may not be made during simulation, and may only be made on articulations that are in a scene.
    pub fn PxArticulationReducedCoordinate_putToSleep_mut(self_: *mut PxArticulationReducedCoordinate);

    /// Adds a link to the articulation with default attribute values.
    ///
    /// The new link, or NULL if the link cannot be created.
    ///
    /// Creating a link is not allowed while the articulation is in a scene. In order to add a link,
    /// remove and then re-add the articulation to the scene.
    ///
    /// When the articulation is added to a scene, the root link adopts the specified pose. The pose of the
    /// root link is propagated through the ensemble of links from parent to child after accounting for each child's
    /// inbound joint frames and the joint positions set by PxArticulationJointReducedCoordinate::setJointPosition().
    /// As a consequence, the pose of each non-root link is automatically overwritten when adding the articulation to the scene.
    pub fn PxArticulationReducedCoordinate_createLink_mut(self_: *mut PxArticulationReducedCoordinate, parent: *mut PxArticulationLink, pose: *const PxTransform) -> *mut PxArticulationLink;

    /// Releases the articulation, and all its links and corresponding joints.
    ///
    /// Attached mimic joints and tendons are released automatically when the articulation is released.
    ///
    /// This call may not be made during simulation.
    ///
    /// This call does not release any PxArticulationCache instance that has been instantiated using [`createCache`]()
    pub fn PxArticulationReducedCoordinate_release_mut(self_: *mut PxArticulationReducedCoordinate);

    /// Returns the number of links in the articulation.
    ///
    /// The number of links.
    pub fn PxArticulationReducedCoordinate_getNbLinks(self_: *const PxArticulationReducedCoordinate) -> u32;

    /// Returns the set of links in the articulation in the order that they were added to the articulation using createLink.
    ///
    /// The order of the links may be different from the order in which the data is stored in the cache, see PxArticulationLink::getLinkIndex.
    ///
    /// The number of links written into the buffer.
    pub fn PxArticulationReducedCoordinate_getLinks(self_: *const PxArticulationReducedCoordinate, userBuffer: *mut *mut PxArticulationLink, bufferSize: u32, startIndex: u32) -> u32;

    /// Returns the number of shapes in the articulation.
    ///
    /// The number of shapes.
    pub fn PxArticulationReducedCoordinate_getNbShapes(self_: *const PxArticulationReducedCoordinate) -> u32;

    /// Sets a name string for the articulation that can be retrieved with getName().
    ///
    /// This is for debugging and is not used by the SDK. The string is not copied by the SDK,
    /// only the pointer is stored.
    pub fn PxArticulationReducedCoordinate_setName_mut(self_: *mut PxArticulationReducedCoordinate, name: *const std::ffi::c_char);

    /// Returns the name string set with setName().
    ///
    /// Name string associated with the articulation.
    pub fn PxArticulationReducedCoordinate_getName(self_: *const PxArticulationReducedCoordinate) -> *const std::ffi::c_char;

    /// Returns the axis-aligned bounding box enclosing the articulation.
    ///
    /// The articulation's bounding box.
    ///
    /// It is not allowed to use this method while the simulation is running, except in a split simulation
    /// during [`PxScene::collide`]() and up to #PxScene::advance(), and in PxContactModifyCallback or in contact report callbacks.
    pub fn PxArticulationReducedCoordinate_getWorldBounds(self_: *const PxArticulationReducedCoordinate, inflation: f32) -> PxBounds3;

    /// Returns the aggregate associated with the articulation.
    ///
    /// The aggregate associated with the articulation or NULL if the articulation does not belong to an aggregate.
    pub fn PxArticulationReducedCoordinate_getAggregate(self_: *const PxArticulationReducedCoordinate) -> *mut PxAggregate;

    /// Sets flags on the articulation.
    ///
    /// This call may not be made during simulation.
    pub fn PxArticulationReducedCoordinate_setArticulationFlags_mut(self_: *mut PxArticulationReducedCoordinate, flags: PxArticulationFlags);

    /// Raises or clears a flag on the articulation.
    ///
    /// This call may not be made during simulation.
    pub fn PxArticulationReducedCoordinate_setArticulationFlag_mut(self_: *mut PxArticulationReducedCoordinate, flag: PxArticulationFlag, value: bool);

    /// Returns the articulation's flags.
    ///
    /// The articulation's flags.
    pub fn PxArticulationReducedCoordinate_getArticulationFlags(self_: *const PxArticulationReducedCoordinate) -> PxArticulationFlags;

    /// Returns the total number of joint degrees-of-freedom (DOFs) of the articulation.
    ///
    /// - The six DOFs of the base of a floating-base articulation are not included in this count.
    /// - Example: Both a fixed-base and a floating-base double-pendulum with two revolute joints will have getDofs() == 2.
    /// - The return value is only valid for articulations that are in a scene.
    ///
    /// The number of joint DOFs, or 0xFFFFFFFF if the articulation is not in a scene.
    pub fn PxArticulationReducedCoordinate_getDofs(self_: *const PxArticulationReducedCoordinate) -> u32;

    /// Creates an articulation cache that can be used to read and write internal articulation data.
    ///
    /// - When the structure of the articulation changes (e.g. adding a link) after the cache was created,
    /// the cache needs to be released and recreated.
    /// - Free the memory allocated for the cache by calling the release() method on the cache.
    /// - Caches can only be created by articulations that are in a scene.
    ///
    /// The cache, or NULL if the articulation is not in a scene.
    pub fn PxArticulationReducedCoordinate_createCache(self_: *const PxArticulationReducedCoordinate) -> *mut PxArticulationCache;

    /// Returns the size of the articulation cache in bytes.
    ///
    /// - The size does not include: the user-allocated memory for the coefficient matrix or lambda values;
    /// the scratch-related memory/members; and the cache version. See comment in [`PxArticulationCache`].
    /// - The return value is only valid for articulations that are in a scene.
    ///
    /// The byte size of the cache, or 0xFFFFFFFF if the articulation is not in a scene.
    pub fn PxArticulationReducedCoordinate_getCacheDataSize(self_: *const PxArticulationReducedCoordinate) -> u32;

    /// Zeroes all data in the articulation cache, except user-provided and scratch memory, and cache version.
    ///
    /// This call may only be made on articulations that are in a scene.
    pub fn PxArticulationReducedCoordinate_zeroCache(self_: *const PxArticulationReducedCoordinate, cache: *mut PxArticulationCache);

    /// Applies the data in the cache to the articulation.
    ///
    /// This call wakes the articulation if it is sleeping, and the autowake parameter is true (default) or:
    /// - a nonzero joint velocity is applied or
    /// - a nonzero joint force is applied or
    /// - a nonzero root velocity is applied
    ///
    /// This call may only be made on articulations that are in a scene, and may not be made during simulation.
    ///
    /// Calling applyCache(cache, PxArticulationCacheFlag::eROOT_TRANSFORM) has the same outcome as calling
    /// PxArticulationReducedCoordinate::setRootGlobalPose() followed by PxArticulationReducedCoordinate::updateKinematic(PxArticulationKinematicFlag::ePOSITION).
    /// Similarly, calling applyCache(cache, PxArticulationCacheFlag::eROOT_VELOCITIES) is the cache equivalent of calling
    /// PxArticulationReducedCoordinate::setRootLinearVelocity() followed by PxArticulationReducedCoordinate::updateKinematic(PxArticulationKinematicFlag::eVELOCITY).
    /// Joint positions follow a similar pattern with applyCache(cache, PxArticulationCacheFlag::ePOSITION) having the same outcome as callling
    /// PxArticulationJointReducedCoordinate::setJointPosition() followed by PxArticulationReducedCoordinate::updateKinematic(PxArticulationKinematicFlag::ePOSITION).
    /// Finally, joint velocities updated with applyCache(PxArticulationCacheFlag::eVELOCITY) will produce the same outcome as calling
    /// PxArticulationJointReducedCoordinate::setJointVelocity() followed by PxArticulationReducedCoordinate::updateKinematic(PxArticulationKinematicFlag::eVELOCITY).
    ///
    /// This method should not be used if the direct GPU API is enabled. See [`PxDirectGPUAPI`] for the details.
    pub fn PxArticulationReducedCoordinate_applyCache_mut(self_: *mut PxArticulationReducedCoordinate, cache: *mut PxArticulationCache, flags: PxArticulationCacheFlags, autowake: bool);

    /// Copies internal data of the articulation to the cache.
    ///
    /// This call may only be made on articulations that are in a scene, and may not be made during simulation.
    ///
    /// This method should not be used if the direct GPU API is enabled. See [`PxDirectGPUAPI`] for the details.
    pub fn PxArticulationReducedCoordinate_copyInternalStateToCache(self_: *const PxArticulationReducedCoordinate, cache: *mut PxArticulationCache, flags: PxArticulationCacheFlags);

    /// Converts maximal-coordinate joint DOF data to reduced coordinates.
    ///
    /// - Indexing into the maximal joint DOF data is via the link's low-level index minus 1 (the root link is not included).
    /// - The reduced-coordinate data follows the cache indexing convention, see PxArticulationCache::jointVelocity.
    ///
    /// The articulation must be in a scene.
    ///
    /// This can be used as a helper function to prepare per joint cache data such as PxArticulationCache::jointVelocity.
    pub fn PxArticulationReducedCoordinate_packJointData(self_: *const PxArticulationReducedCoordinate, maximum: *const f32, reduced: *mut f32);

    /// Converts reduced-coordinate joint DOF data to maximal coordinates.
    ///
    /// - Indexing into the maximal joint DOF data is via the link's low-level index minus 1 (the root link is not included).
    /// - The reduced-coordinate data follows the cache indexing convention, see PxArticulationCache::jointVelocity.
    ///
    /// The articulation must be in a scene.
    pub fn PxArticulationReducedCoordinate_unpackJointData(self_: *const PxArticulationReducedCoordinate, reduced: *const f32, maximum: *mut f32);

    /// Prepares common articulation data based on articulation pose for inverse dynamics calculations.
    ///
    /// Usage:
    /// -[``] Set articulation pose (joint positions and base transform) via articulation cache and applyCache().
    /// -[``] Call commonInit.
    /// -[``] Call inverse dynamics computation method.
    ///
    /// This call may only be made on articulations that are in a scene, and may not be made during simulation.
    pub fn PxArticulationReducedCoordinate_commonInit(self_: *const PxArticulationReducedCoordinate);

    /// Computes the forces required to counteract gravitational forces for the given articulation pose.
    ///
    /// In the case of a fixed-base articulation, the gravity compensation force accounts for the gravity on all the links and provides
    /// the force required to compensate the gravitational forces for all the joint DoFs.
    /// The indexing follows the internal DOF index order, see PxArticulationCache::jointVelocity.
    ///
    /// In the case of a floating-base articulation, the gravity compensation force also accounts for the gravity on the root link and also provides
    /// the force on the root required to compensate its gravitational force. The indexing is:
    /// | Root force X | Root force Y | Root force Z | Root torque X | Root torque Y | Root torque Z | Force/Torque DOF 0 | ... | Force/Torque DOF N |
    ///
    /// - Inputs:	Articulation pose (joint positions + base transform).
    /// - Outputs:	Forces to counteract gravity (in cache).
    ///
    /// - The joint forces returned are determined purely by gravity for the articulation in the current joint and base pose, and joints at rest;
    /// i.e. external forces, joint velocities, and joint accelerations are set to zero. Joint drives are also not considered in the computation.
    /// - commonInit() must be called before the computation, and after setting the articulation pose via applyCache().
    ///
    /// This call may only be made on articulations that are in a scene, and may not be made during simulation.
    pub fn PxArticulationReducedCoordinate_computeGravityCompensation(self_: *const PxArticulationReducedCoordinate, cache: *mut PxArticulationCache);

    /// Computes the joint DOF forces (and root force) required to counteract Coriolis and centrifugal forces for the given articulation state.
    ///
    /// In the case of a fixed-base articulation, the Coriolis and centrifugal compensation force accounts for forces resulting to the current
    /// joint velocities. The indexing follows the internal DOF index order, see PxArticulationCache::jointVelocity.
    ///
    /// In the case of a floating-base articulation, the Coriolis and centrifugal compensation force also accounts for forces resulting to the current
    /// root velocity. The indexing is:
    /// | Root force X | Root force Y | Root force Z | Root torque X | Root torque Y | Root torque Z | Force/Torque DOF 0 | ... | Force/Torque DOF N |
    ///
    /// - Inputs:	Articulation state (joint positions and velocities (in cache), and base transform and spatial velocity).
    /// - Outputs:	Joint forces (and root force) to counteract Coriolis and centrifugal forces (in cache).
    ///
    /// - The forces returned are determined purely by the articulation's state; i.e. external forces, gravity, and joint accelerations are set to zero.
    /// Joint drives and potential damping terms, such as link angular or linear damping, or joint friction, are also not considered in the computation.
    /// - Prior to the computation, update/set the base spatial velocity with PxArticulationCache::rootLinkData and applyCache().
    /// - commonInit() must be called before the computation, and after setting the articulation pose via applyCache().
    ///
    /// This call may only be made on articulations that are in a scene, and may not be made during simulation.
    pub fn PxArticulationReducedCoordinate_computeCoriolisCompensation(self_: *const PxArticulationReducedCoordinate, cache: *mut PxArticulationCache);

    /// Computes the joint DOF forces required to counteract external spatial forces applied to articulation links.
    ///
    /// - Inputs:	External forces on links (in cache), articulation pose (joint positions + base transform).
    /// - Outputs:	Joint forces to counteract the external forces (in cache).
    ///
    /// - Only the external spatial forces provided in the cache and the articulation pose are considered in the computation.
    /// - The external spatial forces are with respect to the links' centers of mass, and not the actor's origin.
    /// - commonInit() must be called before the computation, and after setting the articulation pose via applyCache().
    ///
    /// This call may only be made on articulations that are in a scene, and may not be made during simulation.
    pub fn PxArticulationReducedCoordinate_computeGeneralizedExternalForce(self_: *const PxArticulationReducedCoordinate, cache: *mut PxArticulationCache);

    /// Computes the joint accelerations for the given articulation state and joint forces.
    ///
    /// - Inputs:	Joint forces (in cache) and articulation state (joint positions and velocities (in cache), and base transform and spatial velocity).
    /// - Outputs:	Joint accelerations (in cache).
    ///
    /// - The computation includes Coriolis terms and gravity. However, joint drives, external forces, and potential damping (link damping, friction) terms
    /// are not considered in the computation.
    /// - Prior to the computation, update/set the base spatial velocity with PxArticulationCache::rootLinkData and applyCache().
    /// - commonInit() must be called before the computation, and after setting the articulation pose via applyCache().
    ///
    /// This call may only be made on articulations that are in a scene, and may not be made during simulation.
    pub fn PxArticulationReducedCoordinate_computeJointAcceleration(self_: *const PxArticulationReducedCoordinate, cache: *mut PxArticulationCache);

    /// Computes the joint forces for the given articulation pose and joint accelerations, not considering gravity and velocity.
    ///
    /// - Inputs:	Joint accelerations (in cache).
    /// - Outputs:	Joint forces (in cache).
    ///
    /// - Gravity, Coriolis effects, joint drives and potential damping terms are not considered in the computation
    /// (for example, linear link damping or joint friction).
    /// - To compute the joint force for a different pose, the joint positions and root transform first need to be applied with applyCache() as this function ignores any values set to joint positions and root transform in the cache
    /// - commonInit() must be called before the computation, and after setting the articulation pose via applyCache().
    ///
    /// This call may only be made on articulations that are in a scene, and may not be made during simulation.
    pub fn PxArticulationReducedCoordinate_computeJointForce(self_: *const PxArticulationReducedCoordinate, cache: *mut PxArticulationCache);

    /// Compute the dense Jacobian for the articulation in world space, including the DOFs of a potentially floating base.
    ///
    /// This computes the dense representation of an inherently sparse matrix. Multiplication with this matrix maps
    /// joint space velocities to world-space linear and angular (i.e. spatial) velocities of the centers of mass of the links.
    ///
    /// This call may only be made on articulations that are in a scene, and may not be made during simulation.
    pub fn PxArticulationReducedCoordinate_computeDenseJacobian(self_: *const PxArticulationReducedCoordinate, cache: *mut PxArticulationCache, nRows: *mut u32, nCols: *mut u32);

    /// The API related to loop joints will be removed in a future version once a replacement is made available.
    ///
    /// Computes the coefficient matrix for contact forces.
    ///
    /// - The matrix dimension is getCoefficientMatrixSize() = getDofs() * getNbLoopJoints(), and the DOF (column) indexing follows the internal DOF order, see PxArticulationCache::jointVelocity.
    /// - Each column in the matrix is the joint forces effected by a contact based on impulse strength 1.
    /// - The user must allocate memory for PxArticulationCache::coefficientMatrix where the required size of the PxReal array is equal to getCoefficientMatrixSize().
    /// - commonInit() must be called before the computation, and after setting the articulation pose via applyCache().
    ///
    /// This call may only be made on articulations that are in a scene, and may not be made during simulation.
    pub fn PxArticulationReducedCoordinate_computeCoefficientMatrix(self_: *const PxArticulationReducedCoordinate, cache: *mut PxArticulationCache);

    /// The API related to loop joints will be removed in a future version once a replacement is made available.
    ///
    /// Computes the lambda values when the test impulse is 1.
    ///
    /// - The user must allocate memory for PxArticulationCache::lambda where the required size of the PxReal array is equal to getNbLoopJoints().
    /// - commonInit() must be called before the computation, and after setting the articulation pose via applyCache().
    ///
    /// True if convergence was achieved within maxIter; False if convergence was not achieved or the operation failed otherwise.
    ///
    /// This call may only be made on articulations that are in a scene, and may not be made during simulation.
    pub fn PxArticulationReducedCoordinate_computeLambda(self_: *const PxArticulationReducedCoordinate, cache: *mut PxArticulationCache, initialState: *mut PxArticulationCache, jointTorque: *const f32, maxIter: u32) -> bool;

    /// Compute the mass matrix M that maps accelerations to forces: forces = M * accelerations.
    ///
    /// In the case of a fixed-base articulation, the mass matrix maps joint accelerations to joint forces.
    /// The indexing follows the internal DOF index order, see PxArticulationCache::jointVelocity.
    ///
    /// In the case of a floating-base articulation, the mass matrix also includes terms required to map root accelerations
    /// to root forces. The mass matrix should be used with accelerations and forces that follows the indexing below:
    /// | Root force X       |     | Root linear acceleration X  |
    /// | Root force Y	     |     | Root linear acceleration Y  |
    /// | Root force Z       |     | Root linear acceleration Z  |
    /// | Root torque X      |     | Root angular acceleration X |
    /// | Root torque Y      |     | Root angular acceleration Y |
    /// | Root torque Z      | = M | Root angular acceleration Z |
    /// | Force/Torque DOF 0 |     | Joint acceleration 0        |
    /// | Force/Torque DOF 1 |     | Joint acceleration 1        |
    /// | ...                |     | ...                         |
    /// | Force/Torque DOF N |     | Joint acceleration N        |
    ///
    /// - Inputs:	Articulation pose (joint positions and base transform).
    /// - Outputs:	Mass matrix (in cache).
    ///
    /// commonInit() must be called before the computation, and after setting the articulation pose via applyCache().
    ///
    /// This call may only be made on articulations that are in a scene, and may not be made during simulation.
    ///
    /// The mass matrix is indexed [nCols * row + column].
    pub fn PxArticulationReducedCoordinate_computeMassMatrix(self_: *const PxArticulationReducedCoordinate, cache: *mut PxArticulationCache);

    /// Compute the articulation's center of mass.
    ///
    /// The articulation's center of mass given either in the world frame (rootFrame = false) or in the root frame
    /// (rootFrame = true). PxVec3(0.0f) is returned if the articulation is not in a scene or the call is made during simulation.
    ///
    /// This call may only be made on articulations that are in a scene, and may not be made during simulation.
    pub fn PxArticulationReducedCoordinate_computeArticulationCOM(self_: *const PxArticulationReducedCoordinate, rootFrame: bool) -> PxVec3;

    /// Compute the centroidal momentum matrix and corresponding bias force of an articulation.
    ///
    /// - Inputs:	Articulation state (joint positions and velocities, and base transform and spatial velocity),
    /// articulation mass matrix, Coriolis and Centrifugal compensation forces.
    /// - Outputs:	Centroidal momentum matrix and bias force (in cache).
    ///
    /// commonInit(), computeMassMatrix() and computeCoriolisCompensation() must be called before the computation,
    /// and after setting the articulation pose and velocities via applyCache().
    ///
    /// This call may only be made on articulations that are in a scene, and may not be made during simulation.
    /// This call may also only be made for floating-base articulations.
    pub fn PxArticulationReducedCoordinate_computeCentroidalMomentumMatrix(self_: *const PxArticulationReducedCoordinate, cache: *mut PxArticulationCache);

    /// The API related to loop joints will be removed in a future version once a replacement is made available.
    ///
    /// Adds a loop joint to the articulation system for inverse dynamics.
    ///
    /// This call may not be made during simulation.
    pub fn PxArticulationReducedCoordinate_addLoopJoint_mut(self_: *mut PxArticulationReducedCoordinate, joint: *mut PxConstraint);

    /// The API related to loop joints will be removed in a future version once a replacement is made available.
    ///
    /// Removes a loop joint from the articulation for inverse dynamics.
    ///
    /// This call may not be made during simulation.
    pub fn PxArticulationReducedCoordinate_removeLoopJoint_mut(self_: *mut PxArticulationReducedCoordinate, joint: *mut PxConstraint);

    /// The API related to loop joints will be removed in a future version once a replacement is made available.
    ///
    /// Returns the number of loop joints in the articulation for inverse dynamics.
    ///
    /// The number of loop joints.
    pub fn PxArticulationReducedCoordinate_getNbLoopJoints(self_: *const PxArticulationReducedCoordinate) -> u32;

    /// The API related to loop joints will be removed in a future version once a replacement is made available.
    ///
    /// Returns the set of loop constraints (i.e. joints) in the articulation.
    ///
    /// The number of constraints written into the buffer.
    pub fn PxArticulationReducedCoordinate_getLoopJoints(self_: *const PxArticulationReducedCoordinate, userBuffer: *mut *mut PxConstraint, bufferSize: u32, startIndex: u32) -> u32;

    /// The API related to loop joints will be removed in a future version once a replacement is made available.
    ///
    /// Returns the required size of the coefficient matrix in the articulation.
    ///
    /// Size of the coefficient matrix (equal to getDofs() * getNbLoopJoints()).
    ///
    /// This call may only be made on articulations that are in a scene.
    pub fn PxArticulationReducedCoordinate_getCoefficientMatrixSize(self_: *const PxArticulationReducedCoordinate) -> u32;

    /// Sets the root link transform in the world frame.
    ///
    /// - Use updateKinematic() after all state updates to the articulation via non-cache API such as this method,
    /// in order to update link states for the next simulation frame or querying.
    ///
    /// This call may not be made during simulation.
    ///
    /// PxArticulationCache::rootLinkData similarly allows the root link pose to be updated and potentially offers better performance
    /// if the root link pose is to be updated along with other state variables.
    ///
    /// This method should not be used after the direct GPU API has been enabled and initialized. See [`PxDirectGPUAPI`] for the details.
    pub fn PxArticulationReducedCoordinate_setRootGlobalPose_mut(self_: *mut PxArticulationReducedCoordinate, pose: *const PxTransform, autowake: bool);

    /// Returns the root link transform (world to actor frame).
    ///
    /// The root link transform.
    ///
    /// This call is not allowed while the simulation is running except in a split simulation during [`PxScene::collide`]() and up to #PxScene::advance(),
    /// and in PxContactModifyCallback or in contact report callbacks.
    ///
    /// PxArticulationCache::rootLinkData similarly allows the root link pose to be queried and potentially offers better performance if the root
    /// link pose is to be queried along with other state variables.
    ///
    /// This method should not be used after the direct GPU API has been enabled and initialized. See [`PxDirectGPUAPI`] for the details.
    pub fn PxArticulationReducedCoordinate_getRootGlobalPose(self_: *const PxArticulationReducedCoordinate) -> PxTransform;

    /// Sets the root link linear center-of-mass velocity.
    ///
    /// - The linear velocity is with respect to the link's center of mass and not the actor frame origin.
    /// - The articulation is woken up if the input velocity is nonzero (ignoring autowake) and the articulation is in a scene.
    /// - Use updateKinematic() after all state updates to the articulation via non-cache API such as this method,
    /// in order to update link states for the next simulation frame or querying.
    ///
    /// This call may not be made during simulation, except in a split simulation in-between [`PxScene::fetchCollision`] and #PxScene::advance.
    ///
    /// PxArticulationCache::rootLinkData similarly allows the root link linear velocity to be updated and potentially offers better performance
    /// if the root link linear velocity is to be updated along with other state variables.
    ///
    /// This method should not be used after the direct GPU API has been enabled and initialized. See [`PxDirectGPUAPI`] for the details.
    pub fn PxArticulationReducedCoordinate_setRootLinearVelocity_mut(self_: *mut PxArticulationReducedCoordinate, linearVelocity: *const PxVec3, autowake: bool);

    /// Gets the root link center-of-mass linear velocity.
    ///
    /// - The linear velocity is with respect to the link's center of mass and not the actor frame origin.
    ///
    /// The root link center-of-mass linear velocity.
    ///
    /// This call is not allowed while the simulation is running except in a split simulation during [`PxScene::collide`]() and up to #PxScene::advance(),
    /// and in PxContactModifyCallback or in contact report callbacks.
    ///
    /// PxArticulationCache::rootLinkData similarly allows the root link linear velocity to be queried and potentially offers better performance
    /// if the root link linear velocity is to be queried along with other state variables.
    ///
    /// This method should not be used after the direct GPU API has been enabled and initialized. See [`PxDirectGPUAPI`] for the details.
    pub fn PxArticulationReducedCoordinate_getRootLinearVelocity(self_: *const PxArticulationReducedCoordinate) -> PxVec3;

    /// Sets the root link angular velocity.
    ///
    /// - The articulation is woken up if the input velocity is nonzero (ignoring autowake) and the articulation is in a scene.
    /// - Use updateKinematic() after all state updates to the articulation via non-cache API such as this method,
    /// in order to update link states for the next simulation frame or querying.
    ///
    /// This call may not be made during simulation, except in a split simulation in-between [`PxScene::fetchCollision`] and #PxScene::advance.
    ///
    /// PxArticulationCache::rootLinkData similarly allows the root link angular velocity to be updated and potentially offers better performance
    /// if the root link angular velocity is to be updated along with other state variables.
    ///
    /// This method should not be used after the direct GPU API has been enabled and initialized. See [`PxDirectGPUAPI`] for the details.
    pub fn PxArticulationReducedCoordinate_setRootAngularVelocity_mut(self_: *mut PxArticulationReducedCoordinate, angularVelocity: *const PxVec3, autowake: bool);

    /// Gets the root link angular velocity.
    ///
    /// The root link angular velocity.
    ///
    /// This call is not allowed while the simulation is running except in a split simulation during [`PxScene::collide`]() and up to #PxScene::advance(),
    /// and in PxContactModifyCallback or in contact report callbacks.
    ///
    /// PxArticulationCache::rootLinkData similarly allows the root link angular velocity to be queried and potentially offers better performance
    /// if the root link angular velocity is to be queried along with other state variables.
    ///
    /// This method should not be used after the direct GPU API has been enabled and initialized. See [`PxDirectGPUAPI`] for the details.
    pub fn PxArticulationReducedCoordinate_getRootAngularVelocity(self_: *const PxArticulationReducedCoordinate) -> PxVec3;

    /// Returns the (classical) link acceleration in world space for the given low-level link index.
    ///
    /// - The returned acceleration is not a spatial, but a classical, i.e. body-fixed acceleration (https://en.wikipedia.org/wiki/Spatial_acceleration).
    /// - The (linear) acceleration is with respect to the link's center of mass and not the actor frame origin.
    ///
    /// The link's center-of-mass classical acceleration, or 0 if the call is made before the articulation participated in a first simulation step.
    ///
    /// This call may only be made on articulations that are in a scene. It is not allowed to use this method while the simulation
    /// is running.  The exceptions to this rule are a split simulation during [`PxScene::collide`]() and up to #PxScene::advance();
    /// in PxContactModifyCallback; and in contact report callbacks.
    ///
    /// This method should not be used after the direct GPU API has been enabled and initialized. See [`PxDirectGPUAPI`] for the details.
    pub fn PxArticulationReducedCoordinate_getLinkAcceleration_mut(self_: *mut PxArticulationReducedCoordinate, linkId: u32) -> PxSpatialVelocity;

    /// Returns the GPU articulation index.
    ///
    /// The GPU index, or 0xFFFFFFFF if the articulation is not in a scene.
    pub fn PxArticulationReducedCoordinate_getGPUIndex(self_: *const PxArticulationReducedCoordinate) -> u32;

    /// Creates a spatial tendon to attach to the articulation with default attribute values.
    ///
    /// The new spatial tendon.
    ///
    /// Creating a spatial tendon is not allowed while the articulation is in a scene. In order to
    /// add the tendon, remove and then re-add the articulation to the scene.
    ///
    /// The spatial tendon is released with PxArticulationReducedCoordinate::release()
    pub fn PxArticulationReducedCoordinate_createSpatialTendon_mut(self_: *mut PxArticulationReducedCoordinate) -> *mut PxArticulationSpatialTendon;

    /// Creates a fixed tendon to attach to the articulation with default attribute values.
    ///
    /// The new fixed tendon.
    ///
    /// Creating a fixed tendon is not allowed while the articulation is in a scene. In order to
    /// add the tendon, remove and then re-add the articulation to the scene.
    ///
    /// The fixed tendon is released with PxArticulationReducedCoordinate::release()
    pub fn PxArticulationReducedCoordinate_createFixedTendon_mut(self_: *mut PxArticulationReducedCoordinate) -> *mut PxArticulationFixedTendon;

    /// Returns the spatial tendons attached to the articulation.
    ///
    /// The order of the tendons in the buffer is not necessarily identical to the order in which the tendons were added to the articulation.
    ///
    /// The number of tendons written into the buffer.
    pub fn PxArticulationReducedCoordinate_getSpatialTendons(self_: *const PxArticulationReducedCoordinate, userBuffer: *mut *mut PxArticulationSpatialTendon, bufferSize: u32, startIndex: u32) -> u32;

    /// Returns the number of spatial tendons in the articulation.
    ///
    /// The number of tendons.
    pub fn PxArticulationReducedCoordinate_getNbSpatialTendons(self_: *const PxArticulationReducedCoordinate) -> u32;

    /// Returns the fixed tendons attached to the articulation.
    ///
    /// The order of the tendons in the buffer is not necessarily identical to the order in which the tendons were added to the articulation.
    ///
    /// The number of tendons written into the buffer.
    pub fn PxArticulationReducedCoordinate_getFixedTendons(self_: *const PxArticulationReducedCoordinate, userBuffer: *mut *mut PxArticulationFixedTendon, bufferSize: u32, startIndex: u32) -> u32;

    /// Returns the number of fixed tendons in the articulation.
    ///
    /// The number of tendons.
    pub fn PxArticulationReducedCoordinate_getNbFixedTendons(self_: *const PxArticulationReducedCoordinate) -> u32;

    /// Create a mimic joint that will enforce a relationship between two joints.
    ///
    /// If naturalFrequency is less than or equal to zero it is assumed that the mimic joint has no compliance and is a hard constraint.
    ///
    /// If dampingRatio is less than or equal to zero it is assumed that the mimic joint has no compliance and is a hard constraint.
    ///
    /// In the absence of compliance, the mimic joint enforces the rule: qA + gearRatio*qB + offset = 0 with qA denoting the
    /// joint position of the specified degree of freedom of jointA and qB denoting the joint position of the specified degree of freedom of jointB.
    ///
    /// Larger values of naturalFrequency and dampingRatio will make the mimic joint stiffer and more akin to a hard constraint.
    ///
    /// A damping ratio less than 1.0 is not recommended.
    ///
    /// If dampingRatio is less than or equal to zero and naturalFrequency greater than zero, the mimic joint will behave as a hard constraint.
    /// If dampingRatio is greater than zero and naturalFrequency less than or equal to zero, the mimic joint will also behave as a hard constraint.
    pub fn PxArticulationReducedCoordinate_createMimicJoint_mut(self_: *mut PxArticulationReducedCoordinate, jointA: *const PxArticulationJointReducedCoordinate, axisA: PxArticulationAxis, jointB: *const PxArticulationJointReducedCoordinate, axisB: PxArticulationAxis, gearRatio: f32, offset: f32, naturalFrequency: f32, dampingRatio: f32) -> *mut PxArticulationMimicJoint;

    /// Returns the mimic joints added to the articulation.
    ///
    /// The order of the mimic joints in the buffer is not necessarily identical to the order in which the mimic joints were added to the articulation.
    ///
    /// The number of mimic joints written into the buffer.
    pub fn PxArticulationReducedCoordinate_getMimicJoints(self_: *const PxArticulationReducedCoordinate, userBuffer: *mut *mut PxArticulationMimicJoint, bufferSize: u32, startIndex: u32) -> u32;

    /// Returns the number of mimic joints in the articulation.
    ///
    /// The number of mimic joints.
    pub fn PxArticulationReducedCoordinate_getNbMimicJoints(self_: *const PxArticulationReducedCoordinate) -> u32;

    /// Update link velocities and/or positions in the articulation.
    ///
    /// An alternative that potentially offers better performance is to use the PxArticulationCache API.
    ///
    /// If the application updates the root state (position and velocity) or joint state via any combination of
    /// the non-cache API calls
    ///
    /// - setRootGlobalPose(), setRootLinearVelocity(), setRootAngularVelocity()
    /// - PxArticulationJointReducedCoordinate::setJointPosition(), PxArticulationJointReducedCoordinate::setJointVelocity()
    ///
    /// the application needs to call this method after the state setting in order to update the link states for
    /// the next simulation frame or querying.
    ///
    /// Use
    /// - PxArticulationKinematicFlag::ePOSITION after any changes to the articulation root or joint positions using non-cache API calls. Updates links' positions and velocities.
    /// - PxArticulationKinematicFlag::eVELOCITY after velocity-only changes to the articulation root or joints using non-cache API calls. Updates links' velocities only.
    ///
    /// This call may only be made on articulations that are in a scene, and may not be made during simulation.
    pub fn PxArticulationReducedCoordinate_updateKinematic_mut(self_: *mut PxArticulationReducedCoordinate, flags: PxArticulationKinematicFlags);

    /// Returns the string name of the dynamic type.
    ///
    /// The string name.
    pub fn PxArticulationReducedCoordinate_getConcreteTypeName(self_: *const PxArticulationReducedCoordinate) -> *const std::ffi::c_char;

    /// Gets the parent articulation link of this joint.
    ///
    /// The parent link.
    pub fn PxArticulationJointReducedCoordinate_getParentArticulationLink(self_: *const PxArticulationJointReducedCoordinate) -> *mut PxArticulationLink;

    /// Sets the joint pose in the parent link actor frame.
    ///
    /// This call is not allowed while the simulation is running.
    pub fn PxArticulationJointReducedCoordinate_setParentPose_mut(self_: *mut PxArticulationJointReducedCoordinate, pose: *const PxTransform);

    /// Gets the joint pose in the parent link actor frame.
    ///
    /// The joint pose.
    pub fn PxArticulationJointReducedCoordinate_getParentPose(self_: *const PxArticulationJointReducedCoordinate) -> PxTransform;

    /// Gets the child articulation link of this joint.
    ///
    /// The child link.
    pub fn PxArticulationJointReducedCoordinate_getChildArticulationLink(self_: *const PxArticulationJointReducedCoordinate) -> *mut PxArticulationLink;

    /// Sets the joint pose in the child link actor frame.
    ///
    /// This call is not allowed while the simulation is running.
    pub fn PxArticulationJointReducedCoordinate_setChildPose_mut(self_: *mut PxArticulationJointReducedCoordinate, pose: *const PxTransform);

    /// Gets the joint pose in the child link actor frame.
    ///
    /// The joint pose.
    pub fn PxArticulationJointReducedCoordinate_getChildPose(self_: *const PxArticulationJointReducedCoordinate) -> PxTransform;

    /// Sets the joint type (e.g. revolute).
    ///
    /// Setting the joint type is not allowed while the articulation is in a scene.
    /// In order to amend the joint type, remove and then re-add the articulation to the scene.
    ///
    /// Default:
    /// PxArticulationJointType::eUNDEFINED
    pub fn PxArticulationJointReducedCoordinate_setJointType_mut(self_: *mut PxArticulationJointReducedCoordinate, jointType: PxArticulationJointType);

    /// Gets the joint type.
    ///
    /// The joint type.
    pub fn PxArticulationJointReducedCoordinate_getJointType(self_: *const PxArticulationJointReducedCoordinate) -> PxArticulationJointType;

    /// Sets the joint motion for a given axis.
    ///
    /// Setting the motion of joint axes is not allowed while the articulation is in a scene.
    /// In order to set the motion, remove and then re-add the articulation to the scene.
    ///
    /// Default:
    /// PxArticulationMotion::eLOCKED
    pub fn PxArticulationJointReducedCoordinate_setMotion_mut(self_: *mut PxArticulationJointReducedCoordinate, axis: PxArticulationAxis, motion: PxArticulationMotion);

    /// Returns the joint motion for the given axis.
    ///
    /// The joint motion of the given axis.
    pub fn PxArticulationJointReducedCoordinate_getMotion(self_: *const PxArticulationJointReducedCoordinate, axis: PxArticulationAxis) -> PxArticulationMotion;

    /// Sets the joint limits for a given axis.
    ///
    /// - The motion of the corresponding axis should be set to PxArticulationMotion::eLIMITED in order for the limits to be enforced.
    /// - The lower limit should be strictly smaller than the higher limit. If the limits should be equal, use PxArticulationMotion::eLOCKED
    /// and an appropriate offset in the parent/child joint frames.
    ///
    /// This call is not allowed while the simulation is running.
    ///
    /// For PxArticulationJointType::eSPHERICAL, limit.min and limit.max must both be in range [-Pi, Pi].
    ///
    /// For PxArticulationJointType::eREVOLUTE, limit.min and limit.max must both be in range [-2*Pi, 2*Pi].
    ///
    /// For PxArticulationJointType::eREVOLUTE_UNWRAPPED, limit.min and limit.max must both be in range [-PX_MAX_REAL, PX_MAX_REAL].
    ///
    /// For PxArticulationJointType::ePRISMATIC, limit.min and limit.max must both be in range [-PX_MAX_REAL, PX_MAX_REAL].
    ///
    /// Default:
    /// (0,0)
    pub fn PxArticulationJointReducedCoordinate_setLimitParams_mut(self_: *mut PxArticulationJointReducedCoordinate, axis: PxArticulationAxis, limit: *const PxArticulationLimit);

    /// Returns the joint limits for a given axis.
    ///
    /// The joint limits.
    pub fn PxArticulationJointReducedCoordinate_getLimitParams(self_: *const PxArticulationJointReducedCoordinate, axis: PxArticulationAxis) -> PxArticulationLimit;

    /// Configures a joint drive for the given axis.
    ///
    /// See PxArticulationDrive for parameter details; and the manual for further information, and the drives' implicit spring-damper (i.e. PD control) implementation in particular.
    ///
    /// This call is not allowed while the simulation is running.
    ///
    /// Default:
    /// PxArticulationDrive(0.0f, 0.0f, 0.0f, PxArticulationDriveType::eNONE)
    pub fn PxArticulationJointReducedCoordinate_setDriveParams_mut(self_: *mut PxArticulationJointReducedCoordinate, axis: PxArticulationAxis, drive: *const PxArticulationDrive);

    /// Gets the joint drive configuration for the given axis.
    ///
    /// The drive parameters.
    pub fn PxArticulationJointReducedCoordinate_getDriveParams(self_: *const PxArticulationJointReducedCoordinate, axis: PxArticulationAxis) -> PxArticulationDrive;

    /// Sets the joint drive position target for the given axis.
    ///
    /// The target units are linear units (equivalent to scene units) for a translational axis, or rad for a rotational axis.
    ///
    /// This call is not allowed while the simulation is running.
    ///
    /// This method should not be used after the direct GPU API has been enabled and initialized. See [`PxDirectGPUAPI`] for the details.
    ///
    /// For spherical joints, target must be in range [-Pi, Pi].
    ///
    /// The target is specified in the parent frame of the joint. If Gp, Gc are the parent and child actor poses in the world frame and Lp, Lc are the parent and child joint frames expressed in the parent and child actor frames then the joint will drive the parent and child links to poses that obey Gp * Lp * J = Gc * Lc. For joints restricted to angular motion, J has the form PxTransform(PxVec3(PxZero), PxExp(PxVec3(twistTarget, swing1Target, swing2Target))).  For joints restricted to linear motion, J has the form PxTransform(PxVec3(XTarget, YTarget, ZTarget), PxQuat(PxIdentity)).
    ///
    /// For spherical joints with more than 1 degree of freedom, the joint target angles taken together can collectively represent a rotation of greater than Pi around a vector. When this happens the rotation that matches the joint drive target is not the shortest path rotation.  The joint pose J that is the outcome after driving to the target pose will always be the equivalent of the shortest path rotation.
    ///
    /// Default:
    /// 0.0
    pub fn PxArticulationJointReducedCoordinate_setDriveTarget_mut(self_: *mut PxArticulationJointReducedCoordinate, axis: PxArticulationAxis, target: f32, autowake: bool);

    /// Returns the joint drive position target for the given axis.
    ///
    /// This method should not be used after the direct GPU API has been enabled and initialized. See [`PxDirectGPUAPI`] for the details.
    ///
    /// The target position.
    pub fn PxArticulationJointReducedCoordinate_getDriveTarget(self_: *const PxArticulationJointReducedCoordinate, axis: PxArticulationAxis) -> f32;

    /// Sets the joint drive velocity target for the given axis.
    ///
    /// The target units are linear units (equivalent to scene units) per second for a translational axis, or radians per second for a rotational axis.
    ///
    /// This call is not allowed while the simulation is running.
    ///
    /// This method should not be used after the direct GPU API has been enabled and initialized. See [`PxDirectGPUAPI`] for the details.
    ///
    /// Default:
    /// 0.0
    pub fn PxArticulationJointReducedCoordinate_setDriveVelocity_mut(self_: *mut PxArticulationJointReducedCoordinate, axis: PxArticulationAxis, targetVel: f32, autowake: bool);

    /// Returns the joint drive velocity target for the given axis.
    ///
    /// This method should not be used after the direct GPU API has been enabled and initialized. See [`PxDirectGPUAPI`] for the details.
    ///
    /// The target velocity.
    pub fn PxArticulationJointReducedCoordinate_getDriveVelocity(self_: *const PxArticulationJointReducedCoordinate, axis: PxArticulationAxis) -> f32;

    /// Sets the joint armature for the given axis.
    ///
    /// - The armature is directly added to the joint-space spatial inertia of the corresponding axis.
    /// - The armature is in mass units for a prismatic (i.e. linear) joint, and in mass units * (scene linear units)^2 for a rotational joint.
    ///
    /// This call is not allowed while the simulation is running.
    ///
    /// Default:
    /// 0.0
    pub fn PxArticulationJointReducedCoordinate_setArmature_mut(self_: *mut PxArticulationJointReducedCoordinate, axis: PxArticulationAxis, armature: f32);

    /// Gets the joint armature for the given axis.
    ///
    /// The armature set on the given axis.
    pub fn PxArticulationJointReducedCoordinate_getArmature(self_: *const PxArticulationJointReducedCoordinate, axis: PxArticulationAxis) -> f32;

    /// Sets the joint friction coefficient, which applies to all joint axes.
    ///
    /// - The joint friction is unitless and relates the magnitude of the spatial force [F_trans, T_trans] transmitted from parent to child link to
    /// the maximal friction force F_resist that may be applied by the solver to resist joint motion, per axis; i.e. |F_resist|
    /// <
    /// = coefficient * (|F_trans| + |T_trans|),
    /// where F_resist may refer to a linear force or torque depending on the joint axis.
    /// - The simulated friction effect is therefore similar to static and Coulomb friction. In order to simulate dynamic joint friction, use a joint drive with
    /// zero stiffness and zero velocity target, and an appropriately dimensioned damping parameter.
    ///
    /// This call is not allowed while the simulation is running.
    ///
    /// Default:
    /// 0.05
    pub fn PxArticulationJointReducedCoordinate_setFrictionCoefficient_mut(self_: *mut PxArticulationJointReducedCoordinate, coefficient: f32);

    /// Gets the joint friction coefficient.
    ///
    /// The joint friction coefficient.
    pub fn PxArticulationJointReducedCoordinate_getFrictionCoefficient(self_: *const PxArticulationJointReducedCoordinate) -> f32;

    /// Configures joint friction.
    ///
    /// See PxJointFrictionParams for parameter details; and the manual for further information. The new friction model is applied to all axes where setFrictionParams() has been called.
    /// For axes where setFrictionParams() hasn't been used, the deprecated friction model remains in effect. See setFrictionCoefficient().
    ///
    /// This call is not allowed while the simulation is running.
    ///
    /// Default:
    /// PxJointFrictionParams(0.0f, 0.0f, 0.0f)
    pub fn PxArticulationJointReducedCoordinate_setFrictionParams_mut(self_: *mut PxArticulationJointReducedCoordinate, axis: PxArticulationAxis, jointFrictionParams: *const PxJointFrictionParams);

    /// Gets per-axis joint friction parameters struct.
    ///
    /// The joint friction parameters.
    pub fn PxArticulationJointReducedCoordinate_getFrictionParams(self_: *const PxArticulationJointReducedCoordinate, axis: PxArticulationAxis) -> PxJointFrictionParams;

    /// Sets the maximal joint velocity enforced for all axes.
    ///
    /// - The solver will apply appropriate joint-space impulses in order to enforce the per-axis joint-velocity limit.
    /// - The velocity units are linear units (equivalent to scene units) per second for a translational axis, or radians per second for a rotational axis.
    ///
    /// This call is not allowed while the simulation is running.
    ///
    /// Default:
    /// 100.0
    pub fn PxArticulationJointReducedCoordinate_setMaxJointVelocity_mut(self_: *mut PxArticulationJointReducedCoordinate, maxJointV: f32);

    /// Gets the maximal joint velocity enforced for all axes.
    ///
    /// The maximal per-axis joint velocity.
    pub fn PxArticulationJointReducedCoordinate_getMaxJointVelocity(self_: *const PxArticulationJointReducedCoordinate) -> f32;

    /// Sets the maximal joint velocity enforced for the given axis.
    ///
    /// - The solver will apply appropriate joint-space impulses in order to enforce the per-axis joint-velocity limit.
    /// - The velocity units are linear units (equivalent to scene units) per second for a translational axis, or radians per second for a rotational axis.
    ///
    /// This call is not allowed while the simulation is running.
    ///
    /// Default:
    /// 100.0
    pub fn PxArticulationJointReducedCoordinate_setMaxJointVelocity_mut_1(self_: *mut PxArticulationJointReducedCoordinate, axis: PxArticulationAxis, maxJointV: f32);

    /// Gets the maximal joint velocity enforced for the given axis.
    ///
    /// The maximal joint velocity for the given axis.
    pub fn PxArticulationJointReducedCoordinate_getMaxJointVelocity_1(self_: *const PxArticulationJointReducedCoordinate, axis: PxArticulationAxis) -> f32;

    /// Sets the joint position for the given axis.
    ///
    /// - For performance, prefer PxArticulationCache::jointPosition to set joint positions in a batch articulation state update.
    /// - Use PxArticulationReducedCoordinate::updateKinematic after all state updates to the articulation via non-cache API such as this method,
    /// in order to update link states for the next simulation frame or querying.
    ///
    /// This call is not allowed while the simulation is running.
    ///
    /// This method should not be used after the direct GPU API has been enabled and initialized. See [`PxDirectGPUAPI`] for the details.
    ///
    /// For PxArticulationJointType::eSPHERICAL, jointPos must be in range [-Pi, Pi].
    ///
    /// For PxArticulationJointType::eREVOLUTE, jointPos must be in range [-2*Pi, 2*Pi].
    ///
    /// For PxArticulationJointType::eREVOLUTE_UNWRAPPED, jointPos must be in range [-PX_MAX_REAL, PX_MAX_REAL].
    ///
    /// For PxArticulationJointType::ePRISMATIC, jointPos must be in range [-PX_MAX_REAL, PX_MAX_REAL].
    ///
    /// Joint position is specified in the parent frame of the joint. If Gp, Gc are the parent and child actor poses in the world frame and Lp, Lc are the parent and child joint frames expressed in the parent and child actor frames then the parent and child links will be given poses that obey Gp * Lp * J = Gc * Lc with J denoting the joint pose. For joints restricted to angular motion, J has the form PxTransform(PxVec3(PxZero), PxExp(PxVec3(twistPos, swing1Pos, swing2Pos))).  For joints restricted to linear motion, J has the form PxTransform(PxVec3(xPos, yPos, zPos), PxQuat(PxIdentity)).
    ///
    /// For spherical joints with more than 1 degree of freedom, the input joint positions taken together can collectively represent a rotation of greater than Pi around a vector. When this happens the rotation that matches the joint positions is not the shortest path rotation.  The joint pose J that is the outcome of setting and applying the joint positions will always be the equivalent of the shortest path rotation.
    ///
    /// Default:
    /// 0.0
    pub fn PxArticulationJointReducedCoordinate_setJointPosition_mut(self_: *mut PxArticulationJointReducedCoordinate, axis: PxArticulationAxis, jointPos: f32);

    /// Gets the joint position for the given axis, i.e. joint degree of freedom (DOF).
    ///
    /// For performance, prefer PxArticulationCache::jointPosition to get joint positions in a batch query.
    ///
    /// The joint position in linear units (equivalent to scene units) for a translational axis, or radians for a rotational axis.
    ///
    /// This call is not allowed while the simulation is running except in a split simulation during [`PxScene::collide`]() and up to #PxScene::advance(),
    /// and in PxContactModifyCallback or in contact report callbacks.
    ///
    /// This method should not be used after the direct GPU API has been enabled and initialized. See [`PxDirectGPUAPI`] for the details.
    pub fn PxArticulationJointReducedCoordinate_getJointPosition(self_: *const PxArticulationJointReducedCoordinate, axis: PxArticulationAxis) -> f32;

    /// Sets the joint velocity for the given axis.
    ///
    /// - For performance, prefer PxArticulationCache::jointVelocity to set joint velocities in a batch articulation state update.
    /// - Use PxArticulationReducedCoordinate::updateKinematic after all state updates to the articulation via non-cache API such as this method,
    /// in order to update link states for the next simulation frame or querying.
    ///
    /// This call is not allowed while the simulation is running.
    ///
    /// This method should not be used after the direct GPU API has been enabled and initialized. See [`PxDirectGPUAPI`] for the details.
    ///
    /// Default:
    /// 0.0
    pub fn PxArticulationJointReducedCoordinate_setJointVelocity_mut(self_: *mut PxArticulationJointReducedCoordinate, axis: PxArticulationAxis, jointVel: f32);

    /// Gets the joint velocity for the given axis.
    ///
    /// For performance, prefer PxArticulationCache::jointVelocity to get joint velocities in a batch query.
    ///
    /// The joint velocity in linear units (equivalent to scene units) per second for a translational axis, or radians per second for a rotational axis.
    ///
    /// This call is not allowed while the simulation is running except in a split simulation during [`PxScene::collide`]() and up to #PxScene::advance(),
    /// and in PxContactModifyCallback or in contact report callbacks.
    ///
    /// This method should not be used after the direct GPU API has been enabled and initialized. See [`PxDirectGPUAPI`] for the details.
    pub fn PxArticulationJointReducedCoordinate_getJointVelocity(self_: *const PxArticulationJointReducedCoordinate, axis: PxArticulationAxis) -> f32;

    /// Returns the string name of the dynamic type.
    ///
    /// The string name.
    pub fn PxArticulationJointReducedCoordinate_getConcreteTypeName(self_: *const PxArticulationJointReducedCoordinate) -> *const std::ffi::c_char;

    pub fn PxArticulationJointReducedCoordinate_delete(self_: *mut PxArticulationJointReducedCoordinate);

    /// Sets a name string for the object that can be retrieved with getName().
    ///
    /// This is for debugging and is not used by the SDK. The string is not copied by the SDK,
    /// only the pointer is stored.
    ///
    /// Default:
    /// NULL
    pub fn PxArticulationJointReducedCoordinate_setName_mut(self_: *mut PxArticulationJointReducedCoordinate, name: *const std::ffi::c_char);

    /// Retrieves the name string set with setName().
    ///
    /// Name string associated with object.
    pub fn PxArticulationJointReducedCoordinate_getName(self_: *const PxArticulationJointReducedCoordinate) -> *const std::ffi::c_char;

    /// Adjust the geometry of the shape.
    ///
    /// The type of the passed in geometry must match the geometry type of the shape.
    ///
    /// It is not allowed to change the geometry type of a shape.
    ///
    /// This function does not guarantee correct/continuous behavior when objects are resting on top of old or new geometry.
    pub fn PxShape_setGeometry_mut(self_: *mut PxShape, geometry: *const PxGeometry);

    /// Retrieve a reference to the shape's geometry.
    ///
    /// The returned reference has the same lifetime as the PxShape it comes from.
    ///
    /// Reference to internal PxGeometry object.
    pub fn PxShape_getGeometry(self_: *const PxShape) -> *const PxGeometry;

    /// Retrieves the actor which this shape is associated with.
    ///
    /// The actor this shape is associated with, if it is an exclusive shape, else NULL
    pub fn PxShape_getActor(self_: *const PxShape) -> *mut PxRigidActor;

    /// Sets the pose of the shape in actor space, i.e. relative to the actors to which they are attached.
    ///
    /// This transformation is identity by default.
    ///
    /// The local pose is an attribute of the shape, and so will apply to all actors to which the shape is attached.
    ///
    /// Sleeping:
    /// Does
    /// NOT
    /// wake the associated actor up automatically.
    ///
    /// Note:
    /// Does not automatically update the inertia properties of the owning actor (if applicable); use the
    /// PhysX extensions method [`PxRigidBodyExt::updateMassAndInertia`]() to do this.
    ///
    /// Default:
    /// the identity transform
    pub fn PxShape_setLocalPose_mut(self_: *mut PxShape, pose: *const PxTransform);

    /// Retrieves the pose of the shape in actor space, i.e. relative to the actor they are owned by.
    ///
    /// This transformation is identity by default.
    ///
    /// Pose of shape relative to the actor's frame.
    pub fn PxShape_getLocalPose(self_: *const PxShape) -> PxTransform;

    /// Sets the user definable collision filter data.
    ///
    /// Sleeping:
    /// Does wake up the actor if the filter data change causes a formerly suppressed
    /// collision pair to be enabled.
    ///
    /// Default:
    /// (0,0,0,0)
    pub fn PxShape_setSimulationFilterData_mut(self_: *mut PxShape, data: *const PxFilterData);

    /// Retrieves the shape's collision filter data.
    pub fn PxShape_getSimulationFilterData(self_: *const PxShape) -> PxFilterData;

    /// Sets the user definable query filter data.
    ///
    /// Default:
    /// (0,0,0,0)
    pub fn PxShape_setQueryFilterData_mut(self_: *mut PxShape, data: *const PxFilterData);

    /// Retrieves the shape's Query filter data.
    pub fn PxShape_getQueryFilterData(self_: *const PxShape) -> PxFilterData;

    /// Assigns material(s) to the shape. Will remove existing materials from the shape.
    ///
    /// Sleeping:
    /// Does
    /// NOT
    /// wake the associated actor up automatically.
    pub fn PxShape_setMaterials_mut(self_: *mut PxShape, materials: *const *mut PxMaterial, materialCount: u16);

    /// Assigns surface deformable material(s) to the shape. Will remove existing materials from the shape.
    ///
    /// Sleeping:
    /// Does
    /// NOT
    /// wake the associated actor up automatically.
    pub fn PxShape_setDeformableSurfaceMaterials_mut(self_: *mut PxShape, materials: *const *mut PxDeformableSurfaceMaterial, materialCount: u16);

    /// Assigns deformable volume material(s) to the shape. Will remove existing materials from the shape.
    ///
    /// Sleeping:
    /// Does
    /// NOT
    /// wake the associated actor up automatically.
    pub fn PxShape_setDeformableVolumeMaterials_mut(self_: *mut PxShape, materials: *const *mut PxDeformableVolumeMaterial, materialCount: u16);

    /// Returns the number of materials assigned to the shape.
    ///
    /// You can use [`getMaterials`]() to retrieve the material pointers.
    ///
    /// Number of materials associated with this shape.
    pub fn PxShape_getNbMaterials(self_: *const PxShape) -> u16;

    /// Retrieve all the material pointers associated with the shape.
    ///
    /// You can retrieve the number of material pointers by calling [`getNbMaterials`]()
    ///
    /// Note: The returned data may contain invalid pointers if you release materials using [`PxMaterial::release`]().
    ///
    /// Number of material pointers written to the buffer.
    pub fn PxShape_getMaterials(self_: *const PxShape, userBuffer: *mut *mut PxMaterial, bufferSize: u32, startIndex: u32) -> u32;

    /// Retrieve all the surface deformable material pointers associated with the shape.
    ///
    /// You can retrieve the number of material pointers by calling [`getNbMaterials`]()
    ///
    /// Note: The returned data may contain invalid pointers if you release materials using [`PxMaterial::release`]().
    ///
    /// Number of material pointers written to the buffer.
    pub fn PxShape_getDeformableSurfaceMaterials(self_: *const PxShape, userBuffer: *mut *mut PxDeformableSurfaceMaterial, bufferSize: u32, startIndex: u32) -> u32;

    /// Retrieve all the deformable volume material pointers associated with the shape.
    ///
    /// You can retrieve the number of material pointers by calling [`getNbMaterials`]()
    ///
    /// Note: The returned data may contain invalid pointers if you release materials using [`PxMaterial::release`]().
    ///
    /// Number of material pointers written to the buffer.
    pub fn PxShape_getDeformableVolumeMaterials(self_: *const PxShape, userBuffer: *mut *mut PxDeformableVolumeMaterial, bufferSize: u32, startIndex: u32) -> u32;

    /// Retrieve material from given triangle index.
    ///
    /// The input index is the internal triangle index as used inside the SDK. This is the index
    /// returned to users by various SDK functions such as raycasts.
    ///
    /// This function is only useful for triangle meshes or heightfields, which have per-triangle
    /// materials. For other shapes or SDF triangle meshes, the function returns the single material
    /// associated with the	shape, regardless of the index.
    ///
    /// Material from input triangle
    ///
    /// If faceIndex value of 0xFFFFffff is passed as an input for mesh and heightfield shapes, this function will issue a warning and return NULL.
    ///
    /// Scene queries set the value of PxQueryHit::faceIndex to 0xFFFFffff whenever it is undefined or does not apply.
    pub fn PxShape_getMaterialFromInternalFaceIndex(self_: *const PxShape, faceIndex: u32) -> *mut PxBaseMaterial;

    /// Sets the contact offset.
    ///
    /// Shapes whose distance is less than the sum of their contactOffset values will generate contacts. The contact offset must be positive and
    /// greater than the rest offset. Having a contactOffset greater than than the restOffset allows the collision detection system to
    /// predictively enforce the contact constraint even when the objects are slightly separated. This prevents jitter that would occur
    /// if the constraint were enforced only when shapes were within the rest distance.
    ///
    /// Default:
    /// 0.02f * PxTolerancesScale::length
    ///
    /// Sleeping:
    /// Does
    /// NOT
    /// wake the associated actor up automatically.
    pub fn PxShape_setContactOffset_mut(self_: *mut PxShape, contactOffset: f32);

    /// Retrieves the contact offset.
    ///
    /// The contact offset of the shape.
    pub fn PxShape_getContactOffset(self_: *const PxShape) -> f32;

    /// Sets the rest offset.
    ///
    /// Two shapes will come to rest at a distance equal to the sum of their restOffset values. If the restOffset is 0, they should converge to touching
    /// exactly.  Having a restOffset greater than zero is useful to have objects slide smoothly, so that they do not get hung up on irregularities of
    /// each others' surfaces.
    ///
    /// Default:
    /// 0.0f
    ///
    /// Sleeping:
    /// Does
    /// NOT
    /// wake the associated actor up automatically.
    pub fn PxShape_setRestOffset_mut(self_: *mut PxShape, restOffset: f32);

    /// Retrieves the rest offset.
    ///
    /// The rest offset of the shape.
    pub fn PxShape_getRestOffset(self_: *const PxShape) -> f32;

    /// Sets the density used to interact with fluids.
    ///
    /// To be physically accurate, the density of a rigid body should be computed as its mass divided by its volume. To
    /// simplify tuning the interaction of fluid and rigid bodies, the density for fluid can differ from the real density. This
    /// allows to create floating bodies, even if they are supposed to sink with their mass and volume.
    ///
    /// Default:
    /// 800.0f
    pub fn PxShape_setDensityForFluid_mut(self_: *mut PxShape, densityForFluid: f32);

    /// Retrieves the density used to interact with fluids.
    ///
    /// The density of the body when interacting with fluid.
    pub fn PxShape_getDensityForFluid(self_: *const PxShape) -> f32;

    /// Sets torsional patch radius.
    ///
    /// This defines the radius of the contact patch used to apply torsional friction. If the radius is 0 (and minTorsionalPatchRadius
    /// is 0 too, see [`setMinTorsionalPatchRadius`]), no torsional friction will be applied. If the radius is > 0, some torsional friction
    /// will be applied. This is proportional to the penetration depth so, if the shapes are separated or penetration is zero, no
    /// torsional friction will be applied. It is used to approximate rotational friction introduced by the compression of contacting surfaces.
    ///
    /// Will only be active, if the friction patch has a single anchor point only. This is for example the case, if a contact patch
    /// has a single contact point.
    ///
    /// Only supported in combination with solver type PxSolverType::eTGS.
    ///
    /// Default:
    /// 0.0
    pub fn PxShape_setTorsionalPatchRadius_mut(self_: *mut PxShape, radius: f32);

    /// Gets torsional patch radius.
    ///
    /// See [`setTorsionalPatchRadius`] for more info.
    ///
    /// The torsional patch radius of the shape.
    pub fn PxShape_getTorsionalPatchRadius(self_: *const PxShape) -> f32;

    /// Sets minimum torsional patch radius.
    ///
    /// This defines the minimum radius of the contact patch used to apply torsional friction. If the radius is 0, the amount of torsional friction
    /// that will be applied will be entirely dependent on the value of torsionalPatchRadius.
    ///
    /// If the radius is > 0, some torsional friction will be applied regardless of the value of torsionalPatchRadius or the amount of penetration.
    ///
    /// Will only be active in certain cases, see [`setTorsionalPatchRadius`] for details.
    ///
    /// Default:
    /// 0.0
    pub fn PxShape_setMinTorsionalPatchRadius_mut(self_: *mut PxShape, radius: f32);

    /// Gets minimum torsional patch radius.
    ///
    /// See [`setMinTorsionalPatchRadius`] for more info.
    ///
    /// The minimum torsional patch radius of the shape.
    pub fn PxShape_getMinTorsionalPatchRadius(self_: *const PxShape) -> f32;

    /// Returns the GPU shape index.
    ///
    /// This function only returns valid results if GPU dynamics is enabled.
    ///
    /// The GPU index, or 0xFFFFFFFF if the shape is not attached to a PxActor that is inserted into a PxScene.
    pub fn PxShape_getGPUIndex(self_: *const PxShape) -> u32;

    /// Sets shape flags
    ///
    /// Sleeping:
    /// Does
    /// NOT
    /// wake the associated actor up automatically.
    ///
    /// Default:
    /// PxShapeFlag::eVISUALIZATION | PxShapeFlag::eSIMULATION_SHAPE | PxShapeFlag::eSCENE_QUERY_SHAPE
    pub fn PxShape_setFlag_mut(self_: *mut PxShape, flag: PxShapeFlag, value: bool);

    /// Sets shape flags
    pub fn PxShape_setFlags_mut(self_: *mut PxShape, inFlags: PxShapeFlags);

    /// Retrieves shape flags.
    ///
    /// The values of the shape flags.
    pub fn PxShape_getFlags(self_: *const PxShape) -> PxShapeFlags;

    /// Returns true if the shape is exclusive to an actor.
    pub fn PxShape_isExclusive(self_: *const PxShape) -> bool;

    /// Sets a name string for the object that can be retrieved with [`getName`]().
    ///
    /// This is for debugging and is not used by the SDK.
    /// The string is not copied by the SDK, only the pointer is stored.
    ///
    /// Default:
    /// NULL
    pub fn PxShape_setName_mut(self_: *mut PxShape, name: *const std::ffi::c_char);

    /// retrieves the name string set with setName().
    ///
    /// The name associated with the shape.
    pub fn PxShape_getName(self_: *const PxShape) -> *const std::ffi::c_char;

    pub fn PxShape_getConcreteTypeName(self_: *const PxShape) -> *const std::ffi::c_char;

    /// Deletes the rigid actor object.
    ///
    /// Also releases any shapes associated with the actor.
    ///
    /// Releasing an actor will affect any objects that are connected to the actor (constraint shaders like joints etc.).
    /// Such connected objects will be deleted upon scene deletion, or explicitly by the user by calling release()
    /// on these objects. It is recommended to always remove all objects that reference actors before the actors
    /// themselves are removed. It is not possible to retrieve list of dead connected objects.
    ///
    /// Sleeping:
    /// This call will awaken any sleeping actors contacting the deleted actor (directly or indirectly).
    ///
    /// Calls [`PxActor::release`]() so you might want to check the documentation of that method as well.
    pub fn PxRigidActor_release_mut(self_: *mut PxRigidActor);

    /// Returns the internal actor index.
    ///
    /// This is only defined for actors that have been added to a scene.
    ///
    /// The internal actor index, or 0xffffffff if the actor is not part of a scene.
    pub fn PxRigidActor_getInternalActorIndex(self_: *const PxRigidActor) -> u32;

    /// Retrieves the actors world space transform.
    ///
    /// The getGlobalPose() method retrieves the actor's current actor space to world space transformation.
    ///
    /// It is not allowed to use this method while the simulation is running (except during PxScene::collide(),
    /// in PxContactModifyCallback or in contact report callbacks).
    ///
    /// If this actor is a PxRigidDynamic or PxArticulationLink, this method should not be used after the direct GPU API has been enabled and initialized. See [`PxDirectGPUAPI`] for the details.
    ///
    /// Global pose of object.
    pub fn PxRigidActor_getGlobalPose(self_: *const PxRigidActor) -> PxTransform;

    /// Method for setting an actor's pose in the world.
    ///
    /// This method instantaneously changes the actor space to world space transformation.
    ///
    /// This method is mainly for dynamic rigid bodies (see [`PxRigidDynamic`]). Calling this method on static actors is
    /// likely to result in a performance penalty, since internal optimization structures for static actors may need to be
    /// recomputed. In addition, moving static actors will not interact correctly with dynamic actors or joints.
    ///
    /// To directly control an actor's position and have it correctly interact with dynamic bodies and joints, create a dynamic
    /// body with the PxRigidBodyFlag::eKINEMATIC flag, then use the setKinematicTarget() commands to define its path.
    ///
    /// Even when moving dynamic actors, exercise restraint in making use of this method. Where possible, avoid:
    ///
    /// moving actors into other actors, thus causing overlap (an invalid physical state)
    ///
    /// moving an actor that is connected by a joint to another away from the other (thus causing joint error)
    ///
    /// It is not allowed to use this method if the actor is part of a [`PxPruningStructure`] that has not been
    /// added to a scene yet.
    ///
    /// If this actor is a PxRigidDynamic or PxArticulationLink, this method should not be used after the direct GPU API has been enabled and initialized. See [`PxDirectGPUAPI`] for the details.
    ///
    /// Sleeping:
    /// This call wakes dynamic actors if they are sleeping and the autowake parameter is true (default).
    pub fn PxRigidActor_setGlobalPose_mut(self_: *mut PxRigidActor, pose: *const PxTransform, autowake: bool);

    /// Attach a shape to an actor
    ///
    /// This call will increment the reference count of the shape.
    ///
    /// Mass properties of dynamic rigid actors will not automatically be recomputed
    /// to reflect the new mass distribution implied by the shape. Follow this call with a call to
    /// the PhysX extensions method [`PxRigidBodyExt::updateMassAndInertia`]() to do that.
    ///
    /// Attaching a triangle mesh, heightfield or plane geometry shape configured as eSIMULATION_SHAPE is not supported for
    /// non-kinematic PxRigidDynamic instances.
    ///
    /// Sleeping:
    /// Does
    /// NOT
    /// wake the actor up automatically.
    ///
    /// True if success.
    pub fn PxRigidActor_attachShape_mut(self_: *mut PxRigidActor, shape: *mut PxShape) -> bool;

    /// Detach a shape from an actor.
    ///
    /// This will also decrement the reference count of the PxShape, and if the reference count is zero, will cause it to be deleted.
    ///
    /// Sleeping:
    /// Does
    /// NOT
    /// wake the actor up automatically.
    pub fn PxRigidActor_detachShape_mut(self_: *mut PxRigidActor, shape: *mut PxShape, wakeOnLostTouch: bool);

    /// Returns the number of shapes assigned to the actor.
    ///
    /// You can use [`getShapes`]() to retrieve the shape pointers.
    ///
    /// Number of shapes associated with this actor.
    pub fn PxRigidActor_getNbShapes(self_: *const PxRigidActor) -> u32;

    /// Retrieve all the shape pointers belonging to the actor.
    ///
    /// These are the shapes used by the actor for collision detection.
    ///
    /// You can retrieve the number of shape pointers by calling [`getNbShapes`]()
    ///
    /// Note: Removing shapes with [`PxShape::release`]() will invalidate the pointer of the released shape.
    ///
    /// Number of shape pointers written to the buffer.
    pub fn PxRigidActor_getShapes(self_: *const PxRigidActor, userBuffer: *mut *mut PxShape, bufferSize: u32, startIndex: u32) -> u32;

    /// Returns the number of constraint shaders attached to the actor.
    ///
    /// You can use [`getConstraints`]() to retrieve the constraint shader pointers.
    ///
    /// Number of constraint shaders attached to this actor.
    pub fn PxRigidActor_getNbConstraints(self_: *const PxRigidActor) -> u32;

    /// Retrieve all the constraint shader pointers belonging to the actor.
    ///
    /// You can retrieve the number of constraint shader pointers by calling [`getNbConstraints`]()
    ///
    /// Note: Removing constraint shaders with [`PxConstraint::release`]() will invalidate the pointer of the released constraint.
    ///
    /// Number of constraint shader pointers written to the buffer.
    pub fn PxRigidActor_getConstraints(self_: *const PxRigidActor, userBuffer: *mut *mut PxConstraint, bufferSize: u32, startIndex: u32) -> u32;

    pub fn PxNodeIndex_new(id: u32, articLinkId: u32) -> PxNodeIndex;

    pub fn PxNodeIndex_new_1(id: u32) -> PxNodeIndex;

    pub fn PxNodeIndex_new_2(ind: u64) -> PxNodeIndex;

    pub fn PxNodeIndex_new_3(id: u32, linkData: u32, anon_param2: bool) -> PxNodeIndex;

    pub fn PxNodeIndex_getInd(self_: *const PxNodeIndex) -> u64;

    pub fn PxNodeIndex_index(self_: *const PxNodeIndex) -> u32;

    pub fn PxNodeIndex_linkData(self_: *const PxNodeIndex) -> u32;

    pub fn PxNodeIndex_articulationLinkId(self_: *const PxNodeIndex) -> u32;

    pub fn PxNodeIndex_isArticulation(self_: *const PxNodeIndex) -> u32;

    pub fn PxNodeIndex_isStaticBody(self_: *const PxNodeIndex) -> bool;

    pub fn PxNodeIndex_isValid(self_: *const PxNodeIndex) -> bool;

    pub fn PxNodeIndex_setIndices_mut(self_: *mut PxNodeIndex, index: u32, articLinkId: u32);

    pub fn PxNodeIndex_setIndices_mut_1(self_: *mut PxNodeIndex, index: u32);

    /// Sets the pose of the center of mass relative to the actor.
    ///
    /// Changing this transform will not move the actor in the world!
    ///
    /// Setting an unrealistic center of mass which is a long way from the body can make it difficult for
    /// the SDK to solve constraints. Perhaps leading to instability and jittering bodies.
    ///
    /// Changing this transform will not update the linear velocity reported by getLinearVelocity() to account
    /// for the shift in center of mass. If the shift should be accounted for, the user should update the velocity
    /// using setLinearVelocity().
    ///
    /// Default:
    /// the identity transform
    pub fn PxRigidBody_setCMassLocalPose_mut(self_: *mut PxRigidBody, pose: *const PxTransform);

    /// Retrieves the center of mass pose relative to the actor frame.
    ///
    /// The center of mass pose relative to the actor frame.
    pub fn PxRigidBody_getCMassLocalPose(self_: *const PxRigidBody) -> PxTransform;

    /// Sets the mass of a dynamic actor.
    ///
    /// The mass must be non-negative.
    ///
    /// setMass() does not update the inertial properties of the body, to change the inertia tensor
    /// use setMassSpaceInertiaTensor() or the PhysX extensions method [`PxRigidBodyExt::updateMassAndInertia`]().
    ///
    /// A value of 0 is interpreted as infinite mass.
    ///
    /// Values of 0 are not permitted for instances of PxArticulationLink but are permitted for instances of PxRigidDynamic.
    ///
    /// Default:
    /// 1.0
    ///
    /// Sleeping:
    /// Does
    /// NOT
    /// wake the actor up automatically.
    pub fn PxRigidBody_setMass_mut(self_: *mut PxRigidBody, mass: f32);

    /// Retrieves the mass of the actor.
    ///
    /// A value of 0 is interpreted as infinite mass.
    ///
    /// The mass of this actor.
    pub fn PxRigidBody_getMass(self_: *const PxRigidBody) -> f32;

    /// Retrieves the inverse mass of the actor.
    ///
    /// The inverse mass of this actor.
    pub fn PxRigidBody_getInvMass(self_: *const PxRigidBody) -> f32;

    /// Sets the inertia tensor, using a parameter specified in mass space coordinates.
    ///
    /// Note that such matrices are diagonal -- the passed vector is the diagonal.
    ///
    /// If you have a non diagonal world/actor space inertia tensor(3x3 matrix). Then you need to
    /// diagonalize it and set an appropriate mass space transform. See [`setCMassLocalPose`]().
    ///
    /// The inertia tensor elements must be non-negative.
    ///
    /// A value of 0 in an element is interpreted as infinite inertia along that axis.
    ///
    /// Values of 0 are not permitted for instances of PxArticulationLink but are permitted for instances of PxRigidDynamic.
    ///
    /// Default:
    /// (1.0, 1.0, 1.0)
    ///
    /// Sleeping:
    /// Does
    /// NOT
    /// wake the actor up automatically.
    pub fn PxRigidBody_setMassSpaceInertiaTensor_mut(self_: *mut PxRigidBody, m: *const PxVec3);

    /// Retrieves the diagonal inertia tensor of the actor relative to the mass coordinate frame.
    ///
    /// This method retrieves a mass frame inertia vector.
    ///
    /// The mass space inertia tensor of this actor.
    ///
    /// A value of 0 in an element is interpreted as infinite inertia along that axis.
    pub fn PxRigidBody_getMassSpaceInertiaTensor(self_: *const PxRigidBody) -> PxVec3;

    /// Retrieves the diagonal inverse inertia tensor of the actor relative to the mass coordinate frame.
    ///
    /// This method retrieves a mass frame inverse inertia vector.
    ///
    /// A value of 0 in an element is interpreted as infinite inertia along that axis.
    ///
    /// The mass space inverse inertia tensor of this actor.
    pub fn PxRigidBody_getMassSpaceInvInertiaTensor(self_: *const PxRigidBody) -> PxVec3;

    /// Sets the linear damping coefficient.
    ///
    /// Zero represents no damping. The damping coefficient must be nonnegative.
    ///
    /// Default:
    /// 0.05 for PxArticulationLink, 0.0 for PxRigidDynamic
    pub fn PxRigidBody_setLinearDamping_mut(self_: *mut PxRigidBody, linDamp: f32);

    /// Retrieves the linear damping coefficient.
    ///
    /// The linear damping coefficient associated with this actor.
    pub fn PxRigidBody_getLinearDamping(self_: *const PxRigidBody) -> f32;

    /// Sets the angular damping coefficient.
    ///
    /// Zero represents no damping.
    ///
    /// The angular damping coefficient must be nonnegative.
    ///
    /// Default:
    /// 0.05
    pub fn PxRigidBody_setAngularDamping_mut(self_: *mut PxRigidBody, angDamp: f32);

    /// Retrieves the angular damping coefficient.
    ///
    /// The angular damping coefficient associated with this actor.
    pub fn PxRigidBody_getAngularDamping(self_: *const PxRigidBody) -> f32;

    /// Retrieves the linear velocity of an actor.
    ///
    /// It is not allowed to use this method while the simulation is running (except during PxScene::collide(),
    /// in PxContactModifyCallback or in contact report callbacks).
    ///
    /// The linear velocity is reported with respect to the rigid body's center of mass and not the actor frame origin.
    ///
    /// The linear velocity of the actor.
    pub fn PxRigidBody_getLinearVelocity(self_: *const PxRigidBody) -> PxVec3;

    /// Retrieves the angular velocity of the actor.
    ///
    /// It is not allowed to use this method while the simulation is running (except during PxScene::collide(),
    /// in PxContactModifyCallback or in contact report callbacks).
    ///
    /// The angular velocity of the actor.
    pub fn PxRigidBody_getAngularVelocity(self_: *const PxRigidBody) -> PxVec3;

    /// Lets you set the maximum linear velocity permitted for this actor.
    ///
    /// With this function, you can set the  maximum linear velocity permitted for this rigid body.
    /// Higher linear velocities are clamped to this value.
    ///
    /// Note: The linear velocity is clamped to the set value
    /// before
    /// the solver, which means that
    /// the limit may still be momentarily exceeded.
    ///
    /// Enforcing the limit introduces momentum into the simulation, causing potentially unphysical behavior.
    /// For articulation links, consider using joint damping and limits instead, which preserve momentum.
    ///
    /// Default:
    /// 100 * PxTolerancesScale::length /s for PxArticulationLink, 1e^16 lengthUnits/s for PxRigidDynamic
    pub fn PxRigidBody_setMaxLinearVelocity_mut(self_: *mut PxRigidBody, maxLinVel: f32);

    /// Retrieves the maximum angular velocity permitted for this actor.
    ///
    /// The maximum allowed angular velocity for this actor.
    pub fn PxRigidBody_getMaxLinearVelocity(self_: *const PxRigidBody) -> f32;

    /// Lets you set the maximum angular velocity permitted for this actor.
    ///
    /// For various internal computations, very quickly rotating actors introduce error
    /// into the simulation, which leads to undesired results.
    ///
    /// With this function, you can set the  maximum angular velocity permitted for this rigid body.
    /// Higher angular velocities are clamped to this value.
    ///
    /// Note: The angular velocity is clamped to the set value
    /// before
    /// the solver, which means that
    /// the limit may still be momentarily exceeded.
    ///
    /// Enforcing the limit introduces momentum into the simulation, causing potentially unphysical behavior.
    /// For articulation links, consider using joint damping and limits instead, which preserve momentum.
    ///
    /// Default:
    /// 50.0 rad/s for PxArticulationLink, 100.0 rad/s for PxRigidDynamic
    ///
    /// Range:
    /// [0, 1e^16) rad/s
    pub fn PxRigidBody_setMaxAngularVelocity_mut(self_: *mut PxRigidBody, maxAngVel: f32);

    /// Retrieves the maximum angular velocity permitted for this actor.
    ///
    /// The maximum allowed angular velocity for this actor.
    pub fn PxRigidBody_getMaxAngularVelocity(self_: *const PxRigidBody) -> f32;

    /// Retrieves the linear acceleration of an actor.
    ///
    /// For PxArticulationLink objects, this function is always available.
    ///
    /// For PxRigidDynamic actors, this function only returns valid results if PxSceneFlag::eENABLE_BODY_ACCELERATIONS is enabled.
    /// If that flag is not enabled, the function returns zero for PxRigidDynamic actors.
    ///
    /// The linear acceleration of the actor, or zero if PxSceneFlag::eENABLE_BODY_ACCELERATIONS is disabled and the object is a PxRigidDynamic.
    pub fn PxRigidBody_getLinearAcceleration(self_: *const PxRigidBody) -> PxVec3;

    /// Retrieves the angular acceleration of an actor.
    ///
    /// For PxArticulationLink objects, this function is always available.
    ///
    /// For PxRigidDynamic actors, this function only returns valid results if PxSceneFlag::eENABLE_BODY_ACCELERATIONS is enabled.
    /// If that flag is not enabled, the function returns zero for PxRigidDynamic actors.
    ///
    /// The angular acceleration of the actor, or zero if PxSceneFlag::eENABLE_BODY_ACCELERATIONS is disabled and the object is a PxRigidDynamic.
    pub fn PxRigidBody_getAngularAcceleration(self_: *const PxRigidBody) -> PxVec3;

    /// Applies a force (or impulse) defined in the global coordinate frame to the actor at its center of mass.
    ///
    /// This will not induce a torque
    /// .
    ///
    /// ::PxForceMode determines if the force is to be conventional or impulsive.
    ///
    /// Each actor has an acceleration and a velocity change accumulator which are directly modified using the modes PxForceMode::eACCELERATION
    /// and PxForceMode::eVELOCITY_CHANGE respectively.  The modes PxForceMode::eFORCE and PxForceMode::eIMPULSE also modify these same
    /// accumulators and are just short hand for multiplying the vector parameter by inverse mass and then using PxForceMode::eACCELERATION and
    /// PxForceMode::eVELOCITY_CHANGE respectively.
    ///
    /// It is invalid to use this method if the actor has not been added to a scene already or if PxActorFlag::eDISABLE_SIMULATION is set.
    ///
    /// This method should not be used after the direct GPU API has been enabled and initialized. See [`PxDirectGPUAPI`] for the details.
    ///
    /// The force modes PxForceMode::eIMPULSE and PxForceMode::eVELOCITY_CHANGE can not be applied to articulation links.
    ///
    /// if this is called on an articulation link, only the link is updated, not the entire articulation.
    ///
    /// see [`PxRigidBodyExt::computeVelocityDeltaFromImpulse`] for details of how to compute the change in linear velocity that
    /// will arise from the application of an impulsive force, where an impulsive force is applied force multiplied by a timestep.
    ///
    /// Forces will be cleared automatically after they are applied during the next simulation step. If the forces should be retained for
    /// the following steps, PxRigidBodyFlag::eRETAIN_ACCELERATIONS should be raised.
    ///
    /// Sleeping:
    /// This call wakes the actor if it is sleeping, and the autowake parameter is true (default) or the force is non-zero.
    pub fn PxRigidBody_addForce_mut(self_: *mut PxRigidBody, force: *const PxVec3, mode: PxForceMode, autowake: bool);

    /// Applies an impulsive torque defined in the global coordinate frame to the actor.
    ///
    /// ::PxForceMode determines if the torque is to be conventional or impulsive.
    ///
    /// Each actor has an angular acceleration and an angular velocity change accumulator which are directly modified using the modes
    /// PxForceMode::eACCELERATION and PxForceMode::eVELOCITY_CHANGE respectively.  The modes PxForceMode::eFORCE and PxForceMode::eIMPULSE
    /// also modify these same accumulators and are just short hand for multiplying the vector parameter by inverse inertia and then
    /// using PxForceMode::eACCELERATION and PxForceMode::eVELOCITY_CHANGE respectively.
    ///
    /// It is invalid to use this method if the actor has not been added to a scene already or if PxActorFlag::eDISABLE_SIMULATION is set.
    ///
    /// This method should not be used after the direct GPU API has been enabled and initialized. See [`PxDirectGPUAPI`] for the details.
    ///
    /// The force modes PxForceMode::eIMPULSE and PxForceMode::eVELOCITY_CHANGE can not be applied to articulation links.
    ///
    /// if this called on an articulation link, only the link is updated, not the entire articulation.
    ///
    /// see [`PxRigidBodyExt::computeVelocityDeltaFromImpulse`] for details of how to compute the change in angular velocity that
    /// will arise from the application of an impulsive torque, where an impulsive torque is an applied torque multiplied by a timestep.
    ///
    /// Torques will be cleared after they are applied during the next simulation step. If the Torques should be retained for the following
    /// steps, PxRigidBodyFlag::eRETAIN_ACCELERATIONS should be raised.
    ///
    /// Sleeping:
    /// This call wakes the actor if it is sleeping, and the autowake parameter is true (default) or the torque is non-zero.
    pub fn PxRigidBody_addTorque_mut(self_: *mut PxRigidBody, torque: *const PxVec3, mode: PxForceMode, autowake: bool);

    /// Clears the accumulated forces (sets the accumulated force back to zero).
    ///
    /// Each actor has an acceleration and a velocity change accumulator which are directly modified using the modes PxForceMode::eACCELERATION
    /// and PxForceMode::eVELOCITY_CHANGE respectively.  The modes PxForceMode::eFORCE and PxForceMode::eIMPULSE also modify these same
    /// accumulators (see PxRigidBody::addForce() for details); therefore the effect of calling clearForce(PxForceMode::eFORCE) is equivalent to calling
    /// clearForce(PxForceMode::eACCELERATION), and the effect of calling clearForce(PxForceMode::eIMPULSE) is equivalent to calling
    /// clearForce(PxForceMode::eVELOCITY_CHANGE).
    ///
    /// ::PxForceMode determines if the cleared force is to be conventional or impulsive.
    ///
    /// The force modes PxForceMode::eIMPULSE and PxForceMode::eVELOCITY_CHANGE can not be applied to articulation links.
    ///
    /// This method should not be used after the direct GPU API has been enabled and initialized. See [`PxDirectGPUAPI`] for the details.
    ///
    /// It is invalid to use this method if the actor has not been added to a scene already or if PxActorFlag::eDISABLE_SIMULATION is set.
    pub fn PxRigidBody_clearForce_mut(self_: *mut PxRigidBody, mode: PxForceMode);

    /// Clears the impulsive torque defined in the global coordinate frame to the actor.
    ///
    /// ::PxForceMode determines if the cleared torque is to be conventional or impulsive.
    ///
    /// Each actor has an angular acceleration and a velocity change accumulator which are directly modified using the modes PxForceMode::eACCELERATION
    /// and PxForceMode::eVELOCITY_CHANGE respectively.  The modes PxForceMode::eFORCE and PxForceMode::eIMPULSE also modify these same
    /// accumulators (see PxRigidBody::addTorque() for details); therefore the effect of calling clearTorque(PxForceMode::eFORCE) is equivalent to calling
    /// clearTorque(PxForceMode::eACCELERATION), and the effect of calling clearTorque(PxForceMode::eIMPULSE) is equivalent to calling
    /// clearTorque(PxForceMode::eVELOCITY_CHANGE).
    ///
    /// The force modes PxForceMode::eIMPULSE and PxForceMode::eVELOCITY_CHANGE can not be applied to articulation links.
    ///
    /// This method should not be used after the direct GPU API has been enabled and initialized. See [`PxDirectGPUAPI`] for the details.
    ///
    /// It is invalid to use this method if the actor has not been added to a scene already or if PxActorFlag::eDISABLE_SIMULATION is set.
    pub fn PxRigidBody_clearTorque_mut(self_: *mut PxRigidBody, mode: PxForceMode);

    /// Sets the impulsive force and torque defined in the global coordinate frame to the actor.
    ///
    /// ::PxForceMode determines if the cleared torque is to be conventional or impulsive.
    ///
    /// The force modes PxForceMode::eIMPULSE and PxForceMode::eVELOCITY_CHANGE can not be applied to articulation links.
    ///
    /// This method should not be used after the direct GPU API has been enabled and initialized. See [`PxDirectGPUAPI`] for the details.
    ///
    /// It is invalid to use this method if the actor has not been added to a scene already or if PxActorFlag::eDISABLE_SIMULATION is set.
    ///
    /// Forces and torques will be cleared after they are applied during the next simulation step. If they should be retained for the following
    /// steps, PxRigidBodyFlag::eRETAIN_ACCELERATIONS should be raised.
    pub fn PxRigidBody_setForceAndTorque_mut(self_: *mut PxRigidBody, force: *const PxVec3, torque: *const PxVec3, mode: PxForceMode);

    /// Raises or clears a particular rigid body flag.
    ///
    /// See the list of flags [`PxRigidBodyFlag`]
    ///
    /// Default:
    /// no flags are set
    ///
    /// Sleeping:
    /// Does
    /// NOT
    /// wake the actor up automatically.
    pub fn PxRigidBody_setRigidBodyFlag_mut(self_: *mut PxRigidBody, flag: PxRigidBodyFlag, value: bool);

    pub fn PxRigidBody_setRigidBodyFlags_mut(self_: *mut PxRigidBody, inFlags: PxRigidBodyFlags);

    /// Reads the PxRigidBody flags.
    ///
    /// See the list of flags [`PxRigidBodyFlag`]
    ///
    /// The values of the PxRigidBody flags.
    pub fn PxRigidBody_getRigidBodyFlags(self_: *const PxRigidBody) -> PxRigidBodyFlags;

    /// Sets the CCD minimum advance coefficient.
    ///
    /// The CCD minimum advance coefficient is a value in the range [0, 1] that is used to control the minimum amount of time a body is integrated when
    /// it has a CCD contact. The actual minimum amount of time that is integrated depends on various properties, including the relative speed and collision shapes
    /// of the bodies involved in the contact. From these properties, a numeric value is calculated that determines the maximum distance (and therefore maximum time)
    /// which these bodies could be integrated forwards that would ensure that these bodies did not pass through each-other. This value is then scaled by CCD minimum advance
    /// coefficient to determine the amount of time that will be consumed in the CCD pass.
    ///
    /// Things to consider:
    /// A large value (approaching 1) ensures that the objects will always advance some time. However, larger values increase the chances of objects gently drifting through each-other in
    /// scenes which the constraint solver can't converge, e.g. scenes where an object is being dragged through a wall with a constraint.
    /// A value of 0 ensures that the pair of objects stop at the exact time-of-impact and will not gently drift through each-other. However, with very small/thin objects initially in
    /// contact, this can lead to a large amount of time being dropped and increases the chances of jamming. Jamming occurs when the an object is persistently in contact with an object
    /// such that the time-of-impact is	0, which results in no time being advanced for those objects in that CCD pass.
    ///
    /// The chances of jamming can be reduced by increasing the number of CCD mass
    pub fn PxRigidBody_setMinCCDAdvanceCoefficient_mut(self_: *mut PxRigidBody, advanceCoefficient: f32);

    /// Gets the CCD minimum advance coefficient.
    ///
    /// The value of the CCD min advance coefficient.
    pub fn PxRigidBody_getMinCCDAdvanceCoefficient(self_: *const PxRigidBody) -> f32;

    /// Sets the maximum depenetration velocity permitted to be introduced by the solver.
    /// This value controls how much velocity the solver can introduce to correct for penetrations in contacts.
    pub fn PxRigidBody_setMaxDepenetrationVelocity_mut(self_: *mut PxRigidBody, biasClamp: f32);

    /// Returns the maximum depenetration velocity the solver is permitted to introduced.
    /// This value controls how much velocity the solver can introduce to correct for penetrations in contacts.
    ///
    /// The maximum penetration bias applied by the solver.
    pub fn PxRigidBody_getMaxDepenetrationVelocity(self_: *const PxRigidBody) -> f32;

    /// Sets a limit on the impulse that may be applied at a contact. The maximum impulse at a contact between two dynamic or kinematic
    /// bodies will be the minimum	of the two limit values. For a collision between a static and a dynamic body, the impulse is limited
    /// by the value for the dynamic body.
    pub fn PxRigidBody_setMaxContactImpulse_mut(self_: *mut PxRigidBody, maxImpulse: f32);

    /// Returns the maximum impulse that may be applied at a contact.
    ///
    /// The maximum impulse that may be applied at a contact
    pub fn PxRigidBody_getMaxContactImpulse(self_: *const PxRigidBody) -> f32;

    /// Sets a distance scale whereby the angular influence of a contact on the normal constraint in a contact is
    /// zeroed if normal.cross(offset) falls below this tolerance. Rather than acting as an absolute value, this tolerance
    /// is scaled by the ratio rXn.dot(angVel)/normal.dot(linVel) such that contacts that have relatively larger angular velocity
    /// than linear normal velocity (e.g. rolling wheels) achieve larger slop values as the angular velocity increases.
    pub fn PxRigidBody_setContactSlopCoefficient_mut(self_: *mut PxRigidBody, slopCoefficient: f32);

    /// Returns the contact slop coefficient.
    ///
    /// The contact slop coefficient.
    pub fn PxRigidBody_getContactSlopCoefficient(self_: *const PxRigidBody) -> f32;

    /// Returns the island node index
    ///
    /// The island node index.
    pub fn PxRigidBody_getInternalIslandNodeIndex(self_: *const PxRigidBody) -> PxNodeIndex;

    /// Releases the link from the articulation.
    ///
    /// Only a leaf articulation link can be released.
    ///
    /// Releasing a link is not allowed while the articulation link is in a scene. In order to release a link,
    /// remove and then re-add the corresponding articulation to the scene.
    pub fn PxArticulationLink_release_mut(self_: *mut PxArticulationLink);

    /// Gets the articulation that the link is a part of.
    ///
    /// The articulation.
    pub fn PxArticulationLink_getArticulation(self_: *const PxArticulationLink) -> *mut PxArticulationReducedCoordinate;

    /// Gets the joint which connects this link to its parent.
    ///
    /// The joint connecting the link to the parent. NULL for the root link.
    pub fn PxArticulationLink_getInboundJoint(self_: *const PxArticulationLink) -> *mut PxArticulationJointReducedCoordinate;

    /// Gets the number of degrees of freedom of the joint which connects this link to its parent.
    ///
    /// - The root link DOF-count is defined to be 0 regardless of PxArticulationFlag::eFIX_BASE.
    /// - The return value is only valid for articulations that are in a scene.
    ///
    /// The number of degrees of freedom, or 0xFFFFFFFF if the articulation is not in a scene.
    pub fn PxArticulationLink_getInboundJointDof(self_: *const PxArticulationLink) -> u32;

    /// Gets the number of child links.
    ///
    /// The number of child links.
    pub fn PxArticulationLink_getNbChildren(self_: *const PxArticulationLink) -> u32;

    /// Gets the low-level link index that may be used to index into members of PxArticulationCache.
    ///
    /// The low-level indices are built after an articulation is added to the scene following a breadth-first approach,
    /// where all the links at the current depth are indexed sequentially before moving to the links at the next depth level.
    /// The root of the articulation has therefore the index 0.
    /// Note that the low-level indices may be different from the order in which the links were originally added to the articulation.
    ///
    /// The return value is only valid for articulations that are in a scene.
    ///
    /// The low-level index, or 0xFFFFFFFF if the articulation is not in a scene.
    pub fn PxArticulationLink_getLinkIndex(self_: *const PxArticulationLink) -> u32;

    /// Retrieves the child links.
    ///
    /// The number of articulation links written to the buffer.
    pub fn PxArticulationLink_getChildren(self_: *const PxArticulationLink, userBuffer: *mut *mut PxArticulationLink, bufferSize: u32, startIndex: u32) -> u32;

    /// Set the constraint-force-mixing scale term.
    ///
    /// The cfm scale term is a stabilization term that helps avoid instabilities with over-constrained
    /// configurations. It should be a small value that is multiplied by 1/mass internally to produce
    /// an additional bias added to the unit response term in the solver.
    ///
    /// Default:
    /// 0.025
    /// Range:
    /// [0, 1]
    ///
    /// This call is not allowed while the simulation is running.
    pub fn PxArticulationLink_setCfmScale_mut(self_: *mut PxArticulationLink, cfm: f32);

    /// Get the constraint-force-mixing scale term.
    ///
    /// The constraint-force-mixing scale term.
    pub fn PxArticulationLink_getCfmScale(self_: *const PxArticulationLink) -> f32;

    /// Get the linear velocity of the link.
    ///
    /// - For performance, prefer PxArticulationCache::linkVelocity to get link spatial velocities in a batch query.
    /// - When the articulation state is updated via non-cache API, use PxArticulationReducedCoordinate::updateKinematic before querying velocity.
    ///
    /// The linear velocity of the link.
    ///
    /// This call is not allowed while the simulation is running except in a split simulation during [`PxScene::collide`]() and up to #PxScene::advance(),
    /// and in PxContactModifyCallback or in contact report callbacks.
    ///
    /// The linear velocity is reported with respect to the link's center of mass and not the actor frame origin.
    pub fn PxArticulationLink_getLinearVelocity(self_: *const PxArticulationLink) -> PxVec3;

    /// Get the angular velocity of the link.
    ///
    /// - For performance, prefer PxArticulationCache::linkVelocity to get link spatial velocities in a batch query.
    /// - When the articulation state is updated via non-cache API, use PxArticulationReducedCoordinate::updateKinematic before querying velocity.
    ///
    /// The angular velocity of the link.
    ///
    /// This call is not allowed while the simulation is running except in a split simulation during [`PxScene::collide`]() and up to #PxScene::advance(),
    /// and in PxContactModifyCallback or in contact report callbacks.
    pub fn PxArticulationLink_getAngularVelocity(self_: *const PxArticulationLink) -> PxVec3;

    /// Returns the string name of the dynamic type.
    ///
    /// The string name.
    pub fn PxArticulationLink_getConcreteTypeName(self_: *const PxArticulationLink) -> *const std::ffi::c_char;

    /// Releases a PxConstraint instance.
    ///
    /// This call does not wake up the connected rigid bodies.
    pub fn PxConstraint_release_mut(self_: *mut PxConstraint);

    /// Retrieves the scene which this constraint belongs to.
    ///
    /// Owner Scene. NULL if not part of a scene.
    pub fn PxConstraint_getScene(self_: *const PxConstraint) -> *mut PxScene;

    /// Retrieves the actors for this constraint.
    pub fn PxConstraint_getActors(self_: *const PxConstraint, actor0: *mut *mut PxRigidActor, actor1: *mut *mut PxRigidActor);

    /// Sets the actors for this constraint.
    pub fn PxConstraint_setActors_mut(self_: *mut PxConstraint, actor0: *mut PxRigidActor, actor1: *mut PxRigidActor);

    /// Notify the scene that the constraint shader data has been updated by the application
    pub fn PxConstraint_markDirty_mut(self_: *mut PxConstraint);

    /// Retrieve the flags for this constraint
    ///
    /// the constraint flags
    pub fn PxConstraint_getFlags(self_: *const PxConstraint) -> PxConstraintFlags;

    /// Set the flags for this constraint
    ///
    /// default: PxConstraintFlag::eDRIVE_LIMITS_ARE_FORCES
    pub fn PxConstraint_setFlags_mut(self_: *mut PxConstraint, flags: PxConstraintFlags);

    /// Set a flag for this constraint
    pub fn PxConstraint_setFlag_mut(self_: *mut PxConstraint, flag: PxConstraintFlag, value: bool);

    /// Retrieve the constraint force most recently applied to maintain this constraint.
    ///
    /// It is not allowed to use this method while the simulation is running (except during PxScene::collide(),
    /// in PxContactModifyCallback or in contact report callbacks).
    pub fn PxConstraint_getForce(self_: *const PxConstraint, linear: *mut PxVec3, angular: *mut PxVec3);

    /// whether the constraint is valid.
    ///
    /// A constraint is valid if it has at least one dynamic rigid body or articulation link. A constraint that
    /// is not valid may not be inserted into a scene, and therefore a static actor to which an invalid constraint
    /// is attached may not be inserted into a scene.
    ///
    /// Invalid constraints arise only when an actor to which the constraint is attached has been deleted.
    pub fn PxConstraint_isValid(self_: *const PxConstraint) -> bool;

    /// Set the break force and torque thresholds for this constraint.
    ///
    /// If either the force or torque measured at the constraint exceed these thresholds the constraint will break.
    pub fn PxConstraint_setBreakForce_mut(self_: *mut PxConstraint, linear: f32, angular: f32);

    /// Retrieve the constraint break force and torque thresholds
    pub fn PxConstraint_getBreakForce(self_: *const PxConstraint, linear: *mut f32, angular: *mut f32);

    /// Set the minimum response threshold for a constraint row
    ///
    /// When using mass modification for a joint or infinite inertia for a jointed body, very stiff solver constraints can be generated which
    /// can destabilize simulation. Setting this value to a small positive value (e.g. 1e-8) will cause constraint rows to be ignored if very
    /// large changes in impulses will generate only small changes in velocity. When setting this value, also set
    /// PxConstraintFlag::eDISABLE_PREPROCESSING. The solver accuracy for this joint may be reduced.
    pub fn PxConstraint_setMinResponseThreshold_mut(self_: *mut PxConstraint, threshold: f32);

    /// Retrieve the constraint break force and torque thresholds
    ///
    /// the minimum response threshold for a constraint row
    pub fn PxConstraint_getMinResponseThreshold(self_: *const PxConstraint) -> f32;

    /// Fetch external owner of the constraint.
    ///
    /// Provides a reference to the external owner of a constraint and a unique owner type ID.
    ///
    /// Reference to the external object which owns the constraint.
    pub fn PxConstraint_getExternalReference_mut(self_: *mut PxConstraint, typeID: *mut u32) -> *mut std::ffi::c_void;

    /// Set the constraint functions for this constraint
    pub fn PxConstraint_setConstraintFunctions_mut(self_: *mut PxConstraint, connector: *mut PxConstraintConnector, shaders: *const PxConstraintShaderTable);

    pub fn PxConstraint_getConcreteTypeName(self_: *const PxConstraint) -> *const std::ffi::c_char;

    pub fn PxConstraint_getGPUIndex(self_: *const PxConstraint) -> u32;

    pub fn PxBaseMaterial_delete(self_: *mut PxBaseMaterial);

    pub fn PxBaseMaterial_isKindOf(self_: *const PxBaseMaterial, name: *const std::ffi::c_char) -> bool;

    /// Sets the coefficient of dynamic friction.
    ///
    /// The coefficient of dynamic friction should be in [0, PX_MAX_F32). If set to greater than staticFriction, the effective value of staticFriction will be increased to match.
    ///
    /// Sleeping:
    /// Does
    /// NOT
    /// wake any actors which may be affected.
    pub fn PxMaterial_setDynamicFriction_mut(self_: *mut PxMaterial, coef: f32);

    /// Retrieves the DynamicFriction value.
    ///
    /// The coefficient of dynamic friction.
    pub fn PxMaterial_getDynamicFriction(self_: *const PxMaterial) -> f32;

    /// Sets the coefficient of static friction
    ///
    /// The coefficient of static friction should be in the range [0, PX_MAX_F32)
    ///
    /// Sleeping:
    /// Does
    /// NOT
    /// wake any actors which may be affected.
    pub fn PxMaterial_setStaticFriction_mut(self_: *mut PxMaterial, coef: f32);

    /// Retrieves the coefficient of static friction.
    ///
    /// The coefficient of static friction.
    pub fn PxMaterial_getStaticFriction(self_: *const PxMaterial) -> f32;

    /// Sets the coefficient of restitution or the spring stiffness for compliant contact
    ///
    /// A coefficient of 0 makes the object bounce as little as possible, higher values up to 1.0 result in more bounce.
    /// If a negative value is provided it is interpreted as stiffness term for an implicit spring
    /// simulated at the contact site, with the spring positional error defined by
    /// the contact separation value. Higher stiffness terms produce stiffer springs that behave more like a rigid contact.
    ///
    /// Sleeping:
    /// Does
    /// NOT
    /// wake any actors which may be affected.
    pub fn PxMaterial_setRestitution_mut(self_: *mut PxMaterial, rest: f32);

    /// Retrieves the coefficient of restitution.
    ///
    /// See [`setRestitution`].
    ///
    /// The coefficient of restitution.
    pub fn PxMaterial_getRestitution(self_: *const PxMaterial) -> f32;

    /// Sets the coefficient of damping
    ///
    /// This property only affects the simulation if compliant contact mode is enabled, i.e., a negative restitution value is set.
    /// Damping works together with spring stiffness. Spring stiffness corrects positional error while
    /// damping resists relative velocity. Setting a high damping coefficient can produce spongy contacts.
    ///
    /// Sleeping:
    /// Does
    /// NOT
    /// wake any actors which may be affected.
    pub fn PxMaterial_setDamping_mut(self_: *mut PxMaterial, damping: f32);

    /// Retrieves the coefficient of damping.
    ///
    /// See [`setDamping`].
    ///
    /// The coefficient of damping.
    pub fn PxMaterial_getDamping(self_: *const PxMaterial) -> f32;

    /// Raises or clears a particular material flag.
    ///
    /// See the list of flags [`PxMaterialFlag`]
    ///
    /// Default:
    /// No flag raised.
    ///
    /// Sleeping:
    /// Does
    /// NOT
    /// wake any actors which may be affected.
    pub fn PxMaterial_setFlag_mut(self_: *mut PxMaterial, flag: PxMaterialFlag, b: bool);

    /// sets all the material flags.
    ///
    /// See the list of flags [`PxMaterialFlag`]
    ///
    /// Default:
    /// No flag raised.
    ///
    /// Sleeping:
    /// Does
    /// NOT
    /// wake any actors which may be affected.
    pub fn PxMaterial_setFlags_mut(self_: *mut PxMaterial, flags: PxMaterialFlags);

    /// Retrieves the flags. See [`PxMaterialFlag`].
    ///
    /// The material flags.
    pub fn PxMaterial_getFlags(self_: *const PxMaterial) -> PxMaterialFlags;

    /// Sets the friction combine mode.
    ///
    /// See the enum ::PxCombineMode .
    ///
    /// Default:
    /// PxCombineMode::eAVERAGE
    ///
    /// Sleeping:
    /// Does
    /// NOT
    /// wake any actors which may be affected.
    pub fn PxMaterial_setFrictionCombineMode_mut(self_: *mut PxMaterial, combMode: PxCombineMode);

    /// Retrieves the friction combine mode.
    ///
    /// See [`setFrictionCombineMode`].
    ///
    /// The friction combine mode for this material.
    pub fn PxMaterial_getFrictionCombineMode(self_: *const PxMaterial) -> PxCombineMode;

    /// Sets the restitution combine mode.
    ///
    /// See the enum ::PxCombineMode .
    ///
    /// Default:
    /// PxCombineMode::eAVERAGE
    ///
    /// Sleeping:
    /// Does
    /// NOT
    /// wake any actors which may be affected.
    pub fn PxMaterial_setRestitutionCombineMode_mut(self_: *mut PxMaterial, combMode: PxCombineMode);

    /// Retrieves the restitution combine mode.
    ///
    /// See [`setRestitutionCombineMode`].
    ///
    /// The coefficient of restitution combine mode for this material.
    pub fn PxMaterial_getRestitutionCombineMode(self_: *const PxMaterial) -> PxCombineMode;

    /// Sets the damping combine mode.
    ///
    /// See the enum ::PxCombineMode .
    ///
    /// Default:
    /// PxCombineMode::eAVERAGE
    ///
    /// Sleeping:
    /// Does
    /// NOT
    /// wake any actors which may be affected.
    pub fn PxMaterial_setDampingCombineMode_mut(self_: *mut PxMaterial, combMode: PxCombineMode);

    /// Retrieves the damping combine mode.
    ///
    /// The damping combine mode for this material.
    pub fn PxMaterial_getDampingCombineMode(self_: *const PxMaterial) -> PxCombineMode;

    pub fn PxMaterial_getConcreteTypeName(self_: *const PxMaterial) -> *const std::ffi::c_char;

    /// Constructor
    pub fn PxContactStreamIterator_new(contactPatches: *const u8, contactPoints: *const u8, contactFaceIndices: *const u32, nbPatches: u32, nbContacts: u32) -> PxContactStreamIterator;

    /// Returns whether there are more patches in this stream.
    ///
    /// Whether there are more patches in this stream.
    pub fn PxContactStreamIterator_hasNextPatch(self_: *const PxContactStreamIterator) -> bool;

    /// Returns the total contact count.
    ///
    /// Total contact count.
    pub fn PxContactStreamIterator_getTotalContactCount(self_: *const PxContactStreamIterator) -> u32;

    /// Returns the total patch count.
    ///
    /// Total patch count.
    pub fn PxContactStreamIterator_getTotalPatchCount(self_: *const PxContactStreamIterator) -> u32;

    /// Advances iterator to next contact patch.
    pub fn PxContactStreamIterator_nextPatch_mut(self_: *mut PxContactStreamIterator);

    /// Returns if the current patch has more contacts.
    ///
    /// If there are more contacts in the current patch.
    pub fn PxContactStreamIterator_hasNextContact(self_: *const PxContactStreamIterator) -> bool;

    /// Advances to the next contact in the patch.
    pub fn PxContactStreamIterator_nextContact_mut(self_: *mut PxContactStreamIterator);

    /// Gets the current contact's normal
    ///
    /// The current contact's normal.
    pub fn PxContactStreamIterator_getContactNormal(self_: *const PxContactStreamIterator) -> *const PxVec3;

    /// Gets the inverse mass scale for body 0.
    ///
    /// The inverse mass scale for body 0.
    pub fn PxContactStreamIterator_getInvMassScale0(self_: *const PxContactStreamIterator) -> f32;

    /// Gets the inverse mass scale for body 1.
    ///
    /// The inverse mass scale for body 1.
    pub fn PxContactStreamIterator_getInvMassScale1(self_: *const PxContactStreamIterator) -> f32;

    /// Gets the inverse inertia scale for body 0.
    ///
    /// The inverse inertia scale for body 0.
    pub fn PxContactStreamIterator_getInvInertiaScale0(self_: *const PxContactStreamIterator) -> f32;

    /// Gets the inverse inertia scale for body 1.
    ///
    /// The inverse inertia scale for body 1.
    pub fn PxContactStreamIterator_getInvInertiaScale1(self_: *const PxContactStreamIterator) -> f32;

    /// Gets the contact's max impulse.
    ///
    /// The contact's max impulse.
    pub fn PxContactStreamIterator_getMaxImpulse(self_: *const PxContactStreamIterator) -> f32;

    /// Gets the contact's target velocity.
    ///
    /// The contact's target velocity.
    pub fn PxContactStreamIterator_getTargetVel(self_: *const PxContactStreamIterator) -> *const PxVec3;

    /// Gets the contact's contact point.
    ///
    /// The contact's contact point.
    pub fn PxContactStreamIterator_getContactPoint(self_: *const PxContactStreamIterator) -> *const PxVec3;

    /// Gets the contact's separation.
    ///
    /// The contact's separation.
    pub fn PxContactStreamIterator_getSeparation(self_: *const PxContactStreamIterator) -> f32;

    /// Gets the contact's face index for shape 0.
    ///
    /// The contact's face index for shape 0.
    pub fn PxContactStreamIterator_getFaceIndex0(self_: *const PxContactStreamIterator) -> u32;

    /// Gets the contact's face index for shape 1.
    ///
    /// The contact's face index for shape 1.
    pub fn PxContactStreamIterator_getFaceIndex1(self_: *const PxContactStreamIterator) -> u32;

    /// Gets the contact's static friction coefficient.
    ///
    /// The contact's static friction coefficient.
    pub fn PxContactStreamIterator_getStaticFriction(self_: *const PxContactStreamIterator) -> f32;

    /// Gets the contact's dynamic friction coefficient.
    ///
    /// The contact's dynamic friction coefficient.
    pub fn PxContactStreamIterator_getDynamicFriction(self_: *const PxContactStreamIterator) -> f32;

    /// Gets the contact's restitution coefficient.
    ///
    /// The contact's restitution coefficient.
    pub fn PxContactStreamIterator_getRestitution(self_: *const PxContactStreamIterator) -> f32;

    /// Gets the contact's damping value.
    ///
    /// The contact's damping value.
    pub fn PxContactStreamIterator_getDamping(self_: *const PxContactStreamIterator) -> f32;

    /// Gets the contact's material flags.
    ///
    /// The contact's material flags.
    pub fn PxContactStreamIterator_getMaterialFlags(self_: *const PxContactStreamIterator) -> u32;

    /// Gets the contact's material index for shape 0.
    ///
    /// The contact's material index for shape 0.
    pub fn PxContactStreamIterator_getMaterialIndex0(self_: *const PxContactStreamIterator) -> u16;

    /// Gets the contact's material index for shape 1.
    ///
    /// The contact's material index for shape 1.
    pub fn PxContactStreamIterator_getMaterialIndex1(self_: *const PxContactStreamIterator) -> u16;

    /// Advances the contact stream iterator to a specific contact index.
    ///
    /// True if advancing was possible
    pub fn PxContactStreamIterator_advanceToIndex_mut(self_: *mut PxContactStreamIterator, initialIndex: u32) -> bool;

    /// Constructor
    pub fn PxFrictionAnchorStreamIterator_new(contactPatches: *const u8, frictionPatches: *const u8, patchCount: u32) -> PxFrictionAnchorStreamIterator;

    /// Check if there are more patches.
    ///
    /// true if there are more patches.
    pub fn PxFrictionAnchorStreamIterator_hasNextPatch(self_: *const PxFrictionAnchorStreamIterator) -> bool;

    /// Advance to the next patch.
    pub fn PxFrictionAnchorStreamIterator_nextPatch_mut(self_: *mut PxFrictionAnchorStreamIterator);

    /// Check if current patch has more friction anchors.
    ///
    /// true if there are more friction anchors in current patch.
    pub fn PxFrictionAnchorStreamIterator_hasNextFrictionAnchor(self_: *const PxFrictionAnchorStreamIterator) -> bool;

    /// Advance to the next friction anchor in the patch.
    pub fn PxFrictionAnchorStreamIterator_nextFrictionAnchor_mut(self_: *mut PxFrictionAnchorStreamIterator);

    /// Get the friction anchor's position.
    ///
    /// The friction anchor's position.
    pub fn PxFrictionAnchorStreamIterator_getPosition(self_: *const PxFrictionAnchorStreamIterator) -> *const PxVec3;

    /// Get the friction anchor's impulse.
    ///
    /// The friction anchor's impulse.
    pub fn PxFrictionAnchorStreamIterator_getImpulse(self_: *const PxFrictionAnchorStreamIterator) -> *const PxVec3;

    /// Get the friction anchor's normal.
    ///
    /// The friction anchor's normal.
    pub fn PxFrictionAnchorStreamIterator_getNormal(self_: *const PxFrictionAnchorStreamIterator) -> *const PxVec3;

    /// Get current patch's static friction coefficient.
    ///
    /// The patch's static friction coefficient.
    pub fn PxFrictionAnchorStreamIterator_getStaticFriction(self_: *const PxFrictionAnchorStreamIterator) -> f32;

    /// Get current patch's dynamic friction coefficient.
    ///
    /// The patch's dynamic friction coefficient.
    pub fn PxFrictionAnchorStreamIterator_getDynamicFriction(self_: *const PxFrictionAnchorStreamIterator) -> f32;

    /// Get current patch's combined material flags.
    ///
    /// The patch's combined material flags.
    pub fn PxFrictionAnchorStreamIterator_getMaterialFlags(self_: *const PxFrictionAnchorStreamIterator) -> PxMaterialFlags;

    /// Get the position of a specific contact point in the set.
    ///
    /// Position to the requested point in world space
    pub fn PxContactSet_getPoint(self_: *const PxContactSet, i: u32) -> *const PxVec3;

    /// Alter the position of a specific contact point in the set.
    pub fn PxContactSet_setPoint_mut(self_: *mut PxContactSet, i: u32, p: *const PxVec3);

    /// Get the contact normal of a specific contact point in the set.
    ///
    /// The requested normal in world space
    pub fn PxContactSet_getNormal(self_: *const PxContactSet, i: u32) -> *const PxVec3;

    /// Alter the contact normal of a specific contact point in the set.
    ///
    /// Changing the normal can cause contact points to be ignored.
    pub fn PxContactSet_setNormal_mut(self_: *mut PxContactSet, i: u32, n: *const PxVec3);

    /// Get the separation distance of a specific contact point in the set.
    ///
    /// The separation. Negative implies penetration.
    pub fn PxContactSet_getSeparation(self_: *const PxContactSet, i: u32) -> f32;

    /// Alter the separation of a specific contact point in the set.
    pub fn PxContactSet_setSeparation_mut(self_: *mut PxContactSet, i: u32, s: f32);

    /// Get the target velocity of a specific contact point in the set.
    ///
    /// The target velocity in world frame
    pub fn PxContactSet_getTargetVelocity(self_: *const PxContactSet, i: u32) -> *const PxVec3;

    /// Alter the target velocity of a specific contact point in the set.
    ///
    /// The sign of the velocity needs to be flipped depending on the order of the actors in the pair. There is no guarantee about the consistency of the order from frame to frame.
    pub fn PxContactSet_setTargetVelocity_mut(self_: *mut PxContactSet, i: u32, v: *const PxVec3);

    /// Get the face index with respect to the first shape of the pair for a specific contact point in the set.
    ///
    /// The face index of the first shape
    ///
    /// At the moment, the first shape is never a tri-mesh, therefore this function always returns PXC_CONTACT_NO_FACE_INDEX
    pub fn PxContactSet_getInternalFaceIndex0(self_: *const PxContactSet, i: u32) -> u32;

    /// Get the face index with respect to the second shape of the pair for a specific contact point in the set.
    ///
    /// The face index of the second shape
    pub fn PxContactSet_getInternalFaceIndex1(self_: *const PxContactSet, i: u32) -> u32;

    /// Get the maximum impulse for a specific contact point in the set.
    ///
    /// The maximum impulse
    pub fn PxContactSet_getMaxImpulse(self_: *const PxContactSet, i: u32) -> f32;

    /// Alter the maximum impulse for a specific contact point in the set.
    ///
    /// Must be nonnegative. If set to zero, the contact point will be ignored
    pub fn PxContactSet_setMaxImpulse_mut(self_: *mut PxContactSet, i: u32, s: f32);

    /// Get the restitution coefficient for a specific contact point in the set.
    ///
    /// The restitution coefficient
    pub fn PxContactSet_getRestitution(self_: *const PxContactSet, i: u32) -> f32;

    /// Alter the restitution coefficient for a specific contact point in the set.
    ///
    /// Valid ranges [0,1]
    pub fn PxContactSet_setRestitution_mut(self_: *mut PxContactSet, i: u32, r: f32);

    /// Get the static friction coefficient for a specific contact point in the set.
    ///
    /// The friction coefficient (dimensionless)
    pub fn PxContactSet_getStaticFriction(self_: *const PxContactSet, i: u32) -> f32;

    /// Alter the static friction coefficient for a specific contact point in the set.
    pub fn PxContactSet_setStaticFriction_mut(self_: *mut PxContactSet, i: u32, f: f32);

    /// Get the static friction coefficient for a specific contact point in the set.
    ///
    /// The friction coefficient
    pub fn PxContactSet_getDynamicFriction(self_: *const PxContactSet, i: u32) -> f32;

    /// Alter the static dynamic coefficient for a specific contact point in the set.
    pub fn PxContactSet_setDynamicFriction_mut(self_: *mut PxContactSet, i: u32, f: f32);

    /// Ignore the contact point.
    ///
    /// If a contact point is ignored then no force will get applied at this point. This can be used to disable collision in certain areas of a shape, for example.
    pub fn PxContactSet_ignore_mut(self_: *mut PxContactSet, i: u32);

    /// The number of contact points in the set.
    pub fn PxContactSet_size(self_: *const PxContactSet) -> u32;

    /// Returns the invMassScale of body 0
    ///
    /// A value
    /// <
    /// 1.0 makes this contact treat the body as if it had larger mass. A value of 0.f makes this contact
    /// treat the body as if it had infinite mass. Any value > 1.f makes this contact treat the body as if it had smaller mass.
    pub fn PxContactSet_getInvMassScale0(self_: *const PxContactSet) -> f32;

    /// Returns the invMassScale of body 1
    ///
    /// A value
    /// <
    /// 1.0 makes this contact treat the body as if it had larger mass. A value of 0.f makes this contact
    /// treat the body as if it had infinite mass. Any value > 1.f makes this contact treat the body as if it had smaller mass.
    pub fn PxContactSet_getInvMassScale1(self_: *const PxContactSet) -> f32;

    /// Returns the invInertiaScale of body 0
    ///
    /// A value
    /// <
    /// 1.0 makes this contact treat the body as if it had larger inertia. A value of 0.f makes this contact
    /// treat the body as if it had infinite inertia. Any value > 1.f makes this contact treat the body as if it had smaller inertia.
    pub fn PxContactSet_getInvInertiaScale0(self_: *const PxContactSet) -> f32;

    /// Returns the invInertiaScale of body 1
    ///
    /// A value
    /// <
    /// 1.0 makes this contact treat the body as if it had larger inertia. A value of 0.f makes this contact
    /// treat the body as if it had infinite inertia. Any value > 1.f makes this contact treat the body as if it had smaller inertia.
    pub fn PxContactSet_getInvInertiaScale1(self_: *const PxContactSet) -> f32;

    /// Sets the invMassScale of body 0
    ///
    /// This can be set to any value in the range [0, PX_MAX_F32). A value
    /// <
    /// 1.0 makes this contact treat the body as if it had larger mass. A value of 0.f makes this contact
    /// treat the body as if it had infinite mass. Any value > 1.f makes this contact treat the body as if it had smaller mass.
    pub fn PxContactSet_setInvMassScale0_mut(self_: *mut PxContactSet, scale: f32);

    /// Sets the invMassScale of body 1
    ///
    /// This can be set to any value in the range [0, PX_MAX_F32). A value
    /// <
    /// 1.0 makes this contact treat the body as if it had larger mass. A value of 0.f makes this contact
    /// treat the body as if it had infinite mass. Any value > 1.f makes this contact treat the body as if it had smaller mass.
    pub fn PxContactSet_setInvMassScale1_mut(self_: *mut PxContactSet, scale: f32);

    /// Sets the invInertiaScale of body 0
    ///
    /// This can be set to any value in the range [0, PX_MAX_F32). A value
    /// <
    /// 1.0 makes this contact treat the body as if it had larger inertia. A value of 0.f makes this contact
    /// treat the body as if it had infinite inertia. Any value > 1.f makes this contact treat the body as if it had smaller inertia.
    pub fn PxContactSet_setInvInertiaScale0_mut(self_: *mut PxContactSet, scale: f32);

    /// Sets the invInertiaScale of body 1
    ///
    /// This can be set to any value in the range [0, PX_MAX_F32). A value
    /// <
    /// 1.0 makes this contact treat the body as if it had larger inertia. A value of 0.f makes this contact
    /// treat the body as if it had infinite inertia. Any value > 1.f makes this contact treat the body as if it had smaller inertia.
    pub fn PxContactSet_setInvInertiaScale1_mut(self_: *mut PxContactSet, scale: f32);

    /// Passes modifiable arrays of contacts to the application.
    ///
    /// The initial contacts are regenerated from scratch each frame by collision detection.
    ///
    /// The number of contacts can not be changed, so you cannot add your own contacts.  You may however
    /// disable contacts using PxContactSet::ignore().
    pub fn PxContactModifyCallback_onContactModify_mut(self_: *mut PxContactModifyCallback, pairs: *mut PxContactModifyPair, count: u32);

    /// Passes modifiable arrays of contacts to the application.
    ///
    /// The initial contacts are regenerated from scratch each frame by collision detection.
    ///
    /// The number of contacts can not be changed, so you cannot add your own contacts.  You may however
    /// disable contacts using PxContactSet::ignore().
    pub fn PxCCDContactModifyCallback_onCCDContactModify_mut(self_: *mut PxCCDContactModifyCallback, pairs: *mut PxContactModifyPair, count: u32);

    /// Raises or clears a particular deformable body flag.
    ///
    /// See the list of flags [`PxDeformableBodyFlag`]
    ///
    /// Default:
    /// No flags are set
    pub fn PxDeformableBody_setDeformableBodyFlag_mut(self_: *mut PxDeformableBody, flag: PxDeformableBodyFlag, val: bool);

    /// Sets deformable body flags.
    ///
    /// See the list of flags [`PxDeformableBodyFlag`]
    ///
    /// Default:
    /// No flags are set
    pub fn PxDeformableBody_setDeformableBodyFlags_mut(self_: *mut PxDeformableBody, flags: PxDeformableBodyFlags);

    /// Reads the deformable body flags.
    ///
    /// See the list of flags [`PxDeformableBodyFlag`]
    ///
    /// The values of the deformable body flags.
    pub fn PxDeformableBody_getDeformableBodyFlags(self_: *const PxDeformableBody) -> PxDeformableBodyFlags;

    /// Sets the linear damping parameter.
    ///
    /// After every timestep the velocity is reduced while the magnitude of the
    /// reduction depends on the linearDamping value.
    pub fn PxDeformableBody_setLinearDamping_mut(self_: *mut PxDeformableBody, linearDamping: f32);

    /// Retrieves linear velocity damping parameter.
    ///
    /// The linear damping parameter
    pub fn PxDeformableBody_getLinearDamping(self_: *const PxDeformableBody) -> f32;

    /// Sets the maximal velocity vertices can reach
    ///
    /// Allows to limit the vertices' maximal velocity to control the maximal distance a vertex can move per frame
    /// Default:
    /// 1.0e32
    pub fn PxDeformableBody_setMaxLinearVelocity_mut(self_: *mut PxDeformableBody, maxLinearVelocity: f32);

    /// Retrieves maximal velocity a vertex can have.
    ///
    /// The maximal velocity
    pub fn PxDeformableBody_getMaxLinearVelocity(self_: *const PxDeformableBody) -> f32;

    /// Sets the maximal depenetration velocity vertices can reach
    ///
    /// Allows to limit the vertices' maximal depenetration velocity to avoid that collision responses lead to very high particle velocities
    /// Default:
    /// 1.0e32
    pub fn PxDeformableBody_setMaxDepenetrationVelocity_mut(self_: *mut PxDeformableBody, maxDepenetrationVelocity: f32);

    /// Retrieves maximal depenetration velocity a vertex can have.
    ///
    /// The maximal depenetration velocity
    pub fn PxDeformableBody_getMaxDepenetrationVelocity(self_: *const PxDeformableBody) -> f32;

    /// Sets the self collision filter distance.
    ///
    /// Penetration distance that needs to be exceeded before contacts for self collision are generated.
    /// Will only have an effect if self collisions are enabled.
    /// Default:
    /// 0.1
    pub fn PxDeformableBody_setSelfCollisionFilterDistance_mut(self_: *mut PxDeformableBody, selfCollisionFilterDistance: f32);

    /// Retrieves the self collision filter distance.
    ///
    /// The self collision filter distance
    pub fn PxDeformableBody_getSelfCollisionFilterDistance(self_: *const PxDeformableBody) -> f32;

    /// Sets the solver iteration count for the deformable body.
    ///
    /// Since deformables are currently implemented using an XPBD solver (extended position based dynamics), minVelocityIters is ignored.
    /// Default:
    /// 4 position iterations, 1 velocity iteration
    pub fn PxDeformableBody_setSolverIterationCounts_mut(self_: *mut PxDeformableBody, minPositionIters: u32, minVelocityIters: u32);

    /// Retrieves the solver iteration counts.
    pub fn PxDeformableBody_getSolverIterationCounts(self_: *const PxDeformableBody, minPositionIters: *mut u32, minVelocityIters: *mut u32);

    /// Sets the threshold controlling sleeping of the deformable body.
    ///
    /// Threshold that defines the maximal magnitude of the linear motion a deformable body can move in one second
    /// before it becomes a candidate for sleeping.
    pub fn PxDeformableBody_setSleepThreshold_mut(self_: *mut PxDeformableBody, sleepThreshold: f32);

    /// Retrieves the sleep threshold.
    ///
    /// The sleep threshold
    pub fn PxDeformableBody_getSleepThreshold(self_: *const PxDeformableBody) -> f32;

    /// Sets the threshold controlling settling phase before sleeping of the deformable body.
    ///
    /// Threshold that defines the maximal magnitude of the linear motion a deformable body can move
    /// in one second before it becomes a candidate for sleeping and settling damping is engaged.
    /// The settling threshold needs to be higher than the sleep threshold.
    /// Default:
    /// 0.1
    pub fn PxDeformableBody_setSettlingThreshold_mut(self_: *mut PxDeformableBody, settlingThreshold: f32);

    /// Retrieves the settling threshold.
    ///
    /// The settling threshold
    pub fn PxDeformableBody_getSettlingThreshold(self_: *const PxDeformableBody) -> f32;

    /// Sets the damping parameter used for settling phase.
    ///
    /// If the maximum linear velocity of the deformable body falls below the settling threshold, the deformable body
    /// enters the settling phase in which the settling damping is applied.
    ///
    /// Default:
    /// 10.0
    pub fn PxDeformableBody_setSettlingDamping_mut(self_: *mut PxDeformableBody, settlingDamping: f32);

    /// Retrieves settling damping parameter.
    ///
    /// The settling damping parameter
    pub fn PxDeformableBody_getSettlingDamping(self_: *const PxDeformableBody) -> f32;

    /// Sets the wake counter for the deformable body.
    ///
    /// The wake counter value determines the minimum amount of time until the deformable body can be put to sleep. Please note
    /// that a deformable body will not be put to sleep if any vertex velocity is above the specified threshold
    /// or if other awake objects are touching it.
    ///
    /// Passing in a positive value will wake the deformable body up automatically.
    ///
    /// Default:
    /// 0.4 (which corresponds to 20 frames for a time step of 0.02)
    pub fn PxDeformableBody_setWakeCounter_mut(self_: *mut PxDeformableBody, wakeCounterValue: f32);

    /// Returns the wake counter of the deformable body.
    ///
    /// The wake counter of the deformable body.
    pub fn PxDeformableBody_getWakeCounter(self_: *const PxDeformableBody) -> f32;

    /// Returns true if this deformable body is sleeping.
    ///
    /// When an actor does not move for a period of time, it is no longer simulated in order to save time. This state
    /// is called sleeping. However, because the object automatically wakes up when it is either touched by an awake object,
    /// or a sleep-affecting property is changed by the user, the entire sleep mechanism should be transparent to the user.
    ///
    /// A deformable volume can only go to sleep if all vertices are ready for sleeping. A deformable body is guaranteed to be awake
    /// if at least one of the following holds:
    ///
    /// The wake counter is positive (
    ///
    /// The velocity of any vertex is above the sleep threshold.
    ///
    /// If a deformable body is sleeping, the following state is guaranteed:
    ///
    /// The wake counter is zero.
    ///
    /// The linear velocity of all vertices is zero.
    ///
    /// When a deformable body gets inserted into a scene, it will be considered asleep if all the points above hold, else it will
    /// be treated as awake.
    ///
    /// It is invalid to use this method if the deformable body has not been added to a scene already.
    ///
    /// True if the deformable body is sleeping.
    pub fn PxDeformableBody_isSleeping(self_: *const PxDeformableBody) -> bool;

    /// Retrieve a shape pointer belonging to the actor.
    pub fn PxDeformableBody_getShape_mut(self_: *mut PxDeformableBody) -> *mut PxShape;

    /// Attaches a shape
    ///
    /// Attaches the shape to use for collision detection for deformable surfaces and volumes.
    /// Each deformable needs to have exactly one exclusive shape attached for simulation. If a shape has
    /// already been attached to a deformable, detachShape needs to be called prior to attaching
    /// a new shape.
    ///
    /// Deformable surfaces need a shape with triangle mesh geometry, which can be created with
    /// PxPhysics::createShape(const PxGeometry
    /// &
    /// , const PxDeformableSurfaceMaterial
    /// &
    /// material, bool, PxShapeFlags), or
    /// PxPhysics::createShape(const PxGeometry
    /// &
    /// , PxDeformableSurfaceMaterial*const*, PxU16, bool, PxShapeFlags)
    /// Deformable surfaces use the same triangle mesh for collision detection and dynamics computations.
    ///
    /// Deformable volumes need a shape with tetrahedron mesh geometry, which can be created with
    /// PxPhysics::createShape(const PxGeometry
    /// &
    /// , const PxDeformableVolumeMaterial
    /// &
    /// material, bool, PxShapeFlags), or
    /// PxPhysics::createShape(const PxGeometry
    /// &
    /// , PxDeformableVolumeMaterial*const*, PxU16, bool, PxShapeFlags)
    /// Deformable volumes additionally need a separate tetrahedron mesh for dynamics, which can be attached using
    /// PxDeformbleVolume::attachSimulationMesh.
    ///
    /// Returns true if the operation was successful
    pub fn PxDeformableBody_attachShape_mut(self_: *mut PxDeformableBody, shape: *mut PxShape) -> bool;

    /// Detaches the shape
    ///
    /// Detaches the shape used for collision detection.
    pub fn PxDeformableBody_detachShape_mut(self_: *mut PxDeformableBody);

    /// Returns the cuda context manager
    ///
    /// The cuda context manager
    pub fn PxDeformableBody_getCudaContextManager(self_: *const PxDeformableBody) -> *mut PxCudaContextManager;

    /// Raises or clears a particular deformable surface flag.
    ///
    /// See the list of flags [`PxDeformableSurfaceFlag`]
    ///
    /// Default:
    /// No flags are set
    pub fn PxDeformableSurface_setDeformableSurfaceFlag_mut(self_: *mut PxDeformableSurface, flag: PxDeformableSurfaceFlag, val: bool);

    /// Sets deformable surface flags.
    ///
    /// See the list of flags [`PxDeformableSurfaceFlag`]
    ///
    /// Default:
    /// No flags are set
    pub fn PxDeformableSurface_setDeformableSurfaceFlags_mut(self_: *mut PxDeformableSurface, flags: PxDeformableSurfaceFlags);

    /// Reads the deformable surface flags.
    ///
    /// See the list of flags [`PxDeformableSurfaceFlag`]
    ///
    /// The values of the deformable surface flags.
    pub fn PxDeformableSurface_getDeformableSurfaceFlags(self_: *const PxDeformableSurface) -> PxDeformableSurfaceFlags;

    /// Sets the number of collision pair updates per timestep.
    ///
    /// Collision pair is updated at least once per timestep and increasing the frequency provides better collision pairs.
    /// Default:
    /// 1
    pub fn PxDeformableSurface_setNbCollisionPairUpdatesPerTimestep_mut(self_: *mut PxDeformableSurface, frequency: u32);

    /// Retrieves number of collision pair updates per timestep.
    ///
    /// The number of collision pair updates per timestep.
    pub fn PxDeformableSurface_getNbCollisionPairUpdatesPerTimestep(self_: *const PxDeformableSurface) -> u32;

    /// Sets the number of collision substeps in each sub-timestep.
    ///
    /// Collision constraints can be applied multiple times in each sub-timestep
    /// Default:
    /// 1
    pub fn PxDeformableSurface_setNbCollisionSubsteps_mut(self_: *mut PxDeformableSurface, frequency: u32);

    /// Retrieves the number of collision substeps in each sub-timestep.
    ///
    /// The number of collision substeps in each sub-timestep.
    pub fn PxDeformableSurface_getNbCollisionSubsteps(self_: *const PxDeformableSurface) -> u32;

    /// Gets a pointer to a device buffer containing positions and inverse masses of the
    /// surface deformable.
    ///
    /// This function returns a pointer to device memory for the positions and inverse masses of
    /// the surface deformable. The device memory buffer is used to both initialize/update the vertices of the surface deformable and
    /// read the simulation results.
    ///
    /// It is mandatory to call PxDeformableSurface::markDirty() with PxDeformableSurfaceDataFlag::ePOSITION_INVMASS when
    /// updating data in this buffer.
    ///
    /// The simulation expects 4 consecutive floats for each vertex, aligned to a 16B boundary.
    /// The first 3 floats specify the positions and the last float specifies the inverse mass of the vertex.
    /// The size of the buffer is the number of vertices of the surface deformable mesh * sizeof(PxVec4).
    ///
    /// The device memory pointed to by this pointer is allocated when a shape is attached to the
    /// deformable surface. Calling PxDeformableSurface::detachShape() will deallocate the memory.
    ///
    /// It is not allowed to write to this buffer from the start of the PxScene::simulate() call
    /// until PxScene::fetchResults() returns. Reading the data is allowed once all the PhysX tasks
    /// have finished, reading the data during a completion task is explicitly allowed. The
    /// simulation will read and write directly from/into this buffer.
    ///
    /// It is the users' responsibility to initialize this buffer with the initial positions of
    /// the vertices of the surface deformable mesh.
    ///
    /// PxVec4* A pointer to a device buffer containing positions and inverse masses of
    /// the surface deformable mesh.
    pub fn PxDeformableSurface_getPositionInvMassBufferD_mut(self_: *mut PxDeformableSurface) -> *mut PxVec4;

    /// Gets a pointer to a device buffer containing velocities of the deformable surface.
    ///
    /// This function returns a pointer to device memory for the velocities of the deformable surface. This buffer
    /// is used to both initialize/update the vertices of the surface deformable and read the simulation results.
    ///
    /// It is mandatory to call PxDeformableSurface::markDirty() with PxDeformableSurfaceDataFlag::eVELOCITY when
    /// updating data in this buffer.
    ///
    /// The simulation expects 4 consecutive floats for each vertex, aligned to a 16B boundary. The
    /// first 3 floats specify the velocity of the vertex. The final float is unused. The size of
    /// the buffer is the number of vertices of the surface deformable mesh * sizeof(PxVec4).
    ///
    /// The device memory pointed to by this pointer is allocated when a shape is attached to the
    /// deformable surface. Calling PxDeformableSurface::detachShape() will deallocate the memory.
    ///
    /// It is not allowed to write to this buffer from the start of the PxScene::simulate() call
    /// until PxScene::fetchResults() returns. Reading the data is allowed once all the PhysX tasks
    /// have finished, reading the data during a completion task is explicitly allowed. The
    /// simulation will read and write directly from/into this buffer.
    ///
    /// It is the users' responsibility to initialize this buffer with the initial velocities of
    /// the vertices of the surface deformable mesh.
    ///
    /// PxVec4* A pointer to a device buffer containing the velocities of the surface deformable mesh.
    pub fn PxDeformableSurface_getVelocityBufferD_mut(self_: *mut PxDeformableSurface) -> *mut PxVec4;

    /// Gets a pointer to a device buffer containing the rest positions of the deformable surface.
    ///
    /// This function returns a pointer to device memory for the rest positions of the deformable surface.
    /// This buffer is used to initialize/update the rest positions of the vertices of the deformable surface.
    ///
    /// It is mandatory to call PxDeformableSurface::markDirty() with PxDeformableSurfaceDataFlag::eREST_POSITION when
    /// updating data in this buffer.
    ///
    /// The simulation expects 4 consecutive floats for each vertex, aligned to a 16B boundary.
    /// The first 3 specify the rest position. The last float is unused. The size of the buffer
    /// is the number of vertices of the surface deformable mesh * sizeof(PxVec4).
    ///
    /// The device memory pointed to by this pointer is allocated when a shape is attached to the
    /// deformable surface. Calling PxDeformableSurface::detachShape() will deallocate the memory.
    ///
    /// It is not allowed to write to this buffer from the start of the PxScene::simulate() call
    /// until PxScene::fetchResults() returns. Reading the data is allowed once all the PhysX tasks
    /// have finished, reading the data during a completion task is explicitly allowed. The
    /// simulation will read directly from this buffer.
    ///
    /// It is the users' responsibility to initialize this buffer with the initial rest positions of
    /// the vertices of the surface deformable mesh.
    ///
    /// PxVec4* A pointer to a device buffer containing the rest positions of the surface deformable mesh.
    pub fn PxDeformableSurface_getRestPositionBufferD_mut(self_: *mut PxDeformableSurface) -> *mut PxVec4;

    /// Marks per-vertex simulation state and configuration buffers dirty to signal to the simulation
    /// that changes have been made.
    ///
    /// Calling this function is required to notify the simulation of changes made in the positionInvMass,
    /// velocity and rest position buffers.
    ///
    /// This function can be called multiple times, and dirty flags are accumulated internally until
    /// PxScene::simulate() is called.
    pub fn PxDeformableSurface_markDirty_mut(self_: *mut PxDeformableSurface, flags: PxDeformableSurfaceDataFlags);

    /// Gets the concrete type name.
    ///
    /// The name of the concrete type.
    pub fn PxDeformableSurface_getConcreteTypeName(self_: *const PxDeformableSurface) -> *const std::ffi::c_char;

    /// Sets young's modulus which defines the body's stiffness
    ///
    /// Default:
    /// 1.e6
    pub fn PxDeformableMaterial_setYoungsModulus_mut(self_: *mut PxDeformableMaterial, young: f32);

    /// Retrieves the young's modulus value.
    ///
    /// The young's modulus value.
    pub fn PxDeformableMaterial_getYoungsModulus(self_: *const PxDeformableMaterial) -> f32;

    /// Sets the Poisson's ratio which defines the body's volume preservation.
    ///
    /// Default:
    /// 0.45
    pub fn PxDeformableMaterial_setPoissons_mut(self_: *mut PxDeformableMaterial, poisson: f32);

    /// Retrieves the Poisson's ratio.
    ///
    /// The Poisson's ratio.
    pub fn PxDeformableMaterial_getPoissons(self_: *const PxDeformableMaterial) -> f32;

    /// Sets the dynamic friction value which defines the strength of resistance when two objects slide relative to each other while in contact.
    ///
    /// Default:
    /// 0.0
    pub fn PxDeformableMaterial_setDynamicFriction_mut(self_: *mut PxDeformableMaterial, dynamicFriction: f32);

    /// Retrieves the dynamic friction value
    ///
    /// The dynamic friction value
    pub fn PxDeformableMaterial_getDynamicFriction(self_: *const PxDeformableMaterial) -> f32;

    /// Sets material damping
    pub fn PxDeformableMaterial_setElasticityDamping_mut(self_: *mut PxDeformableMaterial, elasticityDamping: f32);

    /// Retrieves the material damping.
    ///
    /// damping.
    pub fn PxDeformableMaterial_getElasticityDamping(self_: *const PxDeformableMaterial) -> f32;

    /// Sets material thickness
    ///
    /// Default:
    /// 0.001
    pub fn PxDeformableSurfaceMaterial_setThickness_mut(self_: *mut PxDeformableSurfaceMaterial, thickness: f32);

    /// Retrieves the material thickness.
    ///
    /// Default:
    /// 0.001
    ///
    /// thickness.
    pub fn PxDeformableSurfaceMaterial_getThickness(self_: *const PxDeformableSurfaceMaterial) -> f32;

    /// Sets material bending stiffness
    ///
    /// Default:
    /// 0.0
    pub fn PxDeformableSurfaceMaterial_setBendingStiffness_mut(self_: *mut PxDeformableSurfaceMaterial, bendingStiffness: f32);

    /// Retrieves the material bending stiffness.
    ///
    /// bendingStiffness.
    pub fn PxDeformableSurfaceMaterial_getBendingStiffness(self_: *const PxDeformableSurfaceMaterial) -> f32;

    /// Sets material bending damping
    pub fn PxDeformableSurfaceMaterial_setBendingDamping_mut(self_: *mut PxDeformableSurfaceMaterial, bendingDamping: f32);

    /// Retrieves the material bending damping.
    ///
    /// bending damping.
    pub fn PxDeformableSurfaceMaterial_getBendingDamping(self_: *const PxDeformableSurfaceMaterial) -> f32;

    /// Gets the concrete type name.
    ///
    /// The name of the concrete type.
    pub fn PxDeformableSurfaceMaterial_getConcreteTypeName(self_: *const PxDeformableSurfaceMaterial) -> *const std::ffi::c_char;

    /// Raises or clears a particular deformable volume flag.
    ///
    /// See the list of flags [`PxDeformableVolumeFlag`]
    ///
    /// Default:
    /// No flags are set
    pub fn PxDeformableVolume_setDeformableVolumeFlag_mut(self_: *mut PxDeformableVolume, flag: PxDeformableVolumeFlag, val: bool);

    /// Sets deformable volume flags.
    ///
    /// See the list of flags [`PxDeformableVolumeFlag`]
    ///
    /// Default:
    /// No flags are set
    pub fn PxDeformableVolume_setDeformableVolumeFlags_mut(self_: *mut PxDeformableVolume, flags: PxDeformableVolumeFlags);

    /// Reads the deformable volume flags.
    ///
    /// See the list of flags [`PxDeformableVolumeFlag`]
    ///
    /// The values of the deformable volume flags.
    pub fn PxDeformableVolume_getDeformableVolumeFlags(self_: *const PxDeformableVolume) -> PxDeformableVolumeFlags;

    /// Sets the self collision stress tolerance.
    ///
    /// Stress threshold to deactivate collision contacts in case the local stress magnitude exceeds the threshold.
    /// Default:
    /// 0.9
    pub fn PxDeformableVolume_setSelfCollisionStressTolerance_mut(self_: *mut PxDeformableVolume, selfCollisionStressTolerance: f32);

    /// Retrieves the self collision stress tolerance.
    ///
    /// The self collision filter distance
    pub fn PxDeformableVolume_getSelfCollisionStressTolerance(self_: *const PxDeformableVolume) -> f32;

    /// Gets a pointer to a device buffer containing positions and inverse masses of the
    /// collision mesh.
    ///
    /// This function returns a pointer to device memory for the positions and inverse masses of
    /// the deformable volume. This buffer is used to both initialize/update the collision mesh vertices
    /// of the deformable volume and read the simulation results.
    ///
    /// It is mandatory to call PxDeformableVolume::markDirty() with PxDeformableVolumeDataFlag::ePOSITION_INVMASS
    /// when updating data in this buffer.
    ///
    /// The simulation expects 4 consecutive floats for each vertex, aligned to a 16B boundary.
    /// The first 3 floats specify the vertex position and the last float contains the inverse mass of the
    /// vertex. The size of the buffer is the number of vertices of the collision mesh * sizeof(PxVec4).
    ///
    /// The device memory pointed to by this pointer is allocated when a shape is attached to the
    /// deformable volume. Calling PxDeformableVolume::detachShape() will deallocate the memory.
    ///
    /// It is not allowed to write to this buffer from the start of the PxScene::simulate() call
    /// until PxScene::fetchResults() returns. Reading the data is allowed once all the PhysX tasks
    /// have finished, reading the data during a completion task is explicitly allowed. The
    /// simulation will read and write directly from/into this buffer.
    ///
    /// It is the users' responsibility to initialize this buffer with the initial positions of
    /// the vertices of the collision mesh. See PxDeformableVolumeExt::allocateAndInitializeHostMirror(),
    /// PxDeformableVolumeExt::copyToDevice().
    ///
    /// PxVec4* A pointer to a device buffer containing positions and inverse masses of
    /// the collision mesh.
    pub fn PxDeformableVolume_getPositionInvMassBufferD_mut(self_: *mut PxDeformableVolume) -> *mut PxVec4;

    /// Gets a pointer to a device buffer containing rest positions of the collision mesh vertices.
    ///
    /// This function returns a pointer to device memory for the rest positions of the deformable volume collision
    /// mesh. This buffer is used to initialize the rest positions of the collision mesh vertices.
    ///
    /// It is mandatory to call PxDeformableVolume::markDirty() with PxDeformableVolumeDataFlag::eREST_POSITION when
    /// updating data in this buffer.
    ///
    /// The simulation expects 4 floats per vertex, aligned to a 16B boundary. The first 3 specify the
    /// rest position. The last float is unused. The size of the buffer is the number of vertices in
    /// the collision mesh * sizeof(PxVec4).
    ///
    /// The device memory pointed to by this pointer is allocated when a shape is attached to the deformable volume.
    /// Calling PxDeformableVolume::detachShape() will deallocate the memory.
    ///
    /// It is not allowed to write data into this buffer from the start of PxScene::simulate() until
    /// PxScene::fetchResults() returns.
    ///
    /// It is the users' responsibility to initialize this buffer with the initial rest positions of the
    /// vertices of the collision mesh. See PxDeformableVolumeExt::allocateAndInitializeHostMirror(),
    /// PxDeformableVolumeExt::copyToDevice().
    ///
    /// PxVec4* A pointer to a device buffer containing the rest positions of the collision mesh.
    pub fn PxDeformableVolume_getRestPositionBufferD_mut(self_: *mut PxDeformableVolume) -> *mut PxVec4;

    /// Gets a pointer to a device buffer containing the vertex positions of the simulation mesh.
    ///
    /// This function returns a pointer to device memory for the positions and inverse masses of the deformable volume
    /// simulation mesh. This buffer is used to both initialize/update the simulation mesh vertices
    /// of the deformable volume and read the simulation results.
    ///
    /// It is mandatory to call PxDeformableVolume::markDirty() with PxDeformableVolumeDataFlag::eSIM_POSITION_INVMASS when
    /// updating data in this buffer.
    ///
    /// The simulation expects 4 consecutive floats for each vertex, aligned to a 16B boundary. The
    /// first 3 floats specify the positions and the last float specifies the inverse mass of the vertex.
    /// The size of the buffer is the number of vertices of the simulation mesh * sizeof(PxVec4).
    ///
    /// The device memory pointed to by this pointer is allocated when a simulation mesh is attached to the
    /// deformable volume. Calling PxDeformableVolume::detachSimulationMesh() will deallocate the memory.
    ///
    /// It is not allowed to write to this buffer from the start of the PxScene::simulate() call
    /// until PxScene::fetchResults() returns. Reading the data is allowed once all the PhysX tasks
    /// have finished, reading the data during a completion task is explicitly allowed. The
    /// simulation will read and write directly from/into this buffer.
    ///
    /// It is the users' responsibility to initialize this buffer with the initial positions of
    /// the vertices of the simulation mesh. See PxDeformableVolumeExt::allocateAndInitializeHostMirror(),
    /// PxDeformableVolumeExt::copyToDevice().
    ///
    /// PxVec4* A pointer to a device buffer containing the vertex positions of the simulation mesh.
    pub fn PxDeformableVolume_getSimPositionInvMassBufferD_mut(self_: *mut PxDeformableVolume) -> *mut PxVec4;

    /// Gets a pointer to a device buffer containing the vertex velocities of the simulation mesh.
    ///
    /// This function returns a pointer to device memory for the velocities of the deformable volume simulation mesh
    /// vertices. This buffer is used to both initialize/update the simulation mesh vertex velocities
    /// of the deformable volume and read the simulation results.
    ///
    /// It is mandatory to call PxDeformableVolume::markDirty() with PxDeformableVolumeDataFlag::eSIM_VELOCITY when
    /// updating data in this buffer.
    ///
    /// The simulation expects 4 consecutive floats for each vertex, aligned to a 16B boundary. The
    /// first 3 specify the velocities for each vertex. The final float is unused. The size of the
    /// buffer is the number of vertices of the simulation mesh * sizeof(PxVec4).
    ///
    /// The device memory pointed to by this pointer is allocated when a simulation mesh is attached to the
    /// deformable volume. Calling PxDeformableVolume::detachSimulationMesh() will deallocate the memory.
    ///
    /// It is not allowed to write to this buffer from the start of the PxScene::simulate() call
    /// until PxScene::fetchResults() returns. Reading the data is allowed once all the PhysX tasks
    /// have finished, reading the data during a completion task is explicitly allowed. The
    /// simulation will read and write directly from/into this buffer.
    ///
    /// It is the users' responsibility to initialize this buffer with the initial velocities of
    /// the vertices of the simulation mesh. See PxDeformableVolumeExt::allocateAndInitializeHostMirror(),
    /// PxDeformableVolumeExt::copyToDevice().
    ///
    /// PxVec4*  A pointer to a device buffer containing the vertex velocities of the simulation mesh.
    pub fn PxDeformableVolume_getSimVelocityBufferD_mut(self_: *mut PxDeformableVolume) -> *mut PxVec4;

    /// Marks per-vertex simulation state and configuration buffers dirty to signal to the simulation
    /// that changes have been made.
    ///
    /// Calling this function is mandatory to notify the simulation of changes made in the positionInvMass,
    /// simPositionInvMass, simVelocity and rest position buffers.
    ///
    /// This function can be called multiple times, and dirty flags are accumulated internally until
    /// PxScene::simulate() is called.
    pub fn PxDeformableVolume_markDirty_mut(self_: *mut PxDeformableVolume, flags: PxDeformableVolumeDataFlags);

    /// Sets the device buffer containing the kinematic targets for this deformable volume.
    ///
    /// This function sets the kinematic targets for a deformable volume to a user-provided device buffer. This buffer is
    /// read by the simulation to obtain the target position for each vertex of the simulation mesh.
    ///
    /// The simulation expects 4 consecutive float for each vertex, aligned to a 16B boundary. The first 3
    /// floats specify the target positions. The last float determines (together with the flag argument)
    /// if the target is active or not.
    /// For a deformable volume with the flag PxDeformableBodyFlag::eKINEMATIC raised, all target positions are considered
    /// valid. In case a deformable volume has the PxDeformableVolumeFlag::ePARTIALLY_KINEMATIC raised, only target
    /// positions whose corresponding last float has been set to 0.f are considered valid target positions.
    ///
    /// The size of the buffer is the number of vertices of the simulation mesh * sizeof(PxVec4).
    ///
    /// It is the users responsibility to manage the memory pointed to by the input to this function,
    /// as well as guaranteeing the integrity of the input data. In particular, this means that it is
    /// not allowed to write this data from from the start of PxScene::simulate() until PxScene::fetchResults()
    /// returns. The memory is not allowed to be deallocated until PxScene::fetchResults() returns.
    ///
    /// Calling this function with a null pointer for the positions will clear the input and resume normal
    /// simulation. PxDeformableBodyFlag::eKINEMATIC or PxDeformableVolumeFlag::ePARTIALLY_KINEMATIC are ignored
    /// if no targets are set.
    ///
    /// This call is persistent across calls to PxScene::simulate(). Once this function is called, the
    /// simulation will look up the target positions from the same buffer for every call to PxScene::simulate().
    /// The user is allowed to update the target positions without calling this function again, provided that
    /// the synchronization requirements are adhered to (no changes between start of PxScene::simulate() until
    /// PxScene::fetchResults() returns).
    pub fn PxDeformableVolume_setKinematicTargetBufferD_mut(self_: *mut PxDeformableVolume, positions: *const PxVec4);

    /// Attaches a simulation mesh
    ///
    /// Attaches the simulation mesh (geometry) and a state containing inverse mass, rest pose
    /// etc. required to compute the deformation.
    ///
    /// Returns true if the operation was successful
    pub fn PxDeformableVolume_attachSimulationMesh_mut(self_: *mut PxDeformableVolume, simulationMesh: *mut PxTetrahedronMesh, deformableVolumeAuxData: *mut PxDeformableVolumeAuxData) -> bool;

    /// Detaches the simulation mesh
    ///
    /// Detaches the simulation mesh and simulation state used to compute the deformation.
    pub fn PxDeformableVolume_detachSimulationMesh_mut(self_: *mut PxDeformableVolume);

    /// Retrieves the simulation mesh pointer.
    ///
    /// Allows to access the geometry of the tetrahedral mesh used to compute the object's deformation
    ///
    /// Pointer to the simulation mesh
    pub fn PxDeformableVolume_getSimulationMesh_mut(self_: *mut PxDeformableVolume) -> *mut PxTetrahedronMesh;

    /// Const version of getSimulationMesh()
    pub fn PxDeformableVolume_getSimulationMesh(self_: *const PxDeformableVolume) -> *const PxTetrahedronMesh;

    /// Retrieve the collision mesh pointer.
    ///
    /// Allows to access the geometry of the tetrahedral mesh used to perform collision detection
    ///
    /// Pointer to the collision mesh
    pub fn PxDeformableVolume_getCollisionMesh_mut(self_: *mut PxDeformableVolume) -> *mut PxTetrahedronMesh;

    /// Const version of getCollisionMesh()
    pub fn PxDeformableVolume_getCollisionMesh(self_: *const PxDeformableVolume) -> *const PxTetrahedronMesh;

    /// Retrieves the simulation state pointer.
    ///
    /// Allows to access the additional data of the simulation mesh (inverse mass, rest state etc.).
    /// The geometry part of the data is stored in the simulation mesh.
    ///
    /// Pointer to the simulation state
    pub fn PxDeformableVolume_getDeformableVolumeAuxData_mut(self_: *mut PxDeformableVolume) -> *mut PxDeformableVolumeAuxData;

    /// const version of getDeformableVolumeAuxData()
    pub fn PxDeformableVolume_getDeformableVolumeAuxData(self_: *const PxDeformableVolume) -> *const PxDeformableVolumeAuxData;

    /// Returns the GPU deformable volume index.
    ///
    /// The GPU index, or 0xFFFFFFFF if the deformable volume is not in a scene.
    pub fn PxDeformableVolume_getGpuDeformableVolumeIndex_mut(self_: *mut PxDeformableVolume) -> u32;

    /// Gets the concrete type name.
    ///
    /// The name of the concrete type.
    pub fn PxDeformableVolume_getConcreteTypeName(self_: *const PxDeformableVolume) -> *const std::ffi::c_char;

    /// Adjusts a deformable volume kinematic target such that it is properly set as active or inactive. Inactive targets will not affect vertex position, they are ignored by the solver.
    ///
    /// The target with adjusted w component
    pub fn phys_PxConfigureDeformableVolumeKinematicTarget(target: *const PxVec4, isActive: bool) -> PxVec4;

    /// Sets up a deformable volume kinematic target such that it is properly set as active or inactive. Inactive targets will not affect vertex position, they are ignored by the solver.
    ///
    /// The target with configured w component
    pub fn phys_PxConfigureDeformableVolumeKinematicTarget_1(target: *const PxVec3, isActive: bool) -> PxVec4;

    /// Sets the material model.
    pub fn PxDeformableVolumeMaterial_setMaterialModel_mut(self_: *mut PxDeformableVolumeMaterial, model: PxDeformableVolumeMaterialModel);

    /// Retrieves the material model.
    ///
    /// The material model.
    pub fn PxDeformableVolumeMaterial_getMaterialModel(self_: *const PxDeformableVolumeMaterial) -> PxDeformableVolumeMaterialModel;

    /// Gets the concrete type name.
    ///
    /// The name of the concrete type.
    pub fn PxDeformableVolumeMaterial_getConcreteTypeName(self_: *const PxDeformableVolumeMaterial) -> *const std::ffi::c_char;

    /// Notification if an object or its memory gets released
    ///
    /// If release() gets called on a PxBase object, an eUSER_RELEASE event will get fired immediately. The object state can be queried in the callback but
    /// it is not allowed to change the state. Furthermore, when reading from the object it is the user's responsibility to make sure that no other thread
    /// is writing at the same time to the object (this includes the simulation itself, i.e., [`PxScene::fetchResults`]() must not get called at the same time).
    ///
    /// Calling release() on a PxBase object does not necessarily trigger its destructor immediately. For example, the object can be shared and might still
    /// be referenced by other objects or the simulation might still be running and accessing the object state. In such cases the destructor will be called
    /// as soon as it is safe to do so. After the destruction of the object and its memory, an eMEMORY_RELEASE event will get fired. In this case it is not
    /// allowed to dereference the object pointer in the callback.
    pub fn PxDeletionListener_onRelease_mut(self_: *mut PxDeletionListener, observed: *const PxBase, userData: *mut std::ffi::c_void, deletionEvent: PxDeletionEventFlag);

    /// Get positions and inverse masses for this particle buffer.
    ///
    /// A pointer to a device buffer containing the positions and inverse mass packed as PxVec4(pos.x, pos.y, pos.z, inverseMass).
    pub fn PxParticleBuffer_getPositionInvMasses(self_: *const PxParticleBuffer) -> *mut PxVec4;

    /// Get velocities for this particle buffer.
    ///
    /// A pointer to a device buffer containing the velocities packed as PxVec4(vel.x, vel.y, vel.z, 0.0f).
    pub fn PxParticleBuffer_getVelocities(self_: *const PxParticleBuffer) -> *mut PxVec4;

    /// Get phases for this particle buffer.
    ///
    /// See [`PxParticlePhaseFlag`]
    ///
    /// A pointer to a device buffer containing the per-particle phases for this particle buffer.
    pub fn PxParticleBuffer_getPhases(self_: *const PxParticleBuffer) -> *mut u32;

    /// Set the number of active particles for this particle buffer.
    ///
    /// The number of active particles can be
    /// <
    /// = PxParticleBuffer::getMaxParticles(). The particle system will simulate the first
    /// x particles in the [`PxParticleBuffer`], where x is the number of active particles.
    pub fn PxParticleBuffer_setNbActiveParticles_mut(self_: *mut PxParticleBuffer, nbActiveParticles: u32);

    /// Get the number of active particles for this particle buffer.
    ///
    /// The number of active particles.
    pub fn PxParticleBuffer_getNbActiveParticles(self_: *const PxParticleBuffer) -> u32;

    /// Get the maximum number particles this particle buffer can hold.
    ///
    /// The maximum number of particles is specified when creating a [`PxParticleBuffer`]. See #PxPhysics::createParticleBuffer.
    ///
    /// The maximum number of particles.
    pub fn PxParticleBuffer_getMaxParticles(self_: *const PxParticleBuffer) -> u32;

    /// Get the start index for the first particle of this particle buffer in the complete list of
    /// particles of the particle system this buffer is used in.
    ///
    /// The return value is only correct if the particle buffer is assigned to a particle system and at least
    /// one call to simulate() has been performed.
    ///
    /// The index of the first particle in the complete particle list.
    pub fn PxParticleBuffer_getFlatListStartIndex(self_: *const PxParticleBuffer) -> u32;

    /// Raise dirty flags on this particle buffer to communicate that the corresponding data has been updated
    /// by the user.
    ///
    /// See [`PxParticleBufferFlag`].
    pub fn PxParticleBuffer_raiseFlags_mut(self_: *mut PxParticleBuffer, flags: PxParticleBufferFlag);

    /// Release this buffer and deallocate all the memory.
    pub fn PxParticleBuffer_release_mut(self_: *mut PxParticleBuffer);

    /// Retrieve unique index that does not change over the lifetime of a PxParticleBuffer.
    pub fn PxParticleBuffer_getUniqueId(self_: *const PxParticleBuffer) -> u32;

    /// Sets a name string for the object that can be retrieved with getName().
    ///
    /// This is for debugging and is not used by the SDK. The string is not copied by the SDK,
    /// only the pointer is stored.
    ///
    /// Default:
    /// NULL
    pub fn PxParticleBuffer_setName_mut(self_: *mut PxParticleBuffer, name: *const std::ffi::c_char);

    /// Retrieves the name string set with setName().
    ///
    /// Name string associated with object.
    pub fn PxParticleBuffer_getName(self_: *const PxParticleBuffer) -> *const std::ffi::c_char;

    /// Construct parameters with default values.
    pub fn PxDiffuseParticleParams_new() -> PxDiffuseParticleParams;

    /// (re)sets the structure to the default.
    pub fn PxDiffuseParticleParams_setToDefault_mut(self_: *mut PxDiffuseParticleParams);

    /// Get a device buffer of positions and remaining lifetimes for the diffuse particles.
    ///
    /// A device buffer containing positions and lifetimes of diffuse particles packed as PxVec4(pos.x, pos.y, pos.z, lifetime).
    pub fn PxParticleAndDiffuseBuffer_getDiffusePositionLifeTime(self_: *const PxParticleAndDiffuseBuffer) -> *mut PxVec4;

    /// Get a device buffer of velocities for the diffuse particles.
    ///
    /// A device buffer containing velocities of diffuse particles.
    pub fn PxParticleAndDiffuseBuffer_getDiffuseVelocities(self_: *const PxParticleAndDiffuseBuffer) -> *mut PxVec4;

    /// Get number of currently active diffuse particles.
    ///
    /// The number of currently active diffuse particles.
    pub fn PxParticleAndDiffuseBuffer_getNbActiveDiffuseParticles(self_: *const PxParticleAndDiffuseBuffer) -> u32;

    /// Set the maximum possible number of diffuse particles for this buffer.
    ///
    /// Must be in the range [0, PxParticleAndDiffuseBuffer::getMaxDiffuseParticles()]
    pub fn PxParticleAndDiffuseBuffer_setMaxActiveDiffuseParticles_mut(self_: *mut PxParticleAndDiffuseBuffer, maxActiveDiffuseParticles: u32);

    /// Get maximum possible number of diffuse particles.
    ///
    /// The maximum possible number diffuse particles.
    pub fn PxParticleAndDiffuseBuffer_getMaxDiffuseParticles(self_: *const PxParticleAndDiffuseBuffer) -> u32;

    /// Set the parameters for diffuse particle simulation.
    ///
    /// See [`PxDiffuseParticleParams`]
    pub fn PxParticleAndDiffuseBuffer_setDiffuseParticleParams_mut(self_: *mut PxParticleAndDiffuseBuffer, params: *const PxDiffuseParticleParams);

    /// Get the parameters currently used for diffuse particle simulation.
    ///
    /// A PxDiffuseParticleParams structure.
    pub fn PxParticleAndDiffuseBuffer_getDiffuseParticleParams(self_: *const PxParticleAndDiffuseBuffer) -> PxDiffuseParticleParams;

    /// Sets friction
    pub fn PxPBDMaterial_setFriction_mut(self_: *mut PxPBDMaterial, friction: f32);

    /// Retrieves the friction value.
    ///
    /// The friction value.
    pub fn PxPBDMaterial_getFriction(self_: *const PxPBDMaterial) -> f32;

    /// Sets velocity damping term
    pub fn PxPBDMaterial_setDamping_mut(self_: *mut PxPBDMaterial, damping: f32);

    /// Retrieves the velocity damping term
    ///
    /// The velocity damping term.
    pub fn PxPBDMaterial_getDamping(self_: *const PxPBDMaterial) -> f32;

    /// Sets adhesion term
    pub fn PxPBDMaterial_setAdhesion_mut(self_: *mut PxPBDMaterial, adhesion: f32);

    /// Retrieves the adhesion term
    ///
    /// The adhesion term.
    pub fn PxPBDMaterial_getAdhesion(self_: *const PxPBDMaterial) -> f32;

    /// Sets gravity scale term
    pub fn PxPBDMaterial_setGravityScale_mut(self_: *mut PxPBDMaterial, scale: f32);

    /// Retrieves the gravity scale term
    ///
    /// The gravity scale term.
    pub fn PxPBDMaterial_getGravityScale(self_: *const PxPBDMaterial) -> f32;

    /// Sets material adhesion radius scale. This is multiplied by the particle rest offset to compute the fall-off distance
    /// at which point adhesion ceases to operate.
    pub fn PxPBDMaterial_setAdhesionRadiusScale_mut(self_: *mut PxPBDMaterial, scale: f32);

    /// Retrieves the adhesion radius scale.
    ///
    /// The adhesion radius scale.
    pub fn PxPBDMaterial_getAdhesionRadiusScale(self_: *const PxPBDMaterial) -> f32;

    /// Sets viscosity
    pub fn PxPBDMaterial_setViscosity_mut(self_: *mut PxPBDMaterial, viscosity: f32);

    /// Retrieves the viscosity value.
    ///
    /// The viscosity value.
    pub fn PxPBDMaterial_getViscosity(self_: *const PxPBDMaterial) -> f32;

    /// Sets material vorticity confinement coefficient
    pub fn PxPBDMaterial_setVorticityConfinement_mut(self_: *mut PxPBDMaterial, vorticityConfinement: f32);

    /// Retrieves the vorticity confinement coefficient.
    ///
    /// The vorticity confinement coefficient.
    pub fn PxPBDMaterial_getVorticityConfinement(self_: *const PxPBDMaterial) -> f32;

    /// Sets material surface tension coefficient
    pub fn PxPBDMaterial_setSurfaceTension_mut(self_: *mut PxPBDMaterial, surfaceTension: f32);

    /// Retrieves the surface tension coefficient.
    ///
    /// The surface tension coefficient.
    pub fn PxPBDMaterial_getSurfaceTension(self_: *const PxPBDMaterial) -> f32;

    /// Sets material cohesion coefficient
    pub fn PxPBDMaterial_setCohesion_mut(self_: *mut PxPBDMaterial, cohesion: f32);

    /// Retrieves the cohesion coefficient.
    ///
    /// The cohesion coefficient.
    pub fn PxPBDMaterial_getCohesion(self_: *const PxPBDMaterial) -> f32;

    /// Sets material lift coefficient
    ///
    /// Particle-cloth, -rigids, -attachments and -volumes have been deprecated.
    pub fn PxPBDMaterial_setLift_mut(self_: *mut PxPBDMaterial, lift: f32);

    /// Retrieves the lift coefficient.
    ///
    /// Particle-cloth, -rigids, -attachments and -volumes have been deprecated.
    ///
    /// The lift coefficient.
    pub fn PxPBDMaterial_getLift(self_: *const PxPBDMaterial) -> f32;

    /// Sets material drag coefficient
    ///
    /// Particle-cloth, -rigids, -attachments and -volumes have been deprecated.
    pub fn PxPBDMaterial_setDrag_mut(self_: *mut PxPBDMaterial, drag: f32);

    /// Retrieves the drag coefficient.
    ///
    /// Particle-cloth, -rigids, -attachments and -volumes have been deprecated.
    ///
    /// The drag coefficient.
    pub fn PxPBDMaterial_getDrag(self_: *const PxPBDMaterial) -> f32;

    /// Sets the CFL coefficient. Limits the relative motion between two approaching fluid particles.
    ///
    /// The distance to which the motion is clamped is defined by CFLcoefficient*particleContactOffset*2.
    /// A value of 0.5 will thus limit the appoaching motion to a distance of particleContactOffset.
    /// A value much larger than one will typically not limit the motion of the particles.
    pub fn PxPBDMaterial_setCFLCoefficient_mut(self_: *mut PxPBDMaterial, coefficient: f32);

    /// Retrieves the CFL coefficient.
    ///
    /// The CFL coefficient.
    pub fn PxPBDMaterial_getCFLCoefficient(self_: *const PxPBDMaterial) -> f32;

    /// Sets material particle friction scale. This allows the application to scale up/down the frictional effect between particles independent of the friction
    /// coefficient, which also defines frictional behavior between the particle and rigid bodies/soft bodies/cloth etc.
    pub fn PxPBDMaterial_setParticleFrictionScale_mut(self_: *mut PxPBDMaterial, scale: f32);

    /// Retrieves the particle friction scale.
    ///
    /// The particle friction scale.
    pub fn PxPBDMaterial_getParticleFrictionScale(self_: *const PxPBDMaterial) -> f32;

    /// Sets material particle adhesion scale value. This is the adhesive value between particles defined as a scaled multiple of the adhesion parameter.
    pub fn PxPBDMaterial_setParticleAdhesionScale_mut(self_: *mut PxPBDMaterial, adhesion: f32);

    /// Retrieves the particle adhesion scale value.
    ///
    /// The particle adhesion scale value.
    pub fn PxPBDMaterial_getParticleAdhesionScale(self_: *const PxPBDMaterial) -> f32;

    pub fn PxPBDMaterial_getConcreteTypeName(self_: *const PxPBDMaterial) -> *const std::ffi::c_char;

    pub fn PxDeformableAttachmentData_new() -> PxDeformableAttachmentData;

    /// Gets the two actors for this attachment.
    pub fn PxDeformableAttachment_getActors(self_: *const PxDeformableAttachment, actor0: *mut *mut PxActor, actor1: *mut *mut PxActor);

    /// Updates the pose of the attachment.
    pub fn PxDeformableAttachment_updatePose_mut(self_: *mut PxDeformableAttachment, pose: *const PxTransform);

    /// Returns string name of PxDeformableAttachment, used for serialization
    pub fn PxDeformableAttachment_getConcreteTypeName(self_: *const PxDeformableAttachment) -> *const std::ffi::c_char;

    pub fn PxDeformableElementFilterData_new() -> PxDeformableElementFilterData;

    /// Gets the actors for this element filter.
    pub fn PxDeformableElementFilter_getActors(self_: *const PxDeformableElementFilter, actor0: *mut *mut PxActor, actor1: *mut *mut PxActor);

    /// Returns string name of PxDeformableElementFilter, used for serialization
    pub fn PxDeformableElementFilter_getConcreteTypeName(self_: *const PxDeformableElementFilter) -> *const std::ffi::c_char;

    /// Destroys the instance it is called on.
    ///
    /// Use this release method to destroy an instance of this class. Be sure
    /// to not keep a reference to this object after calling release.
    /// Avoid release calls while a scene is simulating (in between simulate() and fetchResults() calls).
    ///
    /// Note that this must be called once for each prior call to PxCreatePhysics, as
    /// there is a reference counter. Also note that you mustn't destroy the PxFoundation instance (holding the allocator, error callback etc.)
    /// until after the reference count reaches 0 and the SDK is actually removed.
    ///
    /// Releasing an SDK will also release any objects created through it (scenes, triangle meshes, convex meshes, heightfields, shapes etc.),
    /// provided the user hasn't already done so.
    ///
    /// Releasing the PxPhysics instance is a prerequisite to releasing the PxFoundation instance.
    pub fn PxPhysics_release_mut(self_: *mut PxPhysics);

    /// Retrieves the Foundation instance.
    ///
    /// A reference to the Foundation object.
    pub fn PxPhysics_getFoundation_mut(self_: *mut PxPhysics) -> *mut PxFoundation;

    /// Gets PxPhysics object insertion interface.
    ///
    /// The insertion interface is needed for PxCreateTriangleMesh, PxCooking::createTriangleMesh etc., this allows runtime mesh creation.
    pub fn PxPhysics_getPhysicsInsertionCallback_mut(self_: *mut PxPhysics) -> *mut PxInsertionCallback;

    /// Retrieves the PxOmniPvd instance if there is one registered with PxPhysics.
    ///
    /// A pointer to a PxOmniPvd object.
    pub fn PxPhysics_getOmniPvd_mut(self_: *mut PxPhysics) -> *mut PxOmniPvd;

    /// Returns the simulation tolerance parameters.
    ///
    /// The current simulation tolerance parameters.
    pub fn PxPhysics_getTolerancesScale(self_: *const PxPhysics) -> *const PxTolerancesScale;

    /// Creates an aggregate with the specified maximum size and filtering hint.
    ///
    /// The previous API used "bool enableSelfCollision" which should now silently evaluates
    /// to a PxAggregateType::eGENERIC aggregate with its self-collision bit.
    ///
    /// Use PxAggregateType::eSTATIC or PxAggregateType::eKINEMATIC for aggregates that will
    /// only contain static or kinematic actors. This provides faster filtering when used in
    /// combination with PxPairFilteringMode.
    ///
    /// The new aggregate.
    pub fn PxPhysics_createAggregate_mut(self_: *mut PxPhysics, maxActor: u32, maxShape: u32, filterHint: u32) -> *mut PxAggregate;

    /// Return the number of aggregates that currently exist.
    ///
    /// Number of aggregates.
    pub fn PxPhysics_getNbAggregates(self_: *const PxPhysics) -> u32;

    /// Creates a triangle mesh object.
    ///
    /// This can then be instanced into [`PxShape`] objects.
    ///
    /// The new triangle mesh.
    pub fn PxPhysics_createTriangleMesh_mut(self_: *mut PxPhysics, stream: *mut PxInputStream) -> *mut PxTriangleMesh;

    /// Return the number of triangle meshes that currently exist.
    ///
    /// Number of triangle meshes.
    pub fn PxPhysics_getNbTriangleMeshes(self_: *const PxPhysics) -> u32;

    /// Writes the array of triangle mesh pointers to a user buffer.
    ///
    /// Returns the number of pointers written.
    ///
    /// The ordering of the triangle meshes in the array is not specified.
    ///
    /// The number of triangle mesh pointers written to userBuffer, this should be less or equal to bufferSize.
    pub fn PxPhysics_getTriangleMeshes(self_: *const PxPhysics, userBuffer: *mut *mut PxTriangleMesh, bufferSize: u32, startIndex: u32) -> u32;

    /// Creates a tetrahedron mesh object.
    ///
    /// This can then be instanced into [`PxShape`] objects.
    ///
    /// The new tetrahedron mesh.
    pub fn PxPhysics_createTetrahedronMesh_mut(self_: *mut PxPhysics, stream: *mut PxInputStream) -> *mut PxTetrahedronMesh;

    /// Return the number of tetrahedron meshes that currently exist.
    ///
    /// Number of tetrahedron meshes.
    pub fn PxPhysics_getNbTetrahedronMeshes(self_: *const PxPhysics) -> u32;

    /// Writes the array of tetrahedron mesh pointers to a user buffer.
    ///
    /// Returns the number of pointers written.
    ///
    /// The ordering of the tetrahedron meshes in the array is not specified.
    ///
    /// The number of tetrahedron mesh pointers written to userBuffer, this should be less or equal to bufferSize.
    pub fn PxPhysics_getTetrahedronMeshes(self_: *const PxPhysics, userBuffer: *mut *mut PxTetrahedronMesh, bufferSize: u32, startIndex: u32) -> u32;

    /// Creates a heightfield object from previously cooked stream.
    ///
    /// This can then be instanced into [`PxShape`] objects.
    ///
    /// The new heightfield.
    pub fn PxPhysics_createHeightField_mut(self_: *mut PxPhysics, stream: *mut PxInputStream) -> *mut PxHeightField;

    /// Return the number of heightfields that currently exist.
    ///
    /// Number of heightfields.
    pub fn PxPhysics_getNbHeightFields(self_: *const PxPhysics) -> u32;

    /// Writes the array of heightfield pointers to a user buffer.
    ///
    /// Returns the number of pointers written.
    ///
    /// The ordering of the heightfields in the array is not specified.
    ///
    /// The number of heightfield pointers written to userBuffer, this should be less or equal to bufferSize.
    pub fn PxPhysics_getHeightFields(self_: *const PxPhysics, userBuffer: *mut *mut PxHeightField, bufferSize: u32, startIndex: u32) -> u32;

    /// Creates a convex mesh object.
    ///
    /// This can then be instanced into [`PxShape`] objects.
    ///
    /// The new convex mesh.
    pub fn PxPhysics_createConvexMesh_mut(self_: *mut PxPhysics, stream: *mut PxInputStream) -> *mut PxConvexMesh;

    /// Return the number of convex meshes that currently exist.
    ///
    /// Number of convex meshes.
    pub fn PxPhysics_getNbConvexMeshes(self_: *const PxPhysics) -> u32;

    /// Writes the array of convex mesh pointers to a user buffer.
    ///
    /// Returns the number of pointers written.
    ///
    /// The ordering of the convex meshes in the array is not specified.
    ///
    /// The number of convex mesh pointers written to userBuffer, this should be less or equal to bufferSize.
    pub fn PxPhysics_getConvexMeshes(self_: *const PxPhysics, userBuffer: *mut *mut PxConvexMesh, bufferSize: u32, startIndex: u32) -> u32;

    /// Creates a deformable volume mesh object.
    ///
    /// The new deformable volume mesh.
    pub fn PxPhysics_createDeformableVolumeMesh_mut(self_: *mut PxPhysics, stream: *mut PxInputStream) -> *mut PxDeformableVolumeMesh;

    /// Creates a bounding volume hierarchy.
    ///
    /// The new BVH.
    pub fn PxPhysics_createBVH_mut(self_: *mut PxPhysics, stream: *mut PxInputStream) -> *mut PxBVH;

    /// Return the number of bounding volume hierarchies that currently exist.
    ///
    /// Number of bounding volume hierarchies.
    pub fn PxPhysics_getNbBVHs(self_: *const PxPhysics) -> u32;

    /// Writes the array of bounding volume hierarchy pointers to a user buffer.
    ///
    /// Returns the number of pointers written.
    ///
    /// The ordering of the BVHs in the array is not specified.
    ///
    /// The number of BVH pointers written to userBuffer, this should be less or equal to bufferSize.
    pub fn PxPhysics_getBVHs(self_: *const PxPhysics, userBuffer: *mut *mut PxBVH, bufferSize: u32, startIndex: u32) -> u32;

    /// Creates a scene.
    ///
    /// Every scene uses a Thread Local Storage slot. This imposes a platform specific limit on the
    /// number of scenes that can be created.
    ///
    /// The new scene object.
    pub fn PxPhysics_createScene_mut(self_: *mut PxPhysics, sceneDesc: *const PxSceneDesc) -> *mut PxScene;

    /// Gets number of created scenes.
    ///
    /// The number of scenes created.
    pub fn PxPhysics_getNbScenes(self_: *const PxPhysics) -> u32;

    /// Writes the array of scene pointers to a user buffer.
    ///
    /// Returns the number of pointers written.
    ///
    /// The ordering of the scene pointers in the array is not specified.
    ///
    /// The number of scene pointers written to userBuffer, this should be less or equal to bufferSize.
    pub fn PxPhysics_getScenes(self_: *const PxPhysics, userBuffer: *mut *mut PxScene, bufferSize: u32, startIndex: u32) -> u32;

    /// Creates a static rigid actor with the specified pose and all other fields initialized
    /// to their default values.
    pub fn PxPhysics_createRigidStatic_mut(self_: *mut PxPhysics, pose: *const PxTransform) -> *mut PxRigidStatic;

    /// Creates a dynamic rigid actor with the specified pose and all other fields initialized
    /// to their default values.
    pub fn PxPhysics_createRigidDynamic_mut(self_: *mut PxPhysics, pose: *const PxTransform) -> *mut PxRigidDynamic;

    /// Creates a pruning structure from actors.
    ///
    /// Every provided actor needs at least one shape with the eSCENE_QUERY_SHAPE flag set.
    ///
    /// Both static and dynamic actors can be provided.
    ///
    /// It is not allowed to pass in actors which are already part of a scene.
    ///
    /// Articulation links cannot be provided.
    ///
    /// Pruning structure created from given actors, or NULL if any of the actors did not comply with the above requirements.
    pub fn PxPhysics_createPruningStructure_mut(self_: *mut PxPhysics, actors: *const *mut PxRigidActor, nbActors: u32) -> *mut PxPruningStructure;

    /// Creates a shape which may be attached to multiple actors
    ///
    /// The shape will be created with a reference count of 1.
    ///
    /// The shape
    ///
    /// Shared shapes are not mutable when they are attached to an actor
    pub fn PxPhysics_createShape_mut(self_: *mut PxPhysics, geometry: *const PxGeometry, material: *const PxMaterial, isExclusive: bool, shapeFlags: PxShapeFlags) -> *mut PxShape;

    /// Creates a shape which may be attached to exactly one deformable volume actor
    ///
    /// The shape will be created with a reference count of 1.
    ///
    /// The shape
    ///
    /// Shared shapes are not mutable when they are attached to an actor
    pub fn PxPhysics_createShape_mut_1(self_: *mut PxPhysics, geometry: *const PxGeometry, material: *const PxDeformableVolumeMaterial, isExclusive: bool, shapeFlags: PxShapeFlags) -> *mut PxShape;

    /// Creates a shape which may be attached to exactly one deformable surface actor
    ///
    /// The shape will be created with a reference count of 1.
    ///
    /// The shape
    ///
    /// Shared shapes are not mutable when they are attached to an actor
    pub fn PxPhysics_createShape_mut_2(self_: *mut PxPhysics, geometry: *const PxGeometry, material: *const PxDeformableSurfaceMaterial, isExclusive: bool, shapeFlags: PxShapeFlags) -> *mut PxShape;

    /// Creates a shape which may be attached to multiple actors
    ///
    /// The shape will be created with a reference count of 1.
    ///
    /// The shape
    ///
    /// Shared shapes are not mutable when they are attached to an actor
    ///
    /// Shapes created from *SDF* triangle-mesh geometries do not support more than one material.
    pub fn PxPhysics_createShape_mut_3(self_: *mut PxPhysics, geometry: *const PxGeometry, materials: *const *mut PxMaterial, materialCount: u16, isExclusive: bool, shapeFlags: PxShapeFlags) -> *mut PxShape;

    pub fn PxPhysics_createShape_mut_4(self_: *mut PxPhysics, geometry: *const PxGeometry, materials: *const *mut PxDeformableSurfaceMaterial, materialCount: u16, isExclusive: bool, shapeFlags: PxShapeFlags) -> *mut PxShape;

    pub fn PxPhysics_createShape_mut_5(self_: *mut PxPhysics, geometry: *const PxGeometry, materials: *const *mut PxDeformableVolumeMaterial, materialCount: u16, isExclusive: bool, shapeFlags: PxShapeFlags) -> *mut PxShape;

    /// Return the number of shapes that currently exist.
    ///
    /// Number of shapes.
    pub fn PxPhysics_getNbShapes(self_: *const PxPhysics) -> u32;

    /// Writes the array of shape pointers to a user buffer.
    ///
    /// Returns the number of pointers written.
    ///
    /// The ordering of the shapes in the array is not specified.
    ///
    /// The number of shape pointers written to userBuffer, this should be less or equal to bufferSize.
    pub fn PxPhysics_getShapes(self_: *const PxPhysics, userBuffer: *mut *mut PxShape, bufferSize: u32, startIndex: u32) -> u32;

    /// Creates a constraint shader.
    ///
    /// A constraint shader will get added automatically to the scene the two linked actors belong to. Either, but not both, of actor0 and actor1 may
    /// be NULL to denote attachment to the world.
    ///
    /// The new constraint shader.
    pub fn PxPhysics_createConstraint_mut(self_: *mut PxPhysics, actor0: *mut PxRigidActor, actor1: *mut PxRigidActor, connector: *mut PxConstraintConnector, shaders: *const PxConstraintShaderTable, dataSize: u32) -> *mut PxConstraint;

    /// Return the number of constraints that currently exist.
    ///
    /// Number of constraints.
    pub fn PxPhysics_getNbConstraints(self_: *const PxPhysics) -> u32;

    /// Creates a reduced-coordinate articulation with all fields initialized to their default values.
    ///
    /// the new articulation
    pub fn PxPhysics_createArticulationReducedCoordinate_mut(self_: *mut PxPhysics) -> *mut PxArticulationReducedCoordinate;

    /// Return the number of articulations that currently exist.
    ///
    /// Number of articulations.
    pub fn PxPhysics_getNbArticulations(self_: *const PxPhysics) -> u32;

    /// Creates an attachment between two actors, based on the provided PxDeformableAttachmentData. At least one of the actors must be a deformable.
    ///
    /// An attachment is a collection of one or more positional constraints between a point on one actor and a point on another actor.
    /// Attachments between two rigid objects are not permitted, use joints instead.
    ///
    /// The attachment is only active when both actors are added to the same scene or one of the actors is NULL.
    ///
    /// The PxDeformableAttachment created if successful, NULL otherwise.
    pub fn PxPhysics_createDeformableAttachment_mut(self_: *mut PxPhysics, data: *const PxDeformableAttachmentData) -> *mut PxDeformableAttachment;

    /// Creates an element-level collision filter between two actors, based on the provided PxDeformableElementFilterData. At least one of the actors must be a deformable.
    ///
    /// Element filters define how parts of deformable actors are excluded from collisions.
    /// They are usually added to avoid conflicting attachment and contact constraints.
    ///
    /// The element filter is only active when both actors are added to the same scene or one of the actors is NULL.
    ///
    /// The PxDeformableElementFilter created if successful, NULL otherwise.
    pub fn PxPhysics_createDeformableElementFilter_mut(self_: *mut PxPhysics, data: *const PxDeformableElementFilterData) -> *mut PxDeformableElementFilter;

    /// Creates a deformable surface with all fields initialized to their default values.
    ///
    /// the new deformable surface
    pub fn PxPhysics_createDeformableSurface_mut(self_: *mut PxPhysics, cudaContextManager: *mut PxCudaContextManager) -> *mut PxDeformableSurface;

    /// Creates a FEM-based deformable volume with all fields initialized to their default values.
    ///
    /// the new deformable volume
    pub fn PxPhysics_createDeformableVolume_mut(self_: *mut PxPhysics, cudaContextManager: *mut PxCudaContextManager) -> *mut PxDeformableVolume;

    /// Creates a particle system with a position-based dynamics (PBD) solver.
    ///
    /// A PBD particle system can be used to simulate particle systems with fluid and granular particles. It also allows simulating cloth using
    /// mass-spring constraints and rigid bodies by shape matching the bodies with particles.
    ///
    /// In order to accelerate neighborhood finding for particle-particle interactions (e.g.: for fluid density constraints) a regular grid is used.
    /// This grid is built every time step but may provide inaccurate neighborhood information during the solver iterations. The neighborhood scale
    /// parameter can be used to configure the grid such that it provides a more conservative neighborhood at the cost of run-time performance.
    /// The grid cell width is defined as 2*particleContactOffset*neighborhoodScale.
    ///
    /// The maxNeighborhood defines how many particles fit into the neighborhood, at the cost of memory.
    ///
    /// Both maxNeighborhood and neighborhoodScale should be set as low as possible for performance, but high enough to not cause any behavioral degredation.
    ///
    /// the new particle system
    pub fn PxPhysics_createPBDParticleSystem_mut(self_: *mut PxPhysics, cudaContextManager: *mut PxCudaContextManager, maxNeighborhood: u32, neighborhoodScale: f32) -> *mut PxPBDParticleSystem;

    /// Create particle buffer to simulate fluid/granular material.
    ///
    /// PxParticleBuffer instance
    pub fn PxPhysics_createParticleBuffer_mut(self_: *mut PxPhysics, maxParticles: u32, cudaContextManager: *mut PxCudaContextManager) -> *mut PxParticleBuffer;

    /// Create a particle buffer for fluid dynamics with diffuse particles. Diffuse particles are used to simulate fluid effects
    /// such as foam, spray and bubbles.
    ///
    /// PxParticleAndDiffuseBuffer instance
    pub fn PxPhysics_createParticleAndDiffuseBuffer_mut(self_: *mut PxPhysics, maxParticles: u32, maxDiffuseParticles: u32, cudaContextManager: *mut PxCudaContextManager) -> *mut PxParticleAndDiffuseBuffer;

    /// Creates a new rigid body material with certain default properties.
    ///
    /// The new rigid body material.
    pub fn PxPhysics_createMaterial_mut(self_: *mut PxPhysics, staticFriction: f32, dynamicFriction: f32, restitution: f32) -> *mut PxMaterial;

    /// Return the number of rigid body materials that currently exist.
    ///
    /// Number of rigid body materials.
    pub fn PxPhysics_getNbMaterials(self_: *const PxPhysics) -> u32;

    /// Writes the array of rigid body material pointers to a user buffer.
    ///
    /// Returns the number of pointers written.
    ///
    /// The ordering of the materials in the array is not specified.
    ///
    /// The number of material pointers written to userBuffer, this should be less or equal to bufferSize.
    pub fn PxPhysics_getMaterials(self_: *const PxPhysics, userBuffer: *mut *mut PxMaterial, bufferSize: u32, startIndex: u32) -> u32;

    /// Creates a new surface deformable material with certain default properties.
    ///
    /// The new surface deformable material.
    pub fn PxPhysics_createDeformableSurfaceMaterial_mut(self_: *mut PxPhysics, youngs: f32, poissons: f32, dynamicFriction: f32, thickness: f32, bendingStiffness: f32, elasticityDamping: f32, bendingDamping: f32) -> *mut PxDeformableSurfaceMaterial;

    /// Return the number of deformable surface materials that currently exist.
    ///
    /// Number of deformable surface materials.
    pub fn PxPhysics_getNbDeformableSurfaceMaterials(self_: *const PxPhysics) -> u32;

    /// Writes the array of deformable surface material pointers to a user buffer.
    ///
    /// Returns the number of pointers written.
    ///
    /// The ordering of the materials in the array is not specified.
    ///
    /// The number of material pointers written to userBuffer, this should be less or equal to bufferSize.
    pub fn PxPhysics_getDeformableSurfaceMaterials(self_: *const PxPhysics, userBuffer: *mut *mut PxDeformableSurfaceMaterial, bufferSize: u32, startIndex: u32) -> u32;

    /// Creates a new deformable volume material with certain default properties.
    ///
    /// The new deformable volume material.
    pub fn PxPhysics_createDeformableVolumeMaterial_mut(self_: *mut PxPhysics, youngs: f32, poissons: f32, dynamicFriction: f32, elasticityDamping: f32) -> *mut PxDeformableVolumeMaterial;

    /// Return the number of deformable volume materials that currently exist.
    ///
    /// Number of materials.
    pub fn PxPhysics_getNbDeformableVolumeMaterials(self_: *const PxPhysics) -> u32;

    /// Writes the array of deformable volume material pointers to a user buffer.
    ///
    /// Returns the number of pointers written.
    ///
    /// The ordering of the materials in the array is not specified.
    ///
    /// The number of material pointers written to userBuffer, this should be less or equal to bufferSize.
    pub fn PxPhysics_getDeformableVolumeMaterials(self_: *const PxPhysics, userBuffer: *mut *mut PxDeformableVolumeMaterial, bufferSize: u32, startIndex: u32) -> u32;

    /// Creates a new PBD material with certain default properties.
    ///
    /// The new PBD material.
    pub fn PxPhysics_createPBDMaterial_mut(self_: *mut PxPhysics, friction: f32, damping: f32, adhesion: f32, viscosity: f32, vorticityConfinement: f32, surfaceTension: f32, cohesion: f32, lift: f32, drag: f32, cflCoefficient: f32, gravityScale: f32) -> *mut PxPBDMaterial;

    /// Return the number of PBD materials that currently exist.
    ///
    /// Number of PBD materials.
    pub fn PxPhysics_getNbPBDMaterials(self_: *const PxPhysics) -> u32;

    /// Writes the array of PBD material pointers to a user buffer.
    ///
    /// Returns the number of pointers written.
    ///
    /// The ordering of the materials in the array is not specified.
    ///
    /// The number of material pointers written to userBuffer, this should be less or equal to bufferSize.
    pub fn PxPhysics_getPBDMaterials(self_: *const PxPhysics, userBuffer: *mut *mut PxPBDMaterial, bufferSize: u32, startIndex: u32) -> u32;

    /// Register a deletion listener. Listeners will be called whenever an object is deleted.
    ///
    /// It is illegal to register or unregister a deletion listener while deletions are being processed.
    ///
    /// By default a registered listener will receive events from all objects. Set the restrictedObjectSet parameter to true on registration and use [`registerDeletionListenerObjects`] to restrict the received events to specific objects.
    ///
    /// The deletion events are only supported on core PhysX objects. In general, objects in extension modules do not provide this functionality, however, in the case of PxJoint objects, the underlying PxConstraint will send the events.
    pub fn PxPhysics_registerDeletionListener_mut(self_: *mut PxPhysics, observer: *mut PxDeletionListener, deletionEvents: *const PxDeletionEventFlags, restrictedObjectSet: bool);

    /// Unregister a deletion listener.
    ///
    /// It is illegal to register or unregister a deletion listener while deletions are being processed.
    pub fn PxPhysics_unregisterDeletionListener_mut(self_: *mut PxPhysics, observer: *mut PxDeletionListener);

    /// Register specific objects for deletion events.
    ///
    /// This method allows for a deletion listener to limit deletion events to specific objects only.
    ///
    /// It is illegal to register or unregister objects while deletions are being processed.
    ///
    /// The deletion listener has to be registered through [`registerDeletionListener`]() and configured to support restricted object sets prior to this method being used.
    pub fn PxPhysics_registerDeletionListenerObjects_mut(self_: *mut PxPhysics, observer: *mut PxDeletionListener, observables: *const *const PxBase, observableCount: u32);

    /// Unregister specific objects for deletion events.
    ///
    /// This method allows to clear previously registered objects for a deletion listener (see [`registerDeletionListenerObjects`]()).
    ///
    /// It is illegal to register or unregister objects while deletions are being processed.
    ///
    /// The deletion listener has to be registered through [`registerDeletionListener`]() and configured to support restricted object sets prior to this method being used.
    pub fn PxPhysics_unregisterDeletionListenerObjects_mut(self_: *mut PxPhysics, observer: *mut PxDeletionListener, observables: *const *const PxBase, observableCount: u32);

    /// Creates an instance of the physics SDK.
    ///
    /// Creates an instance of this class. May not be a class member to avoid name mangling.
    /// Pass the constant [`PX_PHYSICS_VERSION`] as the argument.
    /// There may be only one instance of this class per process. Calling this method after an instance
    /// has been created already will result in an error message and NULL will be returned.
    ///
    /// PxPhysics instance on success, NULL if operation failed
    pub fn phys_PxCreatePhysics(version: u32, foundation: *mut PxFoundation, scale: *const PxTolerancesScale, trackOutstandingAllocations: bool, pvd: *mut PxPvd, omniPvd: *mut PxOmniPvd) -> *mut PxPhysics;

    pub fn phys_PxGetPhysics() -> *mut PxPhysics;

    pub fn PxActorShape_new() -> PxActorShape;

    pub fn PxActorShape_new_1(a: *mut PxRigidActor, s: *mut PxShape) -> PxActorShape;

    /// constructor sets to default
    pub fn PxQueryCache_new() -> PxQueryCache;

    /// constructor to set properties
    pub fn PxQueryCache_new_1(s: *mut PxShape, findex: u32) -> PxQueryCache;

    /// default constructor
    pub fn PxQueryFilterData_new() -> PxQueryFilterData;

    /// constructor to set both filter data and filter flags
    pub fn PxQueryFilterData_new_1(fd: *const PxFilterData, f: PxQueryFlags) -> PxQueryFilterData;

    /// constructor to set filter flags only
    pub fn PxQueryFilterData_new_2(f: PxQueryFlags) -> PxQueryFilterData;

    /// This filter callback is executed before the exact intersection test if PxQueryFlag::ePREFILTER flag was set.
    ///
    /// the updated type for this hit  (see [`PxQueryHitType`])
    pub fn PxQueryFilterCallback_preFilter_mut(self_: *mut PxQueryFilterCallback, filterData: *const PxFilterData, shape: *const PxShape, actor: *const PxRigidActor, queryFlags: *mut PxHitFlags) -> PxQueryHitType;

    /// This filter callback is executed if the exact intersection test returned true and PxQueryFlag::ePOSTFILTER flag was set.
    ///
    /// the updated hit type for this hit  (see [`PxQueryHitType`])
    pub fn PxQueryFilterCallback_postFilter_mut(self_: *mut PxQueryFilterCallback, filterData: *const PxFilterData, hit: *const PxQueryHit, shape: *const PxShape, actor: *const PxRigidActor) -> PxQueryHitType;

    /// virtual destructor
    pub fn PxQueryFilterCallback_delete(self_: *mut PxQueryFilterCallback);

    /// Moves kinematically controlled dynamic actors through the game world.
    ///
    /// You set a dynamic actor to be kinematic using the PxRigidBodyFlag::eKINEMATIC flag
    /// with setRigidBodyFlag().
    ///
    /// The move command will result in a velocity that will move the body into
    /// the desired pose. After the move is carried out during a single time step,
    /// the velocity is returned to zero. Thus, you must continuously call
    /// this in every time step for kinematic actors so that they move along a path.
    ///
    /// This function simply stores the move destination until the next simulation
    /// step is processed, so consecutive calls will simply overwrite the stored target variable.
    ///
    /// The motion is always fully carried out.
    ///
    /// It is invalid to use this method if the actor has not been added to a scene already or if PxActorFlag::eDISABLE_SIMULATION is set.
    ///
    /// Sleeping:
    /// This call wakes the actor if it is sleeping and will set the wake counter to [`PxSceneDesc::wakeCounterResetValue`].
    pub fn PxRigidDynamic_setKinematicTarget_mut(self_: *mut PxRigidDynamic, destination: *const PxTransform);

    /// Get target pose of a kinematically controlled dynamic actor.
    ///
    /// True if the actor is a kinematically controlled dynamic and the target has been set, else False.
    pub fn PxRigidDynamic_getKinematicTarget(self_: *const PxRigidDynamic, target: *mut PxTransform) -> bool;

    /// Returns true if this body is sleeping.
    ///
    /// When an actor does not move for a period of time, it is no longer simulated in order to save time. This state
    /// is called sleeping. However, because the object automatically wakes up when it is either touched by an awake object,
    /// or one of its properties is changed by the user, the entire sleep mechanism should be transparent to the user.
    ///
    /// In general, a dynamic rigid actor is guaranteed to be awake if at least one of the following holds:
    ///
    /// The wake counter is positive (see [`setWakeCounter`]()).
    ///
    /// The linear or angular velocity is non-zero.
    ///
    /// A non-zero force or torque has been applied.
    ///
    /// If a dynamic rigid actor is sleeping, the following state is guaranteed:
    ///
    /// The wake counter is zero.
    ///
    /// The linear and angular velocity is zero.
    ///
    /// There is no force update pending.
    ///
    /// When an actor gets inserted into a scene, it will be considered asleep if all the points above hold, else it will be treated as awake.
    ///
    /// If an actor is asleep after the call to PxScene::fetchResults() returns, it is guaranteed that the pose of the actor
    /// was not changed. You can use this information to avoid updating the transforms of associated objects.
    ///
    /// A kinematic actor is asleep unless a target pose has been set (in which case it will stay awake until two consecutive
    /// simulation steps without a target pose being set have passed). The wake counter will get set to zero or to the reset value
    /// [`PxSceneDesc::wakeCounterResetValue`] in the case where a target pose has been set to be consistent with the definitions above.
    ///
    /// It is invalid to use this method if the actor has not been added to a scene already.
    ///
    /// It is not allowed to use this method while the simulation is running.
    ///
    /// True if the actor is sleeping.
    pub fn PxRigidDynamic_isSleeping(self_: *const PxRigidDynamic) -> bool;

    /// Sets the mass-normalized kinetic energy threshold below which an actor may go to sleep.
    ///
    /// Actors whose kinetic energy divided by their mass is below this threshold will be candidates for sleeping.
    ///
    /// Default:
    /// 5e-5f * PxTolerancesScale::speed * PxTolerancesScale::speed
    pub fn PxRigidDynamic_setSleepThreshold_mut(self_: *mut PxRigidDynamic, threshold: f32);

    /// Returns the mass-normalized kinetic energy below which an actor may go to sleep.
    ///
    /// The energy threshold for sleeping.
    pub fn PxRigidDynamic_getSleepThreshold(self_: *const PxRigidDynamic) -> f32;

    /// Sets the mass-normalized kinetic energy threshold below which an actor may participate in stabilization.
    ///
    /// Actors whose kinetic energy divided by their mass is above this threshold will not participate in stabilization.
    ///
    /// This value has no effect if PxSceneFlag::eENABLE_STABILIZATION was not enabled on the PxSceneDesc.
    ///
    /// Default:
    /// 1e-5f * PxTolerancesScale::speed * PxTolerancesScale::speed
    pub fn PxRigidDynamic_setStabilizationThreshold_mut(self_: *mut PxRigidDynamic, threshold: f32);

    /// Returns the mass-normalized kinetic energy below which an actor may participate in stabilization.
    ///
    /// Actors whose kinetic energy divided by their mass is above this threshold will not participate in stabilization.
    ///
    /// The energy threshold for participating in stabilization.
    pub fn PxRigidDynamic_getStabilizationThreshold(self_: *const PxRigidDynamic) -> f32;

    /// Sets the wake counter for the actor.
    ///
    /// The wake counter value determines the minimum amount of time until the body can be put to sleep. Please note
    /// that a body will not be put to sleep if the energy is above the specified threshold (see [`setSleepThreshold`]())
    /// or if other awake bodies are touching it.
    ///
    /// Passing in a positive value will wake the actor up automatically.
    ///
    /// It is invalid to use this method for kinematic actors since the wake counter for kinematics is defined
    /// based on whether a target pose has been set (see the comment in [`isSleeping`]()).
    ///
    /// It is invalid to use this method if PxActorFlag::eDISABLE_SIMULATION is set.
    ///
    /// Default:
    /// 0.4 (which corresponds to 20 frames for a time step of 0.02)
    pub fn PxRigidDynamic_setWakeCounter_mut(self_: *mut PxRigidDynamic, wakeCounterValue: f32);

    /// Returns the wake counter of the actor.
    ///
    /// It is not allowed to use this method while the simulation is running.
    ///
    /// The wake counter of the actor.
    pub fn PxRigidDynamic_getWakeCounter(self_: *const PxRigidDynamic) -> f32;

    /// Wakes up the actor if it is sleeping.
    ///
    /// The actor will get woken up and might cause other touching actors to wake up as well during the next simulation step.
    ///
    /// This will set the wake counter of the actor to the value specified in [`PxSceneDesc::wakeCounterResetValue`].
    ///
    /// It is invalid to use this method if the actor has not been added to a scene already or if PxActorFlag::eDISABLE_SIMULATION is set.
    ///
    /// It is invalid to use this method for kinematic actors since the sleep state for kinematics is defined
    /// based on whether a target pose has been set (see the comment in [`isSleeping`]()).
    pub fn PxRigidDynamic_wakeUp_mut(self_: *mut PxRigidDynamic);

    /// Forces the actor to sleep.
    ///
    /// The actor will stay asleep during the next simulation step if not touched by another non-sleeping actor.
    ///
    /// Any applied force will be cleared and the velocity and the wake counter of the actor will be set to 0.
    ///
    /// It is invalid to use this method if the actor has not been added to a scene already or if PxActorFlag::eDISABLE_SIMULATION is set.
    ///
    /// It is invalid to use this method for kinematic actors since the sleep state for kinematics is defined
    /// based on whether a target pose has been set (see the comment in [`isSleeping`]()).
    pub fn PxRigidDynamic_putToSleep_mut(self_: *mut PxRigidDynamic);

    /// Reads the PxRigidDynamic lock flags.
    ///
    /// See the list of flags [`PxRigidDynamicLockFlag`]
    ///
    /// The values of the PxRigidDynamic lock flags.
    pub fn PxRigidDynamic_getRigidDynamicLockFlags(self_: *const PxRigidDynamic) -> PxRigidDynamicLockFlags;

    /// Raises or clears a particular PxRigidDynamic lock flag.
    ///
    /// See the list of flags [`PxRigidDynamicLockFlag`]
    ///
    /// Default:
    /// no flags are set
    pub fn PxRigidDynamic_setRigidDynamicLockFlag_mut(self_: *mut PxRigidDynamic, flag: PxRigidDynamicLockFlag, value: bool);

    /// Set all PxRigidDynamic lock flags.
    pub fn PxRigidDynamic_setRigidDynamicLockFlags_mut(self_: *mut PxRigidDynamic, flags: PxRigidDynamicLockFlags);

    /// Retrieves the actor's center-of-mass linear velocity.
    ///
    /// It is not allowed to use this method while the simulation is running (except during PxScene::collide(),
    /// in PxContactModifyCallback or in contact report callbacks).
    ///
    /// The linear velocity is reported with respect to the actor's center of mass and not the actor frame origin.
    ///
    /// This method should not be used after the direct GPU API has been enabled and initialized. See [`PxDirectGPUAPI`] for the details.
    ///
    /// The actor's center-of-mass linear velocity.
    pub fn PxRigidDynamic_getLinearVelocity(self_: *const PxRigidDynamic) -> PxVec3;

    /// Sets the actor's center-of-mass linear velocity.
    ///
    /// Note that if you continuously set the velocity of an actor yourself,
    /// forces such as gravity or friction will not be able to manifest themselves, because forces directly
    /// influence only the velocity/momentum of an actor.
    ///
    /// Default:
    /// (0.0, 0.0, 0.0)
    ///
    /// Sleeping:
    /// This call wakes the actor if it is sleeping, and the autowake parameter is true (default) or the
    /// new velocity is non-zero.
    ///
    /// It is invalid to use this method if PxActorFlag::eDISABLE_SIMULATION is set.
    ///
    /// This method should not be used after the direct GPU API has been enabled and initialized. See [`PxDirectGPUAPI`] for the details.
    ///
    /// The linear velocity is applied with respect to the actor's center of mass and not the actor frame origin.
    pub fn PxRigidDynamic_setLinearVelocity_mut(self_: *mut PxRigidDynamic, linVel: *const PxVec3, autowake: bool);

    /// Retrieves the angular velocity of the actor.
    ///
    /// It is not allowed to use this method while the simulation is running (except during PxScene::collide(),
    /// in PxContactModifyCallback or in contact report callbacks).
    ///
    /// This method should not be used after the direct GPU API has been enabled and initialized. See [`PxDirectGPUAPI`] for the details.
    ///
    /// The angular velocity of the actor.
    pub fn PxRigidDynamic_getAngularVelocity(self_: *const PxRigidDynamic) -> PxVec3;

    /// Sets the angular velocity of the actor.
    ///
    /// Note that if you continuously set the angular velocity of an actor yourself,
    /// forces such as friction will not be able to rotate the actor, because forces directly influence only the velocity/momentum.
    ///
    /// Default:
    /// (0.0, 0.0, 0.0)
    ///
    /// Sleeping:
    /// This call wakes the actor if it is sleeping, and the autowake parameter is true (default) or the
    /// new velocity is non-zero.
    ///
    /// It is invalid to use this method if PxActorFlag::eDISABLE_SIMULATION is set.
    ///
    /// This method should not be used after the direct GPU API has been enabled and initialized. See [`PxDirectGPUAPI`] for the details.
    pub fn PxRigidDynamic_setAngularVelocity_mut(self_: *mut PxRigidDynamic, angVel: *const PxVec3, autowake: bool);

    /// Sets the solver iteration counts for the body.
    ///
    /// The solver iteration count determines how accurately joints and contacts are resolved.
    /// If you are having trouble with jointed bodies oscillating and behaving erratically, then
    /// setting a higher position iteration count may improve their stability.
    ///
    /// If intersecting bodies are being depenetrated too violently, increase the number of velocity
    /// iterations. More velocity iterations will drive the relative exit velocity of the intersecting
    /// objects closer to the correct value given the restitution.
    ///
    /// Default:
    /// 4 position iterations, 1 velocity iteration
    pub fn PxRigidDynamic_setSolverIterationCounts_mut(self_: *mut PxRigidDynamic, minPositionIters: u32, minVelocityIters: u32);

    /// Retrieves the solver iteration counts.
    pub fn PxRigidDynamic_getSolverIterationCounts(self_: *const PxRigidDynamic, minPositionIters: *mut u32, minVelocityIters: *mut u32);

    /// Retrieves the force threshold for contact reports.
    ///
    /// The contact report threshold is a force threshold. If the force between
    /// two actors exceeds this threshold for either of the two actors, a contact report
    /// will be generated according to the contact report threshold flags provided by
    /// the filter shader/callback.
    /// See [`PxPairFlag`].
    ///
    /// The threshold used for a collision between a dynamic actor and the static environment is
    /// the threshold of the dynamic actor, and all contacts with static actors are summed to find
    /// the total normal force.
    ///
    /// Default:
    /// PX_MAX_F32
    ///
    /// Force threshold for contact reports.
    pub fn PxRigidDynamic_getContactReportThreshold(self_: *const PxRigidDynamic) -> f32;

    /// Sets the force threshold for contact reports.
    ///
    /// See [`getContactReportThreshold`]().
    pub fn PxRigidDynamic_setContactReportThreshold_mut(self_: *mut PxRigidDynamic, threshold: f32);

    /// Returns the GPU rigid dynamic index.
    ///
    /// This function only returns valid results if GPU dynamics is enabled.
    ///
    /// The GPU index, or 0xFFFFFFFF if the actor is not inserted into a PxScene.
    pub fn PxRigidDynamic_getGPUIndex(self_: *const PxRigidDynamic) -> u32;

    pub fn PxRigidDynamic_getConcreteTypeName(self_: *const PxRigidDynamic) -> *const std::ffi::c_char;

    pub fn PxRigidStatic_getConcreteTypeName(self_: *const PxRigidStatic) -> *const std::ffi::c_char;

    pub fn PxArticulationGPUAPIMaxCounts_new() -> PxArticulationGPUAPIMaxCounts;

    /// constructor sets to default.
    pub fn PxSceneQueryDesc_new() -> PxSceneQueryDesc;

    /// (re)sets the structure to the default.
    pub fn PxSceneQueryDesc_setToDefault_mut(self_: *mut PxSceneQueryDesc);

    /// Returns true if the descriptor is valid.
    ///
    /// true if the current settings are valid.
    pub fn PxSceneQueryDesc_isValid(self_: *const PxSceneQueryDesc) -> bool;

    /// Sets the rebuild rate of the dynamic tree pruning structures.
    pub fn PxSceneQuerySystemBase_setDynamicTreeRebuildRateHint_mut(self_: *mut PxSceneQuerySystemBase, dynamicTreeRebuildRateHint: u32);

    /// Retrieves the rebuild rate of the dynamic tree pruning structures.
    ///
    /// The rebuild rate of the dynamic tree pruning structures.
    pub fn PxSceneQuerySystemBase_getDynamicTreeRebuildRateHint(self_: *const PxSceneQuerySystemBase) -> u32;

    /// Forces dynamic trees to be immediately rebuilt.
    ///
    /// PxScene will call this function with the PX_SCENE_PRUNER_STATIC or PX_SCENE_PRUNER_DYNAMIC value.
    pub fn PxSceneQuerySystemBase_forceRebuildDynamicTree_mut(self_: *mut PxSceneQuerySystemBase, prunerIndex: u32);

    /// Sets scene query update mode
    pub fn PxSceneQuerySystemBase_setUpdateMode_mut(self_: *mut PxSceneQuerySystemBase, updateMode: PxSceneQueryUpdateMode);

    /// Gets scene query update mode
    ///
    /// Current scene query update mode.
    pub fn PxSceneQuerySystemBase_getUpdateMode(self_: *const PxSceneQuerySystemBase) -> PxSceneQueryUpdateMode;

    /// Retrieves the system's internal scene query timestamp, increased each time a change to the
    /// static scene query structure is performed.
    ///
    /// scene query static timestamp
    pub fn PxSceneQuerySystemBase_getStaticTimestamp(self_: *const PxSceneQuerySystemBase) -> u32;

    /// Flushes any changes to the scene query representation.
    ///
    /// This method updates the state of the scene query representation to match changes in the scene state.
    ///
    /// By default, these changes are buffered until the next query is submitted. Calling this function will not change
    /// the results from scene queries, but can be used to ensure that a query will not perform update work in the course of
    /// its execution.
    ///
    /// A thread performing updates will hold a write lock on the query structure, and thus stall other querying threads. In multithread
    /// scenarios it can be useful to explicitly schedule the period where this lock may be held for a significant period, so that
    /// subsequent queries issued from multiple threads will not block.
    pub fn PxSceneQuerySystemBase_flushUpdates_mut(self_: *mut PxSceneQuerySystemBase);

    /// Performs a raycast against objects in the scene, returns results in a PxRaycastBuffer object
    /// or via a custom user callback implementation inheriting from PxRaycastCallback.
    ///
    /// Touching hits are not ordered.
    ///
    /// Shooting a ray from within an object leads to different results depending on the shape type. Please check the details in user guide article SceneQuery. User can ignore such objects by employing one of the provided filter mechanisms.
    ///
    /// True if any touching or blocking hits were found or any hit was found in case PxQueryFlag::eANY_HIT was specified.
    pub fn PxSceneQuerySystemBase_raycast(self_: *const PxSceneQuerySystemBase, origin: *const PxVec3, unitDir: *const PxVec3, distance: f32, hitCall: *mut PxRaycastCallback, hitFlags: PxHitFlags, filterData: *const PxQueryFilterData, filterCall: *mut PxQueryFilterCallback, cache: *const PxQueryCache, queryFlags: PxGeometryQueryFlags) -> bool;

    /// Performs a sweep test against objects in the scene, returns results in a PxSweepBuffer object
    /// or via a custom user callback implementation inheriting from PxSweepCallback.
    ///
    /// Touching hits are not ordered.
    ///
    /// If a shape from the scene is already overlapping with the query shape in its starting position,
    /// the hit is returned unless eASSUME_NO_INITIAL_OVERLAP was specified.
    ///
    /// True if any touching or blocking hits were found or any hit was found in case PxQueryFlag::eANY_HIT was specified.
    pub fn PxSceneQuerySystemBase_sweep(self_: *const PxSceneQuerySystemBase, geometry: *const PxGeometry, pose: *const PxTransform, unitDir: *const PxVec3, distance: f32, hitCall: *mut PxSweepCallback, hitFlags: PxHitFlags, filterData: *const PxQueryFilterData, filterCall: *mut PxQueryFilterCallback, cache: *const PxQueryCache, inflation: f32, queryFlags: PxGeometryQueryFlags) -> bool;

    /// Performs an overlap test of a given geometry against objects in the scene, returns results in a PxOverlapBuffer object
    /// or via a custom user callback implementation inheriting from PxOverlapCallback.
    ///
    /// Filtering: returning eBLOCK from user filter for overlap queries will cause a warning (see [`PxQueryHitType`]).
    ///
    /// True if any touching or blocking hits were found or any hit was found in case PxQueryFlag::eANY_HIT was specified.
    ///
    /// eBLOCK should not be returned from user filters for overlap(). Doing so will result in undefined behavior, and a warning will be issued.
    ///
    /// If the PxQueryFlag::eNO_BLOCK flag is set, the eBLOCK will instead be automatically converted to an eTOUCH and the warning suppressed.
    pub fn PxSceneQuerySystemBase_overlap(self_: *const PxSceneQuerySystemBase, geometry: *const PxGeometry, pose: *const PxTransform, hitCall: *mut PxOverlapCallback, filterData: *const PxQueryFilterData, filterCall: *mut PxQueryFilterCallback, cache: *const PxQueryCache, queryFlags: PxGeometryQueryFlags) -> bool;

    /// Sets scene query update mode
    pub fn PxSceneSQSystem_setSceneQueryUpdateMode_mut(self_: *mut PxSceneSQSystem, updateMode: PxSceneQueryUpdateMode);

    /// Gets scene query update mode
    ///
    /// Current scene query update mode.
    pub fn PxSceneSQSystem_getSceneQueryUpdateMode(self_: *const PxSceneSQSystem) -> PxSceneQueryUpdateMode;

    /// Retrieves the scene's internal scene query timestamp, increased each time a change to the
    /// static scene query structure is performed.
    ///
    /// scene query static timestamp
    pub fn PxSceneSQSystem_getSceneQueryStaticTimestamp(self_: *const PxSceneSQSystem) -> u32;

    /// Flushes any changes to the scene query representation.
    pub fn PxSceneSQSystem_flushQueryUpdates_mut(self_: *mut PxSceneSQSystem);

    /// Forces dynamic trees to be immediately rebuilt.
    pub fn PxSceneSQSystem_forceDynamicTreeRebuild_mut(self_: *mut PxSceneSQSystem, rebuildStaticStructure: bool, rebuildDynamicStructure: bool);

    /// Return the value of PxSceneQueryDesc::staticStructure that was set when creating the scene with PxPhysics::createScene
    pub fn PxSceneSQSystem_getStaticStructure(self_: *const PxSceneSQSystem) -> PxPruningStructureType;

    /// Return the value of PxSceneQueryDesc::dynamicStructure that was set when creating the scene with PxPhysics::createScene
    pub fn PxSceneSQSystem_getDynamicStructure(self_: *const PxSceneSQSystem) -> PxPruningStructureType;

    /// Executes scene queries update tasks.
    ///
    /// This function will refit dirty shapes within the pruner and will execute a task to build a new AABB tree, which is
    /// build on a different thread. The new AABB tree is built based on the dynamic tree rebuild hint rate. Once
    /// the new tree is ready it will be commited in next fetchQueries call, which must be called after.
    ///
    /// This function is equivalent to the following PxSceneQuerySystem calls:
    /// Synchronous calls:
    /// - PxSceneQuerySystemBase::flushUpdates()
    /// - handle0 = PxSceneQuerySystem::prepareSceneQueryBuildStep(PX_SCENE_PRUNER_STATIC)
    /// - handle1 = PxSceneQuerySystem::prepareSceneQueryBuildStep(PX_SCENE_PRUNER_DYNAMIC)
    /// Asynchronous calls:
    /// - PxSceneQuerySystem::sceneQueryBuildStep(handle0);
    /// - PxSceneQuerySystem::sceneQueryBuildStep(handle1);
    ///
    /// This function is part of the PxSceneSQSystem interface because it uses the PxScene task system under the hood. But
    /// it calls PxSceneQuerySystem functions, which are independent from this system and could be called in a similar
    /// fashion by a separate, possibly user-defined task manager.
    ///
    /// If PxSceneQueryUpdateMode::eBUILD_DISABLED_COMMIT_DISABLED is used, it is required to update the scene queries
    /// using this function.
    pub fn PxSceneSQSystem_sceneQueriesUpdate_mut(self_: *mut PxSceneSQSystem, completionTask: *mut PxBaseTask, controlSimulation: bool);

    /// This checks to see if the scene queries update has completed.
    ///
    /// This does not cause the data available for reading to be updated with the results of the scene queries update, it is simply a status check.
    /// The bool will allow it to either return immediately or block waiting for the condition to be met so that it can return true
    ///
    /// True if the results are available.
    pub fn PxSceneSQSystem_checkQueries_mut(self_: *mut PxSceneSQSystem, block: bool) -> bool;

    /// This method must be called after sceneQueriesUpdate. It will wait for the scene queries update to finish. If the user makes an illegal scene queries update call,
    /// the SDK will issue an error	message.
    ///
    /// If a new AABB tree build finished, then during fetchQueries the current tree within the pruning structure is swapped with the new tree.
    pub fn PxSceneSQSystem_fetchQueries_mut(self_: *mut PxSceneSQSystem, block: bool) -> bool;

    /// Decrements the reference count of the object and releases it if the new reference count is zero.
    pub fn PxSceneQuerySystem_release_mut(self_: *mut PxSceneQuerySystem);

    /// Acquires a counted reference to this object.
    ///
    /// This method increases the reference count of the object by 1. Decrement the reference count by calling release()
    pub fn PxSceneQuerySystem_acquireReference_mut(self_: *mut PxSceneQuerySystem);

    /// Preallocates internal arrays to minimize the amount of reallocations.
    ///
    /// The system does not prevent more allocations than given numbers. It is legal to not call this function at all,
    /// or to add more shapes to the system than the preallocated amounts.
    pub fn PxSceneQuerySystem_preallocate_mut(self_: *mut PxSceneQuerySystem, prunerIndex: u32, nbShapes: u32);

    /// Frees internal memory that may not be in-use anymore.
    ///
    /// This is an entry point for reclaiming transient memory allocated at some point by the SQ system,
    /// but which wasn't been immediately freed for performance reason. Calling this function might free
    /// some memory, but it might also produce a new set of allocations in the next frame.
    pub fn PxSceneQuerySystem_flushMemory_mut(self_: *mut PxSceneQuerySystem);

    /// Adds a shape to the SQ system.
    ///
    /// The same function is used to add either a regular shape, or a SQ compound shape.
    pub fn PxSceneQuerySystem_addSQShape_mut(self_: *mut PxSceneQuerySystem, actor: *const PxRigidActor, shape: *const PxShape, bounds: *const PxBounds3, transform: *const PxTransform, compoundHandle: *const u32, hasPruningStructure: bool);

    /// Removes a shape from the SQ system.
    ///
    /// The same function is used to remove either a regular shape, or a SQ compound shape.
    pub fn PxSceneQuerySystem_removeSQShape_mut(self_: *mut PxSceneQuerySystem, actor: *const PxRigidActor, shape: *const PxShape);

    /// Updates a shape in the SQ system.
    ///
    /// The same function is used to update either a regular shape, or a SQ compound shape.
    ///
    /// The transforms are eager-evaluated, but the bounds are lazy-evaluated. This means that
    /// the updated transform has to be passed to the update function, while the bounds are automatically
    /// recomputed by the system whenever needed.
    pub fn PxSceneQuerySystem_updateSQShape_mut(self_: *mut PxSceneQuerySystem, actor: *const PxRigidActor, shape: *const PxShape, transform: *const PxTransform);

    /// Adds a compound to the SQ system.
    ///
    /// SQ compound handle
    pub fn PxSceneQuerySystem_addSQCompound_mut(self_: *mut PxSceneQuerySystem, actor: *const PxRigidActor, shapes: *mut *const PxShape, bvh: *const PxBVH, transforms: *const PxTransform) -> u32;

    /// Removes a compound from the SQ system.
    pub fn PxSceneQuerySystem_removeSQCompound_mut(self_: *mut PxSceneQuerySystem, compoundHandle: u32);

    /// Updates a compound in the SQ system.
    ///
    /// The compound structures are immediately updated when the call occurs.
    pub fn PxSceneQuerySystem_updateSQCompound_mut(self_: *mut PxSceneQuerySystem, compoundHandle: u32, compoundTransform: *const PxTransform);

    /// Shift the data structures' origin by the specified vector.
    ///
    /// Please refer to the notes of the similar function in PxScene.
    pub fn PxSceneQuerySystem_shiftOrigin_mut(self_: *mut PxSceneQuerySystem, shift: *const PxVec3);

    /// Visualizes the system's internal data-structures, for debugging purposes.
    pub fn PxSceneQuerySystem_visualize(self_: *const PxSceneQuerySystem, prunerIndex: u32, out: *mut PxRenderOutput);

    /// Merges a pruning structure with the SQ system's internal pruners.
    pub fn PxSceneQuerySystem_merge_mut(self_: *mut PxSceneQuerySystem, pruningStructure: *const PxPruningStructure);

    /// Shape to SQ-pruner-handle mapping function.
    ///
    /// This function finds and returns the SQ pruner handle associated with a given (actor/shape) couple
    /// that was previously added to the system. This is needed for the sync function.
    ///
    /// Associated SQ pruner handle.
    pub fn PxSceneQuerySystem_getHandle(self_: *const PxSceneQuerySystem, actor: *const PxRigidActor, shape: *const PxShape, prunerIndex: *mut u32) -> u32;

    /// Synchronizes the scene-query system with another system that references the same objects.
    ///
    /// This function is used when the scene-query objects also exist in another system that can also update them. For example the scene-query objects
    /// (used for raycast, overlap or sweep queries) might be driven by equivalent objects in an external rigid-body simulation engine. In this case
    /// the rigid-body simulation engine computes the new poses and transforms, and passes them to the scene-query system using this function. It is
    /// more efficient than calling updateSQShape on each object individually, since updateSQShape would end up recomputing the bounds already available
    /// in the rigid-body engine.
    pub fn PxSceneQuerySystem_sync_mut(self_: *mut PxSceneQuerySystem, prunerIndex: u32, handles: *const u32, indices: *const u32, bounds: *const PxBounds3, transforms: *const PxTransformPadded, count: u32, ignoredIndices: *const PxBitMap);

    /// Finalizes updates made to the SQ system.
    ///
    /// This function should be called after updates have been made to the SQ system, to fully reflect the changes
    /// inside the internal pruners. In particular it should be called:
    /// - after calls to updateSQShape
    /// - after calls to sync
    ///
    /// This function:
    /// - recomputes bounds of manually updated shapes (i.e. either regular or SQ compound shapes modified by updateSQShape)
    /// - updates dynamic pruners (refit operations)
    /// - incrementally rebuilds AABB-trees
    ///
    /// The amount of work performed in this function depends on PxSceneQueryUpdateMode.
    pub fn PxSceneQuerySystem_finalizeUpdates_mut(self_: *mut PxSceneQuerySystem);

    /// Prepares asynchronous build step.
    ///
    /// This is directly called (synchronously) by PxSceneSQSystem::sceneQueriesUpdate(). See the comments there.
    ///
    /// This function is called to let the system execute any necessary synchronous operation before the
    /// asynchronous sceneQueryBuildStep() function is called.
    ///
    /// If there is any work to do for the specific pruner, the function returns a pruner-specific handle that
    /// will be passed to the corresponding, asynchronous sceneQueryBuildStep function.
    ///
    /// A pruner-specific handle that will be sent to sceneQueryBuildStep if there is any work to do, i.e. to execute the corresponding sceneQueryBuildStep() call.
    ///
    /// Null if there is no work to do, otherwise a pruner-specific handle.
    pub fn PxSceneQuerySystem_prepareSceneQueryBuildStep_mut(self_: *mut PxSceneQuerySystem, prunerIndex: u32) -> *mut std::ffi::c_void;

    /// Executes asynchronous build step.
    ///
    /// This is directly called (asynchronously) by PxSceneSQSystem::sceneQueriesUpdate(). See the comments there.
    ///
    /// This function incrementally builds the internal trees/pruners. It is called asynchronously, i.e. this can be
    /// called from different threads for building multiple trees at the same time.
    pub fn PxSceneQuerySystem_sceneQueryBuildStep_mut(self_: *mut PxSceneQuerySystem, handle: *mut std::ffi::c_void);

    pub fn PxGpuBroadPhaseDesc_new() -> PxGpuBroadPhaseDesc;

    pub fn PxGpuBroadPhaseDesc_isValid(self_: *const PxGpuBroadPhaseDesc) -> bool;

    pub fn PxBroadPhaseDesc_new(type_: PxBroadPhaseType) -> PxBroadPhaseDesc;

    pub fn PxBroadPhaseDesc_isValid(self_: *const PxBroadPhaseDesc) -> bool;

    /// Retrieves the filter group for static objects.
    ///
    /// Mark static objects with this group when adding them to the broadphase.
    /// Overlaps between static objects will not be detected. All static objects
    /// should have the same group.
    ///
    /// Filter group for static objects.
    pub fn phys_PxGetBroadPhaseStaticFilterGroup() -> u32;

    /// Retrieves a filter group for dynamic objects.
    ///
    /// Mark dynamic objects with this group when adding them to the broadphase.
    /// Each dynamic object must have an ID, and overlaps between dynamic objects that have
    /// the same ID will not be detected. This is useful to dismiss overlaps between shapes
    /// of the same (compound) actor directly within the broadphase.
    ///
    /// Filter group for the object.
    pub fn phys_PxGetBroadPhaseDynamicFilterGroup(id: u32) -> u32;

    /// Retrieves a filter group for kinematic objects.
    ///
    /// Mark kinematic objects with this group when adding them to the broadphase.
    /// Each kinematic object must have an ID, and overlaps between kinematic objects that have
    /// the same ID will not be detected.
    ///
    /// Filter group for the object.
    pub fn phys_PxGetBroadPhaseKinematicFilterGroup(id: u32) -> u32;

    pub fn PxBroadPhaseUpdateData_new(created: *const u32, nbCreated: u32, updated: *const u32, nbUpdated: u32, removed: *const u32, nbRemoved: u32, bounds: *const PxBounds3, groups: *const u32, distances: *const f32, capacity: u32) -> PxBroadPhaseUpdateData;

    pub fn PxBroadPhaseResults_new() -> PxBroadPhaseResults;

    /// Returns number of regions currently registered in the broad-phase.
    ///
    /// Number of regions
    pub fn PxBroadPhaseRegions_getNbRegions(self_: *const PxBroadPhaseRegions) -> u32;

    /// Gets broad-phase regions.
    ///
    /// Number of written out regions.
    pub fn PxBroadPhaseRegions_getRegions(self_: *const PxBroadPhaseRegions, userBuffer: *mut PxBroadPhaseRegionInfo, bufferSize: u32, startIndex: u32) -> u32;

    /// Adds a new broad-phase region.
    ///
    /// The total number of regions is limited to PxBroadPhaseCaps::mMaxNbRegions. If that number is exceeded, the call is ignored.
    ///
    /// The newly added region will be automatically populated with already existing objects that touch it, if the
    /// 'populateRegion' parameter is set to true. Otherwise the newly added region will be empty, and it will only be
    /// populated with objects when those objects are added to the simulation, or updated if they already exist.
    ///
    /// Using 'populateRegion=true' has a cost, so it is best to avoid it if possible. In particular it is more efficient
    /// to create the empty regions first (with populateRegion=false) and then add the objects afterwards (rather than
    /// the opposite).
    ///
    /// Objects automatically move from one region to another during their lifetime. The system keeps tracks of what
    /// regions a given object is in. It is legal for an object to be in an arbitrary number of regions. However if an
    /// object leaves all regions, or is created outside of all regions, several things happen:
    /// - collisions get disabled for this object
    /// - the object appears in the getOutOfBoundsObjects() array
    ///
    /// If an out-of-bounds object, whose collisions are disabled, re-enters a valid broadphase region, then collisions
    /// are re-enabled for that object.
    ///
    /// Handle for newly created region, or 0xffffffff in case of failure.
    pub fn PxBroadPhaseRegions_addRegion_mut(self_: *mut PxBroadPhaseRegions, region: *const PxBroadPhaseRegion, populateRegion: bool, bounds: *const PxBounds3, distances: *const f32) -> u32;

    /// Removes a broad-phase region.
    ///
    /// If the region still contains objects, and if those objects do not overlap any region any more, they are not
    /// automatically removed from the simulation. Instead, the PxBroadPhaseCallback::onObjectOutOfBounds notification
    /// is used for each object. Users are responsible for removing the objects from the simulation if this is the
    /// desired behavior.
    ///
    /// If the handle is invalid, or if a valid handle is removed twice, an error message is sent to the error stream.
    ///
    /// True if success
    pub fn PxBroadPhaseRegions_removeRegion_mut(self_: *mut PxBroadPhaseRegions, handle: u32) -> bool;

    pub fn PxBroadPhaseRegions_getNbOutOfBoundsObjects(self_: *const PxBroadPhaseRegions) -> u32;

    pub fn PxBroadPhaseRegions_getOutOfBoundsObjects(self_: *const PxBroadPhaseRegions) -> *const u32;

    pub fn PxBroadPhase_release_mut(self_: *mut PxBroadPhase);

    /// Gets the broadphase type.
    ///
    /// Broadphase type.
    pub fn PxBroadPhase_getType(self_: *const PxBroadPhase) -> PxBroadPhaseType;

    /// Gets broad-phase caps.
    pub fn PxBroadPhase_getCaps(self_: *const PxBroadPhase, caps: *mut PxBroadPhaseCaps);

    /// Retrieves the regions API if applicable.
    ///
    /// For broadphases that do not use explicit user-defined regions, this call returns NULL.
    ///
    /// Region API, or NULL.
    pub fn PxBroadPhase_getRegions_mut(self_: *mut PxBroadPhase) -> *mut PxBroadPhaseRegions;

    /// Retrieves the broadphase allocator.
    ///
    /// User-provided buffers should ideally be allocated with this allocator, for best performance.
    /// This is especially true for the GPU broadphases, whose buffers need to be allocated in CUDA
    /// host memory.
    ///
    /// The broadphase allocator.
    pub fn PxBroadPhase_getAllocator_mut(self_: *mut PxBroadPhase) -> *mut PxAllocatorCallback;

    /// Retrieves the profiler's context ID.
    ///
    /// The context ID.
    pub fn PxBroadPhase_getContextID(self_: *const PxBroadPhase) -> u64;

    /// Sets a scratch buffer
    ///
    /// Some broadphases might take advantage of a scratch buffer to limit runtime allocations.
    ///
    /// All broadphases still work without providing a scratch buffer, this is an optional function
    /// that can potentially reduce runtime allocations.
    pub fn PxBroadPhase_setScratchBlock_mut(self_: *mut PxBroadPhase, scratchBlock: *mut std::ffi::c_void, size: u32);

    /// Updates the broadphase and computes the lists of created/deleted pairs.
    ///
    /// The provided update data describes changes to objects since the last broadphase update.
    ///
    /// To benefit from potentially multithreaded implementations, it is necessary to provide a continuation
    /// task to the function. It is legal to pass NULL there, but the underlying (CPU) implementations will
    /// then run single-threaded.
    pub fn PxBroadPhase_update_mut(self_: *mut PxBroadPhase, updateData: *const PxBroadPhaseUpdateData, continuation: *mut PxBaseTask);

    /// Retrieves the broadphase results after an update.
    ///
    /// This should be called once after each update call to retrieve the results of the broadphase. The
    /// results are incremental, i.e. the system only returns new and lost pairs, not all current pairs.
    pub fn PxBroadPhase_fetchResults_mut(self_: *mut PxBroadPhase, results: *mut PxBroadPhaseResults);

    /// Helper for single-threaded updates.
    ///
    /// This short helper function performs a single-theaded update and reports the results in a single call.
    pub fn PxBroadPhase_updateAndFetchResults_mut(self_: *mut PxBroadPhase, results: *mut PxBroadPhaseResults, updateData: *const PxBroadPhaseUpdateData);

    /// Helper for single-threaded updates.
    ///
    /// This short helper function performs a single-theaded update and reports the results in a single call.
    pub fn PxBroadPhase_update_mut_1(self_: *mut PxBroadPhase, results: *mut PxBroadPhaseResults, updateData: *const PxBroadPhaseUpdateData);

    /// Broadphase factory function.
    ///
    /// Use this function to create a new standalone broadphase.
    ///
    /// Newly created broadphase, or NULL
    pub fn phys_PxCreateBroadPhase(desc: *const PxBroadPhaseDesc) -> *mut PxBroadPhase;

    pub fn PxAABBManager_release_mut(self_: *mut PxAABBManager);

    /// Retrieves the underlying broadphase.
    ///
    /// The managed broadphase.
    pub fn PxAABBManager_getBroadPhase_mut(self_: *mut PxAABBManager) -> *mut PxBroadPhase;

    /// Retrieves the managed bounds.
    ///
    /// This is needed as input parameters to functions like PxBroadPhaseRegions::addRegion.
    ///
    /// The managed object bounds.
    pub fn PxAABBManager_getBounds(self_: *const PxAABBManager) -> *const PxBounds3;

    /// Retrieves the managed distances.
    ///
    /// This is needed as input parameters to functions like PxBroadPhaseRegions::addRegion.
    ///
    /// The managed object distances.
    pub fn PxAABBManager_getDistances(self_: *const PxAABBManager) -> *const f32;

    /// Retrieves the managed filter groups.
    ///
    /// The managed object groups.
    pub fn PxAABBManager_getGroups(self_: *const PxAABBManager) -> *const u32;

    /// Retrieves the managed buffers' capacity.
    ///
    /// Bounds, distances and groups buffers have the same capacity.
    ///
    /// The managed buffers' capacity.
    pub fn PxAABBManager_getCapacity(self_: *const PxAABBManager) -> u32;

    /// Adds an object to the manager.
    ///
    /// Objects' indices are externally managed, i.e. they must be provided by users (as opposed to handles
    /// that could be returned by this manager). The design allows users to identify an object by a single ID,
    /// and use the same ID in multiple sub-systems.
    pub fn PxAABBManager_addObject_mut(self_: *mut PxAABBManager, index: u32, bounds: *const PxBounds3, group: u32, distance: f32);

    /// Removes an object from the manager.
    pub fn PxAABBManager_removeObject_mut(self_: *mut PxAABBManager, index: u32);

    /// Updates an object in the manager.
    ///
    /// This call can update an object's bounds, distance, or both.
    /// It is not possible to update an object's filter group.
    pub fn PxAABBManager_updateObject_mut(self_: *mut PxAABBManager, index: u32, bounds: *const PxBounds3, distance: *const f32);

    /// Updates the broadphase and computes the lists of created/deleted pairs.
    ///
    /// The data necessary for updating the broadphase is internally computed by the AABB manager.
    ///
    /// To benefit from potentially multithreaded implementations, it is necessary to provide a continuation
    /// task to the function. It is legal to pass NULL there, but the underlying (CPU) implementations will
    /// then run single-threaded.
    pub fn PxAABBManager_update_mut(self_: *mut PxAABBManager, continuation: *mut PxBaseTask);

    /// Retrieves the broadphase results after an update.
    ///
    /// This should be called once after each update call to retrieve the results of the broadphase. The
    /// results are incremental, i.e. the system only returns new and lost pairs, not all current pairs.
    pub fn PxAABBManager_fetchResults_mut(self_: *mut PxAABBManager, results: *mut PxBroadPhaseResults);

    /// Helper for single-threaded updates.
    ///
    /// This short helper function performs a single-theaded update and reports the results in a single call.
    pub fn PxAABBManager_updateAndFetchResults_mut(self_: *mut PxAABBManager, results: *mut PxBroadPhaseResults);

    /// Helper for single-threaded updates.
    ///
    /// This short helper function performs a single-theaded update and reports the results in a single call.
    pub fn PxAABBManager_update_mut_1(self_: *mut PxAABBManager, results: *mut PxBroadPhaseResults);

    /// AABB manager factory function.
    ///
    /// Use this function to create a new standalone high-level broadphase.
    ///
    /// Newly created AABB manager, or NULL
    pub fn phys_PxCreateAABBManager(broadphase: *mut PxBroadPhase) -> *mut PxAABBManager;

    /// constructor sets to default
    pub fn PxSceneLimits_new() -> PxSceneLimits;

    /// (re)sets the structure to the default
    pub fn PxSceneLimits_setToDefault_mut(self_: *mut PxSceneLimits);

    /// Returns true if the descriptor is valid.
    ///
    /// true if the current settings are valid.
    pub fn PxSceneLimits_isValid(self_: *const PxSceneLimits) -> bool;

    pub fn PxGpuDynamicsMemoryConfig_new() -> PxGpuDynamicsMemoryConfig;

    pub fn PxGpuDynamicsMemoryConfig_isValid(self_: *const PxGpuDynamicsMemoryConfig) -> bool;

    /// constructor sets to default.
    pub fn PxSceneDesc_new(scale: *const PxTolerancesScale) -> PxSceneDesc;

    /// (re)sets the structure to the default.
    pub fn PxSceneDesc_setToDefault_mut(self_: *mut PxSceneDesc, scale: *const PxTolerancesScale);

    /// Returns true if the descriptor is valid.
    ///
    /// true if the current settings are valid.
    pub fn PxSceneDesc_isValid(self_: *const PxSceneDesc) -> bool;

    pub fn PxSceneDesc_getTolerancesScale(self_: *const PxSceneDesc) -> *const PxTolerancesScale;

    pub fn PxGpuDynamicsMemoryConfigStatistics_new() -> PxGpuDynamicsMemoryConfigStatistics;

    /// Get number of broadphase volumes added for the current simulation step.
    ///
    /// Number of broadphase volumes added.
    pub fn PxSimulationStatistics_getNbBroadPhaseAdds(self_: *const PxSimulationStatistics) -> u32;

    /// Get number of broadphase volumes removed for the current simulation step.
    ///
    /// Number of broadphase volumes removed.
    pub fn PxSimulationStatistics_getNbBroadPhaseRemoves(self_: *const PxSimulationStatistics) -> u32;

    /// Get number of shape collision pairs of a certain type processed for the current simulation step.
    ///
    /// There is an entry for each geometry pair type.
    ///
    /// entry[i][j] = entry[j][i], hence, if you want the sum of all pair
    /// types, you need to discard the symmetric entries
    ///
    /// Number of processed pairs of the specified geometry types.
    pub fn PxSimulationStatistics_getRbPairStats(self_: *const PxSimulationStatistics, pairType: RbPairStatsType, g0: PxGeometryType, g1: PxGeometryType) -> u32;

    pub fn PxSimulationStatistics_new() -> PxSimulationStatistics;

    /// Sets the PVD flag. See PxPvdSceneFlag.
    pub fn PxPvdSceneClient_setScenePvdFlag_mut(self_: *mut PxPvdSceneClient, flag: PxPvdSceneFlag, value: bool);

    /// Sets the PVD flags. See PxPvdSceneFlags.
    pub fn PxPvdSceneClient_setScenePvdFlags_mut(self_: *mut PxPvdSceneClient, flags: PxPvdSceneFlags);

    /// Retrieves the PVD flags. See PxPvdSceneFlags.
    pub fn PxPvdSceneClient_getScenePvdFlags(self_: *const PxPvdSceneClient) -> PxPvdSceneFlags;

    /// update camera on PVD application's render window
    pub fn PxPvdSceneClient_updateCamera_mut(self_: *mut PxPvdSceneClient, name: *const std::ffi::c_char, origin: *const PxVec3, up: *const PxVec3, target: *const PxVec3);

    /// draw points on PVD application's render window
    pub fn PxPvdSceneClient_drawPoints_mut(self_: *mut PxPvdSceneClient, points: *const PxDebugPoint, count: u32);

    /// draw lines on PVD application's render window
    pub fn PxPvdSceneClient_drawLines_mut(self_: *mut PxPvdSceneClient, lines: *const PxDebugLine, count: u32);

    /// draw triangles on PVD application's render window
    pub fn PxPvdSceneClient_drawTriangles_mut(self_: *mut PxPvdSceneClient, triangles: *const PxDebugTriangle, count: u32);

    /// draw text on PVD application's render window
    pub fn PxPvdSceneClient_drawText_mut(self_: *mut PxPvdSceneClient, text: *const PxDebugText);

    pub fn PxDominanceGroupPair_new(a: u8, b: u8) -> PxDominanceGroupPair;

    pub fn PxBroadPhaseCallback_delete(self_: *mut PxBroadPhaseCallback);

    /// Out-of-bounds notification.
    ///
    /// This function is called when an object leaves the broad-phase.
    pub fn PxBroadPhaseCallback_onObjectOutOfBounds_mut(self_: *mut PxBroadPhaseCallback, shape: *mut PxShape, actor: *mut PxActor);

    /// Out-of-bounds notification.
    ///
    /// This function is called when an aggregate leaves the broad-phase.
    pub fn PxBroadPhaseCallback_onObjectOutOfBounds_mut_1(self_: *mut PxBroadPhaseCallback, aggregate: *mut PxAggregate);

    pub fn PxPostSolveCallback_delete(self_: *mut PxPostSolveCallback);

    /// Deletes the scene.
    ///
    /// Removes any actors and constraint shaders from this scene
    /// (if the user hasn't already done so).
    ///
    /// Be sure	to not keep a reference to this object after calling release.
    /// Avoid release calls while the scene is simulating (in between simulate() and fetchResults() calls).
    pub fn PxScene_release_mut(self_: *mut PxScene);

    /// Sets a scene flag. You can only set one flag at a time.
    ///
    /// Not all flags are mutable and changing some will result in an error. Please check [`PxSceneFlag`] to see which flags can be changed.
    pub fn PxScene_setFlag_mut(self_: *mut PxScene, flag: PxSceneFlag, value: bool);

    /// Get the scene flags.
    ///
    /// The scene flags. See [`PxSceneFlag`]
    pub fn PxScene_getFlags(self_: *const PxScene) -> PxSceneFlags;

    /// Set new scene limits.
    ///
    /// Increase the maximum capacity of various data structures in the scene. The new capacities will be
    /// at least as large as required to deal with the objects currently in the scene. Further, these values
    /// are for preallocation and do not represent hard limits.
    pub fn PxScene_setLimits_mut(self_: *mut PxScene, limits: *const PxSceneLimits);

    /// Get current scene limits.
    ///
    /// Current scene limits.
    pub fn PxScene_getLimits(self_: *const PxScene) -> PxSceneLimits;

    /// Call this method to retrieve the Physics SDK.
    ///
    /// The physics SDK this scene is associated with.
    pub fn PxScene_getPhysics_mut(self_: *mut PxScene) -> *mut PxPhysics;

    /// Retrieves the scene's internal timestamp, increased each time a simulation step is completed.
    ///
    /// scene timestamp
    pub fn PxScene_getTimestamp(self_: *const PxScene) -> u32;

    /// Sets a name string for the Scene that can be retrieved with getName().
    ///
    /// This is for debugging and is not used by the SDK. The string is not copied by the SDK,
    /// only the pointer is stored.
    ///
    /// Default:
    /// NULL
    pub fn PxScene_setName_mut(self_: *mut PxScene, name: *const std::ffi::c_char);

    /// Retrieves the name string set with setName().
    ///
    /// Name string associated with the Scene.
    pub fn PxScene_getName(self_: *const PxScene) -> *const std::ffi::c_char;

    /// Adds an articulation to this scene.
    ///
    /// If the articulation is already assigned to a scene (see [`PxArticulationReducedCoordinate::getScene`]), the call is ignored and an error is issued.
    ///
    /// True if success
    pub fn PxScene_addArticulation_mut(self_: *mut PxScene, articulation: *mut PxArticulationReducedCoordinate) -> bool;

    /// Removes an articulation from this scene.
    ///
    /// If the articulation is not part of this scene (see [`PxArticulationReducedCoordinate::getScene`]), the call is ignored and an error is issued.
    ///
    /// If the articulation is in an aggregate it will be removed from the aggregate.
    pub fn PxScene_removeArticulation_mut(self_: *mut PxScene, articulation: *mut PxArticulationReducedCoordinate, wakeOnLostTouch: bool);

    /// Adds an actor to this scene.
    ///
    /// If the actor is already assigned to a scene (see [`PxActor::getScene`]), the call is ignored and an error is issued.
    ///
    /// If the actor has an invalid constraint, in checked builds the call is ignored and an error is issued.
    ///
    /// You can not add individual articulation links (see [`PxArticulationLink`]) to the scene. Use #addArticulation() instead.
    ///
    /// If the actor is a PxRigidActor then each assigned PxConstraint object will get added to the scene automatically if
    /// it connects to another actor that is part of the scene already.
    ///
    /// When a BVH is provided the actor shapes are grouped together.
    /// The scene query pruning structure inside PhysX SDK will store/update one
    /// bound per actor. The scene queries against such an actor will query actor
    /// bounds and then make a local space query against the provided BVH, which is in actor's local space.
    ///
    /// True if success
    pub fn PxScene_addActor_mut(self_: *mut PxScene, actor: *mut PxActor, bvh: *const PxBVH) -> bool;

    /// Adds actors to this scene. Only supports actors of type PxRigidStatic and PxRigidDynamic.
    ///
    /// This method only supports actors of type PxRigidStatic and PxRigidDynamic. For other actors, use addActor() instead.
    /// For articulation links, use addArticulation().
    ///
    /// If one of the actors is already assigned to a scene (see [`PxActor::getScene`]), the call is ignored and an error is issued.
    ///
    /// If an actor in the array contains an invalid constraint, in checked builds the call is ignored and an error is issued.
    ///
    /// If an actor in the array is a PxRigidActor then each assigned PxConstraint object will get added to the scene automatically if
    /// it connects to another actor that is part of the scene already.
    ///
    /// this method is optimized for high performance.
    ///
    /// True if success
    pub fn PxScene_addActors_mut(self_: *mut PxScene, actors: *const *mut PxActor, nbActors: u32) -> bool;

    /// Adds a pruning structure together with its actors to this scene. Only supports actors of type PxRigidStatic and PxRigidDynamic.
    ///
    /// This method only supports actors of type PxRigidStatic and PxRigidDynamic. For other actors, use addActor() instead.
    /// For articulation links, use addArticulation().
    ///
    /// If an actor in the pruning structure contains an invalid constraint, in checked builds the call is ignored and an error is issued.
    ///
    /// For all actors in the pruning structure each assigned PxConstraint object will get added to the scene automatically if
    /// it connects to another actor that is part of the scene already.
    ///
    /// This method is optimized for high performance.
    ///
    /// Merging a PxPruningStructure into an active scene query optimization AABB tree might unbalance the tree. A typical use case for
    /// PxPruningStructure is a large world scenario where blocks of closely positioned actors get streamed in. The merge process finds the
    /// best node in the active scene query optimization AABB tree and inserts the PxPruningStructure. Therefore using PxPruningStructure
    /// for actors scattered throughout the world will result in an unbalanced tree.
    ///
    /// True if success
    pub fn PxScene_addActors_mut_1(self_: *mut PxScene, pruningStructure: *const PxPruningStructure) -> bool;

    /// Removes an actor from this scene.
    ///
    /// If the actor is not part of this scene (see [`PxActor::getScene`]), the call is ignored and an error is issued.
    ///
    /// You can not remove individual articulation links (see [`PxArticulationLink`]) from the scene. Use #removeArticulation() instead.
    ///
    /// If the actor is a PxRigidActor then all assigned PxConstraint objects will get removed from the scene automatically.
    ///
    /// If the actor is in an aggregate it will be removed from the aggregate.
    pub fn PxScene_removeActor_mut(self_: *mut PxScene, actor: *mut PxActor, wakeOnLostTouch: bool);

    /// Removes actors from this scene. Only supports actors of type PxRigidStatic and PxRigidDynamic.
    ///
    /// This method only supports actors of type PxRigidStatic and PxRigidDynamic. For other actors, use removeActor() instead.
    /// For articulation links, use removeArticulation().
    ///
    /// If some actor is not part of this scene (see [`PxActor::getScene`]), the actor remove is ignored and an error is issued.
    ///
    /// You can not remove individual articulation links (see [`PxArticulationLink`]) from the scene. Use #removeArticulation() instead.
    ///
    /// If the actor is a PxRigidActor then all assigned PxConstraint objects will get removed from the scene automatically.
    pub fn PxScene_removeActors_mut(self_: *mut PxScene, actors: *const *mut PxActor, nbActors: u32, wakeOnLostTouch: bool);

    /// Adds an aggregate to this scene.
    ///
    /// If the aggregate is already assigned to a scene (see [`PxAggregate::getScene`]), the call is ignored and an error is issued.
    ///
    /// If the aggregate contains an actor with an invalid constraint, in checked builds the call is ignored and an error is issued.
    ///
    /// If the aggregate already contains actors, those actors are added to the scene as well.
    ///
    /// True if success
    pub fn PxScene_addAggregate_mut(self_: *mut PxScene, aggregate: *mut PxAggregate) -> bool;

    /// Removes an aggregate from this scene.
    ///
    /// If the aggregate is not part of this scene (see [`PxAggregate::getScene`]), the call is ignored and an error is issued.
    ///
    /// If the aggregate contains actors, those actors are removed from the scene as well.
    pub fn PxScene_removeAggregate_mut(self_: *mut PxScene, aggregate: *mut PxAggregate, wakeOnLostTouch: bool);

    /// Adds objects in the collection to this scene.
    ///
    /// This function adds the following types of objects to this scene: PxRigidActor (except PxArticulationLink), PxAggregate, PxArticulationReducedCoordinate.
    /// This method is typically used after deserializing the collection in order to populate the scene with deserialized objects.
    ///
    /// If the collection contains an actor with an invalid constraint, in checked builds the call is ignored and an error is issued.
    ///
    /// True if success
    pub fn PxScene_addCollection_mut(self_: *mut PxScene, collection: *const PxCollection) -> bool;

    /// Retrieve the number of actors of certain types in the scene. For supported types, see PxActorTypeFlags.
    ///
    /// the number of actors.
    pub fn PxScene_getNbActors(self_: *const PxScene, types: PxActorTypeFlags) -> u32;

    /// Retrieve an array of all the actors of certain types in the scene. For supported types, see PxActorTypeFlags.
    ///
    /// Number of actors written to the buffer.
    pub fn PxScene_getActors(self_: *const PxScene, types: PxActorTypeFlags, userBuffer: *mut *mut PxActor, bufferSize: u32, startIndex: u32) -> u32;

    /// Queries the PxScene for a list of the PxActors whose transforms have been
    /// updated during the previous simulation step. Only includes actors of type PxRigidDynamic and PxArticulationLink.
    ///
    /// PxSceneFlag::eENABLE_ACTIVE_ACTORS must be set.
    ///
    /// Do not use this method while the simulation is running. Calls to this method while the simulation is running will be ignored and NULL will be returned.
    ///
    /// This list may contain actors that have been released after fetchResults() of the previous simulation step. It is the user's
    /// responsibility to track such actors and avoid dereferencing the corresponding pointers.
    ///
    /// A pointer to the list of active PxActors generated during the last call to fetchResults().
    pub fn PxScene_getActiveActors_mut(self_: *mut PxScene, nbActorsOut: *mut u32) -> *mut *mut PxActor;

    /// Retrieve the number of deformable surfaces in the scene.
    ///
    /// the number of deformable surfaces.
    ///
    /// See getDeformableSurfaces()
    pub fn PxScene_getNbDeformableSurfaces(self_: *const PxScene) -> u32;

    /// Retrieve an array of all the deformable surfaces in the scene.
    ///
    /// Number of deformable surfaces written to the buffer
    pub fn PxScene_getDeformableSurfaces(self_: *const PxScene, userBuffer: *mut *mut PxDeformableSurface, bufferSize: u32, startIndex: u32) -> u32;

    /// Retrieve the number of deformable volumes in the scene.
    ///
    /// the number of deformable volumes.
    pub fn PxScene_getNbDeformableVolumes(self_: *const PxScene) -> u32;

    /// Retrieve an array of all the deformable volumes in the scene.
    ///
    /// Number of actors written to the buffer.
    pub fn PxScene_getDeformableVolumes(self_: *const PxScene, userBuffer: *mut *mut PxDeformableVolume, bufferSize: u32, startIndex: u32) -> u32;

    /// Retrieve the number of particle systems of the requested type in the scene.
    ///
    /// the number particle systems.
    pub fn PxScene_getNbPBDParticleSystems(self_: *const PxScene) -> u32;

    /// Returns the number of articulations in the scene.
    ///
    /// the number of articulations in this scene.
    pub fn PxScene_getNbArticulations(self_: *const PxScene) -> u32;

    /// Retrieve all the articulations in the scene.
    ///
    /// Number of articulations written to the buffer.
    pub fn PxScene_getArticulations(self_: *const PxScene, userBuffer: *mut *mut PxArticulationReducedCoordinate, bufferSize: u32, startIndex: u32) -> u32;

    /// Returns the number of constraint shaders in the scene.
    ///
    /// the number of constraint shaders in this scene.
    pub fn PxScene_getNbConstraints(self_: *const PxScene) -> u32;

    /// Retrieve all the constraint shaders in the scene.
    ///
    /// Number of constraint shaders written to the buffer.
    pub fn PxScene_getConstraints(self_: *const PxScene, userBuffer: *mut *mut PxConstraint, bufferSize: u32, startIndex: u32) -> u32;

    /// Returns the number of aggregates in the scene.
    ///
    /// the number of aggregates in this scene.
    pub fn PxScene_getNbAggregates(self_: *const PxScene) -> u32;

    /// Retrieve all the aggregates in the scene.
    ///
    /// Number of aggregates written to the buffer.
    pub fn PxScene_getAggregates(self_: *const PxScene, userBuffer: *mut *mut PxAggregate, bufferSize: u32, startIndex: u32) -> u32;

    /// Specifies the dominance behavior of contacts between two actors with two certain dominance groups.
    ///
    /// It is possible to assign each actor to a dominance groups using [`PxActor::setDominanceGroup`]().
    ///
    /// With dominance groups one can have all contacts created between actors act in one direction only. This is useful, for example, if you
    /// want an object to push debris out of its way and be unaffected,while still responding physically to forces and collisions
    /// with non-debris objects.
    ///
    /// Whenever a contact between two actors (a0, a1) needs to be solved, the groups (g0, g1) of both
    /// actors are retrieved. Then the PxDominanceGroupPair setting for this group pair is retrieved with getDominanceGroupPair(g0, g1).
    ///
    /// In the contact, PxDominanceGroupPair::dominance0 becomes the dominance setting for a0, and
    /// PxDominanceGroupPair::dominance1 becomes the dominance setting for a1. A dominanceN setting of 1.0f, the default,
    /// will permit aN to be pushed or pulled by a(1-N) through the contact. A dominanceN setting of 0.0f, will however
    /// prevent aN to be pushed by a(1-N) via the contact. Thus, a PxDominanceGroupPair of (1.0f, 0.0f) makes
    /// the interaction one-way.
    ///
    /// The matrix sampled by getDominanceGroupPair(g1, g2) is initialised by default such that:
    ///
    /// if g1 == g2, then (1.0f, 1.0f) is returned
    /// if g1
    /// <
    /// g2, then (0.0f, 1.0f) is returned
    /// if g1 >  g2, then (1.0f, 0.0f) is returned
    ///
    /// In other words, we permit actors in higher groups to be pushed around by actors in lower groups by default.
    ///
    /// These settings should cover most applications, and in fact not overriding these settings may likely result in higher performance.
    ///
    /// It is not possible to make the matrix asymetric, or to change the diagonal. In other words:
    ///
    /// it is not possible to change (g1, g2) if (g1==g2)
    /// if you set
    ///
    /// (g1, g2) to X, then (g2, g1) will implicitly and automatically be set to ~X, where:
    ///
    /// ~(1.0f, 1.0f) is (1.0f, 1.0f)
    /// ~(0.0f, 1.0f) is (1.0f, 0.0f)
    /// ~(1.0f, 0.0f) is (0.0f, 1.0f)
    ///
    /// These two restrictions are to make sure that contacts between two actors will always evaluate to the same dominance
    /// setting, regardless of the order of the actors.
    ///
    /// Dominance settings are currently specified as floats 0.0f or 1.0f because in the future we may permit arbitrary
    /// fractional settings to express 'partly-one-way' interactions.
    ///
    /// Sleeping:
    /// Does
    /// NOT
    /// wake actors up automatically.
    pub fn PxScene_setDominanceGroupPair_mut(self_: *mut PxScene, group1: u8, group2: u8, dominance: *const PxDominanceGroupPair);

    /// Samples the dominance matrix.
    pub fn PxScene_getDominanceGroupPair(self_: *const PxScene, group1: u8, group2: u8) -> PxDominanceGroupPair;

    /// Return the cpu dispatcher that was set in PxSceneDesc::cpuDispatcher when creating the scene with PxPhysics::createScene
    pub fn PxScene_getCpuDispatcher(self_: *const PxScene) -> *mut PxCpuDispatcher;

    /// Return the CUDA context manager that was set in PxSceneDesc::cudaContextManager when creating the scene with PxPhysics::createScene
    ///
    /// Platform specific:
    /// Applies to PC GPU only.
    pub fn PxScene_getCudaContextManager(self_: *const PxScene) -> *mut PxCudaContextManager;

    /// Reserves a new client ID.
    ///
    /// PX_DEFAULT_CLIENT is always available as the default clientID.
    /// Additional clients are returned by this function. Clients cannot be released once created.
    /// An error is reported when more than a supported number of clients (currently 128) are created.
    pub fn PxScene_createClient_mut(self_: *mut PxScene) -> u8;

    /// Sets a user notify object which receives special simulation events when they occur.
    ///
    /// Do not set the callback while the simulation is running. Calls to this method while the simulation is running will be ignored.
    pub fn PxScene_setSimulationEventCallback_mut(self_: *mut PxScene, callback: *mut PxSimulationEventCallback);

    /// Retrieves the simulationEventCallback pointer set with setSimulationEventCallback().
    ///
    /// The current user notify pointer. See [`PxSimulationEventCallback`].
    pub fn PxScene_getSimulationEventCallback(self_: *const PxScene) -> *mut PxSimulationEventCallback;

    /// Sets a user callback object, which receives callbacks on all contacts generated for specified actors.
    ///
    /// Do not set the callback while the simulation is running. Calls to this method while the simulation is running will be ignored.
    pub fn PxScene_setContactModifyCallback_mut(self_: *mut PxScene, callback: *mut PxContactModifyCallback);

    /// Sets a user callback object, which receives callbacks on all CCD contacts generated for specified actors.
    ///
    /// Do not set the callback while the simulation is running. Calls to this method while the simulation is running will be ignored.
    pub fn PxScene_setCCDContactModifyCallback_mut(self_: *mut PxScene, callback: *mut PxCCDContactModifyCallback);

    /// Retrieves the PxContactModifyCallback pointer set with setContactModifyCallback().
    ///
    /// The current user contact modify callback pointer. See [`PxContactModifyCallback`].
    pub fn PxScene_getContactModifyCallback(self_: *const PxScene) -> *mut PxContactModifyCallback;

    /// Retrieves the PxCCDContactModifyCallback pointer set with setContactModifyCallback().
    ///
    /// The current user contact modify callback pointer. See [`PxContactModifyCallback`].
    pub fn PxScene_getCCDContactModifyCallback(self_: *const PxScene) -> *mut PxCCDContactModifyCallback;

    /// Sets a broad-phase user callback object.
    ///
    /// Do not set the callback while the simulation is running. Calls to this method while the simulation is running will be ignored.
    pub fn PxScene_setBroadPhaseCallback_mut(self_: *mut PxScene, callback: *mut PxBroadPhaseCallback);

    /// Retrieves the PxBroadPhaseCallback pointer set with setBroadPhaseCallback().
    ///
    /// The current broad-phase callback pointer. See [`PxBroadPhaseCallback`].
    pub fn PxScene_getBroadPhaseCallback(self_: *const PxScene) -> *mut PxBroadPhaseCallback;

    /// Sets the shared global filter data which will get passed into the filter shader.
    ///
    /// It is the user's responsibility to ensure that changing the shared global filter data does not change the filter output value for existing pairs.
    /// If the filter output for existing pairs does change nonetheless then such a change will not take effect until the pair gets refiltered.
    /// resetFiltering() can be used to explicitly refilter the pairs of specific objects.
    ///
    /// The provided data will get copied to internal buffers and this copy will be used for filtering calls.
    ///
    /// Do not use this method while the simulation is running. Calls to this method while the simulation is running will be ignored.
    pub fn PxScene_setFilterShaderData_mut(self_: *mut PxScene, data: *const std::ffi::c_void, dataSize: u32);

    /// Gets the shared global filter data in use for this scene.
    ///
    /// The reference points to a copy of the original filter data specified in [`PxSceneDesc`].filterShaderData or provided by #setFilterShaderData().
    ///
    /// Shared filter data for filter shader.
    pub fn PxScene_getFilterShaderData(self_: *const PxScene) -> *const std::ffi::c_void;

    /// Gets the size of the shared global filter data ([`PxSceneDesc`].filterShaderData)
    ///
    /// Size of shared filter data [bytes].
    pub fn PxScene_getFilterShaderDataSize(self_: *const PxScene) -> u32;

    /// Gets the custom collision filter shader in use for this scene.
    ///
    /// Filter shader class that defines the collision pair filtering.
    pub fn PxScene_getFilterShader(self_: *const PxScene) -> *mut std::ffi::c_void;

    /// Gets the custom collision filter callback in use for this scene.
    ///
    /// Filter callback class that defines the collision pair filtering.
    pub fn PxScene_getFilterCallback(self_: *const PxScene) -> *mut PxSimulationFilterCallback;

    /// Marks the object to reset interactions and re-run collision filters in the next simulation step.
    ///
    /// This call forces the object to remove all existing collision interactions, to search anew for existing contact
    /// pairs and to run the collision filters again for found collision pairs.
    ///
    /// The operation is supported for PxRigidActor objects only.
    ///
    /// All persistent state of existing interactions will be lost and can not be retrieved even if the same collison pair
    /// is found again in the next step. This will mean, for example, that you will not get notified about persistent contact
    /// for such an interaction (see [`PxPairFlag::eNOTIFY_TOUCH_PERSISTS`]), the contact pair will be interpreted as newly found instead.
    ///
    /// Lost touch contact reports will be sent for every collision pair which includes this shape, if they have
    /// been requested through [`PxPairFlag::eNOTIFY_TOUCH_LOST`] or #PxPairFlag::eNOTIFY_THRESHOLD_FORCE_LOST.
    ///
    /// This is an expensive operation, don't use it if you don't have to.
    ///
    /// Can be used to retrieve collision pairs that were killed by the collision filters (see [`PxFilterFlag::eKILL`])
    ///
    /// It is invalid to use this method if the actor has not been added to a scene already.
    ///
    /// It is invalid to use this method if PxActorFlag::eDISABLE_SIMULATION is set.
    ///
    /// Do not use this method while the simulation is running.
    ///
    /// Sleeping:
    /// Does wake up the actor.
    ///
    /// True if success
    pub fn PxScene_resetFiltering_mut(self_: *mut PxScene, actor: *mut PxActor) -> bool;

    /// Marks the object to reset interactions and re-run collision filters for specified shapes in the next simulation step.
    ///
    /// This is a specialization of the resetFiltering(PxActor
    /// &
    /// actor) method and allows to reset interactions for specific shapes of
    /// a PxRigidActor.
    ///
    /// Do not use this method while the simulation is running.
    ///
    /// Sleeping:
    /// Does wake up the actor.
    pub fn PxScene_resetFiltering_mut_1(self_: *mut PxScene, actor: *mut PxRigidActor, shapes: *const *mut PxShape, shapeCount: u32) -> bool;

    /// Gets the pair filtering mode for kinematic-kinematic pairs.
    ///
    /// Filtering mode for kinematic-kinematic pairs.
    pub fn PxScene_getKinematicKinematicFilteringMode(self_: *const PxScene) -> PxPairFilteringMode;

    /// Gets the pair filtering mode for static-kinematic pairs.
    ///
    /// Filtering mode for static-kinematic pairs.
    pub fn PxScene_getStaticKinematicFilteringMode(self_: *const PxScene) -> PxPairFilteringMode;

    /// Advances the simulation by an elapsedTime time.
    ///
    /// Large elapsedTime values can lead to instabilities. In such cases elapsedTime
    /// should be subdivided into smaller time intervals and simulate() should be called
    /// multiple times for each interval.
    ///
    /// Calls to simulate() should pair with calls to fetchResults():
    /// Each fetchResults() invocation corresponds to exactly one simulate()
    /// invocation; calling simulate() twice without an intervening fetchResults()
    /// or fetchResults() twice without an intervening simulate() causes an error
    /// condition.
    ///
    /// scene->simulate();
    /// ...do some processing until physics is computed...
    /// scene->fetchResults();
    /// ...now results of run may be retrieved.
    ///
    /// True if success
    pub fn PxScene_simulate_mut(self_: *mut PxScene, elapsedTime: f32, completionTask: *mut PxBaseTask, scratchMemBlock: *mut std::ffi::c_void, scratchMemBlockSize: u32, controlSimulation: bool) -> bool;

    /// Performs dynamics phase of the simulation pipeline.
    ///
    /// Calls to advance() should follow calls to fetchCollision(). An error message will be issued if this sequence is not followed.
    ///
    /// True if success
    pub fn PxScene_advance_mut(self_: *mut PxScene, completionTask: *mut PxBaseTask) -> bool;

    /// Performs collision detection for the scene over elapsedTime
    ///
    /// Calls to collide() should be the first method called to simulate a frame.
    ///
    /// True if success
    pub fn PxScene_collide_mut(self_: *mut PxScene, elapsedTime: f32, completionTask: *mut PxBaseTask, scratchMemBlock: *mut std::ffi::c_void, scratchMemBlockSize: u32, controlSimulation: bool) -> bool;

    /// This checks to see if the simulation run has completed.
    ///
    /// This does not cause the data available for reading to be updated with the results of the simulation, it is simply a status check.
    /// The bool will allow it to either return immediately or block waiting for the condition to be met so that it can return true
    ///
    /// True if the results are available.
    pub fn PxScene_checkResults_mut(self_: *mut PxScene, block: bool) -> bool;

    /// This method must be called after collide() and before advance(). It will wait for the collision phase to finish. If the user makes an illegal simulation call, the SDK will issue an error
    /// message.
    pub fn PxScene_fetchCollision_mut(self_: *mut PxScene, block: bool) -> bool;

    /// This is the big brother to checkResults() it basically does the following:
    ///
    /// True if the results have been fetched.
    pub fn PxScene_fetchResults_mut(self_: *mut PxScene, block: bool, errorState: *mut u32) -> bool;

    /// This call performs the first section of fetchResults, and returns a pointer to the contact streams output by the simulation. It can be used to process contact pairs in parallel, which is often a limiting factor
    /// for fetchResults() performance.
    ///
    /// After calling this function and processing the contact streams, call fetchResultsFinish(). Note that writes to the simulation are not
    /// permitted between the start of fetchResultsStart() and the end of fetchResultsFinish().
    ///
    /// True if the results have been fetched.
    pub fn PxScene_fetchResultsStart_mut(self_: *mut PxScene, contactPairs: *mut *const PxContactPairHeader, nbContactPairs: *mut u32, block: bool) -> bool;

    /// This call processes all event callbacks in parallel. It takes a continuation task, which will be executed once all callbacks have been processed.
    ///
    /// This is a utility function to make it easier to process callbacks in parallel using the PhysX task system. It can only be used in conjunction with
    /// fetchResultsStart(...) and fetchResultsFinish(...)
    pub fn PxScene_processCallbacks_mut(self_: *mut PxScene, continuation: *mut PxBaseTask);

    /// This call performs the second section of fetchResults.
    ///
    /// It must be called after fetchResultsStart() returns and contact reports have been processed.
    ///
    /// Note that once fetchResultsFinish() has been called, the contact streams returned in fetchResultsStart() will be invalid.
    pub fn PxScene_fetchResultsFinish_mut(self_: *mut PxScene, errorState: *mut u32);

    /// This call performs the synchronization of particle system data copies.
    pub fn PxScene_fetchResultsParticleSystem_mut(self_: *mut PxScene);

    /// Clear internal buffers and free memory.
    ///
    /// This method can be used to clear buffers and free internal memory without having to destroy the scene. Can be useful if
    /// the physics data gets streamed in and a checkpoint with a clean state should be created.
    ///
    /// It is not allowed to call this method while the simulation is running. The call will fail.
    pub fn PxScene_flushSimulation_mut(self_: *mut PxScene, sendPendingReports: bool);

    /// Sets a constant gravity for the entire scene.
    ///
    /// Do not use this method while the simulation is running.
    ///
    /// Sleeping:
    /// Does
    /// NOT
    /// wake the actor up automatically.
    pub fn PxScene_setGravity_mut(self_: *mut PxScene, vec: *const PxVec3);

    /// Retrieves the current gravity setting.
    ///
    /// The current gravity for the scene.
    pub fn PxScene_getGravity(self_: *const PxScene) -> PxVec3;

    /// Set the bounce threshold velocity.  Collision speeds below this threshold will not cause a bounce.
    ///
    /// Do not use this method while the simulation is running.
    pub fn PxScene_setBounceThresholdVelocity_mut(self_: *mut PxScene, t: f32);

    /// Return the bounce threshold velocity.
    pub fn PxScene_getBounceThresholdVelocity(self_: *const PxScene) -> f32;

    /// Sets the maximum number of CCD passes
    ///
    /// Do not use this method while the simulation is running.
    pub fn PxScene_setCCDMaxPasses_mut(self_: *mut PxScene, ccdMaxPasses: u32);

    /// Gets the maximum number of CCD passes.
    ///
    /// The maximum number of CCD passes.
    pub fn PxScene_getCCDMaxPasses(self_: *const PxScene) -> u32;

    /// Set the maximum CCD separation.
    ///
    /// Do not use this method while the simulation is running.
    pub fn PxScene_setCCDMaxSeparation_mut(self_: *mut PxScene, t: f32);

    /// Gets the maximum CCD separation.
    ///
    /// The maximum CCD separation.
    pub fn PxScene_getCCDMaxSeparation(self_: *const PxScene) -> f32;

    /// Set the CCD threshold.
    ///
    /// Do not use this method while the simulation is running.
    pub fn PxScene_setCCDThreshold_mut(self_: *mut PxScene, t: f32);

    /// Gets the CCD threshold.
    ///
    /// The CCD threshold.
    pub fn PxScene_getCCDThreshold(self_: *const PxScene) -> f32;

    /// Set the max bias coefficient.
    ///
    /// Do not use this method while the simulation is running.
    pub fn PxScene_setMaxBiasCoefficient_mut(self_: *mut PxScene, t: f32);

    /// Gets the max bias coefficient.
    ///
    /// The max bias coefficient.
    pub fn PxScene_getMaxBiasCoefficient(self_: *const PxScene) -> f32;

    /// Set the friction offset threshold.
    ///
    /// Do not use this method while the simulation is running.
    pub fn PxScene_setFrictionOffsetThreshold_mut(self_: *mut PxScene, t: f32);

    /// Gets the friction offset threshold.
    pub fn PxScene_getFrictionOffsetThreshold(self_: *const PxScene) -> f32;

    /// Set the friction correlation distance.
    ///
    /// Do not use this method while the simulation is running.
    pub fn PxScene_setFrictionCorrelationDistance_mut(self_: *mut PxScene, t: f32);

    /// Gets the friction correlation distance.
    pub fn PxScene_getFrictionCorrelationDistance(self_: *const PxScene) -> f32;

    /// Return the friction model.
    ///
    /// Since only the patch friction model is supported now, the friction type option is obsolete.
    pub fn PxScene_getFrictionType(self_: *const PxScene) -> PxFrictionType;

    /// Return the solver model.
    pub fn PxScene_getSolverType(self_: *const PxScene) -> PxSolverType;

    /// Function that lets you set debug visualization parameters.
    ///
    /// Returns false if the value passed is out of range for usage specified by the enum.
    ///
    /// Do not use this method while the simulation is running.
    ///
    /// False if the parameter is out of range.
    pub fn PxScene_setVisualizationParameter_mut(self_: *mut PxScene, param: PxVisualizationParameter, value: f32) -> bool;

    /// Function that lets you query debug visualization parameters.
    ///
    /// The value of the parameter.
    pub fn PxScene_getVisualizationParameter(self_: *const PxScene, paramEnum: PxVisualizationParameter) -> f32;

    /// Defines a box in world space to which visualization geometry will be (conservatively) culled. Use a non-empty culling box to enable the feature, and an empty culling box to disable it.
    ///
    /// Do not use this method while the simulation is running.
    pub fn PxScene_setVisualizationCullingBox_mut(self_: *mut PxScene, box_: *const PxBounds3);

    /// Retrieves the visualization culling box.
    ///
    /// the box to which the geometry will be culled.
    pub fn PxScene_getVisualizationCullingBox(self_: *const PxScene) -> PxBounds3;

    /// Retrieves the render buffer.
    ///
    /// This will contain the results of any active visualization for this scene.
    ///
    /// Do not use this method while the simulation is running. Calls to this method while the simulation is running will result in undefined behaviour.
    ///
    /// The render buffer.
    pub fn PxScene_getRenderBuffer_mut(self_: *mut PxScene) -> *const PxRenderBuffer;

    /// Call this method to retrieve statistics for the current simulation step.
    ///
    /// Do not use this method while the simulation is running. Calls to this method while the simulation is running will be ignored.
    pub fn PxScene_getSimulationStatistics(self_: *const PxScene, stats: *mut PxSimulationStatistics);

    /// Returns broad-phase type.
    ///
    /// Broad-phase type
    pub fn PxScene_getBroadPhaseType(self_: *const PxScene) -> PxBroadPhaseType;

    /// Gets broad-phase caps.
    ///
    /// True if success
    pub fn PxScene_getBroadPhaseCaps(self_: *const PxScene, caps: *mut PxBroadPhaseCaps) -> bool;

    /// Returns number of regions currently registered in the broad-phase.
    ///
    /// Number of regions
    pub fn PxScene_getNbBroadPhaseRegions(self_: *const PxScene) -> u32;

    /// Gets broad-phase regions.
    ///
    /// Number of written out regions
    pub fn PxScene_getBroadPhaseRegions(self_: *const PxScene, userBuffer: *mut PxBroadPhaseRegionInfo, bufferSize: u32, startIndex: u32) -> u32;

    /// Adds a new broad-phase region.
    ///
    /// The bounds for the new region must be non-empty, otherwise an error occurs and the call is ignored.
    ///
    /// Note that by default, objects already existing in the SDK that might touch this region will not be automatically
    /// added to the region. In other words the newly created region will be empty, and will only be populated with new
    /// objects when they are added to the simulation, or with already existing objects when they are updated.
    ///
    /// It is nonetheless possible to override this default behavior and let the SDK populate the new region automatically
    /// with already existing objects overlapping the incoming region. This has a cost though, and it should only be used
    /// when the game can not guarantee that all objects within the new region will be added to the simulation after the
    /// region itself.
    ///
    /// Objects automatically move from one region to another during their lifetime. The system keeps tracks of what
    /// regions a given object is in. It is legal for an object to be in an arbitrary number of regions. However if an
    /// object leaves all regions, or is created outside of all regions, several things happen:
    /// - collisions get disabled for this object
    /// - if a PxBroadPhaseCallback object is provided, an "out-of-bounds" event is generated via that callback
    /// - if a PxBroadPhaseCallback object is not provided, a warning/error message is sent to the error stream
    ///
    /// If an object goes out-of-bounds and user deletes it during the same frame, neither the out-of-bounds event nor the
    /// error message is generated.
    ///
    /// Handle for newly created region, or 0xffffffff in case of failure.
    pub fn PxScene_addBroadPhaseRegion_mut(self_: *mut PxScene, region: *const PxBroadPhaseRegion, populateRegion: bool) -> u32;

    /// Removes a new broad-phase region.
    ///
    /// If the region still contains objects, and if those objects do not overlap any region any more, they are not
    /// automatically removed from the simulation. Instead, the PxBroadPhaseCallback::onObjectOutOfBounds notification
    /// is used for each object. Users are responsible for removing the objects from the simulation if this is the
    /// desired behavior.
    ///
    /// If the handle is invalid, or if a valid handle is removed twice, an error message is sent to the error stream.
    ///
    /// True if success
    pub fn PxScene_removeBroadPhaseRegion_mut(self_: *mut PxScene, handle: u32) -> bool;

    /// Get the task manager associated with this scene
    ///
    /// the task manager associated with the scene
    pub fn PxScene_getTaskManager(self_: *const PxScene) -> *mut PxTaskManager;

    /// Lock the scene for reading from the calling thread.
    ///
    /// When the PxSceneFlag::eREQUIRE_RW_LOCK flag is enabled lockRead() must be
    /// called before any read calls are made on the scene.
    ///
    /// Multiple threads may read at the same time, no threads may read while a thread is writing.
    /// If a call to lockRead() is made while another thread is holding a write lock
    /// then the calling thread will be blocked until the writing thread calls unlockWrite().
    ///
    /// Lock upgrading is *not* supported, that means it is an error to
    /// call lockRead() followed by lockWrite().
    ///
    /// Recursive locking is supported but each lockRead() call must be paired with an unlockRead().
    pub fn PxScene_lockRead_mut(self_: *mut PxScene, file: *const std::ffi::c_char, line: u32);

    /// Unlock the scene from reading.
    ///
    /// Each unlockRead() must be paired with a lockRead() from the same thread.
    pub fn PxScene_unlockRead_mut(self_: *mut PxScene);

    /// Lock the scene for writing from this thread.
    ///
    /// When the PxSceneFlag::eREQUIRE_RW_LOCK flag is enabled lockWrite() must be
    /// called before any write calls are made on the scene.
    ///
    /// Only one thread may write at a time and no threads may read while a thread is writing.
    /// If a call to lockWrite() is made and there are other threads reading then the
    /// calling thread will be blocked until the readers complete.
    ///
    /// Writers have priority. If a thread is blocked waiting to write then subsequent calls to
    /// lockRead() from other threads will be blocked until the writer completes.
    ///
    /// If multiple threads are waiting to write then the thread that is first
    /// granted access depends on OS scheduling.
    ///
    /// Recursive locking is supported but each lockWrite() call must be paired
    /// with an unlockWrite().
    ///
    /// If a thread has already locked the scene for writing then it may call
    /// lockRead().
    pub fn PxScene_lockWrite_mut(self_: *mut PxScene, file: *const std::ffi::c_char, line: u32);

    /// Unlock the scene from writing.
    ///
    /// Each unlockWrite() must be paired with a lockWrite() from the same thread.
    pub fn PxScene_unlockWrite_mut(self_: *mut PxScene);

    /// set the cache blocks that can be used during simulate().
    ///
    /// Each frame the simulation requires memory to store contact, friction, and contact cache data. This memory is used in blocks of 16K.
    /// Each frame the blocks used by the previous frame are freed, and may be retrieved by the application using PxScene::flushSimulation()
    ///
    /// This call will force allocation of cache blocks if the numBlocks parameter is greater than the currently allocated number
    /// of blocks, and less than the max16KContactDataBlocks parameter specified at scene creation time.
    ///
    /// Do not use this method while the simulation is running.
    pub fn PxScene_setNbContactDataBlocks_mut(self_: *mut PxScene, numBlocks: u32);

    /// get the number of cache blocks currently used by the scene
    ///
    /// This function may not be called while the scene is simulating
    ///
    /// the number of cache blocks currently used by the scene
    pub fn PxScene_getNbContactDataBlocksUsed(self_: *const PxScene) -> u32;

    /// get the maximum number of cache blocks used by the scene
    ///
    /// This function may not be called while the scene is simulating
    ///
    /// the maximum number of cache blocks everused by the scene
    pub fn PxScene_getMaxNbContactDataBlocksUsed(self_: *const PxScene) -> u32;

    /// Return the value of PxSceneDesc::contactReportStreamBufferSize that was set when creating the scene with PxPhysics::createScene
    pub fn PxScene_getContactReportStreamBufferSize(self_: *const PxScene) -> u32;

    /// Sets the number of actors required to spawn a separate rigid body solver thread.
    ///
    /// Do not use this method while the simulation is running.
    pub fn PxScene_setSolverBatchSize_mut(self_: *mut PxScene, solverBatchSize: u32);

    /// Retrieves the number of actors required to spawn a separate rigid body solver thread.
    ///
    /// Current number of actors required to spawn a separate rigid body solver thread.
    pub fn PxScene_getSolverBatchSize(self_: *const PxScene) -> u32;

    /// Sets the number of articulations required to spawn a separate rigid body solver thread.
    ///
    /// Do not use this method while the simulation is running.
    pub fn PxScene_setSolverArticulationBatchSize_mut(self_: *mut PxScene, solverBatchSize: u32);

    /// Retrieves the number of articulations required to spawn a separate rigid body solver thread.
    ///
    /// Current number of articulations required to spawn a separate rigid body solver thread.
    pub fn PxScene_getSolverArticulationBatchSize(self_: *const PxScene) -> u32;

    /// Returns the wake counter reset value.
    ///
    /// Wake counter reset value
    pub fn PxScene_getWakeCounterResetValue(self_: *const PxScene) -> f32;

    /// Shift the scene origin by the specified vector.
    ///
    /// The poses of all objects in the scene and the corresponding data structures will get adjusted to reflect the new origin location
    /// (the shift vector will get subtracted from all object positions).
    ///
    /// It is the user's responsibility to keep track of the summed total origin shift and adjust all input/output to/from PhysX accordingly.
    ///
    /// Do not use this method while the simulation is running. Calls to this method while the simulation is running will be ignored.
    ///
    /// Make sure to propagate the origin shift to other dependent modules (for example, the character controller module etc.).
    ///
    /// This is an expensive operation and we recommend to use it only in the case where distance related precision issues may arise in areas far from the origin.
    pub fn PxScene_shiftOrigin_mut(self_: *mut PxScene, shift: *const PxVec3);

    /// Returns the Pvd client associated with the scene.
    ///
    /// the client, NULL if no PVD supported.
    pub fn PxScene_getScenePvdClient_mut(self_: *mut PxScene) -> *mut PxPvdSceneClient;

    /// Get the PxGpuDynamicsMemoryConfig that was passed into PxPhysics::createScene() as part of PxSceneDesc.
    ///
    /// This will return the values passed as initial configuration, for the actual minimal configuration that would be needed
    /// for a specific simulation of a scene, see PxSimulationStatistics::gpuDynamicsMemoryConfigStatistics.
    ///
    /// The PxGpuDynamicsMemoryConfig used during scene creation.
    pub fn PxScene_getGpuDynamicsConfig(self_: *const PxScene) -> PxGpuDynamicsMemoryConfig;

    /// Get the direct-GPU API instance for this scene.
    ///
    /// Each object of PxDirectGPUAPI is directly associated with a PxScene, and there is only one PxDirectGPUAPI object per scene.
    pub fn PxScene_getDirectGPUAPI_mut(self_: *mut PxScene) -> *mut PxDirectGPUAPI;

    /// Sets the post-solve callback for deformable surface GPU computations. Allows to schedule custom work to be done by the GPU as soon as possible after the deformable surface solver finishes.
    pub fn PxScene_setDeformableSurfaceGpuPostSolveCallback_mut(self_: *mut PxScene, postSolveCallback: *mut PxPostSolveCallback);

    /// Sets the post-solve callback for deformable volume GPU computations. Allows to schedule custom work to be done by the GPU as soon as possible after the deformable volume solver finishes.
    pub fn PxScene_setDeformableVolumeGpuPostSolveCallback_mut(self_: *mut PxScene, postSolveCallback: *mut PxPostSolveCallback);

    /// Constructor
    pub fn PxSceneReadLock_new_alloc(scene: *mut PxScene, file: *const std::ffi::c_char, line: u32) -> *mut PxSceneReadLock;

    pub fn PxSceneReadLock_delete(self_: *mut PxSceneReadLock);

    /// Constructor
    pub fn PxSceneWriteLock_new_alloc(scene: *mut PxScene, file: *const std::ffi::c_char, line: u32) -> *mut PxSceneWriteLock;

    pub fn PxSceneWriteLock_delete(self_: *mut PxSceneWriteLock);

    pub fn PxContactPairExtraDataItem_new() -> PxContactPairExtraDataItem;

    pub fn PxContactPairVelocity_new() -> PxContactPairVelocity;

    pub fn PxContactPairPose_new() -> PxContactPairPose;

    pub fn PxContactPairIndex_new() -> PxContactPairIndex;

    /// Constructor
    pub fn PxContactPairExtraDataIterator_new(stream: *const u8, size: u32) -> PxContactPairExtraDataIterator;

    /// Advances the iterator to next set of extra data items.
    ///
    /// The contact pair extra data stream contains sets of items as requested by the corresponding [`PxPairFlag`] flags
    /// [`PxPairFlag::ePRE_SOLVER_VELOCITY`], #PxPairFlag::ePOST_SOLVER_VELOCITY, #PxPairFlag::eCONTACT_EVENT_POSE. A set can contain one
    /// item of each plus the PxContactPairIndex item. This method parses the stream and points the iterator
    /// member variables to the corresponding items of the current set, if they are available. If CCD is not enabled,
    /// you should only get one set of items. If CCD with multiple passes is enabled, you might get more than one item
    /// set.
    ///
    /// Even though contact pair extra data is requested per shape pair, you will not get an item set per shape pair
    /// but one per actor pair. If, for example, an actor has two shapes and both collide with another actor, then
    /// there will only be one item set (since it applies to both shape pairs).
    ///
    /// True if there was another set of extra data items in the stream, else false.
    pub fn PxContactPairExtraDataIterator_nextItemSet_mut(self_: *mut PxContactPairExtraDataIterator) -> bool;

    pub fn PxContactPairHeader_new() -> PxContactPairHeader;

    pub fn PxContactPair_new() -> PxContactPair;

    /// Extracts the contact points from the stream and stores them in a convenient format.
    ///
    /// Number of contact points written to the buffer.
    pub fn PxContactPair_extractContacts(self_: *const PxContactPair, userBuffer: *mut PxContactPairPoint, bufferSize: u32) -> u32;

    /// Extracts the friction anchors from the stream and stores them in a convenient format.
    ///
    /// Number of friction anchors written to the buffer.
    pub fn PxContactPair_extractFrictionAnchors(self_: *const PxContactPair, userBuffer: *mut PxContactPairFrictionAnchor, bufferSize: u32) -> u32;

    /// Helper method to clone the contact pair and copy the contact data stream into a user buffer.
    ///
    /// The contact data stream is only accessible during the contact report callback. This helper function provides copy functionality
    /// to buffer the contact stream information such that it can get accessed at a later stage.
    pub fn PxContactPair_bufferContacts(self_: *const PxContactPair, newPair: *mut PxContactPair, bufferMemory: *mut u8);

    pub fn PxContactPair_getInternalFaceIndices(self_: *const PxContactPair) -> *const u32;

    pub fn PxTriggerPair_new() -> PxTriggerPair;

    pub fn PxConstraintInfo_new() -> PxConstraintInfo;

    pub fn PxConstraintInfo_new_1(c: *mut PxConstraint, extRef: *mut std::ffi::c_void, t: u32) -> PxConstraintInfo;

    /// This is called when a breakable constraint breaks.
    ///
    /// The user should not release the constraint shader inside this call!
    ///
    /// No event will get reported if the constraint breaks but gets deleted while the time step is still being simulated.
    pub fn PxSimulationEventCallback_onConstraintBreak_mut(self_: *mut PxSimulationEventCallback, constraints: *mut PxConstraintInfo, count: u32);

    /// This is called with the actors which have just been woken up.
    ///
    /// Only supported by rigid bodies yet.
    ///
    /// Only called on actors for which the PxActorFlag eSEND_SLEEP_NOTIFIES has been set.
    ///
    /// Only the latest sleep state transition happening between fetchResults() of the previous frame and fetchResults() of the current frame
    /// will get reported. For example, let us assume actor A is awake, then A->putToSleep() gets called, then later A->wakeUp() gets called.
    /// At the next simulate/fetchResults() step only an onWake() event will get triggered because that was the last transition.
    ///
    /// If an actor gets newly added to a scene with properties such that it is awake and the sleep state does not get changed by
    /// the user or simulation, then an onWake() event will get sent at the next simulate/fetchResults() step.
    pub fn PxSimulationEventCallback_onWake_mut(self_: *mut PxSimulationEventCallback, actors: *mut *mut PxActor, count: u32);

    /// This is called with the actors which have just been put to sleep.
    ///
    /// Only supported by rigid bodies yet.
    ///
    /// Only called on actors for which the PxActorFlag eSEND_SLEEP_NOTIFIES has been set.
    ///
    /// Only the latest sleep state transition happening between fetchResults() of the previous frame and fetchResults() of the current frame
    /// will get reported. For example, let us assume actor A is asleep, then A->wakeUp() gets called, then later A->putToSleep() gets called.
    /// At the next simulate/fetchResults() step only an onSleep() event will get triggered because that was the last transition (assuming the simulation
    /// does not wake the actor up).
    ///
    /// If an actor gets newly added to a scene with properties such that it is asleep and the sleep state does not get changed by
    /// the user or simulation, then an onSleep() event will get sent at the next simulate/fetchResults() step.
    pub fn PxSimulationEventCallback_onSleep_mut(self_: *mut PxSimulationEventCallback, actors: *mut *mut PxActor, count: u32);

    /// This is called when certain contact events occur.
    ///
    /// The method will be called for a pair of actors if one of the colliding shape pairs requested contact notification.
    /// You request which events are reported using the filter shader/callback mechanism (see [`PxSimulationFilterShader`],
    /// [`PxSimulationFilterCallback`], #PxPairFlag).
    ///
    /// Do not keep references to the passed objects, as they will be
    /// invalid after this function returns.
    pub fn PxSimulationEventCallback_onContact_mut(self_: *mut PxSimulationEventCallback, pairHeader: *const PxContactPairHeader, pairs: *const PxContactPair, nbPairs: u32);

    /// This is called with the current trigger pair events.
    ///
    /// Shapes which have been marked as triggers using PxShapeFlag::eTRIGGER_SHAPE will send events
    /// according to the pair flag specification in the filter shader (see [`PxPairFlag`], #PxSimulationFilterShader).
    ///
    /// Trigger shapes will no longer send notification events for interactions with other trigger shapes.
    pub fn PxSimulationEventCallback_onTrigger_mut(self_: *mut PxSimulationEventCallback, pairs: *mut PxTriggerPair, count: u32);

    /// Provides early access to the new pose of moving rigid bodies.
    ///
    /// When this call occurs, rigid bodies having the [`PxRigidBodyFlag::eENABLE_POSE_INTEGRATION_PREVIEW`]
    /// flag set, were moved by the simulation and their new poses can be accessed through the provided buffers.
    ///
    /// The provided buffers are valid and can be read until the next call to [`PxScene::simulate`]() or #PxScene::collide().
    ///
    /// This callback gets triggered while the simulation is running. If the provided rigid body references are used to
    /// read properties of the object, then the callback has to guarantee no other thread is writing to the same body at the same
    /// time.
    ///
    /// The code in this callback should be lightweight as it can block the simulation, that is, the
    /// [`PxScene::fetchResults`]() call.
    pub fn PxSimulationEventCallback_onAdvance_mut(self_: *mut PxSimulationEventCallback, bodyBuffer: *const *const PxRigidBody, poseBuffer: *const PxTransform, count: u32);

    pub fn PxSimulationEventCallback_delete(self_: *mut PxSimulationEventCallback);

    /// Release this object.
    pub fn PxPruningStructure_release_mut(self_: *mut PxPruningStructure);

    /// Retrieve rigid actors in the pruning structure.
    ///
    /// You can retrieve the number of rigid actor pointers by calling [`getNbRigidActors`]()
    ///
    /// Number of rigid actor pointers written to the buffer.
    pub fn PxPruningStructure_getRigidActors(self_: *const PxPruningStructure, userBuffer: *mut *mut PxRigidActor, bufferSize: u32, startIndex: u32) -> u32;

    /// Returns the number of rigid actors in the pruning structure.
    ///
    /// You can use [`getRigidActors`]() to retrieve the rigid actor pointers.
    ///
    /// Number of rigid actors in the pruning structure.
    pub fn PxPruningStructure_getNbRigidActors(self_: *const PxPruningStructure) -> u32;

    /// Gets the merge data for static actors
    ///
    /// This is mainly called by the PxSceneQuerySystem::merge() function to merge a PxPruningStructure
    /// with the internal data-structures of the scene-query system.
    ///
    /// Implementation-dependent merge data for static actors.
    pub fn PxPruningStructure_getStaticMergeData(self_: *const PxPruningStructure) -> *const std::ffi::c_void;

    /// Gets the merge data for dynamic actors
    ///
    /// This is mainly called by the PxSceneQuerySystem::merge() function to merge a PxPruningStructure
    /// with the internal data-structures of the scene-query system.
    ///
    /// Implementation-dependent merge data for dynamic actors.
    pub fn PxPruningStructure_getDynamicMergeData(self_: *const PxPruningStructure) -> *const std::ffi::c_void;

    pub fn PxPruningStructure_getConcreteTypeName(self_: *const PxPruningStructure) -> *const std::ffi::c_char;

    pub fn phys_toVec3(v: *const PxExtendedVec3) -> PxVec3;

    pub fn phys_diff(p1: *const PxExtendedVec3, p0: *const PxExtendedVec3) -> PxVec3;

    pub fn PxObstacle_getType(self_: *const PxObstacle) -> PxGeometryType;

    pub fn PxBoxObstacle_new() -> PxBoxObstacle;

    pub fn PxCapsuleObstacle_new() -> PxCapsuleObstacle;

    /// Releases the context.
    pub fn PxObstacleContext_release_mut(self_: *mut PxObstacleContext);

    /// Retrieves the controller manager associated with this context.
    ///
    /// The associated controller manager
    pub fn PxObstacleContext_getControllerManager(self_: *const PxObstacleContext) -> *mut PxControllerManager;

    /// Adds an obstacle to the context.
    ///
    /// Handle for newly-added obstacle
    pub fn PxObstacleContext_addObstacle_mut(self_: *mut PxObstacleContext, obstacle: *const PxObstacle) -> u32;

    /// Removes an obstacle from the context.
    ///
    /// True if success
    pub fn PxObstacleContext_removeObstacle_mut(self_: *mut PxObstacleContext, handle: u32) -> bool;

    /// Updates data for an existing obstacle.
    ///
    /// True if success
    pub fn PxObstacleContext_updateObstacle_mut(self_: *mut PxObstacleContext, handle: u32, obstacle: *const PxObstacle) -> bool;

    /// Retrieves number of obstacles in the context.
    ///
    /// Number of obstacles in the context
    pub fn PxObstacleContext_getNbObstacles(self_: *const PxObstacleContext) -> u32;

    /// Retrieves desired obstacle.
    ///
    /// Desired obstacle
    pub fn PxObstacleContext_getObstacle(self_: *const PxObstacleContext, i: u32) -> *const PxObstacle;

    /// Retrieves desired obstacle by given handle.
    ///
    /// Desired obstacle
    pub fn PxObstacleContext_getObstacleByHandle(self_: *const PxObstacleContext, handle: u32) -> *const PxObstacle;

    /// Called when current controller hits a shape.
    ///
    /// This is called when the CCT moves and hits a shape. This will not be called when a moving shape hits a non-moving CCT.
    pub fn PxUserControllerHitReport_onShapeHit_mut(self_: *mut PxUserControllerHitReport, hit: *const PxControllerShapeHit);

    /// Called when current controller hits another controller.
    pub fn PxUserControllerHitReport_onControllerHit_mut(self_: *mut PxUserControllerHitReport, hit: *const PxControllersHit);

    /// Called when current controller hits a user-defined obstacle.
    pub fn PxUserControllerHitReport_onObstacleHit_mut(self_: *mut PxUserControllerHitReport, hit: *const PxControllerObstacleHit);

    pub fn PxControllerFilterCallback_delete(self_: *mut PxControllerFilterCallback);

    /// Filtering method for CCT-vs-CCT.
    ///
    /// true to keep the pair, false to filter it out
    pub fn PxControllerFilterCallback_filter_mut(self_: *mut PxControllerFilterCallback, a: *const PxController, b: *const PxController) -> bool;

    pub fn PxControllerFilters_new(filterData: *const PxFilterData, cb: *mut PxQueryFilterCallback, cctFilterCb: *mut PxControllerFilterCallback) -> PxControllerFilters;

    /// returns true if the current settings are valid
    ///
    /// True if the descriptor is valid.
    pub fn PxControllerDesc_isValid(self_: *const PxControllerDesc) -> bool;

    /// Returns the character controller type
    ///
    /// The controllers type.
    pub fn PxControllerDesc_getType(self_: *const PxControllerDesc) -> PxControllerShapeType;

    /// Return the type of controller
    pub fn PxController_getType(self_: *const PxController) -> PxControllerShapeType;

    /// Releases the controller.
    pub fn PxController_release_mut(self_: *mut PxController);

    /// Moves the character using a "collide-and-slide" algorithm.
    ///
    /// Collision flags, collection of ::PxControllerCollisionFlags
    pub fn PxController_move_mut(self_: *mut PxController, disp: *const PxVec3, minDist: f32, elapsedTime: f32, filters: *const PxControllerFilters, obstacles: *const PxObstacleContext) -> PxControllerCollisionFlags;

    /// Sets controller's position.
    ///
    /// The position controlled by this function is the center of the collision shape.
    ///
    /// This is a 'teleport' function, it doesn't check for collisions.
    ///
    /// The character's position must be such that it does not overlap the static geometry.
    ///
    /// To move the character under normal conditions use the [`move`]() function.
    ///
    /// Currently always returns true.
    pub fn PxController_setPosition_mut(self_: *mut PxController, position: *const PxExtendedVec3) -> bool;

    /// Retrieve the raw position of the controller.
    ///
    /// The position retrieved by this function is the center of the collision shape. To retrieve the bottom position of the shape,
    /// a.k.a. the foot position, use the getFootPosition() function.
    ///
    /// The position is updated by calls to move(). Calling this method without calling
    /// move() will return the last position or the initial position of the controller.
    ///
    /// The controller's center position
    pub fn PxController_getPosition(self_: *const PxController) -> *const PxExtendedVec3;

    /// Set controller's foot position.
    ///
    /// The position controlled by this function is the bottom of the collision shape, a.k.a. the foot position.
    ///
    /// The foot position takes the contact offset into account
    ///
    /// This is a 'teleport' function, it doesn't check for collisions.
    ///
    /// To move the character under normal conditions use the [`move`]() function.
    ///
    /// Currently always returns true.
    pub fn PxController_setFootPosition_mut(self_: *mut PxController, position: *const PxExtendedVec3) -> bool;

    /// Retrieve the "foot" position of the controller, i.e. the position of the bottom of the CCT's shape.
    ///
    /// The foot position takes the contact offset into account
    ///
    /// The controller's foot position
    pub fn PxController_getFootPosition(self_: *const PxController) -> PxExtendedVec3;

    /// Get the rigid body actor associated with this controller (see PhysX documentation).
    /// The behavior upon manually altering this actor is undefined, you should primarily
    /// use it for reading const properties.
    ///
    /// the actor associated with the controller.
    pub fn PxController_getActor(self_: *const PxController) -> *mut PxRigidDynamic;

    /// The step height.
    pub fn PxController_setStepOffset_mut(self_: *mut PxController, offset: f32);

    /// Retrieve the step height.
    ///
    /// The step offset for the controller.
    pub fn PxController_getStepOffset(self_: *const PxController) -> f32;

    /// Sets the non-walkable mode for the CCT.
    pub fn PxController_setNonWalkableMode_mut(self_: *mut PxController, flag: PxControllerNonWalkableMode);

    /// Retrieves the non-walkable mode for the CCT.
    ///
    /// The current non-walkable mode.
    pub fn PxController_getNonWalkableMode(self_: *const PxController) -> PxControllerNonWalkableMode;

    /// Retrieve the contact offset.
    ///
    /// The contact offset for the controller.
    pub fn PxController_getContactOffset(self_: *const PxController) -> f32;

    /// Sets the contact offset.
    pub fn PxController_setContactOffset_mut(self_: *mut PxController, offset: f32);

    /// Retrieve the 'up' direction.
    ///
    /// The up direction for the controller.
    pub fn PxController_getUpDirection(self_: *const PxController) -> PxVec3;

    /// Sets the 'up' direction.
    pub fn PxController_setUpDirection_mut(self_: *mut PxController, up: *const PxVec3);

    /// Retrieve the slope limit.
    ///
    /// The slope limit for the controller.
    pub fn PxController_getSlopeLimit(self_: *const PxController) -> f32;

    /// Sets the slope limit.
    ///
    /// This feature can not be enabled at runtime, i.e. if the slope limit is zero when creating the CCT
    /// (which disables the feature) then changing the slope limit at runtime will not have any effect, and the call
    /// will be ignored.
    pub fn PxController_setSlopeLimit_mut(self_: *mut PxController, slopeLimit: f32);

    /// Flushes internal geometry cache.
    ///
    /// The character controller uses caching in order to speed up collision testing. The cache is
    /// automatically flushed when a change to static objects is detected in the scene. For example when a
    /// static shape is added, updated, or removed from the scene, the cache is automatically invalidated.
    ///
    /// However there may be situations that cannot be automatically detected, and those require manual
    /// invalidation of the cache. Currently the user must call this when the filtering behavior changes (the
    /// PxControllerFilters parameter of the PxController::move call).  While the controller in principle
    /// could detect a change in these parameters, it cannot detect a change in the behavior of the filtering
    /// function.
    pub fn PxController_invalidateCache_mut(self_: *mut PxController);

    /// Retrieve the scene associated with the controller.
    ///
    /// The physics scene
    pub fn PxController_getScene_mut(self_: *mut PxController) -> *mut PxScene;

    /// Returns the user data associated with this controller.
    ///
    /// The user pointer associated with the controller.
    pub fn PxController_getUserData(self_: *const PxController) -> *mut std::ffi::c_void;

    /// Sets the user data associated with this controller.
    pub fn PxController_setUserData_mut(self_: *mut PxController, userData: *mut std::ffi::c_void);

    /// Returns information about the controller's internal state.
    pub fn PxController_getState(self_: *const PxController, state: *mut PxControllerState);

    /// Returns the controller's internal statistics.
    pub fn PxController_getStats(self_: *const PxController, stats: *mut PxControllerStats);

    /// Resizes the controller.
    ///
    /// This function attempts to resize the controller to a given size, while making sure the bottom
    /// position of the controller remains constant. In other words the function modifies both the
    /// height and the (center) position of the controller. This is a helper function that can be used
    /// to implement a 'crouch' functionality for example.
    pub fn PxController_resize_mut(self_: *mut PxController, height: f32);

    /// constructor sets to default.
    pub fn PxBoxControllerDesc_new_alloc() -> *mut PxBoxControllerDesc;

    pub fn PxBoxControllerDesc_delete(self_: *mut PxBoxControllerDesc);

    /// (re)sets the structure to the default.
    pub fn PxBoxControllerDesc_setToDefault_mut(self_: *mut PxBoxControllerDesc);

    /// returns true if the current settings are valid
    ///
    /// True if the descriptor is valid.
    pub fn PxBoxControllerDesc_isValid(self_: *const PxBoxControllerDesc) -> bool;

    /// Gets controller's half height.
    ///
    /// The half height of the controller.
    pub fn PxBoxController_getHalfHeight(self_: *const PxBoxController) -> f32;

    /// Gets controller's half side extent.
    ///
    /// The half side extent of the controller.
    pub fn PxBoxController_getHalfSideExtent(self_: *const PxBoxController) -> f32;

    /// Gets controller's half forward extent.
    ///
    /// The half forward extent of the controller.
    pub fn PxBoxController_getHalfForwardExtent(self_: *const PxBoxController) -> f32;

    /// Sets controller's half height.
    ///
    /// this doesn't check for collisions.
    ///
    /// Currently always true.
    pub fn PxBoxController_setHalfHeight_mut(self_: *mut PxBoxController, halfHeight: f32) -> bool;

    /// Sets controller's half side extent.
    ///
    /// this doesn't check for collisions.
    ///
    /// Currently always true.
    pub fn PxBoxController_setHalfSideExtent_mut(self_: *mut PxBoxController, halfSideExtent: f32) -> bool;

    /// Sets controller's half forward extent.
    ///
    /// this doesn't check for collisions.
    ///
    /// Currently always true.
    pub fn PxBoxController_setHalfForwardExtent_mut(self_: *mut PxBoxController, halfForwardExtent: f32) -> bool;

    /// constructor sets to default.
    pub fn PxCapsuleControllerDesc_new_alloc() -> *mut PxCapsuleControllerDesc;

    pub fn PxCapsuleControllerDesc_delete(self_: *mut PxCapsuleControllerDesc);

    /// (re)sets the structure to the default.
    pub fn PxCapsuleControllerDesc_setToDefault_mut(self_: *mut PxCapsuleControllerDesc);

    /// returns true if the current settings are valid
    ///
    /// True if the descriptor is valid.
    pub fn PxCapsuleControllerDesc_isValid(self_: *const PxCapsuleControllerDesc) -> bool;

    /// Gets controller's radius.
    ///
    /// The radius of the controller.
    pub fn PxCapsuleController_getRadius(self_: *const PxCapsuleController) -> f32;

    /// Sets controller's radius.
    ///
    /// this doesn't check for collisions.
    ///
    /// Currently always true.
    pub fn PxCapsuleController_setRadius_mut(self_: *mut PxCapsuleController, radius: f32) -> bool;

    /// Gets controller's height.
    ///
    /// The height of the capsule controller.
    pub fn PxCapsuleController_getHeight(self_: *const PxCapsuleController) -> f32;

    /// Resets controller's height.
    ///
    /// this doesn't check for collisions.
    ///
    /// Currently always true.
    pub fn PxCapsuleController_setHeight_mut(self_: *mut PxCapsuleController, height: f32) -> bool;

    /// Gets controller's climbing mode.
    ///
    /// The capsule controller's climbing mode.
    pub fn PxCapsuleController_getClimbingMode(self_: *const PxCapsuleController) -> PxCapsuleClimbingMode;

    /// Sets controller's climbing mode.
    pub fn PxCapsuleController_setClimbingMode_mut(self_: *mut PxCapsuleController, mode: PxCapsuleClimbingMode) -> bool;

    /// Retrieve behavior flags for a shape.
    ///
    /// When the CCT touches a shape, the CCT's behavior w.r.t. this shape can be customized by users.
    /// This function retrieves the desired PxControllerBehaviorFlag flags capturing the desired behavior.
    ///
    /// Desired behavior flags for the given shape
    pub fn PxControllerBehaviorCallback_getBehaviorFlags_mut(self_: *mut PxControllerBehaviorCallback, shape: *const PxShape, actor: *const PxActor) -> PxControllerBehaviorFlags;

    /// Retrieve behavior flags for a controller.
    ///
    /// When the CCT touches a controller, the CCT's behavior w.r.t. this controller can be customized by users.
    /// This function retrieves the desired PxControllerBehaviorFlag flags capturing the desired behavior.
    ///
    /// The flag PxControllerBehaviorFlag::eCCT_CAN_RIDE_ON_OBJECT is not supported.
    ///
    /// Desired behavior flags for the given controller
    pub fn PxControllerBehaviorCallback_getBehaviorFlags_mut_1(self_: *mut PxControllerBehaviorCallback, controller: *const PxController) -> PxControllerBehaviorFlags;

    /// Retrieve behavior flags for an obstacle.
    ///
    /// When the CCT touches an obstacle, the CCT's behavior w.r.t. this obstacle can be customized by users.
    /// This function retrieves the desired PxControllerBehaviorFlag flags capturing the desired behavior.
    ///
    /// Desired behavior flags for the given obstacle
    pub fn PxControllerBehaviorCallback_getBehaviorFlags_mut_2(self_: *mut PxControllerBehaviorCallback, obstacle: *const PxObstacle) -> PxControllerBehaviorFlags;

    /// Releases the controller manager.
    ///
    /// This will release all associated controllers and obstacle contexts.
    ///
    /// This function is required to be called to release foundation usage.
    pub fn PxControllerManager_release_mut(self_: *mut PxControllerManager);

    /// Returns the scene the manager is adding the controllers to.
    ///
    /// The associated physics scene.
    pub fn PxControllerManager_getScene(self_: *const PxControllerManager) -> *mut PxScene;

    /// Returns the number of controllers that are being managed.
    ///
    /// The number of controllers.
    pub fn PxControllerManager_getNbControllers(self_: *const PxControllerManager) -> u32;

    /// Retrieve one of the controllers in the manager.
    ///
    /// The controller with the specified index.
    pub fn PxControllerManager_getController_mut(self_: *mut PxControllerManager, index: u32) -> *mut PxController;

    /// Creates a new character controller.
    ///
    /// The new controller
    pub fn PxControllerManager_createController_mut(self_: *mut PxControllerManager, desc: *const PxControllerDesc) -> *mut PxController;

    /// Releases all the controllers that are being managed.
    pub fn PxControllerManager_purgeControllers_mut(self_: *mut PxControllerManager);

    /// Retrieves debug data.
    ///
    /// The render buffer filled with debug-render data
    pub fn PxControllerManager_getRenderBuffer_mut(self_: *mut PxControllerManager) -> *mut PxRenderBuffer;

    /// Sets debug rendering flags
    pub fn PxControllerManager_setDebugRenderingFlags_mut(self_: *mut PxControllerManager, flags: PxControllerDebugRenderFlags);

    /// Returns the number of obstacle contexts that are being managed.
    ///
    /// The number of obstacle contexts.
    pub fn PxControllerManager_getNbObstacleContexts(self_: *const PxControllerManager) -> u32;

    /// Retrieve one of the obstacle contexts in the manager.
    ///
    /// The obstacle context with the specified index.
    pub fn PxControllerManager_getObstacleContext_mut(self_: *mut PxControllerManager, index: u32) -> *mut PxObstacleContext;

    /// Creates an obstacle context.
    ///
    /// New obstacle context
    pub fn PxControllerManager_createObstacleContext_mut(self_: *mut PxControllerManager) -> *mut PxObstacleContext;

    /// Computes character-character interactions.
    ///
    /// This function is an optional helper to properly resolve interactions between characters, in case they overlap (which can happen for gameplay reasons, etc).
    ///
    /// You should call this once per frame, before your PxController::move() calls. The function will not move the characters directly, but it will
    /// compute overlap information for each character that will be used in the next move() call.
    ///
    /// You need to provide a proper time value here so that interactions are resolved in a way that do not depend on the framerate.
    ///
    /// If you only have one character in the scene, or if you can guarantee your characters will never overlap, then you do not need to call this function.
    ///
    /// Releasing the manager will automatically release all the associated obstacle contexts.
    pub fn PxControllerManager_computeInteractions_mut(self_: *mut PxControllerManager, elapsedTime: f32, cctFilterCb: *mut PxControllerFilterCallback);

    /// Enables or disables runtime tessellation.
    ///
    /// Large triangles can create accuracy issues in the sweep code, which in turn can lead to characters not sliding smoothly
    /// against geometries, or even penetrating them. This feature allows one to reduce those issues by tessellating large
    /// triangles at runtime, before performing sweeps against them. The amount of tessellation is controlled by the 'maxEdgeLength' parameter.
    /// Any triangle with at least one edge length greater than the maxEdgeLength will get recursively tessellated, until resulting triangles are small enough.
    ///
    /// This features only applies to triangle meshes, convex meshes, heightfields and boxes.
    pub fn PxControllerManager_setTessellation_mut(self_: *mut PxControllerManager, flag: bool, maxEdgeLength: f32);

    /// Enables or disables the overlap recovery module.
    ///
    /// The overlap recovery module can be used to depenetrate CCTs from static objects when an overlap is detected. This can happen
    /// in three main cases:
    /// - when the CCT is directly spawned or teleported in another object
    /// - when the CCT algorithm fails due to limited FPU accuracy
    /// - when the "up vector" is modified, making the rotated CCT shape overlap surrounding objects
    ///
    /// When activated, the CCT module will automatically try to resolve the penetration, and move the CCT to a safe place where it does
    /// not overlap other objects anymore. This only concerns static objects, dynamic objects are ignored by the recovery module.
    ///
    /// When the recovery module is not activated, it is possible for the CCTs to go through static objects. By default, the recovery
    /// module is enabled.
    ///
    /// The recovery module currently works with all geometries except heightfields.
    pub fn PxControllerManager_setOverlapRecoveryModule_mut(self_: *mut PxControllerManager, flag: bool);

    /// Enables or disables the precise sweeps.
    ///
    /// Precise sweeps are more accurate, but also potentially slower than regular sweeps.
    ///
    /// By default, precise sweeps are enabled.
    pub fn PxControllerManager_setPreciseSweeps_mut(self_: *mut PxControllerManager, flag: bool);

    /// Enables or disables vertical sliding against ceilings.
    ///
    /// Geometry is seen as "ceilings" when the following condition is met:
    ///
    /// dot product(contact normal, up direction)
    /// <
    /// 0.0f
    ///
    /// This flag controls whether characters should slide vertically along the geometry in that case.
    ///
    /// By default, sliding is allowed.
    pub fn PxControllerManager_setPreventVerticalSlidingAgainstCeiling_mut(self_: *mut PxControllerManager, flag: bool);

    /// Shift the origin of the character controllers and obstacle objects by the specified vector.
    ///
    /// The positions of all character controllers, obstacle objects and the corresponding data structures will get adjusted to reflect the shifted origin location
    /// (the shift vector will get subtracted from all character controller and obstacle object positions).
    ///
    /// It is the user's responsibility to keep track of the summed total origin shift and adjust all input/output to/from PhysXCharacterKinematic accordingly.
    ///
    /// This call will not automatically shift the PhysX scene and its objects. You need to call PxScene::shiftOrigin() separately to keep the systems in sync.
    pub fn PxControllerManager_shiftOrigin_mut(self_: *mut PxControllerManager, shift: *const PxVec3);

    /// Creates the controller manager.
    ///
    /// New controller manager, or NULL in case of failure (e.g. when a manager has already been created for that scene)
    ///
    /// The character controller is informed by [`PxDeletionListener::onRelease`]() when actors or shapes are released, and updates its internal
    /// caches accordingly. If character controller movement or a call to [`PxControllerManager::shiftOrigin`]() may overlap with actor/shape releases,
    /// internal data structures must be guarded against concurrent access.
    ///
    /// Locking guarantees thread safety in such scenarios.
    ///
    /// locking may result in significant slowdown for release of actors or shapes.
    ///
    /// By default, locking is disabled.
    pub fn phys_PxCreateControllerManager(scene: *mut PxScene, lockingEnabled: bool) -> *mut PxControllerManager;

    pub fn PxDim3_new() -> PxDim3;

    /// Constructor
    pub fn PxSDFDesc_new() -> PxSDFDesc;

    /// Returns true if the descriptor is valid.
    ///
    /// true if the current settings are valid
    pub fn PxSDFDesc_isValid(self_: *const PxSDFDesc) -> bool;

    /// constructor sets to default.
    pub fn PxConvexMeshDesc_new() -> PxConvexMeshDesc;

    /// (re)sets the structure to the default.
    pub fn PxConvexMeshDesc_setToDefault_mut(self_: *mut PxConvexMeshDesc);

    /// Returns true if the descriptor is valid.
    ///
    /// True if the current settings are valid
    pub fn PxConvexMeshDesc_isValid(self_: *const PxConvexMeshDesc) -> bool;

    /// Constructor sets to default.
    pub fn PxTriangleMeshDesc_new() -> PxTriangleMeshDesc;

    /// (re)sets the structure to the default.
    pub fn PxTriangleMeshDesc_setToDefault_mut(self_: *mut PxTriangleMeshDesc);

    /// Returns true if the descriptor is valid.
    ///
    /// true if the current settings are valid
    pub fn PxTriangleMeshDesc_isValid(self_: *const PxTriangleMeshDesc) -> bool;

    /// Constructor to build an empty tetmesh description
    pub fn PxTetrahedronMeshDesc_new() -> PxTetrahedronMeshDesc;

    pub fn PxTetrahedronMeshDesc_isValid(self_: *const PxTetrahedronMeshDesc) -> bool;

    /// Constructor to build an empty simulation description
    pub fn PxDeformableVolumeSimulationDataDesc_new() -> PxDeformableVolumeSimulationDataDesc;

    pub fn PxDeformableVolumeSimulationDataDesc_isValid(self_: *const PxDeformableVolumeSimulationDataDesc) -> bool;

    /// Desc initialization to default value.
    pub fn PxBVH33MidphaseDesc_setToDefault_mut(self_: *mut PxBVH33MidphaseDesc);

    /// Returns true if the descriptor is valid.
    ///
    /// true if the current settings are valid.
    pub fn PxBVH33MidphaseDesc_isValid(self_: *const PxBVH33MidphaseDesc) -> bool;

    /// Desc initialization to default value.
    pub fn PxBVH34MidphaseDesc_setToDefault_mut(self_: *mut PxBVH34MidphaseDesc);

    /// Returns true if the descriptor is valid.
    ///
    /// true if the current settings are valid.
    pub fn PxBVH34MidphaseDesc_isValid(self_: *const PxBVH34MidphaseDesc) -> bool;

    pub fn PxMidphaseDesc_new() -> PxMidphaseDesc;

    /// Returns type of midphase mesh structure.
    ///
    /// PxMeshMidPhase::Enum
    pub fn PxMidphaseDesc_getType(self_: *const PxMidphaseDesc) -> PxMeshMidPhase;

    /// Initialize the midphase mesh structure descriptor
    pub fn PxMidphaseDesc_setToDefault_mut(self_: *mut PxMidphaseDesc, type_: PxMeshMidPhase);

    /// Returns true if the descriptor is valid.
    ///
    /// true if the current settings are valid.
    pub fn PxMidphaseDesc_isValid(self_: *const PxMidphaseDesc) -> bool;

    pub fn PxBVHDesc_new() -> PxBVHDesc;

    /// Initialize the BVH descriptor
    pub fn PxBVHDesc_setToDefault_mut(self_: *mut PxBVHDesc);

    /// Returns true if the descriptor is valid.
    ///
    /// true if the current settings are valid.
    pub fn PxBVHDesc_isValid(self_: *const PxBVHDesc) -> bool;

    pub fn PxCookingParams_new(sc: *const PxTolerancesScale) -> PxCookingParams;

    /// Gets standalone object insertion interface.
    ///
    /// This interface allows the creation of standalone objects that can exist without a PxPhysics or PxScene object.
    pub fn phys_PxGetStandaloneInsertionCallback() -> *mut PxInsertionCallback;

    /// Cooks a bounding volume hierarchy. The results are written to the stream.
    ///
    /// PxCookBVH() allows a BVH description to be cooked into a binary stream
    /// suitable for loading and performing BVH detection at runtime.
    ///
    /// true on success.
    pub fn phys_PxCookBVH(desc: *const PxBVHDesc, stream: *mut PxOutputStream) -> bool;

    /// Cooks and creates a bounding volume hierarchy without going through a stream.
    ///
    /// This method does the same as PxCookBVH, but the produced BVH is not stored
    /// into a stream but is either directly inserted in PxPhysics, or created as a standalone
    /// object. Use this method if you are unable to cook offline.
    ///
    /// PxInsertionCallback can be obtained through PxPhysics::getPhysicsInsertionCallback()
    /// or PxGetStandaloneInsertionCallback().
    ///
    /// PxBVH pointer on success
    pub fn phys_PxCreateBVH(desc: *const PxBVHDesc, insertionCallback: *mut PxInsertionCallback) -> *mut PxBVH;

    /// Cooks a heightfield. The results are written to the stream.
    ///
    /// To create a heightfield object there is an option to precompute some of calculations done while loading the heightfield data.
    ///
    /// PxCookHeightField() allows a heightfield description to be cooked into a binary stream
    /// suitable for loading and performing collision detection at runtime.
    ///
    /// true on success
    pub fn phys_PxCookHeightField(desc: *const PxHeightFieldDesc, stream: *mut PxOutputStream) -> bool;

    /// Cooks and creates a heightfield mesh and inserts it into PxPhysics.
    ///
    /// PxHeightField pointer on success
    pub fn phys_PxCreateHeightField(desc: *const PxHeightFieldDesc, insertionCallback: *mut PxInsertionCallback) -> *mut PxHeightField;

    /// Cooks a convex mesh. The results are written to the stream.
    ///
    /// To create a triangle mesh object it is necessary to first 'cook' the mesh data into
    /// a form which allows the SDK to perform efficient collision detection.
    ///
    /// PxCookConvexMesh() allows a mesh description to be cooked into a binary stream
    /// suitable for loading and performing collision detection at runtime.
    ///
    /// The number of vertices and the number of convex polygons in a cooked convex mesh is limited to 255.
    ///
    /// If those limits are exceeded in either the user-provided data or the final cooked mesh, an error is reported.
    ///
    /// true on success.
    pub fn phys_PxCookConvexMesh(params: *const PxCookingParams, desc: *const PxConvexMeshDesc, stream: *mut PxOutputStream, condition: *mut PxConvexMeshCookingResult) -> bool;

    /// Cooks and creates a convex mesh without going through a stream.
    ///
    /// This method does the same as PxCookConvexMesh, but the produced mesh is not stored
    /// into a stream but is either directly inserted in PxPhysics, or created as a standalone
    /// object. Use this method if you are unable to cook offline.
    ///
    /// PxInsertionCallback can be obtained through PxPhysics::getPhysicsInsertionCallback()
    /// or PxGetStandaloneInsertionCallback().
    ///
    /// PxConvexMesh pointer on success
    pub fn phys_PxCreateConvexMesh(params: *const PxCookingParams, desc: *const PxConvexMeshDesc, insertionCallback: *mut PxInsertionCallback, condition: *mut PxConvexMeshCookingResult) -> *mut PxConvexMesh;

    /// Verifies if the convex mesh is valid. Prints an error message for each inconsistency found.
    ///
    /// The convex mesh descriptor must contain an already created convex mesh - the vertices, indices and polygons must be provided.
    ///
    /// This function should be used if PxConvexFlag::eDISABLE_MESH_VALIDATION is planned to be used in release builds.
    ///
    /// true if all the validity conditions hold, false otherwise.
    pub fn phys_PxValidateConvexMesh(params: *const PxCookingParams, desc: *const PxConvexMeshDesc) -> bool;

    /// Compute hull polygons from given vertices and triangles. Polygons are needed for PxConvexMeshDesc rather than triangles.
    ///
    /// Please note that the resulting polygons may have different number of vertices. Some vertices may be removed.
    /// The output vertices, indices and polygons must be used to construct a hull.
    ///
    /// The provided PxAllocatorCallback does allocate the out arrays. It is the user responsibility to deallocated those arrays.
    ///
    /// true on success
    pub fn phys_PxComputeHullPolygons(params: *const PxCookingParams, mesh: *const PxSimpleTriangleMesh, inCallback: *mut PxAllocatorCallback, nbVerts: *mut u32, vertices: *mut *mut PxVec3, nbIndices: *mut u32, indices: *mut *mut u32, nbPolygons: *mut u32, hullPolygons: *mut *mut PxHullPolygon) -> bool;

    /// Verifies if the triangle mesh is valid. Prints an error message for each inconsistency found.
    ///
    /// The following conditions are true for a valid triangle mesh:
    /// 1. There are no duplicate vertices (within specified vertexWeldTolerance. See PxCookingParams::meshWeldTolerance)
    /// 2. There are no large triangles (within specified PxTolerancesScale.)
    ///
    /// true if all the validity conditions hold, false otherwise.
    pub fn phys_PxValidateTriangleMesh(params: *const PxCookingParams, desc: *const PxTriangleMeshDesc) -> bool;

    /// Cooks a triangle mesh. The results are written to the stream.
    ///
    /// To create a triangle mesh object it is necessary to first 'cook' the mesh data into
    /// a form which allows the SDK to perform efficient collision detection.
    ///
    /// PxCookTriangleMesh() allows a mesh description to be cooked into a binary stream
    /// suitable for loading and performing collision detection at runtime.
    ///
    /// true on success
    pub fn phys_PxCookTriangleMesh(params: *const PxCookingParams, desc: *const PxTriangleMeshDesc, stream: *mut PxOutputStream, condition: *mut PxTriangleMeshCookingResult) -> bool;

    /// Cooks and creates a triangle mesh without going through a stream.
    ///
    /// This method does the same as PxCookTriangleMesh, but the produced mesh is not stored
    /// into a stream but is either directly inserted in PxPhysics, or created as a standalone
    /// object. Use this method if you are unable to cook offline.
    ///
    /// PxInsertionCallback can be obtained through PxPhysics::getPhysicsInsertionCallback()
    /// or PxGetStandaloneInsertionCallback().
    ///
    /// PxTriangleMesh pointer on success.
    pub fn phys_PxCreateTriangleMesh(params: *const PxCookingParams, desc: *const PxTriangleMeshDesc, insertionCallback: *mut PxInsertionCallback, condition: *mut PxTriangleMeshCookingResult) -> *mut PxTriangleMesh;

    /// Cooks a tetrahedron mesh. The results are written to the stream.
    ///
    /// To create a tetrahedron mesh object it is necessary to first 'cook' the mesh data into
    /// a form which allows the SDK to perform efficient collision detection.
    ///
    /// PxCookTetrahedronMesh() allows a mesh description to be cooked into a binary stream
    /// suitable for loading and performing collision detection at runtime.
    ///
    /// true on success
    pub fn phys_PxCookTetrahedronMesh(params: *const PxCookingParams, meshDesc: *const PxTetrahedronMeshDesc, stream: *mut PxOutputStream) -> bool;

    /// Cooks and creates a tetrahedron mesh without going through a stream.
    ///
    /// This method does the same as PxCookTetrahedronMesh, but the produced mesh is not stored
    /// into a stream but is either directly inserted in PxPhysics, or created as a standalone
    /// object. Use this method if you are unable to cook offline.
    ///
    /// PxInsertionCallback can be obtained through PxPhysics::getPhysicsInsertionCallback()
    /// or PxGetStandaloneInsertionCallback().
    ///
    /// PxTetrahedronMesh pointer on success.
    pub fn phys_PxCreateTetrahedronMesh(params: *const PxCookingParams, meshDesc: *const PxTetrahedronMeshDesc, insertionCallback: *mut PxInsertionCallback) -> *mut PxTetrahedronMesh;

    /// Cooks a deformable volume mesh. The results are written to the stream.
    ///
    /// To create a deformable volume mesh object it is necessary to first 'cook' the mesh data into
    /// a form which allows the SDK to perform efficient collision detection and to store data
    /// used during the FEM calculations.
    ///
    /// PxCookDeformableVolumeMesh() allows a mesh description to be cooked into a binary stream
    /// suitable for loading and performing collision detection at runtime.
    ///
    /// true on success
    pub fn phys_PxCookDeformableVolumeMesh(params: *const PxCookingParams, simulationMeshDesc: *const PxTetrahedronMeshDesc, collisionMeshDesc: *const PxTetrahedronMeshDesc, simulationDataDesc: *const PxDeformableVolumeSimulationDataDesc, stream: *mut PxOutputStream) -> bool;

    /// Cooks and creates a deformable volume mesh without going through a stream.
    ///
    /// This method does the same as PxCookDeformableVolumeMesh, but the produced mesh is not stored
    /// into a stream but is either directly inserted in PxPhysics, or created as a standalone
    /// object. Use this method if you are unable to cook offline.
    ///
    /// PxInsertionCallback can be obtained through PxPhysics::getPhysicsInsertionCallback()
    /// or PxGetStandaloneInsertionCallback().
    ///
    /// PxDeformableVolumeMesh pointer on success.
    pub fn phys_PxCreateDeformableVolumeMesh(params: *const PxCookingParams, simulationMeshDesc: *const PxTetrahedronMeshDesc, collisionMeshDesc: *const PxTetrahedronMeshDesc, simulationDataDesc: *const PxDeformableVolumeSimulationDataDesc, insertionCallback: *mut PxInsertionCallback) -> *mut PxDeformableVolumeMesh;

    /// Computes the mapping between collision and simulation mesh
    ///
    /// The deformable volume deformation is computed on the simulation mesh. To deform the collision mesh accordingly
    /// it needs to be specified how its vertices need to be placed and updated inside the deformation mesh.
    /// This method computes that embedding information.
    ///
    /// PxCollisionMeshMappingData pointer that describes how the collision mesh is embedded into the simulation mesh
    pub fn phys_PxComputeModelsMapping(params: *const PxCookingParams, simulationMesh: *mut PxTetrahedronMeshData, collisionMesh: *const PxTetrahedronMeshData, collisionData: *const PxDeformableVolumeCollisionData, vertexToTet: *const PxBoundedData) -> *mut PxCollisionMeshMappingData;

    /// Computes data to accelerate collision detection of tetrahedral meshes
    ///
    /// Computes data structures to speed up collision detection with tetrahedral meshes.
    ///
    /// PxCollisionTetrahedronMeshData pointer that describes the collision mesh
    pub fn phys_PxComputeCollisionData(params: *const PxCookingParams, collisionMeshDesc: *const PxTetrahedronMeshDesc) -> *mut PxCollisionTetrahedronMeshData;

    /// Computes data to accelerate collision detection of tetrahedral meshes
    ///
    /// Computes data to compute and store a deformable volume's deformation using FEM.
    ///
    /// PxSimulationTetrahedronMeshData pointer that describes the simulation mesh
    pub fn phys_PxComputeSimulationData(params: *const PxCookingParams, simulationMeshDesc: *const PxTetrahedronMeshDesc) -> *mut PxSimulationTetrahedronMeshData;

    /// Bundles all data required for deformable volume simulation
    ///
    /// Creates a container that provides everything to create a PxDeformableVolume
    ///
    /// PxDeformableVolumeMesh pointer that represents a deformable volume mesh bundling all data (simulation mesh, collision mesh etc.)
    pub fn phys_PxAssembleDeformableVolumeMesh(simulationMesh: *mut PxTetrahedronMeshData, simulationData: *mut PxDeformableVolumeSimulationData, collisionMesh: *mut PxTetrahedronMeshData, collisionData: *mut PxDeformableVolumeCollisionData, mappingData: *mut PxCollisionMeshMappingData, insertionCallback: *mut PxInsertionCallback) -> *mut PxDeformableVolumeMesh;

    pub fn PxDefaultMemoryOutputStream_new_alloc(allocator: *mut PxAllocatorCallback) -> *mut PxDefaultMemoryOutputStream;

    pub fn PxDefaultMemoryOutputStream_delete(self_: *mut PxDefaultMemoryOutputStream);

    pub fn PxDefaultMemoryOutputStream_write_mut(self_: *mut PxDefaultMemoryOutputStream, src: *const std::ffi::c_void, count: u64) -> u64;

    pub fn PxDefaultMemoryOutputStream_getSize(self_: *const PxDefaultMemoryOutputStream) -> u64;

    pub fn PxDefaultMemoryOutputStream_getData(self_: *const PxDefaultMemoryOutputStream) -> *mut u8;

    pub fn PxDefaultMemoryInputData_new_alloc(data: *const u8, length: u64) -> *mut PxDefaultMemoryInputData;

    pub fn PxDefaultMemoryInputData_read_mut(self_: *mut PxDefaultMemoryInputData, dest: *mut std::ffi::c_void, count: u64) -> u64;

    pub fn PxDefaultMemoryInputData_getLength(self_: *const PxDefaultMemoryInputData) -> u64;

    pub fn PxDefaultMemoryInputData_seek_mut(self_: *mut PxDefaultMemoryInputData, pos: u64);

    pub fn PxDefaultMemoryInputData_tell(self_: *const PxDefaultMemoryInputData) -> u64;

    pub fn PxDefaultFileOutputStream_new_alloc(name: *const std::ffi::c_char) -> *mut PxDefaultFileOutputStream;

    pub fn PxDefaultFileOutputStream_delete(self_: *mut PxDefaultFileOutputStream);

    pub fn PxDefaultFileOutputStream_write_mut(self_: *mut PxDefaultFileOutputStream, src: *const std::ffi::c_void, count: u64) -> u64;

    pub fn PxDefaultFileOutputStream_isValid_mut(self_: *mut PxDefaultFileOutputStream) -> bool;

    pub fn PxDefaultFileInputData_new_alloc(name: *const std::ffi::c_char) -> *mut PxDefaultFileInputData;

    pub fn PxDefaultFileInputData_delete(self_: *mut PxDefaultFileInputData);

    pub fn PxDefaultFileInputData_read_mut(self_: *mut PxDefaultFileInputData, dest: *mut std::ffi::c_void, count: u64) -> u64;

    pub fn PxDefaultFileInputData_seek_mut(self_: *mut PxDefaultFileInputData, pos: u64);

    pub fn PxDefaultFileInputData_tell(self_: *const PxDefaultFileInputData) -> u64;

    pub fn PxDefaultFileInputData_getLength(self_: *const PxDefaultFileInputData) -> u64;

    pub fn PxDefaultFileInputData_isValid(self_: *const PxDefaultFileInputData) -> bool;

    pub fn phys_platformAlignedAlloc(size: usize) -> *mut std::ffi::c_void;

    pub fn phys_platformAlignedFree(ptr: *mut std::ffi::c_void);

    pub fn PxDefaultAllocator_allocate_mut(self_: *mut PxDefaultAllocator, size: usize, anon_param1: *const std::ffi::c_char, anon_param2: *const std::ffi::c_char, anon_param3: i32) -> *mut std::ffi::c_void;

    pub fn PxDefaultAllocator_deallocate_mut(self_: *mut PxDefaultAllocator, ptr: *mut std::ffi::c_void);

    pub fn PxDefaultAllocator_delete(self_: *mut PxDefaultAllocator);

    /// Set the actors for this joint.
    ///
    /// An actor may be NULL to indicate the world frame. At most one of the actors may be NULL.
    pub fn PxJoint_setActors_mut(self_: *mut PxJoint, actor0: *mut PxRigidActor, actor1: *mut PxRigidActor);

    /// Get the actors for this joint.
    pub fn PxJoint_getActors(self_: *const PxJoint, actor0: *mut *mut PxRigidActor, actor1: *mut *mut PxRigidActor);

    /// Set the joint local pose for an actor.
    ///
    /// This is the relative pose which locates the joint frame relative to the actor.
    pub fn PxJoint_setLocalPose_mut(self_: *mut PxJoint, actor: PxJointActorIndex, localPose: *const PxTransform);

    /// get the joint local pose for an actor.
    ///
    /// return the local pose for this joint
    pub fn PxJoint_getLocalPose(self_: *const PxJoint, actor: PxJointActorIndex) -> PxTransform;

    /// get the relative pose for this joint
    ///
    /// This function returns the pose of the joint frame of actor1 relative to actor0
    pub fn PxJoint_getRelativeTransform(self_: *const PxJoint) -> PxTransform;

    /// get the relative linear velocity of the joint
    ///
    /// This function returns the linear velocity of the origin of the constraint frame of actor1, relative to the origin of the constraint
    /// frame of actor0. The value is returned in the constraint frame of actor0
    pub fn PxJoint_getRelativeLinearVelocity(self_: *const PxJoint) -> PxVec3;

    /// get the relative angular velocity of the joint
    ///
    /// This function returns the angular velocity of actor1 relative to actor0. The value is returned in the constraint frame of actor0
    pub fn PxJoint_getRelativeAngularVelocity(self_: *const PxJoint) -> PxVec3;

    /// set the break force for this joint.
    ///
    /// if the constraint force or torque on the joint exceeds the specified values, the joint will break,
    /// at which point it will not constrain the two actors and the flag PxConstraintFlag::eBROKEN will be set. The
    /// force and torque are measured in the joint frame of the first actor
    pub fn PxJoint_setBreakForce_mut(self_: *mut PxJoint, force: f32, torque: f32);

    /// get the break force for this joint.
    pub fn PxJoint_getBreakForce(self_: *const PxJoint, force: *mut f32, torque: *mut f32);

    /// set the constraint flags for this joint.
    pub fn PxJoint_setConstraintFlags_mut(self_: *mut PxJoint, flags: PxConstraintFlags);

    /// set a constraint flags for this joint to a specified value.
    pub fn PxJoint_setConstraintFlag_mut(self_: *mut PxJoint, flag: PxConstraintFlag, value: bool);

    /// get the constraint flags for this joint.
    ///
    /// the constraint flags
    pub fn PxJoint_getConstraintFlags(self_: *const PxJoint) -> PxConstraintFlags;

    /// set the inverse mass scale for actor0.
    pub fn PxJoint_setInvMassScale0_mut(self_: *mut PxJoint, invMassScale: f32);

    /// get the inverse mass scale for actor0.
    ///
    /// inverse mass scale for actor0
    pub fn PxJoint_getInvMassScale0(self_: *const PxJoint) -> f32;

    /// set the inverse inertia scale for actor0.
    pub fn PxJoint_setInvInertiaScale0_mut(self_: *mut PxJoint, invInertiaScale: f32);

    /// get the inverse inertia scale for actor0.
    ///
    /// inverse inertia scale for actor0
    pub fn PxJoint_getInvInertiaScale0(self_: *const PxJoint) -> f32;

    /// set the inverse mass scale for actor1.
    pub fn PxJoint_setInvMassScale1_mut(self_: *mut PxJoint, invMassScale: f32);

    /// get the inverse mass scale for actor1.
    ///
    /// inverse mass scale for actor1
    pub fn PxJoint_getInvMassScale1(self_: *const PxJoint) -> f32;

    /// set the inverse inertia scale for actor1.
    pub fn PxJoint_setInvInertiaScale1_mut(self_: *mut PxJoint, invInertiaScale: f32);

    /// get the inverse inertia scale for actor1.
    ///
    /// inverse inertia scale for actor1
    pub fn PxJoint_getInvInertiaScale1(self_: *const PxJoint) -> f32;

    /// Retrieves the PxConstraint corresponding to this joint.
    ///
    /// This can be used to determine, among other things, the force applied at the joint.
    ///
    /// the constraint
    pub fn PxJoint_getConstraint(self_: *const PxJoint) -> *mut PxConstraint;

    /// Sets a name string for the object that can be retrieved with getName().
    ///
    /// This is for debugging and is not used by the SDK. The string is not copied by the SDK,
    /// only the pointer is stored.
    pub fn PxJoint_setName_mut(self_: *mut PxJoint, name: *const std::ffi::c_char);

    /// Retrieves the name string set with setName().
    ///
    /// Name string associated with object.
    pub fn PxJoint_getName(self_: *const PxJoint) -> *const std::ffi::c_char;

    /// Deletes the joint.
    ///
    /// This call does not wake up the connected rigid bodies.
    pub fn PxJoint_release_mut(self_: *mut PxJoint);

    /// Retrieves the scene which this joint belongs to.
    ///
    /// Owner Scene. NULL if not part of a scene.
    pub fn PxJoint_getScene(self_: *const PxJoint) -> *mut PxScene;

    pub fn PxSpring_new(stiffness_: f32, damping_: f32) -> PxSpring;

    /// Helper function to setup a joint's global frame
    ///
    /// This replaces the following functions from previous SDK versions:
    ///
    /// void NxJointDesc::setGlobalAnchor(const NxVec3
    /// &
    /// wsAnchor);
    /// void NxJointDesc::setGlobalAxis(const NxVec3
    /// &
    /// wsAxis);
    ///
    /// The function sets the joint's localPose using world-space input parameters.
    pub fn phys_PxSetJointGlobalFrame(joint: *mut PxJoint, wsAnchor: *const PxVec3, wsAxis: *const PxVec3);

    /// Create a distance Joint.
    pub fn phys_PxDistanceJointCreate(physics: *mut PxPhysics, actor0: *mut PxRigidActor, localFrame0: *const PxTransform, actor1: *mut PxRigidActor, localFrame1: *const PxTransform) -> *mut PxDistanceJoint;

    /// Return the current distance of the joint
    pub fn PxDistanceJoint_getDistance(self_: *const PxDistanceJoint) -> f32;

    /// Set the allowed minimum distance for the joint.
    ///
    /// The minimum	distance must be no more than the maximum distance
    ///
    /// Default
    /// 0.0f
    /// Range
    /// [0, PX_MAX_F32)
    pub fn PxDistanceJoint_setMinDistance_mut(self_: *mut PxDistanceJoint, distance: f32);

    /// Get the allowed minimum distance for the joint.
    ///
    /// the allowed minimum distance
    pub fn PxDistanceJoint_getMinDistance(self_: *const PxDistanceJoint) -> f32;

    /// Set the allowed maximum distance for the joint.
    ///
    /// The maximum	distance must be no less than the minimum distance.
    ///
    /// Default
    /// 0.0f
    /// Range
    /// [0, PX_MAX_F32)
    pub fn PxDistanceJoint_setMaxDistance_mut(self_: *mut PxDistanceJoint, distance: f32);

    /// Get the allowed maximum distance for the joint.
    ///
    /// the allowed maximum distance
    pub fn PxDistanceJoint_getMaxDistance(self_: *const PxDistanceJoint) -> f32;

    /// Set the error tolerance of the joint.
    pub fn PxDistanceJoint_setTolerance_mut(self_: *mut PxDistanceJoint, tolerance: f32);

    /// Get the error tolerance of the joint.
    ///
    /// the distance beyond the joint's [min, max] range before the joint becomes active.
    ///
    /// Default
    /// 0.25f * PxTolerancesScale::length
    /// Range
    /// (0, PX_MAX_F32)
    ///
    /// This value should be used to ensure that if the minimum distance is zero and the
    /// spring function is in use, the rest length of the spring is non-zero.
    pub fn PxDistanceJoint_getTolerance(self_: *const PxDistanceJoint) -> f32;

    /// Set the strength of the joint spring.
    ///
    /// The spring is used if enabled, and the distance exceeds the range [min-error, max+error].
    ///
    /// Default
    /// 0.0f
    /// Range
    /// [0, PX_MAX_F32)
    pub fn PxDistanceJoint_setStiffness_mut(self_: *mut PxDistanceJoint, stiffness: f32);

    /// Get the strength of the joint spring.
    ///
    /// stiffness the spring strength of the joint
    pub fn PxDistanceJoint_getStiffness(self_: *const PxDistanceJoint) -> f32;

    /// Set the damping of the joint spring.
    ///
    /// The spring is used if enabled, and the distance exceeds the range [min-error, max+error].
    ///
    /// Default
    /// 0.0f
    /// Range
    /// [0, PX_MAX_F32)
    pub fn PxDistanceJoint_setDamping_mut(self_: *mut PxDistanceJoint, damping: f32);

    /// Get the damping of the joint spring.
    ///
    /// the degree of damping of the joint spring of the joint
    pub fn PxDistanceJoint_getDamping(self_: *const PxDistanceJoint) -> f32;

    /// Set the flags specific to the Distance Joint.
    ///
    /// Default
    /// PxDistanceJointFlag::eMAX_DISTANCE_ENABLED
    pub fn PxDistanceJoint_setDistanceJointFlags_mut(self_: *mut PxDistanceJoint, flags: PxDistanceJointFlags);

    /// Set a single flag specific to a Distance Joint to true or false.
    pub fn PxDistanceJoint_setDistanceJointFlag_mut(self_: *mut PxDistanceJoint, flag: PxDistanceJointFlag, value: bool);

    /// Get the flags specific to the Distance Joint.
    ///
    /// the joint flags
    pub fn PxDistanceJoint_getDistanceJointFlags(self_: *const PxDistanceJoint) -> PxDistanceJointFlags;

    /// Returns string name of PxDistanceJoint, used for serialization
    pub fn PxDistanceJoint_getConcreteTypeName(self_: *const PxDistanceJoint) -> *const std::ffi::c_char;

    /// Create a fixed joint.
    pub fn phys_PxFixedJointCreate(physics: *mut PxPhysics, actor0: *mut PxRigidActor, localFrame0: *const PxTransform, actor1: *mut PxRigidActor, localFrame1: *const PxTransform) -> *mut PxFixedJoint;

    /// Returns string name of PxFixedJoint, used for serialization
    pub fn PxFixedJoint_getConcreteTypeName(self_: *const PxFixedJoint) -> *const std::ffi::c_char;

    pub fn PxJointLimitParameters_new_alloc() -> *mut PxJointLimitParameters;

    /// Returns true if the current settings are valid.
    ///
    /// true if the current settings are valid
    pub fn PxJointLimitParameters_isValid(self_: *const PxJointLimitParameters) -> bool;

    pub fn PxJointLimitParameters_isSoft(self_: *const PxJointLimitParameters) -> bool;

    /// construct a linear hard limit
    pub fn PxJointLinearLimit_new(extent: f32) -> PxJointLinearLimit;

    /// construct a linear soft limit
    pub fn PxJointLinearLimit_new_1(extent: f32, spring: *const PxSpring) -> PxJointLinearLimit;

    /// Returns true if the limit is valid
    ///
    /// true if the current settings are valid
    pub fn PxJointLinearLimit_isValid(self_: *const PxJointLinearLimit) -> bool;

    pub fn PxJointLinearLimit_delete(self_: *mut PxJointLinearLimit);

    /// Construct a linear hard limit pair. The lower distance value must be less than the upper distance value.
    pub fn PxJointLinearLimitPair_new(scale: *const PxTolerancesScale, lowerLimit: f32, upperLimit: f32) -> PxJointLinearLimitPair;

    /// construct a linear soft limit pair
    pub fn PxJointLinearLimitPair_new_1(lowerLimit: f32, upperLimit: f32, spring: *const PxSpring) -> PxJointLinearLimitPair;

    /// Returns true if the limit is valid.
    ///
    /// true if the current settings are valid
    pub fn PxJointLinearLimitPair_isValid(self_: *const PxJointLinearLimitPair) -> bool;

    pub fn PxJointLinearLimitPair_delete(self_: *mut PxJointLinearLimitPair);

    /// construct an angular hard limit pair.
    ///
    /// The lower value must be less than the upper value.
    pub fn PxJointAngularLimitPair_new(lowerLimit: f32, upperLimit: f32) -> PxJointAngularLimitPair;

    /// construct an angular soft limit pair.
    ///
    /// The lower value must be less than the upper value.
    pub fn PxJointAngularLimitPair_new_1(lowerLimit: f32, upperLimit: f32, spring: *const PxSpring) -> PxJointAngularLimitPair;

    /// Returns true if the limit is valid.
    ///
    /// true if the current settings are valid
    pub fn PxJointAngularLimitPair_isValid(self_: *const PxJointAngularLimitPair) -> bool;

    pub fn PxJointAngularLimitPair_delete(self_: *mut PxJointAngularLimitPair);

    /// Construct a cone hard limit.
    pub fn PxJointLimitCone_new(yLimitAngle: f32, zLimitAngle: f32) -> PxJointLimitCone;

    /// Construct a cone soft limit.
    pub fn PxJointLimitCone_new_1(yLimitAngle: f32, zLimitAngle: f32, spring: *const PxSpring) -> PxJointLimitCone;

    /// Returns true if the limit is valid.
    ///
    /// true if the current settings are valid
    pub fn PxJointLimitCone_isValid(self_: *const PxJointLimitCone) -> bool;

    pub fn PxJointLimitCone_delete(self_: *mut PxJointLimitCone);

    /// Construct a pyramid hard limit.
    pub fn PxJointLimitPyramid_new(yLimitAngleMin: f32, yLimitAngleMax: f32, zLimitAngleMin: f32, zLimitAngleMax: f32) -> PxJointLimitPyramid;

    /// Construct a pyramid soft limit.
    pub fn PxJointLimitPyramid_new_1(yLimitAngleMin: f32, yLimitAngleMax: f32, zLimitAngleMin: f32, zLimitAngleMax: f32, spring: *const PxSpring) -> PxJointLimitPyramid;

    /// Returns true if the limit is valid.
    ///
    /// true if the current settings are valid
    pub fn PxJointLimitPyramid_isValid(self_: *const PxJointLimitPyramid) -> bool;

    pub fn PxJointLimitPyramid_delete(self_: *mut PxJointLimitPyramid);

    /// Create a prismatic joint.
    pub fn phys_PxPrismaticJointCreate(physics: *mut PxPhysics, actor0: *mut PxRigidActor, localFrame0: *const PxTransform, actor1: *mut PxRigidActor, localFrame1: *const PxTransform) -> *mut PxPrismaticJoint;

    /// returns the displacement of the joint along its axis.
    pub fn PxPrismaticJoint_getPosition(self_: *const PxPrismaticJoint) -> f32;

    /// returns the velocity of the joint along its axis
    pub fn PxPrismaticJoint_getVelocity(self_: *const PxPrismaticJoint) -> f32;

    /// sets the joint limit  parameters.
    ///
    /// The limit range is [-PX_MAX_F32, PX_MAX_F32], but note that the width of the limit (upper-lower) must also be
    /// a valid float.
    pub fn PxPrismaticJoint_setLimit_mut(self_: *mut PxPrismaticJoint, anon_param0: *const PxJointLinearLimitPair);

    /// gets the joint limit  parameters.
    pub fn PxPrismaticJoint_getLimit(self_: *const PxPrismaticJoint) -> PxJointLinearLimitPair;

    /// Set the flags specific to the Prismatic Joint.
    ///
    /// Default
    /// PxPrismaticJointFlags(0)
    pub fn PxPrismaticJoint_setPrismaticJointFlags_mut(self_: *mut PxPrismaticJoint, flags: PxPrismaticJointFlags);

    /// Set a single flag specific to a Prismatic Joint to true or false.
    pub fn PxPrismaticJoint_setPrismaticJointFlag_mut(self_: *mut PxPrismaticJoint, flag: PxPrismaticJointFlag, value: bool);

    /// Get the flags specific to the Prismatic Joint.
    ///
    /// the joint flags
    pub fn PxPrismaticJoint_getPrismaticJointFlags(self_: *const PxPrismaticJoint) -> PxPrismaticJointFlags;

    /// Returns string name of PxPrismaticJoint, used for serialization
    pub fn PxPrismaticJoint_getConcreteTypeName(self_: *const PxPrismaticJoint) -> *const std::ffi::c_char;

    /// Create a revolute joint.
    pub fn phys_PxRevoluteJointCreate(physics: *mut PxPhysics, actor0: *mut PxRigidActor, localFrame0: *const PxTransform, actor1: *mut PxRigidActor, localFrame1: *const PxTransform) -> *mut PxRevoluteJoint;

    /// return the angle of the joint, in the range (-2*Pi, 2*Pi]
    pub fn PxRevoluteJoint_getAngle(self_: *const PxRevoluteJoint) -> f32;

    /// return the velocity of the joint
    pub fn PxRevoluteJoint_getVelocity(self_: *const PxRevoluteJoint) -> f32;

    /// set the joint limit parameters.
    ///
    /// The limit is activated using the flag PxRevoluteJointFlag::eLIMIT_ENABLED
    ///
    /// The limit angle range is (-2*Pi, 2*Pi).
    pub fn PxRevoluteJoint_setLimit_mut(self_: *mut PxRevoluteJoint, limits: *const PxJointAngularLimitPair);

    /// get the joint limit parameters.
    ///
    /// the joint limit parameters
    pub fn PxRevoluteJoint_getLimit(self_: *const PxRevoluteJoint) -> PxJointAngularLimitPair;

    /// set the target velocity for the drive model.
    ///
    /// The motor will only be able to reach this velocity if the maxForce is sufficiently large.
    /// If the joint is spinning faster than this velocity, the motor will actually try to brake
    /// (see PxRevoluteJointFlag::eDRIVE_FREESPIN.)
    ///
    /// The sign of this variable determines the rotation direction, with positive values going
    /// the same way as positive joint angles. Setting a very large target velocity may cause
    /// undesirable results.
    ///
    /// Range:
    /// (-PX_MAX_F32, PX_MAX_F32)
    /// Default:
    /// 0.0
    pub fn PxRevoluteJoint_setDriveVelocity_mut(self_: *mut PxRevoluteJoint, velocity: f32, autowake: bool);

    /// gets the target velocity for the drive model.
    ///
    /// the drive target velocity
    pub fn PxRevoluteJoint_getDriveVelocity(self_: *const PxRevoluteJoint) -> f32;

    /// sets the maximum torque the drive can exert.
    ///
    /// The value set here may be used either as an impulse limit or a force limit, depending on the flag PxConstraintFlag::eDRIVE_LIMITS_ARE_FORCES
    ///
    /// Range:
    /// [0, PX_MAX_F32)
    /// Default:
    /// PX_MAX_F32
    pub fn PxRevoluteJoint_setDriveForceLimit_mut(self_: *mut PxRevoluteJoint, limit: f32);

    /// gets the maximum torque the drive can exert.
    ///
    /// the torque limit
    pub fn PxRevoluteJoint_getDriveForceLimit(self_: *const PxRevoluteJoint) -> f32;

    /// sets the gear ratio for the drive.
    ///
    /// When setting up the drive constraint, the velocity of the first actor is scaled by this value, and its response to drive torque is scaled down.
    /// So if the drive target velocity is zero, the second actor will be driven to the velocity of the first scaled by the gear ratio
    ///
    /// Range:
    /// [0, PX_MAX_F32)
    /// Default:
    /// 1.0
    pub fn PxRevoluteJoint_setDriveGearRatio_mut(self_: *mut PxRevoluteJoint, ratio: f32);

    /// gets the gear ratio.
    ///
    /// the drive gear ratio
    pub fn PxRevoluteJoint_getDriveGearRatio(self_: *const PxRevoluteJoint) -> f32;

    /// sets the flags specific to the Revolute Joint.
    ///
    /// Default
    /// PxRevoluteJointFlags(0)
    pub fn PxRevoluteJoint_setRevoluteJointFlags_mut(self_: *mut PxRevoluteJoint, flags: PxRevoluteJointFlags);

    /// sets a single flag specific to a Revolute Joint.
    pub fn PxRevoluteJoint_setRevoluteJointFlag_mut(self_: *mut PxRevoluteJoint, flag: PxRevoluteJointFlag, value: bool);

    /// gets the flags specific to the Revolute Joint.
    ///
    /// the joint flags
    pub fn PxRevoluteJoint_getRevoluteJointFlags(self_: *const PxRevoluteJoint) -> PxRevoluteJointFlags;

    /// Returns string name of PxRevoluteJoint, used for serialization
    pub fn PxRevoluteJoint_getConcreteTypeName(self_: *const PxRevoluteJoint) -> *const std::ffi::c_char;

    /// Create a spherical joint.
    pub fn phys_PxSphericalJointCreate(physics: *mut PxPhysics, actor0: *mut PxRigidActor, localFrame0: *const PxTransform, actor1: *mut PxRigidActor, localFrame1: *const PxTransform) -> *mut PxSphericalJoint;

    /// Set the limit cone.
    ///
    /// If enabled, the limit cone will constrain the angular movement of the joint to lie
    /// within an elliptical cone.
    ///
    /// the limit cone
    pub fn PxSphericalJoint_getLimitCone(self_: *const PxSphericalJoint) -> PxJointLimitCone;

    /// Get the limit cone.
    pub fn PxSphericalJoint_setLimitCone_mut(self_: *mut PxSphericalJoint, limit: *const PxJointLimitCone);

    /// get the swing angle of the joint from the Y axis
    pub fn PxSphericalJoint_getSwingYAngle(self_: *const PxSphericalJoint) -> f32;

    /// get the swing angle of the joint from the Z axis
    pub fn PxSphericalJoint_getSwingZAngle(self_: *const PxSphericalJoint) -> f32;

    /// Set the flags specific to the Spherical Joint.
    ///
    /// Default
    /// PxSphericalJointFlags(0)
    pub fn PxSphericalJoint_setSphericalJointFlags_mut(self_: *mut PxSphericalJoint, flags: PxSphericalJointFlags);

    /// Set a single flag specific to a Spherical Joint to true or false.
    pub fn PxSphericalJoint_setSphericalJointFlag_mut(self_: *mut PxSphericalJoint, flag: PxSphericalJointFlag, value: bool);

    /// Get the flags specific to the Spherical Joint.
    ///
    /// the joint flags
    pub fn PxSphericalJoint_getSphericalJointFlags(self_: *const PxSphericalJoint) -> PxSphericalJointFlags;

    /// Returns string name of PxSphericalJoint, used for serialization
    pub fn PxSphericalJoint_getConcreteTypeName(self_: *const PxSphericalJoint) -> *const std::ffi::c_char;

    /// Create a D6 joint.
    pub fn phys_PxD6JointCreate(physics: *mut PxPhysics, actor0: *mut PxRigidActor, localFrame0: *const PxTransform, actor1: *mut PxRigidActor, localFrame1: *const PxTransform) -> *mut PxD6Joint;

    /// default constructor for PxD6JointDrive.
    pub fn PxD6JointDrive_new() -> PxD6JointDrive;

    /// constructor a PxD6JointDrive.
    pub fn PxD6JointDrive_new_1(driveStiffness: f32, driveDamping: f32, driveForceLimit: f32, isAcceleration: bool) -> PxD6JointDrive;

    /// returns true if the drive is valid
    pub fn PxD6JointDrive_isValid(self_: *const PxD6JointDrive) -> bool;

    /// Set the motion type around the specified axis.
    ///
    /// Each axis may independently specify that the degree of freedom is locked (blocking relative movement
    /// along or around this axis), limited by the corresponding limit, or free.
    ///
    /// Default:
    /// all degrees of freedom are locked
    pub fn PxD6Joint_setMotion_mut(self_: *mut PxD6Joint, axis: PxD6Axis, type_: PxD6Motion);

    /// Get the motion type around the specified axis.
    ///
    /// the motion type around the specified axis
    pub fn PxD6Joint_getMotion(self_: *const PxD6Joint, axis: PxD6Axis) -> PxD6Motion;

    /// get the twist angle of the joint, in the range (-2*Pi, 2*Pi]
    pub fn PxD6Joint_getTwistAngle(self_: *const PxD6Joint) -> f32;

    /// get the swing angle of the joint from the Y axis
    pub fn PxD6Joint_getSwingYAngle(self_: *const PxD6Joint) -> f32;

    /// get the swing angle of the joint from the Z axis
    pub fn PxD6Joint_getSwingZAngle(self_: *const PxD6Joint) -> f32;

    /// Set the distance limit for the joint.
    ///
    /// A single limit constraints all linear limited degrees of freedom, forming a linear, circular
    /// or spherical constraint on motion depending on the number of limited degrees. This is similar
    /// to a distance limit.
    pub fn PxD6Joint_setDistanceLimit_mut(self_: *mut PxD6Joint, limit: *const PxJointLinearLimit);

    /// Get the distance limit for the joint.
    ///
    /// the distance limit structure
    pub fn PxD6Joint_getDistanceLimit(self_: *const PxD6Joint) -> PxJointLinearLimit;

    /// Set the linear limit for a given linear axis.
    ///
    /// This function extends the previous setDistanceLimit call with the following features:
    /// - there can be a different limit for each linear axis
    /// - each limit is defined by two values, i.e. it can now be asymmetric
    ///
    /// This can be used to create prismatic joints similar to PxPrismaticJoint, or point-in-quad joints,
    /// or point-in-box joints.
    pub fn PxD6Joint_setLinearLimit_mut(self_: *mut PxD6Joint, axis: PxD6Axis, limit: *const PxJointLinearLimitPair);

    /// Get the linear limit for a given linear axis.
    ///
    /// the linear limit pair structure from desired axis
    pub fn PxD6Joint_getLinearLimit(self_: *const PxD6Joint, axis: PxD6Axis) -> PxJointLinearLimitPair;

    /// Set the twist limit for the joint.
    ///
    /// The twist limit controls the range of motion around the twist axis.
    ///
    /// The limit angle range is (-2*Pi, 2*Pi).
    pub fn PxD6Joint_setTwistLimit_mut(self_: *mut PxD6Joint, limit: *const PxJointAngularLimitPair);

    /// Get the twist limit for the joint.
    ///
    /// the twist limit structure
    pub fn PxD6Joint_getTwistLimit(self_: *const PxD6Joint) -> PxJointAngularLimitPair;

    /// Set the swing cone limit for the joint.
    ///
    /// The cone limit is used if either or both swing axes are limited. The extents are
    /// symmetrical and measured in the frame of the parent. If only one swing degree of freedom
    /// is limited, the corresponding value from the cone limit defines the limit range.
    pub fn PxD6Joint_setSwingLimit_mut(self_: *mut PxD6Joint, limit: *const PxJointLimitCone);

    /// Get the cone limit for the joint.
    ///
    /// the swing limit structure
    pub fn PxD6Joint_getSwingLimit(self_: *const PxD6Joint) -> PxJointLimitCone;

    /// Set a pyramidal swing limit for the joint.
    ///
    /// The pyramid limits will only be used in the following cases:
    /// - both swing Y and Z are limited. The limit shape is then a pyramid.
    /// - Y is limited and Z is locked, or vice versa. The limit shape is an asymmetric angular section, similar to
    /// what is supported for the twist axis.
    /// The remaining cases (Y limited and Z is free, or vice versa) are not supported.
    pub fn PxD6Joint_setPyramidSwingLimit_mut(self_: *mut PxD6Joint, limit: *const PxJointLimitPyramid);

    /// Get the pyramidal swing limit for the joint.
    ///
    /// the swing limit structure
    pub fn PxD6Joint_getPyramidSwingLimit(self_: *const PxD6Joint) -> PxJointLimitPyramid;

    /// Set the angular drive model to apply.
    ///
    /// The configuration will limit the allowed set of angular drive types (see [`PxD6Drive`]) to use
    /// when calling [`PxD6Joint::setDrive`]().
    ///
    /// Changing the angular drive model, will reset all the parameters for the angular drives to
    /// their default values (see [`PxD6Joint::setDrive`]() for information on the default values).
    ///
    /// Default
    /// PxD6AngularDriveConfig::eSWING_TWIST
    pub fn PxD6Joint_setAngularDriveConfig_mut(self_: *mut PxD6Joint, config: PxD6AngularDriveConfig);

    /// Get the angular drive model to apply.
    ///
    /// The angular drive model to apply.
    pub fn PxD6Joint_getAngularDriveConfig(self_: *const PxD6Joint) -> PxD6AngularDriveConfig;

    /// Set the drive parameters for the specified drive type.
    ///
    /// The angular drive configuration (see [`PxD6AngularDriveConfig`]) defines what type of
    /// angular drives will be accepted.
    ///
    /// Default
    /// The default drive spring and damping values are zero, the force limit is PX_MAX_F32, and no flags are set.
    pub fn PxD6Joint_setDrive_mut(self_: *mut PxD6Joint, index: PxD6Drive, drive: *const PxD6JointDrive);

    /// Get the drive parameters for the specified drive type.
    ///
    /// The angular drive configuration (see [`PxD6AngularDriveConfig`]) defines what type of
    /// angular drives will be accepted.
    pub fn PxD6Joint_getDrive(self_: *const PxD6Joint, index: PxD6Drive) -> PxD6JointDrive;

    /// Set the drive goal pose
    ///
    /// The goal is relative to the constraint frame of actor[0]
    ///
    /// Default
    /// the identity transform
    pub fn PxD6Joint_setDrivePosition_mut(self_: *mut PxD6Joint, pose: *const PxTransform, autowake: bool);

    /// Get the drive goal pose.
    pub fn PxD6Joint_getDrivePosition(self_: *const PxD6Joint) -> PxTransform;

    /// Set the target goal velocity for drive.
    ///
    /// The velocity is measured in the constraint frame of actor[0]
    pub fn PxD6Joint_setDriveVelocity_mut(self_: *mut PxD6Joint, linear: *const PxVec3, angular: *const PxVec3, autowake: bool);

    /// Get the target goal velocity for joint drive.
    pub fn PxD6Joint_getDriveVelocity(self_: *const PxD6Joint, linear: *mut PxVec3, angular: *mut PxVec3);

    /// Returns the GPU D6 joint index.
    ///
    /// Only use in combination with enabled GPU dynamics and enabled direct GPU API
    /// (see [`PxSceneFlag::eENABLE_GPU_DYNAMICS`], #PxSceneFlag::eENABLE_DIRECT_GPU_API,
    /// [`PxBroadPhaseType::eGPU`])
    ///
    /// The GPU index, or PX_INVALID_D6_JOINT_GPU_INDEX if the joint is not part of a PxScene.
    pub fn PxD6Joint_getGPUIndex(self_: *const PxD6Joint) -> u32;

    /// Returns string name of PxD6Joint, used for serialization
    pub fn PxD6Joint_getConcreteTypeName(self_: *const PxD6Joint) -> *const std::ffi::c_char;

    /// Create a gear Joint.
    pub fn phys_PxGearJointCreate(physics: *mut PxPhysics, actor0: *mut PxRigidActor, localFrame0: *const PxTransform, actor1: *mut PxRigidActor, localFrame1: *const PxTransform) -> *mut PxGearJoint;

    /// Set the hinge/revolute joints connected by the gear joint.
    ///
    /// The passed joints can be either PxRevoluteJoint, PxD6Joint or PxArticulationJointReducedCoordinate.
    /// The joints must define degrees of freedom around the twist axis.
    ///
    /// Note that these joints are only used to compute the positional error correction term,
    /// used to adjust potential drift between jointed actors. The gear joint can run without
    /// calling this function, but in that case some visible overlap may develop over time between
    /// the teeth of the gear meshes.
    ///
    /// Calling this function resets the internal positional error correction term.
    ///
    /// true if success
    pub fn PxGearJoint_setHinges_mut(self_: *mut PxGearJoint, hinge0: *const PxBase, hinge1: *const PxBase) -> bool;

    /// Get the hinge/revolute joints connected by the gear joint.
    pub fn PxGearJoint_getHinges(self_: *const PxGearJoint, hinge0: *mut *const PxBase, hinge1: *mut *const PxBase);

    /// Set the desired gear ratio.
    ///
    /// For two gears with n0 and n1 teeth respectively, the gear ratio is n0/n1.
    ///
    /// You may need to use a negative gear ratio if the joint frames of involved actors are not oriented in the same direction.
    ///
    /// Calling this function resets the internal positional error correction term.
    pub fn PxGearJoint_setGearRatio_mut(self_: *mut PxGearJoint, ratio: f32);

    /// Get the gear ratio.
    ///
    /// Current ratio
    pub fn PxGearJoint_getGearRatio(self_: *const PxGearJoint) -> f32;

    pub fn PxGearJoint_getConcreteTypeName(self_: *const PxGearJoint) -> *const std::ffi::c_char;

    /// Create a rack
    /// &
    /// pinion Joint.
    pub fn phys_PxRackAndPinionJointCreate(physics: *mut PxPhysics, actor0: *mut PxRigidActor, localFrame0: *const PxTransform, actor1: *mut PxRigidActor, localFrame1: *const PxTransform) -> *mut PxRackAndPinionJoint;

    /// Set the hinge
    /// &
    /// prismatic joints connected by the rack
    /// &
    /// pinion joint.
    ///
    /// The passed hinge joint can be either PxRevoluteJoint, PxD6Joint or PxArticulationJointReducedCoordinate.
    /// The passed prismatic joint can be either PxPrismaticJoint or PxD6Joint.
    ///
    /// Note that these joints are only used to compute the positional error correction term,
    /// used to adjust potential drift between jointed actors. The rack
    /// &
    /// pinion joint can run without
    /// calling this function, but in that case some visible overlap may develop over time between
    /// the teeth of the rack
    /// &
    /// pinion meshes.
    ///
    /// Calling this function resets the internal positional error correction term.
    ///
    /// true if success
    pub fn PxRackAndPinionJoint_setJoints_mut(self_: *mut PxRackAndPinionJoint, hinge: *const PxBase, prismatic: *const PxBase) -> bool;

    /// Get the hinge
    /// &
    /// prismatic joints connected by the rack
    /// &
    /// pinion joint.
    pub fn PxRackAndPinionJoint_getJoints(self_: *const PxRackAndPinionJoint, hinge: *mut *const PxBase, prismatic: *mut *const PxBase);

    /// Set the desired ratio directly.
    ///
    /// You may need to use a negative gear ratio if the joint frames of involved actors are not oriented in the same direction.
    ///
    /// Calling this function resets the internal positional error correction term.
    pub fn PxRackAndPinionJoint_setRatio_mut(self_: *mut PxRackAndPinionJoint, ratio: f32);

    /// Get the ratio.
    ///
    /// Current ratio
    pub fn PxRackAndPinionJoint_getRatio(self_: *const PxRackAndPinionJoint) -> f32;

    /// Set the desired ratio indirectly.
    ///
    /// This is a simple helper function that computes the ratio from passed data:
    ///
    /// ratio = (PI*2*nbRackTeeth)/(rackLength*nbPinionTeeth)
    ///
    /// Calling this function resets the internal positional error correction term.
    ///
    /// true if success
    pub fn PxRackAndPinionJoint_setData_mut(self_: *mut PxRackAndPinionJoint, nbRackTeeth: u32, nbPinionTeeth: u32, rackLength: f32) -> bool;

    pub fn PxRackAndPinionJoint_getConcreteTypeName(self_: *const PxRackAndPinionJoint) -> *const std::ffi::c_char;

    pub fn PxGroupsMask_new_alloc() -> *mut PxGroupsMask;

    pub fn PxGroupsMask_delete(self_: *mut PxGroupsMask);

    /// Implementation of a simple filter shader that emulates PhysX 2.8.x filtering
    ///
    /// This shader provides the following logic:
    ///
    /// If one of the two filter objects is a trigger, the pair is acccepted and [`PxPairFlag::eTRIGGER_DEFAULT`] will be used for trigger reports
    ///
    /// Else, if the filter mask logic (see further below) discards the pair it will be suppressed ([`PxFilterFlag::eSUPPRESS`])
    ///
    /// Else, the pair gets accepted and collision response gets enabled ([`PxPairFlag::eCONTACT_DEFAULT`])
    ///
    /// Filter mask logic:
    /// Given the two [`PxFilterData`] structures fd0 and fd1 of two collision objects, the pair passes the filter if the following
    /// conditions are met:
    ///
    /// 1) Collision groups of the pair are enabled
    /// 2) Collision filtering equation is satisfied
    ///
    /// Each actor can belong to a single collision group. Use PxSetGroup to set the group of an actor and PxGetGroup to retrieve the group of an actor.
    /// A collision group is an integer value between 0 and 31 defining which group the actor belongs to. Because that value is written to an actor's
    /// shapes internally (it is stored in the shapes' PxFilterData), this feature does not work with shared shapes, unless they all belong to actors
    /// whose groups are similar. For example it would not work to share a shape between actors A and B, and then assign A to group 0 and B to group 1,
    /// as they would both internally try to write different group values to the same shape.
    ///
    /// Once actors are assigned to groups, it is possible to define how groups collide with each-other using the PxSetGroupCollisionFlag function.
    /// Use this function to set a simple boolean value per group pairs, defining if the corresponding groups should collide. If not, collisions between
    /// actors of these non-colliding groups will be automatically disabled by the PxDefaultSimulationFilterShader.
    pub fn phys_PxDefaultSimulationFilterShader(attributes0: u32, filterData0: PxFilterData, attributes1: u32, filterData1: PxFilterData, pairFlags: *mut PxPairFlags, constantBlock: *const std::ffi::c_void, constantBlockSize: u32) -> PxFilterFlags;

    /// Determines if collision detection is performed between a pair of groups
    ///
    /// Collision group is an integer between 0 and 31.
    ///
    /// PxGetGroupCollisionFlag(a, b) is the same as PxGetGroupCollisionFlag(b, a)
    ///
    /// True if the groups should collide
    pub fn phys_PxGetGroupCollisionFlag(group1: u16, group2: u16) -> bool;

    /// Specifies if collision should be performed by a pair of groups
    ///
    /// Collision group is an integer between 0 and 31.
    ///
    /// PxSetGroupCollisionFlag(a, b) is the same as PxSetGroupCollisionFlag(b, a)
    pub fn phys_PxSetGroupCollisionFlag(group1: u16, group2: u16, enable: bool);

    /// Retrieves the value set with PxSetGroup()
    ///
    /// Collision group is an integer between 0 and 31.
    ///
    /// The collision group this actor belongs to
    pub fn phys_PxGetGroup(actor: *const PxActor) -> u16;

    /// Sets which collision group this actor is part of
    ///
    /// Collision group is an integer between 0 and 31.
    pub fn phys_PxSetGroup(actor: *mut PxActor, collisionGroup: u16);

    /// Retrieves filtering operation. See comments for PxGroupsMask
    pub fn phys_PxGetFilterOps(op0: *mut PxFilterOp, op1: *mut PxFilterOp, op2: *mut PxFilterOp);

    /// Setups filtering operations. See comments for PxGroupsMask
    pub fn phys_PxSetFilterOps(op0: *const PxFilterOp, op1: *const PxFilterOp, op2: *const PxFilterOp);

    /// Retrieves filtering's boolean value. See comments for PxGroupsMask
    ///
    /// flag Boolean value for filter.
    pub fn phys_PxGetFilterBool() -> bool;

    /// Setups filtering's boolean value. See comments for PxGroupsMask
    pub fn phys_PxSetFilterBool(enable: bool);

    /// Gets filtering constant K0 and K1. See comments for PxGroupsMask
    pub fn phys_PxGetFilterConstants(c0: *mut PxGroupsMask, c1: *mut PxGroupsMask);

    /// Setups filtering's K0 and K1 value. See comments for PxGroupsMask
    pub fn phys_PxSetFilterConstants(c0: *const PxGroupsMask, c1: *const PxGroupsMask);

    /// Gets 64-bit mask used for collision filtering. See comments for PxGroupsMask
    ///
    /// The group mask for the actor.
    pub fn phys_PxGetGroupsMask(actor: *const PxActor) -> PxGroupsMask;

    /// Sets 64-bit mask used for collision filtering. See comments for PxGroupsMask
    pub fn phys_PxSetGroupsMask(actor: *mut PxActor, mask: *const PxGroupsMask);

    pub fn PxDefaultErrorCallback_new_alloc() -> *mut PxDefaultErrorCallback;

    pub fn PxDefaultErrorCallback_delete(self_: *mut PxDefaultErrorCallback);

    pub fn PxDefaultErrorCallback_reportError_mut(self_: *mut PxDefaultErrorCallback, code: PxErrorCode, message: *const std::ffi::c_char, file: *const std::ffi::c_char, line: i32);

    /// Creates a new shape with default properties and a list of materials and adds it to the list of shapes of this actor.
    ///
    /// This is equivalent to the following
    ///
    /// PxShape* shape(...) = PxGetPhysics().createShape(...);	// reference count is 1
    /// actor->attachShape(shape);								// increments reference count
    /// shape->release();										// releases user reference, leaving reference count at 1
    ///
    /// As a consequence, detachShape() will result in the release of the last reference, and the shape will be deleted.
    ///
    /// The default shape flags to be set are: eVISUALIZATION, eSIMULATION_SHAPE, eSCENE_QUERY_SHAPE (see [`PxShapeFlag`]).
    /// Triangle mesh, heightfield or plane geometry shapes configured as eSIMULATION_SHAPE are not supported for
    /// non-kinematic PxRigidDynamic instances.
    ///
    /// Creating compounds with a very large number of shapes may adversely affect performance and stability.
    ///
    /// Sleeping:
    /// Does
    /// NOT
    /// wake the actor up automatically.
    ///
    /// The newly created shape.
    pub fn PxRigidActorExt_createExclusiveShape(actor: *mut PxRigidActor, geometry: *const PxGeometry, materials: *const *mut PxMaterial, materialCount: u16, shapeFlags: PxShapeFlags) -> *mut PxShape;

    /// Creates a new shape with default properties and a single material adds it to the list of shapes of this actor.
    ///
    /// This is equivalent to the following
    ///
    /// PxShape* shape(...) = PxGetPhysics().createShape(...);	// reference count is 1
    /// actor->attachShape(shape);								// increments reference count
    /// shape->release();										// releases user reference, leaving reference count at 1
    ///
    /// As a consequence, detachShape() will result in the release of the last reference, and the shape will be deleted.
    ///
    /// The default shape flags to be set are: eVISUALIZATION, eSIMULATION_SHAPE, eSCENE_QUERY_SHAPE (see [`PxShapeFlag`]).
    /// Triangle mesh, heightfield or plane geometry shapes configured as eSIMULATION_SHAPE are not supported for
    /// non-kinematic PxRigidDynamic instances.
    ///
    /// Creating compounds with a very large number of shapes may adversely affect performance and stability.
    ///
    /// Sleeping:
    /// Does
    /// NOT
    /// wake the actor up automatically.
    ///
    /// The newly created shape.
    pub fn PxRigidActorExt_createExclusiveShape_1(actor: *mut PxRigidActor, geometry: *const PxGeometry, material: *const PxMaterial, shapeFlags: PxShapeFlags) -> *mut PxShape;

    /// Gets a list of bounds based on shapes in rigid actor. This list can be used to cook/create
    /// bounding volume hierarchy though PxCooking API.
    pub fn PxRigidActorExt_getRigidActorShapeLocalBoundsList(actor: *const PxRigidActor, numBounds: *mut u32) -> *mut PxBounds3;

    /// Convenience function to create a PxBVH object from a PxRigidActor.
    ///
    /// The computed PxBVH can then be used in PxScene::addActor() or PxAggregate::addActor().
    /// After adding the actor
    /// &
    /// BVH to the scene/aggregate, release the PxBVH object by calling PxBVH::release().
    ///
    /// The PxBVH for this actor.
    pub fn PxRigidActorExt_createBVHFromActor(physics: *mut PxPhysics, actor: *const PxRigidActor) -> *mut PxBVH;

    /// Compute mass properties of the convex core geometry.
    pub fn PxConvexCoreExt_computeMassInfo(convex: *const PxConvexCoreGeometry, density1Mass: *mut f32, inertiaTensor: *mut PxMat33, centerOfMass: *mut PxVec3);

    /// Visualize the convex core geometry
    pub fn PxConvexCoreExt_visualize(convex: *const PxConvexCoreGeometry, pose: *const PxTransform, drawCore: bool, cullbox: *const PxBounds3, out: *mut PxRenderOutput);

    /// Default constructor.
    pub fn PxMassProperties_new() -> PxMassProperties;

    /// Construct from individual elements.
    pub fn PxMassProperties_new_1(m: f32, inertiaT: *const PxMat33, com: *const PxVec3) -> PxMassProperties;

    /// Compute mass properties based on a provided geometry structure.
    ///
    /// This constructor assumes the geometry has a density of 1. Mass and inertia tensor scale linearly with density.
    pub fn PxMassProperties_new_2(geometry: *const PxGeometry) -> PxMassProperties;

    /// Translate the center of mass by a given vector and adjust the inertia tensor accordingly.
    pub fn PxMassProperties_translate_mut(self_: *mut PxMassProperties, t: *const PxVec3);

    /// Get the entries of the diagonalized inertia tensor and the corresponding reference rotation.
    ///
    /// The entries of the diagonalized inertia tensor.
    pub fn PxMassProperties_getMassSpaceInertia(inertia: *const PxMat33, massFrame: *mut PxQuat) -> PxVec3;

    /// Translate an inertia tensor using the parallel axis theorem
    ///
    /// The translated inertia tensor.
    pub fn PxMassProperties_translateInertia(inertia: *const PxMat33, mass: f32, t: *const PxVec3) -> PxMat33;

    /// Rotate an inertia tensor around the center of mass
    ///
    /// The rotated inertia tensor.
    pub fn PxMassProperties_rotateInertia(inertia: *const PxMat33, q: *const PxQuat) -> PxMat33;

    /// Non-uniform scaling of the inertia tensor
    ///
    /// The scaled inertia tensor.
    pub fn PxMassProperties_scaleInertia(inertia: *const PxMat33, scaleRotation: *const PxQuat, scale: *const PxVec3) -> PxMat33;

    /// Sum up individual mass properties.
    ///
    /// The summed up mass properties.
    pub fn PxMassProperties_sum(props: *const PxMassProperties, transforms: *const PxTransform, count: u32) -> PxMassProperties;

    /// Computation of mass properties for a rigid body actor
    ///
    /// To simulate a dynamic rigid actor, the SDK needs a mass and an inertia tensor.
    ///
    /// This method offers functionality to compute the necessary mass and inertia properties based on the shapes declared in
    /// the PxRigidBody descriptor and some additionally specified parameters. For each shape, the shape geometry,
    /// the shape positioning within the actor and the specified shape density are used to compute the body's mass and
    /// inertia properties.
    ///
    /// Shapes without PxShapeFlag::eSIMULATION_SHAPE set are ignored unless includeNonSimShapes is true.
    /// Shapes with plane, triangle mesh or heightfield geometry and PxShapeFlag::eSIMULATION_SHAPE set are not allowed for PxRigidBody collision.
    ///
    /// This method will set the mass, center of mass, and inertia tensor
    ///
    /// if no collision shapes are found, the inertia tensor is set to (1,1,1) and the mass to 1
    ///
    /// if massLocalPose is non-NULL, the rigid body's center of mass parameter  will be set
    /// to the user provided value (massLocalPose) and the inertia tensor will be resolved at that point.
    ///
    /// If all shapes of the actor have the same density then the overloaded method updateMassAndInertia() with a single density parameter can be used instead.
    ///
    /// Boolean. True on success else false.
    pub fn PxRigidBodyExt_updateMassAndInertia(body: *mut PxRigidBody, shapeDensities: *const f32, shapeDensityCount: u32, massLocalPose: *const PxVec3, includeNonSimShapes: bool) -> bool;

    /// Computation of mass properties for a rigid body actor
    ///
    /// See previous method for details.
    ///
    /// Boolean. True on success else false.
    pub fn PxRigidBodyExt_updateMassAndInertia_1(body: *mut PxRigidBody, density: f32, massLocalPose: *const PxVec3, includeNonSimShapes: bool) -> bool;

    /// Computation of mass properties for a rigid body actor
    ///
    /// This method sets the mass, inertia and center of mass of a rigid body. The mass is set to the sum of all user-supplied
    /// shape mass values, and the inertia and center of mass are computed according to the rigid body's shapes and the per shape mass input values.
    ///
    /// If no collision shapes are found, the inertia tensor is set to (1,1,1)
    ///
    /// If a single mass value should be used for the actor as a whole then the overloaded method setMassAndUpdateInertia() with a single mass parameter can be used instead.
    ///
    /// Boolean. True on success else false.
    pub fn PxRigidBodyExt_setMassAndUpdateInertia(body: *mut PxRigidBody, shapeMasses: *const f32, shapeMassCount: u32, massLocalPose: *const PxVec3, includeNonSimShapes: bool) -> bool;

    /// Computation of mass properties for a rigid body actor
    ///
    /// This method sets the mass, inertia and center of mass of a rigid body. The mass is set to the user-supplied
    /// value, and the inertia and center of mass are computed according to the rigid body's shapes and the input mass.
    ///
    /// If no collision shapes are found, the inertia tensor is set to (1,1,1)
    ///
    /// Boolean. True on success else false.
    pub fn PxRigidBodyExt_setMassAndUpdateInertia_1(body: *mut PxRigidBody, mass: f32, massLocalPose: *const PxVec3, includeNonSimShapes: bool) -> bool;

    /// Compute the mass, inertia tensor and center of mass from a list of shapes.
    ///
    /// The mass properties from the combined shapes.
    pub fn PxRigidBodyExt_computeMassPropertiesFromShapes(shapes: *const *const PxShape, shapeCount: u32) -> PxMassProperties;

    /// Applies a force (or impulse) defined in the global coordinate frame, acting at a particular
    /// point in global coordinates, to the actor.
    ///
    /// Note that if the force does not act along the center of mass of the actor, this
    /// will also add the corresponding torque. Because forces are reset at the end of every timestep,
    /// you can maintain a total external force on an object by calling this once every frame.
    ///
    /// if this call is used to apply a force or impulse to an articulation link, only the link is updated, not the entire
    /// articulation
    ///
    /// ::PxForceMode determines if the force is to be conventional or impulsive. Only eFORCE and eIMPULSE are supported, as the
    /// force required to produce a given velocity change or acceleration is underdetermined given only the desired change at a
    /// given point.
    ///
    /// Sleeping:
    /// This call wakes the actor if it is sleeping and the wakeup parameter is true (default).
    pub fn PxRigidBodyExt_addForceAtPos(body: *mut PxRigidBody, force: *const PxVec3, pos: *const PxVec3, mode: PxForceMode, wakeup: bool);

    /// Applies a force (or impulse) defined in the global coordinate frame, acting at a particular
    /// point in local coordinates, to the actor.
    ///
    /// Note that if the force does not act along the center of mass of the actor, this
    /// will also add the corresponding torque. Because forces are reset at the end of every timestep, you can maintain a
    /// total external force on an object by calling this once every frame.
    ///
    /// if this call is used to apply a force or impulse to an articulation link, only the link is updated, not the entire
    /// articulation
    ///
    /// ::PxForceMode determines if the force is to be conventional or impulsive. Only eFORCE and eIMPULSE are supported, as the
    /// force required to produce a given velocity change or acceleration is underdetermined given only the desired change at a
    /// given point.
    ///
    /// Sleeping:
    /// This call wakes the actor if it is sleeping and the wakeup parameter is true (default).
    pub fn PxRigidBodyExt_addForceAtLocalPos(body: *mut PxRigidBody, force: *const PxVec3, pos: *const PxVec3, mode: PxForceMode, wakeup: bool);

    /// Applies a force (or impulse) defined in the actor local coordinate frame, acting at a
    /// particular point in global coordinates, to the actor.
    ///
    /// Note that if the force does not act along the center of mass of the actor, this
    /// will also add the corresponding torque. Because forces are reset at the end of every timestep, you can maintain a
    /// total external force on an object by calling this once every frame.
    ///
    /// if this call is used to apply a force or impulse to an articulation link, only the link is updated, not the entire
    /// articulation
    ///
    /// ::PxForceMode determines if the force is to be conventional or impulsive. Only eFORCE and eIMPULSE are supported, as the
    /// force required to produce a given velocity change or acceleration is underdetermined given only the desired change at a
    /// given point.
    ///
    /// Sleeping:
    /// This call wakes the actor if it is sleeping and the wakeup parameter is true (default).
    pub fn PxRigidBodyExt_addLocalForceAtPos(body: *mut PxRigidBody, force: *const PxVec3, pos: *const PxVec3, mode: PxForceMode, wakeup: bool);

    /// Applies a force (or impulse) defined in the actor local coordinate frame, acting at a
    /// particular point in local coordinates, to the actor.
    ///
    /// Note that if the force does not act along the center of mass of the actor, this
    /// will also add the corresponding torque. Because forces are reset at the end of every timestep, you can maintain a
    /// total external force on an object by calling this once every frame.
    ///
    /// if this call is used to apply a force or impulse to an articulation link, only the link is updated, not the entire
    /// articulation
    ///
    /// ::PxForceMode determines if the force is to be conventional or impulsive. Only eFORCE and eIMPULSE are supported, as the
    /// force required to produce a given velocity change or acceleration is underdetermined given only the desired change at a
    /// given point.
    ///
    /// Sleeping:
    /// This call wakes the actor if it is sleeping and the wakeup parameter is true (default).
    pub fn PxRigidBodyExt_addLocalForceAtLocalPos(body: *mut PxRigidBody, force: *const PxVec3, pos: *const PxVec3, mode: PxForceMode, wakeup: bool);

    /// Computes the velocity of a point given in world coordinates if it were attached to the
    /// specified body and moving with it.
    ///
    /// The velocity of point in the global frame.
    pub fn PxRigidBodyExt_getVelocityAtPos(body: *const PxRigidBody, pos: *const PxVec3) -> PxVec3;

    /// Computes the velocity of a point given in local coordinates if it were attached to the
    /// specified body and moving with it.
    ///
    /// The velocity of point in the local frame.
    pub fn PxRigidBodyExt_getLocalVelocityAtLocalPos(body: *const PxRigidBody, pos: *const PxVec3) -> PxVec3;

    /// Computes the velocity of a point (offset from the origin of the body) given in world coordinates if it were attached to the
    /// specified body and moving with it.
    ///
    /// The velocity of point (offset from the origin of the body) in the global frame.
    pub fn PxRigidBodyExt_getVelocityAtOffset(body: *const PxRigidBody, pos: *const PxVec3) -> PxVec3;

    /// Compute the change to linear and angular velocity that would occur if an impulsive force and torque were to be applied to a specified rigid body.
    ///
    /// The rigid body is left unaffected unless a subsequent independent call is executed that actually applies the computed changes to velocity and angular velocity.
    ///
    /// if this call is used to determine the velocity delta for an articulation link, only the mass properties of the link are taken into account.
    pub fn PxRigidBodyExt_computeVelocityDeltaFromImpulse(body: *const PxRigidBody, impulsiveForce: *const PxVec3, impulsiveTorque: *const PxVec3, deltaLinearVelocity: *mut PxVec3, deltaAngularVelocity: *mut PxVec3);

    /// Computes the linear and angular velocity change vectors for a given impulse at a world space position taking a mass and inertia scale into account
    ///
    /// This function is useful for extracting the respective linear and angular velocity changes from a contact or joint when the mass/inertia ratios have been adjusted.
    ///
    /// if this call is used to determine the velocity delta for an articulation link, only the mass properties of the link are taken into account.
    pub fn PxRigidBodyExt_computeVelocityDeltaFromImpulse_1(body: *const PxRigidBody, globalPose: *const PxTransform, point: *const PxVec3, impulse: *const PxVec3, invMassScale: f32, invInertiaScale: f32, deltaLinearVelocity: *mut PxVec3, deltaAngularVelocity: *mut PxVec3);

    /// Computes the linear and angular impulse vectors for a given impulse at a world space position taking a mass and inertia scale into account
    ///
    /// This function is useful for extracting the respective linear and angular impulses from a contact or joint when the mass/inertia ratios have been adjusted.
    pub fn PxRigidBodyExt_computeLinearAngularImpulse(body: *const PxRigidBody, globalPose: *const PxTransform, point: *const PxVec3, impulse: *const PxVec3, invMassScale: f32, invInertiaScale: f32, linearImpulse: *mut PxVec3, angularImpulse: *mut PxVec3);

    /// Performs a linear sweep through space with the body's geometry objects.
    ///
    /// Supported geometries are: box, sphere, capsule, convex. Other geometry types will be ignored.
    ///
    /// If eTOUCH is returned from the filter callback, it will trigger an error and the hit will be discarded.
    ///
    /// The function sweeps all shapes attached to a given rigid body through space and reports the nearest
    /// object in the scene which intersects any of of the shapes swept paths.
    /// Information about the closest intersection is written to a [`PxSweepHit`] structure.
    ///
    /// True if a blocking hit was found.
    pub fn PxRigidBodyExt_linearSweepSingle(body: *mut PxRigidBody, scene: *mut PxScene, unitDir: *const PxVec3, distance: f32, outputFlags: PxHitFlags, closestHit: *mut PxSweepHit, shapeIndex: *mut u32, filterData: *const PxQueryFilterData, filterCall: *mut PxQueryFilterCallback, cache: *const PxQueryCache, inflation: f32) -> bool;

    /// Performs a linear sweep through space with the body's geometry objects, returning all overlaps.
    ///
    /// Supported geometries are: box, sphere, capsule, convex. Other geometry types will be ignored.
    ///
    /// This function sweeps all shapes attached to a given rigid body through space and reports all
    /// objects in the scene that intersect any of the shapes' swept paths until there are no more objects to report
    /// or a blocking hit is encountered.
    ///
    /// the number of touching hits. If overflow is set to true, the results are incomplete. In case of overflow there are also no guarantees that all touching hits returned are closer than the blocking hit.
    pub fn PxRigidBodyExt_linearSweepMultiple(body: *mut PxRigidBody, scene: *mut PxScene, unitDir: *const PxVec3, distance: f32, outputFlags: PxHitFlags, touchHitBuffer: *mut PxSweepHit, touchHitShapeIndices: *mut u32, touchHitBufferSize: u32, block: *mut PxSweepHit, blockingShapeIndex: *mut i32, overflow: *mut bool, filterData: *const PxQueryFilterData, filterCall: *mut PxQueryFilterCallback, cache: *const PxQueryCache, inflation: f32) -> u32;

    /// Retrieves the world space pose of the shape.
    ///
    /// Global pose of shape.
    pub fn PxShapeExt_getGlobalPose(shape: *const PxShape, actor: *const PxRigidActor) -> PxTransform;

    /// Raycast test against the shape.
    ///
    /// Number of hits between the ray and the shape
    pub fn PxShapeExt_raycast(shape: *const PxShape, actor: *const PxRigidActor, rayOrigin: *const PxVec3, rayDir: *const PxVec3, maxDist: f32, hitFlags: PxHitFlags, maxHits: u32, rayHits: *mut PxRaycastHit) -> u32;

    /// Test overlap between the shape and a geometry object
    ///
    /// True if the shape overlaps the geometry object
    pub fn PxShapeExt_overlap(shape: *const PxShape, actor: *const PxRigidActor, otherGeom: *const PxGeometry, otherGeomPose: *const PxTransform) -> bool;

    /// Sweep a geometry object against the shape.
    ///
    /// Currently only box, sphere, capsule and convex mesh shapes are supported, i.e. the swept geometry object must be one of those types.
    ///
    /// True if the swept geometry object hits the shape
    pub fn PxShapeExt_sweep(shape: *const PxShape, actor: *const PxRigidActor, unitDir: *const PxVec3, distance: f32, otherGeom: *const PxGeometry, otherGeomPose: *const PxTransform, sweepHit: *mut PxSweepHit, hitFlags: PxHitFlags) -> bool;

    /// Retrieves the axis aligned bounding box enclosing the shape.
    ///
    /// The shape's bounding box.
    pub fn PxShapeExt_getWorldBounds(shape: *const PxShape, actor: *const PxRigidActor, inflation: f32) -> PxBounds3;

    pub fn PxMeshOverlapUtil_new_alloc() -> *mut PxMeshOverlapUtil;

    pub fn PxMeshOverlapUtil_delete(self_: *mut PxMeshOverlapUtil);

    /// Find the mesh triangles which touch the specified geometry object.
    ///
    /// Number of overlaps found. Triangle indices can then be accessed through the [`getResults`]() function.
    pub fn PxMeshOverlapUtil_findOverlap_mut(self_: *mut PxMeshOverlapUtil, geom: *const PxGeometry, geomPose: *const PxTransform, meshGeom: *const PxTriangleMeshGeometry, meshPose: *const PxTransform) -> u32;

    /// Find the height field triangles which touch the specified geometry object.
    ///
    /// Number of overlaps found. Triangle indices can then be accessed through the [`getResults`]() function.
    pub fn PxMeshOverlapUtil_findOverlap_mut_1(self_: *mut PxMeshOverlapUtil, geom: *const PxGeometry, geomPose: *const PxTransform, hfGeom: *const PxHeightFieldGeometry, hfPose: *const PxTransform) -> u32;

    /// Retrieves array of triangle indices after a findOverlap call.
    ///
    /// Indices of touched triangles
    pub fn PxMeshOverlapUtil_getResults(self_: *const PxMeshOverlapUtil) -> *const u32;

    /// Retrieves number of triangle indices after a findOverlap call.
    ///
    /// Number of touched triangles
    pub fn PxMeshOverlapUtil_getNbResults(self_: *const PxMeshOverlapUtil) -> u32;

    /// Computes an approximate minimum translational distance (MTD) between a geometry object and a mesh.
    ///
    /// This iterative function computes an approximate vector that can be used to depenetrate a geom object
    /// from a triangle mesh. Returned depenetration vector should be applied to 'geom', to get out of the mesh.
    ///
    /// The function works best when the amount of overlap between the geom object and the mesh is small. If the
    /// geom object's center goes inside the mesh, backface culling usually kicks in, no overlap is detected,
    /// and the function does not compute an MTD vector.
    ///
    /// The function early exits if no overlap is detected after a depenetration attempt. This means that if
    /// maxIter = N, the code will attempt at most N iterations but it might exit earlier if depenetration has
    /// been successful. Usually N = 4 gives good results.
    ///
    /// True if the MTD has successfully been computed, i.e. if objects do overlap.
    pub fn phys_PxComputeTriangleMeshPenetration(direction: *mut PxVec3, depth: *mut f32, geom: *const PxGeometry, geomPose: *const PxTransform, meshGeom: *const PxTriangleMeshGeometry, meshPose: *const PxTransform, maxIter: u32, usedIter: *mut u32) -> bool;

    /// Computes an approximate minimum translational distance (MTD) between a geometry object and a heightfield.
    ///
    /// This iterative function computes an approximate vector that can be used to depenetrate a geom object
    /// from a heightfield. Returned depenetration vector should be applied to 'geom', to get out of the heightfield.
    ///
    /// The function works best when the amount of overlap between the geom object and the mesh is small. If the
    /// geom object's center goes inside the heightfield, backface culling usually kicks in, no overlap is detected,
    /// and the function does not compute an MTD vector.
    ///
    /// The function early exits if no overlap is detected after a depenetration attempt. This means that if
    /// maxIter = N, the code will attempt at most N iterations but it might exit earlier if depenetration has
    /// been successful. Usually N = 4 gives good results.
    ///
    /// True if the MTD has successfully been computed, i.e. if objects do overlap.
    pub fn phys_PxComputeHeightFieldPenetration(direction: *mut PxVec3, depth: *mut f32, geom: *const PxGeometry, geomPose: *const PxTransform, heightFieldGeom: *const PxHeightFieldGeometry, heightFieldPose: *const PxTransform, maxIter: u32, usedIter: *mut u32) -> bool;

    /// Returns whether the collection is serializable with the externalReferences collection.
    ///
    /// Some definitions to explain whether a collection can be serialized or not:
    ///
    /// For definitions of
    /// requires
    /// and
    /// complete
    /// see [`PxSerialization::complete`]
    ///
    /// A serializable object is
    /// subordinate
    /// if it cannot be serialized on its own
    /// The following objects are subordinate:
    /// - articulation links
    /// - articulation joints
    /// - joints
    ///
    /// A collection C can be serialized with external references collection D iff
    /// - C is complete relative to D (no dangling references)
    /// - Every object in D required by an object in C has a valid ID (no unnamed references)
    /// - Every subordinate object in C is required by another object in C (no orphans)
    ///
    /// Whether the collection is serializable
    pub fn PxSerialization_isSerializable(collection: *mut PxCollection, sr: *mut PxSerializationRegistry, externalReferences: *const PxCollection) -> bool;

    /// Adds to a collection all objects such that it can be successfully serialized.
    ///
    /// A collection C is complete relative to an other collection D if every object required by C is either in C or D.
    /// This function adds objects to a collection, such that it becomes complete with respect to the exceptFor collection.
    /// Completeness is needed for serialization. See [`PxSerialization::serializeCollectionToBinary`],
    /// [`PxSerialization::serializeCollectionToXml`].
    ///
    /// Sdk objects require other sdk object according to the following rules:
    /// - joints require their actors and constraint
    /// - rigid actors require their shapes
    /// - shapes require their material(s) and mesh (triangle mesh, convex mesh or height field), if any
    /// - articulations require their links and joints
    /// - aggregates require their actors
    ///
    /// If followJoints is specified another rule is added:
    /// - actors require their joints
    ///
    /// Specifying followJoints will make whole jointed actor chains being added to the collection. Following chains
    /// is interrupted whenever a object in exceptFor is encountered.
    pub fn PxSerialization_complete(collection: *mut PxCollection, sr: *mut PxSerializationRegistry, exceptFor: *const PxCollection, followJoints: bool);

    /// Creates PxSerialObjectId values for unnamed objects in a collection.
    ///
    /// Creates PxSerialObjectId names for unnamed objects in a collection starting at a base value and incrementing,
    /// skipping values that are already assigned to objects in the collection.
    pub fn PxSerialization_createSerialObjectIds(collection: *mut PxCollection, base: u64);

    /// Deserializes a PxCollection from memory.
    ///
    /// Creates a collection from memory. If the collection has external dependencies another collection
    /// can be provided to resolve these.
    ///
    /// The memory block provided has to be 128 bytes aligned and contain a contiguous serialized collection as written
    /// by PxSerialization::serializeCollectionToBinary. The contained binary data needs to be compatible with the current binary format version
    /// which is defined by "PX_PHYSICS_VERSION_MAJOR.PX_PHYSICS_VERSION_MINOR.PX_PHYSICS_VERSION_BUGFIX-PX_BINARY_SERIAL_VERSION".
    /// For a list of compatible sdk releases refer to the documentation of PX_BINARY_SERIAL_VERSION.
    pub fn PxSerialization_createCollectionFromBinary(memBlock: *mut std::ffi::c_void, sr: *mut PxSerializationRegistry, externalRefs: *const PxCollection) -> *mut PxCollection;

    /// Serializes a collection to a binary stream.
    ///
    /// Serializes a collection to a stream. In order to resolve external dependencies the externalReferences collection has to be provided.
    /// Optionally names of objects that where set for example with [`PxActor::setName`] are serialized along with the objects.
    ///
    /// The collection can be successfully serialized if isSerializable(collection) returns true. See [`isSerializable`].
    ///
    /// The implementation of the output stream needs to fulfill the requirements on the memory block input taken by
    /// PxSerialization::createCollectionFromBinary.
    ///
    /// Serialization of objects in a scene that is simultaneously being simulated is not supported and leads to undefined behavior.
    ///
    /// Whether serialization was successful
    pub fn PxSerialization_serializeCollectionToBinary(outputStream: *mut PxOutputStream, collection: *mut PxCollection, sr: *mut PxSerializationRegistry, externalRefs: *const PxCollection, exportNames: bool) -> bool;

    /// Creates an application managed registry for serialization.
    ///
    /// PxSerializationRegistry instance.
    pub fn PxSerialization_createSerializationRegistry(physics: *mut PxPhysics) -> *mut PxSerializationRegistry;

    /// Deletes the dispatcher.
    ///
    /// Do not keep a reference to the deleted instance.
    pub fn PxDefaultCpuDispatcher_release_mut(self_: *mut PxDefaultCpuDispatcher);

    /// Enables profiling at task level.
    ///
    /// By default enabled only in profiling builds.
    pub fn PxDefaultCpuDispatcher_setRunProfiled_mut(self_: *mut PxDefaultCpuDispatcher, runProfiled: bool);

    /// Checks if profiling is enabled at task level.
    ///
    /// True if tasks should be profiled.
    pub fn PxDefaultCpuDispatcher_getRunProfiled(self_: *const PxDefaultCpuDispatcher) -> bool;

    /// Create default dispatcher, extensions SDK needs to be initialized first.
    ///
    /// numThreads may be zero in which case no worker thread are initialized and
    /// simulation tasks will be executed on the thread that calls PxScene::simulate()
    ///
    /// yieldProcessorCount must be greater than zero if eYIELD_PROCESSOR is the chosen mode and equal to zero for all other modes.
    ///
    /// eYIELD_THREAD and eYIELD_PROCESSOR modes will use compute resources even if the simulation is not running.
    /// It is left to users to keep threads inactive, if so desired, when no simulation is running.
    pub fn phys_PxDefaultCpuDispatcherCreate(numThreads: u32, affinityMasks: *mut u32, mode: PxDefaultCpuDispatcherWaitForWorkMode, yieldProcessorCount: u32) -> *mut PxDefaultCpuDispatcher;

    /// Builds smooth vertex normals over a mesh.
    ///
    /// - "smooth" because smoothing groups are not supported here
    /// - takes angles into account for correct cube normals computation
    ///
    /// To use 32bit indices pass a pointer in dFaces and set wFaces to zero. Alternatively pass a pointer to
    /// wFaces and set dFaces to zero.
    ///
    /// True on success.
    pub fn phys_PxBuildSmoothNormals(nbTris: u32, nbVerts: u32, verts: *const PxVec3, dFaces: *const u32, wFaces: *const u16, normals: *mut PxVec3, flip: bool) -> bool;

    /// simple method to create a PxRigidDynamic actor with a single PxShape.
    ///
    /// a new dynamic actor with the PxRigidBodyFlag, or NULL if it could
    /// not be constructed
    pub fn phys_PxCreateDynamic(sdk: *mut PxPhysics, transform: *const PxTransform, geometry: *const PxGeometry, material: *mut PxMaterial, density: f32, shapeOffset: *const PxTransform) -> *mut PxRigidDynamic;

    /// simple method to create a PxRigidDynamic actor with a single PxShape.
    ///
    /// a new dynamic actor with the PxRigidBodyFlag, or NULL if it could
    /// not be constructed
    pub fn phys_PxCreateDynamic_1(sdk: *mut PxPhysics, transform: *const PxTransform, shape: *mut PxShape, density: f32) -> *mut PxRigidDynamic;

    /// simple method to create a kinematic PxRigidDynamic actor with a single PxShape.
    ///
    /// unlike PxCreateDynamic, the geometry is not restricted to box, capsule, sphere or convex. However,
    /// kinematics of other geometry types may not participate in simulation collision and may be used only for
    /// triggers or scene queries of moving objects under animation control. In this case the density parameter
    /// will be ignored and the created shape will be set up as a scene query only shape (see [`PxShapeFlag::eSCENE_QUERY_SHAPE`])
    ///
    /// a new dynamic actor with the PxRigidBodyFlag::eKINEMATIC set, or NULL if it could
    /// not be constructed
    pub fn phys_PxCreateKinematic(sdk: *mut PxPhysics, transform: *const PxTransform, geometry: *const PxGeometry, material: *mut PxMaterial, density: f32, shapeOffset: *const PxTransform) -> *mut PxRigidDynamic;

    /// simple method to create a kinematic PxRigidDynamic actor with a single PxShape.
    ///
    /// unlike PxCreateDynamic, the geometry is not restricted to box, capsule, sphere or convex. However,
    /// kinematics of other geometry types may not participate in simulation collision and may be used only for
    /// triggers or scene queries of moving objects under animation control. In this case the density parameter
    /// will be ignored and the created shape will be set up as a scene query only shape (see [`PxShapeFlag::eSCENE_QUERY_SHAPE`])
    ///
    /// a new dynamic actor with the PxRigidBodyFlag::eKINEMATIC set, or NULL if it could
    /// not be constructed
    pub fn phys_PxCreateKinematic_1(sdk: *mut PxPhysics, transform: *const PxTransform, shape: *mut PxShape, density: f32) -> *mut PxRigidDynamic;

    /// simple method to create a PxRigidStatic actor with a single PxShape.
    ///
    /// a new static actor, or NULL if it could not be constructed
    pub fn phys_PxCreateStatic(sdk: *mut PxPhysics, transform: *const PxTransform, geometry: *const PxGeometry, material: *mut PxMaterial, shapeOffset: *const PxTransform) -> *mut PxRigidStatic;

    /// simple method to create a PxRigidStatic actor with a single PxShape.
    ///
    /// a new static actor, or NULL if it could not be constructed
    pub fn phys_PxCreateStatic_1(sdk: *mut PxPhysics, transform: *const PxTransform, shape: *mut PxShape) -> *mut PxRigidStatic;

    /// create a shape by copying attributes from another shape
    ///
    /// The function clones a PxShape. The following properties are copied:
    /// - geometry
    /// - flags
    /// - materials
    /// - actor-local pose
    /// - contact offset
    /// - rest offset
    /// - simulation filter data
    /// - query filter data
    /// - torsional patch radius
    /// - minimum torsional patch radius
    ///
    /// The following are not copied and retain their default values:
    /// - name
    /// - user data
    ///
    /// the newly-created rigid static
    pub fn phys_PxCloneShape(physicsSDK: *mut PxPhysics, shape: *const PxShape, isExclusive: bool) -> *mut PxShape;

    /// create a static body by copying attributes from another rigid actor
    ///
    /// The function clones a PxRigidDynamic or PxRigidStatic as a PxRigidStatic. A uniform scale is applied. The following properties are copied:
    /// - shapes
    /// - actor flags
    /// - owner client and client behavior bits
    /// - dominance group
    ///
    /// The following are not copied and retain their default values:
    /// - name
    /// - joints or observers
    /// - aggregate or scene membership
    /// - user data
    ///
    /// Transforms are not copied with bit-exact accuracy.
    ///
    /// the newly-created rigid static
    pub fn phys_PxCloneStatic(physicsSDK: *mut PxPhysics, transform: *const PxTransform, actor: *const PxRigidActor) -> *mut PxRigidStatic;

    /// create a dynamic body by copying attributes from an existing body
    ///
    /// The following properties are copied:
    /// - shapes
    /// - actor flags, rigidDynamic flags and rigidDynamic lock flags
    /// - mass, moment of inertia, and center of mass frame
    /// - linear and angular velocity
    /// - linear and angular damping
    /// - maximum linear velocity
    /// - maximum angular velocity
    /// - position and velocity solver iterations
    /// - maximum depenetration velocity
    /// - sleep threshold
    /// - contact report threshold
    /// - dominance group
    /// - owner client and client behavior bits
    /// - name pointer
    /// - kinematic target
    ///
    /// The following are not copied and retain their default values:
    /// - name
    /// - joints or observers
    /// - aggregate or scene membership
    /// - sleep timer
    /// - user data
    ///
    /// Transforms are not copied with bit-exact accuracy.
    ///
    /// the newly-created rigid static
    pub fn phys_PxCloneDynamic(physicsSDK: *mut PxPhysics, transform: *const PxTransform, body: *const PxRigidDynamic) -> *mut PxRigidDynamic;

    /// create a plane actor. The plane equation is n.x + d = 0
    ///
    /// a new static actor, or NULL if it could not be constructed
    pub fn phys_PxCreatePlane(sdk: *mut PxPhysics, plane: *const PxPlane, material: *mut PxMaterial) -> *mut PxRigidStatic;

    /// scale a rigid actor by a uniform scale
    ///
    /// The geometry and relative positions of the actor are multiplied by the given scale value. If the actor is a rigid body or an
    /// articulation link and the scaleMassProps value is true, the mass properties are scaled assuming the density is constant: the
    /// center of mass is linearly scaled, the mass is multiplied by the cube of the scale, and the inertia tensor by the fifth power of the scale.
    pub fn phys_PxScaleRigidActor(actor: *mut PxRigidActor, scale: f32, scaleMassProps: bool);

    pub fn PxStringTableExt_createStringTable(inAllocator: *mut PxAllocatorCallback) -> *mut PxStringTable;

    /// Creates regions for PxSceneDesc, from a global box.
    ///
    /// This helper simply subdivides the given global box into a 2D grid of smaller boxes. Each one of those smaller boxes
    /// is a region of interest for the broadphase. There are nbSubdiv*nbSubdiv regions in the 2D grid. The function does not
    /// subdivide along the given up axis.
    ///
    /// This is the simplest setup one can use with PxBroadPhaseType::eMBP. A more sophisticated setup would try to cover
    /// the game world with a non-uniform set of regions (i.e. not just a grid).
    ///
    /// number of regions written out to the 'regions' array
    pub fn PxBroadPhaseExt_createRegionsFromWorldBounds(regions: *mut PxBounds3, globalBounds: *const PxBounds3, nbSubdiv: u32, upAxis: u32) -> u32;

    /// Raycast returning any blocking hit, not necessarily the closest.
    ///
    /// Returns whether any rigid actor is hit along the ray.
    ///
    /// Shooting a ray from within an object leads to different results depending on the shape type. Please check the details in article SceneQuery. User can ignore such objects by using one of the provided filter mechanisms.
    ///
    /// True if a blocking hit was found.
    pub fn PxSceneQueryExt_raycastAny(scene: *const PxScene, origin: *const PxVec3, unitDir: *const PxVec3, distance: f32, hit: *mut PxQueryHit, filterData: *const PxQueryFilterData, filterCall: *mut PxQueryFilterCallback, cache: *const PxQueryCache) -> bool;

    /// Raycast returning a single result.
    ///
    /// Returns the first rigid actor that is hit along the ray. Data for a blocking hit will be returned as specified by the outputFlags field. Touching hits will be ignored.
    ///
    /// Shooting a ray from within an object leads to different results depending on the shape type. Please check the details in article SceneQuery. User can ignore such objects by using one of the provided filter mechanisms.
    ///
    /// True if a blocking hit was found.
    pub fn PxSceneQueryExt_raycastSingle(scene: *const PxScene, origin: *const PxVec3, unitDir: *const PxVec3, distance: f32, outputFlags: PxHitFlags, hit: *mut PxRaycastHit, filterData: *const PxQueryFilterData, filterCall: *mut PxQueryFilterCallback, cache: *const PxQueryCache) -> bool;

    /// Raycast returning multiple results.
    ///
    /// Find all rigid actors that get hit along the ray. Each result contains data as specified by the outputFlags field.
    ///
    /// Touching hits are not ordered.
    ///
    /// Shooting a ray from within an object leads to different results depending on the shape type. Please check the details in article SceneQuery. User can ignore such objects by using one of the provided filter mechanisms.
    ///
    /// Number of hits in the buffer, or -1 if the buffer overflowed.
    pub fn PxSceneQueryExt_raycastMultiple(scene: *const PxScene, origin: *const PxVec3, unitDir: *const PxVec3, distance: f32, outputFlags: PxHitFlags, hitBuffer: *mut PxRaycastHit, hitBufferSize: u32, blockingHit: *mut bool, filterData: *const PxQueryFilterData, filterCall: *mut PxQueryFilterCallback, cache: *const PxQueryCache) -> i32;

    /// Sweep returning any blocking hit, not necessarily the closest.
    ///
    /// Returns whether any rigid actor is hit along the sweep path.
    ///
    /// If a shape from the scene is already overlapping with the query shape in its starting position, behavior is controlled by the PxSceneQueryFlag::eINITIAL_OVERLAP flag.
    ///
    /// True if a blocking hit was found.
    pub fn PxSceneQueryExt_sweepAny(scene: *const PxScene, geometry: *const PxGeometry, pose: *const PxTransform, unitDir: *const PxVec3, distance: f32, queryFlags: PxHitFlags, hit: *mut PxQueryHit, filterData: *const PxQueryFilterData, filterCall: *mut PxQueryFilterCallback, cache: *const PxQueryCache, inflation: f32) -> bool;

    /// Sweep returning a single result.
    ///
    /// Returns the first rigid actor that is hit along the ray. Data for a blocking hit will be returned as specified by the outputFlags field. Touching hits will be ignored.
    ///
    /// If a shape from the scene is already overlapping with the query shape in its starting position, behavior is controlled by the PxSceneQueryFlag::eINITIAL_OVERLAP flag.
    ///
    /// True if a blocking hit was found.
    pub fn PxSceneQueryExt_sweepSingle(scene: *const PxScene, geometry: *const PxGeometry, pose: *const PxTransform, unitDir: *const PxVec3, distance: f32, outputFlags: PxHitFlags, hit: *mut PxSweepHit, filterData: *const PxQueryFilterData, filterCall: *mut PxQueryFilterCallback, cache: *const PxQueryCache, inflation: f32) -> bool;

    /// Sweep returning multiple results.
    ///
    /// Find all rigid actors that get hit along the sweep. Each result contains data as specified by the outputFlags field.
    ///
    /// Touching hits are not ordered.
    ///
    /// If a shape from the scene is already overlapping with the query shape in its starting position, behavior is controlled by the PxSceneQueryFlag::eINITIAL_OVERLAP flag.
    ///
    /// Number of hits in the buffer, or -1 if the buffer overflowed.
    pub fn PxSceneQueryExt_sweepMultiple(scene: *const PxScene, geometry: *const PxGeometry, pose: *const PxTransform, unitDir: *const PxVec3, distance: f32, outputFlags: PxHitFlags, hitBuffer: *mut PxSweepHit, hitBufferSize: u32, blockingHit: *mut bool, filterData: *const PxQueryFilterData, filterCall: *mut PxQueryFilterCallback, cache: *const PxQueryCache, inflation: f32) -> i32;

    /// Test overlap between a geometry and objects in the scene.
    ///
    /// Filtering: Overlap tests do not distinguish between touching and blocking hit types. Both get written to the hit buffer.
    ///
    /// PxHitFlag::eMESH_MULTIPLE and PxHitFlag::eMESH_BOTH_SIDES have no effect in this case
    ///
    /// Number of hits in the buffer, or -1 if the buffer overflowed.
    pub fn PxSceneQueryExt_overlapMultiple(scene: *const PxScene, geometry: *const PxGeometry, pose: *const PxTransform, hitBuffer: *mut PxOverlapHit, hitBufferSize: u32, filterData: *const PxQueryFilterData, filterCall: *mut PxQueryFilterCallback) -> i32;

    /// Test returning, for a given geometry, any overlapping object in the scene.
    ///
    /// Filtering: Overlap tests do not distinguish between touching and blocking hit types. Both trigger a hit.
    ///
    /// PxHitFlag::eMESH_MULTIPLE and PxHitFlag::eMESH_BOTH_SIDES have no effect in this case
    ///
    /// True if an overlap was found.
    pub fn PxSceneQueryExt_overlapAny(scene: *const PxScene, geometry: *const PxGeometry, pose: *const PxTransform, hit: *mut PxOverlapHit, filterData: *const PxQueryFilterData, filterCall: *mut PxQueryFilterCallback) -> bool;

    pub fn PxBatchQueryExt_release_mut(self_: *mut PxBatchQueryExt);

    /// Performs a raycast against objects in the scene.
    ///
    /// Touching hits are not ordered.
    ///
    /// Shooting a ray from within an object leads to different results depending on the shape type. Please check the details in article SceneQuery. User can ignore such objects by using one of the provided filter mechanisms.
    ///
    /// This query call writes to a list associated with the query object and is NOT thread safe (for performance reasons there is no lock
    /// and overlapping writes from different threads may result in undefined behavior).
    ///
    /// Returns a PxRaycastBuffer pointer that will store the result of the query after execute() is completed.
    /// This will point either to an element of the buffer allocated on construction or to a user buffer passed to the constructor.
    pub fn PxBatchQueryExt_raycast_mut(self_: *mut PxBatchQueryExt, origin: *const PxVec3, unitDir: *const PxVec3, distance: f32, maxNbTouches: u16, hitFlags: PxHitFlags, filterData: *const PxQueryFilterData, cache: *const PxQueryCache) -> *mut PxRaycastBuffer;

    /// Performs a sweep test against objects in the scene.
    ///
    /// Touching hits are not ordered.
    ///
    /// If a shape from the scene is already overlapping with the query shape in its starting position,
    /// the hit is returned unless eASSUME_NO_INITIAL_OVERLAP was specified.
    ///
    /// This query call writes to a list associated with the query object and is NOT thread safe (for performance reasons there is no lock
    /// and overlapping writes from different threads may result in undefined behavior).
    ///
    /// Returns a PxSweepBuffer pointer that will store the result of the query after execute() is completed.
    /// This will point either to an element of the buffer allocated on construction or to a user buffer passed to the constructor.
    pub fn PxBatchQueryExt_sweep_mut(self_: *mut PxBatchQueryExt, geometry: *const PxGeometry, pose: *const PxTransform, unitDir: *const PxVec3, distance: f32, maxNbTouches: u16, hitFlags: PxHitFlags, filterData: *const PxQueryFilterData, cache: *const PxQueryCache, inflation: f32) -> *mut PxSweepBuffer;

    /// Performs an overlap test of a given geometry against objects in the scene.
    ///
    /// Filtering: returning eBLOCK from user filter for overlap queries will cause a warning (see [`PxQueryHitType`]).
    ///
    /// eBLOCK should not be returned from user filters for overlap(). Doing so will result in undefined behavior, and a warning will be issued.
    ///
    /// If the PxQueryFlag::eNO_BLOCK flag is set, the eBLOCK will instead be automatically converted to an eTOUCH and the warning suppressed.
    ///
    /// This query call writes to a list associated with the query object and is NOT thread safe (for performance reasons there is no lock
    /// and overlapping writes from different threads may result in undefined behavior).
    ///
    /// Returns a PxOverlapBuffer pointer that will store the result of the query after execute() is completed.
    /// This will point either to an element of the buffer allocated on construction or to a user buffer passed to the constructor.
    pub fn PxBatchQueryExt_overlap_mut(self_: *mut PxBatchQueryExt, geometry: *const PxGeometry, pose: *const PxTransform, maxNbTouches: u16, filterData: *const PxQueryFilterData, cache: *const PxQueryCache) -> *mut PxOverlapBuffer;

    pub fn PxBatchQueryExt_execute_mut(self_: *mut PxBatchQueryExt);

    /// Create a PxBatchQueryExt without the need for pre-allocated result or touch buffers.
    ///
    /// Returns a PxBatchQueryExt instance. A NULL pointer will be returned if the subsequent allocations fail or if any of the arguments are illegal.
    /// In the event that a NULL pointer is returned a corresponding error will be issued to the error stream.
    pub fn phys_PxCreateBatchQueryExt(scene: *const PxScene, queryFilterCallback: *mut PxQueryFilterCallback, maxNbRaycasts: u32, maxNbRaycastTouches: u32, maxNbSweeps: u32, maxNbSweepTouches: u32, maxNbOverlaps: u32, maxNbOverlapTouches: u32) -> *mut PxBatchQueryExt;

    /// Create a PxBatchQueryExt with user-supplied result and touch buffers.
    ///
    /// Returns a PxBatchQueryExt instance. A NULL pointer will be returned if the subsequent allocations fail or if any of the arguments are illegal.
    /// In the event that a NULL pointer is returned a corresponding error will be issued to the error stream.
    pub fn phys_PxCreateBatchQueryExt_1(scene: *const PxScene, queryFilterCallback: *mut PxQueryFilterCallback, raycastBuffers: *mut PxRaycastBuffer, maxNbRaycasts: u32, raycastTouches: *mut PxRaycastHit, maxNbRaycastTouches: u32, sweepBuffers: *mut PxSweepBuffer, maxNbSweeps: u32, sweepTouches: *mut PxSweepHit, maxNbSweepTouches: u32, overlapBuffers: *mut PxOverlapBuffer, maxNbOverlaps: u32, overlapTouches: *mut PxOverlapHit, maxNbOverlapTouches: u32) -> *mut PxBatchQueryExt;

    /// Creates an external scene query system.
    ///
    /// An external SQ system is the part of a PxScene that deals with scene queries (SQ). This is usually taken care of
    /// by an internal implementation inside PxScene, but it is also possible to re-route all SQ calls to an external
    /// implementation, potentially opening the door to some customizations in behavior and features for advanced users.
    ///
    /// The following external SQ system is an example of how an implementation would look like. It re-uses much of the
    /// same code as the internal version, but it could be re-implemented in a completely different way to match users'
    /// specific needs.
    ///
    /// An external SQ system instance
    pub fn phys_PxCreateExternalSceneQuerySystem(desc: *const PxSceneQueryDesc, contextID: u64) -> *mut PxSceneQuerySystem;

    pub fn PxCustomSceneQuerySystem_delete(self_: *mut PxCustomSceneQuerySystem);

    /// Adds a pruner to the system.
    ///
    /// The internal PhysX scene-query system uses two regular pruners (one for static shapes, one for dynamic shapes) and an optional
    /// compound pruner. Our custom scene query system supports an arbitrary number of regular pruners.
    ///
    /// This can be useful to reduce the load on each pruner, in particular during updates, when internal trees are rebuilt in the
    /// background. On the other hand this implementation simply iterates over all created pruners to perform queries, so their cost
    /// might increase if a large number of pruners is used.
    ///
    /// In any case this serves as an example of how the PxSceneQuerySystem API can be used to customize scene queries.
    ///
    /// A pruner index
    pub fn PxCustomSceneQuerySystem_addPruner_mut(self_: *mut PxCustomSceneQuerySystem, primaryType: PxPruningStructureType, secondaryType: PxDynamicTreeSecondaryPruner, preallocated: u32) -> u32;

    /// Start custom build-steps for all pruners
    ///
    /// This function is used in combination with customBuildstep() and finishCustomBuildstep() to let users take control
    /// of the pruners' build-step
    /// &
    /// commit calls - basically the pruners' update functions. These functions should be used
    /// with the PxSceneQueryUpdateMode::eBUILD_DISABLED_COMMIT_DISABLED update mode, otherwise the build-steps will happen
    /// automatically in fetchResults. For N pruners it can be more efficient to use these custom build-step functions to
    /// perform the updates in parallel:
    ///
    /// - call startCustomBuildstep() first (one synchronous call)
    /// - for each pruner, call customBuildstep() (asynchronous calls from multiple threads)
    /// - once it is done, call finishCustomBuildstep() to finish the update (synchronous call)
    ///
    /// The multi-threaded update is more efficient here than what it is in PxScene, because the "flushShapes()" call is
    /// also multi-threaded (while it is not in PxScene).
    ///
    /// Note that users are responsible for locks here, and these calls should not overlap with other SQ calls. In particular
    /// one should not add new objects to the SQ system or perform queries while these calls are happening.
    ///
    /// The number of pruners in the system.
    pub fn PxCustomSceneQuerySystem_startCustomBuildstep_mut(self_: *mut PxCustomSceneQuerySystem) -> u32;

    /// Perform a custom build-step for a given pruner.
    pub fn PxCustomSceneQuerySystem_customBuildstep_mut(self_: *mut PxCustomSceneQuerySystem, index: u32);

    /// Finish custom build-steps
    ///
    /// Call this function once after all the customBuildstep() calls are done.
    pub fn PxCustomSceneQuerySystem_finishCustomBuildstep_mut(self_: *mut PxCustomSceneQuerySystem);

    pub fn PxCustomSceneQuerySystemAdapter_delete(self_: *mut PxCustomSceneQuerySystemAdapter);

    /// Gets a pruner index for an actor/shape.
    ///
    /// This user-defined function tells the system in which pruner a given actor/shape should go.
    ///
    /// The returned index must be valid, i.e. it must have been previously returned to users by PxCustomSceneQuerySystem::addPruner.
    ///
    /// A pruner index for this actor/shape.
    pub fn PxCustomSceneQuerySystemAdapter_getPrunerIndex(self_: *const PxCustomSceneQuerySystemAdapter, actor: *const PxRigidActor, shape: *const PxShape) -> u32;

    /// Pruner filtering callback.
    ///
    /// This will be called for each query to validate whether it should process a given pruner.
    ///
    /// True to process the pruner, false to skip it entirely
    pub fn PxCustomSceneQuerySystemAdapter_processPruner(self_: *const PxCustomSceneQuerySystemAdapter, prunerIndex: u32, context: *const PxQueryThreadContext, filterData: *const PxQueryFilterData, filterCall: *mut PxQueryFilterCallback) -> bool;

    /// Creates a custom scene query system.
    ///
    /// This is similar to PxCreateExternalSceneQuerySystem, except this function creates a PxCustomSceneQuerySystem object.
    /// It can be plugged to PxScene the same way, via PxSceneDesc::sceneQuerySystem.
    ///
    /// A custom SQ system instance
    pub fn phys_PxCreateCustomSceneQuerySystem(sceneQueryUpdateMode: PxSceneQueryUpdateMode, contextID: u64, adapter: *const PxCustomSceneQuerySystemAdapter, usesTreeOfPruners: bool) -> *mut PxCustomSceneQuerySystem;

    /// Computes closest polygon of the convex hull geometry for a given impact point
    /// and impact direction. When doing sweeps against a scene, one might want to delay
    /// the rather expensive computation of the hit face index for convexes until it is clear
    /// the information is really needed and then use this method to get the corresponding
    /// face index.
    ///
    /// Closest face index of the convex geometry.
    pub fn phys_PxFindFaceIndex(convexGeom: *const PxConvexMeshGeometry, geomPose: *const PxTransform, impactPos: *const PxVec3, unitDir: *const PxVec3) -> u32;

    /// Sets the sampling radius
    ///
    /// Returns true if the sampling was successful and false if there was a problem. Usually an internal overflow is the problem for very big meshes or very small sampling radii.
    pub fn PxPoissonSampler_setSamplingRadius_mut(self_: *mut PxPoissonSampler, samplingRadius: f32) -> bool;

    /// Adds new Poisson Samples inside the sphere specified
    pub fn PxPoissonSampler_addSamplesInSphere_mut(self_: *mut PxPoissonSampler, sphereCenter: *const PxVec3, sphereRadius: f32, createVolumeSamples: bool);

    /// Adds new Poisson Samples inside the box specified
    pub fn PxPoissonSampler_addSamplesInBox_mut(self_: *mut PxPoissonSampler, axisAlignedBox: *const PxBounds3, boxOrientation: *const PxQuat, createVolumeSamples: bool);

    pub fn PxPoissonSampler_delete(self_: *mut PxPoissonSampler);

    /// Creates a shape sampler
    ///
    /// Returns the sampler
    pub fn phys_PxCreateShapeSampler(geometry: *const PxGeometry, transform: *const PxTransform, worldBounds: *const PxBounds3, initialSamplingRadius: f32, numSampleAttemptsAroundPoint: i32) -> *mut PxPoissonSampler;

    /// Checks whether a point is inside the triangle mesh
    ///
    /// Returns true if the point is inside the triangle mesh
    pub fn PxTriangleMeshPoissonSampler_isPointInTriangleMesh_mut(self_: *mut PxTriangleMeshPoissonSampler, p: *const PxVec3) -> bool;

    pub fn PxTriangleMeshPoissonSampler_delete(self_: *mut PxTriangleMeshPoissonSampler);

    /// Creates a triangle mesh sampler
    ///
    /// Returns the sampler
    pub fn phys_PxCreateTriangleMeshSampler(triangles: *const u32, numTriangles: u32, vertices: *const PxVec3, numVertices: u32, initialSamplingRadius: f32, numSampleAttemptsAroundPoint: i32) -> *mut PxTriangleMeshPoissonSampler;

    /// Returns the index of the tetrahedron that contains a point
    ///
    /// The index of the tetrahedron containing the point, -1 if not tetrahedron contains the opoint
    pub fn PxTetrahedronMeshExt_findTetrahedronContainingPoint(mesh: *const PxTetrahedronMesh, point: *const PxVec3, bary: *mut PxVec4, tolerance: f32) -> i32;

    /// Returns the index of the tetrahedron closest to a point
    ///
    /// The index of the tetrahedron closest to the point
    pub fn PxTetrahedronMeshExt_findTetrahedronClosestToPoint(mesh: *const PxTetrahedronMesh, point: *const PxVec3, bary: *mut PxVec4) -> i32;

    /// Distributes a list of triangles masses to vertices.
    ///
    /// The mass for each triangle will be distributed in equal parts to the vertices of said triangle.
    pub fn PxDeformableSurfaceExt_distributeTriangleMassToVertices(deformableSurface: *mut PxDeformableSurface, triangleMasses: *const f32, positionInvMassPinned: *mut PxVec4);

    /// Distributes a uniform density to the vertices of a deformable surface.
    ///
    /// This method distributes mass based on a specified mass per unit area. The mass for each vertex is calculated
    /// according to the area of the triangles connected to it, and the resulting mass is assigned to the vertex.
    pub fn PxDeformableSurfaceExt_distributeDensityToVertices(deformableSurface: *mut PxDeformableSurface, massPerVolume: f32, clothThickness: f32, positionInvMassPinned: *mut PxVec4);

    /// Distributes a total mass uniformly to the vertices of a deformable surface.
    ///
    /// This method calculates the total mass to be distributed across all vertices, and assigns a proportional mass to each
    /// vertex based on the geometry of the surface. The mass is distributed equally to ensure the total mass of the surface
    /// matches the specified value.
    pub fn PxDeformableSurfaceExt_distributeMassToVertices(deformableSurface: *mut PxDeformableSurface, totalMass: f32, positionInvMassPinned: *mut PxVec4);

    /// Allocates and initializes a pinned host memory from a PxTriangleMesh attached to a PxDeformableSurface using a PxShape.
    ///
    /// The user is responsible for deallocation and lifetime management of the positionInvMassPinned, velocityPinned
    /// and restPositionsPinned buffers.
    ///
    /// This method fails if the deformable surface does not have a shape attached to it.
    ///
    /// The number of vertices in the surface deformable mesh.
    pub fn PxDeformableSurfaceExt_allocateAndInitializeHostMirror(deformableSurface: *mut PxDeformableSurface, positions: *const PxVec3, velocities: *const PxVec3, restPositions: *const PxVec3, mass: f32, transform: *const PxTransform, cudaContextManager: *mut PxCudaContextManager, positionInvMassPinned: *mut *mut PxVec4, velocityPinned: *mut *mut PxVec4, restPositionPinned: *mut *mut PxVec4) -> u32;

    /// Allocates and initializes a pinned host memory from given positions, velocities, and rest positions.
    ///
    /// The user is responsible for deallocation and lifetime management of the positionInvMassPinned, velocityPinned
    /// and restPositionsPinned buffers.
    ///
    /// If the input 'restPositions' is a null pointer, positions are used in place of restPositions.
    /// If the input 'velocities' is a null pointer, zero velocities are assigned to velocityPinned.
    ///
    /// The number of vertices in the surface deformable mesh.
    pub fn PxDeformableSurfaceExt_allocateAndInitializeHostMirror_1(positions: *const PxVec3, velocities: *const PxVec3, restPositions: *const PxVec3, nbVertices: u32, mass: f32, transform: *const PxTransform, cudaContextManager: *mut PxCudaContextManager, positionInvMassPinned: *mut *mut PxVec4, velocityPinned: *mut *mut PxVec4, restPositionPinned: *mut *mut PxVec4) -> u32;

    /// Initialize the PhysXExtensions library.
    ///
    /// This should be called before calling any functions or methods in extensions which may require allocation.
    ///
    /// This function does not need to be called before creating a PxDefaultAllocator object.
    pub fn phys_PxInitExtensions(physics: *mut PxPhysics, pvd: *mut PxPvd) -> bool;

    /// Shut down the PhysXExtensions library.
    ///
    /// This function should be called to cleanly shut down the PhysXExtensions library before application exit.
    ///
    /// This function is required to be called to release foundation usage.
    pub fn phys_PxCloseExtensions();

    pub fn PxRepXObject_new(inTypeName: *const std::ffi::c_char, inSerializable: *const std::ffi::c_void, inId: u64) -> PxRepXObject;

    pub fn PxRepXObject_isValid(self_: *const PxRepXObject) -> bool;

    pub fn PxRepXInstantiationArgs_new(inPhysics: *mut PxPhysics, inCooking: *const PxCookingParams, inStringTable: *mut PxStringTable) -> PxRepXInstantiationArgs;

    /// The type this Serializer is meant to operate on.
    pub fn PxRepXSerializer_getTypeName_mut(self_: *mut PxRepXSerializer) -> *const std::ffi::c_char;

    /// Convert from a RepX object to a key-value pair hierarchy
    pub fn PxRepXSerializer_objectToFile_mut(self_: *mut PxRepXSerializer, inLiveObject: *const PxRepXObject, inCollection: *mut PxCollection, inWriter: *mut XmlWriter, inTempBuffer: *mut MemoryBuffer, inArgs: *mut PxRepXInstantiationArgs);

    /// Convert from a descriptor to a live object.  Must be an object of this Serializer type.
    ///
    /// The new live object.  It can be an invalid object if the instantiation cannot take place.
    pub fn PxRepXSerializer_fileToObject_mut(self_: *mut PxRepXSerializer, inReader: *mut XmlReader, inAllocator: *mut XmlMemoryAllocator, inArgs: *mut PxRepXInstantiationArgs, inCollection: *mut PxCollection) -> PxRepXObject;

    pub fn PxVehicleComponent_delete(self_: *mut PxVehicleComponent);

    /// Update function for a vehicle component.
    ///
    /// True if subsequent components in a sequence should get updated, false if the sequence should
    /// be aborted.
    pub fn PxVehicleComponent_update_mut(self_: *mut PxVehicleComponent, dt: f32, context: *const PxVehicleSimulationContext) -> bool;

    pub fn PxVehicleComponentSequence_new() -> PxVehicleComponentSequence;

    /// Add a component to the sequence.
    ///
    /// True on success, else false (for example due to component count limit being reached).
    pub fn PxVehicleComponentSequence_add_mut(self_: *mut PxVehicleComponentSequence, component: *mut PxVehicleComponent) -> bool;

    /// Start a substepping group.
    ///
    /// All components added using [`add`]() will be added to the new substepping group until either the group
    /// is marked as complete with a call to [`endSubstepGroup`]() or a subsequent substepping group is started with
    /// a call to [`beginSubstepGroup`]().
    ///
    /// Groups can be nested with stacked calls to [`beginSubstepGroup`]().
    ///
    /// Each group opened by [`beginSubstepGroup`]() must be closed with a complementary #endSubstepGroup() prior to calling #update().
    ///
    /// Handle for the substepping group on success, else eINVALID_SUBSTEP_GROUP
    pub fn PxVehicleComponentSequence_beginSubstepGroup_mut(self_: *mut PxVehicleComponentSequence, nbSubSteps: u8) -> u8;

    /// End a substepping group
    ///
    /// The group most recently opened with [`beginSubstepGroup`]() will be closed by this call.
    pub fn PxVehicleComponentSequence_endSubstepGroup_mut(self_: *mut PxVehicleComponentSequence);

    /// Set the number of substeps to perform  for a specific substepping group.
    pub fn PxVehicleComponentSequence_setSubsteps_mut(self_: *mut PxVehicleComponentSequence, subGroupHandle: u8, nbSteps: u8);

    /// Update each component in the sequence.
    ///
    /// If the update method of a component in the sequence returns false, the update process gets aborted.
    pub fn PxVehicleComponentSequence_update_mut(self_: *mut PxVehicleComponentSequence, dt: f32, context: *const PxVehicleSimulationContext);

    pub fn PxVehicleAxleDescription_setToDefault_mut(self_: *mut PxVehicleAxleDescription);

    /// Add an axle to the vehicle by specifying the number of wheels on the axle and an array of wheel ids specifying each wheel on the axle.
    pub fn PxVehicleAxleDescription_addAxle_mut(self_: *mut PxVehicleAxleDescription, nbWheelsOnAxle: u32, wheelIdsOnAxle: *const u32);

    /// Return the number of axles on the vehicle.
    ///
    /// The number of axles.
    pub fn PxVehicleAxleDescription_getNbAxles(self_: *const PxVehicleAxleDescription) -> u32;

    /// Return the number of wheels on the ith axle.
    ///
    /// The number of wheels on the specified axle.
    pub fn PxVehicleAxleDescription_getNbWheelsOnAxle(self_: *const PxVehicleAxleDescription, i: u32) -> u32;

    /// Return the wheel id of the jth wheel on the ith axle.
    ///
    /// The wheel id of the jth wheel on the ith axle.
    pub fn PxVehicleAxleDescription_getWheelOnAxle(self_: *const PxVehicleAxleDescription, j: u32, i: u32) -> u32;

    /// Return the number of wheels on the vehicle.
    ///
    /// The number of wheels.
    pub fn PxVehicleAxleDescription_getNbWheels(self_: *const PxVehicleAxleDescription) -> u32;

    /// Return the axle of a specified wheel.
    ///
    /// The axle of the specified wheel.
    pub fn PxVehicleAxleDescription_getAxle(self_: *const PxVehicleAxleDescription, wheelId: u32) -> u32;

    pub fn PxVehicleAxleDescription_isValid(self_: *const PxVehicleAxleDescription) -> bool;

    pub fn PxVehicleAxleDescription_new() -> PxVehicleAxleDescription;

    pub fn PxVehicleFrame_setToDefault_mut(self_: *mut PxVehicleFrame);

    pub fn PxVehicleFrame_getFrame(self_: *const PxVehicleFrame) -> PxMat33;

    pub fn PxVehicleFrame_getLngAxis(self_: *const PxVehicleFrame) -> PxVec3;

    pub fn PxVehicleFrame_getLatAxis(self_: *const PxVehicleFrame) -> PxVec3;

    pub fn PxVehicleFrame_getVrtAxis(self_: *const PxVehicleFrame) -> PxVec3;

    pub fn PxVehicleFrame_isValid(self_: *const PxVehicleFrame) -> bool;

    pub fn PxVehicleFrame_new() -> PxVehicleFrame;

    pub fn PxVehicleScale_setToDefault_mut(self_: *mut PxVehicleScale);

    pub fn PxVehicleScale_isValid(self_: *const PxVehicleScale) -> bool;

    pub fn PxVehicleScale_new() -> PxVehicleScale;

    pub fn PxVehicleTireSlipParams_setToDefault_mut(self_: *mut PxVehicleTireSlipParams);

    pub fn PxVehicleTireSlipParams_transformAndScale(self_: *const PxVehicleTireSlipParams, srcFrame: *const PxVehicleFrame, trgFrame: *const PxVehicleFrame, srcScale: *const PxVehicleScale, trgScale: *const PxVehicleScale) -> PxVehicleTireSlipParams;

    pub fn PxVehicleTireSlipParams_isValid(self_: *const PxVehicleTireSlipParams) -> bool;

    pub fn PxVehicleTireSlipParams_new() -> PxVehicleTireSlipParams;

    pub fn PxVehicleTireAxisStickyParams_transformAndScale(self_: *const PxVehicleTireAxisStickyParams, srcFrame: *const PxVehicleFrame, trgFrame: *const PxVehicleFrame, srcScale: *const PxVehicleScale, trgScale: *const PxVehicleScale) -> PxVehicleTireAxisStickyParams;

    pub fn PxVehicleTireAxisStickyParams_isValid(self_: *const PxVehicleTireAxisStickyParams) -> bool;

    pub fn PxVehicleTireAxisStickyParams_new() -> PxVehicleTireAxisStickyParams;

    pub fn PxVehicleTireStickyParams_setToDefault_mut(self_: *mut PxVehicleTireStickyParams);

    pub fn PxVehicleTireStickyParams_transformAndScale(self_: *const PxVehicleTireStickyParams, srcFrame: *const PxVehicleFrame, trgFrame: *const PxVehicleFrame, srcScale: *const PxVehicleScale, trgScale: *const PxVehicleScale) -> PxVehicleTireStickyParams;

    pub fn PxVehicleTireStickyParams_isValid(self_: *const PxVehicleTireStickyParams) -> bool;

    pub fn PxVehicleTireStickyParams_new() -> PxVehicleTireStickyParams;

    pub fn PxVehiclePvdContext_setToDefault_mut(self_: *mut PxVehiclePvdContext);

    pub fn PxVehiclePvdContext_new() -> PxVehiclePvdContext;

    pub fn PxVehicleSimulationContext_new() -> PxVehicleSimulationContext;

    pub fn PxVehicleSimulationContext_getType(self_: *const PxVehicleSimulationContext) -> PxVehicleSimulationContextType;

    pub fn PxVehicleSimulationContext_setToDefault_mut(self_: *mut PxVehicleSimulationContext);

    pub fn PxVehicleSimulationContext_transformAndScale(self_: *const PxVehicleSimulationContext, srcFrame: *const PxVehicleFrame, trgFrame: *const PxVehicleFrame, srcScale: *const PxVehicleScale, trgScale: *const PxVehicleScale) -> PxVehicleSimulationContext;

    pub fn PxVehiclePhysXSimulationContext_new() -> PxVehiclePhysXSimulationContext;

    pub fn PxVehiclePhysXSimulationContext_setToDefault_mut(self_: *mut PxVehiclePhysXSimulationContext);

    pub fn PxVehiclePhysXSimulationContext_transformAndScale(self_: *const PxVehiclePhysXSimulationContext, srcFrame: *const PxVehicleFrame, trgFrame: *const PxVehicleFrame, srcScale: *const PxVehicleScale, trgScale: *const PxVehicleScale) -> PxVehiclePhysXSimulationContext;

    pub fn PxVehicleRoadGeometryState_setToDefault_mut(self_: *mut PxVehicleRoadGeometryState);

    pub fn PxVehicleRigidBodyState_setToDefault_mut(self_: *mut PxVehicleRigidBodyState);

    /// Compute the vertical speed of the rigid body transformed to the world frame.
    pub fn PxVehicleRigidBodyState_getVerticalSpeed(self_: *const PxVehicleRigidBodyState, frame: *const PxVehicleFrame) -> f32;

    /// Compute the lateral speed of the rigid body transformed to the world frame.
    pub fn PxVehicleRigidBodyState_getLateralSpeed(self_: *const PxVehicleRigidBodyState, frame: *const PxVehicleFrame) -> f32;

    /// Compute the longitudinal speed of the rigid body transformed to the world frame.
    pub fn PxVehicleRigidBodyState_getLongitudinalSpeed(self_: *const PxVehicleRigidBodyState, frame: *const PxVehicleFrame) -> f32;

    pub fn PxVehiclePhysXRoadGeometryQueryState_setToDefault_mut(self_: *mut PxVehiclePhysXRoadGeometryQueryState);

    pub fn PxVehiclePhysXActor_setToDefault_mut(self_: *mut PxVehiclePhysXActor);

    pub fn PxVehiclePhysXSteerState_setToDefault_mut(self_: *mut PxVehiclePhysXSteerState);

    pub fn phys_PxVehicleTransformFrameToFrame(srcFrame: *const PxVehicleFrame, trgFrame: *const PxVehicleFrame, v: *const PxVec3) -> PxVec3;

    pub fn phys_PxVehicleTransformFrameToFrame_1(srcFrame: *const PxVehicleFrame, trgFrame: *const PxVehicleFrame, srcScale: *const PxVehicleScale, trgScale: *const PxVehicleScale, v: *const PxVec3) -> PxVec3;

    pub fn phys_PxVehicleTransformFrameToFrame_2(srcFrame: *const PxVehicleFrame, trgFrame: *const PxVehicleFrame, srcScale: *const PxVehicleScale, trgScale: *const PxVehicleScale, v: *const PxTransform) -> PxTransform;

    pub fn phys_PxVehicleComputeTranslation(frame: *const PxVehicleFrame, lng: f32, lat: f32, vrt: f32) -> PxVec3;

    pub fn phys_PxVehicleComputeRotation(frame: *const PxVehicleFrame, roll: f32, pitch: f32, yaw: f32) -> PxQuat;

    pub fn phys_PxVehicleComputeSign(f: f32) -> f32;

    /// Shift the origin of a vehicle by the specified vector.
    ///
    /// Call this method to adjust the internal data structures of vehicles to reflect the shifted origin location
    /// (the shift vector will get subtracted from all world space spatial data).
    ///
    /// It is the user's responsibility to keep track of the summed total origin shift and adjust all input/output to/from the vehicle accordingly.
    ///
    /// This call will not automatically shift the PhysX scene and its objects. PxScene::shiftOrigin() must be called separately to keep the systems in sync.
    ///
    /// If there is no associated PxRigidActor then set physxActor to NULL.
    ///
    /// If there is an associated PxRigidActor and it is already in a PxScene then the complementary call to PxScene::shiftOrigin() will take care of
    /// shifting the associated PxRigidActor.  This being the case, set physxActor to NULL.  physxActor should be a non-NULL pointer only when there is an
    /// associated PxRigidActor and it is not part of a PxScene.  This can occur if the associated PxRigidActor is updated using PhysX immediate mode.
    ///
    /// If scene queries are independent of PhysX geometry then set queryStates to NULL.
    pub fn phys_PxVehicleShiftOrigin(axleDesc: *const PxVehicleAxleDescription, shift: *const PxVec3, rigidBodyState: *mut PxVehicleRigidBodyState, roadGeometryStates: *mut PxVehicleRoadGeometryState, physxActor: *mut PxVehiclePhysXActor, physxQueryStates: *mut PxVehiclePhysXRoadGeometryQueryState);

    pub fn PxVehicleVectorN_new_alloc(size: u32) -> *mut PxVehicleVectorN;

    pub fn PxVehicleVectorN_delete(self_: *mut PxVehicleVectorN);

    pub fn PxVehicleVectorN_getSize(self_: *const PxVehicleVectorN) -> u32;

    pub fn PxVehicleMatrixNN_new_alloc() -> *mut PxVehicleMatrixNN;

    pub fn PxVehicleMatrixNN_new_alloc_1(size: u32) -> *mut PxVehicleMatrixNN;

    pub fn PxVehicleMatrixNN_delete(self_: *mut PxVehicleMatrixNN);

    pub fn PxVehicleMatrixNN_get(self_: *const PxVehicleMatrixNN, i: u32, j: u32) -> f32;

    pub fn PxVehicleMatrixNN_set_mut(self_: *mut PxVehicleMatrixNN, i: u32, j: u32, val: f32);

    pub fn PxVehicleMatrixNN_getSize(self_: *const PxVehicleMatrixNN) -> u32;

    pub fn PxVehicleMatrixNN_setSize_mut(self_: *mut PxVehicleMatrixNN, size: u32);

    pub fn PxVehicleMatrixNNLUSolver_new_alloc() -> *mut PxVehicleMatrixNNLUSolver;

    pub fn PxVehicleMatrixNNLUSolver_delete(self_: *mut PxVehicleMatrixNNLUSolver);

    pub fn PxVehicleMatrixNNLUSolver_getDet(self_: *const PxVehicleMatrixNNLUSolver) -> f32;

    pub fn PxVehicleMatrixNNLUSolver_decomposeLU_mut(self_: *mut PxVehicleMatrixNNLUSolver, A: *const PxVehicleMatrixNN);

    pub fn PxVehicleMatrixNNLUSolver_solve(self_: *const PxVehicleMatrixNNLUSolver, b: *const PxVehicleVectorN, x: *mut PxVehicleVectorN) -> bool;

    pub fn PxVehicleMatrixNGaussSeidelSolver_solve(self_: *const PxVehicleMatrixNGaussSeidelSolver, maxIterations: u32, tolerance: f32, A: *const PxVehicleMatrixNN, b: *const PxVehicleVectorN, result: *mut PxVehicleVectorN);

    pub fn PxVehicleMatrix33Solver_solve(self_: *const PxVehicleMatrix33Solver, A_: *const PxVehicleMatrixNN, b_: *const PxVehicleVectorN, result: *mut PxVehicleVectorN) -> bool;

    pub fn PxVehicleCommandValueResponseTable_delete(self_: *mut PxVehicleCommandValueResponseTable);

    pub fn PxVehicleCommandNonLinearResponseParams_new() -> PxVehicleCommandNonLinearResponseParams;

    pub fn PxVehicleCommandNonLinearResponseParams_clear_mut(self_: *mut PxVehicleCommandNonLinearResponseParams);

    /// Add a table of normalised response vs speed and associated it with a specified command value.
    ///
    /// commandValueSpeedResponses must be authored as a series of strictly increasing speeds with form {speed, normalizedResponse}
    ///
    /// The responses added must form a series of strictly increasing command values.
    pub fn PxVehicleCommandNonLinearResponseParams_addResponse_mut(self_: *mut PxVehicleCommandNonLinearResponseParams, commandValueSpeedResponses: *const PxVehicleCommandValueResponseTable) -> bool;

    pub fn PxVehicleCommandResponseParams_new() -> PxVehicleCommandResponseParams;

    pub fn PxVehicleBrakeCommandResponseParams_transformAndScale(self_: *const PxVehicleBrakeCommandResponseParams, srcFrame: *const PxVehicleFrame, trgFrame: *const PxVehicleFrame, srcScale: *const PxVehicleScale, trgScale: *const PxVehicleScale) -> PxVehicleBrakeCommandResponseParams;

    pub fn PxVehicleBrakeCommandResponseParams_isValid(self_: *const PxVehicleBrakeCommandResponseParams, axleDesc: *const PxVehicleAxleDescription) -> bool;

    pub fn PxVehicleBrakeCommandResponseParams_new() -> PxVehicleBrakeCommandResponseParams;

    pub fn PxVehicleCommandState_setToDefault_mut(self_: *mut PxVehicleCommandState);

    pub fn PxVehicleCommandState_new() -> PxVehicleCommandState;

    pub fn PxVehicleDirectDriveTransmissionCommandState_setToDefault_mut(self_: *mut PxVehicleDirectDriveTransmissionCommandState);

    pub fn PxVehicleEngineDriveTransmissionCommandState_setToDefault_mut(self_: *mut PxVehicleEngineDriveTransmissionCommandState);

    pub fn PxVehicleEngineDriveTransmissionCommandState_new() -> PxVehicleEngineDriveTransmissionCommandState;

    pub fn PxVehicleTankDriveTransmissionCommandState_setToDefault_mut(self_: *mut PxVehicleTankDriveTransmissionCommandState);

    /// Compute the linear response to a command.
    ///
    /// The linear response of the specified  wheel to the command.
    pub fn phys_PxVehicleLinearResponseCompute(command: f32, wheelId: u32, responseParams: *const PxVehicleCommandResponseParams) -> f32;

    /// Compute the non-linear response to a command.
    ///
    /// responseParams is used to compute an interpolated normalized response to the combination of command and longitudinalSpeed.
    /// The interpolated normalized response is then used in place of the command as input to PxVehicleComputeLinearResponse().
    pub fn phys_PxVehicleNonLinearResponseCompute(command: f32, longitudinalSpeed: f32, wheelId: u32, responseParams: *const PxVehicleCommandResponseParams) -> f32;

    pub fn PxVehicleDirectDriveThrottleCommandResponseParams_transformAndScale(self_: *const PxVehicleDirectDriveThrottleCommandResponseParams, srcFrame: *const PxVehicleFrame, trgFrame: *const PxVehicleFrame, srcScale: *const PxVehicleScale, trgScale: *const PxVehicleScale) -> PxVehicleDirectDriveThrottleCommandResponseParams;

    pub fn PxVehicleDirectDriveThrottleCommandResponseParams_isValid(self_: *const PxVehicleDirectDriveThrottleCommandResponseParams, axleDesc: *const PxVehicleAxleDescription) -> bool;

    pub fn PxVehicleDirectDriveThrottleCommandResponseParams_new() -> PxVehicleDirectDriveThrottleCommandResponseParams;

    pub fn PxVehicleClutchCommandResponseParams_transformAndScale(self_: *const PxVehicleClutchCommandResponseParams, srcFrame: *const PxVehicleFrame, trgFrame: *const PxVehicleFrame, srcScale: *const PxVehicleScale, trgScale: *const PxVehicleScale) -> PxVehicleClutchCommandResponseParams;

    pub fn PxVehicleClutchCommandResponseParams_isValid(self_: *const PxVehicleClutchCommandResponseParams) -> bool;

    pub fn PxVehicleClutchCommandResponseParams_new() -> PxVehicleClutchCommandResponseParams;

    pub fn PxVehicleClutchParams_transformAndScale(self_: *const PxVehicleClutchParams, srcFrame: *const PxVehicleFrame, trgFrame: *const PxVehicleFrame, srcScale: *const PxVehicleScale, trgScale: *const PxVehicleScale) -> PxVehicleClutchParams;

    pub fn PxVehicleClutchParams_isValid(self_: *const PxVehicleClutchParams) -> bool;

    pub fn PxVehicleClutchParams_new() -> PxVehicleClutchParams;

    pub fn PxVehicleEngineParams_transformAndScale(self_: *const PxVehicleEngineParams, srcFrame: *const PxVehicleFrame, trgFrame: *const PxVehicleFrame, srcScale: *const PxVehicleScale, trgScale: *const PxVehicleScale) -> PxVehicleEngineParams;

    pub fn PxVehicleEngineParams_isValid(self_: *const PxVehicleEngineParams) -> bool;

    pub fn PxVehicleEngineParams_delete(self_: *mut PxVehicleEngineParams);

    pub fn PxVehicleEngineParams_new() -> PxVehicleEngineParams;

    pub fn PxVehicleGearboxParams_transformAndScale(self_: *const PxVehicleGearboxParams, srcFrame: *const PxVehicleFrame, trgFrame: *const PxVehicleFrame, srcScale: *const PxVehicleScale, trgScale: *const PxVehicleScale) -> PxVehicleGearboxParams;

    pub fn PxVehicleGearboxParams_isValid(self_: *const PxVehicleGearboxParams) -> bool;

    pub fn PxVehicleGearboxParams_new() -> PxVehicleGearboxParams;

    pub fn PxVehicleAutoboxParams_transformAndScale(self_: *const PxVehicleAutoboxParams, srcFrame: *const PxVehicleFrame, trgFrame: *const PxVehicleFrame, srcScale: *const PxVehicleScale, trgScale: *const PxVehicleScale) -> PxVehicleAutoboxParams;

    pub fn PxVehicleAutoboxParams_isValid(self_: *const PxVehicleAutoboxParams, gearboxParams: *const PxVehicleGearboxParams) -> bool;

    pub fn PxVehicleAutoboxParams_new() -> PxVehicleAutoboxParams;

    pub fn PxVehicleFourWheelDriveDifferentialLegacyParams_transformAndScale(self_: *const PxVehicleFourWheelDriveDifferentialLegacyParams, srcFrame: *const PxVehicleFrame, trgFrame: *const PxVehicleFrame, srcScale: *const PxVehicleScale, trgScale: *const PxVehicleScale) -> PxVehicleFourWheelDriveDifferentialLegacyParams;

    pub fn PxVehicleFourWheelDriveDifferentialLegacyParams_isValid(self_: *const PxVehicleFourWheelDriveDifferentialLegacyParams, axleDesc: *const PxVehicleAxleDescription) -> bool;

    pub fn PxVehicleFourWheelDriveDifferentialLegacyParams_new() -> PxVehicleFourWheelDriveDifferentialLegacyParams;

    pub fn PxVehicleMultiWheelDriveDifferentialParams_setToDefault_mut(self_: *mut PxVehicleMultiWheelDriveDifferentialParams);

    pub fn PxVehicleMultiWheelDriveDifferentialParams_transformAndScale(self_: *const PxVehicleMultiWheelDriveDifferentialParams, srcFrame: *const PxVehicleFrame, trgFrame: *const PxVehicleFrame, srcScale: *const PxVehicleScale, trgScale: *const PxVehicleScale) -> PxVehicleMultiWheelDriveDifferentialParams;

    pub fn PxVehicleMultiWheelDriveDifferentialParams_isValid(self_: *const PxVehicleMultiWheelDriveDifferentialParams, axleDesc: *const PxVehicleAxleDescription) -> bool;

    pub fn PxVehicleMultiWheelDriveDifferentialParams_new() -> PxVehicleMultiWheelDriveDifferentialParams;

    pub fn PxVehicleFourWheelDriveDifferentialParams_setToDefault_mut(self_: *mut PxVehicleFourWheelDriveDifferentialParams);

    pub fn PxVehicleFourWheelDriveDifferentialParams_isValid(self_: *const PxVehicleFourWheelDriveDifferentialParams, axleDesc: *const PxVehicleAxleDescription) -> bool;

    pub fn PxVehicleFourWheelDriveDifferentialParams_transformAndScale(self_: *const PxVehicleFourWheelDriveDifferentialParams, srcFrame: *const PxVehicleFrame, trgFrame: *const PxVehicleFrame, srcScale: *const PxVehicleScale, trgScale: *const PxVehicleScale) -> PxVehicleFourWheelDriveDifferentialParams;

    pub fn PxVehicleFourWheelDriveDifferentialParams_new() -> PxVehicleFourWheelDriveDifferentialParams;

    pub fn PxVehicleTankDriveDifferentialParams_setToDefault_mut(self_: *mut PxVehicleTankDriveDifferentialParams);

    /// Add a tank track by specifying the number of wheels along the track and an array of wheel ids specifying each wheel in the tank track.
    pub fn PxVehicleTankDriveDifferentialParams_addTankTrack_mut(self_: *mut PxVehicleTankDriveDifferentialParams, nbWheelsInTrackToAdd: u32, wheelIdsInTrackToAdd: *const u32, thrustControllerIndex: u32);

    /// Return the number of tracks.
    ///
    /// The number of tracks.
    pub fn PxVehicleTankDriveDifferentialParams_getNbTracks(self_: *const PxVehicleTankDriveDifferentialParams) -> u32;

    /// Return the number of wheels in the ith track.
    ///
    /// The number of wheels in the specified track.
    pub fn PxVehicleTankDriveDifferentialParams_getNbWheelsInTrack(self_: *const PxVehicleTankDriveDifferentialParams, i: u32) -> u32;

    /// Return the array of all wheels in the ith track.
    ///
    /// The array of wheels in the specified track.
    pub fn PxVehicleTankDriveDifferentialParams_getWheelsInTrack(self_: *const PxVehicleTankDriveDifferentialParams, i: u32) -> *const u32;

    /// Return the wheel id of the jth wheel in the ith track.
    ///
    /// The wheel id of the jth wheel in the ith track.
    pub fn PxVehicleTankDriveDifferentialParams_getWheelInTrack(self_: *const PxVehicleTankDriveDifferentialParams, j: u32, i: u32) -> u32;

    /// Return the index of the thrust controller that will control a specified track.
    ///
    /// The index of the thrust controller that will control the ith track.
    pub fn PxVehicleTankDriveDifferentialParams_getThrustControllerIndex(self_: *const PxVehicleTankDriveDifferentialParams, i: u32) -> u32;

    pub fn PxVehicleTankDriveDifferentialParams_transformAndScale(self_: *const PxVehicleTankDriveDifferentialParams, srcFrame: *const PxVehicleFrame, trgFrame: *const PxVehicleFrame, srcScale: *const PxVehicleScale, trgScale: *const PxVehicleScale) -> PxVehicleTankDriveDifferentialParams;

    pub fn PxVehicleTankDriveDifferentialParams_isValid(self_: *const PxVehicleTankDriveDifferentialParams, axleDesc: *const PxVehicleAxleDescription) -> bool;

    pub fn PxVehicleTankDriveDifferentialParams_new() -> PxVehicleTankDriveDifferentialParams;

    pub fn PxVehicleClutchCommandResponseState_setToDefault_mut(self_: *mut PxVehicleClutchCommandResponseState);

    pub fn PxVehicleEngineDriveThrottleCommandResponseState_setToDefault_mut(self_: *mut PxVehicleEngineDriveThrottleCommandResponseState);

    pub fn PxVehicleEngineState_setToDefault_mut(self_: *mut PxVehicleEngineState);

    pub fn PxVehicleGearboxState_setToDefault_mut(self_: *mut PxVehicleGearboxState);

    pub fn PxVehicleAutoboxState_setToDefault_mut(self_: *mut PxVehicleAutoboxState);

    pub fn PxVehicleDifferentialState_setToDefault_mut(self_: *mut PxVehicleDifferentialState);

    pub fn PxVehicleWheelConstraintGroupState_setToDefault_mut(self_: *mut PxVehicleWheelConstraintGroupState);

    /// Add a wheel constraint group by specifying the number of wheels in the group, an array of wheel ids specifying each wheel in the group
    /// and a desired rotational speed relationship.
    ///
    /// constraintMultipliers[j] specifies the target rotational speed of the jth wheel in the constraint group as a multiplier of the rotational
    /// speed of the zeroth wheel in the group.
    pub fn PxVehicleWheelConstraintGroupState_addConstraintGroup_mut(self_: *mut PxVehicleWheelConstraintGroupState, nbWheelsInGroupToAdd: u32, wheelIdsInGroupToAdd: *const u32, constraintMultipliers: *const f32);

    /// Return the number of wheel constraint groups in the vehicle.
    ///
    /// The number of wheel constraint groups.
    pub fn PxVehicleWheelConstraintGroupState_getNbConstraintGroups(self_: *const PxVehicleWheelConstraintGroupState) -> u32;

    /// Return the number of wheels in the ith constraint group.
    ///
    /// The number of wheels in the specified constraint group.
    pub fn PxVehicleWheelConstraintGroupState_getNbWheelsInConstraintGroup(self_: *const PxVehicleWheelConstraintGroupState, i: u32) -> u32;

    /// Return the wheel id of the jth wheel in the ith constraint group.
    ///
    /// The wheel id of the jth wheel in the ith constraint group.
    pub fn PxVehicleWheelConstraintGroupState_getWheelInConstraintGroup(self_: *const PxVehicleWheelConstraintGroupState, j: u32, i: u32) -> u32;

    /// Return the constraint multiplier of the jth wheel in the ith constraint group
    ///
    /// The constraint multiplier of the jth wheel in the ith constraint group.
    pub fn PxVehicleWheelConstraintGroupState_getMultiplierInConstraintGroup(self_: *const PxVehicleWheelConstraintGroupState, j: u32, i: u32) -> f32;

    pub fn PxVehicleClutchSlipState_setToDefault_mut(self_: *mut PxVehicleClutchSlipState);

    /// Compute the coupling strength of the clutch.
    ///
    /// If the gear is in neutral the clutch is fully disengaged and the clutch strength is 0.
    ///
    /// A clutch response state of 0.0 denotes a fully engaged clutch with maximum strength.
    ///
    /// A clutch response state of 1.0 denotes a fully disengaged clutch with a strength of 0.0.
    pub fn phys_PxVehicleClutchStrengthCompute(clutchResponseState: *const PxVehicleClutchCommandResponseState, gearboxParams: *const PxVehicleGearboxParams, gearboxState: *const PxVehicleGearboxState) -> f32;

    /// Compute the damping rate of the engine.
    ///
    /// Engines typically have different damping rates with clutch engaged and disengaged.
    ///
    /// Engines typically have different damping rates at different throttle pedal values.
    ///
    /// In neutral gear the clutch is considered to be fully disengaged.
    pub fn phys_PxVehicleEngineDampingRateCompute(engineParams: *const PxVehicleEngineParams, gearboxParams: *const PxVehicleGearboxParams, gearboxState: *const PxVehicleGearboxState, clutchResponseState: *const PxVehicleClutchCommandResponseState, throttleResponseState: *const PxVehicleEngineDriveThrottleCommandResponseState) -> f32;

    /// Compute the gear ratio delivered by the gearbox in the current gear.
    ///
    /// The gear ratio is the product of the gear ratio of the current gear and the final gear ratio of the gearbox.
    pub fn phys_PxVehicleGearRatioCompute(gearboxParams: *const PxVehicleGearboxParams, gearboxState: *const PxVehicleGearboxState) -> f32;

    /// Compute the drive torque to deliver to the engine.
    pub fn phys_PxVehicleEngineDriveTorqueCompute(engineParams: *const PxVehicleEngineParams, engineState: *const PxVehicleEngineState, throttleCommandResponseState: *const PxVehicleEngineDriveThrottleCommandResponseState) -> f32;

    /// This API was introduced with the new Vehicle API for transition purposes but will be removed in a future version.
    ///
    /// Compute the contribution that each wheel makes to the averaged wheel speed at the clutch plate connected to the wheels driven by
    /// the differential.
    ///
    /// Any wheel on an axle connected to the differential could have a non-zero value, depending on the way the differential couples to the wheels.
    ///
    /// Any wheel on an axle not connected to the differential will have a zero contribution to the averaged wheel speed.
    pub fn phys_PxVehicleLegacyDifferentialWheelSpeedContributionsCompute(diffParams: *const PxVehicleFourWheelDriveDifferentialLegacyParams, nbWheels: u32, diffAveWheelSpeedContributions: *mut f32);

    /// Compute the drive torque response to a throttle command.
    pub fn phys_PxVehicleDirectDriveThrottleCommandResponseUpdate(throttle: f32, transmissionCommands: *const PxVehicleDirectDriveTransmissionCommandState, longitudinalSpeed: f32, wheelId: u32, throttleResponseParams: *const PxVehicleDirectDriveThrottleCommandResponseParams, throttleResponseState: *mut f32);

    /// Determine the actuation state of a wheel given the brake torque, handbrake torque and drive torque applied to it.
    pub fn phys_PxVehicleDirectDriveActuationStateUpdate(brakeTorque: f32, driveTorque: f32, actuationState: *mut PxVehicleWheelActuationState);

    /// Forward integrate the angular speed of a wheel given the brake and drive torque applied to it
    pub fn phys_PxVehicleDirectDriveUpdate(wheelParams: *const PxVehicleWheelParams, actuationState: *const PxVehicleWheelActuationState, brakeTorque: f32, driveTorque: f32, tireForce: *const PxVehicleTireForce, dt: f32, wheelRigidBody1dState: *mut PxVehicleWheelRigidBody1dState);

    /// The autobox will not begin a gear change if a gear change is already ongoing.
    ///
    /// The autobox will not begin a gear change until a threshold time has lapsed since the last automated gear change.
    ///
    /// A gear change is considered as ongoing for as long as PxVehicleGearboxState::currentGear is different from
    /// PxVehicleGearboxState::targetGear.
    ///
    /// The autobox will not shift down from 1st gear or up from reverse gear.
    ///
    /// The autobox shifts in single gear increments or decrements.
    ///
    /// The autobox instantiates a gear change by setting PxVehicleCommandState::targetGear to be different from
    /// from PxVehicleGearboxState::currentGear
    pub fn phys_PxVehicleAutoBoxUpdate(engineParams: *const PxVehicleEngineParams, gearboxParams: *const PxVehicleGearboxParams, autoboxParams: *const PxVehicleAutoboxParams, engineState: *const PxVehicleEngineState, gearboxState: *const PxVehicleGearboxState, dt: f32, targetGearCommand: *mut u32, autoboxState: *mut PxVehicleAutoboxState, throttle: *mut f32);

    /// Propagate input gear commands to the gearbox state.
    ///
    /// Any ongoing gear change must complete before starting another.
    ///
    /// A gear change is considered as ongoing for as long as PxVehicleGearboxState::currentGear is different from
    /// PxVehicleGearboxState::targetGear.
    ///
    /// The gearbox remains in neutral for the duration of the gear change.
    ///
    /// A gear change begins if PxVehicleCommandState::targetGear is different from PxVehicleGearboxState::currentGear.
    pub fn phys_PxVehicleGearCommandResponseUpdate(targetGearCommand: u32, gearboxParams: *const PxVehicleGearboxParams, gearboxState: *mut PxVehicleGearboxState);

    /// Propagate the input clutch command to the clutch response state.
    pub fn phys_PxVehicleClutchCommandResponseLinearUpdate(clutchCommand: f32, clutchResponseParams: *const PxVehicleClutchCommandResponseParams, clutchResponse: *mut PxVehicleClutchCommandResponseState);

    /// Propagate the input throttle command to the throttle response state.
    pub fn phys_PxVehicleEngineDriveThrottleCommandResponseLinearUpdate(commands: *const PxVehicleCommandState, throttleResponse: *mut PxVehicleEngineDriveThrottleCommandResponseState);

    /// Compute the fraction of available torque to be delivered to each wheel and gather a list of all
    /// wheels connected to the differential.
    pub fn phys_PxVehicleDifferentialStateUpdate_2(axleDescription: *const PxVehicleAxleDescription, diffParams: *const PxVehicleMultiWheelDriveDifferentialParams, diffState: *mut PxVehicleDifferentialState);

    /// Update the current gear of the gearbox. If a gear change is ongoing then complete the gear change if a threshold
    /// time has passed since the beginning of the gear change.
    ///
    /// A gear change is considered as ongoing for as long as PxVehicleGearboxState::currentGear is different from
    /// PxVehicleGearboxState::targetGear.
    pub fn phys_PxVehicleGearboxUpdate(gearboxParams: *const PxVehicleGearboxParams, dt: f32, gearboxState: *mut PxVehicleGearboxState);

    pub fn PxVehicleSteerCommandResponseParams_transformAndScale(self_: *const PxVehicleSteerCommandResponseParams, srcFrame: *const PxVehicleFrame, trgFrame: *const PxVehicleFrame, srcScale: *const PxVehicleScale, trgScale: *const PxVehicleScale) -> PxVehicleSteerCommandResponseParams;

    pub fn PxVehicleSteerCommandResponseParams_isValid(self_: *const PxVehicleSteerCommandResponseParams, axleDesc: *const PxVehicleAxleDescription) -> bool;

    pub fn PxVehicleSteerCommandResponseParams_new() -> PxVehicleSteerCommandResponseParams;

    pub fn PxVehicleAckermannParams_isValid(self_: *const PxVehicleAckermannParams, axleDesc: *const PxVehicleAxleDescription) -> bool;

    pub fn PxVehicleAckermannParams_transformAndScale(self_: *const PxVehicleAckermannParams, srcFrame: *const PxVehicleFrame, trgFrame: *const PxVehicleFrame, srcScale: *const PxVehicleScale, trgScale: *const PxVehicleScale) -> PxVehicleAckermannParams;

    pub fn PxVehicleAckermannParams_new() -> PxVehicleAckermannParams;

    /// Compute the yaw angle response to a steer command.
    pub fn phys_PxVehicleSteerCommandResponseUpdate(steer: f32, longitudinalSpeed: f32, wheelId: u32, steerResponseParams: *const PxVehicleSteerCommandResponseParams, steerResponseState: *mut f32);

    pub fn PxVehicleWheelActuationState_setToDefault_mut(self_: *mut PxVehicleWheelActuationState);

    pub fn PxVehicleWheelRigidBody1dState_setToDefault_mut(self_: *mut PxVehicleWheelRigidBody1dState);

    pub fn PxVehicleWheelLocalPose_setToDefault_mut(self_: *mut PxVehicleWheelLocalPose);

    pub fn PxVehicleWheelParams_transformAndScale(self_: *const PxVehicleWheelParams, srcFrame: *const PxVehicleFrame, trgFrame: *const PxVehicleFrame, srcScale: *const PxVehicleScale, trgScale: *const PxVehicleScale) -> PxVehicleWheelParams;

    pub fn PxVehicleWheelParams_isValid(self_: *const PxVehicleWheelParams) -> bool;

    pub fn PxVehicleWheelParams_new() -> PxVehicleWheelParams;

    pub fn PxVehicleTireDirectionState_setToDefault_mut(self_: *mut PxVehicleTireDirectionState);

    pub fn PxVehicleTireSpeedState_setToDefault_mut(self_: *mut PxVehicleTireSpeedState);

    pub fn PxVehicleTireSlipState_setToDefault_mut(self_: *mut PxVehicleTireSlipState);

    pub fn PxVehicleTireGripState_setToDefault_mut(self_: *mut PxVehicleTireGripState);

    pub fn PxVehicleTireCamberAngleState_setToDefault_mut(self_: *mut PxVehicleTireCamberAngleState);

    pub fn PxVehicleTireStickyState_setToDefault_mut(self_: *mut PxVehicleTireStickyState);

    pub fn PxVehicleTireForce_setToDefault_mut(self_: *mut PxVehicleTireForce);

    pub fn PxVehicleDirectDriveCommandResponseComponent_delete(self_: *mut PxVehicleDirectDriveCommandResponseComponent);

    /// Compute a per wheel response to the input brake/handbrake/throttle/steer commands
    /// and determine if there is an intention to accelerate the vehicle.
    pub fn PxVehicleDirectDriveCommandResponseComponent_update_mut(self_: *mut PxVehicleDirectDriveCommandResponseComponent, dt: f32, context: *const PxVehicleSimulationContext) -> bool;

    pub fn PxVehicleDirectDriveActuationStateComponent_delete(self_: *mut PxVehicleDirectDriveActuationStateComponent);

    /// Compute the actuation state for each wheel given the brake, handbrake and throttle states.
    /// \
    pub fn PxVehicleDirectDriveActuationStateComponent_update_mut(self_: *mut PxVehicleDirectDriveActuationStateComponent, dt: f32, context: *const PxVehicleSimulationContext) -> bool;

    pub fn PxVehicleDirectDrivetrainComponent_delete(self_: *mut PxVehicleDirectDrivetrainComponent);

    pub fn PxVehicleDirectDrivetrainComponent_update_mut(self_: *mut PxVehicleDirectDrivetrainComponent, dt: f32, context: *const PxVehicleSimulationContext) -> bool;

    pub fn PxVehicleEngineDriveCommandResponseComponent_delete(self_: *mut PxVehicleEngineDriveCommandResponseComponent);

    pub fn PxVehicleEngineDriveCommandResponseComponent_update_mut(self_: *mut PxVehicleEngineDriveCommandResponseComponent, dt: f32, context: *const PxVehicleSimulationContext) -> bool;

    pub fn PxVehicleMultiWheelDriveDifferentialStateComponent_delete(self_: *mut PxVehicleMultiWheelDriveDifferentialStateComponent);

    pub fn PxVehicleMultiWheelDriveDifferentialStateComponent_getDataForMultiWheelDriveDifferentialStateComponent_mut(self_: *mut PxVehicleMultiWheelDriveDifferentialStateComponent, axleDescription: *mut *const PxVehicleAxleDescription, differentialParams: *mut *const PxVehicleMultiWheelDriveDifferentialParams, differentialState: *mut *mut PxVehicleDifferentialState);

    pub fn PxVehicleMultiWheelDriveDifferentialStateComponent_update_mut(self_: *mut PxVehicleMultiWheelDriveDifferentialStateComponent, dt: f32, context: *const PxVehicleSimulationContext) -> bool;

    pub fn PxVehicleFourWheelDriveDifferentialStateComponent_delete(self_: *mut PxVehicleFourWheelDriveDifferentialStateComponent);

    pub fn PxVehicleFourWheelDriveDifferentialStateComponent_update_mut(self_: *mut PxVehicleFourWheelDriveDifferentialStateComponent, dt: f32, context: *const PxVehicleSimulationContext) -> bool;

    pub fn PxVehicleTankDriveDifferentialStateComponent_delete(self_: *mut PxVehicleTankDriveDifferentialStateComponent);

    pub fn PxVehicleTankDriveDifferentialStateComponent_update_mut(self_: *mut PxVehicleTankDriveDifferentialStateComponent, dt: f32, context: *const PxVehicleSimulationContext) -> bool;

    pub fn PxVehicleLegacyFourWheelDriveDifferentialStateComponent_delete(self_: *mut PxVehicleLegacyFourWheelDriveDifferentialStateComponent);

    pub fn PxVehicleLegacyFourWheelDriveDifferentialStateComponent_update_mut(self_: *mut PxVehicleLegacyFourWheelDriveDifferentialStateComponent, dt: f32, context: *const PxVehicleSimulationContext) -> bool;

    pub fn PxVehicleEngineDriveActuationStateComponent_delete(self_: *mut PxVehicleEngineDriveActuationStateComponent);

    pub fn PxVehicleEngineDriveActuationStateComponent_update_mut(self_: *mut PxVehicleEngineDriveActuationStateComponent, dt: f32, context: *const PxVehicleSimulationContext) -> bool;

    pub fn PxVehicleEngineDrivetrainComponent_delete(self_: *mut PxVehicleEngineDrivetrainComponent);

    pub fn PxVehicleEngineDrivetrainComponent_update_mut(self_: *mut PxVehicleEngineDrivetrainComponent, dt: f32, context: *const PxVehicleSimulationContext) -> bool;

    pub fn PxVehiclePhysXRigidActorParams_new(_physxActorRigidBodyParams: *const PxVehicleRigidBodyParams, _physxActorName: *const std::ffi::c_char) -> PxVehiclePhysXRigidActorParams;

    pub fn PxVehiclePhysXRigidActorShapeParams_new(_geometry: *const PxGeometry, _localPose: *const PxTransform, _material: *const PxMaterial, _flags: PxShapeFlags, _simulationFilterData: *const PxFilterData, _queryFilterData: *const PxFilterData) -> PxVehiclePhysXRigidActorShapeParams;

    pub fn PxVehiclePhysXWheelParams_new(_axleDescription: *const PxVehicleAxleDescription, _wheelParams: *const PxVehicleWheelParams) -> PxVehiclePhysXWheelParams;

    pub fn PxVehiclePhysXWheelShapeParams_new(_material: *const PxMaterial, _flags: PxShapeFlags, _simulationFilterData: PxFilterData, _queryFilterData: PxFilterData) -> PxVehiclePhysXWheelShapeParams;

    /// Create a PxRigidDynamic instance, instantiate it with desired properties and populate it with PxShape instances.
    ///
    /// This is an alternative to PxVehiclePhysXArticulationLinkCreate.
    ///
    /// PxVehiclePhysXActorCreate primarily serves as an illustration of the instantiation of the PhysX class instances
    /// required to simulate a vehicle with a PxRigidDynamic.
    pub fn phys_PxVehiclePhysXActorCreate(vehicleFrame: *const PxVehicleFrame, rigidActorParams: *const PxVehiclePhysXRigidActorParams, rigidActorCmassLocalPose: *const PxTransform, rigidActorShapeParams: *const PxVehiclePhysXRigidActorShapeParams, wheelParams: *const PxVehiclePhysXWheelParams, wheelShapeParams: *const PxVehiclePhysXWheelShapeParams, physics: *mut PxPhysics, params: *const PxCookingParams, vehiclePhysXActor: *mut PxVehiclePhysXActor);

    /// Configure an actor so that it is ready for vehicle simulation.
    pub fn phys_PxVehiclePhysXActorConfigure(rigidActorParams: *const PxVehiclePhysXRigidActorParams, rigidActorCmassLocalPose: *const PxTransform, rigidBody: *mut PxRigidBody);

    /// Create a PxArticulationReducedCoordinate and a single PxArticulationLink,
    /// instantiate the PxArticulationLink with desired properties and populate it with PxShape instances.
    ///
    /// This is an alternative to PxVehiclePhysXActorCreate.
    ///
    /// PxVehiclePhysXArticulationLinkCreate primarily serves as an illustration of the instantiation of the PhysX class instances
    /// required to simulate a vehicle as part of an articulated ensemble.
    pub fn phys_PxVehiclePhysXArticulationLinkCreate(vehicleFrame: *const PxVehicleFrame, rigidActorParams: *const PxVehiclePhysXRigidActorParams, rigidActorCmassLocalPose: *const PxTransform, rigidActorShapeParams: *const PxVehiclePhysXRigidActorShapeParams, wheelParams: *const PxVehiclePhysXWheelParams, wheelShapeParams: *const PxVehiclePhysXWheelShapeParams, physics: *mut PxPhysics, params: *const PxCookingParams, vehiclePhysXActor: *mut PxVehiclePhysXActor);

    /// Release the PxRigidDynamic, PxArticulationReducedCoordinate, PxArticulationLink and PxShape instances
    /// instantiated by PxVehiclePhysXActorCreate or PxVehiclePhysXArticulationLinkCreate.
    pub fn phys_PxVehiclePhysXActorDestroy(vehiclePhysXActor: *mut PxVehiclePhysXActor);

    /// Wake up the physx actor if the actor is asleep and the commands signal an intent to
    /// change the state of the vehicle.
    ///
    /// If the steering has changed, the actor will be woken up.
    ///
    /// On exit from PxVehiclePhysxActorWakeup, physxSteerState.previousSteerCommand is assigned to the value
    /// of commands.steer so that the steer state may be propagated to the subsequent call to PxVehiclePhysxActorWakeup().
    ///
    /// If physxSteerState.previousSteerCommand has value PX_VEHICLE_UNSPECIFIED_STEER_STATE, the steering state
    /// is treated as though it has not changed.
    pub fn phys_PxVehiclePhysxActorWakeup(commands: *const PxVehicleCommandState, transmissionCommands: *const PxVehicleEngineDriveTransmissionCommandState, gearParams: *const PxVehicleGearboxParams, gearState: *const PxVehicleGearboxState, physxActor: *mut PxRigidBody, physxSteerState: *mut PxVehiclePhysXSteerState);

    /// Read the rigid body state from a PhysX actor.
    pub fn phys_PxVehicleReadRigidBodyStateFromPhysXActor(physxActor: *const PxRigidBody, rigidBodyState: *mut PxVehicleRigidBodyState);

    /// Update the local pose of a PxShape that is associated with a wheel.
    pub fn phys_PxVehicleWriteWheelLocalPoseToPhysXWheelShape(wheelLocalPose: *const PxTransform, wheelShapeLocalPose: *const PxTransform, shape: *mut PxShape);

    /// Write the rigid body state to a PhysX actor.
    pub fn phys_PxVehicleWriteRigidBodyStateToPhysXActor(physxActorUpdateMode: PxVehiclePhysXActorUpdateMode, rigidBodyState: *const PxVehicleRigidBodyState, dt: f32, physXActor: *mut PxRigidBody);

    pub fn PxVehiclePhysXActorBeginComponent_delete(self_: *mut PxVehiclePhysXActorBeginComponent);

    pub fn PxVehiclePhysXActorBeginComponent_update_mut(self_: *mut PxVehiclePhysXActorBeginComponent, dt: f32, context: *const PxVehicleSimulationContext) -> bool;

    pub fn PxVehiclePhysXActorEndComponent_delete(self_: *mut PxVehiclePhysXActorEndComponent);

    pub fn PxVehiclePhysXActorEndComponent_update_mut(self_: *mut PxVehiclePhysXActorEndComponent, dt: f32, context: *const PxVehicleSimulationContext) -> bool;

    pub fn PxVehiclePhysXSuspensionLimitConstraintParams_transformAndScale(self_: *const PxVehiclePhysXSuspensionLimitConstraintParams, srcFrame: *const PxVehicleFrame, trgFrame: *const PxVehicleFrame, srcScale: *const PxVehicleScale, trgScale: *const PxVehicleScale) -> PxVehiclePhysXSuspensionLimitConstraintParams;

    pub fn PxVehiclePhysXSuspensionLimitConstraintParams_isValid(self_: *const PxVehiclePhysXSuspensionLimitConstraintParams) -> bool;

    pub fn PxVehiclePhysXSuspensionLimitConstraintParams_new() -> PxVehiclePhysXSuspensionLimitConstraintParams;

    pub fn PxVehiclePhysXConstraintState_setToDefault_mut(self_: *mut PxVehiclePhysXConstraintState);

    pub fn phys_vehicleConstraintSolverPrep(constraints: *mut Px1DConstraint, body0WorldOffset: *mut PxVec3Padded, maxConstraints: u32, anon_param3: *mut PxConstraintInvMassScale, constantBlock: *const std::ffi::c_void, bodyAToWorld: *const PxTransform, bodyBToWorld: *const PxTransform, anon_param7: bool, cA2w: *mut PxVec3Padded, cB2w: *mut PxVec3Padded) -> u32;

    pub fn phys_visualiseVehicleConstraint(viz: *mut PxConstraintVisualizer, constantBlock: *const std::ffi::c_void, body0Transform: *const PxTransform, body1Transform: *const PxTransform, flags: u32);

    pub fn PxVehicleConstraintConnector_new_alloc() -> *mut PxVehicleConstraintConnector;

    pub fn PxVehicleConstraintConnector_new_alloc_1(vehicleConstraintState: *mut PxVehiclePhysXConstraintState) -> *mut PxVehicleConstraintConnector;

    pub fn PxVehicleConstraintConnector_delete(self_: *mut PxVehicleConstraintConnector);

    pub fn PxVehicleConstraintConnector_setConstraintState_mut(self_: *mut PxVehicleConstraintConnector, constraintState: *mut PxVehiclePhysXConstraintState);

    pub fn PxVehicleConstraintConnector_prepareData_mut(self_: *mut PxVehicleConstraintConnector) -> *mut std::ffi::c_void;

    pub fn PxVehicleConstraintConnector_getConstantBlock(self_: *const PxVehicleConstraintConnector) -> *const std::ffi::c_void;

    pub fn PxVehicleConstraintConnector_getPrep(self_: *const PxVehicleConstraintConnector) -> *mut std::ffi::c_void;

    pub fn PxVehicleConstraintConnector_onConstraintRelease_mut(self_: *mut PxVehicleConstraintConnector);

    pub fn PxVehicleConstraintConnector_updateOmniPvdProperties(self_: *const PxVehicleConstraintConnector);

    pub fn PxVehicleConstraintConnector_onComShift_mut(self_: *mut PxVehicleConstraintConnector, actor: u32);

    pub fn PxVehicleConstraintConnector_onOriginShift_mut(self_: *mut PxVehicleConstraintConnector, shift: *const PxVec3);

    pub fn PxVehicleConstraintConnector_getExternalReference_mut(self_: *mut PxVehicleConstraintConnector, typeID: *mut u32) -> *mut std::ffi::c_void;

    pub fn PxVehicleConstraintConnector_getSerializable_mut(self_: *mut PxVehicleConstraintConnector) -> *mut PxBase;

    pub fn PxVehiclePhysXConstraints_setToDefault_mut(self_: *mut PxVehiclePhysXConstraints);

    /// Instantiate the PhysX custom constraints.
    ///
    /// Custom constraints will resolve excess suspension compression and velocity constraints that serve as
    /// a replacement low speed tire model.
    pub fn phys_PxVehicleConstraintsCreate(axleDescription: *const PxVehicleAxleDescription, physics: *mut PxPhysics, physxActor: *mut PxRigidBody, vehicleConstraints: *mut PxVehiclePhysXConstraints);

    /// To ensure the constraints are processed by the PhysX scene they are marked as dirty prior to each simulate step.
    pub fn phys_PxVehicleConstraintsDirtyStateUpdate(vehicleConstraints: *mut PxVehiclePhysXConstraints);

    /// Destroy the PhysX custom constraints.
    pub fn phys_PxVehicleConstraintsDestroy(vehicleConstraints: *mut PxVehiclePhysXConstraints);

    /// Read constraint data from the vehicle's internal state for a single wheel and write it to a
    /// structure that will be read by the associated PxScene and used to impose the constraints during the next
    /// PxScene::simulate() step.
    ///
    /// Constraints include suspension constraints to account for suspension travel limit and sticky
    /// tire constraints that bring the vehicle to rest at low longitudinal and lateral speed.
    pub fn phys_PxVehiclePhysXConstraintStatesUpdate(suspensionParams: *const PxVehicleSuspensionParams, suspensionLimitParams: *const PxVehiclePhysXSuspensionLimitConstraintParams, suspensionState: *const PxVehicleSuspensionState, suspensionComplianceState: *const PxVehicleSuspensionComplianceState, groundPlaneNormal: *const PxVec3, tireStickyDampingLong: f32, tireStickyDampingLat: f32, tireDirectionState: *const PxVehicleTireDirectionState, tireStickyState: *const PxVehicleTireStickyState, rigidBodyState: *const PxVehicleRigidBodyState, constraintState: *mut PxVehiclePhysXConstraintState);

    pub fn PxVehicleSuspensionParams_transformAndScale(self_: *const PxVehicleSuspensionParams, srcFrame: *const PxVehicleFrame, trgFrame: *const PxVehicleFrame, srcScale: *const PxVehicleScale, trgScale: *const PxVehicleScale) -> PxVehicleSuspensionParams;

    pub fn PxVehicleSuspensionParams_isValid(self_: *const PxVehicleSuspensionParams) -> bool;

    pub fn PxVehicleSuspensionParams_new() -> PxVehicleSuspensionParams;

    pub fn PxVehicleSuspensionStateCalculationParams_transformAndScale(self_: *const PxVehicleSuspensionStateCalculationParams, srcFrame: *const PxVehicleFrame, trgFrame: *const PxVehicleFrame, srcScale: *const PxVehicleScale, trgScale: *const PxVehicleScale) -> PxVehicleSuspensionStateCalculationParams;

    pub fn PxVehicleSuspensionStateCalculationParams_isValid(self_: *const PxVehicleSuspensionStateCalculationParams) -> bool;

    pub fn PxVehicleSuspensionStateCalculationParams_new() -> PxVehicleSuspensionStateCalculationParams;

    pub fn PxVehicleSuspensionComplianceParams_transformAndScale(self_: *const PxVehicleSuspensionComplianceParams, srcFrame: *const PxVehicleFrame, trgFrame: *const PxVehicleFrame, srcScale: *const PxVehicleScale, trgScale: *const PxVehicleScale) -> PxVehicleSuspensionComplianceParams;

    pub fn PxVehicleSuspensionComplianceParams_isValid(self_: *const PxVehicleSuspensionComplianceParams) -> bool;

    pub fn PxVehicleSuspensionComplianceParams_delete(self_: *mut PxVehicleSuspensionComplianceParams);

    pub fn PxVehicleSuspensionComplianceParams_new() -> PxVehicleSuspensionComplianceParams;

    pub fn PxVehicleSuspensionForceParams_transformAndScale(self_: *const PxVehicleSuspensionForceParams, srcFrame: *const PxVehicleFrame, trgFrame: *const PxVehicleFrame, srcScale: *const PxVehicleScale, trgScale: *const PxVehicleScale) -> PxVehicleSuspensionForceParams;

    pub fn PxVehicleSuspensionForceParams_isValid(self_: *const PxVehicleSuspensionForceParams) -> bool;

    pub fn PxVehicleSuspensionForceParams_new() -> PxVehicleSuspensionForceParams;

    pub fn PxVehicleSuspensionForceLegacyParams_transformAndScale(self_: *const PxVehicleSuspensionForceLegacyParams, srcFrame: *const PxVehicleFrame, trgFrame: *const PxVehicleFrame, srcScale: *const PxVehicleScale, trgScale: *const PxVehicleScale) -> PxVehicleSuspensionForceLegacyParams;

    pub fn PxVehicleSuspensionForceLegacyParams_isValid(self_: *const PxVehicleSuspensionForceLegacyParams) -> bool;

    pub fn PxVehicleSuspensionForceLegacyParams_new() -> PxVehicleSuspensionForceLegacyParams;

    pub fn PxVehicleAntiRollForceParams_transformAndScale(self_: *const PxVehicleAntiRollForceParams, srcFrame: *const PxVehicleFrame, trgFrame: *const PxVehicleFrame, srcScale: *const PxVehicleScale, trgScale: *const PxVehicleScale) -> PxVehicleAntiRollForceParams;

    pub fn PxVehicleAntiRollForceParams_isValid(self_: *const PxVehicleAntiRollForceParams, axleDesc: *const PxVehicleAxleDescription) -> bool;

    pub fn PxVehicleAntiRollForceParams_new() -> PxVehicleAntiRollForceParams;

    pub fn PxVehicleSuspensionState_setToDefault_mut(self_: *mut PxVehicleSuspensionState, _jounce: f32, _separation: f32);

    pub fn PxVehicleSuspensionState_new() -> PxVehicleSuspensionState;

    pub fn PxVehicleSuspensionComplianceState_setToDefault_mut(self_: *mut PxVehicleSuspensionComplianceState);

    pub fn PxVehicleSuspensionForce_setToDefault_mut(self_: *mut PxVehicleSuspensionForce);

    pub fn PxVehicleAntiRollTorque_setToDefault_mut(self_: *mut PxVehicleAntiRollTorque);

    pub fn PxVehiclePhysXConstraintComponent_delete(self_: *mut PxVehiclePhysXConstraintComponent);

    pub fn PxVehiclePhysXConstraintComponent_update_mut(self_: *mut PxVehiclePhysXConstraintComponent, dt: f32, context: *const PxVehicleSimulationContext) -> bool;

    pub fn PxVehiclePhysXRoadGeometryQueryParams_transformAndScale(self_: *const PxVehiclePhysXRoadGeometryQueryParams, srcFrame: *const PxVehicleFrame, trgFrame: *const PxVehicleFrame, srcScale: *const PxVehicleScale, trgScale: *const PxVehicleScale) -> PxVehiclePhysXRoadGeometryQueryParams;

    pub fn PxVehiclePhysXRoadGeometryQueryParams_isValid(self_: *const PxVehiclePhysXRoadGeometryQueryParams) -> bool;

    pub fn PxVehiclePhysXRoadGeometryQueryParams_new() -> PxVehiclePhysXRoadGeometryQueryParams;

    pub fn PxVehiclePhysXMaterialFriction_isValid(self_: *const PxVehiclePhysXMaterialFriction) -> bool;

    pub fn PxVehiclePhysXMaterialFrictionParams_isValid(self_: *const PxVehiclePhysXMaterialFrictionParams) -> bool;

    /// Create a cylindrical mesh with unit radius and half-width.
    ///
    /// Return a PxConvexMesh instance that represents a convex hull with unit radius and half-width.
    pub fn phys_PxVehicleUnitCylinderSweepMeshCreate(vehicleFrame: *const PxVehicleFrame, physics: *mut PxPhysics, params: *const PxCookingParams) -> *mut PxConvexMesh;

    /// Release the mesh created with PxVehicleUnitCylinderSweepMeshCreate.
    pub fn phys_PxVehicleUnitCylinderSweepMeshDestroy(mesh: *mut PxConvexMesh);

    /// Compute the plane of the road geometry under a wheel and the tire friction of the contact.
    ///
    /// PxVehicleRoadGeometryState::hitState will have value false in the event that the there is no reachable road geometry under the wheel and
    /// true if there is reachable road geometry under the wheel. Road geometry is considered reachable if the suspension can elongate from its
    /// reference pose far enough to place wheel on the ground.
    pub fn phys_PxVehiclePhysXRoadGeometryQueryUpdate(wheelParams: *const PxVehicleWheelParams, suspParams: *const PxVehicleSuspensionParams, queryType: PxVehiclePhysXRoadGeometryQueryType, filterCallback: *mut PxQueryFilterCallback, filterData: *const PxQueryFilterData, materialFrictionParams: *const PxVehiclePhysXMaterialFrictionParams, wheelYawAngle: f32, rigidBodyState: *const PxVehicleRigidBodyState, scene: *const PxScene, unitCylinderSweepMesh: *const PxConvexMesh, frame: *const PxVehicleFrame, roadGeomState: *mut PxVehicleRoadGeometryState, physxRoadGeometryState: *mut PxVehiclePhysXRoadGeometryQueryState);

    pub fn PxVehiclePhysXRoadGeometrySceneQueryComponent_delete(self_: *mut PxVehiclePhysXRoadGeometrySceneQueryComponent);

    pub fn PxVehiclePhysXRoadGeometrySceneQueryComponent_update_mut(self_: *mut PxVehiclePhysXRoadGeometrySceneQueryComponent, dt: f32, context: *const PxVehicleSimulationContext) -> bool;

    /// Destory the attribute handles created by PxVehiclePvdAttributesCreate().
    pub fn phys_PxVehiclePvdAttributesRelease(allocator: *mut PxAllocatorCallback, attributeHandles: *mut PxVehiclePvdAttributeHandles);

    /// Create omnipvd objects that will be used to reflect an individual veicle in omnipvd.
    ///
    /// PxVehiclePvdObjectCreate() must be called after PxVehiclePvdAttributesCreate().
    pub fn phys_PxVehiclePvdObjectCreate(nbWheels: u32, nbAntirolls: u32, maxNbPhysxMaterialFrictions: u32, contextHandle: u64, allocator: *mut PxAllocatorCallback) -> *mut PxVehiclePvdObjectHandles;

    pub fn PxVehicleTireForceParams_transformAndScale(self_: *const PxVehicleTireForceParams, srcFrame: *const PxVehicleFrame, trgFrame: *const PxVehicleFrame, srcScale: *const PxVehicleScale, trgScale: *const PxVehicleScale) -> PxVehicleTireForceParams;

    pub fn PxVehicleTireForceParams_isValid(self_: *const PxVehicleTireForceParams) -> bool;

    pub fn PxVehicleTireForceParams_new() -> PxVehicleTireForceParams;

    pub fn PxVehiclePVDComponent_delete(self_: *mut PxVehiclePVDComponent);

    pub fn PxVehiclePVDComponent_update_mut(self_: *mut PxVehiclePVDComponent, dt: f32, context: *const PxVehicleSimulationContext) -> bool;

    pub fn PxVehicleRigidBodyParams_transformAndScale(self_: *const PxVehicleRigidBodyParams, srcFrame: *const PxVehicleFrame, trgFrame: *const PxVehicleFrame, srcScale: *const PxVehicleScale, trgScale: *const PxVehicleScale) -> PxVehicleRigidBodyParams;

    pub fn PxVehicleRigidBodyParams_isValid(self_: *const PxVehicleRigidBodyParams) -> bool;

    pub fn PxVehicleRigidBodyParams_new() -> PxVehicleRigidBodyParams;

    pub fn PxVehicleRigidBodyComponent_delete(self_: *mut PxVehicleRigidBodyComponent);

    pub fn PxVehicleRigidBodyComponent_update_mut(self_: *mut PxVehicleRigidBodyComponent, dt: f32, context: *const PxVehicleSimulationContext) -> bool;

    /// Compute the quaternion of a wheel in the rigid body frame.
    ///
    /// The quaterion of the wheel in the rigid body frame.
    pub fn phys_PxVehicleComputeWheelLocalOrientation(frame: *const PxVehicleFrame, suspensionParams: *const PxVehicleSuspensionParams, camberAngle: f32, toeAngle: f32, steerAngle: f32, rotationAngle: f32) -> PxQuat;

    /// Compute the quaternion of a wheel in the world frame.
    ///
    /// The quaterion of the wheel in the world frame.
    pub fn phys_PxVehicleComputeWheelOrientation(frame: *const PxVehicleFrame, suspensionParams: *const PxVehicleSuspensionParams, camberAngle: f32, toeAngle: f32, steerAngle: f32, rigidBodyOrientation: *const PxQuat, rotationAngle: f32) -> PxQuat;

    /// Compute the pose of the wheel in the rigid body frame.
    ///
    /// The pose of the wheel in the rigid body frame.
    pub fn phys_PxVehicleComputeWheelLocalPose(frame: *const PxVehicleFrame, suspensionParams: *const PxVehicleSuspensionParams, suspensionState: *const PxVehicleSuspensionState, camberAngle: f32, toeAngle: f32, steerAngle: f32, rotationAngle: f32) -> PxTransform;

    /// Compute the pose of the wheel in the rigid body frame.
    ///
    /// The pose of the wheel in the rigid body frame.
    pub fn phys_PxVehicleComputeWheelLocalPose_1(frame: *const PxVehicleFrame, suspensionParams: *const PxVehicleSuspensionParams, suspensionState: *const PxVehicleSuspensionState, suspensionComplianceState: *const PxVehicleSuspensionComplianceState, steerAngle: f32, wheelState: *const PxVehicleWheelRigidBody1dState) -> PxTransform;

    /// Compute the pose of the wheel in the world frame.
    ///
    /// The pose of the wheel in the world frame.
    pub fn phys_PxVehicleComputeWheelPose(frame: *const PxVehicleFrame, suspensionParams: *const PxVehicleSuspensionParams, suspensionState: *const PxVehicleSuspensionState, camberAngle: f32, toeAngle: f32, steerAngle: f32, rigidBodyPose: *const PxTransform, rotationAngle: f32) -> PxTransform;

    /// Compute the pose of the wheel in the world frame.
    ///
    /// The pose of the wheel in the world frame.
    pub fn phys_PxVehicleComputeWheelPose_1(frame: *const PxVehicleFrame, suspensionParams: *const PxVehicleSuspensionParams, suspensionState: *const PxVehicleSuspensionState, suspensionComplianceState: *const PxVehicleSuspensionComplianceState, steerAngle: f32, rigidBodyPose: *const PxTransform, wheelState: *const PxVehicleWheelRigidBody1dState) -> PxTransform;

    /// Check if the suspension could place the wheel on the ground or not.
    ///
    /// True if the wheel connects to the ground, else false.
    pub fn phys_PxVehicleIsWheelOnGround(suspState: *const PxVehicleSuspensionState) -> bool;

    /// Compute suspension travel direction in the world frame.
    ///
    /// The return value is the suspension travel direction in the world frame.
    ///
    /// The suspension travel direction is used to perform queries against the road geometry.
    pub fn phys_PxVehicleComputeSuspensionDirection(suspensionParams: *const PxVehicleSuspensionParams, rigidBodyPose: *const PxTransform) -> PxVec3;

    /// Compute the start pose of a suspension query.
    pub fn phys_PxVehicleComputeWheelPoseForSuspensionQuery(frame: *const PxVehicleFrame, suspensionParams: *const PxVehicleSuspensionParams, steerAngle: f32, rigidBodyPose: *const PxTransform) -> PxTransform;

    /// Compute the start point, direction and length of a suspension scene raycast.
    ///
    /// start, dir and dist together describe a raycast that begins at the top of wheel at maximum compression
    /// and ends at the bottom of wheel at maximum droop.
    pub fn phys_PxVehicleComputeSuspensionRaycast(frame: *const PxVehicleFrame, wheelParams: *const PxVehicleWheelParams, suspensionParams: *const PxVehicleSuspensionParams, steerAngle: f32, rigidBodyPose: *const PxTransform, start: *mut PxVec3, dir: *mut PxVec3, dist: *mut f32);

    /// Compute the start pose, direction and length of a suspension scene sweep.
    ///
    /// start, dir and dist together describe a sweep that begins with the wheel placed at maximum
    /// compression and ends at the maximum droop pose.
    pub fn phys_PxVehicleComputeSuspensionSweep(frame: *const PxVehicleFrame, suspensionParams: *const PxVehicleSuspensionParams, steerAngle: f32, rigidBodyPose: *const PxTransform, start: *mut PxTransform, dir: *mut PxVec3, dist: *mut f32);

    /// Compute the sprung masses of the suspension springs given (i) the number of sprung masses,
    /// (ii) coordinates of the sprung masses in the rigid body frame, (iii) the center of mass offset of the rigid body, (iv) the
    /// total mass of the rigid body, and (v) the direction of gravity
    ///
    /// True if the sprung masses were successfully computed, false if the sprung masses were not successfully computed.
    pub fn phys_PxVehicleComputeSprungMasses(nbSprungMasses: u32, sprungMassCoordinates: *const PxVec3, totalMass: f32, gravityDirection: PxVehicleAxes, sprungMasses: *mut f32) -> bool;

    /// Compute the suspension compression and compression speed for a single suspension.
    pub fn phys_PxVehicleSuspensionStateUpdate(wheelParams: *const PxVehicleWheelParams, suspensionParams: *const PxVehicleSuspensionParams, suspensionStateCalcParams: *const PxVehicleSuspensionStateCalculationParams, suspensionStiffness: f32, suspensionDamping: f32, steerAngle: f32, roadGeometryState: *const PxVehicleRoadGeometryState, rigidBodyState: *const PxVehicleRigidBodyState, dt: f32, frame: *const PxVehicleFrame, gravity: *const PxVec3, suspState: *mut PxVehicleSuspensionState);

    /// Compute the toe, camber and force application points that are affected by suspension compression.
    pub fn phys_PxVehicleSuspensionComplianceUpdate(suspensionParams: *const PxVehicleSuspensionParams, complianceParams: *const PxVehicleSuspensionComplianceParams, suspensionState: *const PxVehicleSuspensionState, complianceState: *mut PxVehicleSuspensionComplianceState);

    /// Compute the suspension force and torque arising from suspension compression and speed.
    pub fn phys_PxVehicleSuspensionForceUpdate(suspensionParams: *const PxVehicleSuspensionParams, suspensionForceParams: *const PxVehicleSuspensionForceParams, roadGeometryState: *const PxVehicleRoadGeometryState, suspensionState: *const PxVehicleSuspensionState, complianceState: *const PxVehicleSuspensionComplianceState, rigidBodyState: *const PxVehicleRigidBodyState, gravity: *const PxVec3, vehicleMass: f32, suspensionForce: *mut PxVehicleSuspensionForce);

    /// This API was introduced with the new Vehicle API for transition purposes but will be removed in a future version.
    ///
    /// Compute the suspension force and torque arising from suspension compression and speed.
    ///
    /// PxVehicleSuspensionLegacyForceUpdate implements the legacy force computation of PhysX 5.0 and earlier.
    pub fn phys_PxVehicleSuspensionLegacyForceUpdate(suspensionParams: *const PxVehicleSuspensionParams, suspensionForceParams: *const PxVehicleSuspensionForceLegacyParams, roadGeometryState: *const PxVehicleRoadGeometryState, suspensionState: *const PxVehicleSuspensionState, complianceState: *const PxVehicleSuspensionComplianceState, rigidBodyState: *const PxVehicleRigidBodyState, gravity: *const PxVec3, suspensionForce: *mut PxVehicleSuspensionForce);

    pub fn PxVehicleSuspensionComponent_delete(self_: *mut PxVehicleSuspensionComponent);

    /// Update the suspension state and suspension compliance state and use those updated states to compute suspension and anti-roll forces/torques
    /// to apply to the vehicle's rigid body.
    ///
    /// The suspension and anti-roll forces/torques are computed in the world frame.
    pub fn PxVehicleSuspensionComponent_update_mut(self_: *mut PxVehicleSuspensionComponent, dt: f32, context: *const PxVehicleSimulationContext) -> bool;

    pub fn PxVehicleLegacySuspensionComponent_delete(self_: *mut PxVehicleLegacySuspensionComponent);

    /// Update the suspension state and suspension compliance state and use those updated states to compute suspension and anti-roll forces/torques
    /// to apply to the vehicle's rigid body.
    ///
    /// The suspension and anti-roll forces are computed in the world frame.
    ///
    /// PxVehicleLegacySuspensionComponent::update() implements legacy suspension behaviour.
    pub fn PxVehicleLegacySuspensionComponent_update_mut(self_: *mut PxVehicleLegacySuspensionComponent, dt: f32, context: *const PxVehicleSimulationContext) -> bool;

    /// This API was introduced with the new Vehicle API for transition purposes but will be removed in a future version.
    ///
    /// Compute the longitudinal and lateral tire directions in the ground plane.
    ///
    /// PxVehicleTireDirsLegacyUpdate replicates the tire direction calculation of PhysX 5.0 and earlier.
    pub fn phys_PxVehicleTireDirsLegacyUpdate(suspensionParams: *const PxVehicleSuspensionParams, steerAngle: f32, roadGeometryState: *const PxVehicleRoadGeometryState, rigidBodyState: *const PxVehicleRigidBodyState, frame: *const PxVehicleFrame, tireDirectionState: *mut PxVehicleTireDirectionState);

    /// Compute the longitudinal and lateral tire directions in the ground plane.
    ///
    /// The difference between PxVehicleTireDirsUpdate and PxVehicleTireDirsLegacyUpdate is that
    /// PxVehicleTireDirsUpdate accounts for suspension compliance while PxVehicleTireDirsLegacyUpdate does not.
    pub fn phys_PxVehicleTireDirsUpdate(suspensionParams: *const PxVehicleSuspensionParams, steerAngle: f32, groundNormal: *const PxVec3, isWheelOnGround: bool, complianceState: *const PxVehicleSuspensionComplianceState, rigidBodyState: *const PxVehicleRigidBodyState, frame: *const PxVehicleFrame, tireDirectionState: *mut PxVehicleTireDirectionState);

    /// Project the rigid body velocity at the tire  contact point along the tire longitudinal directions.
    pub fn phys_PxVehicleTireSlipSpeedsUpdate(wheelParams: *const PxVehicleWheelParams, suspensionParams: *const PxVehicleSuspensionParams, steerAngle: f32, suspensionStates: *const PxVehicleSuspensionState, tireDirectionState: *const PxVehicleTireDirectionState, rigidBodyState: *const PxVehicleRigidBodyState, roadGeometryState: *const PxVehicleRoadGeometryState, frame: *const PxVehicleFrame, tireSpeedState: *mut PxVehicleTireSpeedState);

    /// Compute a tire's longitudinal and lateral slip angles.
    ///
    /// Longitudinal slip angle has the following theoretical form: (wheelRotationSpeed*wheelRadius - longitudinalSpeed)/|longitudinalSpeed|
    ///
    /// Lateral slip angle has the following theoretical form: atan(lateralSpeed/|longitudinalSpeed|)
    ///
    /// The calculation of both longitudinal and lateral slip angles avoid a zero denominator using minimum values for the denominator set in
    /// tireSlipParams.
    pub fn phys_PxVehicleTireSlipsUpdate(wheelParams: *const PxVehicleWheelParams, tireSlipParams: *const PxVehicleTireSlipParams, actuationState: *const PxVehicleWheelActuationState, tireSpeedState: *mut PxVehicleTireSpeedState, wheelRigidBody1dState: *const PxVehicleWheelRigidBody1dState, tireSlipState: *mut PxVehicleTireSlipState);

    /// This API was introduced with the new Vehicle API for transition purposes but will be removed in a future version.
    ///
    /// Compute a tire's longitudinal and lateral slip angles.
    ///
    /// Longitudinal slip angle has the following theoretical form: (wheelRotationSpeed*wheelRadius - longitudinalSpeed)/|longitudinalSpeed|
    ///
    /// Lateral slip angle has the following theoretical form: atan(lateralSpeed/|longitudinalSpeed|)
    ///
    /// The calculation of both longitudinal and lateral slip angles avoid a zero denominator using minimum values for the denominator set in
    /// tireSlipParams.
    pub fn phys_PxVehicleTireSlipsLegacyUpdate(wheelParams: *const PxVehicleWheelParams, tireSlipParams: *const PxVehicleTireSlipParams, actuationState: *const PxVehicleWheelActuationState, tireSpeedState: *mut PxVehicleTireSpeedState, wheelRigidBody1dState: *const PxVehicleWheelRigidBody1dState, tireSlipState: *mut PxVehicleTireSlipState);

    /// Compute the camber angle of  the wheel
    pub fn phys_PxVehicleTireCamberAnglesUpdate(suspensionParams: *const PxVehicleSuspensionParams, steerAngle: f32, groundNormal: *const PxVec3, isWheelOnGround: bool, complianceState: *const PxVehicleSuspensionComplianceState, rigidBodyState: *const PxVehicleRigidBodyState, frame: *const PxVehicleFrame, tireCamberAngleState: *mut PxVehicleTireCamberAngleState);

    /// Compute the load and friction experienced by the tire.
    ///
    /// If the suspension cannot place the wheel on the ground the tire load and friction will be 0.0.
    pub fn phys_PxVehicleTireGripUpdate(tireForceParams: *const PxVehicleTireForceParams, frictionCoefficient: f32, isWheelOnGround: bool, suspensionForce: *const PxVehicleSuspensionForce, tireSlipState: *const PxVehicleTireSlipState, tireGripState: *mut PxVehicleTireGripState);

    /// Set the tire longitudinal and lateral slip values to 0.0 in the event that the tire has entred tire sticky state. This is
    /// necessary to avoid both tire models being simultaneously active and interfering with each other.
    ///
    /// This function should not be invoked if there is no subsequent component to implement the sticky tire model.
    pub fn phys_PxVehicleTireSlipsAccountingForStickyStatesUpdate(tireStickyState: *const PxVehicleTireStickyState, tireSlipState: *mut PxVehicleTireSlipState);

    /// Compute the longitudinal and lateral forces in the world frame that develop on the tire as a consequence of
    /// the tire's slip angles, friction and load.
    pub fn phys_PxVehicleTireForcesUpdate(wheelParams: *const PxVehicleWheelParams, suspensionParams: *const PxVehicleSuspensionParams, tireForceParams: *const PxVehicleTireForceParams, complianceState: *const PxVehicleSuspensionComplianceState, tireGripState: *const PxVehicleTireGripState, tireDirectionState: *const PxVehicleTireDirectionState, tireSlipState: *const PxVehicleTireSlipState, tireCamberAngleState: *const PxVehicleTireCamberAngleState, rigidBodyState: *const PxVehicleRigidBodyState, tireForce: *mut PxVehicleTireForce);

    pub fn PxVehicleTireComponent_delete(self_: *mut PxVehicleTireComponent);

    pub fn PxVehicleTireComponent_update_mut(self_: *mut PxVehicleTireComponent, dt: f32, context: *const PxVehicleSimulationContext) -> bool;

    pub fn PxVehicleLegacyTireComponent_delete(self_: *mut PxVehicleLegacyTireComponent);

    pub fn PxVehicleLegacyTireComponent_update_mut(self_: *mut PxVehicleLegacyTireComponent, dt: f32, context: *const PxVehicleSimulationContext) -> bool;

    /// Forward integrate the rotation angle of a wheel
    ///
    /// The rotation angle of the wheel plays no role in simulation but is important to compute the pose of the wheel for rendering.
    ///
    /// At low speeds and large  timesteps, wheel rotation speed can become noisy due to singularities in the tire slip computations.
    /// At low speeds, therefore, the wheel speed used for integrating the angle is a blend of current angular speed and rolling angular speed if the
    /// wheel experiences neither brake nor drive torque and can be placed on the ground. The blended rotation speed gets stored in
    /// PxVehicleWheelRigidBody1dState::correctedRotationSpeed.
    pub fn phys_PxVehicleWheelRotationAngleUpdate(wheelParams: *const PxVehicleWheelParams, actuationState: *const PxVehicleWheelActuationState, suspensionState: *const PxVehicleSuspensionState, tireSpeedState: *const PxVehicleTireSpeedState, thresholdForwardSpeedForWheelAngleIntegration: f32, dt: f32, wheelRigidBody1dState: *mut PxVehicleWheelRigidBody1dState);

    pub fn PxVehicleWheelComponent_delete(self_: *mut PxVehicleWheelComponent);

    pub fn PxVehicleWheelComponent_update_mut(self_: *mut PxVehicleWheelComponent, dt: f32, context: *const PxVehicleSimulationContext) -> bool;

    /// Initialize the PhysX Vehicle library.
    ///
    /// This should be called before calling any functions or methods in extensions which may require allocation.
    ///
    /// This function does not need to be called before creating a PxDefaultAllocator object.
    pub fn phys_PxInitVehicleExtension(foundation: *mut PxFoundation) -> bool;

    /// Shut down the PhysX Vehicle library.
    ///
    /// This function should be called to cleanly shut down the PhysX Vehicle library before application exit.
    ///
    /// This function is required to be called to release foundation usage.
    pub fn phys_PxCloseVehicleExtension();

    /// Connects the SDK to the PhysX Visual Debugger application.
    pub fn PxPvd_connect_mut(self_: *mut PxPvd, transport: *mut PxPvdTransport, flags: PxPvdInstrumentationFlags) -> bool;

    /// Disconnects the SDK from the PhysX Visual Debugger application.
    /// If we are still connected, this will kill the entire debugger connection.
    pub fn PxPvd_disconnect_mut(self_: *mut PxPvd);

    /// Return if connection to PVD is created.
    pub fn PxPvd_isConnected_mut(self_: *mut PxPvd, useCachedStatus: bool) -> bool;

    /// returns the PVD data transport
    /// returns NULL if no transport is present.
    pub fn PxPvd_getTransport_mut(self_: *mut PxPvd) -> *mut PxPvdTransport;

    /// Retrieves the PVD flags. See PxPvdInstrumentationFlags.
    pub fn PxPvd_getInstrumentationFlags_mut(self_: *mut PxPvd) -> PxPvdInstrumentationFlags;

    /// Releases the pvd instance.
    pub fn PxPvd_release_mut(self_: *mut PxPvd);

    /// Create a pvd instance.
    pub fn phys_PxCreatePvd(foundation: *mut PxFoundation) -> *mut PxPvd;

    /// Connects to the Visual Debugger application.
    /// return True if success
    pub fn PxPvdTransport_connect_mut(self_: *mut PxPvdTransport) -> bool;

    /// Disconnects from the Visual Debugger application.
    /// If we are still connected, this will kill the entire debugger connection.
    pub fn PxPvdTransport_disconnect_mut(self_: *mut PxPvdTransport);

    /// Return if connection to PVD is created.
    pub fn PxPvdTransport_isConnected_mut(self_: *mut PxPvdTransport) -> bool;

    /// write bytes to the other endpoint of the connection. should lock before witre. If an error occurs
    /// this connection will assume to be dead.
    pub fn PxPvdTransport_write_mut(self_: *mut PxPvdTransport, inBytes: *const u8, inLength: u32) -> bool;

    pub fn PxPvdTransport_lock_mut(self_: *mut PxPvdTransport) -> *mut PxPvdTransport;

    pub fn PxPvdTransport_unlock_mut(self_: *mut PxPvdTransport);

    /// send any data and block until we know it is at least on the wire.
    pub fn PxPvdTransport_flush_mut(self_: *mut PxPvdTransport);

    /// Return size of written data.
    pub fn PxPvdTransport_getWrittenDataSize_mut(self_: *mut PxPvdTransport) -> u64;

    pub fn PxPvdTransport_release_mut(self_: *mut PxPvdTransport);

    /// Create a default socket transport.
    pub fn phys_PxDefaultPvdSocketTransportCreate(host: *const std::ffi::c_char, port: i32, timeoutInMilliseconds: u32) -> *mut PxPvdTransport;

    /// Create a default file transport.
    pub fn phys_PxDefaultPvdFileTransportCreate(name: *const std::ffi::c_char) -> *mut PxPvdTransport;

}
