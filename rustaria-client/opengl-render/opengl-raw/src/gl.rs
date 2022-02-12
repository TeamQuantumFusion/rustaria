#![allow(non_upper_case_globals, non_snake_case, non_camel_case_types, dead_code)]
use std::ffi::c_void;
use std::mem::transmute;
// Common types from OpenGL 1.1
pub type GLenum = std::os::raw::c_uint;
pub type GLboolean = std::os::raw::c_uchar;
pub type GLbitfield = std::os::raw::c_uint;
pub type GLvoid = std::os::raw::c_void;
pub type GLbyte = std::os::raw::c_char;
pub type GLshort = std::os::raw::c_short;
pub type GLint = std::os::raw::c_int;
pub type GLclampx = std::os::raw::c_int;
pub type GLubyte = std::os::raw::c_uchar;
pub type GLushort = std::os::raw::c_ushort;
pub type GLuint = std::os::raw::c_uint;
pub type GLsizei = std::os::raw::c_int;
pub type GLfloat = std::os::raw::c_float;
pub type GLclampf = std::os::raw::c_float;
pub type GLdouble = std::os::raw::c_double;
pub type GLclampd = std::os::raw::c_double;
pub type GLeglImageOES = *const std::os::raw::c_void;
pub type GLchar = std::os::raw::c_char;
pub type GLcharARB = std::os::raw::c_char;

#[cfg(target_os = "macos")]
pub type GLhandleARB = *const std::os::raw::c_void;
#[cfg(not(target_os = "macos"))]
pub type GLhandleARB = std::os::raw::c_uint;

pub type GLhalfARB = std::os::raw::c_ushort;
pub type GLhalf = std::os::raw::c_ushort;

// Must be 32 bits
pub type GLfixed = GLint;

pub type GLintptr = isize;
pub type GLsizeiptr = isize;
pub type GLint64 = i64;
pub type GLuint64 = u64;
pub type GLintptrARB = isize;
pub type GLsizeiptrARB = isize;
pub type GLint64EXT = i64;
pub type GLuint64EXT = u64;

pub enum __GLsync {}
pub type GLsync = *const __GLsync;

// compatible with OpenCL cl_context
pub enum _cl_context {}
pub enum _cl_event {}

pub type GLDEBUGPROC = Option<extern "system" fn(source: GLenum,
                                                 gltype: GLenum,
                                                 id: GLuint,
                                                 severity: GLenum,
                                                 length: GLsizei,
                                                 message: *const GLchar,
                                                 userParam: *mut std::os::raw::c_void)>;
pub type GLDEBUGPROCARB = Option<extern "system" fn(source: GLenum,
                                                    gltype: GLenum,
                                                    id: GLuint,
                                                    severity: GLenum,
                                                    length: GLsizei,
                                                    message: *const GLchar,
                                                    userParam: *mut std::os::raw::c_void)>;
pub type GLDEBUGPROCKHR = Option<extern "system" fn(source: GLenum,
                                                    gltype: GLenum,
                                                    id: GLuint,
                                                    severity: GLenum,
                                                    length: GLsizei,
                                                    message: *const GLchar,
                                                    userParam: *mut std::os::raw::c_void)>;

// GLES 1 types
// "pub type GLclampx = i32;",

// GLES 1/2 types (tagged for GLES 1)
// "pub type GLbyte = i8;",
// "pub type GLubyte = u8;",
// "pub type GLfloat = GLfloat;",
// "pub type GLclampf = GLfloat;",
// "pub type GLfixed = i32;",
// "pub type GLint64 = i64;",
// "pub type GLuint64 = u64;",
// "pub type GLintptr = intptr_t;",
// "pub type GLsizeiptr = ssize_t;",

// GLES 1/2 types (tagged for GLES 2 - attribute syntax is limited)
// "pub type GLbyte = i8;",
// "pub type GLubyte = u8;",
// "pub type GLfloat = GLfloat;",
// "pub type GLclampf = GLfloat;",
// "pub type GLfixed = i32;",
// "pub type GLint64 = i64;",
// "pub type GLuint64 = u64;",
// "pub type GLint64EXT = i64;",
// "pub type GLuint64EXT = u64;",
// "pub type GLintptr = intptr_t;",
// "pub type GLsizeiptr = ssize_t;",

// GLES 2 types (none currently)

// Vendor extension types
pub type GLDEBUGPROCAMD = Option<extern "system" fn(id: GLuint,
                                                    category: GLenum,
                                                    severity: GLenum,
                                                    length: GLsizei,
                                                    message: *const GLchar,
                                                    userParam: *mut std::os::raw::c_void)>;
pub type GLhalfNV = std::os::raw::c_ushort;
pub type GLvdpauSurfaceNV = GLintptr;

pub const ACTIVE_ATOMIC_COUNTER_BUFFERS: GLenum = 0x92D9;
pub const ACTIVE_ATTRIBUTES: GLenum = 0x8B89;
pub const ACTIVE_ATTRIBUTE_MAX_LENGTH: GLenum = 0x8B8A;
pub const ACTIVE_PROGRAM: GLenum = 0x8259;
pub const ACTIVE_RESOURCES: GLenum = 0x92F5;
pub const ACTIVE_SUBROUTINES: GLenum = 0x8DE5;
pub const ACTIVE_SUBROUTINE_MAX_LENGTH: GLenum = 0x8E48;
pub const ACTIVE_SUBROUTINE_UNIFORMS: GLenum = 0x8DE6;
pub const ACTIVE_SUBROUTINE_UNIFORM_LOCATIONS: GLenum = 0x8E47;
pub const ACTIVE_SUBROUTINE_UNIFORM_MAX_LENGTH: GLenum = 0x8E49;
pub const ACTIVE_TEXTURE: GLenum = 0x84E0;
pub const ACTIVE_UNIFORMS: GLenum = 0x8B86;
pub const ACTIVE_UNIFORM_BLOCKS: GLenum = 0x8A36;
pub const ACTIVE_UNIFORM_BLOCK_MAX_NAME_LENGTH: GLenum = 0x8A35;
pub const ACTIVE_UNIFORM_MAX_LENGTH: GLenum = 0x8B87;
pub const ACTIVE_VARIABLES: GLenum = 0x9305;
pub const ALIASED_LINE_WIDTH_RANGE: GLenum = 0x846E;
pub const ALL_BARRIER_BITS: GLenum = 0xFFFFFFFF;
pub const ALL_SHADER_BITS: GLenum = 0xFFFFFFFF;
pub const ALPHA: GLenum = 0x1906;
pub const ALREADY_SIGNALED: GLenum = 0x911A;
pub const ALWAYS: GLenum = 0x0207;
pub const AND: GLenum = 0x1501;
pub const AND_INVERTED: GLenum = 0x1504;
pub const AND_REVERSE: GLenum = 0x1502;
pub const ANY_SAMPLES_PASSED: GLenum = 0x8C2F;
pub const ANY_SAMPLES_PASSED_CONSERVATIVE: GLenum = 0x8D6A;
pub const ARRAY_BUFFER: GLenum = 0x8892;
pub const ARRAY_BUFFER_BINDING: GLenum = 0x8894;
pub const ARRAY_SIZE: GLenum = 0x92FB;
pub const ARRAY_STRIDE: GLenum = 0x92FE;
pub const ATOMIC_COUNTER_BARRIER_BIT: GLenum = 0x00001000;
pub const ATOMIC_COUNTER_BUFFER: GLenum = 0x92C0;
pub const ATOMIC_COUNTER_BUFFER_ACTIVE_ATOMIC_COUNTERS: GLenum = 0x92C5;
pub const ATOMIC_COUNTER_BUFFER_ACTIVE_ATOMIC_COUNTER_INDICES: GLenum = 0x92C6;
pub const ATOMIC_COUNTER_BUFFER_BINDING: GLenum = 0x92C1;
pub const ATOMIC_COUNTER_BUFFER_DATA_SIZE: GLenum = 0x92C4;
pub const ATOMIC_COUNTER_BUFFER_INDEX: GLenum = 0x9301;
pub const ATOMIC_COUNTER_BUFFER_REFERENCED_BY_COMPUTE_SHADER: GLenum = 0x90ED;
pub const ATOMIC_COUNTER_BUFFER_REFERENCED_BY_FRAGMENT_SHADER: GLenum = 0x92CB;
pub const ATOMIC_COUNTER_BUFFER_REFERENCED_BY_GEOMETRY_SHADER: GLenum = 0x92CA;
pub const ATOMIC_COUNTER_BUFFER_REFERENCED_BY_TESS_CONTROL_SHADER: GLenum = 0x92C8;
pub const ATOMIC_COUNTER_BUFFER_REFERENCED_BY_TESS_EVALUATION_SHADER: GLenum = 0x92C9;
pub const ATOMIC_COUNTER_BUFFER_REFERENCED_BY_VERTEX_SHADER: GLenum = 0x92C7;
pub const ATOMIC_COUNTER_BUFFER_SIZE: GLenum = 0x92C3;
pub const ATOMIC_COUNTER_BUFFER_START: GLenum = 0x92C2;
pub const ATTACHED_SHADERS: GLenum = 0x8B85;
pub const AUTO_GENERATE_MIPMAP: GLenum = 0x8295;
pub const BACK: GLenum = 0x0405;
pub const BACK_LEFT: GLenum = 0x0402;
pub const BACK_RIGHT: GLenum = 0x0403;
pub const BGR: GLenum = 0x80E0;
pub const BGRA: GLenum = 0x80E1;
pub const BGRA_INTEGER: GLenum = 0x8D9B;
pub const BGR_INTEGER: GLenum = 0x8D9A;
pub const BLEND: GLenum = 0x0BE2;
pub const BLEND_COLOR: GLenum = 0x8005;
pub const BLEND_DST: GLenum = 0x0BE0;
pub const BLEND_DST_ALPHA: GLenum = 0x80CA;
pub const BLEND_DST_RGB: GLenum = 0x80C8;
pub const BLEND_EQUATION: GLenum = 0x8009;
pub const BLEND_EQUATION_ALPHA: GLenum = 0x883D;
pub const BLEND_EQUATION_RGB: GLenum = 0x8009;
pub const BLEND_SRC: GLenum = 0x0BE1;
pub const BLEND_SRC_ALPHA: GLenum = 0x80CB;
pub const BLEND_SRC_RGB: GLenum = 0x80C9;
pub const BLOCK_INDEX: GLenum = 0x92FD;
pub const BLUE: GLenum = 0x1905;
pub const BLUE_INTEGER: GLenum = 0x8D96;
pub const BOOL: GLenum = 0x8B56;
pub const BOOL_VEC2: GLenum = 0x8B57;
pub const BOOL_VEC3: GLenum = 0x8B58;
pub const BOOL_VEC4: GLenum = 0x8B59;
pub const BUFFER: GLenum = 0x82E0;
pub const BUFFER_ACCESS: GLenum = 0x88BB;
pub const BUFFER_ACCESS_FLAGS: GLenum = 0x911F;
pub const BUFFER_BINDING: GLenum = 0x9302;
pub const BUFFER_DATA_SIZE: GLenum = 0x9303;
pub const BUFFER_IMMUTABLE_STORAGE: GLenum = 0x821F;
pub const BUFFER_MAPPED: GLenum = 0x88BC;
pub const BUFFER_MAP_LENGTH: GLenum = 0x9120;
pub const BUFFER_MAP_OFFSET: GLenum = 0x9121;
pub const BUFFER_MAP_POINTER: GLenum = 0x88BD;
pub const BUFFER_SIZE: GLenum = 0x8764;
pub const BUFFER_STORAGE_FLAGS: GLenum = 0x8220;
pub const BUFFER_UPDATE_BARRIER_BIT: GLenum = 0x00000200;
pub const BUFFER_USAGE: GLenum = 0x8765;
pub const BUFFER_VARIABLE: GLenum = 0x92E5;
pub const BYTE: GLenum = 0x1400;
pub const CAVEAT_SUPPORT: GLenum = 0x82B8;
pub const CCW: GLenum = 0x0901;
pub const CLAMP_READ_COLOR: GLenum = 0x891C;
pub const CLAMP_TO_BORDER: GLenum = 0x812D;
pub const CLAMP_TO_EDGE: GLenum = 0x812F;
pub const CLEAR: GLenum = 0x1500;
pub const CLEAR_BUFFER: GLenum = 0x82B4;
pub const CLEAR_TEXTURE: GLenum = 0x9365;
pub const CLIENT_MAPPED_BUFFER_BARRIER_BIT: GLenum = 0x00004000;
pub const CLIENT_STORAGE_BIT: GLenum = 0x0200;
pub const CLIPPING_INPUT_PRIMITIVES: GLenum = 0x82F6;
pub const CLIPPING_OUTPUT_PRIMITIVES: GLenum = 0x82F7;
pub const CLIP_DEPTH_MODE: GLenum = 0x935D;
pub const CLIP_DISTANCE0: GLenum = 0x3000;
pub const CLIP_DISTANCE1: GLenum = 0x3001;
pub const CLIP_DISTANCE2: GLenum = 0x3002;
pub const CLIP_DISTANCE3: GLenum = 0x3003;
pub const CLIP_DISTANCE4: GLenum = 0x3004;
pub const CLIP_DISTANCE5: GLenum = 0x3005;
pub const CLIP_DISTANCE6: GLenum = 0x3006;
pub const CLIP_DISTANCE7: GLenum = 0x3007;
pub const CLIP_ORIGIN: GLenum = 0x935C;
pub const COLOR: GLenum = 0x1800;
pub const COLOR_ATTACHMENT0: GLenum = 0x8CE0;
pub const COLOR_ATTACHMENT1: GLenum = 0x8CE1;
pub const COLOR_ATTACHMENT10: GLenum = 0x8CEA;
pub const COLOR_ATTACHMENT11: GLenum = 0x8CEB;
pub const COLOR_ATTACHMENT12: GLenum = 0x8CEC;
pub const COLOR_ATTACHMENT13: GLenum = 0x8CED;
pub const COLOR_ATTACHMENT14: GLenum = 0x8CEE;
pub const COLOR_ATTACHMENT15: GLenum = 0x8CEF;
pub const COLOR_ATTACHMENT16: GLenum = 0x8CF0;
pub const COLOR_ATTACHMENT17: GLenum = 0x8CF1;
pub const COLOR_ATTACHMENT18: GLenum = 0x8CF2;
pub const COLOR_ATTACHMENT19: GLenum = 0x8CF3;
pub const COLOR_ATTACHMENT2: GLenum = 0x8CE2;
pub const COLOR_ATTACHMENT20: GLenum = 0x8CF4;
pub const COLOR_ATTACHMENT21: GLenum = 0x8CF5;
pub const COLOR_ATTACHMENT22: GLenum = 0x8CF6;
pub const COLOR_ATTACHMENT23: GLenum = 0x8CF7;
pub const COLOR_ATTACHMENT24: GLenum = 0x8CF8;
pub const COLOR_ATTACHMENT25: GLenum = 0x8CF9;
pub const COLOR_ATTACHMENT26: GLenum = 0x8CFA;
pub const COLOR_ATTACHMENT27: GLenum = 0x8CFB;
pub const COLOR_ATTACHMENT28: GLenum = 0x8CFC;
pub const COLOR_ATTACHMENT29: GLenum = 0x8CFD;
pub const COLOR_ATTACHMENT3: GLenum = 0x8CE3;
pub const COLOR_ATTACHMENT30: GLenum = 0x8CFE;
pub const COLOR_ATTACHMENT31: GLenum = 0x8CFF;
pub const COLOR_ATTACHMENT4: GLenum = 0x8CE4;
pub const COLOR_ATTACHMENT5: GLenum = 0x8CE5;
pub const COLOR_ATTACHMENT6: GLenum = 0x8CE6;
pub const COLOR_ATTACHMENT7: GLenum = 0x8CE7;
pub const COLOR_ATTACHMENT8: GLenum = 0x8CE8;
pub const COLOR_ATTACHMENT9: GLenum = 0x8CE9;
pub const COLOR_BUFFER_BIT: GLenum = 0x00004000;
pub const COLOR_CLEAR_VALUE: GLenum = 0x0C22;
pub const COLOR_COMPONENTS: GLenum = 0x8283;
pub const COLOR_ENCODING: GLenum = 0x8296;
pub const COLOR_LOGIC_OP: GLenum = 0x0BF2;
pub const COLOR_RENDERABLE: GLenum = 0x8286;
pub const COLOR_WRITEMASK: GLenum = 0x0C23;
pub const COMMAND_BARRIER_BIT: GLenum = 0x00000040;
pub const COMPARE_REF_TO_TEXTURE: GLenum = 0x884E;
pub const COMPATIBLE_SUBROUTINES: GLenum = 0x8E4B;
pub const COMPILE_STATUS: GLenum = 0x8B81;
pub const COMPRESSED_R11_EAC: GLenum = 0x9270;
pub const COMPRESSED_RED: GLenum = 0x8225;
pub const COMPRESSED_RED_RGTC1: GLenum = 0x8DBB;
pub const COMPRESSED_RG: GLenum = 0x8226;
pub const COMPRESSED_RG11_EAC: GLenum = 0x9272;
pub const COMPRESSED_RGB: GLenum = 0x84ED;
pub const COMPRESSED_RGB8_ETC2: GLenum = 0x9274;
pub const COMPRESSED_RGB8_PUNCHTHROUGH_ALPHA1_ETC2: GLenum = 0x9276;
pub const COMPRESSED_RGBA: GLenum = 0x84EE;
pub const COMPRESSED_RGBA8_ETC2_EAC: GLenum = 0x9278;
pub const COMPRESSED_RGBA_BPTC_UNORM: GLenum = 0x8E8C;
pub const COMPRESSED_RGB_BPTC_SIGNED_FLOAT: GLenum = 0x8E8E;
pub const COMPRESSED_RGB_BPTC_UNSIGNED_FLOAT: GLenum = 0x8E8F;
pub const COMPRESSED_RG_RGTC2: GLenum = 0x8DBD;
pub const COMPRESSED_SIGNED_R11_EAC: GLenum = 0x9271;
pub const COMPRESSED_SIGNED_RED_RGTC1: GLenum = 0x8DBC;
pub const COMPRESSED_SIGNED_RG11_EAC: GLenum = 0x9273;
pub const COMPRESSED_SIGNED_RG_RGTC2: GLenum = 0x8DBE;
pub const COMPRESSED_SRGB: GLenum = 0x8C48;
pub const COMPRESSED_SRGB8_ALPHA8_ETC2_EAC: GLenum = 0x9279;
pub const COMPRESSED_SRGB8_ETC2: GLenum = 0x9275;
pub const COMPRESSED_SRGB8_PUNCHTHROUGH_ALPHA1_ETC2: GLenum = 0x9277;
pub const COMPRESSED_SRGB_ALPHA: GLenum = 0x8C49;
pub const COMPRESSED_SRGB_ALPHA_BPTC_UNORM: GLenum = 0x8E8D;
pub const COMPRESSED_TEXTURE_FORMATS: GLenum = 0x86A3;
pub const COMPUTE_SHADER: GLenum = 0x91B9;
pub const COMPUTE_SHADER_BIT: GLenum = 0x00000020;
pub const COMPUTE_SHADER_INVOCATIONS: GLenum = 0x82F5;
pub const COMPUTE_SUBROUTINE: GLenum = 0x92ED;
pub const COMPUTE_SUBROUTINE_UNIFORM: GLenum = 0x92F3;
pub const COMPUTE_TEXTURE: GLenum = 0x82A0;
pub const COMPUTE_WORK_GROUP_SIZE: GLenum = 0x8267;
pub const CONDITION_SATISFIED: GLenum = 0x911C;
pub const CONSTANT_ALPHA: GLenum = 0x8003;
pub const CONSTANT_COLOR: GLenum = 0x8001;
pub const CONTEXT_COMPATIBILITY_PROFILE_BIT: GLenum = 0x00000002;
pub const CONTEXT_CORE_PROFILE_BIT: GLenum = 0x00000001;
pub const CONTEXT_FLAGS: GLenum = 0x821E;
pub const CONTEXT_FLAG_DEBUG_BIT: GLenum = 0x00000002;
pub const CONTEXT_FLAG_FORWARD_COMPATIBLE_BIT: GLenum = 0x00000001;
pub const CONTEXT_FLAG_NO_ERROR_BIT: GLenum = 0x00000008;
pub const CONTEXT_FLAG_ROBUST_ACCESS_BIT: GLenum = 0x00000004;
pub const CONTEXT_LOST: GLenum = 0x0507;
pub const CONTEXT_PROFILE_MASK: GLenum = 0x9126;
pub const CONTEXT_RELEASE_BEHAVIOR: GLenum = 0x82FB;
pub const CONTEXT_RELEASE_BEHAVIOR_FLUSH: GLenum = 0x82FC;
pub const COPY: GLenum = 0x1503;
pub const COPY_INVERTED: GLenum = 0x150C;
pub const COPY_READ_BUFFER: GLenum = 0x8F36;
pub const COPY_READ_BUFFER_BINDING: GLenum = 0x8F36;
pub const COPY_WRITE_BUFFER: GLenum = 0x8F37;
pub const COPY_WRITE_BUFFER_BINDING: GLenum = 0x8F37;
pub const CULL_FACE: GLenum = 0x0B44;
pub const CULL_FACE_MODE: GLenum = 0x0B45;
pub const CURRENT_PROGRAM: GLenum = 0x8B8D;
pub const CURRENT_QUERY: GLenum = 0x8865;
pub const CURRENT_VERTEX_ATTRIB: GLenum = 0x8626;
pub const CW: GLenum = 0x0900;
pub const DEBUG_CALLBACK_FUNCTION: GLenum = 0x8244;
pub const DEBUG_CALLBACK_USER_PARAM: GLenum = 0x8245;
pub const DEBUG_GROUP_STACK_DEPTH: GLenum = 0x826D;
pub const DEBUG_LOGGED_MESSAGES: GLenum = 0x9145;
pub const DEBUG_NEXT_LOGGED_MESSAGE_LENGTH: GLenum = 0x8243;
pub const DEBUG_OUTPUT: GLenum = 0x92E0;
pub const DEBUG_OUTPUT_SYNCHRONOUS: GLenum = 0x8242;
pub const DEBUG_SEVERITY_HIGH: GLenum = 0x9146;
pub const DEBUG_SEVERITY_LOW: GLenum = 0x9148;
pub const DEBUG_SEVERITY_MEDIUM: GLenum = 0x9147;
pub const DEBUG_SEVERITY_NOTIFICATION: GLenum = 0x826B;
pub const DEBUG_SOURCE_API: GLenum = 0x8246;
pub const DEBUG_SOURCE_APPLICATION: GLenum = 0x824A;
pub const DEBUG_SOURCE_OTHER: GLenum = 0x824B;
pub const DEBUG_SOURCE_SHADER_COMPILER: GLenum = 0x8248;
pub const DEBUG_SOURCE_THIRD_PARTY: GLenum = 0x8249;
pub const DEBUG_SOURCE_WINDOW_SYSTEM: GLenum = 0x8247;
pub const DEBUG_TYPE_DEPRECATED_BEHAVIOR: GLenum = 0x824D;
pub const DEBUG_TYPE_ERROR: GLenum = 0x824C;
pub const DEBUG_TYPE_MARKER: GLenum = 0x8268;
pub const DEBUG_TYPE_OTHER: GLenum = 0x8251;
pub const DEBUG_TYPE_PERFORMANCE: GLenum = 0x8250;
pub const DEBUG_TYPE_POP_GROUP: GLenum = 0x826A;
pub const DEBUG_TYPE_PORTABILITY: GLenum = 0x824F;
pub const DEBUG_TYPE_PUSH_GROUP: GLenum = 0x8269;
pub const DEBUG_TYPE_UNDEFINED_BEHAVIOR: GLenum = 0x824E;
pub const DECR: GLenum = 0x1E03;
pub const DECR_WRAP: GLenum = 0x8508;
pub const DELETE_STATUS: GLenum = 0x8B80;
pub const DEPTH: GLenum = 0x1801;
pub const DEPTH24_STENCIL8: GLenum = 0x88F0;
pub const DEPTH32F_STENCIL8: GLenum = 0x8CAD;
pub const DEPTH_ATTACHMENT: GLenum = 0x8D00;
pub const DEPTH_BUFFER_BIT: GLenum = 0x00000100;
pub const DEPTH_CLAMP: GLenum = 0x864F;
pub const DEPTH_CLEAR_VALUE: GLenum = 0x0B73;
pub const DEPTH_COMPONENT: GLenum = 0x1902;
pub const DEPTH_COMPONENT16: GLenum = 0x81A5;
pub const DEPTH_COMPONENT24: GLenum = 0x81A6;
pub const DEPTH_COMPONENT32: GLenum = 0x81A7;
pub const DEPTH_COMPONENT32F: GLenum = 0x8CAC;
pub const DEPTH_COMPONENTS: GLenum = 0x8284;
pub const DEPTH_FUNC: GLenum = 0x0B74;
pub const DEPTH_RANGE: GLenum = 0x0B70;
pub const DEPTH_RENDERABLE: GLenum = 0x8287;
pub const DEPTH_STENCIL: GLenum = 0x84F9;
pub const DEPTH_STENCIL_ATTACHMENT: GLenum = 0x821A;
pub const DEPTH_STENCIL_TEXTURE_MODE: GLenum = 0x90EA;
pub const DEPTH_TEST: GLenum = 0x0B71;
pub const DEPTH_WRITEMASK: GLenum = 0x0B72;
pub const DISPATCH_INDIRECT_BUFFER: GLenum = 0x90EE;
pub const DISPATCH_INDIRECT_BUFFER_BINDING: GLenum = 0x90EF;
pub const DISPLAY_LIST: GLenum = 0x82E7;
pub const DITHER: GLenum = 0x0BD0;
pub const DONT_CARE: GLenum = 0x1100;
pub const DOUBLE: GLenum = 0x140A;
pub const DOUBLEBUFFER: GLenum = 0x0C32;
pub const DOUBLE_MAT2: GLenum = 0x8F46;
pub const DOUBLE_MAT2x3: GLenum = 0x8F49;
pub const DOUBLE_MAT2x4: GLenum = 0x8F4A;
pub const DOUBLE_MAT3: GLenum = 0x8F47;
pub const DOUBLE_MAT3x2: GLenum = 0x8F4B;
pub const DOUBLE_MAT3x4: GLenum = 0x8F4C;
pub const DOUBLE_MAT4: GLenum = 0x8F48;
pub const DOUBLE_MAT4x2: GLenum = 0x8F4D;
pub const DOUBLE_MAT4x3: GLenum = 0x8F4E;
pub const DOUBLE_VEC2: GLenum = 0x8FFC;
pub const DOUBLE_VEC3: GLenum = 0x8FFD;
pub const DOUBLE_VEC4: GLenum = 0x8FFE;
pub const DRAW_BUFFER: GLenum = 0x0C01;
pub const DRAW_BUFFER0: GLenum = 0x8825;
pub const DRAW_BUFFER1: GLenum = 0x8826;
pub const DRAW_BUFFER10: GLenum = 0x882F;
pub const DRAW_BUFFER11: GLenum = 0x8830;
pub const DRAW_BUFFER12: GLenum = 0x8831;
pub const DRAW_BUFFER13: GLenum = 0x8832;
pub const DRAW_BUFFER14: GLenum = 0x8833;
pub const DRAW_BUFFER15: GLenum = 0x8834;
pub const DRAW_BUFFER2: GLenum = 0x8827;
pub const DRAW_BUFFER3: GLenum = 0x8828;
pub const DRAW_BUFFER4: GLenum = 0x8829;
pub const DRAW_BUFFER5: GLenum = 0x882A;
pub const DRAW_BUFFER6: GLenum = 0x882B;
pub const DRAW_BUFFER7: GLenum = 0x882C;
pub const DRAW_BUFFER8: GLenum = 0x882D;
pub const DRAW_BUFFER9: GLenum = 0x882E;
pub const DRAW_FRAMEBUFFER: GLenum = 0x8CA9;
pub const DRAW_FRAMEBUFFER_BINDING: GLenum = 0x8CA6;
pub const DRAW_INDIRECT_BUFFER: GLenum = 0x8F3F;
pub const DRAW_INDIRECT_BUFFER_BINDING: GLenum = 0x8F43;
pub const DST_ALPHA: GLenum = 0x0304;
pub const DST_COLOR: GLenum = 0x0306;
pub const DYNAMIC_COPY: GLenum = 0x88EA;
pub const DYNAMIC_DRAW: GLenum = 0x88E8;
pub const DYNAMIC_READ: GLenum = 0x88E9;
pub const DYNAMIC_STORAGE_BIT: GLenum = 0x0100;
pub const ELEMENT_ARRAY_BARRIER_BIT: GLenum = 0x00000002;
pub const ELEMENT_ARRAY_BUFFER: GLenum = 0x8893;
pub const ELEMENT_ARRAY_BUFFER_BINDING: GLenum = 0x8895;
pub const EQUAL: GLenum = 0x0202;
pub const EQUIV: GLenum = 0x1509;
pub const EXTENSIONS: GLenum = 0x1F03;
pub const FALSE: GLboolean = 0;
pub const FASTEST: GLenum = 0x1101;
pub const FILL: GLenum = 0x1B02;
pub const FILTER: GLenum = 0x829A;
pub const FIRST_VERTEX_CONVENTION: GLenum = 0x8E4D;
pub const FIXED: GLenum = 0x140C;
pub const FIXED_ONLY: GLenum = 0x891D;
pub const FLOAT: GLenum = 0x1406;
pub const FLOAT_32_UNSIGNED_INT_24_8_REV: GLenum = 0x8DAD;
pub const FLOAT_MAT2: GLenum = 0x8B5A;
pub const FLOAT_MAT2x3: GLenum = 0x8B65;
pub const FLOAT_MAT2x4: GLenum = 0x8B66;
pub const FLOAT_MAT3: GLenum = 0x8B5B;
pub const FLOAT_MAT3x2: GLenum = 0x8B67;
pub const FLOAT_MAT3x4: GLenum = 0x8B68;
pub const FLOAT_MAT4: GLenum = 0x8B5C;
pub const FLOAT_MAT4x2: GLenum = 0x8B69;
pub const FLOAT_MAT4x3: GLenum = 0x8B6A;
pub const FLOAT_VEC2: GLenum = 0x8B50;
pub const FLOAT_VEC3: GLenum = 0x8B51;
pub const FLOAT_VEC4: GLenum = 0x8B52;
pub const FRACTIONAL_EVEN: GLenum = 0x8E7C;
pub const FRACTIONAL_ODD: GLenum = 0x8E7B;
pub const FRAGMENT_INTERPOLATION_OFFSET_BITS: GLenum = 0x8E5D;
pub const FRAGMENT_SHADER: GLenum = 0x8B30;
pub const FRAGMENT_SHADER_BIT: GLenum = 0x00000002;
pub const FRAGMENT_SHADER_DERIVATIVE_HINT: GLenum = 0x8B8B;
pub const FRAGMENT_SHADER_INVOCATIONS: GLenum = 0x82F4;
pub const FRAGMENT_SUBROUTINE: GLenum = 0x92EC;
pub const FRAGMENT_SUBROUTINE_UNIFORM: GLenum = 0x92F2;
pub const FRAGMENT_TEXTURE: GLenum = 0x829F;
pub const FRAMEBUFFER: GLenum = 0x8D40;
pub const FRAMEBUFFER_ATTACHMENT_ALPHA_SIZE: GLenum = 0x8215;
pub const FRAMEBUFFER_ATTACHMENT_BLUE_SIZE: GLenum = 0x8214;
pub const FRAMEBUFFER_ATTACHMENT_COLOR_ENCODING: GLenum = 0x8210;
pub const FRAMEBUFFER_ATTACHMENT_COMPONENT_TYPE: GLenum = 0x8211;
pub const FRAMEBUFFER_ATTACHMENT_DEPTH_SIZE: GLenum = 0x8216;
pub const FRAMEBUFFER_ATTACHMENT_GREEN_SIZE: GLenum = 0x8213;
pub const FRAMEBUFFER_ATTACHMENT_LAYERED: GLenum = 0x8DA7;
pub const FRAMEBUFFER_ATTACHMENT_OBJECT_NAME: GLenum = 0x8CD1;
pub const FRAMEBUFFER_ATTACHMENT_OBJECT_TYPE: GLenum = 0x8CD0;
pub const FRAMEBUFFER_ATTACHMENT_RED_SIZE: GLenum = 0x8212;
pub const FRAMEBUFFER_ATTACHMENT_STENCIL_SIZE: GLenum = 0x8217;
pub const FRAMEBUFFER_ATTACHMENT_TEXTURE_CUBE_MAP_FACE: GLenum = 0x8CD3;
pub const FRAMEBUFFER_ATTACHMENT_TEXTURE_LAYER: GLenum = 0x8CD4;
pub const FRAMEBUFFER_ATTACHMENT_TEXTURE_LEVEL: GLenum = 0x8CD2;
pub const FRAMEBUFFER_BARRIER_BIT: GLenum = 0x00000400;
pub const FRAMEBUFFER_BINDING: GLenum = 0x8CA6;
pub const FRAMEBUFFER_BLEND: GLenum = 0x828B;
pub const FRAMEBUFFER_COMPLETE: GLenum = 0x8CD5;
pub const FRAMEBUFFER_DEFAULT: GLenum = 0x8218;
pub const FRAMEBUFFER_DEFAULT_FIXED_SAMPLE_LOCATIONS: GLenum = 0x9314;
pub const FRAMEBUFFER_DEFAULT_HEIGHT: GLenum = 0x9311;
pub const FRAMEBUFFER_DEFAULT_LAYERS: GLenum = 0x9312;
pub const FRAMEBUFFER_DEFAULT_SAMPLES: GLenum = 0x9313;
pub const FRAMEBUFFER_DEFAULT_WIDTH: GLenum = 0x9310;
pub const FRAMEBUFFER_INCOMPLETE_ATTACHMENT: GLenum = 0x8CD6;
pub const FRAMEBUFFER_INCOMPLETE_DRAW_BUFFER: GLenum = 0x8CDB;
pub const FRAMEBUFFER_INCOMPLETE_LAYER_TARGETS: GLenum = 0x8DA8;
pub const FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT: GLenum = 0x8CD7;
pub const FRAMEBUFFER_INCOMPLETE_MULTISAMPLE: GLenum = 0x8D56;
pub const FRAMEBUFFER_INCOMPLETE_READ_BUFFER: GLenum = 0x8CDC;
pub const FRAMEBUFFER_RENDERABLE: GLenum = 0x8289;
pub const FRAMEBUFFER_RENDERABLE_LAYERED: GLenum = 0x828A;
pub const FRAMEBUFFER_SRGB: GLenum = 0x8DB9;
pub const FRAMEBUFFER_UNDEFINED: GLenum = 0x8219;
pub const FRAMEBUFFER_UNSUPPORTED: GLenum = 0x8CDD;
pub const FRONT: GLenum = 0x0404;
pub const FRONT_AND_BACK: GLenum = 0x0408;
pub const FRONT_FACE: GLenum = 0x0B46;
pub const FRONT_LEFT: GLenum = 0x0400;
pub const FRONT_RIGHT: GLenum = 0x0401;
pub const FULL_SUPPORT: GLenum = 0x82B7;
pub const FUNC_ADD: GLenum = 0x8006;
pub const FUNC_REVERSE_SUBTRACT: GLenum = 0x800B;
pub const FUNC_SUBTRACT: GLenum = 0x800A;
pub const GEOMETRY_INPUT_TYPE: GLenum = 0x8917;
pub const GEOMETRY_OUTPUT_TYPE: GLenum = 0x8918;
pub const GEOMETRY_SHADER: GLenum = 0x8DD9;
pub const GEOMETRY_SHADER_BIT: GLenum = 0x00000004;
pub const GEOMETRY_SHADER_INVOCATIONS: GLenum = 0x887F;
pub const GEOMETRY_SHADER_PRIMITIVES_EMITTED: GLenum = 0x82F3;
pub const GEOMETRY_SUBROUTINE: GLenum = 0x92EB;
pub const GEOMETRY_SUBROUTINE_UNIFORM: GLenum = 0x92F1;
pub const GEOMETRY_TEXTURE: GLenum = 0x829E;
pub const GEOMETRY_VERTICES_OUT: GLenum = 0x8916;
pub const GEQUAL: GLenum = 0x0206;
pub const GET_TEXTURE_IMAGE_FORMAT: GLenum = 0x8291;
pub const GET_TEXTURE_IMAGE_TYPE: GLenum = 0x8292;
pub const GREATER: GLenum = 0x0204;
pub const GREEN: GLenum = 0x1904;
pub const GREEN_INTEGER: GLenum = 0x8D95;
pub const GUILTY_CONTEXT_RESET: GLenum = 0x8253;
pub const HALF_FLOAT: GLenum = 0x140B;
pub const HIGH_FLOAT: GLenum = 0x8DF2;
pub const HIGH_INT: GLenum = 0x8DF5;
pub const IMAGE_1D: GLenum = 0x904C;
pub const IMAGE_1D_ARRAY: GLenum = 0x9052;
pub const IMAGE_2D: GLenum = 0x904D;
pub const IMAGE_2D_ARRAY: GLenum = 0x9053;
pub const IMAGE_2D_MULTISAMPLE: GLenum = 0x9055;
pub const IMAGE_2D_MULTISAMPLE_ARRAY: GLenum = 0x9056;
pub const IMAGE_2D_RECT: GLenum = 0x904F;
pub const IMAGE_3D: GLenum = 0x904E;
pub const IMAGE_BINDING_ACCESS: GLenum = 0x8F3E;
pub const IMAGE_BINDING_FORMAT: GLenum = 0x906E;
pub const IMAGE_BINDING_LAYER: GLenum = 0x8F3D;
pub const IMAGE_BINDING_LAYERED: GLenum = 0x8F3C;
pub const IMAGE_BINDING_LEVEL: GLenum = 0x8F3B;
pub const IMAGE_BINDING_NAME: GLenum = 0x8F3A;
pub const IMAGE_BUFFER: GLenum = 0x9051;
pub const IMAGE_CLASS_10_10_10_2: GLenum = 0x82C3;
pub const IMAGE_CLASS_11_11_10: GLenum = 0x82C2;
pub const IMAGE_CLASS_1_X_16: GLenum = 0x82BE;
pub const IMAGE_CLASS_1_X_32: GLenum = 0x82BB;
pub const IMAGE_CLASS_1_X_8: GLenum = 0x82C1;
pub const IMAGE_CLASS_2_X_16: GLenum = 0x82BD;
pub const IMAGE_CLASS_2_X_32: GLenum = 0x82BA;
pub const IMAGE_CLASS_2_X_8: GLenum = 0x82C0;
pub const IMAGE_CLASS_4_X_16: GLenum = 0x82BC;
pub const IMAGE_CLASS_4_X_32: GLenum = 0x82B9;
pub const IMAGE_CLASS_4_X_8: GLenum = 0x82BF;
pub const IMAGE_COMPATIBILITY_CLASS: GLenum = 0x82A8;
pub const IMAGE_CUBE: GLenum = 0x9050;
pub const IMAGE_CUBE_MAP_ARRAY: GLenum = 0x9054;
pub const IMAGE_FORMAT_COMPATIBILITY_BY_CLASS: GLenum = 0x90C9;
pub const IMAGE_FORMAT_COMPATIBILITY_BY_SIZE: GLenum = 0x90C8;
pub const IMAGE_FORMAT_COMPATIBILITY_TYPE: GLenum = 0x90C7;
pub const IMAGE_PIXEL_FORMAT: GLenum = 0x82A9;
pub const IMAGE_PIXEL_TYPE: GLenum = 0x82AA;
pub const IMAGE_TEXEL_SIZE: GLenum = 0x82A7;
pub const IMPLEMENTATION_COLOR_READ_FORMAT: GLenum = 0x8B9B;
pub const IMPLEMENTATION_COLOR_READ_TYPE: GLenum = 0x8B9A;
pub const INCR: GLenum = 0x1E02;
pub const INCR_WRAP: GLenum = 0x8507;
pub const INFO_LOG_LENGTH: GLenum = 0x8B84;
pub const INNOCENT_CONTEXT_RESET: GLenum = 0x8254;
pub const INT: GLenum = 0x1404;
pub const INTERLEAVED_ATTRIBS: GLenum = 0x8C8C;
pub const INTERNALFORMAT_ALPHA_SIZE: GLenum = 0x8274;
pub const INTERNALFORMAT_ALPHA_TYPE: GLenum = 0x827B;
pub const INTERNALFORMAT_BLUE_SIZE: GLenum = 0x8273;
pub const INTERNALFORMAT_BLUE_TYPE: GLenum = 0x827A;
pub const INTERNALFORMAT_DEPTH_SIZE: GLenum = 0x8275;
pub const INTERNALFORMAT_DEPTH_TYPE: GLenum = 0x827C;
pub const INTERNALFORMAT_GREEN_SIZE: GLenum = 0x8272;
pub const INTERNALFORMAT_GREEN_TYPE: GLenum = 0x8279;
pub const INTERNALFORMAT_PREFERRED: GLenum = 0x8270;
pub const INTERNALFORMAT_RED_SIZE: GLenum = 0x8271;
pub const INTERNALFORMAT_RED_TYPE: GLenum = 0x8278;
pub const INTERNALFORMAT_SHARED_SIZE: GLenum = 0x8277;
pub const INTERNALFORMAT_STENCIL_SIZE: GLenum = 0x8276;
pub const INTERNALFORMAT_STENCIL_TYPE: GLenum = 0x827D;
pub const INTERNALFORMAT_SUPPORTED: GLenum = 0x826F;
pub const INT_2_10_10_10_REV: GLenum = 0x8D9F;
pub const INT_IMAGE_1D: GLenum = 0x9057;
pub const INT_IMAGE_1D_ARRAY: GLenum = 0x905D;
pub const INT_IMAGE_2D: GLenum = 0x9058;
pub const INT_IMAGE_2D_ARRAY: GLenum = 0x905E;
pub const INT_IMAGE_2D_MULTISAMPLE: GLenum = 0x9060;
pub const INT_IMAGE_2D_MULTISAMPLE_ARRAY: GLenum = 0x9061;
pub const INT_IMAGE_2D_RECT: GLenum = 0x905A;
pub const INT_IMAGE_3D: GLenum = 0x9059;
pub const INT_IMAGE_BUFFER: GLenum = 0x905C;
pub const INT_IMAGE_CUBE: GLenum = 0x905B;
pub const INT_IMAGE_CUBE_MAP_ARRAY: GLenum = 0x905F;
pub const INT_SAMPLER_1D: GLenum = 0x8DC9;
pub const INT_SAMPLER_1D_ARRAY: GLenum = 0x8DCE;
pub const INT_SAMPLER_2D: GLenum = 0x8DCA;
pub const INT_SAMPLER_2D_ARRAY: GLenum = 0x8DCF;
pub const INT_SAMPLER_2D_MULTISAMPLE: GLenum = 0x9109;
pub const INT_SAMPLER_2D_MULTISAMPLE_ARRAY: GLenum = 0x910C;
pub const INT_SAMPLER_2D_RECT: GLenum = 0x8DCD;
pub const INT_SAMPLER_3D: GLenum = 0x8DCB;
pub const INT_SAMPLER_BUFFER: GLenum = 0x8DD0;
pub const INT_SAMPLER_CUBE: GLenum = 0x8DCC;
pub const INT_SAMPLER_CUBE_MAP_ARRAY: GLenum = 0x900E;
pub const INT_VEC2: GLenum = 0x8B53;
pub const INT_VEC3: GLenum = 0x8B54;
pub const INT_VEC4: GLenum = 0x8B55;
pub const INVALID_ENUM: GLenum = 0x0500;
pub const INVALID_FRAMEBUFFER_OPERATION: GLenum = 0x0506;
pub const INVALID_INDEX: GLuint = 0xFFFFFFFF;
pub const INVALID_OPERATION: GLenum = 0x0502;
pub const INVALID_VALUE: GLenum = 0x0501;
pub const INVERT: GLenum = 0x150A;
pub const ISOLINES: GLenum = 0x8E7A;
pub const IS_PER_PATCH: GLenum = 0x92E7;
pub const IS_ROW_MAJOR: GLenum = 0x9300;
pub const KEEP: GLenum = 0x1E00;
pub const LAST_VERTEX_CONVENTION: GLenum = 0x8E4E;
pub const LAYER_PROVOKING_VERTEX: GLenum = 0x825E;
pub const LEFT: GLenum = 0x0406;
pub const LEQUAL: GLenum = 0x0203;
pub const LESS: GLenum = 0x0201;
pub const LINE: GLenum = 0x1B01;
pub const LINEAR: GLenum = 0x2601;
pub const LINEAR_MIPMAP_LINEAR: GLenum = 0x2703;
pub const LINEAR_MIPMAP_NEAREST: GLenum = 0x2701;
pub const LINES: GLenum = 0x0001;
pub const LINES_ADJACENCY: GLenum = 0x000A;
pub const LINE_LOOP: GLenum = 0x0002;
pub const LINE_SMOOTH: GLenum = 0x0B20;
pub const LINE_SMOOTH_HINT: GLenum = 0x0C52;
pub const LINE_STRIP: GLenum = 0x0003;
pub const LINE_STRIP_ADJACENCY: GLenum = 0x000B;
pub const LINE_WIDTH: GLenum = 0x0B21;
pub const LINE_WIDTH_GRANULARITY: GLenum = 0x0B23;
pub const LINE_WIDTH_RANGE: GLenum = 0x0B22;
pub const LINK_STATUS: GLenum = 0x8B82;
pub const LOCATION: GLenum = 0x930E;
pub const LOCATION_COMPONENT: GLenum = 0x934A;
pub const LOCATION_INDEX: GLenum = 0x930F;
pub const LOGIC_OP_MODE: GLenum = 0x0BF0;
pub const LOSE_CONTEXT_ON_RESET: GLenum = 0x8252;
pub const LOWER_LEFT: GLenum = 0x8CA1;
pub const LOW_FLOAT: GLenum = 0x8DF0;
pub const LOW_INT: GLenum = 0x8DF3;
pub const MAJOR_VERSION: GLenum = 0x821B;
pub const MANUAL_GENERATE_MIPMAP: GLenum = 0x8294;
pub const MAP_COHERENT_BIT: GLenum = 0x0080;
pub const MAP_FLUSH_EXPLICIT_BIT: GLenum = 0x0010;
pub const MAP_INVALIDATE_BUFFER_BIT: GLenum = 0x0008;
pub const MAP_INVALIDATE_RANGE_BIT: GLenum = 0x0004;
pub const MAP_PERSISTENT_BIT: GLenum = 0x0040;
pub const MAP_READ_BIT: GLenum = 0x0001;
pub const MAP_UNSYNCHRONIZED_BIT: GLenum = 0x0020;
pub const MAP_WRITE_BIT: GLenum = 0x0002;
pub const MATRIX_STRIDE: GLenum = 0x92FF;
pub const MAX: GLenum = 0x8008;
pub const MAX_3D_TEXTURE_SIZE: GLenum = 0x8073;
pub const MAX_ARRAY_TEXTURE_LAYERS: GLenum = 0x88FF;
pub const MAX_ATOMIC_COUNTER_BUFFER_BINDINGS: GLenum = 0x92DC;
pub const MAX_ATOMIC_COUNTER_BUFFER_SIZE: GLenum = 0x92D8;
pub const MAX_CLIP_DISTANCES: GLenum = 0x0D32;
pub const MAX_COLOR_ATTACHMENTS: GLenum = 0x8CDF;
pub const MAX_COLOR_TEXTURE_SAMPLES: GLenum = 0x910E;
pub const MAX_COMBINED_ATOMIC_COUNTERS: GLenum = 0x92D7;
pub const MAX_COMBINED_ATOMIC_COUNTER_BUFFERS: GLenum = 0x92D1;
pub const MAX_COMBINED_CLIP_AND_CULL_DISTANCES: GLenum = 0x82FA;
pub const MAX_COMBINED_COMPUTE_UNIFORM_COMPONENTS: GLenum = 0x8266;
pub const MAX_COMBINED_DIMENSIONS: GLenum = 0x8282;
pub const MAX_COMBINED_FRAGMENT_UNIFORM_COMPONENTS: GLenum = 0x8A33;
pub const MAX_COMBINED_GEOMETRY_UNIFORM_COMPONENTS: GLenum = 0x8A32;
pub const MAX_COMBINED_IMAGE_UNIFORMS: GLenum = 0x90CF;
pub const MAX_COMBINED_IMAGE_UNITS_AND_FRAGMENT_OUTPUTS: GLenum = 0x8F39;
pub const MAX_COMBINED_SHADER_OUTPUT_RESOURCES: GLenum = 0x8F39;
pub const MAX_COMBINED_SHADER_STORAGE_BLOCKS: GLenum = 0x90DC;
pub const MAX_COMBINED_TESS_CONTROL_UNIFORM_COMPONENTS: GLenum = 0x8E1E;
pub const MAX_COMBINED_TESS_EVALUATION_UNIFORM_COMPONENTS: GLenum = 0x8E1F;
pub const MAX_COMBINED_TEXTURE_IMAGE_UNITS: GLenum = 0x8B4D;
pub const MAX_COMBINED_UNIFORM_BLOCKS: GLenum = 0x8A2E;
pub const MAX_COMBINED_VERTEX_UNIFORM_COMPONENTS: GLenum = 0x8A31;
pub const MAX_COMPUTE_ATOMIC_COUNTERS: GLenum = 0x8265;
pub const MAX_COMPUTE_ATOMIC_COUNTER_BUFFERS: GLenum = 0x8264;
pub const MAX_COMPUTE_IMAGE_UNIFORMS: GLenum = 0x91BD;
pub const MAX_COMPUTE_SHADER_STORAGE_BLOCKS: GLenum = 0x90DB;
pub const MAX_COMPUTE_SHARED_MEMORY_SIZE: GLenum = 0x8262;
pub const MAX_COMPUTE_TEXTURE_IMAGE_UNITS: GLenum = 0x91BC;
pub const MAX_COMPUTE_UNIFORM_BLOCKS: GLenum = 0x91BB;
pub const MAX_COMPUTE_UNIFORM_COMPONENTS: GLenum = 0x8263;
pub const MAX_COMPUTE_WORK_GROUP_COUNT: GLenum = 0x91BE;
pub const MAX_COMPUTE_WORK_GROUP_INVOCATIONS: GLenum = 0x90EB;
pub const MAX_COMPUTE_WORK_GROUP_SIZE: GLenum = 0x91BF;
pub const MAX_CUBE_MAP_TEXTURE_SIZE: GLenum = 0x851C;
pub const MAX_CULL_DISTANCES: GLenum = 0x82F9;
pub const MAX_DEBUG_GROUP_STACK_DEPTH: GLenum = 0x826C;
pub const MAX_DEBUG_LOGGED_MESSAGES: GLenum = 0x9144;
pub const MAX_DEBUG_MESSAGE_LENGTH: GLenum = 0x9143;
pub const MAX_DEPTH: GLenum = 0x8280;
pub const MAX_DEPTH_TEXTURE_SAMPLES: GLenum = 0x910F;
pub const MAX_DRAW_BUFFERS: GLenum = 0x8824;
pub const MAX_DUAL_SOURCE_DRAW_BUFFERS: GLenum = 0x88FC;
pub const MAX_ELEMENTS_INDICES: GLenum = 0x80E9;
pub const MAX_ELEMENTS_VERTICES: GLenum = 0x80E8;
pub const MAX_ELEMENT_INDEX: GLenum = 0x8D6B;
pub const MAX_FRAGMENT_ATOMIC_COUNTERS: GLenum = 0x92D6;
pub const MAX_FRAGMENT_ATOMIC_COUNTER_BUFFERS: GLenum = 0x92D0;
pub const MAX_FRAGMENT_IMAGE_UNIFORMS: GLenum = 0x90CE;
pub const MAX_FRAGMENT_INPUT_COMPONENTS: GLenum = 0x9125;
pub const MAX_FRAGMENT_INTERPOLATION_OFFSET: GLenum = 0x8E5C;
pub const MAX_FRAGMENT_SHADER_STORAGE_BLOCKS: GLenum = 0x90DA;
pub const MAX_FRAGMENT_UNIFORM_BLOCKS: GLenum = 0x8A2D;
pub const MAX_FRAGMENT_UNIFORM_COMPONENTS: GLenum = 0x8B49;
pub const MAX_FRAGMENT_UNIFORM_VECTORS: GLenum = 0x8DFD;
pub const MAX_FRAMEBUFFER_HEIGHT: GLenum = 0x9316;
pub const MAX_FRAMEBUFFER_LAYERS: GLenum = 0x9317;
pub const MAX_FRAMEBUFFER_SAMPLES: GLenum = 0x9318;
pub const MAX_FRAMEBUFFER_WIDTH: GLenum = 0x9315;
pub const MAX_GEOMETRY_ATOMIC_COUNTERS: GLenum = 0x92D5;
pub const MAX_GEOMETRY_ATOMIC_COUNTER_BUFFERS: GLenum = 0x92CF;
pub const MAX_GEOMETRY_IMAGE_UNIFORMS: GLenum = 0x90CD;
pub const MAX_GEOMETRY_INPUT_COMPONENTS: GLenum = 0x9123;
pub const MAX_GEOMETRY_OUTPUT_COMPONENTS: GLenum = 0x9124;
pub const MAX_GEOMETRY_OUTPUT_VERTICES: GLenum = 0x8DE0;
pub const MAX_GEOMETRY_SHADER_INVOCATIONS: GLenum = 0x8E5A;
pub const MAX_GEOMETRY_SHADER_STORAGE_BLOCKS: GLenum = 0x90D7;
pub const MAX_GEOMETRY_TEXTURE_IMAGE_UNITS: GLenum = 0x8C29;
pub const MAX_GEOMETRY_TOTAL_OUTPUT_COMPONENTS: GLenum = 0x8DE1;
pub const MAX_GEOMETRY_UNIFORM_BLOCKS: GLenum = 0x8A2C;
pub const MAX_GEOMETRY_UNIFORM_COMPONENTS: GLenum = 0x8DDF;
pub const MAX_HEIGHT: GLenum = 0x827F;
pub const MAX_IMAGE_SAMPLES: GLenum = 0x906D;
pub const MAX_IMAGE_UNITS: GLenum = 0x8F38;
pub const MAX_INTEGER_SAMPLES: GLenum = 0x9110;
pub const MAX_LABEL_LENGTH: GLenum = 0x82E8;
pub const MAX_LAYERS: GLenum = 0x8281;
pub const MAX_NAME_LENGTH: GLenum = 0x92F6;
pub const MAX_NUM_ACTIVE_VARIABLES: GLenum = 0x92F7;
pub const MAX_NUM_COMPATIBLE_SUBROUTINES: GLenum = 0x92F8;
pub const MAX_PATCH_VERTICES: GLenum = 0x8E7D;
pub const MAX_PROGRAM_TEXEL_OFFSET: GLenum = 0x8905;
pub const MAX_PROGRAM_TEXTURE_GATHER_OFFSET: GLenum = 0x8E5F;
pub const MAX_RECTANGLE_TEXTURE_SIZE: GLenum = 0x84F8;
pub const MAX_RENDERBUFFER_SIZE: GLenum = 0x84E8;
pub const MAX_SAMPLES: GLenum = 0x8D57;
pub const MAX_SAMPLE_MASK_WORDS: GLenum = 0x8E59;
pub const MAX_SERVER_WAIT_TIMEOUT: GLenum = 0x9111;
pub const MAX_SHADER_STORAGE_BLOCK_SIZE: GLenum = 0x90DE;
pub const MAX_SHADER_STORAGE_BUFFER_BINDINGS: GLenum = 0x90DD;
pub const MAX_SUBROUTINES: GLenum = 0x8DE7;
pub const MAX_SUBROUTINE_UNIFORM_LOCATIONS: GLenum = 0x8DE8;
pub const MAX_TESS_CONTROL_ATOMIC_COUNTERS: GLenum = 0x92D3;
pub const MAX_TESS_CONTROL_ATOMIC_COUNTER_BUFFERS: GLenum = 0x92CD;
pub const MAX_TESS_CONTROL_IMAGE_UNIFORMS: GLenum = 0x90CB;
pub const MAX_TESS_CONTROL_INPUT_COMPONENTS: GLenum = 0x886C;
pub const MAX_TESS_CONTROL_OUTPUT_COMPONENTS: GLenum = 0x8E83;
pub const MAX_TESS_CONTROL_SHADER_STORAGE_BLOCKS: GLenum = 0x90D8;
pub const MAX_TESS_CONTROL_TEXTURE_IMAGE_UNITS: GLenum = 0x8E81;
pub const MAX_TESS_CONTROL_TOTAL_OUTPUT_COMPONENTS: GLenum = 0x8E85;
pub const MAX_TESS_CONTROL_UNIFORM_BLOCKS: GLenum = 0x8E89;
pub const MAX_TESS_CONTROL_UNIFORM_COMPONENTS: GLenum = 0x8E7F;
pub const MAX_TESS_EVALUATION_ATOMIC_COUNTERS: GLenum = 0x92D4;
pub const MAX_TESS_EVALUATION_ATOMIC_COUNTER_BUFFERS: GLenum = 0x92CE;
pub const MAX_TESS_EVALUATION_IMAGE_UNIFORMS: GLenum = 0x90CC;
pub const MAX_TESS_EVALUATION_INPUT_COMPONENTS: GLenum = 0x886D;
pub const MAX_TESS_EVALUATION_OUTPUT_COMPONENTS: GLenum = 0x8E86;
pub const MAX_TESS_EVALUATION_SHADER_STORAGE_BLOCKS: GLenum = 0x90D9;
pub const MAX_TESS_EVALUATION_TEXTURE_IMAGE_UNITS: GLenum = 0x8E82;
pub const MAX_TESS_EVALUATION_UNIFORM_BLOCKS: GLenum = 0x8E8A;
pub const MAX_TESS_EVALUATION_UNIFORM_COMPONENTS: GLenum = 0x8E80;
pub const MAX_TESS_GEN_LEVEL: GLenum = 0x8E7E;
pub const MAX_TESS_PATCH_COMPONENTS: GLenum = 0x8E84;
pub const MAX_TEXTURE_BUFFER_SIZE: GLenum = 0x8C2B;
pub const MAX_TEXTURE_IMAGE_UNITS: GLenum = 0x8872;
pub const MAX_TEXTURE_LOD_BIAS: GLenum = 0x84FD;
pub const MAX_TEXTURE_MAX_ANISOTROPY: GLenum = 0x84FF;
pub const MAX_TEXTURE_SIZE: GLenum = 0x0D33;
pub const MAX_TRANSFORM_FEEDBACK_BUFFERS: GLenum = 0x8E70;
pub const MAX_TRANSFORM_FEEDBACK_INTERLEAVED_COMPONENTS: GLenum = 0x8C8A;
pub const MAX_TRANSFORM_FEEDBACK_SEPARATE_ATTRIBS: GLenum = 0x8C8B;
pub const MAX_TRANSFORM_FEEDBACK_SEPARATE_COMPONENTS: GLenum = 0x8C80;
pub const MAX_UNIFORM_BLOCK_SIZE: GLenum = 0x8A30;
pub const MAX_UNIFORM_BUFFER_BINDINGS: GLenum = 0x8A2F;
pub const MAX_UNIFORM_LOCATIONS: GLenum = 0x826E;
pub const MAX_VARYING_COMPONENTS: GLenum = 0x8B4B;
pub const MAX_VARYING_FLOATS: GLenum = 0x8B4B;
pub const MAX_VARYING_VECTORS: GLenum = 0x8DFC;
pub const MAX_VERTEX_ATOMIC_COUNTERS: GLenum = 0x92D2;
pub const MAX_VERTEX_ATOMIC_COUNTER_BUFFERS: GLenum = 0x92CC;
pub const MAX_VERTEX_ATTRIBS: GLenum = 0x8869;
pub const MAX_VERTEX_ATTRIB_BINDINGS: GLenum = 0x82DA;
pub const MAX_VERTEX_ATTRIB_RELATIVE_OFFSET: GLenum = 0x82D9;
pub const MAX_VERTEX_ATTRIB_STRIDE: GLenum = 0x82E5;
pub const MAX_VERTEX_IMAGE_UNIFORMS: GLenum = 0x90CA;
pub const MAX_VERTEX_OUTPUT_COMPONENTS: GLenum = 0x9122;
pub const MAX_VERTEX_SHADER_STORAGE_BLOCKS: GLenum = 0x90D6;
pub const MAX_VERTEX_STREAMS: GLenum = 0x8E71;
pub const MAX_VERTEX_TEXTURE_IMAGE_UNITS: GLenum = 0x8B4C;
pub const MAX_VERTEX_UNIFORM_BLOCKS: GLenum = 0x8A2B;
pub const MAX_VERTEX_UNIFORM_COMPONENTS: GLenum = 0x8B4A;
pub const MAX_VERTEX_UNIFORM_VECTORS: GLenum = 0x8DFB;
pub const MAX_VIEWPORTS: GLenum = 0x825B;
pub const MAX_VIEWPORT_DIMS: GLenum = 0x0D3A;
pub const MAX_WIDTH: GLenum = 0x827E;
pub const MEDIUM_FLOAT: GLenum = 0x8DF1;
pub const MEDIUM_INT: GLenum = 0x8DF4;
pub const MIN: GLenum = 0x8007;
pub const MINOR_VERSION: GLenum = 0x821C;
pub const MIN_FRAGMENT_INTERPOLATION_OFFSET: GLenum = 0x8E5B;
pub const MIN_MAP_BUFFER_ALIGNMENT: GLenum = 0x90BC;
pub const MIN_PROGRAM_TEXEL_OFFSET: GLenum = 0x8904;
pub const MIN_PROGRAM_TEXTURE_GATHER_OFFSET: GLenum = 0x8E5E;
pub const MIN_SAMPLE_SHADING_VALUE: GLenum = 0x8C37;
pub const MIPMAP: GLenum = 0x8293;
pub const MIRRORED_REPEAT: GLenum = 0x8370;
pub const MIRROR_CLAMP_TO_EDGE: GLenum = 0x8743;
pub const MULTISAMPLE: GLenum = 0x809D;
pub const NAME_LENGTH: GLenum = 0x92F9;
pub const NAND: GLenum = 0x150E;
pub const NEAREST: GLenum = 0x2600;
pub const NEAREST_MIPMAP_LINEAR: GLenum = 0x2702;
pub const NEAREST_MIPMAP_NEAREST: GLenum = 0x2700;
pub const NEGATIVE_ONE_TO_ONE: GLenum = 0x935E;
pub const NEVER: GLenum = 0x0200;
pub const NICEST: GLenum = 0x1102;
pub const NONE: GLenum = 0;
pub const NOOP: GLenum = 0x1505;
pub const NOR: GLenum = 0x1508;
pub const NOTEQUAL: GLenum = 0x0205;
pub const NO_ERROR: GLenum = 0;
pub const NO_RESET_NOTIFICATION: GLenum = 0x8261;
pub const NUM_ACTIVE_VARIABLES: GLenum = 0x9304;
pub const NUM_COMPATIBLE_SUBROUTINES: GLenum = 0x8E4A;
pub const NUM_COMPRESSED_TEXTURE_FORMATS: GLenum = 0x86A2;
pub const NUM_EXTENSIONS: GLenum = 0x821D;
pub const NUM_PROGRAM_BINARY_FORMATS: GLenum = 0x87FE;
pub const NUM_SAMPLE_COUNTS: GLenum = 0x9380;
pub const NUM_SHADER_BINARY_FORMATS: GLenum = 0x8DF9;
pub const NUM_SHADING_LANGUAGE_VERSIONS: GLenum = 0x82E9;
pub const NUM_SPIR_V_EXTENSIONS: GLenum = 0x9554;
pub const OBJECT_TYPE: GLenum = 0x9112;
pub const OFFSET: GLenum = 0x92FC;
pub const ONE: GLenum = 1;
pub const ONE_MINUS_CONSTANT_ALPHA: GLenum = 0x8004;
pub const ONE_MINUS_CONSTANT_COLOR: GLenum = 0x8002;
pub const ONE_MINUS_DST_ALPHA: GLenum = 0x0305;
pub const ONE_MINUS_DST_COLOR: GLenum = 0x0307;
pub const ONE_MINUS_SRC1_ALPHA: GLenum = 0x88FB;
pub const ONE_MINUS_SRC1_COLOR: GLenum = 0x88FA;
pub const ONE_MINUS_SRC_ALPHA: GLenum = 0x0303;
pub const ONE_MINUS_SRC_COLOR: GLenum = 0x0301;
pub const OR: GLenum = 0x1507;
pub const OR_INVERTED: GLenum = 0x150D;
pub const OR_REVERSE: GLenum = 0x150B;
pub const OUT_OF_MEMORY: GLenum = 0x0505;
pub const PACK_ALIGNMENT: GLenum = 0x0D05;
pub const PACK_COMPRESSED_BLOCK_DEPTH: GLenum = 0x912D;
pub const PACK_COMPRESSED_BLOCK_HEIGHT: GLenum = 0x912C;
pub const PACK_COMPRESSED_BLOCK_SIZE: GLenum = 0x912E;
pub const PACK_COMPRESSED_BLOCK_WIDTH: GLenum = 0x912B;
pub const PACK_IMAGE_HEIGHT: GLenum = 0x806C;
pub const PACK_LSB_FIRST: GLenum = 0x0D01;
pub const PACK_ROW_LENGTH: GLenum = 0x0D02;
pub const PACK_SKIP_IMAGES: GLenum = 0x806B;
pub const PACK_SKIP_PIXELS: GLenum = 0x0D04;
pub const PACK_SKIP_ROWS: GLenum = 0x0D03;
pub const PACK_SWAP_BYTES: GLenum = 0x0D00;
pub const PARAMETER_BUFFER: GLenum = 0x80EE;
pub const PARAMETER_BUFFER_BINDING: GLenum = 0x80EF;
pub const PATCHES: GLenum = 0x000E;
pub const PATCH_DEFAULT_INNER_LEVEL: GLenum = 0x8E73;
pub const PATCH_DEFAULT_OUTER_LEVEL: GLenum = 0x8E74;
pub const PATCH_VERTICES: GLenum = 0x8E72;
pub const PIXEL_BUFFER_BARRIER_BIT: GLenum = 0x00000080;
pub const PIXEL_PACK_BUFFER: GLenum = 0x88EB;
pub const PIXEL_PACK_BUFFER_BINDING: GLenum = 0x88ED;
pub const PIXEL_UNPACK_BUFFER: GLenum = 0x88EC;
pub const PIXEL_UNPACK_BUFFER_BINDING: GLenum = 0x88EF;
pub const POINT: GLenum = 0x1B00;
pub const POINTS: GLenum = 0x0000;
pub const POINT_FADE_THRESHOLD_SIZE: GLenum = 0x8128;
pub const POINT_SIZE: GLenum = 0x0B11;
pub const POINT_SIZE_GRANULARITY: GLenum = 0x0B13;
pub const POINT_SIZE_RANGE: GLenum = 0x0B12;
pub const POINT_SPRITE_COORD_ORIGIN: GLenum = 0x8CA0;
pub const POLYGON_MODE: GLenum = 0x0B40;
pub const POLYGON_OFFSET_CLAMP: GLenum = 0x8E1B;
pub const POLYGON_OFFSET_FACTOR: GLenum = 0x8038;
pub const POLYGON_OFFSET_FILL: GLenum = 0x8037;
pub const POLYGON_OFFSET_LINE: GLenum = 0x2A02;
pub const POLYGON_OFFSET_POINT: GLenum = 0x2A01;
pub const POLYGON_OFFSET_UNITS: GLenum = 0x2A00;
pub const POLYGON_SMOOTH: GLenum = 0x0B41;
pub const POLYGON_SMOOTH_HINT: GLenum = 0x0C53;
pub const PRIMITIVES_GENERATED: GLenum = 0x8C87;
pub const PRIMITIVES_SUBMITTED: GLenum = 0x82EF;
pub const PRIMITIVE_RESTART: GLenum = 0x8F9D;
pub const PRIMITIVE_RESTART_FIXED_INDEX: GLenum = 0x8D69;
pub const PRIMITIVE_RESTART_FOR_PATCHES_SUPPORTED: GLenum = 0x8221;
pub const PRIMITIVE_RESTART_INDEX: GLenum = 0x8F9E;
pub const PROGRAM: GLenum = 0x82E2;
pub const PROGRAM_BINARY_FORMATS: GLenum = 0x87FF;
pub const PROGRAM_BINARY_LENGTH: GLenum = 0x8741;
pub const PROGRAM_BINARY_RETRIEVABLE_HINT: GLenum = 0x8257;
pub const PROGRAM_INPUT: GLenum = 0x92E3;
pub const PROGRAM_OUTPUT: GLenum = 0x92E4;
pub const PROGRAM_PIPELINE: GLenum = 0x82E4;
pub const PROGRAM_PIPELINE_BINDING: GLenum = 0x825A;
pub const PROGRAM_POINT_SIZE: GLenum = 0x8642;
pub const PROGRAM_SEPARABLE: GLenum = 0x8258;
pub const PROVOKING_VERTEX: GLenum = 0x8E4F;
pub const PROXY_TEXTURE_1D: GLenum = 0x8063;
pub const PROXY_TEXTURE_1D_ARRAY: GLenum = 0x8C19;
pub const PROXY_TEXTURE_2D: GLenum = 0x8064;
pub const PROXY_TEXTURE_2D_ARRAY: GLenum = 0x8C1B;
pub const PROXY_TEXTURE_2D_MULTISAMPLE: GLenum = 0x9101;
pub const PROXY_TEXTURE_2D_MULTISAMPLE_ARRAY: GLenum = 0x9103;
pub const PROXY_TEXTURE_3D: GLenum = 0x8070;
pub const PROXY_TEXTURE_CUBE_MAP: GLenum = 0x851B;
pub const PROXY_TEXTURE_CUBE_MAP_ARRAY: GLenum = 0x900B;
pub const PROXY_TEXTURE_RECTANGLE: GLenum = 0x84F7;
pub const QUADS: GLenum = 0x0007;
pub const QUADS_FOLLOW_PROVOKING_VERTEX_CONVENTION: GLenum = 0x8E4C;
pub const QUERY: GLenum = 0x82E3;
pub const QUERY_BUFFER: GLenum = 0x9192;
pub const QUERY_BUFFER_BARRIER_BIT: GLenum = 0x00008000;
pub const QUERY_BUFFER_BINDING: GLenum = 0x9193;
pub const QUERY_BY_REGION_NO_WAIT: GLenum = 0x8E16;
pub const QUERY_BY_REGION_NO_WAIT_INVERTED: GLenum = 0x8E1A;
pub const QUERY_BY_REGION_WAIT: GLenum = 0x8E15;
pub const QUERY_BY_REGION_WAIT_INVERTED: GLenum = 0x8E19;
pub const QUERY_COUNTER_BITS: GLenum = 0x8864;
pub const QUERY_NO_WAIT: GLenum = 0x8E14;
pub const QUERY_NO_WAIT_INVERTED: GLenum = 0x8E18;
pub const QUERY_RESULT: GLenum = 0x8866;
pub const QUERY_RESULT_AVAILABLE: GLenum = 0x8867;
pub const QUERY_RESULT_NO_WAIT: GLenum = 0x9194;
pub const QUERY_TARGET: GLenum = 0x82EA;
pub const QUERY_WAIT: GLenum = 0x8E13;
pub const QUERY_WAIT_INVERTED: GLenum = 0x8E17;
pub const R11F_G11F_B10F: GLenum = 0x8C3A;
pub const R16: GLenum = 0x822A;
pub const R16F: GLenum = 0x822D;
pub const R16I: GLenum = 0x8233;
pub const R16UI: GLenum = 0x8234;
pub const R16_SNORM: GLenum = 0x8F98;
pub const R32F: GLenum = 0x822E;
pub const R32I: GLenum = 0x8235;
pub const R32UI: GLenum = 0x8236;
pub const R3_G3_B2: GLenum = 0x2A10;
pub const R8: GLenum = 0x8229;
pub const R8I: GLenum = 0x8231;
pub const R8UI: GLenum = 0x8232;
pub const R8_SNORM: GLenum = 0x8F94;
pub const RASTERIZER_DISCARD: GLenum = 0x8C89;
pub const READ_BUFFER: GLenum = 0x0C02;
pub const READ_FRAMEBUFFER: GLenum = 0x8CA8;
pub const READ_FRAMEBUFFER_BINDING: GLenum = 0x8CAA;
pub const READ_ONLY: GLenum = 0x88B8;
pub const READ_PIXELS: GLenum = 0x828C;
pub const READ_PIXELS_FORMAT: GLenum = 0x828D;
pub const READ_PIXELS_TYPE: GLenum = 0x828E;
pub const READ_WRITE: GLenum = 0x88BA;
pub const RED: GLenum = 0x1903;
pub const RED_INTEGER: GLenum = 0x8D94;
pub const REFERENCED_BY_COMPUTE_SHADER: GLenum = 0x930B;
pub const REFERENCED_BY_FRAGMENT_SHADER: GLenum = 0x930A;
pub const REFERENCED_BY_GEOMETRY_SHADER: GLenum = 0x9309;
pub const REFERENCED_BY_TESS_CONTROL_SHADER: GLenum = 0x9307;
pub const REFERENCED_BY_TESS_EVALUATION_SHADER: GLenum = 0x9308;
pub const REFERENCED_BY_VERTEX_SHADER: GLenum = 0x9306;
pub const RENDERBUFFER: GLenum = 0x8D41;
pub const RENDERBUFFER_ALPHA_SIZE: GLenum = 0x8D53;
pub const RENDERBUFFER_BINDING: GLenum = 0x8CA7;
pub const RENDERBUFFER_BLUE_SIZE: GLenum = 0x8D52;
pub const RENDERBUFFER_DEPTH_SIZE: GLenum = 0x8D54;
pub const RENDERBUFFER_GREEN_SIZE: GLenum = 0x8D51;
pub const RENDERBUFFER_HEIGHT: GLenum = 0x8D43;
pub const RENDERBUFFER_INTERNAL_FORMAT: GLenum = 0x8D44;
pub const RENDERBUFFER_RED_SIZE: GLenum = 0x8D50;
pub const RENDERBUFFER_SAMPLES: GLenum = 0x8CAB;
pub const RENDERBUFFER_STENCIL_SIZE: GLenum = 0x8D55;
pub const RENDERBUFFER_WIDTH: GLenum = 0x8D42;
pub const RENDERER: GLenum = 0x1F01;
pub const REPEAT: GLenum = 0x2901;
pub const REPLACE: GLenum = 0x1E01;
pub const RESET_NOTIFICATION_STRATEGY: GLenum = 0x8256;
pub const RG: GLenum = 0x8227;
pub const RG16: GLenum = 0x822C;
pub const RG16F: GLenum = 0x822F;
pub const RG16I: GLenum = 0x8239;
pub const RG16UI: GLenum = 0x823A;
pub const RG16_SNORM: GLenum = 0x8F99;
pub const RG32F: GLenum = 0x8230;
pub const RG32I: GLenum = 0x823B;
pub const RG32UI: GLenum = 0x823C;
pub const RG8: GLenum = 0x822B;
pub const RG8I: GLenum = 0x8237;
pub const RG8UI: GLenum = 0x8238;
pub const RG8_SNORM: GLenum = 0x8F95;
pub const RGB: GLenum = 0x1907;
pub const RGB10: GLenum = 0x8052;
pub const RGB10_A2: GLenum = 0x8059;
pub const RGB10_A2UI: GLenum = 0x906F;
pub const RGB12: GLenum = 0x8053;
pub const RGB16: GLenum = 0x8054;
pub const RGB16F: GLenum = 0x881B;
pub const RGB16I: GLenum = 0x8D89;
pub const RGB16UI: GLenum = 0x8D77;
pub const RGB16_SNORM: GLenum = 0x8F9A;
pub const RGB32F: GLenum = 0x8815;
pub const RGB32I: GLenum = 0x8D83;
pub const RGB32UI: GLenum = 0x8D71;
pub const RGB4: GLenum = 0x804F;
pub const RGB5: GLenum = 0x8050;
pub const RGB565: GLenum = 0x8D62;
pub const RGB5_A1: GLenum = 0x8057;
pub const RGB8: GLenum = 0x8051;
pub const RGB8I: GLenum = 0x8D8F;
pub const RGB8UI: GLenum = 0x8D7D;
pub const RGB8_SNORM: GLenum = 0x8F96;
pub const RGB9_E5: GLenum = 0x8C3D;
pub const RGBA: GLenum = 0x1908;
pub const RGBA12: GLenum = 0x805A;
pub const RGBA16: GLenum = 0x805B;
pub const RGBA16F: GLenum = 0x881A;
pub const RGBA16I: GLenum = 0x8D88;
pub const RGBA16UI: GLenum = 0x8D76;
pub const RGBA16_SNORM: GLenum = 0x8F9B;
pub const RGBA2: GLenum = 0x8055;
pub const RGBA32F: GLenum = 0x8814;
pub const RGBA32I: GLenum = 0x8D82;
pub const RGBA32UI: GLenum = 0x8D70;
pub const RGBA4: GLenum = 0x8056;
pub const RGBA8: GLenum = 0x8058;
pub const RGBA8I: GLenum = 0x8D8E;
pub const RGBA8UI: GLenum = 0x8D7C;
pub const RGBA8_SNORM: GLenum = 0x8F97;
pub const RGBA_INTEGER: GLenum = 0x8D99;
pub const RGB_INTEGER: GLenum = 0x8D98;
pub const RG_INTEGER: GLenum = 0x8228;
pub const RIGHT: GLenum = 0x0407;
pub const SAMPLER: GLenum = 0x82E6;
pub const SAMPLER_1D: GLenum = 0x8B5D;
pub const SAMPLER_1D_ARRAY: GLenum = 0x8DC0;
pub const SAMPLER_1D_ARRAY_SHADOW: GLenum = 0x8DC3;
pub const SAMPLER_1D_SHADOW: GLenum = 0x8B61;
pub const SAMPLER_2D: GLenum = 0x8B5E;
pub const SAMPLER_2D_ARRAY: GLenum = 0x8DC1;
pub const SAMPLER_2D_ARRAY_SHADOW: GLenum = 0x8DC4;
pub const SAMPLER_2D_MULTISAMPLE: GLenum = 0x9108;
pub const SAMPLER_2D_MULTISAMPLE_ARRAY: GLenum = 0x910B;
pub const SAMPLER_2D_RECT: GLenum = 0x8B63;
pub const SAMPLER_2D_RECT_SHADOW: GLenum = 0x8B64;
pub const SAMPLER_2D_SHADOW: GLenum = 0x8B62;
pub const SAMPLER_3D: GLenum = 0x8B5F;
pub const SAMPLER_BINDING: GLenum = 0x8919;
pub const SAMPLER_BUFFER: GLenum = 0x8DC2;
pub const SAMPLER_CUBE: GLenum = 0x8B60;
pub const SAMPLER_CUBE_MAP_ARRAY: GLenum = 0x900C;
pub const SAMPLER_CUBE_MAP_ARRAY_SHADOW: GLenum = 0x900D;
pub const SAMPLER_CUBE_SHADOW: GLenum = 0x8DC5;
pub const SAMPLES: GLenum = 0x80A9;
pub const SAMPLES_PASSED: GLenum = 0x8914;
pub const SAMPLE_ALPHA_TO_COVERAGE: GLenum = 0x809E;
pub const SAMPLE_ALPHA_TO_ONE: GLenum = 0x809F;
pub const SAMPLE_BUFFERS: GLenum = 0x80A8;
pub const SAMPLE_COVERAGE: GLenum = 0x80A0;
pub const SAMPLE_COVERAGE_INVERT: GLenum = 0x80AB;
pub const SAMPLE_COVERAGE_VALUE: GLenum = 0x80AA;
pub const SAMPLE_MASK: GLenum = 0x8E51;
pub const SAMPLE_MASK_VALUE: GLenum = 0x8E52;
pub const SAMPLE_POSITION: GLenum = 0x8E50;
pub const SAMPLE_SHADING: GLenum = 0x8C36;
pub const SCISSOR_BOX: GLenum = 0x0C10;
pub const SCISSOR_TEST: GLenum = 0x0C11;
pub const SEPARATE_ATTRIBS: GLenum = 0x8C8D;
pub const SET: GLenum = 0x150F;
pub const SHADER: GLenum = 0x82E1;
pub const SHADER_BINARY_FORMATS: GLenum = 0x8DF8;
pub const SHADER_BINARY_FORMAT_SPIR_V: GLenum = 0x9551;
pub const SHADER_COMPILER: GLenum = 0x8DFA;
pub const SHADER_IMAGE_ACCESS_BARRIER_BIT: GLenum = 0x00000020;
pub const SHADER_IMAGE_ATOMIC: GLenum = 0x82A6;
pub const SHADER_IMAGE_LOAD: GLenum = 0x82A4;
pub const SHADER_IMAGE_STORE: GLenum = 0x82A5;
pub const SHADER_SOURCE_LENGTH: GLenum = 0x8B88;
pub const SHADER_STORAGE_BARRIER_BIT: GLenum = 0x00002000;
pub const SHADER_STORAGE_BLOCK: GLenum = 0x92E6;
pub const SHADER_STORAGE_BUFFER: GLenum = 0x90D2;
pub const SHADER_STORAGE_BUFFER_BINDING: GLenum = 0x90D3;
pub const SHADER_STORAGE_BUFFER_OFFSET_ALIGNMENT: GLenum = 0x90DF;
pub const SHADER_STORAGE_BUFFER_SIZE: GLenum = 0x90D5;
pub const SHADER_STORAGE_BUFFER_START: GLenum = 0x90D4;
pub const SHADER_TYPE: GLenum = 0x8B4F;
pub const SHADING_LANGUAGE_VERSION: GLenum = 0x8B8C;
pub const SHORT: GLenum = 0x1402;
pub const SIGNALED: GLenum = 0x9119;
pub const SIGNED_NORMALIZED: GLenum = 0x8F9C;
pub const SIMULTANEOUS_TEXTURE_AND_DEPTH_TEST: GLenum = 0x82AC;
pub const SIMULTANEOUS_TEXTURE_AND_DEPTH_WRITE: GLenum = 0x82AE;
pub const SIMULTANEOUS_TEXTURE_AND_STENCIL_TEST: GLenum = 0x82AD;
pub const SIMULTANEOUS_TEXTURE_AND_STENCIL_WRITE: GLenum = 0x82AF;
pub const SMOOTH_LINE_WIDTH_GRANULARITY: GLenum = 0x0B23;
pub const SMOOTH_LINE_WIDTH_RANGE: GLenum = 0x0B22;
pub const SMOOTH_POINT_SIZE_GRANULARITY: GLenum = 0x0B13;
pub const SMOOTH_POINT_SIZE_RANGE: GLenum = 0x0B12;
pub const SPIR_V_BINARY: GLenum = 0x9552;
pub const SPIR_V_EXTENSIONS: GLenum = 0x9553;
pub const SRC1_ALPHA: GLenum = 0x8589;
pub const SRC1_COLOR: GLenum = 0x88F9;
pub const SRC_ALPHA: GLenum = 0x0302;
pub const SRC_ALPHA_SATURATE: GLenum = 0x0308;
pub const SRC_COLOR: GLenum = 0x0300;
pub const SRGB: GLenum = 0x8C40;
pub const SRGB8: GLenum = 0x8C41;
pub const SRGB8_ALPHA8: GLenum = 0x8C43;
pub const SRGB_ALPHA: GLenum = 0x8C42;
pub const SRGB_READ: GLenum = 0x8297;
pub const SRGB_WRITE: GLenum = 0x8298;
pub const STACK_OVERFLOW: GLenum = 0x0503;
pub const STACK_UNDERFLOW: GLenum = 0x0504;
pub const STATIC_COPY: GLenum = 0x88E6;
pub const STATIC_DRAW: GLenum = 0x88E4;
pub const STATIC_READ: GLenum = 0x88E5;
pub const STENCIL: GLenum = 0x1802;
pub const STENCIL_ATTACHMENT: GLenum = 0x8D20;
pub const STENCIL_BACK_FAIL: GLenum = 0x8801;
pub const STENCIL_BACK_FUNC: GLenum = 0x8800;
pub const STENCIL_BACK_PASS_DEPTH_FAIL: GLenum = 0x8802;
pub const STENCIL_BACK_PASS_DEPTH_PASS: GLenum = 0x8803;
pub const STENCIL_BACK_REF: GLenum = 0x8CA3;
pub const STENCIL_BACK_VALUE_MASK: GLenum = 0x8CA4;
pub const STENCIL_BACK_WRITEMASK: GLenum = 0x8CA5;
pub const STENCIL_BUFFER_BIT: GLenum = 0x00000400;
pub const STENCIL_CLEAR_VALUE: GLenum = 0x0B91;
pub const STENCIL_COMPONENTS: GLenum = 0x8285;
pub const STENCIL_FAIL: GLenum = 0x0B94;
pub const STENCIL_FUNC: GLenum = 0x0B92;
pub const STENCIL_INDEX: GLenum = 0x1901;
pub const STENCIL_INDEX1: GLenum = 0x8D46;
pub const STENCIL_INDEX16: GLenum = 0x8D49;
pub const STENCIL_INDEX4: GLenum = 0x8D47;
pub const STENCIL_INDEX8: GLenum = 0x8D48;
pub const STENCIL_PASS_DEPTH_FAIL: GLenum = 0x0B95;
pub const STENCIL_PASS_DEPTH_PASS: GLenum = 0x0B96;
pub const STENCIL_REF: GLenum = 0x0B97;
pub const STENCIL_RENDERABLE: GLenum = 0x8288;
pub const STENCIL_TEST: GLenum = 0x0B90;
pub const STENCIL_VALUE_MASK: GLenum = 0x0B93;
pub const STENCIL_WRITEMASK: GLenum = 0x0B98;
pub const STEREO: GLenum = 0x0C33;
pub const STREAM_COPY: GLenum = 0x88E2;
pub const STREAM_DRAW: GLenum = 0x88E0;
pub const STREAM_READ: GLenum = 0x88E1;
pub const SUBPIXEL_BITS: GLenum = 0x0D50;
pub const SYNC_CONDITION: GLenum = 0x9113;
pub const SYNC_FENCE: GLenum = 0x9116;
pub const SYNC_FLAGS: GLenum = 0x9115;
pub const SYNC_FLUSH_COMMANDS_BIT: GLenum = 0x00000001;
pub const SYNC_GPU_COMMANDS_COMPLETE: GLenum = 0x9117;
pub const SYNC_STATUS: GLenum = 0x9114;
pub const TESS_CONTROL_OUTPUT_VERTICES: GLenum = 0x8E75;
pub const TESS_CONTROL_SHADER: GLenum = 0x8E88;
pub const TESS_CONTROL_SHADER_BIT: GLenum = 0x00000008;
pub const TESS_CONTROL_SHADER_PATCHES: GLenum = 0x82F1;
pub const TESS_CONTROL_SUBROUTINE: GLenum = 0x92E9;
pub const TESS_CONTROL_SUBROUTINE_UNIFORM: GLenum = 0x92EF;
pub const TESS_CONTROL_TEXTURE: GLenum = 0x829C;
pub const TESS_EVALUATION_SHADER: GLenum = 0x8E87;
pub const TESS_EVALUATION_SHADER_BIT: GLenum = 0x00000010;
pub const TESS_EVALUATION_SHADER_INVOCATIONS: GLenum = 0x82F2;
pub const TESS_EVALUATION_SUBROUTINE: GLenum = 0x92EA;
pub const TESS_EVALUATION_SUBROUTINE_UNIFORM: GLenum = 0x92F0;
pub const TESS_EVALUATION_TEXTURE: GLenum = 0x829D;
pub const TESS_GEN_MODE: GLenum = 0x8E76;
pub const TESS_GEN_POINT_MODE: GLenum = 0x8E79;
pub const TESS_GEN_SPACING: GLenum = 0x8E77;
pub const TESS_GEN_VERTEX_ORDER: GLenum = 0x8E78;
pub const TEXTURE: GLenum = 0x1702;
pub const TEXTURE0: GLenum = 0x84C0;
pub const TEXTURE1: GLenum = 0x84C1;
pub const TEXTURE10: GLenum = 0x84CA;
pub const TEXTURE11: GLenum = 0x84CB;
pub const TEXTURE12: GLenum = 0x84CC;
pub const TEXTURE13: GLenum = 0x84CD;
pub const TEXTURE14: GLenum = 0x84CE;
pub const TEXTURE15: GLenum = 0x84CF;
pub const TEXTURE16: GLenum = 0x84D0;
pub const TEXTURE17: GLenum = 0x84D1;
pub const TEXTURE18: GLenum = 0x84D2;
pub const TEXTURE19: GLenum = 0x84D3;
pub const TEXTURE2: GLenum = 0x84C2;
pub const TEXTURE20: GLenum = 0x84D4;
pub const TEXTURE21: GLenum = 0x84D5;
pub const TEXTURE22: GLenum = 0x84D6;
pub const TEXTURE23: GLenum = 0x84D7;
pub const TEXTURE24: GLenum = 0x84D8;
pub const TEXTURE25: GLenum = 0x84D9;
pub const TEXTURE26: GLenum = 0x84DA;
pub const TEXTURE27: GLenum = 0x84DB;
pub const TEXTURE28: GLenum = 0x84DC;
pub const TEXTURE29: GLenum = 0x84DD;
pub const TEXTURE3: GLenum = 0x84C3;
pub const TEXTURE30: GLenum = 0x84DE;
pub const TEXTURE31: GLenum = 0x84DF;
pub const TEXTURE4: GLenum = 0x84C4;
pub const TEXTURE5: GLenum = 0x84C5;
pub const TEXTURE6: GLenum = 0x84C6;
pub const TEXTURE7: GLenum = 0x84C7;
pub const TEXTURE8: GLenum = 0x84C8;
pub const TEXTURE9: GLenum = 0x84C9;
pub const TEXTURE_1D: GLenum = 0x0DE0;
pub const TEXTURE_1D_ARRAY: GLenum = 0x8C18;
pub const TEXTURE_2D: GLenum = 0x0DE1;
pub const TEXTURE_2D_ARRAY: GLenum = 0x8C1A;
pub const TEXTURE_2D_MULTISAMPLE: GLenum = 0x9100;
pub const TEXTURE_2D_MULTISAMPLE_ARRAY: GLenum = 0x9102;
pub const TEXTURE_3D: GLenum = 0x806F;
pub const TEXTURE_ALPHA_SIZE: GLenum = 0x805F;
pub const TEXTURE_ALPHA_TYPE: GLenum = 0x8C13;
pub const TEXTURE_BASE_LEVEL: GLenum = 0x813C;
pub const TEXTURE_BINDING_1D: GLenum = 0x8068;
pub const TEXTURE_BINDING_1D_ARRAY: GLenum = 0x8C1C;
pub const TEXTURE_BINDING_2D: GLenum = 0x8069;
pub const TEXTURE_BINDING_2D_ARRAY: GLenum = 0x8C1D;
pub const TEXTURE_BINDING_2D_MULTISAMPLE: GLenum = 0x9104;
pub const TEXTURE_BINDING_2D_MULTISAMPLE_ARRAY: GLenum = 0x9105;
pub const TEXTURE_BINDING_3D: GLenum = 0x806A;
pub const TEXTURE_BINDING_BUFFER: GLenum = 0x8C2C;
pub const TEXTURE_BINDING_CUBE_MAP: GLenum = 0x8514;
pub const TEXTURE_BINDING_CUBE_MAP_ARRAY: GLenum = 0x900A;
pub const TEXTURE_BINDING_RECTANGLE: GLenum = 0x84F6;
pub const TEXTURE_BLUE_SIZE: GLenum = 0x805E;
pub const TEXTURE_BLUE_TYPE: GLenum = 0x8C12;
pub const TEXTURE_BORDER_COLOR: GLenum = 0x1004;
pub const TEXTURE_BUFFER: GLenum = 0x8C2A;
pub const TEXTURE_BUFFER_BINDING: GLenum = 0x8C2A;
pub const TEXTURE_BUFFER_DATA_STORE_BINDING: GLenum = 0x8C2D;
pub const TEXTURE_BUFFER_OFFSET: GLenum = 0x919D;
pub const TEXTURE_BUFFER_OFFSET_ALIGNMENT: GLenum = 0x919F;
pub const TEXTURE_BUFFER_SIZE: GLenum = 0x919E;
pub const TEXTURE_COMPARE_FUNC: GLenum = 0x884D;
pub const TEXTURE_COMPARE_MODE: GLenum = 0x884C;
pub const TEXTURE_COMPRESSED: GLenum = 0x86A1;
pub const TEXTURE_COMPRESSED_BLOCK_HEIGHT: GLenum = 0x82B2;
pub const TEXTURE_COMPRESSED_BLOCK_SIZE: GLenum = 0x82B3;
pub const TEXTURE_COMPRESSED_BLOCK_WIDTH: GLenum = 0x82B1;
pub const TEXTURE_COMPRESSED_IMAGE_SIZE: GLenum = 0x86A0;
pub const TEXTURE_COMPRESSION_HINT: GLenum = 0x84EF;
pub const TEXTURE_CUBE_MAP: GLenum = 0x8513;
pub const TEXTURE_CUBE_MAP_ARRAY: GLenum = 0x9009;
pub const TEXTURE_CUBE_MAP_NEGATIVE_X: GLenum = 0x8516;
pub const TEXTURE_CUBE_MAP_NEGATIVE_Y: GLenum = 0x8518;
pub const TEXTURE_CUBE_MAP_NEGATIVE_Z: GLenum = 0x851A;
pub const TEXTURE_CUBE_MAP_POSITIVE_X: GLenum = 0x8515;
pub const TEXTURE_CUBE_MAP_POSITIVE_Y: GLenum = 0x8517;
pub const TEXTURE_CUBE_MAP_POSITIVE_Z: GLenum = 0x8519;
pub const TEXTURE_CUBE_MAP_SEAMLESS: GLenum = 0x884F;
pub const TEXTURE_DEPTH: GLenum = 0x8071;
pub const TEXTURE_DEPTH_SIZE: GLenum = 0x884A;
pub const TEXTURE_DEPTH_TYPE: GLenum = 0x8C16;
pub const TEXTURE_FETCH_BARRIER_BIT: GLenum = 0x00000008;
pub const TEXTURE_FIXED_SAMPLE_LOCATIONS: GLenum = 0x9107;
pub const TEXTURE_GATHER: GLenum = 0x82A2;
pub const TEXTURE_GATHER_SHADOW: GLenum = 0x82A3;
pub const TEXTURE_GREEN_SIZE: GLenum = 0x805D;
pub const TEXTURE_GREEN_TYPE: GLenum = 0x8C11;
pub const TEXTURE_HEIGHT: GLenum = 0x1001;
pub const TEXTURE_IMAGE_FORMAT: GLenum = 0x828F;
pub const TEXTURE_IMAGE_TYPE: GLenum = 0x8290;
pub const TEXTURE_IMMUTABLE_FORMAT: GLenum = 0x912F;
pub const TEXTURE_IMMUTABLE_LEVELS: GLenum = 0x82DF;
pub const TEXTURE_INTERNAL_FORMAT: GLenum = 0x1003;
pub const TEXTURE_LOD_BIAS: GLenum = 0x8501;
pub const TEXTURE_MAG_FILTER: GLenum = 0x2800;
pub const TEXTURE_MAX_ANISOTROPY: GLenum = 0x84FE;
pub const TEXTURE_MAX_LEVEL: GLenum = 0x813D;
pub const TEXTURE_MAX_LOD: GLenum = 0x813B;
pub const TEXTURE_MIN_FILTER: GLenum = 0x2801;
pub const TEXTURE_MIN_LOD: GLenum = 0x813A;
pub const TEXTURE_RECTANGLE: GLenum = 0x84F5;
pub const TEXTURE_RED_SIZE: GLenum = 0x805C;
pub const TEXTURE_RED_TYPE: GLenum = 0x8C10;
pub const TEXTURE_SAMPLES: GLenum = 0x9106;
pub const TEXTURE_SHADOW: GLenum = 0x82A1;
pub const TEXTURE_SHARED_SIZE: GLenum = 0x8C3F;
pub const TEXTURE_STENCIL_SIZE: GLenum = 0x88F1;
pub const TEXTURE_SWIZZLE_A: GLenum = 0x8E45;
pub const TEXTURE_SWIZZLE_B: GLenum = 0x8E44;
pub const TEXTURE_SWIZZLE_G: GLenum = 0x8E43;
pub const TEXTURE_SWIZZLE_R: GLenum = 0x8E42;
pub const TEXTURE_SWIZZLE_RGBA: GLenum = 0x8E46;
pub const TEXTURE_TARGET: GLenum = 0x1006;
pub const TEXTURE_UPDATE_BARRIER_BIT: GLenum = 0x00000100;
pub const TEXTURE_VIEW: GLenum = 0x82B5;
pub const TEXTURE_VIEW_MIN_LAYER: GLenum = 0x82DD;
pub const TEXTURE_VIEW_MIN_LEVEL: GLenum = 0x82DB;
pub const TEXTURE_VIEW_NUM_LAYERS: GLenum = 0x82DE;
pub const TEXTURE_VIEW_NUM_LEVELS: GLenum = 0x82DC;
pub const TEXTURE_WIDTH: GLenum = 0x1000;
pub const TEXTURE_WRAP_R: GLenum = 0x8072;
pub const TEXTURE_WRAP_S: GLenum = 0x2802;
pub const TEXTURE_WRAP_T: GLenum = 0x2803;
pub const TIMEOUT_EXPIRED: GLenum = 0x911B;
pub const TIMEOUT_IGNORED: GLuint64 = 0xFFFFFFFFFFFFFFFF;
pub const TIMESTAMP: GLenum = 0x8E28;
pub const TIME_ELAPSED: GLenum = 0x88BF;
pub const TOP_LEVEL_ARRAY_SIZE: GLenum = 0x930C;
pub const TOP_LEVEL_ARRAY_STRIDE: GLenum = 0x930D;
pub const TRANSFORM_FEEDBACK: GLenum = 0x8E22;
pub const TRANSFORM_FEEDBACK_ACTIVE: GLenum = 0x8E24;
pub const TRANSFORM_FEEDBACK_BARRIER_BIT: GLenum = 0x00000800;
pub const TRANSFORM_FEEDBACK_BINDING: GLenum = 0x8E25;
pub const TRANSFORM_FEEDBACK_BUFFER: GLenum = 0x8C8E;
pub const TRANSFORM_FEEDBACK_BUFFER_ACTIVE: GLenum = 0x8E24;
pub const TRANSFORM_FEEDBACK_BUFFER_BINDING: GLenum = 0x8C8F;
pub const TRANSFORM_FEEDBACK_BUFFER_INDEX: GLenum = 0x934B;
pub const TRANSFORM_FEEDBACK_BUFFER_MODE: GLenum = 0x8C7F;
pub const TRANSFORM_FEEDBACK_BUFFER_PAUSED: GLenum = 0x8E23;
pub const TRANSFORM_FEEDBACK_BUFFER_SIZE: GLenum = 0x8C85;
pub const TRANSFORM_FEEDBACK_BUFFER_START: GLenum = 0x8C84;
pub const TRANSFORM_FEEDBACK_BUFFER_STRIDE: GLenum = 0x934C;
pub const TRANSFORM_FEEDBACK_OVERFLOW: GLenum = 0x82EC;
pub const TRANSFORM_FEEDBACK_PAUSED: GLenum = 0x8E23;
pub const TRANSFORM_FEEDBACK_PRIMITIVES_WRITTEN: GLenum = 0x8C88;
pub const TRANSFORM_FEEDBACK_STREAM_OVERFLOW: GLenum = 0x82ED;
pub const TRANSFORM_FEEDBACK_VARYING: GLenum = 0x92F4;
pub const TRANSFORM_FEEDBACK_VARYINGS: GLenum = 0x8C83;
pub const TRANSFORM_FEEDBACK_VARYING_MAX_LENGTH: GLenum = 0x8C76;
pub const TRIANGLES: GLenum = 0x0004;
pub const TRIANGLES_ADJACENCY: GLenum = 0x000C;
pub const TRIANGLE_FAN: GLenum = 0x0006;
pub const TRIANGLE_STRIP: GLenum = 0x0005;
pub const TRIANGLE_STRIP_ADJACENCY: GLenum = 0x000D;
pub const TRUE: GLboolean = 1;
pub const TYPE: GLenum = 0x92FA;
pub const UNDEFINED_VERTEX: GLenum = 0x8260;
pub const UNIFORM: GLenum = 0x92E1;
pub const UNIFORM_ARRAY_STRIDE: GLenum = 0x8A3C;
pub const UNIFORM_ATOMIC_COUNTER_BUFFER_INDEX: GLenum = 0x92DA;
pub const UNIFORM_BARRIER_BIT: GLenum = 0x00000004;
pub const UNIFORM_BLOCK: GLenum = 0x92E2;
pub const UNIFORM_BLOCK_ACTIVE_UNIFORMS: GLenum = 0x8A42;
pub const UNIFORM_BLOCK_ACTIVE_UNIFORM_INDICES: GLenum = 0x8A43;
pub const UNIFORM_BLOCK_BINDING: GLenum = 0x8A3F;
pub const UNIFORM_BLOCK_DATA_SIZE: GLenum = 0x8A40;
pub const UNIFORM_BLOCK_INDEX: GLenum = 0x8A3A;
pub const UNIFORM_BLOCK_NAME_LENGTH: GLenum = 0x8A41;
pub const UNIFORM_BLOCK_REFERENCED_BY_COMPUTE_SHADER: GLenum = 0x90EC;
pub const UNIFORM_BLOCK_REFERENCED_BY_FRAGMENT_SHADER: GLenum = 0x8A46;
pub const UNIFORM_BLOCK_REFERENCED_BY_GEOMETRY_SHADER: GLenum = 0x8A45;
pub const UNIFORM_BLOCK_REFERENCED_BY_TESS_CONTROL_SHADER: GLenum = 0x84F0;
pub const UNIFORM_BLOCK_REFERENCED_BY_TESS_EVALUATION_SHADER: GLenum = 0x84F1;
pub const UNIFORM_BLOCK_REFERENCED_BY_VERTEX_SHADER: GLenum = 0x8A44;
pub const UNIFORM_BUFFER: GLenum = 0x8A11;
pub const UNIFORM_BUFFER_BINDING: GLenum = 0x8A28;
pub const UNIFORM_BUFFER_OFFSET_ALIGNMENT: GLenum = 0x8A34;
pub const UNIFORM_BUFFER_SIZE: GLenum = 0x8A2A;
pub const UNIFORM_BUFFER_START: GLenum = 0x8A29;
pub const UNIFORM_IS_ROW_MAJOR: GLenum = 0x8A3E;
pub const UNIFORM_MATRIX_STRIDE: GLenum = 0x8A3D;
pub const UNIFORM_NAME_LENGTH: GLenum = 0x8A39;
pub const UNIFORM_OFFSET: GLenum = 0x8A3B;
pub const UNIFORM_SIZE: GLenum = 0x8A38;
pub const UNIFORM_TYPE: GLenum = 0x8A37;
pub const UNKNOWN_CONTEXT_RESET: GLenum = 0x8255;
pub const UNPACK_ALIGNMENT: GLenum = 0x0CF5;
pub const UNPACK_COMPRESSED_BLOCK_DEPTH: GLenum = 0x9129;
pub const UNPACK_COMPRESSED_BLOCK_HEIGHT: GLenum = 0x9128;
pub const UNPACK_COMPRESSED_BLOCK_SIZE: GLenum = 0x912A;
pub const UNPACK_COMPRESSED_BLOCK_WIDTH: GLenum = 0x9127;
pub const UNPACK_IMAGE_HEIGHT: GLenum = 0x806E;
pub const UNPACK_LSB_FIRST: GLenum = 0x0CF1;
pub const UNPACK_ROW_LENGTH: GLenum = 0x0CF2;
pub const UNPACK_SKIP_IMAGES: GLenum = 0x806D;
pub const UNPACK_SKIP_PIXELS: GLenum = 0x0CF4;
pub const UNPACK_SKIP_ROWS: GLenum = 0x0CF3;
pub const UNPACK_SWAP_BYTES: GLenum = 0x0CF0;
pub const UNSIGNALED: GLenum = 0x9118;
pub const UNSIGNED_BYTE: GLenum = 0x1401;
pub const UNSIGNED_BYTE_2_3_3_REV: GLenum = 0x8362;
pub const UNSIGNED_BYTE_3_3_2: GLenum = 0x8032;
pub const UNSIGNED_INT: GLenum = 0x1405;
pub const UNSIGNED_INT_10F_11F_11F_REV: GLenum = 0x8C3B;
pub const UNSIGNED_INT_10_10_10_2: GLenum = 0x8036;
pub const UNSIGNED_INT_24_8: GLenum = 0x84FA;
pub const UNSIGNED_INT_2_10_10_10_REV: GLenum = 0x8368;
pub const UNSIGNED_INT_5_9_9_9_REV: GLenum = 0x8C3E;
pub const UNSIGNED_INT_8_8_8_8: GLenum = 0x8035;
pub const UNSIGNED_INT_8_8_8_8_REV: GLenum = 0x8367;
pub const UNSIGNED_INT_ATOMIC_COUNTER: GLenum = 0x92DB;
pub const UNSIGNED_INT_IMAGE_1D: GLenum = 0x9062;
pub const UNSIGNED_INT_IMAGE_1D_ARRAY: GLenum = 0x9068;
pub const UNSIGNED_INT_IMAGE_2D: GLenum = 0x9063;
pub const UNSIGNED_INT_IMAGE_2D_ARRAY: GLenum = 0x9069;
pub const UNSIGNED_INT_IMAGE_2D_MULTISAMPLE: GLenum = 0x906B;
pub const UNSIGNED_INT_IMAGE_2D_MULTISAMPLE_ARRAY: GLenum = 0x906C;
pub const UNSIGNED_INT_IMAGE_2D_RECT: GLenum = 0x9065;
pub const UNSIGNED_INT_IMAGE_3D: GLenum = 0x9064;
pub const UNSIGNED_INT_IMAGE_BUFFER: GLenum = 0x9067;
pub const UNSIGNED_INT_IMAGE_CUBE: GLenum = 0x9066;
pub const UNSIGNED_INT_IMAGE_CUBE_MAP_ARRAY: GLenum = 0x906A;
pub const UNSIGNED_INT_SAMPLER_1D: GLenum = 0x8DD1;
pub const UNSIGNED_INT_SAMPLER_1D_ARRAY: GLenum = 0x8DD6;
pub const UNSIGNED_INT_SAMPLER_2D: GLenum = 0x8DD2;
pub const UNSIGNED_INT_SAMPLER_2D_ARRAY: GLenum = 0x8DD7;
pub const UNSIGNED_INT_SAMPLER_2D_MULTISAMPLE: GLenum = 0x910A;
pub const UNSIGNED_INT_SAMPLER_2D_MULTISAMPLE_ARRAY: GLenum = 0x910D;
pub const UNSIGNED_INT_SAMPLER_2D_RECT: GLenum = 0x8DD5;
pub const UNSIGNED_INT_SAMPLER_3D: GLenum = 0x8DD3;
pub const UNSIGNED_INT_SAMPLER_BUFFER: GLenum = 0x8DD8;
pub const UNSIGNED_INT_SAMPLER_CUBE: GLenum = 0x8DD4;
pub const UNSIGNED_INT_SAMPLER_CUBE_MAP_ARRAY: GLenum = 0x900F;
pub const UNSIGNED_INT_VEC2: GLenum = 0x8DC6;
pub const UNSIGNED_INT_VEC3: GLenum = 0x8DC7;
pub const UNSIGNED_INT_VEC4: GLenum = 0x8DC8;
pub const UNSIGNED_NORMALIZED: GLenum = 0x8C17;
pub const UNSIGNED_SHORT: GLenum = 0x1403;
pub const UNSIGNED_SHORT_1_5_5_5_REV: GLenum = 0x8366;
pub const UNSIGNED_SHORT_4_4_4_4: GLenum = 0x8033;
pub const UNSIGNED_SHORT_4_4_4_4_REV: GLenum = 0x8365;
pub const UNSIGNED_SHORT_5_5_5_1: GLenum = 0x8034;
pub const UNSIGNED_SHORT_5_6_5: GLenum = 0x8363;
pub const UNSIGNED_SHORT_5_6_5_REV: GLenum = 0x8364;
pub const UPPER_LEFT: GLenum = 0x8CA2;
pub const VALIDATE_STATUS: GLenum = 0x8B83;
pub const VENDOR: GLenum = 0x1F00;
pub const VERSION: GLenum = 0x1F02;
pub const VERTEX_ARRAY: GLenum = 0x8074;
pub const VERTEX_ARRAY_BINDING: GLenum = 0x85B5;
pub const VERTEX_ATTRIB_ARRAY_BARRIER_BIT: GLenum = 0x00000001;
pub const VERTEX_ATTRIB_ARRAY_BUFFER_BINDING: GLenum = 0x889F;
pub const VERTEX_ATTRIB_ARRAY_DIVISOR: GLenum = 0x88FE;
pub const VERTEX_ATTRIB_ARRAY_ENABLED: GLenum = 0x8622;
pub const VERTEX_ATTRIB_ARRAY_INTEGER: GLenum = 0x88FD;
pub const VERTEX_ATTRIB_ARRAY_LONG: GLenum = 0x874E;
pub const VERTEX_ATTRIB_ARRAY_NORMALIZED: GLenum = 0x886A;
pub const VERTEX_ATTRIB_ARRAY_POINTER: GLenum = 0x8645;
pub const VERTEX_ATTRIB_ARRAY_SIZE: GLenum = 0x8623;
pub const VERTEX_ATTRIB_ARRAY_STRIDE: GLenum = 0x8624;
pub const VERTEX_ATTRIB_ARRAY_TYPE: GLenum = 0x8625;
pub const VERTEX_ATTRIB_BINDING: GLenum = 0x82D4;
pub const VERTEX_ATTRIB_RELATIVE_OFFSET: GLenum = 0x82D5;
pub const VERTEX_BINDING_BUFFER: GLenum = 0x8F4F;
pub const VERTEX_BINDING_DIVISOR: GLenum = 0x82D6;
pub const VERTEX_BINDING_OFFSET: GLenum = 0x82D7;
pub const VERTEX_BINDING_STRIDE: GLenum = 0x82D8;
pub const VERTEX_PROGRAM_POINT_SIZE: GLenum = 0x8642;
pub const VERTEX_SHADER: GLenum = 0x8B31;
pub const VERTEX_SHADER_BIT: GLenum = 0x00000001;
pub const VERTEX_SHADER_INVOCATIONS: GLenum = 0x82F0;
pub const VERTEX_SUBROUTINE: GLenum = 0x92E8;
pub const VERTEX_SUBROUTINE_UNIFORM: GLenum = 0x92EE;
pub const VERTEX_TEXTURE: GLenum = 0x829B;
pub const VERTICES_SUBMITTED: GLenum = 0x82EE;
pub const VIEWPORT: GLenum = 0x0BA2;
pub const VIEWPORT_BOUNDS_RANGE: GLenum = 0x825D;
pub const VIEWPORT_INDEX_PROVOKING_VERTEX: GLenum = 0x825F;
pub const VIEWPORT_SUBPIXEL_BITS: GLenum = 0x825C;
pub const VIEW_CLASS_128_BITS: GLenum = 0x82C4;
pub const VIEW_CLASS_16_BITS: GLenum = 0x82CA;
pub const VIEW_CLASS_24_BITS: GLenum = 0x82C9;
pub const VIEW_CLASS_32_BITS: GLenum = 0x82C8;
pub const VIEW_CLASS_48_BITS: GLenum = 0x82C7;
pub const VIEW_CLASS_64_BITS: GLenum = 0x82C6;
pub const VIEW_CLASS_8_BITS: GLenum = 0x82CB;
pub const VIEW_CLASS_96_BITS: GLenum = 0x82C5;
pub const VIEW_CLASS_BPTC_FLOAT: GLenum = 0x82D3;
pub const VIEW_CLASS_BPTC_UNORM: GLenum = 0x82D2;
pub const VIEW_CLASS_RGTC1_RED: GLenum = 0x82D0;
pub const VIEW_CLASS_RGTC2_RG: GLenum = 0x82D1;
pub const VIEW_CLASS_S3TC_DXT1_RGB: GLenum = 0x82CC;
pub const VIEW_CLASS_S3TC_DXT1_RGBA: GLenum = 0x82CD;
pub const VIEW_CLASS_S3TC_DXT3_RGBA: GLenum = 0x82CE;
pub const VIEW_CLASS_S3TC_DXT5_RGBA: GLenum = 0x82CF;
pub const VIEW_COMPATIBILITY_CLASS: GLenum = 0x82B6;
pub const WAIT_FAILED: GLenum = 0x911D;
pub const WRITE_ONLY: GLenum = 0x88B9;
pub const XOR: GLenum = 0x1506;
pub const ZERO: GLenum = 0;
pub const ZERO_TO_ONE: GLenum = 0x935F;
pub unsafe fn ActiveShaderProgram(pipeline: GLuint, program: GLuint) { _ActiveShaderProgram(pipeline, program) }
pub unsafe fn ActiveTexture(texture: GLenum) { _ActiveTexture(texture) }
pub unsafe fn AttachShader(program: GLuint, shader: GLuint) { _AttachShader(program, shader) }
pub unsafe fn BeginConditionalRender(id: GLuint, mode: GLenum) { _BeginConditionalRender(id, mode) }
pub unsafe fn BeginQuery(target: GLenum, id: GLuint) { _BeginQuery(target, id) }
pub unsafe fn BeginQueryIndexed(target: GLenum, index: GLuint, id: GLuint) { _BeginQueryIndexed(target, index, id) }
pub unsafe fn BeginTransformFeedback(primitiveMode: GLenum) { _BeginTransformFeedback(primitiveMode) }
pub unsafe fn BindAttribLocation(program: GLuint, index: GLuint, name: *const GLchar) { _BindAttribLocation(program, index, name) }
pub unsafe fn BindBuffer(target: GLenum, buffer: GLuint) { _BindBuffer(target, buffer) }
pub unsafe fn BindBufferBase(target: GLenum, index: GLuint, buffer: GLuint) { _BindBufferBase(target, index, buffer) }
pub unsafe fn BindBufferRange(target: GLenum, index: GLuint, buffer: GLuint, offset: GLintptr, size: GLsizeiptr) { _BindBufferRange(target, index, buffer, offset, size) }
pub unsafe fn BindBuffersBase(target: GLenum, first: GLuint, count: GLsizei, buffers: *const GLuint) { _BindBuffersBase(target, first, count, buffers) }
pub unsafe fn BindBuffersRange(target: GLenum, first: GLuint, count: GLsizei, buffers: *const GLuint, offsets: *const GLintptr, sizes: *const GLsizeiptr) { _BindBuffersRange(target, first, count, buffers, offsets, sizes) }
pub unsafe fn BindFragDataLocation(program: GLuint, color: GLuint, name: *const GLchar) { _BindFragDataLocation(program, color, name) }
pub unsafe fn BindFragDataLocationIndexed(program: GLuint, colorNumber: GLuint, index: GLuint, name: *const GLchar) { _BindFragDataLocationIndexed(program, colorNumber, index, name) }
pub unsafe fn BindFramebuffer(target: GLenum, framebuffer: GLuint) { _BindFramebuffer(target, framebuffer) }
pub unsafe fn BindImageTexture(unit: GLuint, texture: GLuint, level: GLint, layered: GLboolean, layer: GLint, access: GLenum, format: GLenum) { _BindImageTexture(unit, texture, level, layered, layer, access, format) }
pub unsafe fn BindImageTextures(first: GLuint, count: GLsizei, textures: *const GLuint) { _BindImageTextures(first, count, textures) }
pub unsafe fn BindProgramPipeline(pipeline: GLuint) { _BindProgramPipeline(pipeline) }
pub unsafe fn BindRenderbuffer(target: GLenum, renderbuffer: GLuint) { _BindRenderbuffer(target, renderbuffer) }
pub unsafe fn BindSampler(unit: GLuint, sampler: GLuint) { _BindSampler(unit, sampler) }
pub unsafe fn BindSamplers(first: GLuint, count: GLsizei, samplers: *const GLuint) { _BindSamplers(first, count, samplers) }
pub unsafe fn BindTexture(target: GLenum, texture: GLuint) { _BindTexture(target, texture) }
pub unsafe fn BindTextureUnit(unit: GLuint, texture: GLuint) { _BindTextureUnit(unit, texture) }
pub unsafe fn BindTextures(first: GLuint, count: GLsizei, textures: *const GLuint) { _BindTextures(first, count, textures) }
pub unsafe fn BindTransformFeedback(target: GLenum, id: GLuint) { _BindTransformFeedback(target, id) }
pub unsafe fn BindVertexArray(array: GLuint) { _BindVertexArray(array) }
pub unsafe fn BindVertexBuffer(bindingindex: GLuint, buffer: GLuint, offset: GLintptr, stride: GLsizei) { _BindVertexBuffer(bindingindex, buffer, offset, stride) }
pub unsafe fn BindVertexBuffers(first: GLuint, count: GLsizei, buffers: *const GLuint, offsets: *const GLintptr, strides: *const GLsizei) { _BindVertexBuffers(first, count, buffers, offsets, strides) }
pub unsafe fn BlendColor(red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat) { _BlendColor(red, green, blue, alpha) }
pub unsafe fn BlendEquation(mode: GLenum) { _BlendEquation(mode) }
pub unsafe fn BlendEquationSeparate(modeRGB: GLenum, modeAlpha: GLenum) { _BlendEquationSeparate(modeRGB, modeAlpha) }
pub unsafe fn BlendEquationSeparatei(buf: GLuint, modeRGB: GLenum, modeAlpha: GLenum) { _BlendEquationSeparatei(buf, modeRGB, modeAlpha) }
pub unsafe fn BlendEquationi(buf: GLuint, mode: GLenum) { _BlendEquationi(buf, mode) }
pub unsafe fn BlendFunc(sfactor: GLenum, dfactor: GLenum) { _BlendFunc(sfactor, dfactor) }
pub unsafe fn BlendFuncSeparate(sfactorRGB: GLenum, dfactorRGB: GLenum, sfactorAlpha: GLenum, dfactorAlpha: GLenum) { _BlendFuncSeparate(sfactorRGB, dfactorRGB, sfactorAlpha, dfactorAlpha) }
pub unsafe fn BlendFuncSeparatei(buf: GLuint, srcRGB: GLenum, dstRGB: GLenum, srcAlpha: GLenum, dstAlpha: GLenum) { _BlendFuncSeparatei(buf, srcRGB, dstRGB, srcAlpha, dstAlpha) }
pub unsafe fn BlendFunci(buf: GLuint, src: GLenum, dst: GLenum) { _BlendFunci(buf, src, dst) }
pub unsafe fn BlitFramebuffer(srcX0: GLint, srcY0: GLint, srcX1: GLint, srcY1: GLint, dstX0: GLint, dstY0: GLint, dstX1: GLint, dstY1: GLint, mask: GLbitfield, filter: GLenum) { _BlitFramebuffer(srcX0, srcY0, srcX1, srcY1, dstX0, dstY0, dstX1, dstY1, mask, filter) }
pub unsafe fn BlitNamedFramebuffer(readFramebuffer: GLuint, drawFramebuffer: GLuint, srcX0: GLint, srcY0: GLint, srcX1: GLint, srcY1: GLint, dstX0: GLint, dstY0: GLint, dstX1: GLint, dstY1: GLint, mask: GLbitfield, filter: GLenum) { _BlitNamedFramebuffer(readFramebuffer, drawFramebuffer, srcX0, srcY0, srcX1, srcY1, dstX0, dstY0, dstX1, dstY1, mask, filter) }
pub unsafe fn BufferData(target: GLenum, size: GLsizeiptr, data: *const c_void, usage: GLenum) { _BufferData(target, size, data, usage) }
pub unsafe fn BufferStorage(target: GLenum, size: GLsizeiptr, data: *const c_void, flags: GLbitfield) { _BufferStorage(target, size, data, flags) }
pub unsafe fn BufferSubData(target: GLenum, offset: GLintptr, size: GLsizeiptr, data: *const c_void) { _BufferSubData(target, offset, size, data) }
pub unsafe fn CheckFramebufferStatus(target: GLenum) -> GLenum { _CheckFramebufferStatus(target) }
pub unsafe fn CheckNamedFramebufferStatus(framebuffer: GLuint, target: GLenum) -> GLenum { _CheckNamedFramebufferStatus(framebuffer, target) }
pub unsafe fn ClampColor(target: GLenum, clamp: GLenum) { _ClampColor(target, clamp) }
pub unsafe fn Clear(mask: GLbitfield) { _Clear(mask) }
pub unsafe fn ClearBufferData(target: GLenum, internalformat: GLenum, format: GLenum, type_: GLenum, data: *const c_void) { _ClearBufferData(target, internalformat, format, type_, data) }
pub unsafe fn ClearBufferSubData(target: GLenum, internalformat: GLenum, offset: GLintptr, size: GLsizeiptr, format: GLenum, type_: GLenum, data: *const c_void) { _ClearBufferSubData(target, internalformat, offset, size, format, type_, data) }
pub unsafe fn ClearBufferfi(buffer: GLenum, drawbuffer: GLint, depth: GLfloat, stencil: GLint) { _ClearBufferfi(buffer, drawbuffer, depth, stencil) }
pub unsafe fn ClearBufferfv(buffer: GLenum, drawbuffer: GLint, value: *const GLfloat) { _ClearBufferfv(buffer, drawbuffer, value) }
pub unsafe fn ClearBufferiv(buffer: GLenum, drawbuffer: GLint, value: *const GLint) { _ClearBufferiv(buffer, drawbuffer, value) }
pub unsafe fn ClearBufferuiv(buffer: GLenum, drawbuffer: GLint, value: *const GLuint) { _ClearBufferuiv(buffer, drawbuffer, value) }
pub unsafe fn ClearColor(red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat) { _ClearColor(red, green, blue, alpha) }
pub unsafe fn ClearDepth(depth: GLdouble) { _ClearDepth(depth) }
pub unsafe fn ClearDepthf(d: GLfloat) { _ClearDepthf(d) }
pub unsafe fn ClearNamedBufferData(buffer: GLuint, internalformat: GLenum, format: GLenum, type_: GLenum, data: *const c_void) { _ClearNamedBufferData(buffer, internalformat, format, type_, data) }
pub unsafe fn ClearNamedBufferSubData(buffer: GLuint, internalformat: GLenum, offset: GLintptr, size: GLsizeiptr, format: GLenum, type_: GLenum, data: *const c_void) { _ClearNamedBufferSubData(buffer, internalformat, offset, size, format, type_, data) }
pub unsafe fn ClearNamedFramebufferfi(framebuffer: GLuint, buffer: GLenum, drawbuffer: GLint, depth: GLfloat, stencil: GLint) { _ClearNamedFramebufferfi(framebuffer, buffer, drawbuffer, depth, stencil) }
pub unsafe fn ClearNamedFramebufferfv(framebuffer: GLuint, buffer: GLenum, drawbuffer: GLint, value: *const GLfloat) { _ClearNamedFramebufferfv(framebuffer, buffer, drawbuffer, value) }
pub unsafe fn ClearNamedFramebufferiv(framebuffer: GLuint, buffer: GLenum, drawbuffer: GLint, value: *const GLint) { _ClearNamedFramebufferiv(framebuffer, buffer, drawbuffer, value) }
pub unsafe fn ClearNamedFramebufferuiv(framebuffer: GLuint, buffer: GLenum, drawbuffer: GLint, value: *const GLuint) { _ClearNamedFramebufferuiv(framebuffer, buffer, drawbuffer, value) }
pub unsafe fn ClearStencil(s: GLint) { _ClearStencil(s) }
pub unsafe fn ClearTexImage(texture: GLuint, level: GLint, format: GLenum, type_: GLenum, data: *const c_void) { _ClearTexImage(texture, level, format, type_, data) }
pub unsafe fn ClearTexSubImage(texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: GLenum, type_: GLenum, data: *const c_void) { _ClearTexSubImage(texture, level, xoffset, yoffset, zoffset, width, height, depth, format, type_, data) }
pub unsafe fn ClientWaitSync(sync: GLsync, flags: GLbitfield, timeout: GLuint64) -> GLenum { _ClientWaitSync(sync, flags, timeout) }
pub unsafe fn ClipControl(origin: GLenum, depth: GLenum) { _ClipControl(origin, depth) }
pub unsafe fn ColorMask(red: GLboolean, green: GLboolean, blue: GLboolean, alpha: GLboolean) { _ColorMask(red, green, blue, alpha) }
pub unsafe fn ColorMaski(index: GLuint, r: GLboolean, g: GLboolean, b: GLboolean, a: GLboolean) { _ColorMaski(index, r, g, b, a) }
pub unsafe fn ColorP3ui(type_: GLenum, color: GLuint) { _ColorP3ui(type_, color) }
pub unsafe fn ColorP3uiv(type_: GLenum, color: *const GLuint) { _ColorP3uiv(type_, color) }
pub unsafe fn ColorP4ui(type_: GLenum, color: GLuint) { _ColorP4ui(type_, color) }
pub unsafe fn ColorP4uiv(type_: GLenum, color: *const GLuint) { _ColorP4uiv(type_, color) }
pub unsafe fn CompileShader(shader: GLuint) { _CompileShader(shader) }
pub unsafe fn CompressedTexImage1D(target: GLenum, level: GLint, internalformat: GLenum, width: GLsizei, border: GLint, imageSize: GLsizei, data: *const c_void) { _CompressedTexImage1D(target, level, internalformat, width, border, imageSize, data) }
pub unsafe fn CompressedTexImage2D(target: GLenum, level: GLint, internalformat: GLenum, width: GLsizei, height: GLsizei, border: GLint, imageSize: GLsizei, data: *const c_void) { _CompressedTexImage2D(target, level, internalformat, width, height, border, imageSize, data) }
pub unsafe fn CompressedTexImage3D(target: GLenum, level: GLint, internalformat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei, border: GLint, imageSize: GLsizei, data: *const c_void) { _CompressedTexImage3D(target, level, internalformat, width, height, depth, border, imageSize, data) }
pub unsafe fn CompressedTexSubImage1D(target: GLenum, level: GLint, xoffset: GLint, width: GLsizei, format: GLenum, imageSize: GLsizei, data: *const c_void) { _CompressedTexSubImage1D(target, level, xoffset, width, format, imageSize, data) }
pub unsafe fn CompressedTexSubImage2D(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: GLenum, imageSize: GLsizei, data: *const c_void) { _CompressedTexSubImage2D(target, level, xoffset, yoffset, width, height, format, imageSize, data) }
pub unsafe fn CompressedTexSubImage3D(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: GLenum, imageSize: GLsizei, data: *const c_void) { _CompressedTexSubImage3D(target, level, xoffset, yoffset, zoffset, width, height, depth, format, imageSize, data) }
pub unsafe fn CompressedTextureSubImage1D(texture: GLuint, level: GLint, xoffset: GLint, width: GLsizei, format: GLenum, imageSize: GLsizei, data: *const c_void) { _CompressedTextureSubImage1D(texture, level, xoffset, width, format, imageSize, data) }
pub unsafe fn CompressedTextureSubImage2D(texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: GLenum, imageSize: GLsizei, data: *const c_void) { _CompressedTextureSubImage2D(texture, level, xoffset, yoffset, width, height, format, imageSize, data) }
pub unsafe fn CompressedTextureSubImage3D(texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: GLenum, imageSize: GLsizei, data: *const c_void) { _CompressedTextureSubImage3D(texture, level, xoffset, yoffset, zoffset, width, height, depth, format, imageSize, data) }
pub unsafe fn CopyBufferSubData(readTarget: GLenum, writeTarget: GLenum, readOffset: GLintptr, writeOffset: GLintptr, size: GLsizeiptr) { _CopyBufferSubData(readTarget, writeTarget, readOffset, writeOffset, size) }
pub unsafe fn CopyImageSubData(srcName: GLuint, srcTarget: GLenum, srcLevel: GLint, srcX: GLint, srcY: GLint, srcZ: GLint, dstName: GLuint, dstTarget: GLenum, dstLevel: GLint, dstX: GLint, dstY: GLint, dstZ: GLint, srcWidth: GLsizei, srcHeight: GLsizei, srcDepth: GLsizei) { _CopyImageSubData(srcName, srcTarget, srcLevel, srcX, srcY, srcZ, dstName, dstTarget, dstLevel, dstX, dstY, dstZ, srcWidth, srcHeight, srcDepth) }
pub unsafe fn CopyNamedBufferSubData(readBuffer: GLuint, writeBuffer: GLuint, readOffset: GLintptr, writeOffset: GLintptr, size: GLsizeiptr) { _CopyNamedBufferSubData(readBuffer, writeBuffer, readOffset, writeOffset, size) }
pub unsafe fn CopyTexImage1D(target: GLenum, level: GLint, internalformat: GLenum, x: GLint, y: GLint, width: GLsizei, border: GLint) { _CopyTexImage1D(target, level, internalformat, x, y, width, border) }
pub unsafe fn CopyTexImage2D(target: GLenum, level: GLint, internalformat: GLenum, x: GLint, y: GLint, width: GLsizei, height: GLsizei, border: GLint) { _CopyTexImage2D(target, level, internalformat, x, y, width, height, border) }
pub unsafe fn CopyTexSubImage1D(target: GLenum, level: GLint, xoffset: GLint, x: GLint, y: GLint, width: GLsizei) { _CopyTexSubImage1D(target, level, xoffset, x, y, width) }
pub unsafe fn CopyTexSubImage2D(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, x: GLint, y: GLint, width: GLsizei, height: GLsizei) { _CopyTexSubImage2D(target, level, xoffset, yoffset, x, y, width, height) }
pub unsafe fn CopyTexSubImage3D(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, x: GLint, y: GLint, width: GLsizei, height: GLsizei) { _CopyTexSubImage3D(target, level, xoffset, yoffset, zoffset, x, y, width, height) }
pub unsafe fn CopyTextureSubImage1D(texture: GLuint, level: GLint, xoffset: GLint, x: GLint, y: GLint, width: GLsizei) { _CopyTextureSubImage1D(texture, level, xoffset, x, y, width) }
pub unsafe fn CopyTextureSubImage2D(texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, x: GLint, y: GLint, width: GLsizei, height: GLsizei) { _CopyTextureSubImage2D(texture, level, xoffset, yoffset, x, y, width, height) }
pub unsafe fn CopyTextureSubImage3D(texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, x: GLint, y: GLint, width: GLsizei, height: GLsizei) { _CopyTextureSubImage3D(texture, level, xoffset, yoffset, zoffset, x, y, width, height) }
pub unsafe fn CreateBuffers(n: GLsizei, buffers: *mut GLuint) { _CreateBuffers(n, buffers) }
pub unsafe fn CreateFramebuffers(n: GLsizei, framebuffers: *mut GLuint) { _CreateFramebuffers(n, framebuffers) }
pub unsafe fn CreateProgram() -> GLuint { _CreateProgram() }
pub unsafe fn CreateProgramPipelines(n: GLsizei, pipelines: *mut GLuint) { _CreateProgramPipelines(n, pipelines) }
pub unsafe fn CreateQueries(target: GLenum, n: GLsizei, ids: *mut GLuint) { _CreateQueries(target, n, ids) }
pub unsafe fn CreateRenderbuffers(n: GLsizei, renderbuffers: *mut GLuint) { _CreateRenderbuffers(n, renderbuffers) }
pub unsafe fn CreateSamplers(n: GLsizei, samplers: *mut GLuint) { _CreateSamplers(n, samplers) }
pub unsafe fn CreateShader(type_: GLenum) -> GLuint { _CreateShader(type_) }
pub unsafe fn CreateShaderProgramv(type_: GLenum, count: GLsizei, strings: *const *const GLchar) -> GLuint { _CreateShaderProgramv(type_, count, strings) }
pub unsafe fn CreateTextures(target: GLenum, n: GLsizei, textures: *mut GLuint) { _CreateTextures(target, n, textures) }
pub unsafe fn CreateTransformFeedbacks(n: GLsizei, ids: *mut GLuint) { _CreateTransformFeedbacks(n, ids) }
pub unsafe fn CreateVertexArrays(n: GLsizei, arrays: *mut GLuint) { _CreateVertexArrays(n, arrays) }
pub unsafe fn CullFace(mode: GLenum) { _CullFace(mode) }
pub unsafe fn DebugMessageCallback(callback: GLDEBUGPROC, userParam: *const c_void) { _DebugMessageCallback(callback, userParam) }
pub unsafe fn DebugMessageControl(source: GLenum, type_: GLenum, severity: GLenum, count: GLsizei, ids: *const GLuint, enabled: GLboolean) { _DebugMessageControl(source, type_, severity, count, ids, enabled) }
pub unsafe fn DebugMessageInsert(source: GLenum, type_: GLenum, id: GLuint, severity: GLenum, length: GLsizei, buf: *const GLchar) { _DebugMessageInsert(source, type_, id, severity, length, buf) }
pub unsafe fn DeleteBuffers(n: GLsizei, buffers: *const GLuint) { _DeleteBuffers(n, buffers) }
pub unsafe fn DeleteFramebuffers(n: GLsizei, framebuffers: *const GLuint) { _DeleteFramebuffers(n, framebuffers) }
pub unsafe fn DeleteProgram(program: GLuint) { _DeleteProgram(program) }
pub unsafe fn DeleteProgramPipelines(n: GLsizei, pipelines: *const GLuint) { _DeleteProgramPipelines(n, pipelines) }
pub unsafe fn DeleteQueries(n: GLsizei, ids: *const GLuint) { _DeleteQueries(n, ids) }
pub unsafe fn DeleteRenderbuffers(n: GLsizei, renderbuffers: *const GLuint) { _DeleteRenderbuffers(n, renderbuffers) }
pub unsafe fn DeleteSamplers(count: GLsizei, samplers: *const GLuint) { _DeleteSamplers(count, samplers) }
pub unsafe fn DeleteShader(shader: GLuint) { _DeleteShader(shader) }
pub unsafe fn DeleteSync(sync: GLsync) { _DeleteSync(sync) }
pub unsafe fn DeleteTextures(n: GLsizei, textures: *const GLuint) { _DeleteTextures(n, textures) }
pub unsafe fn DeleteTransformFeedbacks(n: GLsizei, ids: *const GLuint) { _DeleteTransformFeedbacks(n, ids) }
pub unsafe fn DeleteVertexArrays(n: GLsizei, arrays: *const GLuint) { _DeleteVertexArrays(n, arrays) }
pub unsafe fn DepthFunc(func: GLenum) { _DepthFunc(func) }
pub unsafe fn DepthMask(flag: GLboolean) { _DepthMask(flag) }
pub unsafe fn DepthRange(n: GLdouble, f: GLdouble) { _DepthRange(n, f) }
pub unsafe fn DepthRangeArrayv(first: GLuint, count: GLsizei, v: *const GLdouble) { _DepthRangeArrayv(first, count, v) }
pub unsafe fn DepthRangeIndexed(index: GLuint, n: GLdouble, f: GLdouble) { _DepthRangeIndexed(index, n, f) }
pub unsafe fn DepthRangef(n: GLfloat, f: GLfloat) { _DepthRangef(n, f) }
pub unsafe fn DetachShader(program: GLuint, shader: GLuint) { _DetachShader(program, shader) }
pub unsafe fn Disable(cap: GLenum) { _Disable(cap) }
pub unsafe fn DisableVertexArrayAttrib(vaobj: GLuint, index: GLuint) { _DisableVertexArrayAttrib(vaobj, index) }
pub unsafe fn DisableVertexAttribArray(index: GLuint) { _DisableVertexAttribArray(index) }
pub unsafe fn Disablei(target: GLenum, index: GLuint) { _Disablei(target, index) }
pub unsafe fn DispatchCompute(num_groups_x: GLuint, num_groups_y: GLuint, num_groups_z: GLuint) { _DispatchCompute(num_groups_x, num_groups_y, num_groups_z) }
pub unsafe fn DispatchComputeIndirect(indirect: GLintptr) { _DispatchComputeIndirect(indirect) }
pub unsafe fn DrawArrays(mode: GLenum, first: GLint, count: GLsizei) { _DrawArrays(mode, first, count) }
pub unsafe fn DrawArraysIndirect(mode: GLenum, indirect: *const c_void) { _DrawArraysIndirect(mode, indirect) }
pub unsafe fn DrawArraysInstanced(mode: GLenum, first: GLint, count: GLsizei, instancecount: GLsizei) { _DrawArraysInstanced(mode, first, count, instancecount) }
pub unsafe fn DrawArraysInstancedBaseInstance(mode: GLenum, first: GLint, count: GLsizei, instancecount: GLsizei, baseinstance: GLuint) { _DrawArraysInstancedBaseInstance(mode, first, count, instancecount, baseinstance) }
pub unsafe fn DrawBuffer(buf: GLenum) { _DrawBuffer(buf) }
pub unsafe fn DrawBuffers(n: GLsizei, bufs: *const GLenum) { _DrawBuffers(n, bufs) }
pub unsafe fn DrawElements(mode: GLenum, count: GLsizei, type_: GLenum, indices: *const c_void) { _DrawElements(mode, count, type_, indices) }
pub unsafe fn DrawElementsBaseVertex(mode: GLenum, count: GLsizei, type_: GLenum, indices: *const c_void, basevertex: GLint) { _DrawElementsBaseVertex(mode, count, type_, indices, basevertex) }
pub unsafe fn DrawElementsIndirect(mode: GLenum, type_: GLenum, indirect: *const c_void) { _DrawElementsIndirect(mode, type_, indirect) }
pub unsafe fn DrawElementsInstanced(mode: GLenum, count: GLsizei, type_: GLenum, indices: *const c_void, instancecount: GLsizei) { _DrawElementsInstanced(mode, count, type_, indices, instancecount) }
pub unsafe fn DrawElementsInstancedBaseInstance(mode: GLenum, count: GLsizei, type_: GLenum, indices: *const c_void, instancecount: GLsizei, baseinstance: GLuint) { _DrawElementsInstancedBaseInstance(mode, count, type_, indices, instancecount, baseinstance) }
pub unsafe fn DrawElementsInstancedBaseVertex(mode: GLenum, count: GLsizei, type_: GLenum, indices: *const c_void, instancecount: GLsizei, basevertex: GLint) { _DrawElementsInstancedBaseVertex(mode, count, type_, indices, instancecount, basevertex) }
pub unsafe fn DrawElementsInstancedBaseVertexBaseInstance(mode: GLenum, count: GLsizei, type_: GLenum, indices: *const c_void, instancecount: GLsizei, basevertex: GLint, baseinstance: GLuint) { _DrawElementsInstancedBaseVertexBaseInstance(mode, count, type_, indices, instancecount, basevertex, baseinstance) }
pub unsafe fn DrawRangeElements(mode: GLenum, start: GLuint, end: GLuint, count: GLsizei, type_: GLenum, indices: *const c_void) { _DrawRangeElements(mode, start, end, count, type_, indices) }
pub unsafe fn DrawRangeElementsBaseVertex(mode: GLenum, start: GLuint, end: GLuint, count: GLsizei, type_: GLenum, indices: *const c_void, basevertex: GLint) { _DrawRangeElementsBaseVertex(mode, start, end, count, type_, indices, basevertex) }
pub unsafe fn DrawTransformFeedback(mode: GLenum, id: GLuint) { _DrawTransformFeedback(mode, id) }
pub unsafe fn DrawTransformFeedbackInstanced(mode: GLenum, id: GLuint, instancecount: GLsizei) { _DrawTransformFeedbackInstanced(mode, id, instancecount) }
pub unsafe fn DrawTransformFeedbackStream(mode: GLenum, id: GLuint, stream: GLuint) { _DrawTransformFeedbackStream(mode, id, stream) }
pub unsafe fn DrawTransformFeedbackStreamInstanced(mode: GLenum, id: GLuint, stream: GLuint, instancecount: GLsizei) { _DrawTransformFeedbackStreamInstanced(mode, id, stream, instancecount) }
pub unsafe fn Enable(cap: GLenum) { _Enable(cap) }
pub unsafe fn EnableVertexArrayAttrib(vaobj: GLuint, index: GLuint) { _EnableVertexArrayAttrib(vaobj, index) }
pub unsafe fn EnableVertexAttribArray(index: GLuint) { _EnableVertexAttribArray(index) }
pub unsafe fn Enablei(target: GLenum, index: GLuint) { _Enablei(target, index) }
pub unsafe fn EndConditionalRender() { _EndConditionalRender() }
pub unsafe fn EndQuery(target: GLenum) { _EndQuery(target) }
pub unsafe fn EndQueryIndexed(target: GLenum, index: GLuint) { _EndQueryIndexed(target, index) }
pub unsafe fn EndTransformFeedback() { _EndTransformFeedback() }
pub unsafe fn FenceSync(condition: GLenum, flags: GLbitfield) -> GLsync { _FenceSync(condition, flags) }
pub unsafe fn Finish() { _Finish() }
pub unsafe fn Flush() { _Flush() }
pub unsafe fn FlushMappedBufferRange(target: GLenum, offset: GLintptr, length: GLsizeiptr) { _FlushMappedBufferRange(target, offset, length) }
pub unsafe fn FlushMappedNamedBufferRange(buffer: GLuint, offset: GLintptr, length: GLsizeiptr) { _FlushMappedNamedBufferRange(buffer, offset, length) }
pub unsafe fn FramebufferParameteri(target: GLenum, pname: GLenum, param: GLint) { _FramebufferParameteri(target, pname, param) }
pub unsafe fn FramebufferRenderbuffer(target: GLenum, attachment: GLenum, renderbuffertarget: GLenum, renderbuffer: GLuint) { _FramebufferRenderbuffer(target, attachment, renderbuffertarget, renderbuffer) }
pub unsafe fn FramebufferTexture(target: GLenum, attachment: GLenum, texture: GLuint, level: GLint) { _FramebufferTexture(target, attachment, texture, level) }
pub unsafe fn FramebufferTexture1D(target: GLenum, attachment: GLenum, textarget: GLenum, texture: GLuint, level: GLint) { _FramebufferTexture1D(target, attachment, textarget, texture, level) }
pub unsafe fn FramebufferTexture2D(target: GLenum, attachment: GLenum, textarget: GLenum, texture: GLuint, level: GLint) { _FramebufferTexture2D(target, attachment, textarget, texture, level) }
pub unsafe fn FramebufferTexture3D(target: GLenum, attachment: GLenum, textarget: GLenum, texture: GLuint, level: GLint, zoffset: GLint) { _FramebufferTexture3D(target, attachment, textarget, texture, level, zoffset) }
pub unsafe fn FramebufferTextureLayer(target: GLenum, attachment: GLenum, texture: GLuint, level: GLint, layer: GLint) { _FramebufferTextureLayer(target, attachment, texture, level, layer) }
pub unsafe fn FrontFace(mode: GLenum) { _FrontFace(mode) }
pub unsafe fn GenBuffers(n: GLsizei, buffers: *mut GLuint) { _GenBuffers(n, buffers) }
pub unsafe fn GenFramebuffers(n: GLsizei, framebuffers: *mut GLuint) { _GenFramebuffers(n, framebuffers) }
pub unsafe fn GenProgramPipelines(n: GLsizei, pipelines: *mut GLuint) { _GenProgramPipelines(n, pipelines) }
pub unsafe fn GenQueries(n: GLsizei, ids: *mut GLuint) { _GenQueries(n, ids) }
pub unsafe fn GenRenderbuffers(n: GLsizei, renderbuffers: *mut GLuint) { _GenRenderbuffers(n, renderbuffers) }
pub unsafe fn GenSamplers(count: GLsizei, samplers: *mut GLuint) { _GenSamplers(count, samplers) }
pub unsafe fn GenTextures(n: GLsizei, textures: *mut GLuint) { _GenTextures(n, textures) }
pub unsafe fn GenTransformFeedbacks(n: GLsizei, ids: *mut GLuint) { _GenTransformFeedbacks(n, ids) }
pub unsafe fn GenVertexArrays(n: GLsizei, arrays: *mut GLuint) { _GenVertexArrays(n, arrays) }
pub unsafe fn GenerateMipmap(target: GLenum) { _GenerateMipmap(target) }
pub unsafe fn GenerateTextureMipmap(texture: GLuint) { _GenerateTextureMipmap(texture) }
pub unsafe fn GetActiveAtomicCounterBufferiv(program: GLuint, bufferIndex: GLuint, pname: GLenum, params: *mut GLint) { _GetActiveAtomicCounterBufferiv(program, bufferIndex, pname, params) }
pub unsafe fn GetActiveAttrib(program: GLuint, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, size: *mut GLint, type_: *mut GLenum, name: *mut GLchar) { _GetActiveAttrib(program, index, bufSize, length, size, type_, name) }
pub unsafe fn GetActiveSubroutineName(program: GLuint, shadertype: GLenum, index: GLuint, bufsize: GLsizei, length: *mut GLsizei, name: *mut GLchar) { _GetActiveSubroutineName(program, shadertype, index, bufsize, length, name) }
pub unsafe fn GetActiveSubroutineUniformName(program: GLuint, shadertype: GLenum, index: GLuint, bufsize: GLsizei, length: *mut GLsizei, name: *mut GLchar) { _GetActiveSubroutineUniformName(program, shadertype, index, bufsize, length, name) }
pub unsafe fn GetActiveSubroutineUniformiv(program: GLuint, shadertype: GLenum, index: GLuint, pname: GLenum, values: *mut GLint) { _GetActiveSubroutineUniformiv(program, shadertype, index, pname, values) }
pub unsafe fn GetActiveUniform(program: GLuint, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, size: *mut GLint, type_: *mut GLenum, name: *mut GLchar) { _GetActiveUniform(program, index, bufSize, length, size, type_, name) }
pub unsafe fn GetActiveUniformBlockName(program: GLuint, uniformBlockIndex: GLuint, bufSize: GLsizei, length: *mut GLsizei, uniformBlockName: *mut GLchar) { _GetActiveUniformBlockName(program, uniformBlockIndex, bufSize, length, uniformBlockName) }
pub unsafe fn GetActiveUniformBlockiv(program: GLuint, uniformBlockIndex: GLuint, pname: GLenum, params: *mut GLint) { _GetActiveUniformBlockiv(program, uniformBlockIndex, pname, params) }
pub unsafe fn GetActiveUniformName(program: GLuint, uniformIndex: GLuint, bufSize: GLsizei, length: *mut GLsizei, uniformName: *mut GLchar) { _GetActiveUniformName(program, uniformIndex, bufSize, length, uniformName) }
pub unsafe fn GetActiveUniformsiv(program: GLuint, uniformCount: GLsizei, uniformIndices: *const GLuint, pname: GLenum, params: *mut GLint) { _GetActiveUniformsiv(program, uniformCount, uniformIndices, pname, params) }
pub unsafe fn GetAttachedShaders(program: GLuint, maxCount: GLsizei, count: *mut GLsizei, shaders: *mut GLuint) { _GetAttachedShaders(program, maxCount, count, shaders) }
pub unsafe fn GetAttribLocation(program: GLuint, name: *const GLchar) -> GLint { _GetAttribLocation(program, name) }
pub unsafe fn GetBooleani_v(target: GLenum, index: GLuint, data: *mut GLboolean) { _GetBooleani_v(target, index, data) }
pub unsafe fn GetBooleanv(pname: GLenum, data: *mut GLboolean) { _GetBooleanv(pname, data) }
pub unsafe fn GetBufferParameteri64v(target: GLenum, pname: GLenum, params: *mut GLint64) { _GetBufferParameteri64v(target, pname, params) }
pub unsafe fn GetBufferParameteriv(target: GLenum, pname: GLenum, params: *mut GLint) { _GetBufferParameteriv(target, pname, params) }
pub unsafe fn GetBufferPointerv(target: GLenum, pname: GLenum, params: *const *mut c_void) { _GetBufferPointerv(target, pname, params) }
pub unsafe fn GetBufferSubData(target: GLenum, offset: GLintptr, size: GLsizeiptr, data: *mut c_void) { _GetBufferSubData(target, offset, size, data) }
pub unsafe fn GetCompressedTexImage(target: GLenum, level: GLint, img: *mut c_void) { _GetCompressedTexImage(target, level, img) }
pub unsafe fn GetCompressedTextureImage(texture: GLuint, level: GLint, bufSize: GLsizei, pixels: *mut c_void) { _GetCompressedTextureImage(texture, level, bufSize, pixels) }
pub unsafe fn GetCompressedTextureSubImage(texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, bufSize: GLsizei, pixels: *mut c_void) { _GetCompressedTextureSubImage(texture, level, xoffset, yoffset, zoffset, width, height, depth, bufSize, pixels) }
pub unsafe fn GetDebugMessageLog(count: GLuint, bufSize: GLsizei, sources: *mut GLenum, types: *mut GLenum, ids: *mut GLuint, severities: *mut GLenum, lengths: *mut GLsizei, messageLog: *mut GLchar) -> GLuint { _GetDebugMessageLog(count, bufSize, sources, types, ids, severities, lengths, messageLog) }
pub unsafe fn GetDoublei_v(target: GLenum, index: GLuint, data: *mut GLdouble) { _GetDoublei_v(target, index, data) }
pub unsafe fn GetDoublev(pname: GLenum, data: *mut GLdouble) { _GetDoublev(pname, data) }
pub unsafe fn GetError() -> GLenum { _GetError() }
pub unsafe fn GetFloati_v(target: GLenum, index: GLuint, data: *mut GLfloat) { _GetFloati_v(target, index, data) }
pub unsafe fn GetFloatv(pname: GLenum, data: *mut GLfloat) { _GetFloatv(pname, data) }
pub unsafe fn GetFragDataIndex(program: GLuint, name: *const GLchar) -> GLint { _GetFragDataIndex(program, name) }
pub unsafe fn GetFragDataLocation(program: GLuint, name: *const GLchar) -> GLint { _GetFragDataLocation(program, name) }
pub unsafe fn GetFramebufferAttachmentParameteriv(target: GLenum, attachment: GLenum, pname: GLenum, params: *mut GLint) { _GetFramebufferAttachmentParameteriv(target, attachment, pname, params) }
pub unsafe fn GetFramebufferParameteriv(target: GLenum, pname: GLenum, params: *mut GLint) { _GetFramebufferParameteriv(target, pname, params) }
pub unsafe fn GetGraphicsResetStatus() -> GLenum { _GetGraphicsResetStatus() }
pub unsafe fn GetInteger64i_v(target: GLenum, index: GLuint, data: *mut GLint64) { _GetInteger64i_v(target, index, data) }
pub unsafe fn GetInteger64v(pname: GLenum, data: *mut GLint64) { _GetInteger64v(pname, data) }
pub unsafe fn GetIntegeri_v(target: GLenum, index: GLuint, data: *mut GLint) { _GetIntegeri_v(target, index, data) }
pub unsafe fn GetIntegerv(pname: GLenum, data: *mut GLint) { _GetIntegerv(pname, data) }
pub unsafe fn GetInternalformati64v(target: GLenum, internalformat: GLenum, pname: GLenum, bufSize: GLsizei, params: *mut GLint64) { _GetInternalformati64v(target, internalformat, pname, bufSize, params) }
pub unsafe fn GetInternalformativ(target: GLenum, internalformat: GLenum, pname: GLenum, bufSize: GLsizei, params: *mut GLint) { _GetInternalformativ(target, internalformat, pname, bufSize, params) }
pub unsafe fn GetMultisamplefv(pname: GLenum, index: GLuint, val: *mut GLfloat) { _GetMultisamplefv(pname, index, val) }
pub unsafe fn GetNamedBufferParameteri64v(buffer: GLuint, pname: GLenum, params: *mut GLint64) { _GetNamedBufferParameteri64v(buffer, pname, params) }
pub unsafe fn GetNamedBufferParameteriv(buffer: GLuint, pname: GLenum, params: *mut GLint) { _GetNamedBufferParameteriv(buffer, pname, params) }
pub unsafe fn GetNamedBufferPointerv(buffer: GLuint, pname: GLenum, params: *const *mut c_void) { _GetNamedBufferPointerv(buffer, pname, params) }
pub unsafe fn GetNamedBufferSubData(buffer: GLuint, offset: GLintptr, size: GLsizeiptr, data: *mut c_void) { _GetNamedBufferSubData(buffer, offset, size, data) }
pub unsafe fn GetNamedFramebufferAttachmentParameteriv(framebuffer: GLuint, attachment: GLenum, pname: GLenum, params: *mut GLint) { _GetNamedFramebufferAttachmentParameteriv(framebuffer, attachment, pname, params) }
pub unsafe fn GetNamedFramebufferParameteriv(framebuffer: GLuint, pname: GLenum, param: *mut GLint) { _GetNamedFramebufferParameteriv(framebuffer, pname, param) }
pub unsafe fn GetNamedRenderbufferParameteriv(renderbuffer: GLuint, pname: GLenum, params: *mut GLint) { _GetNamedRenderbufferParameteriv(renderbuffer, pname, params) }
pub unsafe fn GetObjectLabel(identifier: GLenum, name: GLuint, bufSize: GLsizei, length: *mut GLsizei, label: *mut GLchar) { _GetObjectLabel(identifier, name, bufSize, length, label) }
pub unsafe fn GetObjectPtrLabel(ptr: *const c_void, bufSize: GLsizei, length: *mut GLsizei, label: *mut GLchar) { _GetObjectPtrLabel(ptr, bufSize, length, label) }
pub unsafe fn GetPointerv(pname: GLenum, params: *const *mut c_void) { _GetPointerv(pname, params) }
pub unsafe fn GetProgramBinary(program: GLuint, bufSize: GLsizei, length: *mut GLsizei, binaryFormat: *mut GLenum, binary: *mut c_void) { _GetProgramBinary(program, bufSize, length, binaryFormat, binary) }
pub unsafe fn GetProgramInfoLog(program: GLuint, bufSize: GLsizei, length: *mut GLsizei, infoLog: *mut GLchar) { _GetProgramInfoLog(program, bufSize, length, infoLog) }
pub unsafe fn GetProgramInterfaceiv(program: GLuint, programInterface: GLenum, pname: GLenum, params: *mut GLint) { _GetProgramInterfaceiv(program, programInterface, pname, params) }
pub unsafe fn GetProgramPipelineInfoLog(pipeline: GLuint, bufSize: GLsizei, length: *mut GLsizei, infoLog: *mut GLchar) { _GetProgramPipelineInfoLog(pipeline, bufSize, length, infoLog) }
pub unsafe fn GetProgramPipelineiv(pipeline: GLuint, pname: GLenum, params: *mut GLint) { _GetProgramPipelineiv(pipeline, pname, params) }
pub unsafe fn GetProgramResourceIndex(program: GLuint, programInterface: GLenum, name: *const GLchar) -> GLuint { _GetProgramResourceIndex(program, programInterface, name) }
pub unsafe fn GetProgramResourceLocation(program: GLuint, programInterface: GLenum, name: *const GLchar) -> GLint { _GetProgramResourceLocation(program, programInterface, name) }
pub unsafe fn GetProgramResourceLocationIndex(program: GLuint, programInterface: GLenum, name: *const GLchar) -> GLint { _GetProgramResourceLocationIndex(program, programInterface, name) }
pub unsafe fn GetProgramResourceName(program: GLuint, programInterface: GLenum, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, name: *mut GLchar) { _GetProgramResourceName(program, programInterface, index, bufSize, length, name) }
pub unsafe fn GetProgramResourceiv(program: GLuint, programInterface: GLenum, index: GLuint, propCount: GLsizei, props: *const GLenum, bufSize: GLsizei, length: *mut GLsizei, params: *mut GLint) { _GetProgramResourceiv(program, programInterface, index, propCount, props, bufSize, length, params) }
pub unsafe fn GetProgramStageiv(program: GLuint, shadertype: GLenum, pname: GLenum, values: *mut GLint) { _GetProgramStageiv(program, shadertype, pname, values) }
pub unsafe fn GetProgramiv(program: GLuint, pname: GLenum, params: *mut GLint) { _GetProgramiv(program, pname, params) }
pub unsafe fn GetQueryBufferObjecti64v(id: GLuint, buffer: GLuint, pname: GLenum, offset: GLintptr) { _GetQueryBufferObjecti64v(id, buffer, pname, offset) }
pub unsafe fn GetQueryBufferObjectiv(id: GLuint, buffer: GLuint, pname: GLenum, offset: GLintptr) { _GetQueryBufferObjectiv(id, buffer, pname, offset) }
pub unsafe fn GetQueryBufferObjectui64v(id: GLuint, buffer: GLuint, pname: GLenum, offset: GLintptr) { _GetQueryBufferObjectui64v(id, buffer, pname, offset) }
pub unsafe fn GetQueryBufferObjectuiv(id: GLuint, buffer: GLuint, pname: GLenum, offset: GLintptr) { _GetQueryBufferObjectuiv(id, buffer, pname, offset) }
pub unsafe fn GetQueryIndexediv(target: GLenum, index: GLuint, pname: GLenum, params: *mut GLint) { _GetQueryIndexediv(target, index, pname, params) }
pub unsafe fn GetQueryObjecti64v(id: GLuint, pname: GLenum, params: *mut GLint64) { _GetQueryObjecti64v(id, pname, params) }
pub unsafe fn GetQueryObjectiv(id: GLuint, pname: GLenum, params: *mut GLint) { _GetQueryObjectiv(id, pname, params) }
pub unsafe fn GetQueryObjectui64v(id: GLuint, pname: GLenum, params: *mut GLuint64) { _GetQueryObjectui64v(id, pname, params) }
pub unsafe fn GetQueryObjectuiv(id: GLuint, pname: GLenum, params: *mut GLuint) { _GetQueryObjectuiv(id, pname, params) }
pub unsafe fn GetQueryiv(target: GLenum, pname: GLenum, params: *mut GLint) { _GetQueryiv(target, pname, params) }
pub unsafe fn GetRenderbufferParameteriv(target: GLenum, pname: GLenum, params: *mut GLint) { _GetRenderbufferParameteriv(target, pname, params) }
pub unsafe fn GetSamplerParameterIiv(sampler: GLuint, pname: GLenum, params: *mut GLint) { _GetSamplerParameterIiv(sampler, pname, params) }
pub unsafe fn GetSamplerParameterIuiv(sampler: GLuint, pname: GLenum, params: *mut GLuint) { _GetSamplerParameterIuiv(sampler, pname, params) }
pub unsafe fn GetSamplerParameterfv(sampler: GLuint, pname: GLenum, params: *mut GLfloat) { _GetSamplerParameterfv(sampler, pname, params) }
pub unsafe fn GetSamplerParameteriv(sampler: GLuint, pname: GLenum, params: *mut GLint) { _GetSamplerParameteriv(sampler, pname, params) }
pub unsafe fn GetShaderInfoLog(shader: GLuint, bufSize: GLsizei, length: *mut GLsizei, infoLog: *mut GLchar) { _GetShaderInfoLog(shader, bufSize, length, infoLog) }
pub unsafe fn GetShaderPrecisionFormat(shadertype: GLenum, precisiontype: GLenum, range: *mut GLint, precision: *mut GLint) { _GetShaderPrecisionFormat(shadertype, precisiontype, range, precision) }
pub unsafe fn GetShaderSource(shader: GLuint, bufSize: GLsizei, length: *mut GLsizei, source: *mut GLchar) { _GetShaderSource(shader, bufSize, length, source) }
pub unsafe fn GetShaderiv(shader: GLuint, pname: GLenum, params: *mut GLint) { _GetShaderiv(shader, pname, params) }
pub unsafe fn GetString(name: GLenum) -> *const GLubyte { _GetString(name) }
pub unsafe fn GetStringi(name: GLenum, index: GLuint) -> *const GLubyte { _GetStringi(name, index) }
pub unsafe fn GetSubroutineIndex(program: GLuint, shadertype: GLenum, name: *const GLchar) -> GLuint { _GetSubroutineIndex(program, shadertype, name) }
pub unsafe fn GetSubroutineUniformLocation(program: GLuint, shadertype: GLenum, name: *const GLchar) -> GLint { _GetSubroutineUniformLocation(program, shadertype, name) }
pub unsafe fn GetSynciv(sync: GLsync, pname: GLenum, bufSize: GLsizei, length: *mut GLsizei, values: *mut GLint) { _GetSynciv(sync, pname, bufSize, length, values) }
pub unsafe fn GetTexImage(target: GLenum, level: GLint, format: GLenum, type_: GLenum, pixels: *mut c_void) { _GetTexImage(target, level, format, type_, pixels) }
pub unsafe fn GetTexLevelParameterfv(target: GLenum, level: GLint, pname: GLenum, params: *mut GLfloat) { _GetTexLevelParameterfv(target, level, pname, params) }
pub unsafe fn GetTexLevelParameteriv(target: GLenum, level: GLint, pname: GLenum, params: *mut GLint) { _GetTexLevelParameteriv(target, level, pname, params) }
pub unsafe fn GetTexParameterIiv(target: GLenum, pname: GLenum, params: *mut GLint) { _GetTexParameterIiv(target, pname, params) }
pub unsafe fn GetTexParameterIuiv(target: GLenum, pname: GLenum, params: *mut GLuint) { _GetTexParameterIuiv(target, pname, params) }
pub unsafe fn GetTexParameterfv(target: GLenum, pname: GLenum, params: *mut GLfloat) { _GetTexParameterfv(target, pname, params) }
pub unsafe fn GetTexParameteriv(target: GLenum, pname: GLenum, params: *mut GLint) { _GetTexParameteriv(target, pname, params) }
pub unsafe fn GetTextureImage(texture: GLuint, level: GLint, format: GLenum, type_: GLenum, bufSize: GLsizei, pixels: *mut c_void) { _GetTextureImage(texture, level, format, type_, bufSize, pixels) }
pub unsafe fn GetTextureLevelParameterfv(texture: GLuint, level: GLint, pname: GLenum, params: *mut GLfloat) { _GetTextureLevelParameterfv(texture, level, pname, params) }
pub unsafe fn GetTextureLevelParameteriv(texture: GLuint, level: GLint, pname: GLenum, params: *mut GLint) { _GetTextureLevelParameteriv(texture, level, pname, params) }
pub unsafe fn GetTextureParameterIiv(texture: GLuint, pname: GLenum, params: *mut GLint) { _GetTextureParameterIiv(texture, pname, params) }
pub unsafe fn GetTextureParameterIuiv(texture: GLuint, pname: GLenum, params: *mut GLuint) { _GetTextureParameterIuiv(texture, pname, params) }
pub unsafe fn GetTextureParameterfv(texture: GLuint, pname: GLenum, params: *mut GLfloat) { _GetTextureParameterfv(texture, pname, params) }
pub unsafe fn GetTextureParameteriv(texture: GLuint, pname: GLenum, params: *mut GLint) { _GetTextureParameteriv(texture, pname, params) }
pub unsafe fn GetTextureSubImage(texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: GLenum, type_: GLenum, bufSize: GLsizei, pixels: *mut c_void) { _GetTextureSubImage(texture, level, xoffset, yoffset, zoffset, width, height, depth, format, type_, bufSize, pixels) }
pub unsafe fn GetTransformFeedbackVarying(program: GLuint, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, size: *mut GLsizei, type_: *mut GLenum, name: *mut GLchar) { _GetTransformFeedbackVarying(program, index, bufSize, length, size, type_, name) }
pub unsafe fn GetTransformFeedbacki64_v(xfb: GLuint, pname: GLenum, index: GLuint, param: *mut GLint64) { _GetTransformFeedbacki64_v(xfb, pname, index, param) }
pub unsafe fn GetTransformFeedbacki_v(xfb: GLuint, pname: GLenum, index: GLuint, param: *mut GLint) { _GetTransformFeedbacki_v(xfb, pname, index, param) }
pub unsafe fn GetTransformFeedbackiv(xfb: GLuint, pname: GLenum, param: *mut GLint) { _GetTransformFeedbackiv(xfb, pname, param) }
pub unsafe fn GetUniformBlockIndex(program: GLuint, uniformBlockName: *const GLchar) -> GLuint { _GetUniformBlockIndex(program, uniformBlockName) }
pub unsafe fn GetUniformIndices(program: GLuint, uniformCount: GLsizei, uniformNames: *const *const GLchar, uniformIndices: *mut GLuint) { _GetUniformIndices(program, uniformCount, uniformNames, uniformIndices) }
pub unsafe fn GetUniformLocation(program: GLuint, name: *const GLchar) -> GLint { _GetUniformLocation(program, name) }
pub unsafe fn GetUniformSubroutineuiv(shadertype: GLenum, location: GLint, params: *mut GLuint) { _GetUniformSubroutineuiv(shadertype, location, params) }
pub unsafe fn GetUniformdv(program: GLuint, location: GLint, params: *mut GLdouble) { _GetUniformdv(program, location, params) }
pub unsafe fn GetUniformfv(program: GLuint, location: GLint, params: *mut GLfloat) { _GetUniformfv(program, location, params) }
pub unsafe fn GetUniformiv(program: GLuint, location: GLint, params: *mut GLint) { _GetUniformiv(program, location, params) }
pub unsafe fn GetUniformuiv(program: GLuint, location: GLint, params: *mut GLuint) { _GetUniformuiv(program, location, params) }
pub unsafe fn GetVertexArrayIndexed64iv(vaobj: GLuint, index: GLuint, pname: GLenum, param: *mut GLint64) { _GetVertexArrayIndexed64iv(vaobj, index, pname, param) }
pub unsafe fn GetVertexArrayIndexediv(vaobj: GLuint, index: GLuint, pname: GLenum, param: *mut GLint) { _GetVertexArrayIndexediv(vaobj, index, pname, param) }
pub unsafe fn GetVertexArrayiv(vaobj: GLuint, pname: GLenum, param: *mut GLint) { _GetVertexArrayiv(vaobj, pname, param) }
pub unsafe fn GetVertexAttribIiv(index: GLuint, pname: GLenum, params: *mut GLint) { _GetVertexAttribIiv(index, pname, params) }
pub unsafe fn GetVertexAttribIuiv(index: GLuint, pname: GLenum, params: *mut GLuint) { _GetVertexAttribIuiv(index, pname, params) }
pub unsafe fn GetVertexAttribLdv(index: GLuint, pname: GLenum, params: *mut GLdouble) { _GetVertexAttribLdv(index, pname, params) }
pub unsafe fn GetVertexAttribPointerv(index: GLuint, pname: GLenum, pointer: *const *mut c_void) { _GetVertexAttribPointerv(index, pname, pointer) }
pub unsafe fn GetVertexAttribdv(index: GLuint, pname: GLenum, params: *mut GLdouble) { _GetVertexAttribdv(index, pname, params) }
pub unsafe fn GetVertexAttribfv(index: GLuint, pname: GLenum, params: *mut GLfloat) { _GetVertexAttribfv(index, pname, params) }
pub unsafe fn GetVertexAttribiv(index: GLuint, pname: GLenum, params: *mut GLint) { _GetVertexAttribiv(index, pname, params) }
pub unsafe fn GetnColorTable(target: GLenum, format: GLenum, type_: GLenum, bufSize: GLsizei, table: *mut c_void) { _GetnColorTable(target, format, type_, bufSize, table) }
pub unsafe fn GetnCompressedTexImage(target: GLenum, lod: GLint, bufSize: GLsizei, pixels: *mut c_void) { _GetnCompressedTexImage(target, lod, bufSize, pixels) }
pub unsafe fn GetnConvolutionFilter(target: GLenum, format: GLenum, type_: GLenum, bufSize: GLsizei, image: *mut c_void) { _GetnConvolutionFilter(target, format, type_, bufSize, image) }
pub unsafe fn GetnHistogram(target: GLenum, reset: GLboolean, format: GLenum, type_: GLenum, bufSize: GLsizei, values: *mut c_void) { _GetnHistogram(target, reset, format, type_, bufSize, values) }
pub unsafe fn GetnMapdv(target: GLenum, query: GLenum, bufSize: GLsizei, v: *mut GLdouble) { _GetnMapdv(target, query, bufSize, v) }
pub unsafe fn GetnMapfv(target: GLenum, query: GLenum, bufSize: GLsizei, v: *mut GLfloat) { _GetnMapfv(target, query, bufSize, v) }
pub unsafe fn GetnMapiv(target: GLenum, query: GLenum, bufSize: GLsizei, v: *mut GLint) { _GetnMapiv(target, query, bufSize, v) }
pub unsafe fn GetnMinmax(target: GLenum, reset: GLboolean, format: GLenum, type_: GLenum, bufSize: GLsizei, values: *mut c_void) { _GetnMinmax(target, reset, format, type_, bufSize, values) }
pub unsafe fn GetnPixelMapfv(map: GLenum, bufSize: GLsizei, values: *mut GLfloat) { _GetnPixelMapfv(map, bufSize, values) }
pub unsafe fn GetnPixelMapuiv(map: GLenum, bufSize: GLsizei, values: *mut GLuint) { _GetnPixelMapuiv(map, bufSize, values) }
pub unsafe fn GetnPixelMapusv(map: GLenum, bufSize: GLsizei, values: *mut GLushort) { _GetnPixelMapusv(map, bufSize, values) }
pub unsafe fn GetnPolygonStipple(bufSize: GLsizei, pattern: *mut GLubyte) { _GetnPolygonStipple(bufSize, pattern) }
pub unsafe fn GetnSeparableFilter(target: GLenum, format: GLenum, type_: GLenum, rowBufSize: GLsizei, row: *mut c_void, columnBufSize: GLsizei, column: *mut c_void, span: *mut c_void) { _GetnSeparableFilter(target, format, type_, rowBufSize, row, columnBufSize, column, span) }
pub unsafe fn GetnTexImage(target: GLenum, level: GLint, format: GLenum, type_: GLenum, bufSize: GLsizei, pixels: *mut c_void) { _GetnTexImage(target, level, format, type_, bufSize, pixels) }
pub unsafe fn GetnUniformdv(program: GLuint, location: GLint, bufSize: GLsizei, params: *mut GLdouble) { _GetnUniformdv(program, location, bufSize, params) }
pub unsafe fn GetnUniformfv(program: GLuint, location: GLint, bufSize: GLsizei, params: *mut GLfloat) { _GetnUniformfv(program, location, bufSize, params) }
pub unsafe fn GetnUniformiv(program: GLuint, location: GLint, bufSize: GLsizei, params: *mut GLint) { _GetnUniformiv(program, location, bufSize, params) }
pub unsafe fn GetnUniformuiv(program: GLuint, location: GLint, bufSize: GLsizei, params: *mut GLuint) { _GetnUniformuiv(program, location, bufSize, params) }
pub unsafe fn Hint(target: GLenum, mode: GLenum) { _Hint(target, mode) }
pub unsafe fn InvalidateBufferData(buffer: GLuint) { _InvalidateBufferData(buffer) }
pub unsafe fn InvalidateBufferSubData(buffer: GLuint, offset: GLintptr, length: GLsizeiptr) { _InvalidateBufferSubData(buffer, offset, length) }
pub unsafe fn InvalidateFramebuffer(target: GLenum, numAttachments: GLsizei, attachments: *const GLenum) { _InvalidateFramebuffer(target, numAttachments, attachments) }
pub unsafe fn InvalidateNamedFramebufferData(framebuffer: GLuint, numAttachments: GLsizei, attachments: *const GLenum) { _InvalidateNamedFramebufferData(framebuffer, numAttachments, attachments) }
pub unsafe fn InvalidateNamedFramebufferSubData(framebuffer: GLuint, numAttachments: GLsizei, attachments: *const GLenum, x: GLint, y: GLint, width: GLsizei, height: GLsizei) { _InvalidateNamedFramebufferSubData(framebuffer, numAttachments, attachments, x, y, width, height) }
pub unsafe fn InvalidateSubFramebuffer(target: GLenum, numAttachments: GLsizei, attachments: *const GLenum, x: GLint, y: GLint, width: GLsizei, height: GLsizei) { _InvalidateSubFramebuffer(target, numAttachments, attachments, x, y, width, height) }
pub unsafe fn InvalidateTexImage(texture: GLuint, level: GLint) { _InvalidateTexImage(texture, level) }
pub unsafe fn InvalidateTexSubImage(texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei) { _InvalidateTexSubImage(texture, level, xoffset, yoffset, zoffset, width, height, depth) }
pub unsafe fn IsBuffer(buffer: GLuint) -> GLboolean { _IsBuffer(buffer) }
pub unsafe fn IsEnabled(cap: GLenum) -> GLboolean { _IsEnabled(cap) }
pub unsafe fn IsEnabledi(target: GLenum, index: GLuint) -> GLboolean { _IsEnabledi(target, index) }
pub unsafe fn IsFramebuffer(framebuffer: GLuint) -> GLboolean { _IsFramebuffer(framebuffer) }
pub unsafe fn IsProgram(program: GLuint) -> GLboolean { _IsProgram(program) }
pub unsafe fn IsProgramPipeline(pipeline: GLuint) -> GLboolean { _IsProgramPipeline(pipeline) }
pub unsafe fn IsQuery(id: GLuint) -> GLboolean { _IsQuery(id) }
pub unsafe fn IsRenderbuffer(renderbuffer: GLuint) -> GLboolean { _IsRenderbuffer(renderbuffer) }
pub unsafe fn IsSampler(sampler: GLuint) -> GLboolean { _IsSampler(sampler) }
pub unsafe fn IsShader(shader: GLuint) -> GLboolean { _IsShader(shader) }
pub unsafe fn IsSync(sync: GLsync) -> GLboolean { _IsSync(sync) }
pub unsafe fn IsTexture(texture: GLuint) -> GLboolean { _IsTexture(texture) }
pub unsafe fn IsTransformFeedback(id: GLuint) -> GLboolean { _IsTransformFeedback(id) }
pub unsafe fn IsVertexArray(array: GLuint) -> GLboolean { _IsVertexArray(array) }
pub unsafe fn LineWidth(width: GLfloat) { _LineWidth(width) }
pub unsafe fn LinkProgram(program: GLuint) { _LinkProgram(program) }
pub unsafe fn LogicOp(opcode: GLenum) { _LogicOp(opcode) }
pub unsafe fn MapBuffer(target: GLenum, access: GLenum) -> *mut c_void { _MapBuffer(target, access) }
pub unsafe fn MapBufferRange(target: GLenum, offset: GLintptr, length: GLsizeiptr, access: GLbitfield) -> *mut c_void { _MapBufferRange(target, offset, length, access) }
pub unsafe fn MapNamedBuffer(buffer: GLuint, access: GLenum) -> *mut c_void { _MapNamedBuffer(buffer, access) }
pub unsafe fn MapNamedBufferRange(buffer: GLuint, offset: GLintptr, length: GLsizeiptr, access: GLbitfield) -> *mut c_void { _MapNamedBufferRange(buffer, offset, length, access) }
pub unsafe fn MemoryBarrier(barriers: GLbitfield) { _MemoryBarrier(barriers) }
pub unsafe fn MemoryBarrierByRegion(barriers: GLbitfield) { _MemoryBarrierByRegion(barriers) }
pub unsafe fn MinSampleShading(value: GLfloat) { _MinSampleShading(value) }
pub unsafe fn MultiDrawArrays(mode: GLenum, first: *const GLint, count: *const GLsizei, drawcount: GLsizei) { _MultiDrawArrays(mode, first, count, drawcount) }
pub unsafe fn MultiDrawArraysIndirect(mode: GLenum, indirect: *const c_void, drawcount: GLsizei, stride: GLsizei) { _MultiDrawArraysIndirect(mode, indirect, drawcount, stride) }
pub unsafe fn MultiDrawArraysIndirectCount(mode: GLenum, indirect: *const c_void, drawcount: GLintptr, maxdrawcount: GLsizei, stride: GLsizei) { _MultiDrawArraysIndirectCount(mode, indirect, drawcount, maxdrawcount, stride) }
pub unsafe fn MultiDrawElements(mode: GLenum, count: *const GLsizei, type_: GLenum, indices: *const *const c_void, drawcount: GLsizei) { _MultiDrawElements(mode, count, type_, indices, drawcount) }
pub unsafe fn MultiDrawElementsBaseVertex(mode: GLenum, count: *const GLsizei, type_: GLenum, indices: *const *const c_void, drawcount: GLsizei, basevertex: *const GLint) { _MultiDrawElementsBaseVertex(mode, count, type_, indices, drawcount, basevertex) }
pub unsafe fn MultiDrawElementsIndirect(mode: GLenum, type_: GLenum, indirect: *const c_void, drawcount: GLsizei, stride: GLsizei) { _MultiDrawElementsIndirect(mode, type_, indirect, drawcount, stride) }
pub unsafe fn MultiDrawElementsIndirectCount(mode: GLenum, type_: GLenum, indirect: *const c_void, drawcount: GLintptr, maxdrawcount: GLsizei, stride: GLsizei) { _MultiDrawElementsIndirectCount(mode, type_, indirect, drawcount, maxdrawcount, stride) }
pub unsafe fn MultiTexCoordP1ui(texture: GLenum, type_: GLenum, coords: GLuint) { _MultiTexCoordP1ui(texture, type_, coords) }
pub unsafe fn MultiTexCoordP1uiv(texture: GLenum, type_: GLenum, coords: *const GLuint) { _MultiTexCoordP1uiv(texture, type_, coords) }
pub unsafe fn MultiTexCoordP2ui(texture: GLenum, type_: GLenum, coords: GLuint) { _MultiTexCoordP2ui(texture, type_, coords) }
pub unsafe fn MultiTexCoordP2uiv(texture: GLenum, type_: GLenum, coords: *const GLuint) { _MultiTexCoordP2uiv(texture, type_, coords) }
pub unsafe fn MultiTexCoordP3ui(texture: GLenum, type_: GLenum, coords: GLuint) { _MultiTexCoordP3ui(texture, type_, coords) }
pub unsafe fn MultiTexCoordP3uiv(texture: GLenum, type_: GLenum, coords: *const GLuint) { _MultiTexCoordP3uiv(texture, type_, coords) }
pub unsafe fn MultiTexCoordP4ui(texture: GLenum, type_: GLenum, coords: GLuint) { _MultiTexCoordP4ui(texture, type_, coords) }
pub unsafe fn MultiTexCoordP4uiv(texture: GLenum, type_: GLenum, coords: *const GLuint) { _MultiTexCoordP4uiv(texture, type_, coords) }
pub unsafe fn NamedBufferData(buffer: GLuint, size: GLsizeiptr, data: *const c_void, usage: GLenum) { _NamedBufferData(buffer, size, data, usage) }
pub unsafe fn NamedBufferStorage(buffer: GLuint, size: GLsizeiptr, data: *const c_void, flags: GLbitfield) { _NamedBufferStorage(buffer, size, data, flags) }
pub unsafe fn NamedBufferSubData(buffer: GLuint, offset: GLintptr, size: GLsizeiptr, data: *const c_void) { _NamedBufferSubData(buffer, offset, size, data) }
pub unsafe fn NamedFramebufferDrawBuffer(framebuffer: GLuint, buf: GLenum) { _NamedFramebufferDrawBuffer(framebuffer, buf) }
pub unsafe fn NamedFramebufferDrawBuffers(framebuffer: GLuint, n: GLsizei, bufs: *const GLenum) { _NamedFramebufferDrawBuffers(framebuffer, n, bufs) }
pub unsafe fn NamedFramebufferParameteri(framebuffer: GLuint, pname: GLenum, param: GLint) { _NamedFramebufferParameteri(framebuffer, pname, param) }
pub unsafe fn NamedFramebufferReadBuffer(framebuffer: GLuint, src: GLenum) { _NamedFramebufferReadBuffer(framebuffer, src) }
pub unsafe fn NamedFramebufferRenderbuffer(framebuffer: GLuint, attachment: GLenum, renderbuffertarget: GLenum, renderbuffer: GLuint) { _NamedFramebufferRenderbuffer(framebuffer, attachment, renderbuffertarget, renderbuffer) }
pub unsafe fn NamedFramebufferTexture(framebuffer: GLuint, attachment: GLenum, texture: GLuint, level: GLint) { _NamedFramebufferTexture(framebuffer, attachment, texture, level) }
pub unsafe fn NamedFramebufferTextureLayer(framebuffer: GLuint, attachment: GLenum, texture: GLuint, level: GLint, layer: GLint) { _NamedFramebufferTextureLayer(framebuffer, attachment, texture, level, layer) }
pub unsafe fn NamedRenderbufferStorage(renderbuffer: GLuint, internalformat: GLenum, width: GLsizei, height: GLsizei) { _NamedRenderbufferStorage(renderbuffer, internalformat, width, height) }
pub unsafe fn NamedRenderbufferStorageMultisample(renderbuffer: GLuint, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei) { _NamedRenderbufferStorageMultisample(renderbuffer, samples, internalformat, width, height) }
pub unsafe fn NormalP3ui(type_: GLenum, coords: GLuint) { _NormalP3ui(type_, coords) }
pub unsafe fn NormalP3uiv(type_: GLenum, coords: *const GLuint) { _NormalP3uiv(type_, coords) }
pub unsafe fn ObjectLabel(identifier: GLenum, name: GLuint, length: GLsizei, label: *const GLchar) { _ObjectLabel(identifier, name, length, label) }
pub unsafe fn ObjectPtrLabel(ptr: *const c_void, length: GLsizei, label: *const GLchar) { _ObjectPtrLabel(ptr, length, label) }
pub unsafe fn PatchParameterfv(pname: GLenum, values: *const GLfloat) { _PatchParameterfv(pname, values) }
pub unsafe fn PatchParameteri(pname: GLenum, value: GLint) { _PatchParameteri(pname, value) }
pub unsafe fn PauseTransformFeedback() { _PauseTransformFeedback() }
pub unsafe fn PixelStoref(pname: GLenum, param: GLfloat) { _PixelStoref(pname, param) }
pub unsafe fn PixelStorei(pname: GLenum, param: GLint) { _PixelStorei(pname, param) }
pub unsafe fn PointParameterf(pname: GLenum, param: GLfloat) { _PointParameterf(pname, param) }
pub unsafe fn PointParameterfv(pname: GLenum, params: *const GLfloat) { _PointParameterfv(pname, params) }
pub unsafe fn PointParameteri(pname: GLenum, param: GLint) { _PointParameteri(pname, param) }
pub unsafe fn PointParameteriv(pname: GLenum, params: *const GLint) { _PointParameteriv(pname, params) }
pub unsafe fn PointSize(size: GLfloat) { _PointSize(size) }
pub unsafe fn PolygonMode(face: GLenum, mode: GLenum) { _PolygonMode(face, mode) }
pub unsafe fn PolygonOffset(factor: GLfloat, units: GLfloat) { _PolygonOffset(factor, units) }
pub unsafe fn PolygonOffsetClamp(factor: GLfloat, units: GLfloat, clamp: GLfloat) { _PolygonOffsetClamp(factor, units, clamp) }
pub unsafe fn PopDebugGroup() { _PopDebugGroup() }
pub unsafe fn PrimitiveRestartIndex(index: GLuint) { _PrimitiveRestartIndex(index) }
pub unsafe fn ProgramBinary(program: GLuint, binaryFormat: GLenum, binary: *const c_void, length: GLsizei) { _ProgramBinary(program, binaryFormat, binary, length) }
pub unsafe fn ProgramParameteri(program: GLuint, pname: GLenum, value: GLint) { _ProgramParameteri(program, pname, value) }
pub unsafe fn ProgramUniform1d(program: GLuint, location: GLint, v0: GLdouble) { _ProgramUniform1d(program, location, v0) }
pub unsafe fn ProgramUniform1dv(program: GLuint, location: GLint, count: GLsizei, value: *const GLdouble) { _ProgramUniform1dv(program, location, count, value) }
pub unsafe fn ProgramUniform1f(program: GLuint, location: GLint, v0: GLfloat) { _ProgramUniform1f(program, location, v0) }
pub unsafe fn ProgramUniform1fv(program: GLuint, location: GLint, count: GLsizei, value: *const GLfloat) { _ProgramUniform1fv(program, location, count, value) }
pub unsafe fn ProgramUniform1i(program: GLuint, location: GLint, v0: GLint) { _ProgramUniform1i(program, location, v0) }
pub unsafe fn ProgramUniform1iv(program: GLuint, location: GLint, count: GLsizei, value: *const GLint) { _ProgramUniform1iv(program, location, count, value) }
pub unsafe fn ProgramUniform1ui(program: GLuint, location: GLint, v0: GLuint) { _ProgramUniform1ui(program, location, v0) }
pub unsafe fn ProgramUniform1uiv(program: GLuint, location: GLint, count: GLsizei, value: *const GLuint) { _ProgramUniform1uiv(program, location, count, value) }
pub unsafe fn ProgramUniform2d(program: GLuint, location: GLint, v0: GLdouble, v1: GLdouble) { _ProgramUniform2d(program, location, v0, v1) }
pub unsafe fn ProgramUniform2dv(program: GLuint, location: GLint, count: GLsizei, value: *const GLdouble) { _ProgramUniform2dv(program, location, count, value) }
pub unsafe fn ProgramUniform2f(program: GLuint, location: GLint, v0: GLfloat, v1: GLfloat) { _ProgramUniform2f(program, location, v0, v1) }
pub unsafe fn ProgramUniform2fv(program: GLuint, location: GLint, count: GLsizei, value: *const GLfloat) { _ProgramUniform2fv(program, location, count, value) }
pub unsafe fn ProgramUniform2i(program: GLuint, location: GLint, v0: GLint, v1: GLint) { _ProgramUniform2i(program, location, v0, v1) }
pub unsafe fn ProgramUniform2iv(program: GLuint, location: GLint, count: GLsizei, value: *const GLint) { _ProgramUniform2iv(program, location, count, value) }
pub unsafe fn ProgramUniform2ui(program: GLuint, location: GLint, v0: GLuint, v1: GLuint) { _ProgramUniform2ui(program, location, v0, v1) }
pub unsafe fn ProgramUniform2uiv(program: GLuint, location: GLint, count: GLsizei, value: *const GLuint) { _ProgramUniform2uiv(program, location, count, value) }
pub unsafe fn ProgramUniform3d(program: GLuint, location: GLint, v0: GLdouble, v1: GLdouble, v2: GLdouble) { _ProgramUniform3d(program, location, v0, v1, v2) }
pub unsafe fn ProgramUniform3dv(program: GLuint, location: GLint, count: GLsizei, value: *const GLdouble) { _ProgramUniform3dv(program, location, count, value) }
pub unsafe fn ProgramUniform3f(program: GLuint, location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat) { _ProgramUniform3f(program, location, v0, v1, v2) }
pub unsafe fn ProgramUniform3fv(program: GLuint, location: GLint, count: GLsizei, value: *const GLfloat) { _ProgramUniform3fv(program, location, count, value) }
pub unsafe fn ProgramUniform3i(program: GLuint, location: GLint, v0: GLint, v1: GLint, v2: GLint) { _ProgramUniform3i(program, location, v0, v1, v2) }
pub unsafe fn ProgramUniform3iv(program: GLuint, location: GLint, count: GLsizei, value: *const GLint) { _ProgramUniform3iv(program, location, count, value) }
pub unsafe fn ProgramUniform3ui(program: GLuint, location: GLint, v0: GLuint, v1: GLuint, v2: GLuint) { _ProgramUniform3ui(program, location, v0, v1, v2) }
pub unsafe fn ProgramUniform3uiv(program: GLuint, location: GLint, count: GLsizei, value: *const GLuint) { _ProgramUniform3uiv(program, location, count, value) }
pub unsafe fn ProgramUniform4d(program: GLuint, location: GLint, v0: GLdouble, v1: GLdouble, v2: GLdouble, v3: GLdouble) { _ProgramUniform4d(program, location, v0, v1, v2, v3) }
pub unsafe fn ProgramUniform4dv(program: GLuint, location: GLint, count: GLsizei, value: *const GLdouble) { _ProgramUniform4dv(program, location, count, value) }
pub unsafe fn ProgramUniform4f(program: GLuint, location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat, v3: GLfloat) { _ProgramUniform4f(program, location, v0, v1, v2, v3) }
pub unsafe fn ProgramUniform4fv(program: GLuint, location: GLint, count: GLsizei, value: *const GLfloat) { _ProgramUniform4fv(program, location, count, value) }
pub unsafe fn ProgramUniform4i(program: GLuint, location: GLint, v0: GLint, v1: GLint, v2: GLint, v3: GLint) { _ProgramUniform4i(program, location, v0, v1, v2, v3) }
pub unsafe fn ProgramUniform4iv(program: GLuint, location: GLint, count: GLsizei, value: *const GLint) { _ProgramUniform4iv(program, location, count, value) }
pub unsafe fn ProgramUniform4ui(program: GLuint, location: GLint, v0: GLuint, v1: GLuint, v2: GLuint, v3: GLuint) { _ProgramUniform4ui(program, location, v0, v1, v2, v3) }
pub unsafe fn ProgramUniform4uiv(program: GLuint, location: GLint, count: GLsizei, value: *const GLuint) { _ProgramUniform4uiv(program, location, count, value) }
pub unsafe fn ProgramUniformMatrix2dv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) { _ProgramUniformMatrix2dv(program, location, count, transpose, value) }
pub unsafe fn ProgramUniformMatrix2fv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) { _ProgramUniformMatrix2fv(program, location, count, transpose, value) }
pub unsafe fn ProgramUniformMatrix2x3dv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) { _ProgramUniformMatrix2x3dv(program, location, count, transpose, value) }
pub unsafe fn ProgramUniformMatrix2x3fv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) { _ProgramUniformMatrix2x3fv(program, location, count, transpose, value) }
pub unsafe fn ProgramUniformMatrix2x4dv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) { _ProgramUniformMatrix2x4dv(program, location, count, transpose, value) }
pub unsafe fn ProgramUniformMatrix2x4fv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) { _ProgramUniformMatrix2x4fv(program, location, count, transpose, value) }
pub unsafe fn ProgramUniformMatrix3dv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) { _ProgramUniformMatrix3dv(program, location, count, transpose, value) }
pub unsafe fn ProgramUniformMatrix3fv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) { _ProgramUniformMatrix3fv(program, location, count, transpose, value) }
pub unsafe fn ProgramUniformMatrix3x2dv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) { _ProgramUniformMatrix3x2dv(program, location, count, transpose, value) }
pub unsafe fn ProgramUniformMatrix3x2fv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) { _ProgramUniformMatrix3x2fv(program, location, count, transpose, value) }
pub unsafe fn ProgramUniformMatrix3x4dv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) { _ProgramUniformMatrix3x4dv(program, location, count, transpose, value) }
pub unsafe fn ProgramUniformMatrix3x4fv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) { _ProgramUniformMatrix3x4fv(program, location, count, transpose, value) }
pub unsafe fn ProgramUniformMatrix4dv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) { _ProgramUniformMatrix4dv(program, location, count, transpose, value) }
pub unsafe fn ProgramUniformMatrix4fv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) { _ProgramUniformMatrix4fv(program, location, count, transpose, value) }
pub unsafe fn ProgramUniformMatrix4x2dv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) { _ProgramUniformMatrix4x2dv(program, location, count, transpose, value) }
pub unsafe fn ProgramUniformMatrix4x2fv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) { _ProgramUniformMatrix4x2fv(program, location, count, transpose, value) }
pub unsafe fn ProgramUniformMatrix4x3dv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) { _ProgramUniformMatrix4x3dv(program, location, count, transpose, value) }
pub unsafe fn ProgramUniformMatrix4x3fv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) { _ProgramUniformMatrix4x3fv(program, location, count, transpose, value) }
pub unsafe fn ProvokingVertex(mode: GLenum) { _ProvokingVertex(mode) }
pub unsafe fn PushDebugGroup(source: GLenum, id: GLuint, length: GLsizei, message: *const GLchar) { _PushDebugGroup(source, id, length, message) }
pub unsafe fn QueryCounter(id: GLuint, target: GLenum) { _QueryCounter(id, target) }
pub unsafe fn ReadBuffer(src: GLenum) { _ReadBuffer(src) }
pub unsafe fn ReadPixels(x: GLint, y: GLint, width: GLsizei, height: GLsizei, format: GLenum, type_: GLenum, pixels: *mut c_void) { _ReadPixels(x, y, width, height, format, type_, pixels) }
pub unsafe fn ReadnPixels(x: GLint, y: GLint, width: GLsizei, height: GLsizei, format: GLenum, type_: GLenum, bufSize: GLsizei, data: *mut c_void) { _ReadnPixels(x, y, width, height, format, type_, bufSize, data) }
pub unsafe fn ReleaseShaderCompiler() { _ReleaseShaderCompiler() }
pub unsafe fn RenderbufferStorage(target: GLenum, internalformat: GLenum, width: GLsizei, height: GLsizei) { _RenderbufferStorage(target, internalformat, width, height) }
pub unsafe fn RenderbufferStorageMultisample(target: GLenum, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei) { _RenderbufferStorageMultisample(target, samples, internalformat, width, height) }
pub unsafe fn ResumeTransformFeedback() { _ResumeTransformFeedback() }
pub unsafe fn SampleCoverage(value: GLfloat, invert: GLboolean) { _SampleCoverage(value, invert) }
pub unsafe fn SampleMaski(maskNumber: GLuint, mask: GLbitfield) { _SampleMaski(maskNumber, mask) }
pub unsafe fn SamplerParameterIiv(sampler: GLuint, pname: GLenum, param: *const GLint) { _SamplerParameterIiv(sampler, pname, param) }
pub unsafe fn SamplerParameterIuiv(sampler: GLuint, pname: GLenum, param: *const GLuint) { _SamplerParameterIuiv(sampler, pname, param) }
pub unsafe fn SamplerParameterf(sampler: GLuint, pname: GLenum, param: GLfloat) { _SamplerParameterf(sampler, pname, param) }
pub unsafe fn SamplerParameterfv(sampler: GLuint, pname: GLenum, param: *const GLfloat) { _SamplerParameterfv(sampler, pname, param) }
pub unsafe fn SamplerParameteri(sampler: GLuint, pname: GLenum, param: GLint) { _SamplerParameteri(sampler, pname, param) }
pub unsafe fn SamplerParameteriv(sampler: GLuint, pname: GLenum, param: *const GLint) { _SamplerParameteriv(sampler, pname, param) }
pub unsafe fn Scissor(x: GLint, y: GLint, width: GLsizei, height: GLsizei) { _Scissor(x, y, width, height) }
pub unsafe fn ScissorArrayv(first: GLuint, count: GLsizei, v: *const GLint) { _ScissorArrayv(first, count, v) }
pub unsafe fn ScissorIndexed(index: GLuint, left: GLint, bottom: GLint, width: GLsizei, height: GLsizei) { _ScissorIndexed(index, left, bottom, width, height) }
pub unsafe fn ScissorIndexedv(index: GLuint, v: *const GLint) { _ScissorIndexedv(index, v) }
pub unsafe fn SecondaryColorP3ui(type_: GLenum, color: GLuint) { _SecondaryColorP3ui(type_, color) }
pub unsafe fn SecondaryColorP3uiv(type_: GLenum, color: *const GLuint) { _SecondaryColorP3uiv(type_, color) }
pub unsafe fn ShaderBinary(count: GLsizei, shaders: *const GLuint, binaryformat: GLenum, binary: *const c_void, length: GLsizei) { _ShaderBinary(count, shaders, binaryformat, binary, length) }
pub unsafe fn ShaderSource(shader: GLuint, count: GLsizei, string: *const *const GLchar, length: *const GLint) { _ShaderSource(shader, count, string, length) }
pub unsafe fn ShaderStorageBlockBinding(program: GLuint, storageBlockIndex: GLuint, storageBlockBinding: GLuint) { _ShaderStorageBlockBinding(program, storageBlockIndex, storageBlockBinding) }
pub unsafe fn SpecializeShader(shader: GLuint, pEntryPoint: *const GLchar, numSpecializationConstants: GLuint, pConstantIndex: *const GLuint, pConstantValue: *const GLuint) { _SpecializeShader(shader, pEntryPoint, numSpecializationConstants, pConstantIndex, pConstantValue) }
pub unsafe fn StencilFunc(func: GLenum, ref_: GLint, mask: GLuint) { _StencilFunc(func, ref_, mask) }
pub unsafe fn StencilFuncSeparate(face: GLenum, func: GLenum, ref_: GLint, mask: GLuint) { _StencilFuncSeparate(face, func, ref_, mask) }
pub unsafe fn StencilMask(mask: GLuint) { _StencilMask(mask) }
pub unsafe fn StencilMaskSeparate(face: GLenum, mask: GLuint) { _StencilMaskSeparate(face, mask) }
pub unsafe fn StencilOp(fail: GLenum, zfail: GLenum, zpass: GLenum) { _StencilOp(fail, zfail, zpass) }
pub unsafe fn StencilOpSeparate(face: GLenum, sfail: GLenum, dpfail: GLenum, dppass: GLenum) { _StencilOpSeparate(face, sfail, dpfail, dppass) }
pub unsafe fn TexBuffer(target: GLenum, internalformat: GLenum, buffer: GLuint) { _TexBuffer(target, internalformat, buffer) }
pub unsafe fn TexBufferRange(target: GLenum, internalformat: GLenum, buffer: GLuint, offset: GLintptr, size: GLsizeiptr) { _TexBufferRange(target, internalformat, buffer, offset, size) }
pub unsafe fn TexCoordP1ui(type_: GLenum, coords: GLuint) { _TexCoordP1ui(type_, coords) }
pub unsafe fn TexCoordP1uiv(type_: GLenum, coords: *const GLuint) { _TexCoordP1uiv(type_, coords) }
pub unsafe fn TexCoordP2ui(type_: GLenum, coords: GLuint) { _TexCoordP2ui(type_, coords) }
pub unsafe fn TexCoordP2uiv(type_: GLenum, coords: *const GLuint) { _TexCoordP2uiv(type_, coords) }
pub unsafe fn TexCoordP3ui(type_: GLenum, coords: GLuint) { _TexCoordP3ui(type_, coords) }
pub unsafe fn TexCoordP3uiv(type_: GLenum, coords: *const GLuint) { _TexCoordP3uiv(type_, coords) }
pub unsafe fn TexCoordP4ui(type_: GLenum, coords: GLuint) { _TexCoordP4ui(type_, coords) }
pub unsafe fn TexCoordP4uiv(type_: GLenum, coords: *const GLuint) { _TexCoordP4uiv(type_, coords) }
pub unsafe fn TexImage1D(target: GLenum, level: GLint, internalformat: GLint, width: GLsizei, border: GLint, format: GLenum, type_: GLenum, pixels: *const c_void) { _TexImage1D(target, level, internalformat, width, border, format, type_, pixels) }
pub unsafe fn TexImage2D(target: GLenum, level: GLint, internalformat: GLint, width: GLsizei, height: GLsizei, border: GLint, format: GLenum, type_: GLenum, pixels: *const c_void) { _TexImage2D(target, level, internalformat, width, height, border, format, type_, pixels) }
pub unsafe fn TexImage2DMultisample(target: GLenum, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, fixedsamplelocations: GLboolean) { _TexImage2DMultisample(target, samples, internalformat, width, height, fixedsamplelocations) }
pub unsafe fn TexImage3D(target: GLenum, level: GLint, internalformat: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, border: GLint, format: GLenum, type_: GLenum, pixels: *const c_void) { _TexImage3D(target, level, internalformat, width, height, depth, border, format, type_, pixels) }
pub unsafe fn TexImage3DMultisample(target: GLenum, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei, fixedsamplelocations: GLboolean) { _TexImage3DMultisample(target, samples, internalformat, width, height, depth, fixedsamplelocations) }
pub unsafe fn TexParameterIiv(target: GLenum, pname: GLenum, params: *const GLint) { _TexParameterIiv(target, pname, params) }
pub unsafe fn TexParameterIuiv(target: GLenum, pname: GLenum, params: *const GLuint) { _TexParameterIuiv(target, pname, params) }
pub unsafe fn TexParameterf(target: GLenum, pname: GLenum, param: GLfloat) { _TexParameterf(target, pname, param) }
pub unsafe fn TexParameterfv(target: GLenum, pname: GLenum, params: *const GLfloat) { _TexParameterfv(target, pname, params) }
pub unsafe fn TexParameteri(target: GLenum, pname: GLenum, param: GLint) { _TexParameteri(target, pname, param) }
pub unsafe fn TexParameteriv(target: GLenum, pname: GLenum, params: *const GLint) { _TexParameteriv(target, pname, params) }
pub unsafe fn TexStorage1D(target: GLenum, levels: GLsizei, internalformat: GLenum, width: GLsizei) { _TexStorage1D(target, levels, internalformat, width) }
pub unsafe fn TexStorage2D(target: GLenum, levels: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei) { _TexStorage2D(target, levels, internalformat, width, height) }
pub unsafe fn TexStorage2DMultisample(target: GLenum, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, fixedsamplelocations: GLboolean) { _TexStorage2DMultisample(target, samples, internalformat, width, height, fixedsamplelocations) }
pub unsafe fn TexStorage3D(target: GLenum, levels: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei) { _TexStorage3D(target, levels, internalformat, width, height, depth) }
pub unsafe fn TexStorage3DMultisample(target: GLenum, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei, fixedsamplelocations: GLboolean) { _TexStorage3DMultisample(target, samples, internalformat, width, height, depth, fixedsamplelocations) }
pub unsafe fn TexSubImage1D(target: GLenum, level: GLint, xoffset: GLint, width: GLsizei, format: GLenum, type_: GLenum, pixels: *const c_void) { _TexSubImage1D(target, level, xoffset, width, format, type_, pixels) }
pub unsafe fn TexSubImage2D(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: GLenum, type_: GLenum, pixels: *const c_void) { _TexSubImage2D(target, level, xoffset, yoffset, width, height, format, type_, pixels) }
pub unsafe fn TexSubImage3D(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: GLenum, type_: GLenum, pixels: *const c_void) { _TexSubImage3D(target, level, xoffset, yoffset, zoffset, width, height, depth, format, type_, pixels) }
pub unsafe fn TextureBarrier() { _TextureBarrier() }
pub unsafe fn TextureBuffer(texture: GLuint, internalformat: GLenum, buffer: GLuint) { _TextureBuffer(texture, internalformat, buffer) }
pub unsafe fn TextureBufferRange(texture: GLuint, internalformat: GLenum, buffer: GLuint, offset: GLintptr, size: GLsizeiptr) { _TextureBufferRange(texture, internalformat, buffer, offset, size) }
pub unsafe fn TextureParameterIiv(texture: GLuint, pname: GLenum, params: *const GLint) { _TextureParameterIiv(texture, pname, params) }
pub unsafe fn TextureParameterIuiv(texture: GLuint, pname: GLenum, params: *const GLuint) { _TextureParameterIuiv(texture, pname, params) }
pub unsafe fn TextureParameterf(texture: GLuint, pname: GLenum, param: GLfloat) { _TextureParameterf(texture, pname, param) }
pub unsafe fn TextureParameterfv(texture: GLuint, pname: GLenum, param: *const GLfloat) { _TextureParameterfv(texture, pname, param) }
pub unsafe fn TextureParameteri(texture: GLuint, pname: GLenum, param: GLint) { _TextureParameteri(texture, pname, param) }
pub unsafe fn TextureParameteriv(texture: GLuint, pname: GLenum, param: *const GLint) { _TextureParameteriv(texture, pname, param) }
pub unsafe fn TextureStorage1D(texture: GLuint, levels: GLsizei, internalformat: GLenum, width: GLsizei) { _TextureStorage1D(texture, levels, internalformat, width) }
pub unsafe fn TextureStorage2D(texture: GLuint, levels: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei) { _TextureStorage2D(texture, levels, internalformat, width, height) }
pub unsafe fn TextureStorage2DMultisample(texture: GLuint, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, fixedsamplelocations: GLboolean) { _TextureStorage2DMultisample(texture, samples, internalformat, width, height, fixedsamplelocations) }
pub unsafe fn TextureStorage3D(texture: GLuint, levels: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei) { _TextureStorage3D(texture, levels, internalformat, width, height, depth) }
pub unsafe fn TextureStorage3DMultisample(texture: GLuint, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei, fixedsamplelocations: GLboolean) { _TextureStorage3DMultisample(texture, samples, internalformat, width, height, depth, fixedsamplelocations) }
pub unsafe fn TextureSubImage1D(texture: GLuint, level: GLint, xoffset: GLint, width: GLsizei, format: GLenum, type_: GLenum, pixels: *const c_void) { _TextureSubImage1D(texture, level, xoffset, width, format, type_, pixels) }
pub unsafe fn TextureSubImage2D(texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: GLenum, type_: GLenum, pixels: *const c_void) { _TextureSubImage2D(texture, level, xoffset, yoffset, width, height, format, type_, pixels) }
pub unsafe fn TextureSubImage3D(texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: GLenum, type_: GLenum, pixels: *const c_void) { _TextureSubImage3D(texture, level, xoffset, yoffset, zoffset, width, height, depth, format, type_, pixels) }
pub unsafe fn TextureView(texture: GLuint, target: GLenum, origtexture: GLuint, internalformat: GLenum, minlevel: GLuint, numlevels: GLuint, minlayer: GLuint, numlayers: GLuint) { _TextureView(texture, target, origtexture, internalformat, minlevel, numlevels, minlayer, numlayers) }
pub unsafe fn TransformFeedbackBufferBase(xfb: GLuint, index: GLuint, buffer: GLuint) { _TransformFeedbackBufferBase(xfb, index, buffer) }
pub unsafe fn TransformFeedbackBufferRange(xfb: GLuint, index: GLuint, buffer: GLuint, offset: GLintptr, size: GLsizeiptr) { _TransformFeedbackBufferRange(xfb, index, buffer, offset, size) }
pub unsafe fn TransformFeedbackVaryings(program: GLuint, count: GLsizei, varyings: *const *const GLchar, bufferMode: GLenum) { _TransformFeedbackVaryings(program, count, varyings, bufferMode) }
pub unsafe fn Uniform1d(location: GLint, x: GLdouble) { _Uniform1d(location, x) }
pub unsafe fn Uniform1dv(location: GLint, count: GLsizei, value: *const GLdouble) { _Uniform1dv(location, count, value) }
pub unsafe fn Uniform1f(location: GLint, v0: GLfloat) { _Uniform1f(location, v0) }
pub unsafe fn Uniform1fv(location: GLint, count: GLsizei, value: *const GLfloat) { _Uniform1fv(location, count, value) }
pub unsafe fn Uniform1i(location: GLint, v0: GLint) { _Uniform1i(location, v0) }
pub unsafe fn Uniform1iv(location: GLint, count: GLsizei, value: *const GLint) { _Uniform1iv(location, count, value) }
pub unsafe fn Uniform1ui(location: GLint, v0: GLuint) { _Uniform1ui(location, v0) }
pub unsafe fn Uniform1uiv(location: GLint, count: GLsizei, value: *const GLuint) { _Uniform1uiv(location, count, value) }
pub unsafe fn Uniform2d(location: GLint, x: GLdouble, y: GLdouble) { _Uniform2d(location, x, y) }
pub unsafe fn Uniform2dv(location: GLint, count: GLsizei, value: *const GLdouble) { _Uniform2dv(location, count, value) }
pub unsafe fn Uniform2f(location: GLint, v0: GLfloat, v1: GLfloat) { _Uniform2f(location, v0, v1) }
pub unsafe fn Uniform2fv(location: GLint, count: GLsizei, value: *const GLfloat) { _Uniform2fv(location, count, value) }
pub unsafe fn Uniform2i(location: GLint, v0: GLint, v1: GLint) { _Uniform2i(location, v0, v1) }
pub unsafe fn Uniform2iv(location: GLint, count: GLsizei, value: *const GLint) { _Uniform2iv(location, count, value) }
pub unsafe fn Uniform2ui(location: GLint, v0: GLuint, v1: GLuint) { _Uniform2ui(location, v0, v1) }
pub unsafe fn Uniform2uiv(location: GLint, count: GLsizei, value: *const GLuint) { _Uniform2uiv(location, count, value) }
pub unsafe fn Uniform3d(location: GLint, x: GLdouble, y: GLdouble, z: GLdouble) { _Uniform3d(location, x, y, z) }
pub unsafe fn Uniform3dv(location: GLint, count: GLsizei, value: *const GLdouble) { _Uniform3dv(location, count, value) }
pub unsafe fn Uniform3f(location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat) { _Uniform3f(location, v0, v1, v2) }
pub unsafe fn Uniform3fv(location: GLint, count: GLsizei, value: *const GLfloat) { _Uniform3fv(location, count, value) }
pub unsafe fn Uniform3i(location: GLint, v0: GLint, v1: GLint, v2: GLint) { _Uniform3i(location, v0, v1, v2) }
pub unsafe fn Uniform3iv(location: GLint, count: GLsizei, value: *const GLint) { _Uniform3iv(location, count, value) }
pub unsafe fn Uniform3ui(location: GLint, v0: GLuint, v1: GLuint, v2: GLuint) { _Uniform3ui(location, v0, v1, v2) }
pub unsafe fn Uniform3uiv(location: GLint, count: GLsizei, value: *const GLuint) { _Uniform3uiv(location, count, value) }
pub unsafe fn Uniform4d(location: GLint, x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble) { _Uniform4d(location, x, y, z, w) }
pub unsafe fn Uniform4dv(location: GLint, count: GLsizei, value: *const GLdouble) { _Uniform4dv(location, count, value) }
pub unsafe fn Uniform4f(location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat, v3: GLfloat) { _Uniform4f(location, v0, v1, v2, v3) }
pub unsafe fn Uniform4fv(location: GLint, count: GLsizei, value: *const GLfloat) { _Uniform4fv(location, count, value) }
pub unsafe fn Uniform4i(location: GLint, v0: GLint, v1: GLint, v2: GLint, v3: GLint) { _Uniform4i(location, v0, v1, v2, v3) }
pub unsafe fn Uniform4iv(location: GLint, count: GLsizei, value: *const GLint) { _Uniform4iv(location, count, value) }
pub unsafe fn Uniform4ui(location: GLint, v0: GLuint, v1: GLuint, v2: GLuint, v3: GLuint) { _Uniform4ui(location, v0, v1, v2, v3) }
pub unsafe fn Uniform4uiv(location: GLint, count: GLsizei, value: *const GLuint) { _Uniform4uiv(location, count, value) }
pub unsafe fn UniformBlockBinding(program: GLuint, uniformBlockIndex: GLuint, uniformBlockBinding: GLuint) { _UniformBlockBinding(program, uniformBlockIndex, uniformBlockBinding) }
pub unsafe fn UniformMatrix2dv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) { _UniformMatrix2dv(location, count, transpose, value) }
pub unsafe fn UniformMatrix2fv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) { _UniformMatrix2fv(location, count, transpose, value) }
pub unsafe fn UniformMatrix2x3dv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) { _UniformMatrix2x3dv(location, count, transpose, value) }
pub unsafe fn UniformMatrix2x3fv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) { _UniformMatrix2x3fv(location, count, transpose, value) }
pub unsafe fn UniformMatrix2x4dv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) { _UniformMatrix2x4dv(location, count, transpose, value) }
pub unsafe fn UniformMatrix2x4fv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) { _UniformMatrix2x4fv(location, count, transpose, value) }
pub unsafe fn UniformMatrix3dv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) { _UniformMatrix3dv(location, count, transpose, value) }
pub unsafe fn UniformMatrix3fv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) { _UniformMatrix3fv(location, count, transpose, value) }
pub unsafe fn UniformMatrix3x2dv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) { _UniformMatrix3x2dv(location, count, transpose, value) }
pub unsafe fn UniformMatrix3x2fv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) { _UniformMatrix3x2fv(location, count, transpose, value) }
pub unsafe fn UniformMatrix3x4dv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) { _UniformMatrix3x4dv(location, count, transpose, value) }
pub unsafe fn UniformMatrix3x4fv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) { _UniformMatrix3x4fv(location, count, transpose, value) }
pub unsafe fn UniformMatrix4dv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) { _UniformMatrix4dv(location, count, transpose, value) }
pub unsafe fn UniformMatrix4fv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) { _UniformMatrix4fv(location, count, transpose, value) }
pub unsafe fn UniformMatrix4x2dv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) { _UniformMatrix4x2dv(location, count, transpose, value) }
pub unsafe fn UniformMatrix4x2fv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) { _UniformMatrix4x2fv(location, count, transpose, value) }
pub unsafe fn UniformMatrix4x3dv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) { _UniformMatrix4x3dv(location, count, transpose, value) }
pub unsafe fn UniformMatrix4x3fv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) { _UniformMatrix4x3fv(location, count, transpose, value) }
pub unsafe fn UniformSubroutinesuiv(shadertype: GLenum, count: GLsizei, indices: *const GLuint) { _UniformSubroutinesuiv(shadertype, count, indices) }
pub unsafe fn UnmapBuffer(target: GLenum) -> GLboolean { _UnmapBuffer(target) }
pub unsafe fn UnmapNamedBuffer(buffer: GLuint) -> GLboolean { _UnmapNamedBuffer(buffer) }
pub unsafe fn UseProgram(program: GLuint) { _UseProgram(program) }
pub unsafe fn UseProgramStages(pipeline: GLuint, stages: GLbitfield, program: GLuint) { _UseProgramStages(pipeline, stages, program) }
pub unsafe fn ValidateProgram(program: GLuint) { _ValidateProgram(program) }
pub unsafe fn ValidateProgramPipeline(pipeline: GLuint) { _ValidateProgramPipeline(pipeline) }
pub unsafe fn VertexArrayAttribBinding(vaobj: GLuint, attribindex: GLuint, bindingindex: GLuint) { _VertexArrayAttribBinding(vaobj, attribindex, bindingindex) }
pub unsafe fn VertexArrayAttribFormat(vaobj: GLuint, attribindex: GLuint, size: GLint, type_: GLenum, normalized: GLboolean, relativeoffset: GLuint) { _VertexArrayAttribFormat(vaobj, attribindex, size, type_, normalized, relativeoffset) }
pub unsafe fn VertexArrayAttribIFormat(vaobj: GLuint, attribindex: GLuint, size: GLint, type_: GLenum, relativeoffset: GLuint) { _VertexArrayAttribIFormat(vaobj, attribindex, size, type_, relativeoffset) }
pub unsafe fn VertexArrayAttribLFormat(vaobj: GLuint, attribindex: GLuint, size: GLint, type_: GLenum, relativeoffset: GLuint) { _VertexArrayAttribLFormat(vaobj, attribindex, size, type_, relativeoffset) }
pub unsafe fn VertexArrayBindingDivisor(vaobj: GLuint, bindingindex: GLuint, divisor: GLuint) { _VertexArrayBindingDivisor(vaobj, bindingindex, divisor) }
pub unsafe fn VertexArrayElementBuffer(vaobj: GLuint, buffer: GLuint) { _VertexArrayElementBuffer(vaobj, buffer) }
pub unsafe fn VertexArrayVertexBuffer(vaobj: GLuint, bindingindex: GLuint, buffer: GLuint, offset: GLintptr, stride: GLsizei) { _VertexArrayVertexBuffer(vaobj, bindingindex, buffer, offset, stride) }
pub unsafe fn VertexArrayVertexBuffers(vaobj: GLuint, first: GLuint, count: GLsizei, buffers: *const GLuint, offsets: *const GLintptr, strides: *const GLsizei) { _VertexArrayVertexBuffers(vaobj, first, count, buffers, offsets, strides) }
pub unsafe fn VertexAttrib1d(index: GLuint, x: GLdouble) { _VertexAttrib1d(index, x) }
pub unsafe fn VertexAttrib1dv(index: GLuint, v: *const GLdouble) { _VertexAttrib1dv(index, v) }
pub unsafe fn VertexAttrib1f(index: GLuint, x: GLfloat) { _VertexAttrib1f(index, x) }
pub unsafe fn VertexAttrib1fv(index: GLuint, v: *const GLfloat) { _VertexAttrib1fv(index, v) }
pub unsafe fn VertexAttrib1s(index: GLuint, x: GLshort) { _VertexAttrib1s(index, x) }
pub unsafe fn VertexAttrib1sv(index: GLuint, v: *const GLshort) { _VertexAttrib1sv(index, v) }
pub unsafe fn VertexAttrib2d(index: GLuint, x: GLdouble, y: GLdouble) { _VertexAttrib2d(index, x, y) }
pub unsafe fn VertexAttrib2dv(index: GLuint, v: *const GLdouble) { _VertexAttrib2dv(index, v) }
pub unsafe fn VertexAttrib2f(index: GLuint, x: GLfloat, y: GLfloat) { _VertexAttrib2f(index, x, y) }
pub unsafe fn VertexAttrib2fv(index: GLuint, v: *const GLfloat) { _VertexAttrib2fv(index, v) }
pub unsafe fn VertexAttrib2s(index: GLuint, x: GLshort, y: GLshort) { _VertexAttrib2s(index, x, y) }
pub unsafe fn VertexAttrib2sv(index: GLuint, v: *const GLshort) { _VertexAttrib2sv(index, v) }
pub unsafe fn VertexAttrib3d(index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble) { _VertexAttrib3d(index, x, y, z) }
pub unsafe fn VertexAttrib3dv(index: GLuint, v: *const GLdouble) { _VertexAttrib3dv(index, v) }
pub unsafe fn VertexAttrib3f(index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat) { _VertexAttrib3f(index, x, y, z) }
pub unsafe fn VertexAttrib3fv(index: GLuint, v: *const GLfloat) { _VertexAttrib3fv(index, v) }
pub unsafe fn VertexAttrib3s(index: GLuint, x: GLshort, y: GLshort, z: GLshort) { _VertexAttrib3s(index, x, y, z) }
pub unsafe fn VertexAttrib3sv(index: GLuint, v: *const GLshort) { _VertexAttrib3sv(index, v) }
pub unsafe fn VertexAttrib4Nbv(index: GLuint, v: *const GLbyte) { _VertexAttrib4Nbv(index, v) }
pub unsafe fn VertexAttrib4Niv(index: GLuint, v: *const GLint) { _VertexAttrib4Niv(index, v) }
pub unsafe fn VertexAttrib4Nsv(index: GLuint, v: *const GLshort) { _VertexAttrib4Nsv(index, v) }
pub unsafe fn VertexAttrib4Nub(index: GLuint, x: GLubyte, y: GLubyte, z: GLubyte, w: GLubyte) { _VertexAttrib4Nub(index, x, y, z, w) }
pub unsafe fn VertexAttrib4Nubv(index: GLuint, v: *const GLubyte) { _VertexAttrib4Nubv(index, v) }
pub unsafe fn VertexAttrib4Nuiv(index: GLuint, v: *const GLuint) { _VertexAttrib4Nuiv(index, v) }
pub unsafe fn VertexAttrib4Nusv(index: GLuint, v: *const GLushort) { _VertexAttrib4Nusv(index, v) }
pub unsafe fn VertexAttrib4bv(index: GLuint, v: *const GLbyte) { _VertexAttrib4bv(index, v) }
pub unsafe fn VertexAttrib4d(index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble) { _VertexAttrib4d(index, x, y, z, w) }
pub unsafe fn VertexAttrib4dv(index: GLuint, v: *const GLdouble) { _VertexAttrib4dv(index, v) }
pub unsafe fn VertexAttrib4f(index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat) { _VertexAttrib4f(index, x, y, z, w) }
pub unsafe fn VertexAttrib4fv(index: GLuint, v: *const GLfloat) { _VertexAttrib4fv(index, v) }
pub unsafe fn VertexAttrib4iv(index: GLuint, v: *const GLint) { _VertexAttrib4iv(index, v) }
pub unsafe fn VertexAttrib4s(index: GLuint, x: GLshort, y: GLshort, z: GLshort, w: GLshort) { _VertexAttrib4s(index, x, y, z, w) }
pub unsafe fn VertexAttrib4sv(index: GLuint, v: *const GLshort) { _VertexAttrib4sv(index, v) }
pub unsafe fn VertexAttrib4ubv(index: GLuint, v: *const GLubyte) { _VertexAttrib4ubv(index, v) }
pub unsafe fn VertexAttrib4uiv(index: GLuint, v: *const GLuint) { _VertexAttrib4uiv(index, v) }
pub unsafe fn VertexAttrib4usv(index: GLuint, v: *const GLushort) { _VertexAttrib4usv(index, v) }
pub unsafe fn VertexAttribBinding(attribindex: GLuint, bindingindex: GLuint) { _VertexAttribBinding(attribindex, bindingindex) }
pub unsafe fn VertexAttribDivisor(index: GLuint, divisor: GLuint) { _VertexAttribDivisor(index, divisor) }
pub unsafe fn VertexAttribFormat(attribindex: GLuint, size: GLint, type_: GLenum, normalized: GLboolean, relativeoffset: GLuint) { _VertexAttribFormat(attribindex, size, type_, normalized, relativeoffset) }
pub unsafe fn VertexAttribI1i(index: GLuint, x: GLint) { _VertexAttribI1i(index, x) }
pub unsafe fn VertexAttribI1iv(index: GLuint, v: *const GLint) { _VertexAttribI1iv(index, v) }
pub unsafe fn VertexAttribI1ui(index: GLuint, x: GLuint) { _VertexAttribI1ui(index, x) }
pub unsafe fn VertexAttribI1uiv(index: GLuint, v: *const GLuint) { _VertexAttribI1uiv(index, v) }
pub unsafe fn VertexAttribI2i(index: GLuint, x: GLint, y: GLint) { _VertexAttribI2i(index, x, y) }
pub unsafe fn VertexAttribI2iv(index: GLuint, v: *const GLint) { _VertexAttribI2iv(index, v) }
pub unsafe fn VertexAttribI2ui(index: GLuint, x: GLuint, y: GLuint) { _VertexAttribI2ui(index, x, y) }
pub unsafe fn VertexAttribI2uiv(index: GLuint, v: *const GLuint) { _VertexAttribI2uiv(index, v) }
pub unsafe fn VertexAttribI3i(index: GLuint, x: GLint, y: GLint, z: GLint) { _VertexAttribI3i(index, x, y, z) }
pub unsafe fn VertexAttribI3iv(index: GLuint, v: *const GLint) { _VertexAttribI3iv(index, v) }
pub unsafe fn VertexAttribI3ui(index: GLuint, x: GLuint, y: GLuint, z: GLuint) { _VertexAttribI3ui(index, x, y, z) }
pub unsafe fn VertexAttribI3uiv(index: GLuint, v: *const GLuint) { _VertexAttribI3uiv(index, v) }
pub unsafe fn VertexAttribI4bv(index: GLuint, v: *const GLbyte) { _VertexAttribI4bv(index, v) }
pub unsafe fn VertexAttribI4i(index: GLuint, x: GLint, y: GLint, z: GLint, w: GLint) { _VertexAttribI4i(index, x, y, z, w) }
pub unsafe fn VertexAttribI4iv(index: GLuint, v: *const GLint) { _VertexAttribI4iv(index, v) }
pub unsafe fn VertexAttribI4sv(index: GLuint, v: *const GLshort) { _VertexAttribI4sv(index, v) }
pub unsafe fn VertexAttribI4ubv(index: GLuint, v: *const GLubyte) { _VertexAttribI4ubv(index, v) }
pub unsafe fn VertexAttribI4ui(index: GLuint, x: GLuint, y: GLuint, z: GLuint, w: GLuint) { _VertexAttribI4ui(index, x, y, z, w) }
pub unsafe fn VertexAttribI4uiv(index: GLuint, v: *const GLuint) { _VertexAttribI4uiv(index, v) }
pub unsafe fn VertexAttribI4usv(index: GLuint, v: *const GLushort) { _VertexAttribI4usv(index, v) }
pub unsafe fn VertexAttribIFormat(attribindex: GLuint, size: GLint, type_: GLenum, relativeoffset: GLuint) { _VertexAttribIFormat(attribindex, size, type_, relativeoffset) }
pub unsafe fn VertexAttribIPointer(index: GLuint, size: GLint, type_: GLenum, stride: GLsizei, pointer: *const c_void) { _VertexAttribIPointer(index, size, type_, stride, pointer) }
pub unsafe fn VertexAttribL1d(index: GLuint, x: GLdouble) { _VertexAttribL1d(index, x) }
pub unsafe fn VertexAttribL1dv(index: GLuint, v: *const GLdouble) { _VertexAttribL1dv(index, v) }
pub unsafe fn VertexAttribL2d(index: GLuint, x: GLdouble, y: GLdouble) { _VertexAttribL2d(index, x, y) }
pub unsafe fn VertexAttribL2dv(index: GLuint, v: *const GLdouble) { _VertexAttribL2dv(index, v) }
pub unsafe fn VertexAttribL3d(index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble) { _VertexAttribL3d(index, x, y, z) }
pub unsafe fn VertexAttribL3dv(index: GLuint, v: *const GLdouble) { _VertexAttribL3dv(index, v) }
pub unsafe fn VertexAttribL4d(index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble) { _VertexAttribL4d(index, x, y, z, w) }
pub unsafe fn VertexAttribL4dv(index: GLuint, v: *const GLdouble) { _VertexAttribL4dv(index, v) }
pub unsafe fn VertexAttribLFormat(attribindex: GLuint, size: GLint, type_: GLenum, relativeoffset: GLuint) { _VertexAttribLFormat(attribindex, size, type_, relativeoffset) }
pub unsafe fn VertexAttribLPointer(index: GLuint, size: GLint, type_: GLenum, stride: GLsizei, pointer: *const c_void) { _VertexAttribLPointer(index, size, type_, stride, pointer) }
pub unsafe fn VertexAttribP1ui(index: GLuint, type_: GLenum, normalized: GLboolean, value: GLuint) { _VertexAttribP1ui(index, type_, normalized, value) }
pub unsafe fn VertexAttribP1uiv(index: GLuint, type_: GLenum, normalized: GLboolean, value: *const GLuint) { _VertexAttribP1uiv(index, type_, normalized, value) }
pub unsafe fn VertexAttribP2ui(index: GLuint, type_: GLenum, normalized: GLboolean, value: GLuint) { _VertexAttribP2ui(index, type_, normalized, value) }
pub unsafe fn VertexAttribP2uiv(index: GLuint, type_: GLenum, normalized: GLboolean, value: *const GLuint) { _VertexAttribP2uiv(index, type_, normalized, value) }
pub unsafe fn VertexAttribP3ui(index: GLuint, type_: GLenum, normalized: GLboolean, value: GLuint) { _VertexAttribP3ui(index, type_, normalized, value) }
pub unsafe fn VertexAttribP3uiv(index: GLuint, type_: GLenum, normalized: GLboolean, value: *const GLuint) { _VertexAttribP3uiv(index, type_, normalized, value) }
pub unsafe fn VertexAttribP4ui(index: GLuint, type_: GLenum, normalized: GLboolean, value: GLuint) { _VertexAttribP4ui(index, type_, normalized, value) }
pub unsafe fn VertexAttribP4uiv(index: GLuint, type_: GLenum, normalized: GLboolean, value: *const GLuint) { _VertexAttribP4uiv(index, type_, normalized, value) }
pub unsafe fn VertexAttribPointer(index: GLuint, size: GLint, type_: GLenum, normalized: GLboolean, stride: GLsizei, pointer: *const c_void) { _VertexAttribPointer(index, size, type_, normalized, stride, pointer) }
pub unsafe fn VertexBindingDivisor(bindingindex: GLuint, divisor: GLuint) { _VertexBindingDivisor(bindingindex, divisor) }
pub unsafe fn VertexP2ui(type_: GLenum, value: GLuint) { _VertexP2ui(type_, value) }
pub unsafe fn VertexP2uiv(type_: GLenum, value: *const GLuint) { _VertexP2uiv(type_, value) }
pub unsafe fn VertexP3ui(type_: GLenum, value: GLuint) { _VertexP3ui(type_, value) }
pub unsafe fn VertexP3uiv(type_: GLenum, value: *const GLuint) { _VertexP3uiv(type_, value) }
pub unsafe fn VertexP4ui(type_: GLenum, value: GLuint) { _VertexP4ui(type_, value) }
pub unsafe fn VertexP4uiv(type_: GLenum, value: *const GLuint) { _VertexP4uiv(type_, value) }
pub unsafe fn Viewport(x: GLint, y: GLint, width: GLsizei, height: GLsizei) { _Viewport(x, y, width, height) }
pub unsafe fn ViewportArrayv(first: GLuint, count: GLsizei, v: *const GLfloat) { _ViewportArrayv(first, count, v) }
pub unsafe fn ViewportIndexedf(index: GLuint, x: GLfloat, y: GLfloat, w: GLfloat, h: GLfloat) { _ViewportIndexedf(index, x, y, w, h) }
pub unsafe fn ViewportIndexedfv(index: GLuint, v: *const GLfloat) { _ViewportIndexedfv(index, v) }
pub unsafe fn WaitSync(sync: GLsync, flags: GLbitfield, timeout: GLuint64) { _WaitSync(sync, flags, timeout) }
static mut _ActiveShaderProgram: extern "system" fn(GLuint, GLuint) = unsafe { transmute(ERR) };
static mut _ActiveTexture: extern "system" fn(GLenum) = unsafe { transmute(ERR) };
static mut _AttachShader: extern "system" fn(GLuint, GLuint) = unsafe { transmute(ERR) };
static mut _BeginConditionalRender: extern "system" fn(GLuint, GLenum) = unsafe { transmute(ERR) };
static mut _BeginQuery: extern "system" fn(GLenum, GLuint) = unsafe { transmute(ERR) };
static mut _BeginQueryIndexed: extern "system" fn(GLenum, GLuint, GLuint) = unsafe { transmute(ERR) };
static mut _BeginTransformFeedback: extern "system" fn(GLenum) = unsafe { transmute(ERR) };
static mut _BindAttribLocation: extern "system" fn(GLuint, GLuint, *const GLchar) = unsafe { transmute(ERR) };
static mut _BindBuffer: extern "system" fn(GLenum, GLuint) = unsafe { transmute(ERR) };
static mut _BindBufferBase: extern "system" fn(GLenum, GLuint, GLuint) = unsafe { transmute(ERR) };
static mut _BindBufferRange: extern "system" fn(GLenum, GLuint, GLuint, GLintptr, GLsizeiptr) = unsafe { transmute(ERR) };
static mut _BindBuffersBase: extern "system" fn(GLenum, GLuint, GLsizei, *const GLuint) = unsafe { transmute(ERR) };
static mut _BindBuffersRange: extern "system" fn(GLenum, GLuint, GLsizei, *const GLuint, *const GLintptr, *const GLsizeiptr) = unsafe { transmute(ERR) };
static mut _BindFragDataLocation: extern "system" fn(GLuint, GLuint, *const GLchar) = unsafe { transmute(ERR) };
static mut _BindFragDataLocationIndexed: extern "system" fn(GLuint, GLuint, GLuint, *const GLchar) = unsafe { transmute(ERR) };
static mut _BindFramebuffer: extern "system" fn(GLenum, GLuint) = unsafe { transmute(ERR) };
static mut _BindImageTexture: extern "system" fn(GLuint, GLuint, GLint, GLboolean, GLint, GLenum, GLenum) = unsafe { transmute(ERR) };
static mut _BindImageTextures: extern "system" fn(GLuint, GLsizei, *const GLuint) = unsafe { transmute(ERR) };
static mut _BindProgramPipeline: extern "system" fn(GLuint) = unsafe { transmute(ERR) };
static mut _BindRenderbuffer: extern "system" fn(GLenum, GLuint) = unsafe { transmute(ERR) };
static mut _BindSampler: extern "system" fn(GLuint, GLuint) = unsafe { transmute(ERR) };
static mut _BindSamplers: extern "system" fn(GLuint, GLsizei, *const GLuint) = unsafe { transmute(ERR) };
static mut _BindTexture: extern "system" fn(GLenum, GLuint) = unsafe { transmute(ERR) };
static mut _BindTextureUnit: extern "system" fn(GLuint, GLuint) = unsafe { transmute(ERR) };
static mut _BindTextures: extern "system" fn(GLuint, GLsizei, *const GLuint) = unsafe { transmute(ERR) };
static mut _BindTransformFeedback: extern "system" fn(GLenum, GLuint) = unsafe { transmute(ERR) };
static mut _BindVertexArray: extern "system" fn(GLuint) = unsafe { transmute(ERR) };
static mut _BindVertexBuffer: extern "system" fn(GLuint, GLuint, GLintptr, GLsizei) = unsafe { transmute(ERR) };
static mut _BindVertexBuffers: extern "system" fn(GLuint, GLsizei, *const GLuint, *const GLintptr, *const GLsizei) = unsafe { transmute(ERR) };
static mut _BlendColor: extern "system" fn(GLfloat, GLfloat, GLfloat, GLfloat) = unsafe { transmute(ERR) };
static mut _BlendEquation: extern "system" fn(GLenum) = unsafe { transmute(ERR) };
static mut _BlendEquationSeparate: extern "system" fn(GLenum, GLenum) = unsafe { transmute(ERR) };
static mut _BlendEquationSeparatei: extern "system" fn(GLuint, GLenum, GLenum) = unsafe { transmute(ERR) };
static mut _BlendEquationi: extern "system" fn(GLuint, GLenum) = unsafe { transmute(ERR) };
static mut _BlendFunc: extern "system" fn(GLenum, GLenum) = unsafe { transmute(ERR) };
static mut _BlendFuncSeparate: extern "system" fn(GLenum, GLenum, GLenum, GLenum) = unsafe { transmute(ERR) };
static mut _BlendFuncSeparatei: extern "system" fn(GLuint, GLenum, GLenum, GLenum, GLenum) = unsafe { transmute(ERR) };
static mut _BlendFunci: extern "system" fn(GLuint, GLenum, GLenum) = unsafe { transmute(ERR) };
static mut _BlitFramebuffer: extern "system" fn(GLint, GLint, GLint, GLint, GLint, GLint, GLint, GLint, GLbitfield, GLenum) = unsafe { transmute(ERR) };
static mut _BlitNamedFramebuffer: extern "system" fn(GLuint, GLuint, GLint, GLint, GLint, GLint, GLint, GLint, GLint, GLint, GLbitfield, GLenum) = unsafe { transmute(ERR) };
static mut _BufferData: extern "system" fn(GLenum, GLsizeiptr, *const c_void, GLenum) = unsafe { transmute(ERR) };
static mut _BufferStorage: extern "system" fn(GLenum, GLsizeiptr, *const c_void, GLbitfield) = unsafe { transmute(ERR) };
static mut _BufferSubData: extern "system" fn(GLenum, GLintptr, GLsizeiptr, *const c_void) = unsafe { transmute(ERR) };
static mut _CheckFramebufferStatus: extern "system" fn(GLenum) -> GLenum = unsafe { transmute(ERR) };
static mut _CheckNamedFramebufferStatus: extern "system" fn(GLuint, GLenum) -> GLenum = unsafe { transmute(ERR) };
static mut _ClampColor: extern "system" fn(GLenum, GLenum) = unsafe { transmute(ERR) };
static mut _Clear: extern "system" fn(GLbitfield) = unsafe { transmute(ERR) };
static mut _ClearBufferData: extern "system" fn(GLenum, GLenum, GLenum, GLenum, *const c_void) = unsafe { transmute(ERR) };
static mut _ClearBufferSubData: extern "system" fn(GLenum, GLenum, GLintptr, GLsizeiptr, GLenum, GLenum, *const c_void) = unsafe { transmute(ERR) };
static mut _ClearBufferfi: extern "system" fn(GLenum, GLint, GLfloat, GLint) = unsafe { transmute(ERR) };
static mut _ClearBufferfv: extern "system" fn(GLenum, GLint, *const GLfloat) = unsafe { transmute(ERR) };
static mut _ClearBufferiv: extern "system" fn(GLenum, GLint, *const GLint) = unsafe { transmute(ERR) };
static mut _ClearBufferuiv: extern "system" fn(GLenum, GLint, *const GLuint) = unsafe { transmute(ERR) };
static mut _ClearColor: extern "system" fn(GLfloat, GLfloat, GLfloat, GLfloat) = unsafe { transmute(ERR) };
static mut _ClearDepth: extern "system" fn(GLdouble) = unsafe { transmute(ERR) };
static mut _ClearDepthf: extern "system" fn(GLfloat) = unsafe { transmute(ERR) };
static mut _ClearNamedBufferData: extern "system" fn(GLuint, GLenum, GLenum, GLenum, *const c_void) = unsafe { transmute(ERR) };
static mut _ClearNamedBufferSubData: extern "system" fn(GLuint, GLenum, GLintptr, GLsizeiptr, GLenum, GLenum, *const c_void) = unsafe { transmute(ERR) };
static mut _ClearNamedFramebufferfi: extern "system" fn(GLuint, GLenum, GLint, GLfloat, GLint) = unsafe { transmute(ERR) };
static mut _ClearNamedFramebufferfv: extern "system" fn(GLuint, GLenum, GLint, *const GLfloat) = unsafe { transmute(ERR) };
static mut _ClearNamedFramebufferiv: extern "system" fn(GLuint, GLenum, GLint, *const GLint) = unsafe { transmute(ERR) };
static mut _ClearNamedFramebufferuiv: extern "system" fn(GLuint, GLenum, GLint, *const GLuint) = unsafe { transmute(ERR) };
static mut _ClearStencil: extern "system" fn(GLint) = unsafe { transmute(ERR) };
static mut _ClearTexImage: extern "system" fn(GLuint, GLint, GLenum, GLenum, *const c_void) = unsafe { transmute(ERR) };
static mut _ClearTexSubImage: extern "system" fn(GLuint, GLint, GLint, GLint, GLint, GLsizei, GLsizei, GLsizei, GLenum, GLenum, *const c_void) = unsafe { transmute(ERR) };
static mut _ClientWaitSync: extern "system" fn(GLsync, GLbitfield, GLuint64) -> GLenum = unsafe { transmute(ERR) };
static mut _ClipControl: extern "system" fn(GLenum, GLenum) = unsafe { transmute(ERR) };
static mut _ColorMask: extern "system" fn(GLboolean, GLboolean, GLboolean, GLboolean) = unsafe { transmute(ERR) };
static mut _ColorMaski: extern "system" fn(GLuint, GLboolean, GLboolean, GLboolean, GLboolean) = unsafe { transmute(ERR) };
static mut _ColorP3ui: extern "system" fn(GLenum, GLuint) = unsafe { transmute(ERR) };
static mut _ColorP3uiv: extern "system" fn(GLenum, *const GLuint) = unsafe { transmute(ERR) };
static mut _ColorP4ui: extern "system" fn(GLenum, GLuint) = unsafe { transmute(ERR) };
static mut _ColorP4uiv: extern "system" fn(GLenum, *const GLuint) = unsafe { transmute(ERR) };
static mut _CompileShader: extern "system" fn(GLuint) = unsafe { transmute(ERR) };
static mut _CompressedTexImage1D: extern "system" fn(GLenum, GLint, GLenum, GLsizei, GLint, GLsizei, *const c_void) = unsafe { transmute(ERR) };
static mut _CompressedTexImage2D: extern "system" fn(GLenum, GLint, GLenum, GLsizei, GLsizei, GLint, GLsizei, *const c_void) = unsafe { transmute(ERR) };
static mut _CompressedTexImage3D: extern "system" fn(GLenum, GLint, GLenum, GLsizei, GLsizei, GLsizei, GLint, GLsizei, *const c_void) = unsafe { transmute(ERR) };
static mut _CompressedTexSubImage1D: extern "system" fn(GLenum, GLint, GLint, GLsizei, GLenum, GLsizei, *const c_void) = unsafe { transmute(ERR) };
static mut _CompressedTexSubImage2D: extern "system" fn(GLenum, GLint, GLint, GLint, GLsizei, GLsizei, GLenum, GLsizei, *const c_void) = unsafe { transmute(ERR) };
static mut _CompressedTexSubImage3D: extern "system" fn(GLenum, GLint, GLint, GLint, GLint, GLsizei, GLsizei, GLsizei, GLenum, GLsizei, *const c_void) = unsafe { transmute(ERR) };
static mut _CompressedTextureSubImage1D: extern "system" fn(GLuint, GLint, GLint, GLsizei, GLenum, GLsizei, *const c_void) = unsafe { transmute(ERR) };
static mut _CompressedTextureSubImage2D: extern "system" fn(GLuint, GLint, GLint, GLint, GLsizei, GLsizei, GLenum, GLsizei, *const c_void) = unsafe { transmute(ERR) };
static mut _CompressedTextureSubImage3D: extern "system" fn(GLuint, GLint, GLint, GLint, GLint, GLsizei, GLsizei, GLsizei, GLenum, GLsizei, *const c_void) = unsafe { transmute(ERR) };
static mut _CopyBufferSubData: extern "system" fn(GLenum, GLenum, GLintptr, GLintptr, GLsizeiptr) = unsafe { transmute(ERR) };
static mut _CopyImageSubData: extern "system" fn(GLuint, GLenum, GLint, GLint, GLint, GLint, GLuint, GLenum, GLint, GLint, GLint, GLint, GLsizei, GLsizei, GLsizei) = unsafe { transmute(ERR) };
static mut _CopyNamedBufferSubData: extern "system" fn(GLuint, GLuint, GLintptr, GLintptr, GLsizeiptr) = unsafe { transmute(ERR) };
static mut _CopyTexImage1D: extern "system" fn(GLenum, GLint, GLenum, GLint, GLint, GLsizei, GLint) = unsafe { transmute(ERR) };
static mut _CopyTexImage2D: extern "system" fn(GLenum, GLint, GLenum, GLint, GLint, GLsizei, GLsizei, GLint) = unsafe { transmute(ERR) };
static mut _CopyTexSubImage1D: extern "system" fn(GLenum, GLint, GLint, GLint, GLint, GLsizei) = unsafe { transmute(ERR) };
static mut _CopyTexSubImage2D: extern "system" fn(GLenum, GLint, GLint, GLint, GLint, GLint, GLsizei, GLsizei) = unsafe { transmute(ERR) };
static mut _CopyTexSubImage3D: extern "system" fn(GLenum, GLint, GLint, GLint, GLint, GLint, GLint, GLsizei, GLsizei) = unsafe { transmute(ERR) };
static mut _CopyTextureSubImage1D: extern "system" fn(GLuint, GLint, GLint, GLint, GLint, GLsizei) = unsafe { transmute(ERR) };
static mut _CopyTextureSubImage2D: extern "system" fn(GLuint, GLint, GLint, GLint, GLint, GLint, GLsizei, GLsizei) = unsafe { transmute(ERR) };
static mut _CopyTextureSubImage3D: extern "system" fn(GLuint, GLint, GLint, GLint, GLint, GLint, GLint, GLsizei, GLsizei) = unsafe { transmute(ERR) };
static mut _CreateBuffers: extern "system" fn(GLsizei, *mut GLuint) = unsafe { transmute(ERR) };
static mut _CreateFramebuffers: extern "system" fn(GLsizei, *mut GLuint) = unsafe { transmute(ERR) };
static mut _CreateProgram: extern "system" fn() -> GLuint = unsafe { transmute(ERR) };
static mut _CreateProgramPipelines: extern "system" fn(GLsizei, *mut GLuint) = unsafe { transmute(ERR) };
static mut _CreateQueries: extern "system" fn(GLenum, GLsizei, *mut GLuint) = unsafe { transmute(ERR) };
static mut _CreateRenderbuffers: extern "system" fn(GLsizei, *mut GLuint) = unsafe { transmute(ERR) };
static mut _CreateSamplers: extern "system" fn(GLsizei, *mut GLuint) = unsafe { transmute(ERR) };
static mut _CreateShader: extern "system" fn(GLenum) -> GLuint = unsafe { transmute(ERR) };
static mut _CreateShaderProgramv: extern "system" fn(GLenum, GLsizei, *const *const GLchar) -> GLuint = unsafe { transmute(ERR) };
static mut _CreateTextures: extern "system" fn(GLenum, GLsizei, *mut GLuint) = unsafe { transmute(ERR) };
static mut _CreateTransformFeedbacks: extern "system" fn(GLsizei, *mut GLuint) = unsafe { transmute(ERR) };
static mut _CreateVertexArrays: extern "system" fn(GLsizei, *mut GLuint) = unsafe { transmute(ERR) };
static mut _CullFace: extern "system" fn(GLenum) = unsafe { transmute(ERR) };
static mut _DebugMessageCallback: extern "system" fn(GLDEBUGPROC, *const c_void) = unsafe { transmute(ERR) };
static mut _DebugMessageControl: extern "system" fn(GLenum, GLenum, GLenum, GLsizei, *const GLuint, GLboolean) = unsafe { transmute(ERR) };
static mut _DebugMessageInsert: extern "system" fn(GLenum, GLenum, GLuint, GLenum, GLsizei, *const GLchar) = unsafe { transmute(ERR) };
static mut _DeleteBuffers: extern "system" fn(GLsizei, *const GLuint) = unsafe { transmute(ERR) };
static mut _DeleteFramebuffers: extern "system" fn(GLsizei, *const GLuint) = unsafe { transmute(ERR) };
static mut _DeleteProgram: extern "system" fn(GLuint) = unsafe { transmute(ERR) };
static mut _DeleteProgramPipelines: extern "system" fn(GLsizei, *const GLuint) = unsafe { transmute(ERR) };
static mut _DeleteQueries: extern "system" fn(GLsizei, *const GLuint) = unsafe { transmute(ERR) };
static mut _DeleteRenderbuffers: extern "system" fn(GLsizei, *const GLuint) = unsafe { transmute(ERR) };
static mut _DeleteSamplers: extern "system" fn(GLsizei, *const GLuint) = unsafe { transmute(ERR) };
static mut _DeleteShader: extern "system" fn(GLuint) = unsafe { transmute(ERR) };
static mut _DeleteSync: extern "system" fn(GLsync) = unsafe { transmute(ERR) };
static mut _DeleteTextures: extern "system" fn(GLsizei, *const GLuint) = unsafe { transmute(ERR) };
static mut _DeleteTransformFeedbacks: extern "system" fn(GLsizei, *const GLuint) = unsafe { transmute(ERR) };
static mut _DeleteVertexArrays: extern "system" fn(GLsizei, *const GLuint) = unsafe { transmute(ERR) };
static mut _DepthFunc: extern "system" fn(GLenum) = unsafe { transmute(ERR) };
static mut _DepthMask: extern "system" fn(GLboolean) = unsafe { transmute(ERR) };
static mut _DepthRange: extern "system" fn(GLdouble, GLdouble) = unsafe { transmute(ERR) };
static mut _DepthRangeArrayv: extern "system" fn(GLuint, GLsizei, *const GLdouble) = unsafe { transmute(ERR) };
static mut _DepthRangeIndexed: extern "system" fn(GLuint, GLdouble, GLdouble) = unsafe { transmute(ERR) };
static mut _DepthRangef: extern "system" fn(GLfloat, GLfloat) = unsafe { transmute(ERR) };
static mut _DetachShader: extern "system" fn(GLuint, GLuint) = unsafe { transmute(ERR) };
static mut _Disable: extern "system" fn(GLenum) = unsafe { transmute(ERR) };
static mut _DisableVertexArrayAttrib: extern "system" fn(GLuint, GLuint) = unsafe { transmute(ERR) };
static mut _DisableVertexAttribArray: extern "system" fn(GLuint) = unsafe { transmute(ERR) };
static mut _Disablei: extern "system" fn(GLenum, GLuint) = unsafe { transmute(ERR) };
static mut _DispatchCompute: extern "system" fn(GLuint, GLuint, GLuint) = unsafe { transmute(ERR) };
static mut _DispatchComputeIndirect: extern "system" fn(GLintptr) = unsafe { transmute(ERR) };
static mut _DrawArrays: extern "system" fn(GLenum, GLint, GLsizei) = unsafe { transmute(ERR) };
static mut _DrawArraysIndirect: extern "system" fn(GLenum, *const c_void) = unsafe { transmute(ERR) };
static mut _DrawArraysInstanced: extern "system" fn(GLenum, GLint, GLsizei, GLsizei) = unsafe { transmute(ERR) };
static mut _DrawArraysInstancedBaseInstance: extern "system" fn(GLenum, GLint, GLsizei, GLsizei, GLuint) = unsafe { transmute(ERR) };
static mut _DrawBuffer: extern "system" fn(GLenum) = unsafe { transmute(ERR) };
static mut _DrawBuffers: extern "system" fn(GLsizei, *const GLenum) = unsafe { transmute(ERR) };
static mut _DrawElements: extern "system" fn(GLenum, GLsizei, GLenum, *const c_void) = unsafe { transmute(ERR) };
static mut _DrawElementsBaseVertex: extern "system" fn(GLenum, GLsizei, GLenum, *const c_void, GLint) = unsafe { transmute(ERR) };
static mut _DrawElementsIndirect: extern "system" fn(GLenum, GLenum, *const c_void) = unsafe { transmute(ERR) };
static mut _DrawElementsInstanced: extern "system" fn(GLenum, GLsizei, GLenum, *const c_void, GLsizei) = unsafe { transmute(ERR) };
static mut _DrawElementsInstancedBaseInstance: extern "system" fn(GLenum, GLsizei, GLenum, *const c_void, GLsizei, GLuint) = unsafe { transmute(ERR) };
static mut _DrawElementsInstancedBaseVertex: extern "system" fn(GLenum, GLsizei, GLenum, *const c_void, GLsizei, GLint) = unsafe { transmute(ERR) };
static mut _DrawElementsInstancedBaseVertexBaseInstance: extern "system" fn(GLenum, GLsizei, GLenum, *const c_void, GLsizei, GLint, GLuint) = unsafe { transmute(ERR) };
static mut _DrawRangeElements: extern "system" fn(GLenum, GLuint, GLuint, GLsizei, GLenum, *const c_void) = unsafe { transmute(ERR) };
static mut _DrawRangeElementsBaseVertex: extern "system" fn(GLenum, GLuint, GLuint, GLsizei, GLenum, *const c_void, GLint) = unsafe { transmute(ERR) };
static mut _DrawTransformFeedback: extern "system" fn(GLenum, GLuint) = unsafe { transmute(ERR) };
static mut _DrawTransformFeedbackInstanced: extern "system" fn(GLenum, GLuint, GLsizei) = unsafe { transmute(ERR) };
static mut _DrawTransformFeedbackStream: extern "system" fn(GLenum, GLuint, GLuint) = unsafe { transmute(ERR) };
static mut _DrawTransformFeedbackStreamInstanced: extern "system" fn(GLenum, GLuint, GLuint, GLsizei) = unsafe { transmute(ERR) };
static mut _Enable: extern "system" fn(GLenum) = unsafe { transmute(ERR) };
static mut _EnableVertexArrayAttrib: extern "system" fn(GLuint, GLuint) = unsafe { transmute(ERR) };
static mut _EnableVertexAttribArray: extern "system" fn(GLuint) = unsafe { transmute(ERR) };
static mut _Enablei: extern "system" fn(GLenum, GLuint) = unsafe { transmute(ERR) };
static mut _EndConditionalRender: extern "system" fn() = unsafe { transmute(ERR) };
static mut _EndQuery: extern "system" fn(GLenum) = unsafe { transmute(ERR) };
static mut _EndQueryIndexed: extern "system" fn(GLenum, GLuint) = unsafe { transmute(ERR) };
static mut _EndTransformFeedback: extern "system" fn() = unsafe { transmute(ERR) };
static mut _FenceSync: extern "system" fn(GLenum, GLbitfield) -> GLsync = unsafe { transmute(ERR) };
static mut _Finish: extern "system" fn() = unsafe { transmute(ERR) };
static mut _Flush: extern "system" fn() = unsafe { transmute(ERR) };
static mut _FlushMappedBufferRange: extern "system" fn(GLenum, GLintptr, GLsizeiptr) = unsafe { transmute(ERR) };
static mut _FlushMappedNamedBufferRange: extern "system" fn(GLuint, GLintptr, GLsizeiptr) = unsafe { transmute(ERR) };
static mut _FramebufferParameteri: extern "system" fn(GLenum, GLenum, GLint) = unsafe { transmute(ERR) };
static mut _FramebufferRenderbuffer: extern "system" fn(GLenum, GLenum, GLenum, GLuint) = unsafe { transmute(ERR) };
static mut _FramebufferTexture: extern "system" fn(GLenum, GLenum, GLuint, GLint) = unsafe { transmute(ERR) };
static mut _FramebufferTexture1D: extern "system" fn(GLenum, GLenum, GLenum, GLuint, GLint) = unsafe { transmute(ERR) };
static mut _FramebufferTexture2D: extern "system" fn(GLenum, GLenum, GLenum, GLuint, GLint) = unsafe { transmute(ERR) };
static mut _FramebufferTexture3D: extern "system" fn(GLenum, GLenum, GLenum, GLuint, GLint, GLint) = unsafe { transmute(ERR) };
static mut _FramebufferTextureLayer: extern "system" fn(GLenum, GLenum, GLuint, GLint, GLint) = unsafe { transmute(ERR) };
static mut _FrontFace: extern "system" fn(GLenum) = unsafe { transmute(ERR) };
static mut _GenBuffers: extern "system" fn(GLsizei, *mut GLuint) = unsafe { transmute(ERR) };
static mut _GenFramebuffers: extern "system" fn(GLsizei, *mut GLuint) = unsafe { transmute(ERR) };
static mut _GenProgramPipelines: extern "system" fn(GLsizei, *mut GLuint) = unsafe { transmute(ERR) };
static mut _GenQueries: extern "system" fn(GLsizei, *mut GLuint) = unsafe { transmute(ERR) };
static mut _GenRenderbuffers: extern "system" fn(GLsizei, *mut GLuint) = unsafe { transmute(ERR) };
static mut _GenSamplers: extern "system" fn(GLsizei, *mut GLuint) = unsafe { transmute(ERR) };
static mut _GenTextures: extern "system" fn(GLsizei, *mut GLuint) = unsafe { transmute(ERR) };
static mut _GenTransformFeedbacks: extern "system" fn(GLsizei, *mut GLuint) = unsafe { transmute(ERR) };
static mut _GenVertexArrays: extern "system" fn(GLsizei, *mut GLuint) = unsafe { transmute(ERR) };
static mut _GenerateMipmap: extern "system" fn(GLenum) = unsafe { transmute(ERR) };
static mut _GenerateTextureMipmap: extern "system" fn(GLuint) = unsafe { transmute(ERR) };
static mut _GetActiveAtomicCounterBufferiv: extern "system" fn(GLuint, GLuint, GLenum, *mut GLint) = unsafe { transmute(ERR) };
static mut _GetActiveAttrib: extern "system" fn(GLuint, GLuint, GLsizei, *mut GLsizei, *mut GLint, *mut GLenum, *mut GLchar) = unsafe { transmute(ERR) };
static mut _GetActiveSubroutineName: extern "system" fn(GLuint, GLenum, GLuint, GLsizei, *mut GLsizei, *mut GLchar) = unsafe { transmute(ERR) };
static mut _GetActiveSubroutineUniformName: extern "system" fn(GLuint, GLenum, GLuint, GLsizei, *mut GLsizei, *mut GLchar) = unsafe { transmute(ERR) };
static mut _GetActiveSubroutineUniformiv: extern "system" fn(GLuint, GLenum, GLuint, GLenum, *mut GLint) = unsafe { transmute(ERR) };
static mut _GetActiveUniform: extern "system" fn(GLuint, GLuint, GLsizei, *mut GLsizei, *mut GLint, *mut GLenum, *mut GLchar) = unsafe { transmute(ERR) };
static mut _GetActiveUniformBlockName: extern "system" fn(GLuint, GLuint, GLsizei, *mut GLsizei, *mut GLchar) = unsafe { transmute(ERR) };
static mut _GetActiveUniformBlockiv: extern "system" fn(GLuint, GLuint, GLenum, *mut GLint) = unsafe { transmute(ERR) };
static mut _GetActiveUniformName: extern "system" fn(GLuint, GLuint, GLsizei, *mut GLsizei, *mut GLchar) = unsafe { transmute(ERR) };
static mut _GetActiveUniformsiv: extern "system" fn(GLuint, GLsizei, *const GLuint, GLenum, *mut GLint) = unsafe { transmute(ERR) };
static mut _GetAttachedShaders: extern "system" fn(GLuint, GLsizei, *mut GLsizei, *mut GLuint) = unsafe { transmute(ERR) };
static mut _GetAttribLocation: extern "system" fn(GLuint, *const GLchar) -> GLint = unsafe { transmute(ERR) };
static mut _GetBooleani_v: extern "system" fn(GLenum, GLuint, *mut GLboolean) = unsafe { transmute(ERR) };
static mut _GetBooleanv: extern "system" fn(GLenum, *mut GLboolean) = unsafe { transmute(ERR) };
static mut _GetBufferParameteri64v: extern "system" fn(GLenum, GLenum, *mut GLint64) = unsafe { transmute(ERR) };
static mut _GetBufferParameteriv: extern "system" fn(GLenum, GLenum, *mut GLint) = unsafe { transmute(ERR) };
static mut _GetBufferPointerv: extern "system" fn(GLenum, GLenum, *const *mut c_void) = unsafe { transmute(ERR) };
static mut _GetBufferSubData: extern "system" fn(GLenum, GLintptr, GLsizeiptr, *mut c_void) = unsafe { transmute(ERR) };
static mut _GetCompressedTexImage: extern "system" fn(GLenum, GLint, *mut c_void) = unsafe { transmute(ERR) };
static mut _GetCompressedTextureImage: extern "system" fn(GLuint, GLint, GLsizei, *mut c_void) = unsafe { transmute(ERR) };
static mut _GetCompressedTextureSubImage: extern "system" fn(GLuint, GLint, GLint, GLint, GLint, GLsizei, GLsizei, GLsizei, GLsizei, *mut c_void) = unsafe { transmute(ERR) };
static mut _GetDebugMessageLog: extern "system" fn(GLuint, GLsizei, *mut GLenum, *mut GLenum, *mut GLuint, *mut GLenum, *mut GLsizei, *mut GLchar) -> GLuint = unsafe { transmute(ERR) };
static mut _GetDoublei_v: extern "system" fn(GLenum, GLuint, *mut GLdouble) = unsafe { transmute(ERR) };
static mut _GetDoublev: extern "system" fn(GLenum, *mut GLdouble) = unsafe { transmute(ERR) };
static mut _GetError: extern "system" fn() -> GLenum = unsafe { transmute(ERR) };
static mut _GetFloati_v: extern "system" fn(GLenum, GLuint, *mut GLfloat) = unsafe { transmute(ERR) };
static mut _GetFloatv: extern "system" fn(GLenum, *mut GLfloat) = unsafe { transmute(ERR) };
static mut _GetFragDataIndex: extern "system" fn(GLuint, *const GLchar) -> GLint = unsafe { transmute(ERR) };
static mut _GetFragDataLocation: extern "system" fn(GLuint, *const GLchar) -> GLint = unsafe { transmute(ERR) };
static mut _GetFramebufferAttachmentParameteriv: extern "system" fn(GLenum, GLenum, GLenum, *mut GLint) = unsafe { transmute(ERR) };
static mut _GetFramebufferParameteriv: extern "system" fn(GLenum, GLenum, *mut GLint) = unsafe { transmute(ERR) };
static mut _GetGraphicsResetStatus: extern "system" fn() -> GLenum = unsafe { transmute(ERR) };
static mut _GetInteger64i_v: extern "system" fn(GLenum, GLuint, *mut GLint64) = unsafe { transmute(ERR) };
static mut _GetInteger64v: extern "system" fn(GLenum, *mut GLint64) = unsafe { transmute(ERR) };
static mut _GetIntegeri_v: extern "system" fn(GLenum, GLuint, *mut GLint) = unsafe { transmute(ERR) };
static mut _GetIntegerv: extern "system" fn(GLenum, *mut GLint) = unsafe { transmute(ERR) };
static mut _GetInternalformati64v: extern "system" fn(GLenum, GLenum, GLenum, GLsizei, *mut GLint64) = unsafe { transmute(ERR) };
static mut _GetInternalformativ: extern "system" fn(GLenum, GLenum, GLenum, GLsizei, *mut GLint) = unsafe { transmute(ERR) };
static mut _GetMultisamplefv: extern "system" fn(GLenum, GLuint, *mut GLfloat) = unsafe { transmute(ERR) };
static mut _GetNamedBufferParameteri64v: extern "system" fn(GLuint, GLenum, *mut GLint64) = unsafe { transmute(ERR) };
static mut _GetNamedBufferParameteriv: extern "system" fn(GLuint, GLenum, *mut GLint) = unsafe { transmute(ERR) };
static mut _GetNamedBufferPointerv: extern "system" fn(GLuint, GLenum, *const *mut c_void) = unsafe { transmute(ERR) };
static mut _GetNamedBufferSubData: extern "system" fn(GLuint, GLintptr, GLsizeiptr, *mut c_void) = unsafe { transmute(ERR) };
static mut _GetNamedFramebufferAttachmentParameteriv: extern "system" fn(GLuint, GLenum, GLenum, *mut GLint) = unsafe { transmute(ERR) };
static mut _GetNamedFramebufferParameteriv: extern "system" fn(GLuint, GLenum, *mut GLint) = unsafe { transmute(ERR) };
static mut _GetNamedRenderbufferParameteriv: extern "system" fn(GLuint, GLenum, *mut GLint) = unsafe { transmute(ERR) };
static mut _GetObjectLabel: extern "system" fn(GLenum, GLuint, GLsizei, *mut GLsizei, *mut GLchar) = unsafe { transmute(ERR) };
static mut _GetObjectPtrLabel: extern "system" fn(*const c_void, GLsizei, *mut GLsizei, *mut GLchar) = unsafe { transmute(ERR) };
static mut _GetPointerv: extern "system" fn(GLenum, *const *mut c_void) = unsafe { transmute(ERR) };
static mut _GetProgramBinary: extern "system" fn(GLuint, GLsizei, *mut GLsizei, *mut GLenum, *mut c_void) = unsafe { transmute(ERR) };
static mut _GetProgramInfoLog: extern "system" fn(GLuint, GLsizei, *mut GLsizei, *mut GLchar) = unsafe { transmute(ERR) };
static mut _GetProgramInterfaceiv: extern "system" fn(GLuint, GLenum, GLenum, *mut GLint) = unsafe { transmute(ERR) };
static mut _GetProgramPipelineInfoLog: extern "system" fn(GLuint, GLsizei, *mut GLsizei, *mut GLchar) = unsafe { transmute(ERR) };
static mut _GetProgramPipelineiv: extern "system" fn(GLuint, GLenum, *mut GLint) = unsafe { transmute(ERR) };
static mut _GetProgramResourceIndex: extern "system" fn(GLuint, GLenum, *const GLchar) -> GLuint = unsafe { transmute(ERR) };
static mut _GetProgramResourceLocation: extern "system" fn(GLuint, GLenum, *const GLchar) -> GLint = unsafe { transmute(ERR) };
static mut _GetProgramResourceLocationIndex: extern "system" fn(GLuint, GLenum, *const GLchar) -> GLint = unsafe { transmute(ERR) };
static mut _GetProgramResourceName: extern "system" fn(GLuint, GLenum, GLuint, GLsizei, *mut GLsizei, *mut GLchar) = unsafe { transmute(ERR) };
static mut _GetProgramResourceiv: extern "system" fn(GLuint, GLenum, GLuint, GLsizei, *const GLenum, GLsizei, *mut GLsizei, *mut GLint) = unsafe { transmute(ERR) };
static mut _GetProgramStageiv: extern "system" fn(GLuint, GLenum, GLenum, *mut GLint) = unsafe { transmute(ERR) };
static mut _GetProgramiv: extern "system" fn(GLuint, GLenum, *mut GLint) = unsafe { transmute(ERR) };
static mut _GetQueryBufferObjecti64v: extern "system" fn(GLuint, GLuint, GLenum, GLintptr) = unsafe { transmute(ERR) };
static mut _GetQueryBufferObjectiv: extern "system" fn(GLuint, GLuint, GLenum, GLintptr) = unsafe { transmute(ERR) };
static mut _GetQueryBufferObjectui64v: extern "system" fn(GLuint, GLuint, GLenum, GLintptr) = unsafe { transmute(ERR) };
static mut _GetQueryBufferObjectuiv: extern "system" fn(GLuint, GLuint, GLenum, GLintptr) = unsafe { transmute(ERR) };
static mut _GetQueryIndexediv: extern "system" fn(GLenum, GLuint, GLenum, *mut GLint) = unsafe { transmute(ERR) };
static mut _GetQueryObjecti64v: extern "system" fn(GLuint, GLenum, *mut GLint64) = unsafe { transmute(ERR) };
static mut _GetQueryObjectiv: extern "system" fn(GLuint, GLenum, *mut GLint) = unsafe { transmute(ERR) };
static mut _GetQueryObjectui64v: extern "system" fn(GLuint, GLenum, *mut GLuint64) = unsafe { transmute(ERR) };
static mut _GetQueryObjectuiv: extern "system" fn(GLuint, GLenum, *mut GLuint) = unsafe { transmute(ERR) };
static mut _GetQueryiv: extern "system" fn(GLenum, GLenum, *mut GLint) = unsafe { transmute(ERR) };
static mut _GetRenderbufferParameteriv: extern "system" fn(GLenum, GLenum, *mut GLint) = unsafe { transmute(ERR) };
static mut _GetSamplerParameterIiv: extern "system" fn(GLuint, GLenum, *mut GLint) = unsafe { transmute(ERR) };
static mut _GetSamplerParameterIuiv: extern "system" fn(GLuint, GLenum, *mut GLuint) = unsafe { transmute(ERR) };
static mut _GetSamplerParameterfv: extern "system" fn(GLuint, GLenum, *mut GLfloat) = unsafe { transmute(ERR) };
static mut _GetSamplerParameteriv: extern "system" fn(GLuint, GLenum, *mut GLint) = unsafe { transmute(ERR) };
static mut _GetShaderInfoLog: extern "system" fn(GLuint, GLsizei, *mut GLsizei, *mut GLchar) = unsafe { transmute(ERR) };
static mut _GetShaderPrecisionFormat: extern "system" fn(GLenum, GLenum, *mut GLint, *mut GLint) = unsafe { transmute(ERR) };
static mut _GetShaderSource: extern "system" fn(GLuint, GLsizei, *mut GLsizei, *mut GLchar) = unsafe { transmute(ERR) };
static mut _GetShaderiv: extern "system" fn(GLuint, GLenum, *mut GLint) = unsafe { transmute(ERR) };
static mut _GetString: extern "system" fn(GLenum) -> *const GLubyte = unsafe { transmute(ERR) };
static mut _GetStringi: extern "system" fn(GLenum, GLuint) -> *const GLubyte = unsafe { transmute(ERR) };
static mut _GetSubroutineIndex: extern "system" fn(GLuint, GLenum, *const GLchar) -> GLuint = unsafe { transmute(ERR) };
static mut _GetSubroutineUniformLocation: extern "system" fn(GLuint, GLenum, *const GLchar) -> GLint = unsafe { transmute(ERR) };
static mut _GetSynciv: extern "system" fn(GLsync, GLenum, GLsizei, *mut GLsizei, *mut GLint) = unsafe { transmute(ERR) };
static mut _GetTexImage: extern "system" fn(GLenum, GLint, GLenum, GLenum, *mut c_void) = unsafe { transmute(ERR) };
static mut _GetTexLevelParameterfv: extern "system" fn(GLenum, GLint, GLenum, *mut GLfloat) = unsafe { transmute(ERR) };
static mut _GetTexLevelParameteriv: extern "system" fn(GLenum, GLint, GLenum, *mut GLint) = unsafe { transmute(ERR) };
static mut _GetTexParameterIiv: extern "system" fn(GLenum, GLenum, *mut GLint) = unsafe { transmute(ERR) };
static mut _GetTexParameterIuiv: extern "system" fn(GLenum, GLenum, *mut GLuint) = unsafe { transmute(ERR) };
static mut _GetTexParameterfv: extern "system" fn(GLenum, GLenum, *mut GLfloat) = unsafe { transmute(ERR) };
static mut _GetTexParameteriv: extern "system" fn(GLenum, GLenum, *mut GLint) = unsafe { transmute(ERR) };
static mut _GetTextureImage: extern "system" fn(GLuint, GLint, GLenum, GLenum, GLsizei, *mut c_void) = unsafe { transmute(ERR) };
static mut _GetTextureLevelParameterfv: extern "system" fn(GLuint, GLint, GLenum, *mut GLfloat) = unsafe { transmute(ERR) };
static mut _GetTextureLevelParameteriv: extern "system" fn(GLuint, GLint, GLenum, *mut GLint) = unsafe { transmute(ERR) };
static mut _GetTextureParameterIiv: extern "system" fn(GLuint, GLenum, *mut GLint) = unsafe { transmute(ERR) };
static mut _GetTextureParameterIuiv: extern "system" fn(GLuint, GLenum, *mut GLuint) = unsafe { transmute(ERR) };
static mut _GetTextureParameterfv: extern "system" fn(GLuint, GLenum, *mut GLfloat) = unsafe { transmute(ERR) };
static mut _GetTextureParameteriv: extern "system" fn(GLuint, GLenum, *mut GLint) = unsafe { transmute(ERR) };
static mut _GetTextureSubImage: extern "system" fn(GLuint, GLint, GLint, GLint, GLint, GLsizei, GLsizei, GLsizei, GLenum, GLenum, GLsizei, *mut c_void) = unsafe { transmute(ERR) };
static mut _GetTransformFeedbackVarying: extern "system" fn(GLuint, GLuint, GLsizei, *mut GLsizei, *mut GLsizei, *mut GLenum, *mut GLchar) = unsafe { transmute(ERR) };
static mut _GetTransformFeedbacki64_v: extern "system" fn(GLuint, GLenum, GLuint, *mut GLint64) = unsafe { transmute(ERR) };
static mut _GetTransformFeedbacki_v: extern "system" fn(GLuint, GLenum, GLuint, *mut GLint) = unsafe { transmute(ERR) };
static mut _GetTransformFeedbackiv: extern "system" fn(GLuint, GLenum, *mut GLint) = unsafe { transmute(ERR) };
static mut _GetUniformBlockIndex: extern "system" fn(GLuint, *const GLchar) -> GLuint = unsafe { transmute(ERR) };
static mut _GetUniformIndices: extern "system" fn(GLuint, GLsizei, *const *const GLchar, *mut GLuint) = unsafe { transmute(ERR) };
static mut _GetUniformLocation: extern "system" fn(GLuint, *const GLchar) -> GLint = unsafe { transmute(ERR) };
static mut _GetUniformSubroutineuiv: extern "system" fn(GLenum, GLint, *mut GLuint) = unsafe { transmute(ERR) };
static mut _GetUniformdv: extern "system" fn(GLuint, GLint, *mut GLdouble) = unsafe { transmute(ERR) };
static mut _GetUniformfv: extern "system" fn(GLuint, GLint, *mut GLfloat) = unsafe { transmute(ERR) };
static mut _GetUniformiv: extern "system" fn(GLuint, GLint, *mut GLint) = unsafe { transmute(ERR) };
static mut _GetUniformuiv: extern "system" fn(GLuint, GLint, *mut GLuint) = unsafe { transmute(ERR) };
static mut _GetVertexArrayIndexed64iv: extern "system" fn(GLuint, GLuint, GLenum, *mut GLint64) = unsafe { transmute(ERR) };
static mut _GetVertexArrayIndexediv: extern "system" fn(GLuint, GLuint, GLenum, *mut GLint) = unsafe { transmute(ERR) };
static mut _GetVertexArrayiv: extern "system" fn(GLuint, GLenum, *mut GLint) = unsafe { transmute(ERR) };
static mut _GetVertexAttribIiv: extern "system" fn(GLuint, GLenum, *mut GLint) = unsafe { transmute(ERR) };
static mut _GetVertexAttribIuiv: extern "system" fn(GLuint, GLenum, *mut GLuint) = unsafe { transmute(ERR) };
static mut _GetVertexAttribLdv: extern "system" fn(GLuint, GLenum, *mut GLdouble) = unsafe { transmute(ERR) };
static mut _GetVertexAttribPointerv: extern "system" fn(GLuint, GLenum, *const *mut c_void) = unsafe { transmute(ERR) };
static mut _GetVertexAttribdv: extern "system" fn(GLuint, GLenum, *mut GLdouble) = unsafe { transmute(ERR) };
static mut _GetVertexAttribfv: extern "system" fn(GLuint, GLenum, *mut GLfloat) = unsafe { transmute(ERR) };
static mut _GetVertexAttribiv: extern "system" fn(GLuint, GLenum, *mut GLint) = unsafe { transmute(ERR) };
static mut _GetnColorTable: extern "system" fn(GLenum, GLenum, GLenum, GLsizei, *mut c_void) = unsafe { transmute(ERR) };
static mut _GetnCompressedTexImage: extern "system" fn(GLenum, GLint, GLsizei, *mut c_void) = unsafe { transmute(ERR) };
static mut _GetnConvolutionFilter: extern "system" fn(GLenum, GLenum, GLenum, GLsizei, *mut c_void) = unsafe { transmute(ERR) };
static mut _GetnHistogram: extern "system" fn(GLenum, GLboolean, GLenum, GLenum, GLsizei, *mut c_void) = unsafe { transmute(ERR) };
static mut _GetnMapdv: extern "system" fn(GLenum, GLenum, GLsizei, *mut GLdouble) = unsafe { transmute(ERR) };
static mut _GetnMapfv: extern "system" fn(GLenum, GLenum, GLsizei, *mut GLfloat) = unsafe { transmute(ERR) };
static mut _GetnMapiv: extern "system" fn(GLenum, GLenum, GLsizei, *mut GLint) = unsafe { transmute(ERR) };
static mut _GetnMinmax: extern "system" fn(GLenum, GLboolean, GLenum, GLenum, GLsizei, *mut c_void) = unsafe { transmute(ERR) };
static mut _GetnPixelMapfv: extern "system" fn(GLenum, GLsizei, *mut GLfloat) = unsafe { transmute(ERR) };
static mut _GetnPixelMapuiv: extern "system" fn(GLenum, GLsizei, *mut GLuint) = unsafe { transmute(ERR) };
static mut _GetnPixelMapusv: extern "system" fn(GLenum, GLsizei, *mut GLushort) = unsafe { transmute(ERR) };
static mut _GetnPolygonStipple: extern "system" fn(GLsizei, *mut GLubyte) = unsafe { transmute(ERR) };
static mut _GetnSeparableFilter: extern "system" fn(GLenum, GLenum, GLenum, GLsizei, *mut c_void, GLsizei, *mut c_void, *mut c_void) = unsafe { transmute(ERR) };
static mut _GetnTexImage: extern "system" fn(GLenum, GLint, GLenum, GLenum, GLsizei, *mut c_void) = unsafe { transmute(ERR) };
static mut _GetnUniformdv: extern "system" fn(GLuint, GLint, GLsizei, *mut GLdouble) = unsafe { transmute(ERR) };
static mut _GetnUniformfv: extern "system" fn(GLuint, GLint, GLsizei, *mut GLfloat) = unsafe { transmute(ERR) };
static mut _GetnUniformiv: extern "system" fn(GLuint, GLint, GLsizei, *mut GLint) = unsafe { transmute(ERR) };
static mut _GetnUniformuiv: extern "system" fn(GLuint, GLint, GLsizei, *mut GLuint) = unsafe { transmute(ERR) };
static mut _Hint: extern "system" fn(GLenum, GLenum) = unsafe { transmute(ERR) };
static mut _InvalidateBufferData: extern "system" fn(GLuint) = unsafe { transmute(ERR) };
static mut _InvalidateBufferSubData: extern "system" fn(GLuint, GLintptr, GLsizeiptr) = unsafe { transmute(ERR) };
static mut _InvalidateFramebuffer: extern "system" fn(GLenum, GLsizei, *const GLenum) = unsafe { transmute(ERR) };
static mut _InvalidateNamedFramebufferData: extern "system" fn(GLuint, GLsizei, *const GLenum) = unsafe { transmute(ERR) };
static mut _InvalidateNamedFramebufferSubData: extern "system" fn(GLuint, GLsizei, *const GLenum, GLint, GLint, GLsizei, GLsizei) = unsafe { transmute(ERR) };
static mut _InvalidateSubFramebuffer: extern "system" fn(GLenum, GLsizei, *const GLenum, GLint, GLint, GLsizei, GLsizei) = unsafe { transmute(ERR) };
static mut _InvalidateTexImage: extern "system" fn(GLuint, GLint) = unsafe { transmute(ERR) };
static mut _InvalidateTexSubImage: extern "system" fn(GLuint, GLint, GLint, GLint, GLint, GLsizei, GLsizei, GLsizei) = unsafe { transmute(ERR) };
static mut _IsBuffer: extern "system" fn(GLuint) -> GLboolean = unsafe { transmute(ERR) };
static mut _IsEnabled: extern "system" fn(GLenum) -> GLboolean = unsafe { transmute(ERR) };
static mut _IsEnabledi: extern "system" fn(GLenum, GLuint) -> GLboolean = unsafe { transmute(ERR) };
static mut _IsFramebuffer: extern "system" fn(GLuint) -> GLboolean = unsafe { transmute(ERR) };
static mut _IsProgram: extern "system" fn(GLuint) -> GLboolean = unsafe { transmute(ERR) };
static mut _IsProgramPipeline: extern "system" fn(GLuint) -> GLboolean = unsafe { transmute(ERR) };
static mut _IsQuery: extern "system" fn(GLuint) -> GLboolean = unsafe { transmute(ERR) };
static mut _IsRenderbuffer: extern "system" fn(GLuint) -> GLboolean = unsafe { transmute(ERR) };
static mut _IsSampler: extern "system" fn(GLuint) -> GLboolean = unsafe { transmute(ERR) };
static mut _IsShader: extern "system" fn(GLuint) -> GLboolean = unsafe { transmute(ERR) };
static mut _IsSync: extern "system" fn(GLsync) -> GLboolean = unsafe { transmute(ERR) };
static mut _IsTexture: extern "system" fn(GLuint) -> GLboolean = unsafe { transmute(ERR) };
static mut _IsTransformFeedback: extern "system" fn(GLuint) -> GLboolean = unsafe { transmute(ERR) };
static mut _IsVertexArray: extern "system" fn(GLuint) -> GLboolean = unsafe { transmute(ERR) };
static mut _LineWidth: extern "system" fn(GLfloat) = unsafe { transmute(ERR) };
static mut _LinkProgram: extern "system" fn(GLuint) = unsafe { transmute(ERR) };
static mut _LogicOp: extern "system" fn(GLenum) = unsafe { transmute(ERR) };
static mut _MapBuffer: extern "system" fn(GLenum, GLenum) -> *mut c_void = unsafe { transmute(ERR) };
static mut _MapBufferRange: extern "system" fn(GLenum, GLintptr, GLsizeiptr, GLbitfield) -> *mut c_void = unsafe { transmute(ERR) };
static mut _MapNamedBuffer: extern "system" fn(GLuint, GLenum) -> *mut c_void = unsafe { transmute(ERR) };
static mut _MapNamedBufferRange: extern "system" fn(GLuint, GLintptr, GLsizeiptr, GLbitfield) -> *mut c_void = unsafe { transmute(ERR) };
static mut _MemoryBarrier: extern "system" fn(GLbitfield) = unsafe { transmute(ERR) };
static mut _MemoryBarrierByRegion: extern "system" fn(GLbitfield) = unsafe { transmute(ERR) };
static mut _MinSampleShading: extern "system" fn(GLfloat) = unsafe { transmute(ERR) };
static mut _MultiDrawArrays: extern "system" fn(GLenum, *const GLint, *const GLsizei, GLsizei) = unsafe { transmute(ERR) };
static mut _MultiDrawArraysIndirect: extern "system" fn(GLenum, *const c_void, GLsizei, GLsizei) = unsafe { transmute(ERR) };
static mut _MultiDrawArraysIndirectCount: extern "system" fn(GLenum, *const c_void, GLintptr, GLsizei, GLsizei) = unsafe { transmute(ERR) };
static mut _MultiDrawElements: extern "system" fn(GLenum, *const GLsizei, GLenum, *const *const c_void, GLsizei) = unsafe { transmute(ERR) };
static mut _MultiDrawElementsBaseVertex: extern "system" fn(GLenum, *const GLsizei, GLenum, *const *const c_void, GLsizei, *const GLint) = unsafe { transmute(ERR) };
static mut _MultiDrawElementsIndirect: extern "system" fn(GLenum, GLenum, *const c_void, GLsizei, GLsizei) = unsafe { transmute(ERR) };
static mut _MultiDrawElementsIndirectCount: extern "system" fn(GLenum, GLenum, *const c_void, GLintptr, GLsizei, GLsizei) = unsafe { transmute(ERR) };
static mut _MultiTexCoordP1ui: extern "system" fn(GLenum, GLenum, GLuint) = unsafe { transmute(ERR) };
static mut _MultiTexCoordP1uiv: extern "system" fn(GLenum, GLenum, *const GLuint) = unsafe { transmute(ERR) };
static mut _MultiTexCoordP2ui: extern "system" fn(GLenum, GLenum, GLuint) = unsafe { transmute(ERR) };
static mut _MultiTexCoordP2uiv: extern "system" fn(GLenum, GLenum, *const GLuint) = unsafe { transmute(ERR) };
static mut _MultiTexCoordP3ui: extern "system" fn(GLenum, GLenum, GLuint) = unsafe { transmute(ERR) };
static mut _MultiTexCoordP3uiv: extern "system" fn(GLenum, GLenum, *const GLuint) = unsafe { transmute(ERR) };
static mut _MultiTexCoordP4ui: extern "system" fn(GLenum, GLenum, GLuint) = unsafe { transmute(ERR) };
static mut _MultiTexCoordP4uiv: extern "system" fn(GLenum, GLenum, *const GLuint) = unsafe { transmute(ERR) };
static mut _NamedBufferData: extern "system" fn(GLuint, GLsizeiptr, *const c_void, GLenum) = unsafe { transmute(ERR) };
static mut _NamedBufferStorage: extern "system" fn(GLuint, GLsizeiptr, *const c_void, GLbitfield) = unsafe { transmute(ERR) };
static mut _NamedBufferSubData: extern "system" fn(GLuint, GLintptr, GLsizeiptr, *const c_void) = unsafe { transmute(ERR) };
static mut _NamedFramebufferDrawBuffer: extern "system" fn(GLuint, GLenum) = unsafe { transmute(ERR) };
static mut _NamedFramebufferDrawBuffers: extern "system" fn(GLuint, GLsizei, *const GLenum) = unsafe { transmute(ERR) };
static mut _NamedFramebufferParameteri: extern "system" fn(GLuint, GLenum, GLint) = unsafe { transmute(ERR) };
static mut _NamedFramebufferReadBuffer: extern "system" fn(GLuint, GLenum) = unsafe { transmute(ERR) };
static mut _NamedFramebufferRenderbuffer: extern "system" fn(GLuint, GLenum, GLenum, GLuint) = unsafe { transmute(ERR) };
static mut _NamedFramebufferTexture: extern "system" fn(GLuint, GLenum, GLuint, GLint) = unsafe { transmute(ERR) };
static mut _NamedFramebufferTextureLayer: extern "system" fn(GLuint, GLenum, GLuint, GLint, GLint) = unsafe { transmute(ERR) };
static mut _NamedRenderbufferStorage: extern "system" fn(GLuint, GLenum, GLsizei, GLsizei) = unsafe { transmute(ERR) };
static mut _NamedRenderbufferStorageMultisample: extern "system" fn(GLuint, GLsizei, GLenum, GLsizei, GLsizei) = unsafe { transmute(ERR) };
static mut _NormalP3ui: extern "system" fn(GLenum, GLuint) = unsafe { transmute(ERR) };
static mut _NormalP3uiv: extern "system" fn(GLenum, *const GLuint) = unsafe { transmute(ERR) };
static mut _ObjectLabel: extern "system" fn(GLenum, GLuint, GLsizei, *const GLchar) = unsafe { transmute(ERR) };
static mut _ObjectPtrLabel: extern "system" fn(*const c_void, GLsizei, *const GLchar) = unsafe { transmute(ERR) };
static mut _PatchParameterfv: extern "system" fn(GLenum, *const GLfloat) = unsafe { transmute(ERR) };
static mut _PatchParameteri: extern "system" fn(GLenum, GLint) = unsafe { transmute(ERR) };
static mut _PauseTransformFeedback: extern "system" fn() = unsafe { transmute(ERR) };
static mut _PixelStoref: extern "system" fn(GLenum, GLfloat) = unsafe { transmute(ERR) };
static mut _PixelStorei: extern "system" fn(GLenum, GLint) = unsafe { transmute(ERR) };
static mut _PointParameterf: extern "system" fn(GLenum, GLfloat) = unsafe { transmute(ERR) };
static mut _PointParameterfv: extern "system" fn(GLenum, *const GLfloat) = unsafe { transmute(ERR) };
static mut _PointParameteri: extern "system" fn(GLenum, GLint) = unsafe { transmute(ERR) };
static mut _PointParameteriv: extern "system" fn(GLenum, *const GLint) = unsafe { transmute(ERR) };
static mut _PointSize: extern "system" fn(GLfloat) = unsafe { transmute(ERR) };
static mut _PolygonMode: extern "system" fn(GLenum, GLenum) = unsafe { transmute(ERR) };
static mut _PolygonOffset: extern "system" fn(GLfloat, GLfloat) = unsafe { transmute(ERR) };
static mut _PolygonOffsetClamp: extern "system" fn(GLfloat, GLfloat, GLfloat) = unsafe { transmute(ERR) };
static mut _PopDebugGroup: extern "system" fn() = unsafe { transmute(ERR) };
static mut _PrimitiveRestartIndex: extern "system" fn(GLuint) = unsafe { transmute(ERR) };
static mut _ProgramBinary: extern "system" fn(GLuint, GLenum, *const c_void, GLsizei) = unsafe { transmute(ERR) };
static mut _ProgramParameteri: extern "system" fn(GLuint, GLenum, GLint) = unsafe { transmute(ERR) };
static mut _ProgramUniform1d: extern "system" fn(GLuint, GLint, GLdouble) = unsafe { transmute(ERR) };
static mut _ProgramUniform1dv: extern "system" fn(GLuint, GLint, GLsizei, *const GLdouble) = unsafe { transmute(ERR) };
static mut _ProgramUniform1f: extern "system" fn(GLuint, GLint, GLfloat) = unsafe { transmute(ERR) };
static mut _ProgramUniform1fv: extern "system" fn(GLuint, GLint, GLsizei, *const GLfloat) = unsafe { transmute(ERR) };
static mut _ProgramUniform1i: extern "system" fn(GLuint, GLint, GLint) = unsafe { transmute(ERR) };
static mut _ProgramUniform1iv: extern "system" fn(GLuint, GLint, GLsizei, *const GLint) = unsafe { transmute(ERR) };
static mut _ProgramUniform1ui: extern "system" fn(GLuint, GLint, GLuint) = unsafe { transmute(ERR) };
static mut _ProgramUniform1uiv: extern "system" fn(GLuint, GLint, GLsizei, *const GLuint) = unsafe { transmute(ERR) };
static mut _ProgramUniform2d: extern "system" fn(GLuint, GLint, GLdouble, GLdouble) = unsafe { transmute(ERR) };
static mut _ProgramUniform2dv: extern "system" fn(GLuint, GLint, GLsizei, *const GLdouble) = unsafe { transmute(ERR) };
static mut _ProgramUniform2f: extern "system" fn(GLuint, GLint, GLfloat, GLfloat) = unsafe { transmute(ERR) };
static mut _ProgramUniform2fv: extern "system" fn(GLuint, GLint, GLsizei, *const GLfloat) = unsafe { transmute(ERR) };
static mut _ProgramUniform2i: extern "system" fn(GLuint, GLint, GLint, GLint) = unsafe { transmute(ERR) };
static mut _ProgramUniform2iv: extern "system" fn(GLuint, GLint, GLsizei, *const GLint) = unsafe { transmute(ERR) };
static mut _ProgramUniform2ui: extern "system" fn(GLuint, GLint, GLuint, GLuint) = unsafe { transmute(ERR) };
static mut _ProgramUniform2uiv: extern "system" fn(GLuint, GLint, GLsizei, *const GLuint) = unsafe { transmute(ERR) };
static mut _ProgramUniform3d: extern "system" fn(GLuint, GLint, GLdouble, GLdouble, GLdouble) = unsafe { transmute(ERR) };
static mut _ProgramUniform3dv: extern "system" fn(GLuint, GLint, GLsizei, *const GLdouble) = unsafe { transmute(ERR) };
static mut _ProgramUniform3f: extern "system" fn(GLuint, GLint, GLfloat, GLfloat, GLfloat) = unsafe { transmute(ERR) };
static mut _ProgramUniform3fv: extern "system" fn(GLuint, GLint, GLsizei, *const GLfloat) = unsafe { transmute(ERR) };
static mut _ProgramUniform3i: extern "system" fn(GLuint, GLint, GLint, GLint, GLint) = unsafe { transmute(ERR) };
static mut _ProgramUniform3iv: extern "system" fn(GLuint, GLint, GLsizei, *const GLint) = unsafe { transmute(ERR) };
static mut _ProgramUniform3ui: extern "system" fn(GLuint, GLint, GLuint, GLuint, GLuint) = unsafe { transmute(ERR) };
static mut _ProgramUniform3uiv: extern "system" fn(GLuint, GLint, GLsizei, *const GLuint) = unsafe { transmute(ERR) };
static mut _ProgramUniform4d: extern "system" fn(GLuint, GLint, GLdouble, GLdouble, GLdouble, GLdouble) = unsafe { transmute(ERR) };
static mut _ProgramUniform4dv: extern "system" fn(GLuint, GLint, GLsizei, *const GLdouble) = unsafe { transmute(ERR) };
static mut _ProgramUniform4f: extern "system" fn(GLuint, GLint, GLfloat, GLfloat, GLfloat, GLfloat) = unsafe { transmute(ERR) };
static mut _ProgramUniform4fv: extern "system" fn(GLuint, GLint, GLsizei, *const GLfloat) = unsafe { transmute(ERR) };
static mut _ProgramUniform4i: extern "system" fn(GLuint, GLint, GLint, GLint, GLint, GLint) = unsafe { transmute(ERR) };
static mut _ProgramUniform4iv: extern "system" fn(GLuint, GLint, GLsizei, *const GLint) = unsafe { transmute(ERR) };
static mut _ProgramUniform4ui: extern "system" fn(GLuint, GLint, GLuint, GLuint, GLuint, GLuint) = unsafe { transmute(ERR) };
static mut _ProgramUniform4uiv: extern "system" fn(GLuint, GLint, GLsizei, *const GLuint) = unsafe { transmute(ERR) };
static mut _ProgramUniformMatrix2dv: extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLdouble) = unsafe { transmute(ERR) };
static mut _ProgramUniformMatrix2fv: extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLfloat) = unsafe { transmute(ERR) };
static mut _ProgramUniformMatrix2x3dv: extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLdouble) = unsafe { transmute(ERR) };
static mut _ProgramUniformMatrix2x3fv: extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLfloat) = unsafe { transmute(ERR) };
static mut _ProgramUniformMatrix2x4dv: extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLdouble) = unsafe { transmute(ERR) };
static mut _ProgramUniformMatrix2x4fv: extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLfloat) = unsafe { transmute(ERR) };
static mut _ProgramUniformMatrix3dv: extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLdouble) = unsafe { transmute(ERR) };
static mut _ProgramUniformMatrix3fv: extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLfloat) = unsafe { transmute(ERR) };
static mut _ProgramUniformMatrix3x2dv: extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLdouble) = unsafe { transmute(ERR) };
static mut _ProgramUniformMatrix3x2fv: extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLfloat) = unsafe { transmute(ERR) };
static mut _ProgramUniformMatrix3x4dv: extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLdouble) = unsafe { transmute(ERR) };
static mut _ProgramUniformMatrix3x4fv: extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLfloat) = unsafe { transmute(ERR) };
static mut _ProgramUniformMatrix4dv: extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLdouble) = unsafe { transmute(ERR) };
static mut _ProgramUniformMatrix4fv: extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLfloat) = unsafe { transmute(ERR) };
static mut _ProgramUniformMatrix4x2dv: extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLdouble) = unsafe { transmute(ERR) };
static mut _ProgramUniformMatrix4x2fv: extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLfloat) = unsafe { transmute(ERR) };
static mut _ProgramUniformMatrix4x3dv: extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLdouble) = unsafe { transmute(ERR) };
static mut _ProgramUniformMatrix4x3fv: extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLfloat) = unsafe { transmute(ERR) };
static mut _ProvokingVertex: extern "system" fn(GLenum) = unsafe { transmute(ERR) };
static mut _PushDebugGroup: extern "system" fn(GLenum, GLuint, GLsizei, *const GLchar) = unsafe { transmute(ERR) };
static mut _QueryCounter: extern "system" fn(GLuint, GLenum) = unsafe { transmute(ERR) };
static mut _ReadBuffer: extern "system" fn(GLenum) = unsafe { transmute(ERR) };
static mut _ReadPixels: extern "system" fn(GLint, GLint, GLsizei, GLsizei, GLenum, GLenum, *mut c_void) = unsafe { transmute(ERR) };
static mut _ReadnPixels: extern "system" fn(GLint, GLint, GLsizei, GLsizei, GLenum, GLenum, GLsizei, *mut c_void) = unsafe { transmute(ERR) };
static mut _ReleaseShaderCompiler: extern "system" fn() = unsafe { transmute(ERR) };
static mut _RenderbufferStorage: extern "system" fn(GLenum, GLenum, GLsizei, GLsizei) = unsafe { transmute(ERR) };
static mut _RenderbufferStorageMultisample: extern "system" fn(GLenum, GLsizei, GLenum, GLsizei, GLsizei) = unsafe { transmute(ERR) };
static mut _ResumeTransformFeedback: extern "system" fn() = unsafe { transmute(ERR) };
static mut _SampleCoverage: extern "system" fn(GLfloat, GLboolean) = unsafe { transmute(ERR) };
static mut _SampleMaski: extern "system" fn(GLuint, GLbitfield) = unsafe { transmute(ERR) };
static mut _SamplerParameterIiv: extern "system" fn(GLuint, GLenum, *const GLint) = unsafe { transmute(ERR) };
static mut _SamplerParameterIuiv: extern "system" fn(GLuint, GLenum, *const GLuint) = unsafe { transmute(ERR) };
static mut _SamplerParameterf: extern "system" fn(GLuint, GLenum, GLfloat) = unsafe { transmute(ERR) };
static mut _SamplerParameterfv: extern "system" fn(GLuint, GLenum, *const GLfloat) = unsafe { transmute(ERR) };
static mut _SamplerParameteri: extern "system" fn(GLuint, GLenum, GLint) = unsafe { transmute(ERR) };
static mut _SamplerParameteriv: extern "system" fn(GLuint, GLenum, *const GLint) = unsafe { transmute(ERR) };
static mut _Scissor: extern "system" fn(GLint, GLint, GLsizei, GLsizei) = unsafe { transmute(ERR) };
static mut _ScissorArrayv: extern "system" fn(GLuint, GLsizei, *const GLint) = unsafe { transmute(ERR) };
static mut _ScissorIndexed: extern "system" fn(GLuint, GLint, GLint, GLsizei, GLsizei) = unsafe { transmute(ERR) };
static mut _ScissorIndexedv: extern "system" fn(GLuint, *const GLint) = unsafe { transmute(ERR) };
static mut _SecondaryColorP3ui: extern "system" fn(GLenum, GLuint) = unsafe { transmute(ERR) };
static mut _SecondaryColorP3uiv: extern "system" fn(GLenum, *const GLuint) = unsafe { transmute(ERR) };
static mut _ShaderBinary: extern "system" fn(GLsizei, *const GLuint, GLenum, *const c_void, GLsizei) = unsafe { transmute(ERR) };
static mut _ShaderSource: extern "system" fn(GLuint, GLsizei, *const *const GLchar, *const GLint) = unsafe { transmute(ERR) };
static mut _ShaderStorageBlockBinding: extern "system" fn(GLuint, GLuint, GLuint) = unsafe { transmute(ERR) };
static mut _SpecializeShader: extern "system" fn(GLuint, *const GLchar, GLuint, *const GLuint, *const GLuint) = unsafe { transmute(ERR) };
static mut _StencilFunc: extern "system" fn(GLenum, GLint, GLuint) = unsafe { transmute(ERR) };
static mut _StencilFuncSeparate: extern "system" fn(GLenum, GLenum, GLint, GLuint) = unsafe { transmute(ERR) };
static mut _StencilMask: extern "system" fn(GLuint) = unsafe { transmute(ERR) };
static mut _StencilMaskSeparate: extern "system" fn(GLenum, GLuint) = unsafe { transmute(ERR) };
static mut _StencilOp: extern "system" fn(GLenum, GLenum, GLenum) = unsafe { transmute(ERR) };
static mut _StencilOpSeparate: extern "system" fn(GLenum, GLenum, GLenum, GLenum) = unsafe { transmute(ERR) };
static mut _TexBuffer: extern "system" fn(GLenum, GLenum, GLuint) = unsafe { transmute(ERR) };
static mut _TexBufferRange: extern "system" fn(GLenum, GLenum, GLuint, GLintptr, GLsizeiptr) = unsafe { transmute(ERR) };
static mut _TexCoordP1ui: extern "system" fn(GLenum, GLuint) = unsafe { transmute(ERR) };
static mut _TexCoordP1uiv: extern "system" fn(GLenum, *const GLuint) = unsafe { transmute(ERR) };
static mut _TexCoordP2ui: extern "system" fn(GLenum, GLuint) = unsafe { transmute(ERR) };
static mut _TexCoordP2uiv: extern "system" fn(GLenum, *const GLuint) = unsafe { transmute(ERR) };
static mut _TexCoordP3ui: extern "system" fn(GLenum, GLuint) = unsafe { transmute(ERR) };
static mut _TexCoordP3uiv: extern "system" fn(GLenum, *const GLuint) = unsafe { transmute(ERR) };
static mut _TexCoordP4ui: extern "system" fn(GLenum, GLuint) = unsafe { transmute(ERR) };
static mut _TexCoordP4uiv: extern "system" fn(GLenum, *const GLuint) = unsafe { transmute(ERR) };
static mut _TexImage1D: extern "system" fn(GLenum, GLint, GLint, GLsizei, GLint, GLenum, GLenum, *const c_void) = unsafe { transmute(ERR) };
static mut _TexImage2D: extern "system" fn(GLenum, GLint, GLint, GLsizei, GLsizei, GLint, GLenum, GLenum, *const c_void) = unsafe { transmute(ERR) };
static mut _TexImage2DMultisample: extern "system" fn(GLenum, GLsizei, GLenum, GLsizei, GLsizei, GLboolean) = unsafe { transmute(ERR) };
static mut _TexImage3D: extern "system" fn(GLenum, GLint, GLint, GLsizei, GLsizei, GLsizei, GLint, GLenum, GLenum, *const c_void) = unsafe { transmute(ERR) };
static mut _TexImage3DMultisample: extern "system" fn(GLenum, GLsizei, GLenum, GLsizei, GLsizei, GLsizei, GLboolean) = unsafe { transmute(ERR) };
static mut _TexParameterIiv: extern "system" fn(GLenum, GLenum, *const GLint) = unsafe { transmute(ERR) };
static mut _TexParameterIuiv: extern "system" fn(GLenum, GLenum, *const GLuint) = unsafe { transmute(ERR) };
static mut _TexParameterf: extern "system" fn(GLenum, GLenum, GLfloat) = unsafe { transmute(ERR) };
static mut _TexParameterfv: extern "system" fn(GLenum, GLenum, *const GLfloat) = unsafe { transmute(ERR) };
static mut _TexParameteri: extern "system" fn(GLenum, GLenum, GLint) = unsafe { transmute(ERR) };
static mut _TexParameteriv: extern "system" fn(GLenum, GLenum, *const GLint) = unsafe { transmute(ERR) };
static mut _TexStorage1D: extern "system" fn(GLenum, GLsizei, GLenum, GLsizei) = unsafe { transmute(ERR) };
static mut _TexStorage2D: extern "system" fn(GLenum, GLsizei, GLenum, GLsizei, GLsizei) = unsafe { transmute(ERR) };
static mut _TexStorage2DMultisample: extern "system" fn(GLenum, GLsizei, GLenum, GLsizei, GLsizei, GLboolean) = unsafe { transmute(ERR) };
static mut _TexStorage3D: extern "system" fn(GLenum, GLsizei, GLenum, GLsizei, GLsizei, GLsizei) = unsafe { transmute(ERR) };
static mut _TexStorage3DMultisample: extern "system" fn(GLenum, GLsizei, GLenum, GLsizei, GLsizei, GLsizei, GLboolean) = unsafe { transmute(ERR) };
static mut _TexSubImage1D: extern "system" fn(GLenum, GLint, GLint, GLsizei, GLenum, GLenum, *const c_void) = unsafe { transmute(ERR) };
static mut _TexSubImage2D: extern "system" fn(GLenum, GLint, GLint, GLint, GLsizei, GLsizei, GLenum, GLenum, *const c_void) = unsafe { transmute(ERR) };
static mut _TexSubImage3D: extern "system" fn(GLenum, GLint, GLint, GLint, GLint, GLsizei, GLsizei, GLsizei, GLenum, GLenum, *const c_void) = unsafe { transmute(ERR) };
static mut _TextureBarrier: extern "system" fn() = unsafe { transmute(ERR) };
static mut _TextureBuffer: extern "system" fn(GLuint, GLenum, GLuint) = unsafe { transmute(ERR) };
static mut _TextureBufferRange: extern "system" fn(GLuint, GLenum, GLuint, GLintptr, GLsizeiptr) = unsafe { transmute(ERR) };
static mut _TextureParameterIiv: extern "system" fn(GLuint, GLenum, *const GLint) = unsafe { transmute(ERR) };
static mut _TextureParameterIuiv: extern "system" fn(GLuint, GLenum, *const GLuint) = unsafe { transmute(ERR) };
static mut _TextureParameterf: extern "system" fn(GLuint, GLenum, GLfloat) = unsafe { transmute(ERR) };
static mut _TextureParameterfv: extern "system" fn(GLuint, GLenum, *const GLfloat) = unsafe { transmute(ERR) };
static mut _TextureParameteri: extern "system" fn(GLuint, GLenum, GLint) = unsafe { transmute(ERR) };
static mut _TextureParameteriv: extern "system" fn(GLuint, GLenum, *const GLint) = unsafe { transmute(ERR) };
static mut _TextureStorage1D: extern "system" fn(GLuint, GLsizei, GLenum, GLsizei) = unsafe { transmute(ERR) };
static mut _TextureStorage2D: extern "system" fn(GLuint, GLsizei, GLenum, GLsizei, GLsizei) = unsafe { transmute(ERR) };
static mut _TextureStorage2DMultisample: extern "system" fn(GLuint, GLsizei, GLenum, GLsizei, GLsizei, GLboolean) = unsafe { transmute(ERR) };
static mut _TextureStorage3D: extern "system" fn(GLuint, GLsizei, GLenum, GLsizei, GLsizei, GLsizei) = unsafe { transmute(ERR) };
static mut _TextureStorage3DMultisample: extern "system" fn(GLuint, GLsizei, GLenum, GLsizei, GLsizei, GLsizei, GLboolean) = unsafe { transmute(ERR) };
static mut _TextureSubImage1D: extern "system" fn(GLuint, GLint, GLint, GLsizei, GLenum, GLenum, *const c_void) = unsafe { transmute(ERR) };
static mut _TextureSubImage2D: extern "system" fn(GLuint, GLint, GLint, GLint, GLsizei, GLsizei, GLenum, GLenum, *const c_void) = unsafe { transmute(ERR) };
static mut _TextureSubImage3D: extern "system" fn(GLuint, GLint, GLint, GLint, GLint, GLsizei, GLsizei, GLsizei, GLenum, GLenum, *const c_void) = unsafe { transmute(ERR) };
static mut _TextureView: extern "system" fn(GLuint, GLenum, GLuint, GLenum, GLuint, GLuint, GLuint, GLuint) = unsafe { transmute(ERR) };
static mut _TransformFeedbackBufferBase: extern "system" fn(GLuint, GLuint, GLuint) = unsafe { transmute(ERR) };
static mut _TransformFeedbackBufferRange: extern "system" fn(GLuint, GLuint, GLuint, GLintptr, GLsizeiptr) = unsafe { transmute(ERR) };
static mut _TransformFeedbackVaryings: extern "system" fn(GLuint, GLsizei, *const *const GLchar, GLenum) = unsafe { transmute(ERR) };
static mut _Uniform1d: extern "system" fn(GLint, GLdouble) = unsafe { transmute(ERR) };
static mut _Uniform1dv: extern "system" fn(GLint, GLsizei, *const GLdouble) = unsafe { transmute(ERR) };
static mut _Uniform1f: extern "system" fn(GLint, GLfloat) = unsafe { transmute(ERR) };
static mut _Uniform1fv: extern "system" fn(GLint, GLsizei, *const GLfloat) = unsafe { transmute(ERR) };
static mut _Uniform1i: extern "system" fn(GLint, GLint) = unsafe { transmute(ERR) };
static mut _Uniform1iv: extern "system" fn(GLint, GLsizei, *const GLint) = unsafe { transmute(ERR) };
static mut _Uniform1ui: extern "system" fn(GLint, GLuint) = unsafe { transmute(ERR) };
static mut _Uniform1uiv: extern "system" fn(GLint, GLsizei, *const GLuint) = unsafe { transmute(ERR) };
static mut _Uniform2d: extern "system" fn(GLint, GLdouble, GLdouble) = unsafe { transmute(ERR) };
static mut _Uniform2dv: extern "system" fn(GLint, GLsizei, *const GLdouble) = unsafe { transmute(ERR) };
static mut _Uniform2f: extern "system" fn(GLint, GLfloat, GLfloat) = unsafe { transmute(ERR) };
static mut _Uniform2fv: extern "system" fn(GLint, GLsizei, *const GLfloat) = unsafe { transmute(ERR) };
static mut _Uniform2i: extern "system" fn(GLint, GLint, GLint) = unsafe { transmute(ERR) };
static mut _Uniform2iv: extern "system" fn(GLint, GLsizei, *const GLint) = unsafe { transmute(ERR) };
static mut _Uniform2ui: extern "system" fn(GLint, GLuint, GLuint) = unsafe { transmute(ERR) };
static mut _Uniform2uiv: extern "system" fn(GLint, GLsizei, *const GLuint) = unsafe { transmute(ERR) };
static mut _Uniform3d: extern "system" fn(GLint, GLdouble, GLdouble, GLdouble) = unsafe { transmute(ERR) };
static mut _Uniform3dv: extern "system" fn(GLint, GLsizei, *const GLdouble) = unsafe { transmute(ERR) };
static mut _Uniform3f: extern "system" fn(GLint, GLfloat, GLfloat, GLfloat) = unsafe { transmute(ERR) };
static mut _Uniform3fv: extern "system" fn(GLint, GLsizei, *const GLfloat) = unsafe { transmute(ERR) };
static mut _Uniform3i: extern "system" fn(GLint, GLint, GLint, GLint) = unsafe { transmute(ERR) };
static mut _Uniform3iv: extern "system" fn(GLint, GLsizei, *const GLint) = unsafe { transmute(ERR) };
static mut _Uniform3ui: extern "system" fn(GLint, GLuint, GLuint, GLuint) = unsafe { transmute(ERR) };
static mut _Uniform3uiv: extern "system" fn(GLint, GLsizei, *const GLuint) = unsafe { transmute(ERR) };
static mut _Uniform4d: extern "system" fn(GLint, GLdouble, GLdouble, GLdouble, GLdouble) = unsafe { transmute(ERR) };
static mut _Uniform4dv: extern "system" fn(GLint, GLsizei, *const GLdouble) = unsafe { transmute(ERR) };
static mut _Uniform4f: extern "system" fn(GLint, GLfloat, GLfloat, GLfloat, GLfloat) = unsafe { transmute(ERR) };
static mut _Uniform4fv: extern "system" fn(GLint, GLsizei, *const GLfloat) = unsafe { transmute(ERR) };
static mut _Uniform4i: extern "system" fn(GLint, GLint, GLint, GLint, GLint) = unsafe { transmute(ERR) };
static mut _Uniform4iv: extern "system" fn(GLint, GLsizei, *const GLint) = unsafe { transmute(ERR) };
static mut _Uniform4ui: extern "system" fn(GLint, GLuint, GLuint, GLuint, GLuint) = unsafe { transmute(ERR) };
static mut _Uniform4uiv: extern "system" fn(GLint, GLsizei, *const GLuint) = unsafe { transmute(ERR) };
static mut _UniformBlockBinding: extern "system" fn(GLuint, GLuint, GLuint) = unsafe { transmute(ERR) };
static mut _UniformMatrix2dv: extern "system" fn(GLint, GLsizei, GLboolean, *const GLdouble) = unsafe { transmute(ERR) };
static mut _UniformMatrix2fv: extern "system" fn(GLint, GLsizei, GLboolean, *const GLfloat) = unsafe { transmute(ERR) };
static mut _UniformMatrix2x3dv: extern "system" fn(GLint, GLsizei, GLboolean, *const GLdouble) = unsafe { transmute(ERR) };
static mut _UniformMatrix2x3fv: extern "system" fn(GLint, GLsizei, GLboolean, *const GLfloat) = unsafe { transmute(ERR) };
static mut _UniformMatrix2x4dv: extern "system" fn(GLint, GLsizei, GLboolean, *const GLdouble) = unsafe { transmute(ERR) };
static mut _UniformMatrix2x4fv: extern "system" fn(GLint, GLsizei, GLboolean, *const GLfloat) = unsafe { transmute(ERR) };
static mut _UniformMatrix3dv: extern "system" fn(GLint, GLsizei, GLboolean, *const GLdouble) = unsafe { transmute(ERR) };
static mut _UniformMatrix3fv: extern "system" fn(GLint, GLsizei, GLboolean, *const GLfloat) = unsafe { transmute(ERR) };
static mut _UniformMatrix3x2dv: extern "system" fn(GLint, GLsizei, GLboolean, *const GLdouble) = unsafe { transmute(ERR) };
static mut _UniformMatrix3x2fv: extern "system" fn(GLint, GLsizei, GLboolean, *const GLfloat) = unsafe { transmute(ERR) };
static mut _UniformMatrix3x4dv: extern "system" fn(GLint, GLsizei, GLboolean, *const GLdouble) = unsafe { transmute(ERR) };
static mut _UniformMatrix3x4fv: extern "system" fn(GLint, GLsizei, GLboolean, *const GLfloat) = unsafe { transmute(ERR) };
static mut _UniformMatrix4dv: extern "system" fn(GLint, GLsizei, GLboolean, *const GLdouble) = unsafe { transmute(ERR) };
static mut _UniformMatrix4fv: extern "system" fn(GLint, GLsizei, GLboolean, *const GLfloat) = unsafe { transmute(ERR) };
static mut _UniformMatrix4x2dv: extern "system" fn(GLint, GLsizei, GLboolean, *const GLdouble) = unsafe { transmute(ERR) };
static mut _UniformMatrix4x2fv: extern "system" fn(GLint, GLsizei, GLboolean, *const GLfloat) = unsafe { transmute(ERR) };
static mut _UniformMatrix4x3dv: extern "system" fn(GLint, GLsizei, GLboolean, *const GLdouble) = unsafe { transmute(ERR) };
static mut _UniformMatrix4x3fv: extern "system" fn(GLint, GLsizei, GLboolean, *const GLfloat) = unsafe { transmute(ERR) };
static mut _UniformSubroutinesuiv: extern "system" fn(GLenum, GLsizei, *const GLuint) = unsafe { transmute(ERR) };
static mut _UnmapBuffer: extern "system" fn(GLenum) -> GLboolean = unsafe { transmute(ERR) };
static mut _UnmapNamedBuffer: extern "system" fn(GLuint) -> GLboolean = unsafe { transmute(ERR) };
static mut _UseProgram: extern "system" fn(GLuint) = unsafe { transmute(ERR) };
static mut _UseProgramStages: extern "system" fn(GLuint, GLbitfield, GLuint) = unsafe { transmute(ERR) };
static mut _ValidateProgram: extern "system" fn(GLuint) = unsafe { transmute(ERR) };
static mut _ValidateProgramPipeline: extern "system" fn(GLuint) = unsafe { transmute(ERR) };
static mut _VertexArrayAttribBinding: extern "system" fn(GLuint, GLuint, GLuint) = unsafe { transmute(ERR) };
static mut _VertexArrayAttribFormat: extern "system" fn(GLuint, GLuint, GLint, GLenum, GLboolean, GLuint) = unsafe { transmute(ERR) };
static mut _VertexArrayAttribIFormat: extern "system" fn(GLuint, GLuint, GLint, GLenum, GLuint) = unsafe { transmute(ERR) };
static mut _VertexArrayAttribLFormat: extern "system" fn(GLuint, GLuint, GLint, GLenum, GLuint) = unsafe { transmute(ERR) };
static mut _VertexArrayBindingDivisor: extern "system" fn(GLuint, GLuint, GLuint) = unsafe { transmute(ERR) };
static mut _VertexArrayElementBuffer: extern "system" fn(GLuint, GLuint) = unsafe { transmute(ERR) };
static mut _VertexArrayVertexBuffer: extern "system" fn(GLuint, GLuint, GLuint, GLintptr, GLsizei) = unsafe { transmute(ERR) };
static mut _VertexArrayVertexBuffers: extern "system" fn(GLuint, GLuint, GLsizei, *const GLuint, *const GLintptr, *const GLsizei) = unsafe { transmute(ERR) };
static mut _VertexAttrib1d: extern "system" fn(GLuint, GLdouble) = unsafe { transmute(ERR) };
static mut _VertexAttrib1dv: extern "system" fn(GLuint, *const GLdouble) = unsafe { transmute(ERR) };
static mut _VertexAttrib1f: extern "system" fn(GLuint, GLfloat) = unsafe { transmute(ERR) };
static mut _VertexAttrib1fv: extern "system" fn(GLuint, *const GLfloat) = unsafe { transmute(ERR) };
static mut _VertexAttrib1s: extern "system" fn(GLuint, GLshort) = unsafe { transmute(ERR) };
static mut _VertexAttrib1sv: extern "system" fn(GLuint, *const GLshort) = unsafe { transmute(ERR) };
static mut _VertexAttrib2d: extern "system" fn(GLuint, GLdouble, GLdouble) = unsafe { transmute(ERR) };
static mut _VertexAttrib2dv: extern "system" fn(GLuint, *const GLdouble) = unsafe { transmute(ERR) };
static mut _VertexAttrib2f: extern "system" fn(GLuint, GLfloat, GLfloat) = unsafe { transmute(ERR) };
static mut _VertexAttrib2fv: extern "system" fn(GLuint, *const GLfloat) = unsafe { transmute(ERR) };
static mut _VertexAttrib2s: extern "system" fn(GLuint, GLshort, GLshort) = unsafe { transmute(ERR) };
static mut _VertexAttrib2sv: extern "system" fn(GLuint, *const GLshort) = unsafe { transmute(ERR) };
static mut _VertexAttrib3d: extern "system" fn(GLuint, GLdouble, GLdouble, GLdouble) = unsafe { transmute(ERR) };
static mut _VertexAttrib3dv: extern "system" fn(GLuint, *const GLdouble) = unsafe { transmute(ERR) };
static mut _VertexAttrib3f: extern "system" fn(GLuint, GLfloat, GLfloat, GLfloat) = unsafe { transmute(ERR) };
static mut _VertexAttrib3fv: extern "system" fn(GLuint, *const GLfloat) = unsafe { transmute(ERR) };
static mut _VertexAttrib3s: extern "system" fn(GLuint, GLshort, GLshort, GLshort) = unsafe { transmute(ERR) };
static mut _VertexAttrib3sv: extern "system" fn(GLuint, *const GLshort) = unsafe { transmute(ERR) };
static mut _VertexAttrib4Nbv: extern "system" fn(GLuint, *const GLbyte) = unsafe { transmute(ERR) };
static mut _VertexAttrib4Niv: extern "system" fn(GLuint, *const GLint) = unsafe { transmute(ERR) };
static mut _VertexAttrib4Nsv: extern "system" fn(GLuint, *const GLshort) = unsafe { transmute(ERR) };
static mut _VertexAttrib4Nub: extern "system" fn(GLuint, GLubyte, GLubyte, GLubyte, GLubyte) = unsafe { transmute(ERR) };
static mut _VertexAttrib4Nubv: extern "system" fn(GLuint, *const GLubyte) = unsafe { transmute(ERR) };
static mut _VertexAttrib4Nuiv: extern "system" fn(GLuint, *const GLuint) = unsafe { transmute(ERR) };
static mut _VertexAttrib4Nusv: extern "system" fn(GLuint, *const GLushort) = unsafe { transmute(ERR) };
static mut _VertexAttrib4bv: extern "system" fn(GLuint, *const GLbyte) = unsafe { transmute(ERR) };
static mut _VertexAttrib4d: extern "system" fn(GLuint, GLdouble, GLdouble, GLdouble, GLdouble) = unsafe { transmute(ERR) };
static mut _VertexAttrib4dv: extern "system" fn(GLuint, *const GLdouble) = unsafe { transmute(ERR) };
static mut _VertexAttrib4f: extern "system" fn(GLuint, GLfloat, GLfloat, GLfloat, GLfloat) = unsafe { transmute(ERR) };
static mut _VertexAttrib4fv: extern "system" fn(GLuint, *const GLfloat) = unsafe { transmute(ERR) };
static mut _VertexAttrib4iv: extern "system" fn(GLuint, *const GLint) = unsafe { transmute(ERR) };
static mut _VertexAttrib4s: extern "system" fn(GLuint, GLshort, GLshort, GLshort, GLshort) = unsafe { transmute(ERR) };
static mut _VertexAttrib4sv: extern "system" fn(GLuint, *const GLshort) = unsafe { transmute(ERR) };
static mut _VertexAttrib4ubv: extern "system" fn(GLuint, *const GLubyte) = unsafe { transmute(ERR) };
static mut _VertexAttrib4uiv: extern "system" fn(GLuint, *const GLuint) = unsafe { transmute(ERR) };
static mut _VertexAttrib4usv: extern "system" fn(GLuint, *const GLushort) = unsafe { transmute(ERR) };
static mut _VertexAttribBinding: extern "system" fn(GLuint, GLuint) = unsafe { transmute(ERR) };
static mut _VertexAttribDivisor: extern "system" fn(GLuint, GLuint) = unsafe { transmute(ERR) };
static mut _VertexAttribFormat: extern "system" fn(GLuint, GLint, GLenum, GLboolean, GLuint) = unsafe { transmute(ERR) };
static mut _VertexAttribI1i: extern "system" fn(GLuint, GLint) = unsafe { transmute(ERR) };
static mut _VertexAttribI1iv: extern "system" fn(GLuint, *const GLint) = unsafe { transmute(ERR) };
static mut _VertexAttribI1ui: extern "system" fn(GLuint, GLuint) = unsafe { transmute(ERR) };
static mut _VertexAttribI1uiv: extern "system" fn(GLuint, *const GLuint) = unsafe { transmute(ERR) };
static mut _VertexAttribI2i: extern "system" fn(GLuint, GLint, GLint) = unsafe { transmute(ERR) };
static mut _VertexAttribI2iv: extern "system" fn(GLuint, *const GLint) = unsafe { transmute(ERR) };
static mut _VertexAttribI2ui: extern "system" fn(GLuint, GLuint, GLuint) = unsafe { transmute(ERR) };
static mut _VertexAttribI2uiv: extern "system" fn(GLuint, *const GLuint) = unsafe { transmute(ERR) };
static mut _VertexAttribI3i: extern "system" fn(GLuint, GLint, GLint, GLint) = unsafe { transmute(ERR) };
static mut _VertexAttribI3iv: extern "system" fn(GLuint, *const GLint) = unsafe { transmute(ERR) };
static mut _VertexAttribI3ui: extern "system" fn(GLuint, GLuint, GLuint, GLuint) = unsafe { transmute(ERR) };
static mut _VertexAttribI3uiv: extern "system" fn(GLuint, *const GLuint) = unsafe { transmute(ERR) };
static mut _VertexAttribI4bv: extern "system" fn(GLuint, *const GLbyte) = unsafe { transmute(ERR) };
static mut _VertexAttribI4i: extern "system" fn(GLuint, GLint, GLint, GLint, GLint) = unsafe { transmute(ERR) };
static mut _VertexAttribI4iv: extern "system" fn(GLuint, *const GLint) = unsafe { transmute(ERR) };
static mut _VertexAttribI4sv: extern "system" fn(GLuint, *const GLshort) = unsafe { transmute(ERR) };
static mut _VertexAttribI4ubv: extern "system" fn(GLuint, *const GLubyte) = unsafe { transmute(ERR) };
static mut _VertexAttribI4ui: extern "system" fn(GLuint, GLuint, GLuint, GLuint, GLuint) = unsafe { transmute(ERR) };
static mut _VertexAttribI4uiv: extern "system" fn(GLuint, *const GLuint) = unsafe { transmute(ERR) };
static mut _VertexAttribI4usv: extern "system" fn(GLuint, *const GLushort) = unsafe { transmute(ERR) };
static mut _VertexAttribIFormat: extern "system" fn(GLuint, GLint, GLenum, GLuint) = unsafe { transmute(ERR) };
static mut _VertexAttribIPointer: extern "system" fn(GLuint, GLint, GLenum, GLsizei, *const c_void) = unsafe { transmute(ERR) };
static mut _VertexAttribL1d: extern "system" fn(GLuint, GLdouble) = unsafe { transmute(ERR) };
static mut _VertexAttribL1dv: extern "system" fn(GLuint, *const GLdouble) = unsafe { transmute(ERR) };
static mut _VertexAttribL2d: extern "system" fn(GLuint, GLdouble, GLdouble) = unsafe { transmute(ERR) };
static mut _VertexAttribL2dv: extern "system" fn(GLuint, *const GLdouble) = unsafe { transmute(ERR) };
static mut _VertexAttribL3d: extern "system" fn(GLuint, GLdouble, GLdouble, GLdouble) = unsafe { transmute(ERR) };
static mut _VertexAttribL3dv: extern "system" fn(GLuint, *const GLdouble) = unsafe { transmute(ERR) };
static mut _VertexAttribL4d: extern "system" fn(GLuint, GLdouble, GLdouble, GLdouble, GLdouble) = unsafe { transmute(ERR) };
static mut _VertexAttribL4dv: extern "system" fn(GLuint, *const GLdouble) = unsafe { transmute(ERR) };
static mut _VertexAttribLFormat: extern "system" fn(GLuint, GLint, GLenum, GLuint) = unsafe { transmute(ERR) };
static mut _VertexAttribLPointer: extern "system" fn(GLuint, GLint, GLenum, GLsizei, *const c_void) = unsafe { transmute(ERR) };
static mut _VertexAttribP1ui: extern "system" fn(GLuint, GLenum, GLboolean, GLuint) = unsafe { transmute(ERR) };
static mut _VertexAttribP1uiv: extern "system" fn(GLuint, GLenum, GLboolean, *const GLuint) = unsafe { transmute(ERR) };
static mut _VertexAttribP2ui: extern "system" fn(GLuint, GLenum, GLboolean, GLuint) = unsafe { transmute(ERR) };
static mut _VertexAttribP2uiv: extern "system" fn(GLuint, GLenum, GLboolean, *const GLuint) = unsafe { transmute(ERR) };
static mut _VertexAttribP3ui: extern "system" fn(GLuint, GLenum, GLboolean, GLuint) = unsafe { transmute(ERR) };
static mut _VertexAttribP3uiv: extern "system" fn(GLuint, GLenum, GLboolean, *const GLuint) = unsafe { transmute(ERR) };
static mut _VertexAttribP4ui: extern "system" fn(GLuint, GLenum, GLboolean, GLuint) = unsafe { transmute(ERR) };
static mut _VertexAttribP4uiv: extern "system" fn(GLuint, GLenum, GLboolean, *const GLuint) = unsafe { transmute(ERR) };
static mut _VertexAttribPointer: extern "system" fn(GLuint, GLint, GLenum, GLboolean, GLsizei, *const c_void) = unsafe { transmute(ERR) };
static mut _VertexBindingDivisor: extern "system" fn(GLuint, GLuint) = unsafe { transmute(ERR) };
static mut _VertexP2ui: extern "system" fn(GLenum, GLuint) = unsafe { transmute(ERR) };
static mut _VertexP2uiv: extern "system" fn(GLenum, *const GLuint) = unsafe { transmute(ERR) };
static mut _VertexP3ui: extern "system" fn(GLenum, GLuint) = unsafe { transmute(ERR) };
static mut _VertexP3uiv: extern "system" fn(GLenum, *const GLuint) = unsafe { transmute(ERR) };
static mut _VertexP4ui: extern "system" fn(GLenum, GLuint) = unsafe { transmute(ERR) };
static mut _VertexP4uiv: extern "system" fn(GLenum, *const GLuint) = unsafe { transmute(ERR) };
static mut _Viewport: extern "system" fn(GLint, GLint, GLsizei, GLsizei) = unsafe { transmute(ERR) };
static mut _ViewportArrayv: extern "system" fn(GLuint, GLsizei, *const GLfloat) = unsafe { transmute(ERR) };
static mut _ViewportIndexedf: extern "system" fn(GLuint, GLfloat, GLfloat, GLfloat, GLfloat) = unsafe { transmute(ERR) };
static mut _ViewportIndexedfv: extern "system" fn(GLuint, *const GLfloat) = unsafe { transmute(ERR) };
static mut _WaitSync: extern "system" fn(GLsync, GLbitfield, GLuint64) = unsafe { transmute(ERR) };
static ERR: extern "system" fn() = gl_not_loaded;
		extern "system" fn gl_not_loaded() {
			panic!("GL Function not loaded")
		}#[inline(never)]
		unsafe fn load(func: &mut dyn FnMut(&'static str) -> *const c_void,
				   symbol: &'static str,
				   fallbacks: &[&'static str]) -> *const c_void {
			let mut ptr = func(symbol);
			if ptr.is_null() {
				for &sym in fallbacks {
					ptr = func(sym);
					if !ptr.is_null() { break; }
				}
			}
            if ptr.is_null() {
                ptr = transmute(ERR);
            }
			ptr
		}
		pub unsafe fn load_gl<F: FnMut(&'static str) -> *const c_void>(func: &mut F) {_ActiveShaderProgram = transmute(load(func, "ActiveShaderProgram", &[]));
_ActiveTexture = transmute(load(func, "ActiveTexture", &["ActiveTextureARB"]));
_AttachShader = transmute(load(func, "AttachShader", &["AttachObjectARB"]));
_BeginConditionalRender = transmute(load(func, "BeginConditionalRender", &["BeginConditionalRenderNV"]));
_BeginQuery = transmute(load(func, "BeginQuery", &["BeginQueryARB"]));
_BeginQueryIndexed = transmute(load(func, "BeginQueryIndexed", &[]));
_BeginTransformFeedback = transmute(load(func, "BeginTransformFeedback", &["BeginTransformFeedbackEXT", "BeginTransformFeedbackNV"]));
_BindAttribLocation = transmute(load(func, "BindAttribLocation", &["BindAttribLocationARB"]));
_BindBuffer = transmute(load(func, "BindBuffer", &["BindBufferARB"]));
_BindBufferBase = transmute(load(func, "BindBufferBase", &["BindBufferBaseEXT", "BindBufferBaseNV"]));
_BindBufferRange = transmute(load(func, "BindBufferRange", &["BindBufferRangeEXT", "BindBufferRangeNV"]));
_BindBuffersBase = transmute(load(func, "BindBuffersBase", &[]));
_BindBuffersRange = transmute(load(func, "BindBuffersRange", &[]));
_BindFragDataLocation = transmute(load(func, "BindFragDataLocation", &["BindFragDataLocationEXT"]));
_BindFragDataLocationIndexed = transmute(load(func, "BindFragDataLocationIndexed", &["BindFragDataLocationIndexedEXT"]));
_BindFramebuffer = transmute(load(func, "BindFramebuffer", &[]));
_BindImageTexture = transmute(load(func, "BindImageTexture", &[]));
_BindImageTextures = transmute(load(func, "BindImageTextures", &[]));
_BindProgramPipeline = transmute(load(func, "BindProgramPipeline", &[]));
_BindRenderbuffer = transmute(load(func, "BindRenderbuffer", &[]));
_BindSampler = transmute(load(func, "BindSampler", &[]));
_BindSamplers = transmute(load(func, "BindSamplers", &[]));
_BindTexture = transmute(load(func, "BindTexture", &["BindTextureEXT"]));
_BindTextureUnit = transmute(load(func, "BindTextureUnit", &[]));
_BindTextures = transmute(load(func, "BindTextures", &[]));
_BindTransformFeedback = transmute(load(func, "BindTransformFeedback", &[]));
_BindVertexArray = transmute(load(func, "BindVertexArray", &["BindVertexArrayOES"]));
_BindVertexBuffer = transmute(load(func, "BindVertexBuffer", &[]));
_BindVertexBuffers = transmute(load(func, "BindVertexBuffers", &[]));
_BlendColor = transmute(load(func, "BlendColor", &["BlendColorEXT"]));
_BlendEquation = transmute(load(func, "BlendEquation", &["BlendEquationEXT"]));
_BlendEquationSeparate = transmute(load(func, "BlendEquationSeparate", &["BlendEquationSeparateEXT"]));
_BlendEquationSeparatei = transmute(load(func, "BlendEquationSeparatei", &["BlendEquationSeparateIndexedAMD", "BlendEquationSeparateiARB", "BlendEquationSeparateiEXT", "BlendEquationSeparateiOES"]));
_BlendEquationi = transmute(load(func, "BlendEquationi", &["BlendEquationIndexedAMD", "BlendEquationiARB", "BlendEquationiEXT", "BlendEquationiOES"]));
_BlendFunc = transmute(load(func, "BlendFunc", &[]));
_BlendFuncSeparate = transmute(load(func, "BlendFuncSeparate", &["BlendFuncSeparateEXT", "BlendFuncSeparateINGR"]));
_BlendFuncSeparatei = transmute(load(func, "BlendFuncSeparatei", &["BlendFuncSeparateIndexedAMD", "BlendFuncSeparateiARB", "BlendFuncSeparateiEXT", "BlendFuncSeparateiOES"]));
_BlendFunci = transmute(load(func, "BlendFunci", &["BlendFuncIndexedAMD", "BlendFunciARB", "BlendFunciEXT", "BlendFunciOES"]));
_BlitFramebuffer = transmute(load(func, "BlitFramebuffer", &["BlitFramebufferEXT", "BlitFramebufferNV"]));
_BlitNamedFramebuffer = transmute(load(func, "BlitNamedFramebuffer", &[]));
_BufferData = transmute(load(func, "BufferData", &["BufferDataARB"]));
_BufferStorage = transmute(load(func, "BufferStorage", &["BufferStorageEXT"]));
_BufferSubData = transmute(load(func, "BufferSubData", &["BufferSubDataARB"]));
_CheckFramebufferStatus = transmute(load(func, "CheckFramebufferStatus", &["CheckFramebufferStatusEXT"]));
_CheckNamedFramebufferStatus = transmute(load(func, "CheckNamedFramebufferStatus", &[]));
_ClampColor = transmute(load(func, "ClampColor", &["ClampColorARB"]));
_Clear = transmute(load(func, "Clear", &[]));
_ClearBufferData = transmute(load(func, "ClearBufferData", &[]));
_ClearBufferSubData = transmute(load(func, "ClearBufferSubData", &[]));
_ClearBufferfi = transmute(load(func, "ClearBufferfi", &[]));
_ClearBufferfv = transmute(load(func, "ClearBufferfv", &[]));
_ClearBufferiv = transmute(load(func, "ClearBufferiv", &[]));
_ClearBufferuiv = transmute(load(func, "ClearBufferuiv", &[]));
_ClearColor = transmute(load(func, "ClearColor", &[]));
_ClearDepth = transmute(load(func, "ClearDepth", &[]));
_ClearDepthf = transmute(load(func, "ClearDepthf", &["ClearDepthfOES"]));
_ClearNamedBufferData = transmute(load(func, "ClearNamedBufferData", &[]));
_ClearNamedBufferSubData = transmute(load(func, "ClearNamedBufferSubData", &[]));
_ClearNamedFramebufferfi = transmute(load(func, "ClearNamedFramebufferfi", &[]));
_ClearNamedFramebufferfv = transmute(load(func, "ClearNamedFramebufferfv", &[]));
_ClearNamedFramebufferiv = transmute(load(func, "ClearNamedFramebufferiv", &[]));
_ClearNamedFramebufferuiv = transmute(load(func, "ClearNamedFramebufferuiv", &[]));
_ClearStencil = transmute(load(func, "ClearStencil", &[]));
_ClearTexImage = transmute(load(func, "ClearTexImage", &["ClearTexImageEXT"]));
_ClearTexSubImage = transmute(load(func, "ClearTexSubImage", &["ClearTexSubImageEXT"]));
_ClientWaitSync = transmute(load(func, "ClientWaitSync", &["ClientWaitSyncAPPLE"]));
_ClipControl = transmute(load(func, "ClipControl", &["ClipControlEXT"]));
_ColorMask = transmute(load(func, "ColorMask", &[]));
_ColorMaski = transmute(load(func, "ColorMaski", &["ColorMaskIndexedEXT", "ColorMaskiEXT", "ColorMaskiOES"]));
_ColorP3ui = transmute(load(func, "ColorP3ui", &[]));
_ColorP3uiv = transmute(load(func, "ColorP3uiv", &[]));
_ColorP4ui = transmute(load(func, "ColorP4ui", &[]));
_ColorP4uiv = transmute(load(func, "ColorP4uiv", &[]));
_CompileShader = transmute(load(func, "CompileShader", &["CompileShaderARB"]));
_CompressedTexImage1D = transmute(load(func, "CompressedTexImage1D", &["CompressedTexImage1DARB"]));
_CompressedTexImage2D = transmute(load(func, "CompressedTexImage2D", &["CompressedTexImage2DARB"]));
_CompressedTexImage3D = transmute(load(func, "CompressedTexImage3D", &["CompressedTexImage3DARB"]));
_CompressedTexSubImage1D = transmute(load(func, "CompressedTexSubImage1D", &["CompressedTexSubImage1DARB"]));
_CompressedTexSubImage2D = transmute(load(func, "CompressedTexSubImage2D", &["CompressedTexSubImage2DARB"]));
_CompressedTexSubImage3D = transmute(load(func, "CompressedTexSubImage3D", &["CompressedTexSubImage3DARB"]));
_CompressedTextureSubImage1D = transmute(load(func, "CompressedTextureSubImage1D", &[]));
_CompressedTextureSubImage2D = transmute(load(func, "CompressedTextureSubImage2D", &[]));
_CompressedTextureSubImage3D = transmute(load(func, "CompressedTextureSubImage3D", &[]));
_CopyBufferSubData = transmute(load(func, "CopyBufferSubData", &["CopyBufferSubDataNV"]));
_CopyImageSubData = transmute(load(func, "CopyImageSubData", &["CopyImageSubDataEXT", "CopyImageSubDataOES"]));
_CopyNamedBufferSubData = transmute(load(func, "CopyNamedBufferSubData", &[]));
_CopyTexImage1D = transmute(load(func, "CopyTexImage1D", &["CopyTexImage1DEXT"]));
_CopyTexImage2D = transmute(load(func, "CopyTexImage2D", &["CopyTexImage2DEXT"]));
_CopyTexSubImage1D = transmute(load(func, "CopyTexSubImage1D", &["CopyTexSubImage1DEXT"]));
_CopyTexSubImage2D = transmute(load(func, "CopyTexSubImage2D", &["CopyTexSubImage2DEXT"]));
_CopyTexSubImage3D = transmute(load(func, "CopyTexSubImage3D", &["CopyTexSubImage3DEXT"]));
_CopyTextureSubImage1D = transmute(load(func, "CopyTextureSubImage1D", &[]));
_CopyTextureSubImage2D = transmute(load(func, "CopyTextureSubImage2D", &[]));
_CopyTextureSubImage3D = transmute(load(func, "CopyTextureSubImage3D", &[]));
_CreateBuffers = transmute(load(func, "CreateBuffers", &[]));
_CreateFramebuffers = transmute(load(func, "CreateFramebuffers", &[]));
_CreateProgram = transmute(load(func, "CreateProgram", &["CreateProgramObjectARB"]));
_CreateProgramPipelines = transmute(load(func, "CreateProgramPipelines", &[]));
_CreateQueries = transmute(load(func, "CreateQueries", &[]));
_CreateRenderbuffers = transmute(load(func, "CreateRenderbuffers", &[]));
_CreateSamplers = transmute(load(func, "CreateSamplers", &[]));
_CreateShader = transmute(load(func, "CreateShader", &["CreateShaderObjectARB"]));
_CreateShaderProgramv = transmute(load(func, "CreateShaderProgramv", &[]));
_CreateTextures = transmute(load(func, "CreateTextures", &[]));
_CreateTransformFeedbacks = transmute(load(func, "CreateTransformFeedbacks", &[]));
_CreateVertexArrays = transmute(load(func, "CreateVertexArrays", &[]));
_CullFace = transmute(load(func, "CullFace", &[]));
_DebugMessageCallback = transmute(load(func, "DebugMessageCallback", &["DebugMessageCallbackARB", "DebugMessageCallbackKHR"]));
_DebugMessageControl = transmute(load(func, "DebugMessageControl", &["DebugMessageControlARB", "DebugMessageControlKHR"]));
_DebugMessageInsert = transmute(load(func, "DebugMessageInsert", &["DebugMessageInsertARB", "DebugMessageInsertKHR"]));
_DeleteBuffers = transmute(load(func, "DeleteBuffers", &["DeleteBuffersARB"]));
_DeleteFramebuffers = transmute(load(func, "DeleteFramebuffers", &["DeleteFramebuffersEXT"]));
_DeleteProgram = transmute(load(func, "DeleteProgram", &[]));
_DeleteProgramPipelines = transmute(load(func, "DeleteProgramPipelines", &[]));
_DeleteQueries = transmute(load(func, "DeleteQueries", &["DeleteQueriesARB"]));
_DeleteRenderbuffers = transmute(load(func, "DeleteRenderbuffers", &["DeleteRenderbuffersEXT"]));
_DeleteSamplers = transmute(load(func, "DeleteSamplers", &[]));
_DeleteShader = transmute(load(func, "DeleteShader", &[]));
_DeleteSync = transmute(load(func, "DeleteSync", &["DeleteSyncAPPLE"]));
_DeleteTextures = transmute(load(func, "DeleteTextures", &[]));
_DeleteTransformFeedbacks = transmute(load(func, "DeleteTransformFeedbacks", &["DeleteTransformFeedbacksNV"]));
_DeleteVertexArrays = transmute(load(func, "DeleteVertexArrays", &["DeleteVertexArraysAPPLE", "DeleteVertexArraysOES"]));
_DepthFunc = transmute(load(func, "DepthFunc", &[]));
_DepthMask = transmute(load(func, "DepthMask", &[]));
_DepthRange = transmute(load(func, "DepthRange", &[]));
_DepthRangeArrayv = transmute(load(func, "DepthRangeArrayv", &[]));
_DepthRangeIndexed = transmute(load(func, "DepthRangeIndexed", &[]));
_DepthRangef = transmute(load(func, "DepthRangef", &["DepthRangefOES"]));
_DetachShader = transmute(load(func, "DetachShader", &["DetachObjectARB"]));
_Disable = transmute(load(func, "Disable", &[]));
_DisableVertexArrayAttrib = transmute(load(func, "DisableVertexArrayAttrib", &[]));
_DisableVertexAttribArray = transmute(load(func, "DisableVertexAttribArray", &["DisableVertexAttribArrayARB"]));
_Disablei = transmute(load(func, "Disablei", &["DisableIndexedEXT", "DisableiEXT", "DisableiNV", "DisableiOES"]));
_DispatchCompute = transmute(load(func, "DispatchCompute", &[]));
_DispatchComputeIndirect = transmute(load(func, "DispatchComputeIndirect", &[]));
_DrawArrays = transmute(load(func, "DrawArrays", &["DrawArraysEXT"]));
_DrawArraysIndirect = transmute(load(func, "DrawArraysIndirect", &[]));
_DrawArraysInstanced = transmute(load(func, "DrawArraysInstanced", &["DrawArraysInstancedANGLE", "DrawArraysInstancedARB", "DrawArraysInstancedEXT", "DrawArraysInstancedNV"]));
_DrawArraysInstancedBaseInstance = transmute(load(func, "DrawArraysInstancedBaseInstance", &["DrawArraysInstancedBaseInstanceEXT"]));
_DrawBuffer = transmute(load(func, "DrawBuffer", &[]));
_DrawBuffers = transmute(load(func, "DrawBuffers", &["DrawBuffersARB", "DrawBuffersATI", "DrawBuffersEXT"]));
_DrawElements = transmute(load(func, "DrawElements", &[]));
_DrawElementsBaseVertex = transmute(load(func, "DrawElementsBaseVertex", &["DrawElementsBaseVertexEXT", "DrawElementsBaseVertexOES"]));
_DrawElementsIndirect = transmute(load(func, "DrawElementsIndirect", &[]));
_DrawElementsInstanced = transmute(load(func, "DrawElementsInstanced", &["DrawElementsInstancedANGLE", "DrawElementsInstancedARB", "DrawElementsInstancedEXT", "DrawElementsInstancedNV"]));
_DrawElementsInstancedBaseInstance = transmute(load(func, "DrawElementsInstancedBaseInstance", &["DrawElementsInstancedBaseInstanceEXT"]));
_DrawElementsInstancedBaseVertex = transmute(load(func, "DrawElementsInstancedBaseVertex", &["DrawElementsInstancedBaseVertexEXT", "DrawElementsInstancedBaseVertexOES"]));
_DrawElementsInstancedBaseVertexBaseInstance = transmute(load(func, "DrawElementsInstancedBaseVertexBaseInstance", &["DrawElementsInstancedBaseVertexBaseInstanceEXT"]));
_DrawRangeElements = transmute(load(func, "DrawRangeElements", &["DrawRangeElementsEXT"]));
_DrawRangeElementsBaseVertex = transmute(load(func, "DrawRangeElementsBaseVertex", &["DrawRangeElementsBaseVertexEXT", "DrawRangeElementsBaseVertexOES"]));
_DrawTransformFeedback = transmute(load(func, "DrawTransformFeedback", &["DrawTransformFeedbackEXT", "DrawTransformFeedbackNV"]));
_DrawTransformFeedbackInstanced = transmute(load(func, "DrawTransformFeedbackInstanced", &["DrawTransformFeedbackInstancedEXT"]));
_DrawTransformFeedbackStream = transmute(load(func, "DrawTransformFeedbackStream", &[]));
_DrawTransformFeedbackStreamInstanced = transmute(load(func, "DrawTransformFeedbackStreamInstanced", &[]));
_Enable = transmute(load(func, "Enable", &[]));
_EnableVertexArrayAttrib = transmute(load(func, "EnableVertexArrayAttrib", &[]));
_EnableVertexAttribArray = transmute(load(func, "EnableVertexAttribArray", &["EnableVertexAttribArrayARB"]));
_Enablei = transmute(load(func, "Enablei", &["EnableIndexedEXT", "EnableiEXT", "EnableiNV", "EnableiOES"]));
_EndConditionalRender = transmute(load(func, "EndConditionalRender", &["EndConditionalRenderNV", "EndConditionalRenderNVX"]));
_EndQuery = transmute(load(func, "EndQuery", &["EndQueryARB"]));
_EndQueryIndexed = transmute(load(func, "EndQueryIndexed", &[]));
_EndTransformFeedback = transmute(load(func, "EndTransformFeedback", &["EndTransformFeedbackEXT", "EndTransformFeedbackNV"]));
_FenceSync = transmute(load(func, "FenceSync", &["FenceSyncAPPLE"]));
_Finish = transmute(load(func, "Finish", &[]));
_Flush = transmute(load(func, "Flush", &[]));
_FlushMappedBufferRange = transmute(load(func, "FlushMappedBufferRange", &["FlushMappedBufferRangeAPPLE", "FlushMappedBufferRangeEXT"]));
_FlushMappedNamedBufferRange = transmute(load(func, "FlushMappedNamedBufferRange", &[]));
_FramebufferParameteri = transmute(load(func, "FramebufferParameteri", &[]));
_FramebufferRenderbuffer = transmute(load(func, "FramebufferRenderbuffer", &["FramebufferRenderbufferEXT"]));
_FramebufferTexture = transmute(load(func, "FramebufferTexture", &["FramebufferTextureARB", "FramebufferTextureEXT", "FramebufferTextureOES"]));
_FramebufferTexture1D = transmute(load(func, "FramebufferTexture1D", &["FramebufferTexture1DEXT"]));
_FramebufferTexture2D = transmute(load(func, "FramebufferTexture2D", &["FramebufferTexture2DEXT"]));
_FramebufferTexture3D = transmute(load(func, "FramebufferTexture3D", &["FramebufferTexture3DEXT"]));
_FramebufferTextureLayer = transmute(load(func, "FramebufferTextureLayer", &["FramebufferTextureLayerARB", "FramebufferTextureLayerEXT"]));
_FrontFace = transmute(load(func, "FrontFace", &[]));
_GenBuffers = transmute(load(func, "GenBuffers", &["GenBuffersARB"]));
_GenFramebuffers = transmute(load(func, "GenFramebuffers", &["GenFramebuffersEXT"]));
_GenProgramPipelines = transmute(load(func, "GenProgramPipelines", &[]));
_GenQueries = transmute(load(func, "GenQueries", &["GenQueriesARB"]));
_GenRenderbuffers = transmute(load(func, "GenRenderbuffers", &["GenRenderbuffersEXT"]));
_GenSamplers = transmute(load(func, "GenSamplers", &[]));
_GenTextures = transmute(load(func, "GenTextures", &[]));
_GenTransformFeedbacks = transmute(load(func, "GenTransformFeedbacks", &["GenTransformFeedbacksNV"]));
_GenVertexArrays = transmute(load(func, "GenVertexArrays", &["GenVertexArraysAPPLE", "GenVertexArraysOES"]));
_GenerateMipmap = transmute(load(func, "GenerateMipmap", &["GenerateMipmapEXT"]));
_GenerateTextureMipmap = transmute(load(func, "GenerateTextureMipmap", &[]));
_GetActiveAtomicCounterBufferiv = transmute(load(func, "GetActiveAtomicCounterBufferiv", &[]));
_GetActiveAttrib = transmute(load(func, "GetActiveAttrib", &["GetActiveAttribARB"]));
_GetActiveSubroutineName = transmute(load(func, "GetActiveSubroutineName", &[]));
_GetActiveSubroutineUniformName = transmute(load(func, "GetActiveSubroutineUniformName", &[]));
_GetActiveSubroutineUniformiv = transmute(load(func, "GetActiveSubroutineUniformiv", &[]));
_GetActiveUniform = transmute(load(func, "GetActiveUniform", &["GetActiveUniformARB"]));
_GetActiveUniformBlockName = transmute(load(func, "GetActiveUniformBlockName", &[]));
_GetActiveUniformBlockiv = transmute(load(func, "GetActiveUniformBlockiv", &[]));
_GetActiveUniformName = transmute(load(func, "GetActiveUniformName", &[]));
_GetActiveUniformsiv = transmute(load(func, "GetActiveUniformsiv", &[]));
_GetAttachedShaders = transmute(load(func, "GetAttachedShaders", &[]));
_GetAttribLocation = transmute(load(func, "GetAttribLocation", &["GetAttribLocationARB"]));
_GetBooleani_v = transmute(load(func, "GetBooleani_v", &["GetBooleanIndexedvEXT"]));
_GetBooleanv = transmute(load(func, "GetBooleanv", &[]));
_GetBufferParameteri64v = transmute(load(func, "GetBufferParameteri64v", &[]));
_GetBufferParameteriv = transmute(load(func, "GetBufferParameteriv", &["GetBufferParameterivARB"]));
_GetBufferPointerv = transmute(load(func, "GetBufferPointerv", &["GetBufferPointervARB", "GetBufferPointervOES"]));
_GetBufferSubData = transmute(load(func, "GetBufferSubData", &["GetBufferSubDataARB"]));
_GetCompressedTexImage = transmute(load(func, "GetCompressedTexImage", &["GetCompressedTexImageARB"]));
_GetCompressedTextureImage = transmute(load(func, "GetCompressedTextureImage", &[]));
_GetCompressedTextureSubImage = transmute(load(func, "GetCompressedTextureSubImage", &[]));
_GetDebugMessageLog = transmute(load(func, "GetDebugMessageLog", &["GetDebugMessageLogARB", "GetDebugMessageLogKHR"]));
_GetDoublei_v = transmute(load(func, "GetDoublei_v", &["GetDoubleIndexedvEXT", "GetDoublei_vEXT"]));
_GetDoublev = transmute(load(func, "GetDoublev", &[]));
_GetError = transmute(load(func, "GetError", &[]));
_GetFloati_v = transmute(load(func, "GetFloati_v", &["GetFloatIndexedvEXT", "GetFloati_vEXT", "GetFloati_vNV", "GetFloati_vOES"]));
_GetFloatv = transmute(load(func, "GetFloatv", &[]));
_GetFragDataIndex = transmute(load(func, "GetFragDataIndex", &["GetFragDataIndexEXT"]));
_GetFragDataLocation = transmute(load(func, "GetFragDataLocation", &["GetFragDataLocationEXT"]));
_GetFramebufferAttachmentParameteriv = transmute(load(func, "GetFramebufferAttachmentParameteriv", &["GetFramebufferAttachmentParameterivEXT"]));
_GetFramebufferParameteriv = transmute(load(func, "GetFramebufferParameteriv", &[]));
_GetGraphicsResetStatus = transmute(load(func, "GetGraphicsResetStatus", &["GetGraphicsResetStatusEXT", "GetGraphicsResetStatusKHR"]));
_GetInteger64i_v = transmute(load(func, "GetInteger64i_v", &[]));
_GetInteger64v = transmute(load(func, "GetInteger64v", &["GetInteger64vAPPLE"]));
_GetIntegeri_v = transmute(load(func, "GetIntegeri_v", &["GetIntegerIndexedvEXT"]));
_GetIntegerv = transmute(load(func, "GetIntegerv", &[]));
_GetInternalformati64v = transmute(load(func, "GetInternalformati64v", &[]));
_GetInternalformativ = transmute(load(func, "GetInternalformativ", &[]));
_GetMultisamplefv = transmute(load(func, "GetMultisamplefv", &["GetMultisamplefvNV"]));
_GetNamedBufferParameteri64v = transmute(load(func, "GetNamedBufferParameteri64v", &[]));
_GetNamedBufferParameteriv = transmute(load(func, "GetNamedBufferParameteriv", &[]));
_GetNamedBufferPointerv = transmute(load(func, "GetNamedBufferPointerv", &[]));
_GetNamedBufferSubData = transmute(load(func, "GetNamedBufferSubData", &[]));
_GetNamedFramebufferAttachmentParameteriv = transmute(load(func, "GetNamedFramebufferAttachmentParameteriv", &[]));
_GetNamedFramebufferParameteriv = transmute(load(func, "GetNamedFramebufferParameteriv", &[]));
_GetNamedRenderbufferParameteriv = transmute(load(func, "GetNamedRenderbufferParameteriv", &[]));
_GetObjectLabel = transmute(load(func, "GetObjectLabel", &["GetObjectLabelKHR"]));
_GetObjectPtrLabel = transmute(load(func, "GetObjectPtrLabel", &["GetObjectPtrLabelKHR"]));
_GetPointerv = transmute(load(func, "GetPointerv", &["GetPointervEXT", "GetPointervKHR"]));
_GetProgramBinary = transmute(load(func, "GetProgramBinary", &["GetProgramBinaryOES"]));
_GetProgramInfoLog = transmute(load(func, "GetProgramInfoLog", &[]));
_GetProgramInterfaceiv = transmute(load(func, "GetProgramInterfaceiv", &[]));
_GetProgramPipelineInfoLog = transmute(load(func, "GetProgramPipelineInfoLog", &[]));
_GetProgramPipelineiv = transmute(load(func, "GetProgramPipelineiv", &[]));
_GetProgramResourceIndex = transmute(load(func, "GetProgramResourceIndex", &[]));
_GetProgramResourceLocation = transmute(load(func, "GetProgramResourceLocation", &[]));
_GetProgramResourceLocationIndex = transmute(load(func, "GetProgramResourceLocationIndex", &[]));
_GetProgramResourceName = transmute(load(func, "GetProgramResourceName", &[]));
_GetProgramResourceiv = transmute(load(func, "GetProgramResourceiv", &[]));
_GetProgramStageiv = transmute(load(func, "GetProgramStageiv", &[]));
_GetProgramiv = transmute(load(func, "GetProgramiv", &[]));
_GetQueryBufferObjecti64v = transmute(load(func, "GetQueryBufferObjecti64v", &[]));
_GetQueryBufferObjectiv = transmute(load(func, "GetQueryBufferObjectiv", &[]));
_GetQueryBufferObjectui64v = transmute(load(func, "GetQueryBufferObjectui64v", &[]));
_GetQueryBufferObjectuiv = transmute(load(func, "GetQueryBufferObjectuiv", &[]));
_GetQueryIndexediv = transmute(load(func, "GetQueryIndexediv", &[]));
_GetQueryObjecti64v = transmute(load(func, "GetQueryObjecti64v", &["GetQueryObjecti64vEXT"]));
_GetQueryObjectiv = transmute(load(func, "GetQueryObjectiv", &["GetQueryObjectivARB", "GetQueryObjectivEXT"]));
_GetQueryObjectui64v = transmute(load(func, "GetQueryObjectui64v", &["GetQueryObjectui64vEXT"]));
_GetQueryObjectuiv = transmute(load(func, "GetQueryObjectuiv", &["GetQueryObjectuivARB"]));
_GetQueryiv = transmute(load(func, "GetQueryiv", &["GetQueryivARB"]));
_GetRenderbufferParameteriv = transmute(load(func, "GetRenderbufferParameteriv", &["GetRenderbufferParameterivEXT"]));
_GetSamplerParameterIiv = transmute(load(func, "GetSamplerParameterIiv", &["GetSamplerParameterIivEXT", "GetSamplerParameterIivOES"]));
_GetSamplerParameterIuiv = transmute(load(func, "GetSamplerParameterIuiv", &["GetSamplerParameterIuivEXT", "GetSamplerParameterIuivOES"]));
_GetSamplerParameterfv = transmute(load(func, "GetSamplerParameterfv", &[]));
_GetSamplerParameteriv = transmute(load(func, "GetSamplerParameteriv", &[]));
_GetShaderInfoLog = transmute(load(func, "GetShaderInfoLog", &[]));
_GetShaderPrecisionFormat = transmute(load(func, "GetShaderPrecisionFormat", &[]));
_GetShaderSource = transmute(load(func, "GetShaderSource", &["GetShaderSourceARB"]));
_GetShaderiv = transmute(load(func, "GetShaderiv", &[]));
_GetString = transmute(load(func, "GetString", &[]));
_GetStringi = transmute(load(func, "GetStringi", &[]));
_GetSubroutineIndex = transmute(load(func, "GetSubroutineIndex", &[]));
_GetSubroutineUniformLocation = transmute(load(func, "GetSubroutineUniformLocation", &[]));
_GetSynciv = transmute(load(func, "GetSynciv", &["GetSyncivAPPLE"]));
_GetTexImage = transmute(load(func, "GetTexImage", &[]));
_GetTexLevelParameterfv = transmute(load(func, "GetTexLevelParameterfv", &[]));
_GetTexLevelParameteriv = transmute(load(func, "GetTexLevelParameteriv", &[]));
_GetTexParameterIiv = transmute(load(func, "GetTexParameterIiv", &["GetTexParameterIivEXT", "GetTexParameterIivOES"]));
_GetTexParameterIuiv = transmute(load(func, "GetTexParameterIuiv", &["GetTexParameterIuivEXT", "GetTexParameterIuivOES"]));
_GetTexParameterfv = transmute(load(func, "GetTexParameterfv", &[]));
_GetTexParameteriv = transmute(load(func, "GetTexParameteriv", &[]));
_GetTextureImage = transmute(load(func, "GetTextureImage", &[]));
_GetTextureLevelParameterfv = transmute(load(func, "GetTextureLevelParameterfv", &[]));
_GetTextureLevelParameteriv = transmute(load(func, "GetTextureLevelParameteriv", &[]));
_GetTextureParameterIiv = transmute(load(func, "GetTextureParameterIiv", &[]));
_GetTextureParameterIuiv = transmute(load(func, "GetTextureParameterIuiv", &[]));
_GetTextureParameterfv = transmute(load(func, "GetTextureParameterfv", &[]));
_GetTextureParameteriv = transmute(load(func, "GetTextureParameteriv", &[]));
_GetTextureSubImage = transmute(load(func, "GetTextureSubImage", &[]));
_GetTransformFeedbackVarying = transmute(load(func, "GetTransformFeedbackVarying", &["GetTransformFeedbackVaryingEXT"]));
_GetTransformFeedbacki64_v = transmute(load(func, "GetTransformFeedbacki64_v", &[]));
_GetTransformFeedbacki_v = transmute(load(func, "GetTransformFeedbacki_v", &[]));
_GetTransformFeedbackiv = transmute(load(func, "GetTransformFeedbackiv", &[]));
_GetUniformBlockIndex = transmute(load(func, "GetUniformBlockIndex", &[]));
_GetUniformIndices = transmute(load(func, "GetUniformIndices", &[]));
_GetUniformLocation = transmute(load(func, "GetUniformLocation", &["GetUniformLocationARB"]));
_GetUniformSubroutineuiv = transmute(load(func, "GetUniformSubroutineuiv", &[]));
_GetUniformdv = transmute(load(func, "GetUniformdv", &[]));
_GetUniformfv = transmute(load(func, "GetUniformfv", &["GetUniformfvARB"]));
_GetUniformiv = transmute(load(func, "GetUniformiv", &["GetUniformivARB"]));
_GetUniformuiv = transmute(load(func, "GetUniformuiv", &["GetUniformuivEXT"]));
_GetVertexArrayIndexed64iv = transmute(load(func, "GetVertexArrayIndexed64iv", &[]));
_GetVertexArrayIndexediv = transmute(load(func, "GetVertexArrayIndexediv", &[]));
_GetVertexArrayiv = transmute(load(func, "GetVertexArrayiv", &[]));
_GetVertexAttribIiv = transmute(load(func, "GetVertexAttribIiv", &["GetVertexAttribIivEXT"]));
_GetVertexAttribIuiv = transmute(load(func, "GetVertexAttribIuiv", &["GetVertexAttribIuivEXT"]));
_GetVertexAttribLdv = transmute(load(func, "GetVertexAttribLdv", &["GetVertexAttribLdvEXT"]));
_GetVertexAttribPointerv = transmute(load(func, "GetVertexAttribPointerv", &["GetVertexAttribPointervARB", "GetVertexAttribPointervNV"]));
_GetVertexAttribdv = transmute(load(func, "GetVertexAttribdv", &["GetVertexAttribdvARB", "GetVertexAttribdvNV"]));
_GetVertexAttribfv = transmute(load(func, "GetVertexAttribfv", &["GetVertexAttribfvARB", "GetVertexAttribfvNV"]));
_GetVertexAttribiv = transmute(load(func, "GetVertexAttribiv", &["GetVertexAttribivARB", "GetVertexAttribivNV"]));
_GetnColorTable = transmute(load(func, "GetnColorTable", &[]));
_GetnCompressedTexImage = transmute(load(func, "GetnCompressedTexImage", &[]));
_GetnConvolutionFilter = transmute(load(func, "GetnConvolutionFilter", &[]));
_GetnHistogram = transmute(load(func, "GetnHistogram", &[]));
_GetnMapdv = transmute(load(func, "GetnMapdv", &[]));
_GetnMapfv = transmute(load(func, "GetnMapfv", &[]));
_GetnMapiv = transmute(load(func, "GetnMapiv", &[]));
_GetnMinmax = transmute(load(func, "GetnMinmax", &[]));
_GetnPixelMapfv = transmute(load(func, "GetnPixelMapfv", &[]));
_GetnPixelMapuiv = transmute(load(func, "GetnPixelMapuiv", &[]));
_GetnPixelMapusv = transmute(load(func, "GetnPixelMapusv", &[]));
_GetnPolygonStipple = transmute(load(func, "GetnPolygonStipple", &[]));
_GetnSeparableFilter = transmute(load(func, "GetnSeparableFilter", &[]));
_GetnTexImage = transmute(load(func, "GetnTexImage", &[]));
_GetnUniformdv = transmute(load(func, "GetnUniformdv", &[]));
_GetnUniformfv = transmute(load(func, "GetnUniformfv", &["GetnUniformfvEXT", "GetnUniformfvKHR"]));
_GetnUniformiv = transmute(load(func, "GetnUniformiv", &["GetnUniformivEXT", "GetnUniformivKHR"]));
_GetnUniformuiv = transmute(load(func, "GetnUniformuiv", &["GetnUniformuivKHR"]));
_Hint = transmute(load(func, "Hint", &[]));
_InvalidateBufferData = transmute(load(func, "InvalidateBufferData", &[]));
_InvalidateBufferSubData = transmute(load(func, "InvalidateBufferSubData", &[]));
_InvalidateFramebuffer = transmute(load(func, "InvalidateFramebuffer", &[]));
_InvalidateNamedFramebufferData = transmute(load(func, "InvalidateNamedFramebufferData", &[]));
_InvalidateNamedFramebufferSubData = transmute(load(func, "InvalidateNamedFramebufferSubData", &[]));
_InvalidateSubFramebuffer = transmute(load(func, "InvalidateSubFramebuffer", &[]));
_InvalidateTexImage = transmute(load(func, "InvalidateTexImage", &[]));
_InvalidateTexSubImage = transmute(load(func, "InvalidateTexSubImage", &[]));
_IsBuffer = transmute(load(func, "IsBuffer", &["IsBufferARB"]));
_IsEnabled = transmute(load(func, "IsEnabled", &[]));
_IsEnabledi = transmute(load(func, "IsEnabledi", &["IsEnabledIndexedEXT", "IsEnablediEXT", "IsEnablediNV", "IsEnablediOES"]));
_IsFramebuffer = transmute(load(func, "IsFramebuffer", &["IsFramebufferEXT"]));
_IsProgram = transmute(load(func, "IsProgram", &[]));
_IsProgramPipeline = transmute(load(func, "IsProgramPipeline", &[]));
_IsQuery = transmute(load(func, "IsQuery", &["IsQueryARB"]));
_IsRenderbuffer = transmute(load(func, "IsRenderbuffer", &["IsRenderbufferEXT"]));
_IsSampler = transmute(load(func, "IsSampler", &[]));
_IsShader = transmute(load(func, "IsShader", &[]));
_IsSync = transmute(load(func, "IsSync", &["IsSyncAPPLE"]));
_IsTexture = transmute(load(func, "IsTexture", &[]));
_IsTransformFeedback = transmute(load(func, "IsTransformFeedback", &["IsTransformFeedbackNV"]));
_IsVertexArray = transmute(load(func, "IsVertexArray", &["IsVertexArrayAPPLE", "IsVertexArrayOES"]));
_LineWidth = transmute(load(func, "LineWidth", &[]));
_LinkProgram = transmute(load(func, "LinkProgram", &["LinkProgramARB"]));
_LogicOp = transmute(load(func, "LogicOp", &[]));
_MapBuffer = transmute(load(func, "MapBuffer", &["MapBufferARB", "MapBufferOES"]));
_MapBufferRange = transmute(load(func, "MapBufferRange", &["MapBufferRangeEXT"]));
_MapNamedBuffer = transmute(load(func, "MapNamedBuffer", &[]));
_MapNamedBufferRange = transmute(load(func, "MapNamedBufferRange", &[]));
_MemoryBarrier = transmute(load(func, "MemoryBarrier", &["MemoryBarrierEXT"]));
_MemoryBarrierByRegion = transmute(load(func, "MemoryBarrierByRegion", &[]));
_MinSampleShading = transmute(load(func, "MinSampleShading", &["MinSampleShadingARB", "MinSampleShadingOES"]));
_MultiDrawArrays = transmute(load(func, "MultiDrawArrays", &["MultiDrawArraysEXT"]));
_MultiDrawArraysIndirect = transmute(load(func, "MultiDrawArraysIndirect", &["MultiDrawArraysIndirectAMD", "MultiDrawArraysIndirectEXT"]));
_MultiDrawArraysIndirectCount = transmute(load(func, "MultiDrawArraysIndirectCount", &["MultiDrawArraysIndirectCountARB"]));
_MultiDrawElements = transmute(load(func, "MultiDrawElements", &["MultiDrawElementsEXT"]));
_MultiDrawElementsBaseVertex = transmute(load(func, "MultiDrawElementsBaseVertex", &["MultiDrawElementsBaseVertexEXT"]));
_MultiDrawElementsIndirect = transmute(load(func, "MultiDrawElementsIndirect", &["MultiDrawElementsIndirectAMD", "MultiDrawElementsIndirectEXT"]));
_MultiDrawElementsIndirectCount = transmute(load(func, "MultiDrawElementsIndirectCount", &["MultiDrawElementsIndirectCountARB"]));
_MultiTexCoordP1ui = transmute(load(func, "MultiTexCoordP1ui", &[]));
_MultiTexCoordP1uiv = transmute(load(func, "MultiTexCoordP1uiv", &[]));
_MultiTexCoordP2ui = transmute(load(func, "MultiTexCoordP2ui", &[]));
_MultiTexCoordP2uiv = transmute(load(func, "MultiTexCoordP2uiv", &[]));
_MultiTexCoordP3ui = transmute(load(func, "MultiTexCoordP3ui", &[]));
_MultiTexCoordP3uiv = transmute(load(func, "MultiTexCoordP3uiv", &[]));
_MultiTexCoordP4ui = transmute(load(func, "MultiTexCoordP4ui", &[]));
_MultiTexCoordP4uiv = transmute(load(func, "MultiTexCoordP4uiv", &[]));
_NamedBufferData = transmute(load(func, "NamedBufferData", &[]));
_NamedBufferStorage = transmute(load(func, "NamedBufferStorage", &["NamedBufferStorageEXT"]));
_NamedBufferSubData = transmute(load(func, "NamedBufferSubData", &["NamedBufferSubDataEXT"]));
_NamedFramebufferDrawBuffer = transmute(load(func, "NamedFramebufferDrawBuffer", &[]));
_NamedFramebufferDrawBuffers = transmute(load(func, "NamedFramebufferDrawBuffers", &[]));
_NamedFramebufferParameteri = transmute(load(func, "NamedFramebufferParameteri", &[]));
_NamedFramebufferReadBuffer = transmute(load(func, "NamedFramebufferReadBuffer", &[]));
_NamedFramebufferRenderbuffer = transmute(load(func, "NamedFramebufferRenderbuffer", &[]));
_NamedFramebufferTexture = transmute(load(func, "NamedFramebufferTexture", &[]));
_NamedFramebufferTextureLayer = transmute(load(func, "NamedFramebufferTextureLayer", &[]));
_NamedRenderbufferStorage = transmute(load(func, "NamedRenderbufferStorage", &[]));
_NamedRenderbufferStorageMultisample = transmute(load(func, "NamedRenderbufferStorageMultisample", &[]));
_NormalP3ui = transmute(load(func, "NormalP3ui", &[]));
_NormalP3uiv = transmute(load(func, "NormalP3uiv", &[]));
_ObjectLabel = transmute(load(func, "ObjectLabel", &["ObjectLabelKHR"]));
_ObjectPtrLabel = transmute(load(func, "ObjectPtrLabel", &["ObjectPtrLabelKHR"]));
_PatchParameterfv = transmute(load(func, "PatchParameterfv", &[]));
_PatchParameteri = transmute(load(func, "PatchParameteri", &["PatchParameteriEXT", "PatchParameteriOES"]));
_PauseTransformFeedback = transmute(load(func, "PauseTransformFeedback", &["PauseTransformFeedbackNV"]));
_PixelStoref = transmute(load(func, "PixelStoref", &[]));
_PixelStorei = transmute(load(func, "PixelStorei", &[]));
_PointParameterf = transmute(load(func, "PointParameterf", &["PointParameterfARB", "PointParameterfEXT", "PointParameterfSGIS"]));
_PointParameterfv = transmute(load(func, "PointParameterfv", &["PointParameterfvARB", "PointParameterfvEXT", "PointParameterfvSGIS"]));
_PointParameteri = transmute(load(func, "PointParameteri", &["PointParameteriNV"]));
_PointParameteriv = transmute(load(func, "PointParameteriv", &["PointParameterivNV"]));
_PointSize = transmute(load(func, "PointSize", &[]));
_PolygonMode = transmute(load(func, "PolygonMode", &["PolygonModeNV"]));
_PolygonOffset = transmute(load(func, "PolygonOffset", &[]));
_PolygonOffsetClamp = transmute(load(func, "PolygonOffsetClamp", &["PolygonOffsetClampEXT"]));
_PopDebugGroup = transmute(load(func, "PopDebugGroup", &["PopDebugGroupKHR"]));
_PrimitiveRestartIndex = transmute(load(func, "PrimitiveRestartIndex", &[]));
_ProgramBinary = transmute(load(func, "ProgramBinary", &["ProgramBinaryOES"]));
_ProgramParameteri = transmute(load(func, "ProgramParameteri", &["ProgramParameteriARB", "ProgramParameteriEXT"]));
_ProgramUniform1d = transmute(load(func, "ProgramUniform1d", &[]));
_ProgramUniform1dv = transmute(load(func, "ProgramUniform1dv", &[]));
_ProgramUniform1f = transmute(load(func, "ProgramUniform1f", &["ProgramUniform1fEXT"]));
_ProgramUniform1fv = transmute(load(func, "ProgramUniform1fv", &["ProgramUniform1fvEXT"]));
_ProgramUniform1i = transmute(load(func, "ProgramUniform1i", &["ProgramUniform1iEXT"]));
_ProgramUniform1iv = transmute(load(func, "ProgramUniform1iv", &["ProgramUniform1ivEXT"]));
_ProgramUniform1ui = transmute(load(func, "ProgramUniform1ui", &["ProgramUniform1uiEXT"]));
_ProgramUniform1uiv = transmute(load(func, "ProgramUniform1uiv", &["ProgramUniform1uivEXT"]));
_ProgramUniform2d = transmute(load(func, "ProgramUniform2d", &[]));
_ProgramUniform2dv = transmute(load(func, "ProgramUniform2dv", &[]));
_ProgramUniform2f = transmute(load(func, "ProgramUniform2f", &["ProgramUniform2fEXT"]));
_ProgramUniform2fv = transmute(load(func, "ProgramUniform2fv", &["ProgramUniform2fvEXT"]));
_ProgramUniform2i = transmute(load(func, "ProgramUniform2i", &["ProgramUniform2iEXT"]));
_ProgramUniform2iv = transmute(load(func, "ProgramUniform2iv", &["ProgramUniform2ivEXT"]));
_ProgramUniform2ui = transmute(load(func, "ProgramUniform2ui", &["ProgramUniform2uiEXT"]));
_ProgramUniform2uiv = transmute(load(func, "ProgramUniform2uiv", &["ProgramUniform2uivEXT"]));
_ProgramUniform3d = transmute(load(func, "ProgramUniform3d", &[]));
_ProgramUniform3dv = transmute(load(func, "ProgramUniform3dv", &[]));
_ProgramUniform3f = transmute(load(func, "ProgramUniform3f", &["ProgramUniform3fEXT"]));
_ProgramUniform3fv = transmute(load(func, "ProgramUniform3fv", &["ProgramUniform3fvEXT"]));
_ProgramUniform3i = transmute(load(func, "ProgramUniform3i", &["ProgramUniform3iEXT"]));
_ProgramUniform3iv = transmute(load(func, "ProgramUniform3iv", &["ProgramUniform3ivEXT"]));
_ProgramUniform3ui = transmute(load(func, "ProgramUniform3ui", &["ProgramUniform3uiEXT"]));
_ProgramUniform3uiv = transmute(load(func, "ProgramUniform3uiv", &["ProgramUniform3uivEXT"]));
_ProgramUniform4d = transmute(load(func, "ProgramUniform4d", &[]));
_ProgramUniform4dv = transmute(load(func, "ProgramUniform4dv", &[]));
_ProgramUniform4f = transmute(load(func, "ProgramUniform4f", &["ProgramUniform4fEXT"]));
_ProgramUniform4fv = transmute(load(func, "ProgramUniform4fv", &["ProgramUniform4fvEXT"]));
_ProgramUniform4i = transmute(load(func, "ProgramUniform4i", &["ProgramUniform4iEXT"]));
_ProgramUniform4iv = transmute(load(func, "ProgramUniform4iv", &["ProgramUniform4ivEXT"]));
_ProgramUniform4ui = transmute(load(func, "ProgramUniform4ui", &["ProgramUniform4uiEXT"]));
_ProgramUniform4uiv = transmute(load(func, "ProgramUniform4uiv", &["ProgramUniform4uivEXT"]));
_ProgramUniformMatrix2dv = transmute(load(func, "ProgramUniformMatrix2dv", &[]));
_ProgramUniformMatrix2fv = transmute(load(func, "ProgramUniformMatrix2fv", &["ProgramUniformMatrix2fvEXT"]));
_ProgramUniformMatrix2x3dv = transmute(load(func, "ProgramUniformMatrix2x3dv", &[]));
_ProgramUniformMatrix2x3fv = transmute(load(func, "ProgramUniformMatrix2x3fv", &["ProgramUniformMatrix2x3fvEXT"]));
_ProgramUniformMatrix2x4dv = transmute(load(func, "ProgramUniformMatrix2x4dv", &[]));
_ProgramUniformMatrix2x4fv = transmute(load(func, "ProgramUniformMatrix2x4fv", &["ProgramUniformMatrix2x4fvEXT"]));
_ProgramUniformMatrix3dv = transmute(load(func, "ProgramUniformMatrix3dv", &[]));
_ProgramUniformMatrix3fv = transmute(load(func, "ProgramUniformMatrix3fv", &["ProgramUniformMatrix3fvEXT"]));
_ProgramUniformMatrix3x2dv = transmute(load(func, "ProgramUniformMatrix3x2dv", &[]));
_ProgramUniformMatrix3x2fv = transmute(load(func, "ProgramUniformMatrix3x2fv", &["ProgramUniformMatrix3x2fvEXT"]));
_ProgramUniformMatrix3x4dv = transmute(load(func, "ProgramUniformMatrix3x4dv", &[]));
_ProgramUniformMatrix3x4fv = transmute(load(func, "ProgramUniformMatrix3x4fv", &["ProgramUniformMatrix3x4fvEXT"]));
_ProgramUniformMatrix4dv = transmute(load(func, "ProgramUniformMatrix4dv", &[]));
_ProgramUniformMatrix4fv = transmute(load(func, "ProgramUniformMatrix4fv", &["ProgramUniformMatrix4fvEXT"]));
_ProgramUniformMatrix4x2dv = transmute(load(func, "ProgramUniformMatrix4x2dv", &[]));
_ProgramUniformMatrix4x2fv = transmute(load(func, "ProgramUniformMatrix4x2fv", &["ProgramUniformMatrix4x2fvEXT"]));
_ProgramUniformMatrix4x3dv = transmute(load(func, "ProgramUniformMatrix4x3dv", &[]));
_ProgramUniformMatrix4x3fv = transmute(load(func, "ProgramUniformMatrix4x3fv", &["ProgramUniformMatrix4x3fvEXT"]));
_ProvokingVertex = transmute(load(func, "ProvokingVertex", &["ProvokingVertexEXT"]));
_PushDebugGroup = transmute(load(func, "PushDebugGroup", &["PushDebugGroupKHR"]));
_QueryCounter = transmute(load(func, "QueryCounter", &["QueryCounterEXT"]));
_ReadBuffer = transmute(load(func, "ReadBuffer", &[]));
_ReadPixels = transmute(load(func, "ReadPixels", &[]));
_ReadnPixels = transmute(load(func, "ReadnPixels", &["ReadnPixelsARB", "ReadnPixelsEXT", "ReadnPixelsKHR"]));
_ReleaseShaderCompiler = transmute(load(func, "ReleaseShaderCompiler", &[]));
_RenderbufferStorage = transmute(load(func, "RenderbufferStorage", &["RenderbufferStorageEXT"]));
_RenderbufferStorageMultisample = transmute(load(func, "RenderbufferStorageMultisample", &["RenderbufferStorageMultisampleEXT", "RenderbufferStorageMultisampleNV"]));
_ResumeTransformFeedback = transmute(load(func, "ResumeTransformFeedback", &["ResumeTransformFeedbackNV"]));
_SampleCoverage = transmute(load(func, "SampleCoverage", &["SampleCoverageARB"]));
_SampleMaski = transmute(load(func, "SampleMaski", &[]));
_SamplerParameterIiv = transmute(load(func, "SamplerParameterIiv", &["SamplerParameterIivEXT", "SamplerParameterIivOES"]));
_SamplerParameterIuiv = transmute(load(func, "SamplerParameterIuiv", &["SamplerParameterIuivEXT", "SamplerParameterIuivOES"]));
_SamplerParameterf = transmute(load(func, "SamplerParameterf", &[]));
_SamplerParameterfv = transmute(load(func, "SamplerParameterfv", &[]));
_SamplerParameteri = transmute(load(func, "SamplerParameteri", &[]));
_SamplerParameteriv = transmute(load(func, "SamplerParameteriv", &[]));
_Scissor = transmute(load(func, "Scissor", &[]));
_ScissorArrayv = transmute(load(func, "ScissorArrayv", &["ScissorArrayvNV", "ScissorArrayvOES"]));
_ScissorIndexed = transmute(load(func, "ScissorIndexed", &["ScissorIndexedNV", "ScissorIndexedOES"]));
_ScissorIndexedv = transmute(load(func, "ScissorIndexedv", &["ScissorIndexedvNV", "ScissorIndexedvOES"]));
_SecondaryColorP3ui = transmute(load(func, "SecondaryColorP3ui", &[]));
_SecondaryColorP3uiv = transmute(load(func, "SecondaryColorP3uiv", &[]));
_ShaderBinary = transmute(load(func, "ShaderBinary", &[]));
_ShaderSource = transmute(load(func, "ShaderSource", &["ShaderSourceARB"]));
_ShaderStorageBlockBinding = transmute(load(func, "ShaderStorageBlockBinding", &[]));
_SpecializeShader = transmute(load(func, "SpecializeShader", &["SpecializeShaderARB"]));
_StencilFunc = transmute(load(func, "StencilFunc", &[]));
_StencilFuncSeparate = transmute(load(func, "StencilFuncSeparate", &[]));
_StencilMask = transmute(load(func, "StencilMask", &[]));
_StencilMaskSeparate = transmute(load(func, "StencilMaskSeparate", &[]));
_StencilOp = transmute(load(func, "StencilOp", &[]));
_StencilOpSeparate = transmute(load(func, "StencilOpSeparate", &["StencilOpSeparateATI"]));
_TexBuffer = transmute(load(func, "TexBuffer", &["TexBufferARB", "TexBufferEXT", "TexBufferOES"]));
_TexBufferRange = transmute(load(func, "TexBufferRange", &["TexBufferRangeEXT", "TexBufferRangeOES"]));
_TexCoordP1ui = transmute(load(func, "TexCoordP1ui", &[]));
_TexCoordP1uiv = transmute(load(func, "TexCoordP1uiv", &[]));
_TexCoordP2ui = transmute(load(func, "TexCoordP2ui", &[]));
_TexCoordP2uiv = transmute(load(func, "TexCoordP2uiv", &[]));
_TexCoordP3ui = transmute(load(func, "TexCoordP3ui", &[]));
_TexCoordP3uiv = transmute(load(func, "TexCoordP3uiv", &[]));
_TexCoordP4ui = transmute(load(func, "TexCoordP4ui", &[]));
_TexCoordP4uiv = transmute(load(func, "TexCoordP4uiv", &[]));
_TexImage1D = transmute(load(func, "TexImage1D", &[]));
_TexImage2D = transmute(load(func, "TexImage2D", &[]));
_TexImage2DMultisample = transmute(load(func, "TexImage2DMultisample", &[]));
_TexImage3D = transmute(load(func, "TexImage3D", &["TexImage3DEXT"]));
_TexImage3DMultisample = transmute(load(func, "TexImage3DMultisample", &[]));
_TexParameterIiv = transmute(load(func, "TexParameterIiv", &["TexParameterIivEXT", "TexParameterIivOES"]));
_TexParameterIuiv = transmute(load(func, "TexParameterIuiv", &["TexParameterIuivEXT", "TexParameterIuivOES"]));
_TexParameterf = transmute(load(func, "TexParameterf", &[]));
_TexParameterfv = transmute(load(func, "TexParameterfv", &[]));
_TexParameteri = transmute(load(func, "TexParameteri", &[]));
_TexParameteriv = transmute(load(func, "TexParameteriv", &[]));
_TexStorage1D = transmute(load(func, "TexStorage1D", &["TexStorage1DEXT"]));
_TexStorage2D = transmute(load(func, "TexStorage2D", &["TexStorage2DEXT"]));
_TexStorage2DMultisample = transmute(load(func, "TexStorage2DMultisample", &[]));
_TexStorage3D = transmute(load(func, "TexStorage3D", &["TexStorage3DEXT"]));
_TexStorage3DMultisample = transmute(load(func, "TexStorage3DMultisample", &["TexStorage3DMultisampleOES"]));
_TexSubImage1D = transmute(load(func, "TexSubImage1D", &["TexSubImage1DEXT"]));
_TexSubImage2D = transmute(load(func, "TexSubImage2D", &["TexSubImage2DEXT"]));
_TexSubImage3D = transmute(load(func, "TexSubImage3D", &["TexSubImage3DEXT"]));
_TextureBarrier = transmute(load(func, "TextureBarrier", &[]));
_TextureBuffer = transmute(load(func, "TextureBuffer", &[]));
_TextureBufferRange = transmute(load(func, "TextureBufferRange", &[]));
_TextureParameterIiv = transmute(load(func, "TextureParameterIiv", &[]));
_TextureParameterIuiv = transmute(load(func, "TextureParameterIuiv", &[]));
_TextureParameterf = transmute(load(func, "TextureParameterf", &[]));
_TextureParameterfv = transmute(load(func, "TextureParameterfv", &[]));
_TextureParameteri = transmute(load(func, "TextureParameteri", &[]));
_TextureParameteriv = transmute(load(func, "TextureParameteriv", &[]));
_TextureStorage1D = transmute(load(func, "TextureStorage1D", &[]));
_TextureStorage2D = transmute(load(func, "TextureStorage2D", &[]));
_TextureStorage2DMultisample = transmute(load(func, "TextureStorage2DMultisample", &[]));
_TextureStorage3D = transmute(load(func, "TextureStorage3D", &[]));
_TextureStorage3DMultisample = transmute(load(func, "TextureStorage3DMultisample", &[]));
_TextureSubImage1D = transmute(load(func, "TextureSubImage1D", &[]));
_TextureSubImage2D = transmute(load(func, "TextureSubImage2D", &[]));
_TextureSubImage3D = transmute(load(func, "TextureSubImage3D", &[]));
_TextureView = transmute(load(func, "TextureView", &["TextureViewEXT", "TextureViewOES"]));
_TransformFeedbackBufferBase = transmute(load(func, "TransformFeedbackBufferBase", &[]));
_TransformFeedbackBufferRange = transmute(load(func, "TransformFeedbackBufferRange", &[]));
_TransformFeedbackVaryings = transmute(load(func, "TransformFeedbackVaryings", &["TransformFeedbackVaryingsEXT"]));
_Uniform1d = transmute(load(func, "Uniform1d", &[]));
_Uniform1dv = transmute(load(func, "Uniform1dv", &[]));
_Uniform1f = transmute(load(func, "Uniform1f", &["Uniform1fARB"]));
_Uniform1fv = transmute(load(func, "Uniform1fv", &["Uniform1fvARB"]));
_Uniform1i = transmute(load(func, "Uniform1i", &["Uniform1iARB"]));
_Uniform1iv = transmute(load(func, "Uniform1iv", &["Uniform1ivARB"]));
_Uniform1ui = transmute(load(func, "Uniform1ui", &["Uniform1uiEXT"]));
_Uniform1uiv = transmute(load(func, "Uniform1uiv", &["Uniform1uivEXT"]));
_Uniform2d = transmute(load(func, "Uniform2d", &[]));
_Uniform2dv = transmute(load(func, "Uniform2dv", &[]));
_Uniform2f = transmute(load(func, "Uniform2f", &["Uniform2fARB"]));
_Uniform2fv = transmute(load(func, "Uniform2fv", &["Uniform2fvARB"]));
_Uniform2i = transmute(load(func, "Uniform2i", &["Uniform2iARB"]));
_Uniform2iv = transmute(load(func, "Uniform2iv", &["Uniform2ivARB"]));
_Uniform2ui = transmute(load(func, "Uniform2ui", &["Uniform2uiEXT"]));
_Uniform2uiv = transmute(load(func, "Uniform2uiv", &["Uniform2uivEXT"]));
_Uniform3d = transmute(load(func, "Uniform3d", &[]));
_Uniform3dv = transmute(load(func, "Uniform3dv", &[]));
_Uniform3f = transmute(load(func, "Uniform3f", &["Uniform3fARB"]));
_Uniform3fv = transmute(load(func, "Uniform3fv", &["Uniform3fvARB"]));
_Uniform3i = transmute(load(func, "Uniform3i", &["Uniform3iARB"]));
_Uniform3iv = transmute(load(func, "Uniform3iv", &["Uniform3ivARB"]));
_Uniform3ui = transmute(load(func, "Uniform3ui", &["Uniform3uiEXT"]));
_Uniform3uiv = transmute(load(func, "Uniform3uiv", &["Uniform3uivEXT"]));
_Uniform4d = transmute(load(func, "Uniform4d", &[]));
_Uniform4dv = transmute(load(func, "Uniform4dv", &[]));
_Uniform4f = transmute(load(func, "Uniform4f", &["Uniform4fARB"]));
_Uniform4fv = transmute(load(func, "Uniform4fv", &["Uniform4fvARB"]));
_Uniform4i = transmute(load(func, "Uniform4i", &["Uniform4iARB"]));
_Uniform4iv = transmute(load(func, "Uniform4iv", &["Uniform4ivARB"]));
_Uniform4ui = transmute(load(func, "Uniform4ui", &["Uniform4uiEXT"]));
_Uniform4uiv = transmute(load(func, "Uniform4uiv", &["Uniform4uivEXT"]));
_UniformBlockBinding = transmute(load(func, "UniformBlockBinding", &[]));
_UniformMatrix2dv = transmute(load(func, "UniformMatrix2dv", &[]));
_UniformMatrix2fv = transmute(load(func, "UniformMatrix2fv", &["UniformMatrix2fvARB"]));
_UniformMatrix2x3dv = transmute(load(func, "UniformMatrix2x3dv", &[]));
_UniformMatrix2x3fv = transmute(load(func, "UniformMatrix2x3fv", &["UniformMatrix2x3fvNV"]));
_UniformMatrix2x4dv = transmute(load(func, "UniformMatrix2x4dv", &[]));
_UniformMatrix2x4fv = transmute(load(func, "UniformMatrix2x4fv", &["UniformMatrix2x4fvNV"]));
_UniformMatrix3dv = transmute(load(func, "UniformMatrix3dv", &[]));
_UniformMatrix3fv = transmute(load(func, "UniformMatrix3fv", &["UniformMatrix3fvARB"]));
_UniformMatrix3x2dv = transmute(load(func, "UniformMatrix3x2dv", &[]));
_UniformMatrix3x2fv = transmute(load(func, "UniformMatrix3x2fv", &["UniformMatrix3x2fvNV"]));
_UniformMatrix3x4dv = transmute(load(func, "UniformMatrix3x4dv", &[]));
_UniformMatrix3x4fv = transmute(load(func, "UniformMatrix3x4fv", &["UniformMatrix3x4fvNV"]));
_UniformMatrix4dv = transmute(load(func, "UniformMatrix4dv", &[]));
_UniformMatrix4fv = transmute(load(func, "UniformMatrix4fv", &["UniformMatrix4fvARB"]));
_UniformMatrix4x2dv = transmute(load(func, "UniformMatrix4x2dv", &[]));
_UniformMatrix4x2fv = transmute(load(func, "UniformMatrix4x2fv", &["UniformMatrix4x2fvNV"]));
_UniformMatrix4x3dv = transmute(load(func, "UniformMatrix4x3dv", &[]));
_UniformMatrix4x3fv = transmute(load(func, "UniformMatrix4x3fv", &["UniformMatrix4x3fvNV"]));
_UniformSubroutinesuiv = transmute(load(func, "UniformSubroutinesuiv", &[]));
_UnmapBuffer = transmute(load(func, "UnmapBuffer", &["UnmapBufferARB", "UnmapBufferOES"]));
_UnmapNamedBuffer = transmute(load(func, "UnmapNamedBuffer", &[]));
_UseProgram = transmute(load(func, "UseProgram", &["UseProgramObjectARB"]));
_UseProgramStages = transmute(load(func, "UseProgramStages", &[]));
_ValidateProgram = transmute(load(func, "ValidateProgram", &["ValidateProgramARB"]));
_ValidateProgramPipeline = transmute(load(func, "ValidateProgramPipeline", &[]));
_VertexArrayAttribBinding = transmute(load(func, "VertexArrayAttribBinding", &[]));
_VertexArrayAttribFormat = transmute(load(func, "VertexArrayAttribFormat", &[]));
_VertexArrayAttribIFormat = transmute(load(func, "VertexArrayAttribIFormat", &[]));
_VertexArrayAttribLFormat = transmute(load(func, "VertexArrayAttribLFormat", &[]));
_VertexArrayBindingDivisor = transmute(load(func, "VertexArrayBindingDivisor", &[]));
_VertexArrayElementBuffer = transmute(load(func, "VertexArrayElementBuffer", &[]));
_VertexArrayVertexBuffer = transmute(load(func, "VertexArrayVertexBuffer", &[]));
_VertexArrayVertexBuffers = transmute(load(func, "VertexArrayVertexBuffers", &[]));
_VertexAttrib1d = transmute(load(func, "VertexAttrib1d", &["VertexAttrib1dARB", "VertexAttrib1dNV"]));
_VertexAttrib1dv = transmute(load(func, "VertexAttrib1dv", &["VertexAttrib1dvARB", "VertexAttrib1dvNV"]));
_VertexAttrib1f = transmute(load(func, "VertexAttrib1f", &["VertexAttrib1fARB", "VertexAttrib1fNV"]));
_VertexAttrib1fv = transmute(load(func, "VertexAttrib1fv", &["VertexAttrib1fvARB", "VertexAttrib1fvNV"]));
_VertexAttrib1s = transmute(load(func, "VertexAttrib1s", &["VertexAttrib1sARB", "VertexAttrib1sNV"]));
_VertexAttrib1sv = transmute(load(func, "VertexAttrib1sv", &["VertexAttrib1svARB", "VertexAttrib1svNV"]));
_VertexAttrib2d = transmute(load(func, "VertexAttrib2d", &["VertexAttrib2dARB", "VertexAttrib2dNV"]));
_VertexAttrib2dv = transmute(load(func, "VertexAttrib2dv", &["VertexAttrib2dvARB", "VertexAttrib2dvNV"]));
_VertexAttrib2f = transmute(load(func, "VertexAttrib2f", &["VertexAttrib2fARB", "VertexAttrib2fNV"]));
_VertexAttrib2fv = transmute(load(func, "VertexAttrib2fv", &["VertexAttrib2fvARB", "VertexAttrib2fvNV"]));
_VertexAttrib2s = transmute(load(func, "VertexAttrib2s", &["VertexAttrib2sARB", "VertexAttrib2sNV"]));
_VertexAttrib2sv = transmute(load(func, "VertexAttrib2sv", &["VertexAttrib2svARB", "VertexAttrib2svNV"]));
_VertexAttrib3d = transmute(load(func, "VertexAttrib3d", &["VertexAttrib3dARB", "VertexAttrib3dNV"]));
_VertexAttrib3dv = transmute(load(func, "VertexAttrib3dv", &["VertexAttrib3dvARB", "VertexAttrib3dvNV"]));
_VertexAttrib3f = transmute(load(func, "VertexAttrib3f", &["VertexAttrib3fARB", "VertexAttrib3fNV"]));
_VertexAttrib3fv = transmute(load(func, "VertexAttrib3fv", &["VertexAttrib3fvARB", "VertexAttrib3fvNV"]));
_VertexAttrib3s = transmute(load(func, "VertexAttrib3s", &["VertexAttrib3sARB", "VertexAttrib3sNV"]));
_VertexAttrib3sv = transmute(load(func, "VertexAttrib3sv", &["VertexAttrib3svARB", "VertexAttrib3svNV"]));
_VertexAttrib4Nbv = transmute(load(func, "VertexAttrib4Nbv", &["VertexAttrib4NbvARB"]));
_VertexAttrib4Niv = transmute(load(func, "VertexAttrib4Niv", &["VertexAttrib4NivARB"]));
_VertexAttrib4Nsv = transmute(load(func, "VertexAttrib4Nsv", &["VertexAttrib4NsvARB"]));
_VertexAttrib4Nub = transmute(load(func, "VertexAttrib4Nub", &["VertexAttrib4NubARB", "VertexAttrib4ubNV"]));
_VertexAttrib4Nubv = transmute(load(func, "VertexAttrib4Nubv", &["VertexAttrib4NubvARB", "VertexAttrib4ubvNV"]));
_VertexAttrib4Nuiv = transmute(load(func, "VertexAttrib4Nuiv", &["VertexAttrib4NuivARB"]));
_VertexAttrib4Nusv = transmute(load(func, "VertexAttrib4Nusv", &["VertexAttrib4NusvARB"]));
_VertexAttrib4bv = transmute(load(func, "VertexAttrib4bv", &["VertexAttrib4bvARB"]));
_VertexAttrib4d = transmute(load(func, "VertexAttrib4d", &["VertexAttrib4dARB", "VertexAttrib4dNV"]));
_VertexAttrib4dv = transmute(load(func, "VertexAttrib4dv", &["VertexAttrib4dvARB", "VertexAttrib4dvNV"]));
_VertexAttrib4f = transmute(load(func, "VertexAttrib4f", &["VertexAttrib4fARB", "VertexAttrib4fNV"]));
_VertexAttrib4fv = transmute(load(func, "VertexAttrib4fv", &["VertexAttrib4fvARB", "VertexAttrib4fvNV"]));
_VertexAttrib4iv = transmute(load(func, "VertexAttrib4iv", &["VertexAttrib4ivARB"]));
_VertexAttrib4s = transmute(load(func, "VertexAttrib4s", &["VertexAttrib4sARB", "VertexAttrib4sNV"]));
_VertexAttrib4sv = transmute(load(func, "VertexAttrib4sv", &["VertexAttrib4svARB", "VertexAttrib4svNV"]));
_VertexAttrib4ubv = transmute(load(func, "VertexAttrib4ubv", &["VertexAttrib4ubvARB"]));
_VertexAttrib4uiv = transmute(load(func, "VertexAttrib4uiv", &["VertexAttrib4uivARB"]));
_VertexAttrib4usv = transmute(load(func, "VertexAttrib4usv", &["VertexAttrib4usvARB"]));
_VertexAttribBinding = transmute(load(func, "VertexAttribBinding", &[]));
_VertexAttribDivisor = transmute(load(func, "VertexAttribDivisor", &["VertexAttribDivisorANGLE", "VertexAttribDivisorARB", "VertexAttribDivisorEXT", "VertexAttribDivisorNV"]));
_VertexAttribFormat = transmute(load(func, "VertexAttribFormat", &[]));
_VertexAttribI1i = transmute(load(func, "VertexAttribI1i", &["VertexAttribI1iEXT"]));
_VertexAttribI1iv = transmute(load(func, "VertexAttribI1iv", &["VertexAttribI1ivEXT"]));
_VertexAttribI1ui = transmute(load(func, "VertexAttribI1ui", &["VertexAttribI1uiEXT"]));
_VertexAttribI1uiv = transmute(load(func, "VertexAttribI1uiv", &["VertexAttribI1uivEXT"]));
_VertexAttribI2i = transmute(load(func, "VertexAttribI2i", &["VertexAttribI2iEXT"]));
_VertexAttribI2iv = transmute(load(func, "VertexAttribI2iv", &["VertexAttribI2ivEXT"]));
_VertexAttribI2ui = transmute(load(func, "VertexAttribI2ui", &["VertexAttribI2uiEXT"]));
_VertexAttribI2uiv = transmute(load(func, "VertexAttribI2uiv", &["VertexAttribI2uivEXT"]));
_VertexAttribI3i = transmute(load(func, "VertexAttribI3i", &["VertexAttribI3iEXT"]));
_VertexAttribI3iv = transmute(load(func, "VertexAttribI3iv", &["VertexAttribI3ivEXT"]));
_VertexAttribI3ui = transmute(load(func, "VertexAttribI3ui", &["VertexAttribI3uiEXT"]));
_VertexAttribI3uiv = transmute(load(func, "VertexAttribI3uiv", &["VertexAttribI3uivEXT"]));
_VertexAttribI4bv = transmute(load(func, "VertexAttribI4bv", &["VertexAttribI4bvEXT"]));
_VertexAttribI4i = transmute(load(func, "VertexAttribI4i", &["VertexAttribI4iEXT"]));
_VertexAttribI4iv = transmute(load(func, "VertexAttribI4iv", &["VertexAttribI4ivEXT"]));
_VertexAttribI4sv = transmute(load(func, "VertexAttribI4sv", &["VertexAttribI4svEXT"]));
_VertexAttribI4ubv = transmute(load(func, "VertexAttribI4ubv", &["VertexAttribI4ubvEXT"]));
_VertexAttribI4ui = transmute(load(func, "VertexAttribI4ui", &["VertexAttribI4uiEXT"]));
_VertexAttribI4uiv = transmute(load(func, "VertexAttribI4uiv", &["VertexAttribI4uivEXT"]));
_VertexAttribI4usv = transmute(load(func, "VertexAttribI4usv", &["VertexAttribI4usvEXT"]));
_VertexAttribIFormat = transmute(load(func, "VertexAttribIFormat", &[]));
_VertexAttribIPointer = transmute(load(func, "VertexAttribIPointer", &["VertexAttribIPointerEXT"]));
_VertexAttribL1d = transmute(load(func, "VertexAttribL1d", &["VertexAttribL1dEXT"]));
_VertexAttribL1dv = transmute(load(func, "VertexAttribL1dv", &["VertexAttribL1dvEXT"]));
_VertexAttribL2d = transmute(load(func, "VertexAttribL2d", &["VertexAttribL2dEXT"]));
_VertexAttribL2dv = transmute(load(func, "VertexAttribL2dv", &["VertexAttribL2dvEXT"]));
_VertexAttribL3d = transmute(load(func, "VertexAttribL3d", &["VertexAttribL3dEXT"]));
_VertexAttribL3dv = transmute(load(func, "VertexAttribL3dv", &["VertexAttribL3dvEXT"]));
_VertexAttribL4d = transmute(load(func, "VertexAttribL4d", &["VertexAttribL4dEXT"]));
_VertexAttribL4dv = transmute(load(func, "VertexAttribL4dv", &["VertexAttribL4dvEXT"]));
_VertexAttribLFormat = transmute(load(func, "VertexAttribLFormat", &[]));
_VertexAttribLPointer = transmute(load(func, "VertexAttribLPointer", &["VertexAttribLPointerEXT"]));
_VertexAttribP1ui = transmute(load(func, "VertexAttribP1ui", &[]));
_VertexAttribP1uiv = transmute(load(func, "VertexAttribP1uiv", &[]));
_VertexAttribP2ui = transmute(load(func, "VertexAttribP2ui", &[]));
_VertexAttribP2uiv = transmute(load(func, "VertexAttribP2uiv", &[]));
_VertexAttribP3ui = transmute(load(func, "VertexAttribP3ui", &[]));
_VertexAttribP3uiv = transmute(load(func, "VertexAttribP3uiv", &[]));
_VertexAttribP4ui = transmute(load(func, "VertexAttribP4ui", &[]));
_VertexAttribP4uiv = transmute(load(func, "VertexAttribP4uiv", &[]));
_VertexAttribPointer = transmute(load(func, "VertexAttribPointer", &["VertexAttribPointerARB"]));
_VertexBindingDivisor = transmute(load(func, "VertexBindingDivisor", &[]));
_VertexP2ui = transmute(load(func, "VertexP2ui", &[]));
_VertexP2uiv = transmute(load(func, "VertexP2uiv", &[]));
_VertexP3ui = transmute(load(func, "VertexP3ui", &[]));
_VertexP3uiv = transmute(load(func, "VertexP3uiv", &[]));
_VertexP4ui = transmute(load(func, "VertexP4ui", &[]));
_VertexP4uiv = transmute(load(func, "VertexP4uiv", &[]));
_Viewport = transmute(load(func, "Viewport", &[]));
_ViewportArrayv = transmute(load(func, "ViewportArrayv", &["ViewportArrayvNV", "ViewportArrayvOES"]));
_ViewportIndexedf = transmute(load(func, "ViewportIndexedf", &["ViewportIndexedfOES", "ViewportIndexedfNV"]));
_ViewportIndexedfv = transmute(load(func, "ViewportIndexedfv", &["ViewportIndexedfvOES", "ViewportIndexedfvNV"]));
_WaitSync = transmute(load(func, "WaitSync", &["WaitSyncAPPLE"]));
}