/* automatically generated by rust-bindgen 0.71.1 */

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sycl_device {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sycl_queue {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sycl_event {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUstream_st {
    _unused: [u8; 0],
}
pub type cudaStream_t = *mut CUstream_st;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ihipStream_t {
    _unused: [u8; 0],
}
pub type hipStream_t = *mut ihipStream_t;
pub type MTLDevice_id = *mut ::std::os::raw::c_void;
pub type MTLCommandQueue_id = *mut ::std::os::raw::c_void;
pub type MTLBuffer_id = *mut ::std::os::raw::c_void;
unsafe extern "C" {
    pub fn oidnGetNumPhysicalDevices() -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn oidnGetPhysicalDeviceBool(
        physicalDeviceID: ::std::os::raw::c_int,
        name: *const ::std::os::raw::c_char,
    ) -> bool;
}
unsafe extern "C" {
    pub fn oidnGetPhysicalDeviceInt(
        physicalDeviceID: ::std::os::raw::c_int,
        name: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn oidnGetPhysicalDeviceString(
        physicalDeviceID: ::std::os::raw::c_int,
        name: *const ::std::os::raw::c_char,
    ) -> *const ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn oidnGetPhysicalDeviceData(
        physicalDeviceID: ::std::os::raw::c_int,
        name: *const ::std::os::raw::c_char,
        byteSize: *mut usize,
    ) -> *const ::std::os::raw::c_void;
}
pub const OIDNDeviceType_OIDN_DEVICE_TYPE_DEFAULT: OIDNDeviceType = 0;
pub const OIDNDeviceType_OIDN_DEVICE_TYPE_CPU: OIDNDeviceType = 1;
pub const OIDNDeviceType_OIDN_DEVICE_TYPE_SYCL: OIDNDeviceType = 2;
pub const OIDNDeviceType_OIDN_DEVICE_TYPE_CUDA: OIDNDeviceType = 3;
pub const OIDNDeviceType_OIDN_DEVICE_TYPE_HIP: OIDNDeviceType = 4;
pub const OIDNDeviceType_OIDN_DEVICE_TYPE_METAL: OIDNDeviceType = 5;
pub type OIDNDeviceType = ::std::os::raw::c_uint;
pub const OIDNError_OIDN_ERROR_NONE: OIDNError = 0;
pub const OIDNError_OIDN_ERROR_UNKNOWN: OIDNError = 1;
pub const OIDNError_OIDN_ERROR_INVALID_ARGUMENT: OIDNError = 2;
pub const OIDNError_OIDN_ERROR_INVALID_OPERATION: OIDNError = 3;
pub const OIDNError_OIDN_ERROR_OUT_OF_MEMORY: OIDNError = 4;
pub const OIDNError_OIDN_ERROR_UNSUPPORTED_HARDWARE: OIDNError = 5;
pub const OIDNError_OIDN_ERROR_CANCELLED: OIDNError = 6;
pub type OIDNError = ::std::os::raw::c_uint;
pub type OIDNErrorFunction = ::std::option::Option<
    unsafe extern "C" fn(
        userPtr: *mut ::std::os::raw::c_void,
        code: OIDNError,
        message: *const ::std::os::raw::c_char,
    ),
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OIDNDeviceImpl {
    _unused: [u8; 0],
}
pub type OIDNDevice = *mut OIDNDeviceImpl;
unsafe extern "C" {
    pub fn oidnIsCPUDeviceSupported() -> bool;
}
unsafe extern "C" {
    pub fn oidnIsSYCLDeviceSupported(device: *const sycl_device) -> bool;
}
unsafe extern "C" {
    pub fn oidnIsCUDADeviceSupported(deviceID: ::std::os::raw::c_int) -> bool;
}
unsafe extern "C" {
    pub fn oidnIsHIPDeviceSupported(deviceID: ::std::os::raw::c_int) -> bool;
}
unsafe extern "C" {
    pub fn oidnIsMetalDeviceSupported(device: MTLDevice_id) -> bool;
}
unsafe extern "C" {
    pub fn oidnNewDevice(type_: OIDNDeviceType) -> OIDNDevice;
}
unsafe extern "C" {
    pub fn oidnNewDeviceByID(physicalDeviceID: ::std::os::raw::c_int) -> OIDNDevice;
}
unsafe extern "C" {
    pub fn oidnNewDeviceByUUID(uuid: *const ::std::os::raw::c_void) -> OIDNDevice;
}
unsafe extern "C" {
    pub fn oidnNewDeviceByLUID(luid: *const ::std::os::raw::c_void) -> OIDNDevice;
}
unsafe extern "C" {
    pub fn oidnNewDeviceByPCIAddress(
        pciDomain: ::std::os::raw::c_int,
        pciBus: ::std::os::raw::c_int,
        pciDevice: ::std::os::raw::c_int,
        pciFunction: ::std::os::raw::c_int,
    ) -> OIDNDevice;
}
unsafe extern "C" {
    pub fn oidnNewSYCLDevice(
        queues: *const sycl_queue,
        numQueues: ::std::os::raw::c_int,
    ) -> OIDNDevice;
}
unsafe extern "C" {
    pub fn oidnNewCUDADevice(
        deviceIDs: *const ::std::os::raw::c_int,
        streams: *const cudaStream_t,
        numPairs: ::std::os::raw::c_int,
    ) -> OIDNDevice;
}
unsafe extern "C" {
    pub fn oidnNewHIPDevice(
        deviceIDs: *const ::std::os::raw::c_int,
        streams: *const hipStream_t,
        numPairs: ::std::os::raw::c_int,
    ) -> OIDNDevice;
}
unsafe extern "C" {
    pub fn oidnNewMetalDevice(
        commandQueues: *const MTLCommandQueue_id,
        numQueues: ::std::os::raw::c_int,
    ) -> OIDNDevice;
}
unsafe extern "C" {
    pub fn oidnRetainDevice(device: OIDNDevice);
}
unsafe extern "C" {
    pub fn oidnReleaseDevice(device: OIDNDevice);
}
unsafe extern "C" {
    pub fn oidnSetDeviceBool(device: OIDNDevice, name: *const ::std::os::raw::c_char, value: bool);
}
unsafe extern "C" {
    pub fn oidnSetDeviceInt(
        device: OIDNDevice,
        name: *const ::std::os::raw::c_char,
        value: ::std::os::raw::c_int,
    );
}
unsafe extern "C" {
    pub fn oidnGetDeviceBool(device: OIDNDevice, name: *const ::std::os::raw::c_char) -> bool;
}
unsafe extern "C" {
    pub fn oidnGetDeviceInt(
        device: OIDNDevice,
        name: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn oidnSetDeviceErrorFunction(
        device: OIDNDevice,
        func: OIDNErrorFunction,
        userPtr: *mut ::std::os::raw::c_void,
    );
}
unsafe extern "C" {
    pub fn oidnGetDeviceError(
        device: OIDNDevice,
        outMessage: *mut *const ::std::os::raw::c_char,
    ) -> OIDNError;
}
unsafe extern "C" {
    pub fn oidnCommitDevice(device: OIDNDevice);
}
unsafe extern "C" {
    pub fn oidnSyncDevice(device: OIDNDevice);
}
pub const OIDNFormat_OIDN_FORMAT_UNDEFINED: OIDNFormat = 0;
pub const OIDNFormat_OIDN_FORMAT_FLOAT: OIDNFormat = 1;
pub const OIDNFormat_OIDN_FORMAT_FLOAT2: OIDNFormat = 2;
pub const OIDNFormat_OIDN_FORMAT_FLOAT3: OIDNFormat = 3;
pub const OIDNFormat_OIDN_FORMAT_FLOAT4: OIDNFormat = 4;
pub const OIDNFormat_OIDN_FORMAT_HALF: OIDNFormat = 257;
pub const OIDNFormat_OIDN_FORMAT_HALF2: OIDNFormat = 258;
pub const OIDNFormat_OIDN_FORMAT_HALF3: OIDNFormat = 259;
pub const OIDNFormat_OIDN_FORMAT_HALF4: OIDNFormat = 260;
pub type OIDNFormat = ::std::os::raw::c_uint;
pub const OIDNStorage_OIDN_STORAGE_UNDEFINED: OIDNStorage = 0;
pub const OIDNStorage_OIDN_STORAGE_HOST: OIDNStorage = 1;
pub const OIDNStorage_OIDN_STORAGE_DEVICE: OIDNStorage = 2;
pub const OIDNStorage_OIDN_STORAGE_MANAGED: OIDNStorage = 3;
pub type OIDNStorage = ::std::os::raw::c_uint;
pub const OIDNExternalMemoryTypeFlag_OIDN_EXTERNAL_MEMORY_TYPE_FLAG_NONE:
    OIDNExternalMemoryTypeFlag = 0;
pub const OIDNExternalMemoryTypeFlag_OIDN_EXTERNAL_MEMORY_TYPE_FLAG_OPAQUE_FD:
    OIDNExternalMemoryTypeFlag = 1;
pub const OIDNExternalMemoryTypeFlag_OIDN_EXTERNAL_MEMORY_TYPE_FLAG_DMA_BUF:
    OIDNExternalMemoryTypeFlag = 2;
pub const OIDNExternalMemoryTypeFlag_OIDN_EXTERNAL_MEMORY_TYPE_FLAG_OPAQUE_WIN32:
    OIDNExternalMemoryTypeFlag = 4;
pub const OIDNExternalMemoryTypeFlag_OIDN_EXTERNAL_MEMORY_TYPE_FLAG_OPAQUE_WIN32_KMT:
    OIDNExternalMemoryTypeFlag = 8;
pub const OIDNExternalMemoryTypeFlag_OIDN_EXTERNAL_MEMORY_TYPE_FLAG_D3D11_TEXTURE:
    OIDNExternalMemoryTypeFlag = 16;
pub const OIDNExternalMemoryTypeFlag_OIDN_EXTERNAL_MEMORY_TYPE_FLAG_D3D11_TEXTURE_KMT:
    OIDNExternalMemoryTypeFlag = 32;
pub const OIDNExternalMemoryTypeFlag_OIDN_EXTERNAL_MEMORY_TYPE_FLAG_D3D11_RESOURCE:
    OIDNExternalMemoryTypeFlag = 64;
pub const OIDNExternalMemoryTypeFlag_OIDN_EXTERNAL_MEMORY_TYPE_FLAG_D3D11_RESOURCE_KMT:
    OIDNExternalMemoryTypeFlag = 128;
pub const OIDNExternalMemoryTypeFlag_OIDN_EXTERNAL_MEMORY_TYPE_FLAG_D3D12_HEAP:
    OIDNExternalMemoryTypeFlag = 256;
pub const OIDNExternalMemoryTypeFlag_OIDN_EXTERNAL_MEMORY_TYPE_FLAG_D3D12_RESOURCE:
    OIDNExternalMemoryTypeFlag = 512;
pub type OIDNExternalMemoryTypeFlag = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OIDNBufferImpl {
    _unused: [u8; 0],
}
pub type OIDNBuffer = *mut OIDNBufferImpl;
unsafe extern "C" {
    pub fn oidnNewBuffer(device: OIDNDevice, byteSize: usize) -> OIDNBuffer;
}
unsafe extern "C" {
    pub fn oidnNewBufferWithStorage(
        device: OIDNDevice,
        byteSize: usize,
        storage: OIDNStorage,
    ) -> OIDNBuffer;
}
unsafe extern "C" {
    pub fn oidnNewSharedBuffer(
        device: OIDNDevice,
        devPtr: *mut ::std::os::raw::c_void,
        byteSize: usize,
    ) -> OIDNBuffer;
}
unsafe extern "C" {
    pub fn oidnNewSharedBufferFromFD(
        device: OIDNDevice,
        fdType: OIDNExternalMemoryTypeFlag,
        fd: ::std::os::raw::c_int,
        byteSize: usize,
    ) -> OIDNBuffer;
}
unsafe extern "C" {
    pub fn oidnNewSharedBufferFromWin32Handle(
        device: OIDNDevice,
        handleType: OIDNExternalMemoryTypeFlag,
        handle: *mut ::std::os::raw::c_void,
        name: *const ::std::os::raw::c_void,
        byteSize: usize,
    ) -> OIDNBuffer;
}
unsafe extern "C" {
    pub fn oidnNewSharedBufferFromMetal(device: OIDNDevice, buffer: MTLBuffer_id) -> OIDNBuffer;
}
unsafe extern "C" {
    pub fn oidnGetBufferSize(buffer: OIDNBuffer) -> usize;
}
unsafe extern "C" {
    pub fn oidnGetBufferStorage(buffer: OIDNBuffer) -> OIDNStorage;
}
unsafe extern "C" {
    pub fn oidnGetBufferData(buffer: OIDNBuffer) -> *mut ::std::os::raw::c_void;
}
unsafe extern "C" {
    pub fn oidnReadBuffer(
        buffer: OIDNBuffer,
        byteOffset: usize,
        byteSize: usize,
        dstHostPtr: *mut ::std::os::raw::c_void,
    );
}
unsafe extern "C" {
    pub fn oidnReadBufferAsync(
        buffer: OIDNBuffer,
        byteOffset: usize,
        byteSize: usize,
        dstHostPtr: *mut ::std::os::raw::c_void,
    );
}
unsafe extern "C" {
    pub fn oidnWriteBuffer(
        buffer: OIDNBuffer,
        byteOffset: usize,
        byteSize: usize,
        srcHostPtr: *const ::std::os::raw::c_void,
    );
}
unsafe extern "C" {
    pub fn oidnWriteBufferAsync(
        buffer: OIDNBuffer,
        byteOffset: usize,
        byteSize: usize,
        srcHostPtr: *const ::std::os::raw::c_void,
    );
}
unsafe extern "C" {
    pub fn oidnRetainBuffer(buffer: OIDNBuffer);
}
unsafe extern "C" {
    pub fn oidnReleaseBuffer(buffer: OIDNBuffer);
}
pub const OIDNQuality_OIDN_QUALITY_DEFAULT: OIDNQuality = 0;
pub const OIDNQuality_OIDN_QUALITY_FAST: OIDNQuality = 4;
pub const OIDNQuality_OIDN_QUALITY_BALANCED: OIDNQuality = 5;
pub const OIDNQuality_OIDN_QUALITY_HIGH: OIDNQuality = 6;
pub type OIDNQuality = ::std::os::raw::c_uint;
pub type OIDNProgressMonitorFunction = ::std::option::Option<
    unsafe extern "C" fn(userPtr: *mut ::std::os::raw::c_void, n: f64) -> bool,
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OIDNFilterImpl {
    _unused: [u8; 0],
}
pub type OIDNFilter = *mut OIDNFilterImpl;
unsafe extern "C" {
    pub fn oidnNewFilter(device: OIDNDevice, type_: *const ::std::os::raw::c_char) -> OIDNFilter;
}
unsafe extern "C" {
    pub fn oidnRetainFilter(filter: OIDNFilter);
}
unsafe extern "C" {
    pub fn oidnReleaseFilter(filter: OIDNFilter);
}
unsafe extern "C" {
    pub fn oidnSetFilterImage(
        filter: OIDNFilter,
        name: *const ::std::os::raw::c_char,
        buffer: OIDNBuffer,
        format: OIDNFormat,
        width: usize,
        height: usize,
        byteOffset: usize,
        pixelByteStride: usize,
        rowByteStride: usize,
    );
}
unsafe extern "C" {
    pub fn oidnSetSharedFilterImage(
        filter: OIDNFilter,
        name: *const ::std::os::raw::c_char,
        devPtr: *mut ::std::os::raw::c_void,
        format: OIDNFormat,
        width: usize,
        height: usize,
        byteOffset: usize,
        pixelByteStride: usize,
        rowByteStride: usize,
    );
}
unsafe extern "C" {
    pub fn oidnUnsetFilterImage(filter: OIDNFilter, name: *const ::std::os::raw::c_char);
}
unsafe extern "C" {
    pub fn oidnSetSharedFilterData(
        filter: OIDNFilter,
        name: *const ::std::os::raw::c_char,
        hostPtr: *mut ::std::os::raw::c_void,
        byteSize: usize,
    );
}
unsafe extern "C" {
    pub fn oidnUpdateFilterData(filter: OIDNFilter, name: *const ::std::os::raw::c_char);
}
unsafe extern "C" {
    pub fn oidnUnsetFilterData(filter: OIDNFilter, name: *const ::std::os::raw::c_char);
}
unsafe extern "C" {
    pub fn oidnSetFilterBool(filter: OIDNFilter, name: *const ::std::os::raw::c_char, value: bool);
}
unsafe extern "C" {
    pub fn oidnGetFilterBool(filter: OIDNFilter, name: *const ::std::os::raw::c_char) -> bool;
}
unsafe extern "C" {
    pub fn oidnSetFilterInt(
        filter: OIDNFilter,
        name: *const ::std::os::raw::c_char,
        value: ::std::os::raw::c_int,
    );
}
unsafe extern "C" {
    pub fn oidnGetFilterInt(
        filter: OIDNFilter,
        name: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn oidnSetFilterFloat(filter: OIDNFilter, name: *const ::std::os::raw::c_char, value: f32);
}
unsafe extern "C" {
    pub fn oidnGetFilterFloat(filter: OIDNFilter, name: *const ::std::os::raw::c_char) -> f32;
}
unsafe extern "C" {
    pub fn oidnSetFilterProgressMonitorFunction(
        filter: OIDNFilter,
        func: OIDNProgressMonitorFunction,
        userPtr: *mut ::std::os::raw::c_void,
    );
}
unsafe extern "C" {
    pub fn oidnCommitFilter(filter: OIDNFilter);
}
unsafe extern "C" {
    pub fn oidnExecuteFilter(filter: OIDNFilter);
}
unsafe extern "C" {
    pub fn oidnExecuteFilterAsync(filter: OIDNFilter);
}
unsafe extern "C" {
    pub fn oidnExecuteSYCLFilterAsync(
        filter: OIDNFilter,
        depEvents: *const sycl_event,
        numDepEvents: ::std::os::raw::c_int,
        doneEvent: *mut sycl_event,
    );
}
