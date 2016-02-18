//! Custom enumerators not specifically based on OpenCL C-style enums.
//!
//! Bleh. Implementing these sucks.
//! 
//! TODO: ADD ERROR VARIANT FOR EACH OF THE RESULT ENUMS.

#![allow(dead_code)]

use std;
use std::convert::Into;
use libc::{size_t, c_void};
use util;
use raw::{MemRaw, SamplerRaw, PlatformInfo, DeviceIdRaw, ContextInfo, ContextRaw, CommandQueueProperties};
use error::{Result as OclResult};
// use cl_h;

// Until everything can be implemented:
pub type TemporaryPlaceholderType = ();


/// Kernel argument option type.
///
/// The type argument `T` is ignored for `Mem`, `Sampler`, and `Other` 
/// (just put `usize` or anything).
///
pub enum KernelArg<'a, T: 'a> {
    /// Type `T` is ignored.
    Mem(MemRaw),
    /// Type `T` is ignored.
    Sampler(SamplerRaw),
    Scalar(&'a T),
    Vector(&'a [T]),
    /// Length in multiples of T (not bytes).
    Local(usize),
    /// `size`: size in bytes. Type `T` is ignored.
    Other { size: size_t, value: *const c_void },
}

/// Platform info result.
/// TODO: ADD ERROR VARIANT.
// #[derive(Clone, Copy, Debug, PartialEq)]
pub enum PlatformInfoResult {
    Profile(String),
    Version(String),
    Name(String),
    Vendor(String),
    Extensions(String),
    // Error(Box<OclError>),
}

impl PlatformInfoResult {
    pub fn new(request_param: PlatformInfo, result_string: Vec<u8>) -> OclResult<PlatformInfoResult> {
        // match result_string {
        //     Ok(rs) => {
        //         // let string = String::from_utf8(result_string).expect("FIXME: src/raw/custom/enums.rs");
        //         let string = try!(String::from_utf8(rs));

        //         Ok(match request_param {
        //             PlatformInfo::Profile => PlatformInfoResult::Profile(string),
        //             PlatformInfo::Version => PlatformInfoResult::Version(string),
        //             PlatformInfo::Name => PlatformInfoResult::Name(string),
        //             PlatformInfo::Vendor => PlatformInfoResult::Vendor(string),
        //             PlatformInfo::Extensions => PlatformInfoResult::Extensions(string),
        //         })
        //     },
        //     Err(err) => {
        //         PlatformInfoResult::Error(Box::new(err.clone))
        //     }
        // }

        let string = try!(String::from_utf8(result_string));

        Ok(match request_param {
            PlatformInfo::Profile => PlatformInfoResult::Profile(string),
            PlatformInfo::Version => PlatformInfoResult::Version(string),
            PlatformInfo::Name => PlatformInfoResult::Name(string),
            PlatformInfo::Vendor => PlatformInfoResult::Vendor(string),
            PlatformInfo::Extensions => PlatformInfoResult::Extensions(string),
        })
    }

    pub fn to_string(&self) -> String {
        self.as_str().to_string()
    }

    pub fn as_str(&self) -> &str {
        match self {
            &PlatformInfoResult::Profile(ref s) => s,
            &PlatformInfoResult::Version(ref s) => s,
            &PlatformInfoResult::Name(ref s) => s,
            &PlatformInfoResult::Vendor(ref s) => s,
            &PlatformInfoResult::Extensions(ref s) => s,
            // &PlatformInfoResult::Error(ref err) => err.description(),
        }
    }
}

impl Into<String> for PlatformInfoResult {
    fn into(self) -> String {
        match self {
            PlatformInfoResult::Profile(string) => string,
            PlatformInfoResult::Version(string) => string,
            PlatformInfoResult::Name(string) => string,
            PlatformInfoResult::Vendor(string) => string,
            PlatformInfoResult::Extensions(string) => string,
            // PlatformInfoResult::Error(err) => (*err).description().to_string(),
        }
    }
}

// impl std::error::Error for PlatformInfoResult {
//     fn description(&self) -> &str {
//         self.as_str()
//     }
// }

impl std::fmt::Debug for PlatformInfoResult {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl std::fmt::Display for PlatformInfoResult {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// A device info result.
///
/// [FIXME]: Implement the rest of this beast... eventually...
pub enum DeviceInfoResult {
    TemporaryPlaceholderVariant(Vec<u8>),
    Type(TemporaryPlaceholderType),
    VendorId(TemporaryPlaceholderType),
    MaxComputeUnits(TemporaryPlaceholderType),
    MaxWorkItemDimensions(TemporaryPlaceholderType),
    MaxWorkGroupSize(TemporaryPlaceholderType),
    MaxWorkItemSizes(TemporaryPlaceholderType),
    PreferredVectorWidthChar(TemporaryPlaceholderType),
    PreferredVectorWidthShort(TemporaryPlaceholderType),
    PreferredVectorWidthInt(TemporaryPlaceholderType),
    PreferredVectorWidthLong(TemporaryPlaceholderType),
    PreferredVectorWidthFloat(TemporaryPlaceholderType),
    PreferredVectorWidthDouble(TemporaryPlaceholderType),
    MaxClockFrequency(TemporaryPlaceholderType),
    AddressBits(TemporaryPlaceholderType),
    MaxReadImageArgs(TemporaryPlaceholderType),
    MaxWriteImageArgs(TemporaryPlaceholderType),
    MaxMemAllocSize(TemporaryPlaceholderType),
    Image2dMaxWidth(TemporaryPlaceholderType),
    Image2dMaxHeight(TemporaryPlaceholderType),
    Image3dMaxWidth(TemporaryPlaceholderType),
    Image3dMaxHeight(TemporaryPlaceholderType),
    Image3dMaxDepth(TemporaryPlaceholderType),
    ImageSupport(TemporaryPlaceholderType),
    MaxParameterSize(TemporaryPlaceholderType),
    MaxSamplers(TemporaryPlaceholderType),
    MemBaseAddrAlign(TemporaryPlaceholderType),
    MinDataTypeAlignSize(TemporaryPlaceholderType),
    SingleFpConfig(TemporaryPlaceholderType),
    GlobalMemCacheType(TemporaryPlaceholderType),
    GlobalMemCachelineSize(TemporaryPlaceholderType),
    GlobalMemCacheSize(TemporaryPlaceholderType),
    GlobalMemSize(TemporaryPlaceholderType),
    MaxConstantBufferSize(TemporaryPlaceholderType),
    MaxConstantArgs(TemporaryPlaceholderType),
    LocalMemType(TemporaryPlaceholderType),
    LocalMemSize(TemporaryPlaceholderType),
    ErrorCorrectionSupport(TemporaryPlaceholderType),
    ProfilingTimerResolution(TemporaryPlaceholderType),
    EndianLittle(TemporaryPlaceholderType),
    Available(TemporaryPlaceholderType),
    CompilerAvailable(TemporaryPlaceholderType),
    ExecutionCapabilities(TemporaryPlaceholderType),
    QueueProperties(TemporaryPlaceholderType),
    Name(TemporaryPlaceholderType),
    Vendor(TemporaryPlaceholderType),
    DriverVersion(TemporaryPlaceholderType),
    Profile(TemporaryPlaceholderType),
    Version(TemporaryPlaceholderType),
    Extensions(TemporaryPlaceholderType),
    Platform(TemporaryPlaceholderType),
    DoubleFpConfig(TemporaryPlaceholderType),
    HalfFpConfig(TemporaryPlaceholderType),
    PreferredVectorWidthHalf(TemporaryPlaceholderType),
    HostUnifiedMemory(TemporaryPlaceholderType),
    NativeVectorWidthChar(TemporaryPlaceholderType),
    NativeVectorWidthShort(TemporaryPlaceholderType),
    NativeVectorWidthInt(TemporaryPlaceholderType),
    NativeVectorWidthLong(TemporaryPlaceholderType),
    NativeVectorWidthFloat(TemporaryPlaceholderType),
    NativeVectorWidthDouble(TemporaryPlaceholderType),
    NativeVectorWidthHalf(TemporaryPlaceholderType),
    OpenclCVersion(TemporaryPlaceholderType),
    LinkerAvailable(TemporaryPlaceholderType),
    BuiltInKernels(TemporaryPlaceholderType),
    ImageMaxBufferSize(TemporaryPlaceholderType),
    ImageMaxArraySize(TemporaryPlaceholderType),
    ParentDevice(TemporaryPlaceholderType),
    PartitionMaxSubDevices(TemporaryPlaceholderType),
    PartitionProperties(TemporaryPlaceholderType),
    PartitionAffinityDomain(TemporaryPlaceholderType),
    PartitionType(TemporaryPlaceholderType),
    ReferenceCount(TemporaryPlaceholderType),
    PreferredInteropUserSync(TemporaryPlaceholderType),
    PrintfBufferSize(TemporaryPlaceholderType),
    ImagePitchAlignment(TemporaryPlaceholderType),
    ImageBaseAddressAlignment(TemporaryPlaceholderType),
    // Error(Box<OclError>),
}

impl DeviceInfoResult {
    // [FIXME]: THIS IS A JANKY MOFO (a what?).
    // NOTE: Interestingly, this actually works sorta decently as long as there
    // isn't a 4, 8, or 24 character string. Ummm, yeah. Fix this.
    pub fn to_string(&self) -> String {
        match self {
            &DeviceInfoResult::TemporaryPlaceholderVariant(ref v) => {
                // TEMPORARY (and retarded):
                to_string_retarded(v)
            },
            _ => panic!("DeviceInfoResult: Converting this variant to string not yet implemented."),
        }
    }
}

impl std::fmt::Debug for DeviceInfoResult {
    /// [INCOMPLETE]: TEMPORARY
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", &self.to_string())
    }
}

impl std::fmt::Display for DeviceInfoResult {
    /// [INCOMPLETE]: TEMPORARY
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", &self.to_string())
    }
}



/// [UNSTABLE][INCOMPLETE] A context info result.
///
/// [FIXME]: Figure out what to do with the properties variant.
pub enum ContextInfoResult {
    ReferenceCount(u32),
    Devices(Vec<DeviceIdRaw>),
    Properties(Vec<u8>),
    NumDevices(u32),
    // Error(Box<OclError>),
}

impl ContextInfoResult {
    pub fn new(request_param: ContextInfo, result: Vec<u8>) -> OclResult<ContextInfoResult> {
        Ok(match request_param {
            ContextInfo::ReferenceCount => {
                ContextInfoResult::ReferenceCount(util::bytes_to_u32(&result))
            },
            ContextInfo::Devices => {
                ContextInfoResult::Devices(
                    unsafe { util::bytes_into_vec::<DeviceIdRaw>(result) }
                )
            },
            ContextInfo::Properties => {
                // unsafe { ContextInfoResult::Properties(
                //     ContextInfoOrPropertiesPointerType.from_u32(util::bytes_into::
                //         <cl_h::cl_context_properties>(result))
                // ) }
                ContextInfoResult::Properties(result)
            },
            ContextInfo::NumDevices => {
                ContextInfoResult::NumDevices(util::bytes_to_u32(&result))
            },
        })
    }

    pub fn to_string(&self) -> String {
        match self {
            &ContextInfoResult::ReferenceCount(ref count) => count.to_string(),
            &ContextInfoResult::Devices(ref vec) => format!("{:?}", vec),
            &ContextInfoResult::Properties(ref props) => format!("{:?}", props),
            &ContextInfoResult::NumDevices(ref num) => num.to_string(),
            // &ContextInfoResult::Error(ref err) => err.description().to_string(),
        }
    }
}

impl std::fmt::Debug for ContextInfoResult {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", &self.to_string())
    }
}

impl std::fmt::Display for ContextInfoResult {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", &self.to_string())
    }
}


/// [UNSTABLE][INCOMPLETE] A command queue info result.
pub enum CommandQueueInfoResult {
    TemporaryPlaceholderVariant(Vec<u8>),
    Context(ContextRaw),
    Device(DeviceIdRaw),
    ReferenceCount(u32),
    Properties(CommandQueueProperties),
    // Error(Box<OclError>),
}

impl CommandQueueInfoResult {
    // TODO: IMPLEMENT THIS PROPERLY.
    pub fn to_string(&self) -> String {
        match self {
            &CommandQueueInfoResult::TemporaryPlaceholderVariant(ref v) => {
               to_string_retarded(v)
            },
            _ => panic!("CommandQueueInfoResult: Converting this variant to string not yet implemented."),
        }
    }
}

impl std::fmt::Debug for CommandQueueInfoResult {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", &self.to_string())
    }
}

impl std::fmt::Display for CommandQueueInfoResult {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", &self.to_string())
    }
}



/// [UNSTABLE][INCOMPLETE] A mem info result.
pub enum MemInfoResult {
    TemporaryPlaceholderVariant(Vec<u8>),
    Type(TemporaryPlaceholderType),
    Flags(TemporaryPlaceholderType),
    Size(TemporaryPlaceholderType),
    HostPtr(TemporaryPlaceholderType),
    MapCount(TemporaryPlaceholderType),
    ReferenceCount(TemporaryPlaceholderType),
    Context(TemporaryPlaceholderType),
    AssociatedMemobject(TemporaryPlaceholderType),
    Offset(TemporaryPlaceholderType),
    // Error(Box<OclError>),
}

impl MemInfoResult {
    // TODO: IMPLEMENT THIS PROPERLY.
    pub fn to_string(&self) -> String {
        match self {
            &MemInfoResult::TemporaryPlaceholderVariant(ref v) => {
               to_string_retarded(v)
            },
            _ => panic!("MemInfoResult: Converting this variant to string not yet implemented."),
        }
    }
}

impl std::fmt::Debug for MemInfoResult {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", &self.to_string())
    }
}

impl std::fmt::Display for MemInfoResult {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", &self.to_string())
    }
}



/// [UNSTABLE][INCOMPLETE] An image info result.
pub enum ImageInfoResult {
    TemporaryPlaceholderVariant(Vec<u8>),
    Format(TemporaryPlaceholderType),
    ElementSize(TemporaryPlaceholderType),
    RowPitch(TemporaryPlaceholderType),
    SlicePitch(TemporaryPlaceholderType),
    Width(TemporaryPlaceholderType),
    Height(TemporaryPlaceholderType),
    Depth(TemporaryPlaceholderType),
    ArraySize(TemporaryPlaceholderType),
    Buffer(TemporaryPlaceholderType),
    NumMipLevels(TemporaryPlaceholderType),
    NumSamples(TemporaryPlaceholderType),
    // Error(Box<OclError>),
}

impl ImageInfoResult {
    // TODO: IMPLEMENT THIS PROPERLY.
    pub fn to_string(&self) -> String {
        match self {
            &ImageInfoResult::TemporaryPlaceholderVariant(ref v) => {
               to_string_retarded(v)
            },
            _ => panic!("ImageInfoResult: Converting this variant to string not yet implemented."),
        }
    }
}

impl std::fmt::Debug for ImageInfoResult {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", &self.to_string())
    }
}

impl std::fmt::Display for ImageInfoResult {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", &self.to_string())
    }
}



/// [UNSTABLE][INCOMPLETE] A sampler info result.
pub enum SamplerInfoResult {
    TemporaryPlaceholderVariant(Vec<u8>),
    ReferenceCount(TemporaryPlaceholderType),
    Context(TemporaryPlaceholderType),
    NormalizedCoords(TemporaryPlaceholderType),
    AddressingMode(TemporaryPlaceholderType),
    FilterMode(TemporaryPlaceholderType),
    // Error(Box<OclError>),
}

impl SamplerInfoResult {
    // TODO: IMPLEMENT THIS PROPERLY.
    pub fn to_string(&self) -> String {
        match self {
            &SamplerInfoResult::TemporaryPlaceholderVariant(ref v) => {
               to_string_retarded(v)
            },
            _ => panic!("SamplerInfoResult: Converting this variant to string not yet implemented."),
        }
    }
}

impl std::fmt::Debug for SamplerInfoResult {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", &self.to_string())
    }
}

impl std::fmt::Display for SamplerInfoResult {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", &self.to_string())
    }
}



/// [UNSTABLE][INCOMPLETE] A program info result.
pub enum ProgramInfoResult {
    TemporaryPlaceholderVariant(Vec<u8>),
    ReferenceCount(TemporaryPlaceholderType),
    Context(TemporaryPlaceholderType),
    NumDevices(TemporaryPlaceholderType),
    Devices(TemporaryPlaceholderType),
    Source(TemporaryPlaceholderType),
    BinarySizes(TemporaryPlaceholderType),
    Binaries(TemporaryPlaceholderType),
    NumKernels(TemporaryPlaceholderType),
    KernelNames(TemporaryPlaceholderType),
    // Error(Box<OclError>),
}

impl ProgramInfoResult {
    // TODO: IMPLEMENT THIS PROPERLY.
    pub fn to_string(&self) -> String {
        match self {
            &ProgramInfoResult::TemporaryPlaceholderVariant(ref v) => {
               to_string_retarded(v)
            },
            _ => panic!("ProgramInfoResult: Converting this variant to string not yet implemented."),
        }
    }
}

impl std::fmt::Debug for ProgramInfoResult {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", &self.to_string())
    }
}

impl std::fmt::Display for ProgramInfoResult {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", &self.to_string())
    }
}



/// [UNSTABLE][INCOMPLETE] A program build info result.
pub enum ProgramBuildInfoResult {
    TemporaryPlaceholderVariant(Vec<u8>),
    BuildStatus(TemporaryPlaceholderType),
    BuildOptions(TemporaryPlaceholderType),
    BuildLog(TemporaryPlaceholderType),
    BinaryType(TemporaryPlaceholderType),
    // Error(Box<OclError>),
}

impl ProgramBuildInfoResult {
    // TODO: IMPLEMENT THIS PROPERLY.
    pub fn to_string(&self) -> String {
        match self {
            &ProgramBuildInfoResult::TemporaryPlaceholderVariant(ref v) => {
               to_string_retarded(v)
            },
            _ => panic!("ProgramBuildInfoResult: Converting this variant to string not yet implemented."),
        }
    }
}

impl std::fmt::Debug for ProgramBuildInfoResult {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", &self.to_string())
    }
}

impl std::fmt::Display for ProgramBuildInfoResult {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", &self.to_string())
    }
}



/// [UNSTABLE][INCOMPLETE] A kernel info result.
pub enum KernelInfoResult {
    TemporaryPlaceholderVariant(Vec<u8>),
    FunctionName(TemporaryPlaceholderType),
    NumArgs(TemporaryPlaceholderType),
    ReferenceCount(TemporaryPlaceholderType),
    Context(TemporaryPlaceholderType),
    Program(TemporaryPlaceholderType),
    Attributes(TemporaryPlaceholderType),
    // Error(Box<OclError>),
}

impl KernelInfoResult {
    // TODO: IMPLEMENT THIS PROPERLY.
    pub fn to_string(&self) -> String {
        match self {
            &KernelInfoResult::TemporaryPlaceholderVariant(ref v) => {
               to_string_retarded(v)
            },
            _ => panic!("KernelInfoResult: Converting this variant to string not yet implemented."),
        }
    }
}

impl std::fmt::Debug for KernelInfoResult {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", &self.to_string())
    }
}

impl std::fmt::Display for KernelInfoResult {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", &self.to_string())
    }
}



/// [UNSTABLE][INCOMPLETE] A kernel arg info result.
pub enum KernelArgInfoResult {
    TemporaryPlaceholderVariant(Vec<u8>),
    AddressQualifier(TemporaryPlaceholderType),
    AccessQualifier(TemporaryPlaceholderType),
    TypeName(TemporaryPlaceholderType),
    TypeQualifier(TemporaryPlaceholderType),
    Name(TemporaryPlaceholderType),
    // Error(Box<OclError>),
}

impl KernelArgInfoResult {
    // TODO: IMPLEMENT THIS PROPERLY.
    pub fn to_string(&self) -> String {
        match self {
            &KernelArgInfoResult::TemporaryPlaceholderVariant(ref v) => {
               to_string_retarded(v)
            },
            _ => panic!("KernelArgInfoResult: Converting this variant to string not yet implemented."),
        }
    }
}

impl std::fmt::Debug for KernelArgInfoResult {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", &self.to_string())
    }
}

impl std::fmt::Display for KernelArgInfoResult {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", &self.to_string())
    }
}



/// [UNSTABLE][INCOMPLETE] A kernel work groups info result.
pub enum KernelWorkGroupInfoResult {
    TemporaryPlaceholderVariant(Vec<u8>),
    WorkGroupSize(TemporaryPlaceholderType),
    CompileWorkGroupSize(TemporaryPlaceholderType),
    LocalMemSize(TemporaryPlaceholderType),
    PreferredWorkGroupSizeMultiple(TemporaryPlaceholderType),
    PrivateMemSize(TemporaryPlaceholderType),
    GlobalWorkSize(TemporaryPlaceholderType),
    // Error(Box<OclError>),
}

impl KernelWorkGroupInfoResult {
    // TODO: IMPLEMENT THIS PROPERLY.
    pub fn to_string(&self) -> String {
        match self {
            &KernelWorkGroupInfoResult::TemporaryPlaceholderVariant(ref v) => {
               to_string_retarded(v)
            },
            _ => panic!("KernelWorkGroupInfoResult: Converting this variant to string not yet implemented."),
        }
    }
}

impl std::fmt::Debug for KernelWorkGroupInfoResult {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", &self.to_string())
    }
}

impl std::fmt::Display for KernelWorkGroupInfoResult {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", &self.to_string())
    }
}



/// [UNSTABLE][INCOMPLETE] An event info result.
pub enum EventInfoResult {
    TemporaryPlaceholderVariant(Vec<u8>),
    CommandQueue(TemporaryPlaceholderType),
    CommandType(TemporaryPlaceholderType),
    ReferenceCount(TemporaryPlaceholderType),
    CommandExecutionStatus(TemporaryPlaceholderType),
    Context(TemporaryPlaceholderType),
    // Error(Box<OclError>),
}

impl EventInfoResult {
    // TODO: IMPLEMENT THIS PROPERLY.
    pub fn to_string(&self) -> String {
        match self {
            &EventInfoResult::TemporaryPlaceholderVariant(ref v) => {
               to_string_retarded(v)
            },
            _ => panic!("EventInfoResult: Converting this variant to string not yet implemented."),
        }
    }
}

impl std::fmt::Debug for EventInfoResult {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", &self.to_string())
    }
}

impl std::fmt::Display for EventInfoResult {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", &self.to_string())
    }
}



/// [UNSTABLE][INCOMPLETE] A profiling info result.
pub enum ProfilingInfoResult {
    TemporaryPlaceholderVariant(Vec<u8>),
    Queued(TemporaryPlaceholderType),
    Submit(TemporaryPlaceholderType),
    Start(TemporaryPlaceholderType),
    End(TemporaryPlaceholderType),
    // Error(Box<OclError>),
}

impl ProfilingInfoResult {
    // TODO: IMPLEMENT THIS PROPERLY.
    pub fn to_string(&self) -> String {
        match self {
            &ProfilingInfoResult::TemporaryPlaceholderVariant(ref v) => {
               to_string_retarded(v)
            },
            _ => panic!("ProfilingInfoResult: Converting this variant to string not yet implemented."),
        }
    }
}

impl std::fmt::Debug for ProfilingInfoResult {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", &self.to_string())
    }
}

impl std::fmt::Display for ProfilingInfoResult {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", &self.to_string())
    }
}



/// TEMPORARY
fn to_string_retarded(v: &Vec<u8>) -> String {
    if v.len() == 4 {
        util::bytes_to_u32(&v[..]).to_string()
    } else if v.len() == 8 {
        unsafe { util::bytes_to::<usize>(&v[..]).to_string() }
    } else if v.len() == 3 * 8 {
        unsafe { format!("{:?}", util::bytes_to_vec::<usize>(&v[..])) }
    } else {
        String::from_utf8(v.clone()).unwrap_or(format!("{:?}", v))
    }
}
