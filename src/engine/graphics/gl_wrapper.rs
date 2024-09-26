#![allow(non_snake_case)]

use std::{ffi::{c_void, CString}, fmt::Debug, ops::{Deref, DerefMut}};

use crate::engine::errors::{Result, GraphicsError};

use gl46::*;
use glfw::GLProc;

use self::private::Sealed;

mod private {
    pub trait Sealed {}
}

pub trait GLType : Sealed + Clone + Copy + Debug {
    fn gl_type() -> GLenum;
}

impl Sealed for bool {}
impl GLType for bool {
    fn gl_type() -> GLenum {
        GL_BOOL
    }
}

impl Sealed for i8 {}
impl GLType for i8 {
    fn gl_type() -> GLenum {
        GL_BYTE
    }
}

impl Sealed for u8 {}
impl GLType for u8 {
    fn gl_type() -> GLenum {
        GL_UNSIGNED_BYTE
    }
}

impl Sealed for i16 {}
impl GLType for i16 {
    fn gl_type() -> GLenum {
        GL_SHORT
    }
}

impl Sealed for u16 {}
impl GLType for u16 {
    fn gl_type() -> GLenum {
        GL_UNSIGNED_SHORT
    }
}

impl Sealed for i32 {}
impl GLType for i32 {
    fn gl_type() -> GLenum {
        GL_INT
    }
}

impl Sealed for u32 {}
impl GLType for u32 {
    fn gl_type() -> GLenum {
        GL_UNSIGNED_INT
    }
}

impl Sealed for f32 {}
impl GLType for f32 {
    fn gl_type() -> GLenum {
        GL_FLOAT
    }
}

impl Sealed for f64 {}
impl GLType for f64 {
    fn gl_type() -> GLenum {
        GL_DOUBLE
    }
}

pub struct Attrib {
    pub name: String,
    pub type_: AttributeType,
    pub size: i32
}
    
pub struct Uniform {
    pub name: String,
    pub type_: UniformType,
    pub size: i32
}

pub struct CStringArray {
    strings: Box<[CString]>,
    ptrs: Box<[*const u8]>
}

impl CStringArray {
    pub fn new(strings: &[&str]) -> CStringArray {
        let strings: Box<[CString]> = strings.iter().map(|s| CString::new(*s).unwrap()).collect();
        let ptrs = strings.iter().map(|s| s.as_ptr() as _).collect();

        CStringArray { strings, ptrs }
    }

    pub fn as_ptr(&self) -> *const *const u8 {
        self.ptrs.as_ptr()
    }
}

impl Deref for CStringArray {
    type Target = [CString];

    fn deref(&self) -> &Self::Target {
        &self.strings
    }
}

impl DerefMut for CStringArray {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.strings
    }
}

// ## I commented this out because I was only using it once. ##
// ## If I need this functionality again, uncomment it.      ##

// struct StringPointer<'a> {
//     string: &'a str,
//     ptr: *const u8,
//     len: i32
// }

// impl StringPointer<'_> {
//     pub fn new<'a>(string: &'a str) -> StringPointer<'a> {
//         StringPointer { string, ptr: string.as_ptr(), len: string.len() as i32 }
//     }

//     pub fn as_ptr(&self) -> *const *const u8 {
//         &self.ptr
//     }

//     pub fn len_ptr(&self) -> *const i32 {
//         &self.len
//     }

//     pub fn len(&self) -> usize {
//         self.string.len()
//     }
// }

pub struct GLWrapper {
    fns: GlFns
}

impl GLWrapper {
    pub(in crate::engine::graphics) fn init_gl<F: Fn(*const u8) -> GLProc>(f: F) -> Result<GLWrapper> {
        let fns = unsafe { GlFns::load_from(&f).map_err(|e| GraphicsError::GLLoadError { msg: e })? };

        Ok(GLWrapper { fns })
    }

    pub fn glActiveTexture(&self, texture: TextureUnit) {
        unsafe { self.fns.ActiveTexture(texture) }
    }
    
    pub fn glAttachShader(&self, program: u32, shader: u32) {
        self.fns.AttachShader(program, shader)
    }
    
    pub fn glBeginConditionalRender(&self, id: u32, mode: ConditionalRenderMode) {
        unsafe { self.fns.BeginConditionalRender(id, mode) }
    }
    
    pub fn glBeginQuery(&self, target: QueryTarget, id: u32) {
        unsafe { self.fns.BeginQuery(target, id) }
    }
    
    pub fn glBeginTransformFeedback(&self, primitiveMode: PrimitiveType) {
        unsafe { self.fns.BeginTransformFeedback(primitiveMode) }
    }
    
    pub fn glBindAttribLocation(&self, program: u32, index: u32, name: &str) {
        let null_str = CString::new(name).unwrap();
        unsafe { self.fns.BindAttribLocation(program, index, null_str.as_ptr() as _) }
    }
    
    pub fn glBindBuffer(&self, target: BufferTargetARB, buffer: u32) {
        unsafe { self.fns.BindBuffer(target, buffer) }
    }
    
    pub fn glBindBufferBase(&self, target: BufferTargetARB, index: u32, buffer: u32) {
        unsafe { self.fns.BindBufferBase(target, index, buffer) }
    }
    
    pub fn glBindBufferRange(&self, target: BufferTargetARB, index: u32, buffer: u32, offset: isize, size: isize) {
        unsafe { self.fns.BindBufferRange(target, index, buffer, offset, size) }
    }
    
    pub fn glBindFragDataLocation(&self, program: u32, color: u32, name: &str) {
        let null_str = CString::new(name).unwrap();
        unsafe { self.fns.BindFragDataLocation(program, color, null_str.as_ptr() as _) }
    }
    
    pub fn glBindFragDataLocationIndexed(&self, program: u32, colorNumber: u32, index: u32, name: &str) {
        let null_str = CString::new(name).unwrap();
        unsafe { self.fns.BindFragDataLocationIndexed(program, colorNumber, index, null_str.as_ptr() as _) }
    }
    
    pub fn glBindFramebuffer(&self, target: FramebufferTarget, framebuffer: u32) {
        unsafe { self.fns.BindFramebuffer(target, framebuffer) }
    }
    
    pub fn glBindRenderbuffer(&self, target: RenderbufferTarget, renderbuffer: u32) {
        unsafe { self.fns.BindRenderbuffer(target, renderbuffer) }
    }
    
    pub fn glBindSampler(&self, unit: u32, sampler: u32) {
        unsafe { self.fns.BindSampler(unit, sampler) }
    }
    
    pub fn glBindTexture(&self, target: TextureTarget, texture: u32) {
        unsafe { self.fns.BindTexture(target, texture) }
    }
    
    pub fn glBindVertexArray(&self, array: u32) {
        self.fns.BindVertexArray(array)
    }
    
    pub fn glBlendColor(&self, red: f32, green: f32, blue: f32, alpha: f32) {
        unsafe { self.fns.BlendColor(red, green, blue, alpha) }
    }
    
    pub fn glBlendEquation(&self, mode: BlendEquationModeEXT) {
        unsafe { self.fns.BlendEquation(mode) }
    }
    
    pub fn glBlendEquationSeparate(&self, modeRGB: BlendEquationModeEXT, modeAlpha: BlendEquationModeEXT) {
        unsafe { self.fns.BlendEquationSeparate(modeRGB, modeAlpha) }
    }
    
    pub fn glBlendFunc(&self, sfactor: BlendingFactor, dfactor: BlendingFactor) {
        unsafe { self.fns.BlendFunc(sfactor, dfactor) }
    }
    
    pub fn glBlendFuncSeparate(&self, sfactorRGB: BlendingFactor, dfactorRGB: BlendingFactor, sfactorAlpha: BlendingFactor, dfactorAlpha: BlendingFactor) {
        unsafe { self.fns.BlendFuncSeparate(sfactorRGB, dfactorRGB, sfactorAlpha, dfactorAlpha) }
    }
    
    pub fn glBlitFramebuffer(&self, srcX0: i32, srcY0: i32, srcX1: i32, srcY1: i32, dstX0: i32, dstY0: i32, dstX1: i32, dstY1: i32, mask: GLbitfield, filter: BlitFramebufferFilter) {
        unsafe { self.fns.BlitFramebuffer(srcX0, srcY0, srcX1, srcY1, dstX0, dstY0, dstX1, dstY1, mask, filter) }
    }
    
    pub fn glBufferData<T>(&self, target: BufferTargetARB, data: &[T], usage: BufferUsageARB) {
        unsafe { self.fns.BufferData(target, (data.len() * std::mem::size_of::<T>()) as _, data.as_ptr() as _, usage) }
    }
    
    pub fn glBufferSubData<T>(&self, target: BufferTargetARB, offset: isize, data: &[T]) {
        unsafe { self.fns.BufferSubData(target, offset, (data.len() * std::mem::size_of::<T>()) as _, data.as_ptr() as _) }
    }

    pub fn glBufferNull(&self, target: BufferTargetARB, size: usize, usage: BufferUsageARB) {
        unsafe { self.fns.BufferData(target, size as _, std::ptr::null(), usage )}
    }
    
    pub fn glCheckFramebufferStatus(&self, target: FramebufferTarget) -> FramebufferStatus {
        unsafe { self.fns.CheckFramebufferStatus(target) }
    }
    
    pub fn glClampColor(&self, target: ClampColorTargetARB, clamp: ClampColorModeARB) {
        unsafe { self.fns.ClampColor(target, clamp) }
    }
    
    pub fn glClear(&self, mask: GLbitfield) {
        unsafe { self.fns.Clear(mask) }
    }
    
    pub fn glClearBufferfi(&self, buffer: Buffer, drawbuffer: i32, depth: f32, stencil: i32) {
        unsafe { self.fns.ClearBufferfi(buffer, drawbuffer, depth as _, stencil) }
    }
    
    pub fn glClearBufferfv(&self, buffer: Buffer, drawbuffer: i32, value: &[f32]) {
        let expected_size = match buffer {
            GL_COLOR => 4,
            GL_DEPTH => 1,
            GL_STENCIL => 1,
            _ => -1
        };

        if value.len() as i32 != expected_size {
            panic!("Incorrect value size!")
        }

        unsafe { self.fns.ClearBufferfv(buffer, drawbuffer, value.as_ptr()) }
    }
    
    pub fn glClearBufferiv(&self, buffer: Buffer, drawbuffer: i32, value: &[i32]) {
        let expected_size = match buffer {
            GL_COLOR => 4,
            GL_DEPTH => 1,
            GL_STENCIL => 1,
            _ => -1
        };

        if value.len() as i32 != expected_size {
            panic!("Incorrect value size!")
        }

        unsafe { self.fns.ClearBufferiv(buffer, drawbuffer, value.as_ptr()) }
    }
    
    pub fn glClearBufferuiv(&self, buffer: Buffer, drawbuffer: i32, value: &[u32]) {
        let expected_size = match buffer {
            GL_COLOR => 4,
            GL_DEPTH => 1,
            GL_STENCIL => 1,
            _ => -1
        };

        if value.len() as i32 != expected_size {
            panic!("Incorrect value size!")
        }

        unsafe { self.fns.ClearBufferuiv(buffer, drawbuffer, value.as_ptr()) }
    }
    
    pub fn glClearColor(&self, red: f32, green: f32, blue: f32, alpha: f32) {
        unsafe { self.fns.ClearColor(red, green, blue, alpha) }
    }
    
    pub fn glClearDepth(&self, depth: f64) {
        unsafe { self.fns.ClearDepth(depth) }
    }
    
    pub fn glClearStencil(&self, s: i32) {
        unsafe { self.fns.ClearStencil(s) }
    }
    
    pub fn glClientWaitSync(&self, sync: GLsync, flags: GLbitfield, timeout: u64) -> SyncStatus {
        unsafe { self.fns.ClientWaitSync(sync, flags, timeout) }
    }
    
    pub fn glColorMask(&self, red: u8, green: u8, blue: u8, alpha: u8) {
        unsafe { self.fns.ColorMask(red, green, blue, alpha) }
    }
    
    pub fn glColorMaski(&self, index: u32, r: u8, g: u8, b: u8, a: u8) {
        unsafe { self.fns.ColorMaski(index, r, g, b, a) }
    }
    
    pub fn glCompileShader(&self, shader: u32) {
        self.fns.CompileShader(shader)
    }
    
    pub fn glCompressedTexImage1D(&self, target: TextureTarget, level: i32, internalformat: InternalFormat, width: u32, border: i32, data: &[u8]) {
        unsafe { self.fns.CompressedTexImage1D(target, level, internalformat, width as _, border, data.len() as _, data.as_ptr() as _) }
    }
    
    pub fn glCompressedTexImage2D(&self, target: TextureTarget, level: i32, internalformat: InternalFormat, width: u32, height: u32, border: i32, data: &[u8]) {
        unsafe { self.fns.CompressedTexImage2D(target, level, internalformat, width as _, height as _, border, data.len() as _, data.as_ptr() as _) }
    }
    
    pub fn glCompressedTexImage3D(&self, target: TextureTarget, level: i32, internalformat: InternalFormat, width: u32, height: u32, depth: u32, border: i32, data: &[u8]) {
        unsafe { self.fns.CompressedTexImage3D(target, level, internalformat, width as _, height as _, depth as _, border, data.len() as _, data.as_ptr() as _) }
    }
    
    pub fn glCompressedTexSubImage1D(&self, target: TextureTarget, level: i32, xoffset: i32, width: u32, format: PixelFormat, data: &[u8]) {
        unsafe { self.fns.CompressedTexSubImage1D(target, level, xoffset, width as _, format, data.len() as _, data.as_ptr() as _) }
    }
    
    pub fn glCompressedTexSubImage2D(&self, target: TextureTarget, level: i32, xoffset: i32, yoffset: i32, width: u32, height: u32, format: PixelFormat, data: &[u8]) {
        unsafe { self.fns.CompressedTexSubImage2D(target, level, xoffset, yoffset, width as _, height as _, format, data.len() as _, data.as_ptr() as _) }
    }
    
    pub fn glCompressedTexSubImage3D(&self, target: TextureTarget, level: i32, xoffset: i32, yoffset: i32, zoffset: i32, width: u32, height: u32, depth: u32, format: PixelFormat, data: &[u8]) {
        unsafe { self.fns.CompressedTexSubImage3D(target, level, xoffset, yoffset, zoffset, width as _, height as _, depth as _, format, data.len() as _, data.as_ptr() as _) }
    }
    
    pub fn glCopyBufferSubData(&self, readTarget: CopyBufferSubDataTarget, writeTarget: CopyBufferSubDataTarget, readOffset: isize, writeOffset: isize, size: isize) {
        unsafe { self.fns.CopyBufferSubData(readTarget, writeTarget, readOffset, writeOffset, size) }
    }
    
    pub fn glCopyTexImage1D(&self, target: TextureTarget, level: i32, internalformat: InternalFormat, x: i32, y: i32, width: u32, border: i32) {
        unsafe { self.fns.CopyTexImage1D(target, level, internalformat, x, y, width as _, border) }
    }
    
    pub fn glCopyTexImage2D(&self, target: TextureTarget, level: i32, internalformat: InternalFormat, x: i32, y: i32, width: u32, height: u32, border: i32) {
        unsafe { self.fns.CopyTexImage2D(target, level, internalformat, x, y, width as _, height as _, border) }
    }
    
    pub fn glCopyTexSubImage1D(&self, target: TextureTarget, level: i32, xoffset: i32, x: i32, y: i32, width: u32) {
        unsafe { self.fns.CopyTexSubImage1D(target, level, xoffset, x, y, width as _) }
    }
    
    pub fn glCopyTexSubImage2D(&self, target: TextureTarget, level: i32, xoffset: i32, yoffset: i32, x: i32, y: i32, width: u32, height: u32) {
        unsafe { self.fns.CopyTexSubImage2D(target, level, xoffset, yoffset, x, y, width as _, height as _) }
    }
    
    pub fn glCopyTexSubImage3D(&self, target: TextureTarget, level: i32, xoffset: i32, yoffset: i32, zoffset: i32, x: i32, y: i32, width: u32, height: u32) {
        unsafe { self.fns.CopyTexSubImage3D(target, level, xoffset, yoffset, zoffset, x, y, width as _, height as _) }
    }
    
    pub fn glCreateProgram(&self) -> u32 {
        self.fns.CreateProgram()
    }
    
    pub fn glCreateShader(&self, type_: ShaderType) -> u32 {
        self.fns.CreateShader(type_)
    }
    
    pub fn glCullFace(&self, mode: CullFaceMode) {
        unsafe { self.fns.CullFace(mode) }
    }
    
    pub fn glDeleteBuffers(&self, buffers: &[u32]) {
        unsafe { self.fns.DeleteBuffers(buffers.len() as _, buffers.as_ptr()) }
    }
    
    pub fn glDeleteFramebuffers(&self, framebuffers: &[u32]) {
        unsafe { self.fns.DeleteFramebuffers(framebuffers.len() as _, framebuffers.as_ptr()) }
    }
    
    pub fn glDeleteProgram(&self, program: u32) {
        self.fns.DeleteProgram(program)
    }
    
    pub fn glDeleteQueries(&self, ids: &[u32]) {
        unsafe { self.fns.DeleteQueries(ids.len() as _, ids.as_ptr()) }
    }
    
    pub fn glDeleteRenderbuffers(&self, renderbuffers: &[u32]) {
        unsafe { self.fns.DeleteRenderbuffers(renderbuffers.len() as _, renderbuffers.as_ptr()) }
    }
    
    pub fn glDeleteSamplers(&self, samplers: &[u32]) {
        unsafe { self.fns.DeleteSamplers(samplers.len() as _, samplers.as_ptr()) }
    }
    
    pub fn glDeleteShader(&self, shader: u32) {
        self.fns.DeleteShader(shader)
    }
    
    pub fn glDeleteSync(&self, sync: GLsync) {
        unsafe { self.fns.DeleteSync(sync) }
    }
    
    pub fn glDeleteTextures(&self, textures: &[u32]) {
        unsafe { self.fns.DeleteTextures(textures.len() as _, textures.as_ptr()) }
    }
    
    pub fn glDeleteVertexArrays(&self, arrays: &[u32]) {
        unsafe { self.fns.DeleteVertexArrays(arrays.len() as _, arrays.as_ptr()) }
    }
    
    pub fn glDepthFunc(&self, func: DepthFunction) {
        unsafe { self.fns.DepthFunc(func) }
    }
    
    pub fn glDepthMask(&self, flag: u8) {
        unsafe { self.fns.DepthMask(flag) }
    }
    
    pub fn glDepthRange(&self, n: f64, f: f64) {
        unsafe { self.fns.DepthRange(n, f) }
    }
    
    pub fn glDetachShader(&self, program: u32, shader: u32) {
        unsafe { self.fns.DetachShader(program, shader) }
    }
    
    pub fn glDisable(&self, cap: EnableCap) {
        unsafe { self.fns.Disable(cap) }
    }
    
    pub fn glDisableVertexAttribArray(&self, index: u32) {
        unsafe { self.fns.DisableVertexAttribArray(index) }
    }
    
    pub fn glDisablei(&self, target: EnableCap, index: u32) {
        unsafe { self.fns.Disablei(target, index) }
    }
    
    pub fn glDrawArrays(&self, mode: PrimitiveType, first: i32, count: i32) {
        unsafe { self.fns.DrawArrays(mode, first, count) }
    }
    
    pub fn glDrawArraysInstanced(&self, mode: PrimitiveType, first: i32, count: i32, instancecount: u32) {
        unsafe { self.fns.DrawArraysInstanced(mode, first, count, instancecount as _) }
    }
    
    pub fn glDrawBuffer(&self, buf: DrawBufferMode) {
        unsafe { self.fns.DrawBuffer(buf) }
    }
    
    pub fn glDrawBuffers(&self, bufs: &[DrawBufferMode]) {
        unsafe { self.fns.DrawBuffers(bufs.len() as _, bufs.as_ptr()) }
    }
    
    pub fn glDrawElements(&self, mode: PrimitiveType, indices: &[u32]) {
        unsafe { self.fns.DrawElements(mode, indices.len() as _, GL_UNSIGNED_INT, indices.as_ptr() as _) }
    }
    
    pub fn glDrawElementsBaseVertex(&self, mode: PrimitiveType, indices: &[u32], basevertex: i32) {
        unsafe { self.fns.DrawElementsBaseVertex(mode, indices.len() as _, GL_UNSIGNED_INT, indices.as_ptr() as _, basevertex) }
    }
    
    pub fn glDrawElementsInstanced(&self, mode: PrimitiveType, indices: &[u32], instancecount: i32) {
        unsafe { self.fns.DrawElementsInstanced(mode, indices.len() as _, GL_UNSIGNED_INT, indices.as_ptr() as _, instancecount) }
    }
    
    pub fn glDrawElementsInstancedBaseVertex(&self, mode: PrimitiveType, indices: &[u32], instancecount: i32, basevertex: i32) {
        unsafe { self.fns.DrawElementsInstancedBaseVertex(mode, indices.len() as _, GL_UNSIGNED_INT, indices.as_ptr() as _, instancecount, basevertex) }
    }
    
    pub fn glDrawRangeElements(&self, mode: PrimitiveType, start: u32, end: u32, indices: &[u32]) {
        unsafe { self.fns.DrawRangeElements(mode, start, end, indices.len() as _, GL_UNSIGNED_INT, indices.as_ptr() as _) }
    }
    
    pub fn glDrawRangeElementsBaseVertex(&self, mode: PrimitiveType, start: u32, end: u32, indices: &[u32], basevertex: i32) {
        unsafe { self.fns.DrawRangeElementsBaseVertex(mode, start, end, indices.len() as _, GL_UNSIGNED_INT, indices.as_ptr() as _, basevertex) }
    }
    
    pub fn glEnable(&self, cap: EnableCap) {
        unsafe { self.fns.Enable(cap) }
    }
    
    pub fn glEnableVertexAttribArray(&self, index: u32) {
        unsafe { self.fns.EnableVertexAttribArray(index) }
    }
    
    pub fn glEnablei(&self, target: EnableCap, index: u32) {
        unsafe { self.fns.Enablei(target, index) }
    }
    
    pub fn glEndConditionalRender(&self) {
        unsafe { self.fns.EndConditionalRender() }
    }
    
    pub fn glEndQuery(&self, target: QueryTarget) {
        unsafe { self.fns.EndQuery(target) }
    }
    
    pub fn glEndTransformFeedback(&self) {
        unsafe { self.fns.EndTransformFeedback() }
    }
    
    pub fn glFenceSync(&self, condition: SyncCondition, flags: GLbitfield) -> GLsync {
        unsafe { self.fns.FenceSync(condition, flags) }
    }
    
    pub fn glFinish(&self) {
        unsafe { self.fns.Finish() }
    }
    
    pub fn glFlush(&self) {
        unsafe { self.fns.Flush() }
    }
    
    pub fn glFlushMappedBufferRange(&self, target: BufferTargetARB, offset: isize, length: isize) {
        unsafe { self.fns.FlushMappedBufferRange(target, offset, length) }
    }
    
    pub fn glFramebufferRenderbuffer(&self, target: FramebufferTarget, attachment: FramebufferAttachment, renderbuffertarget: RenderbufferTarget, renderbuffer: u32) {
        unsafe { self.fns.FramebufferRenderbuffer(target, attachment, renderbuffertarget, renderbuffer) }
    }
    
    pub fn glFramebufferTexture(&self, target: FramebufferTarget, attachment: FramebufferAttachment, texture: u32, level: i32) {
        unsafe { self.fns.FramebufferTexture(target, attachment, texture, level) }
    }
    
    pub fn glFramebufferTexture1D(&self, target: FramebufferTarget, attachment: FramebufferAttachment, textarget: TextureTarget, texture: u32, level: i32) {
        unsafe { self.fns.FramebufferTexture1D(target, attachment, textarget, texture, level) }
    }
    
    pub fn glFramebufferTexture2D(&self, target: FramebufferTarget, attachment: FramebufferAttachment, textarget: TextureTarget, texture: u32, level: i32) {
        unsafe { self.fns.FramebufferTexture2D(target, attachment, textarget, texture, level) }
    }
    
    pub fn glFramebufferTexture3D(&self, target: FramebufferTarget, attachment: FramebufferAttachment, textarget: TextureTarget, texture: u32, level: i32, zoffset: i32) {
        unsafe { self.fns.FramebufferTexture3D(target, attachment, textarget, texture, level, zoffset) }
    }
    
    pub fn glFramebufferTextureLayer(&self, target: FramebufferTarget, attachment: FramebufferAttachment, texture: u32, level: i32, layer: i32) {
        unsafe { self.fns.FramebufferTextureLayer(target, attachment, texture, level, layer) }
    }
    
    pub fn glFrontFace(&self, mode: FrontFaceDirection) {
        unsafe { self.fns.FrontFace(mode) }
    }
    
    pub fn glGenBuffers(&self, buffers: &mut [u32]) {
        unsafe { self.fns.GenBuffers(buffers.len() as _, buffers.as_mut_ptr()) }
    }
    
    pub fn glGenBuffer(&self, buffer: &mut u32) {
        unsafe { self.fns.GenBuffers(1, buffer) }
    }
    
    pub fn glGenFramebuffers(&self, framebuffers: &mut [u32]) {
        unsafe { self.fns.GenFramebuffers(framebuffers.len() as _, framebuffers.as_mut_ptr()) }
    }
    
    pub fn glGenFramebuffer(&self, framebuffer: &mut u32) {
        unsafe { self.fns.GenFramebuffers(1, framebuffer) }
    }
    
    pub fn glGenQueries(&self, ids: &mut [u32]) {
        unsafe { self.fns.GenQueries(ids.len() as _, ids.as_mut_ptr()) }
    }
    
    pub fn glGenQuery(&self, id: &mut u32) {
        unsafe { self.fns.GenQueries(1, id) }
    }
    
    pub fn glGenRenderbuffers(&self, renderbuffers: &mut [u32]) {
        unsafe { self.fns.GenRenderbuffers(renderbuffers.len() as _, renderbuffers.as_mut_ptr()) }
    }
    
    pub fn glGenRenderbuffer(&self, renderbuffer: &mut u32) {
        unsafe { self.fns.GenRenderbuffers(1, renderbuffer) }
    }
    
    pub fn glGenSamplers(&self, samplers: &mut [u32]) {
        unsafe { self.fns.GenSamplers(samplers.len() as _, samplers.as_mut_ptr()) }
    }
    
    pub fn glGenSampler(&self, sampler: &mut u32) {
        unsafe { self.fns.GenSamplers(1, sampler) }
    }
    
    pub fn glGenTextures(&self, textures: &mut [u32]) {
        unsafe { self.fns.GenTextures(textures.len() as _, textures.as_mut_ptr()) }
    }
    
    pub fn glGenTexture(&self, texture: &mut u32) {
        unsafe { self.fns.GenTextures(1, texture) }
    }
    
    pub fn glGenVertexArrays(&self, arrays: &mut [u32]) {
        unsafe { self.fns.GenVertexArrays(arrays.len() as _, arrays.as_mut_ptr()) }
    }
    
    pub fn glGenVertexArray(&self, array: &mut u32) {
        unsafe { self.fns.GenVertexArrays(1, array) }
    }
    
    pub fn glGenerateMipmap(&self, target: TextureTarget) {
        unsafe { self.fns.GenerateMipmap(target) }
    }

    // NEEDS TESTING
    pub fn glGetActiveAttrib(&self, program: u32, index: u32) -> Attrib {
        let mut name = [0u8; 100];
        let mut size = 0i32;
        let mut type_ = GL_INT;
        let mut length = 0i32;

        unsafe { self.fns.GetActiveAttrib(program, index, 100, &mut length as _, &mut size as _, &mut type_ as _, name.as_mut_ptr()); }

        let name = String::from_utf8_lossy(&name[..length as _]).into_owned();

        Attrib { name, type_, size }
    }

    pub fn glGetActiveUniform(&self, program: u32, index: u32) -> Uniform {
        let mut name = [0u8; 100];
        let mut size = 0i32;
        let mut type_ = GL_INT;
        let mut length = 0i32;

        unsafe {self.fns.GetActiveUniform(program, index, 100, &mut length as _, &mut size as _, &mut type_ as _, name.as_mut_ptr()); }

        let name = String::from_utf8_lossy(&name[0..length as _]).into_owned();

        Uniform { name, type_, size }
    }
    
    pub fn glGetActiveUniformBlockName(&self, program: u32, uniformBlockIndex: u32) -> String {
        let mut uniformBlockName = [0u8; 100];
        let mut length = 0i32;

        unsafe { self.fns.GetActiveUniformBlockName(program, uniformBlockIndex, 100, &mut length as _, uniformBlockName.as_mut_ptr()); }

        String::from_utf8_lossy(&uniformBlockName[..length as _]).into_owned()
    }
    
    pub unsafe fn glGetActiveUniformBlockiv(&self, program: u32, uniformBlockIndex: u32, pname: UniformBlockPName, params: *mut i32) {
        self.fns.GetActiveUniformBlockiv(program, uniformBlockIndex, pname, params)
    }
    
    pub fn glGetActiveUniformName(&self, program: u32, uniformIndex: u32) -> String {
        let mut uniformName = [0u8; 100];
        let mut length = 0i32;

        unsafe { self.fns.GetActiveUniformName(program, uniformIndex, 100, &mut length as _, uniformName.as_mut_ptr()); }

        String::from_utf8_lossy(&uniformName[..length as _]).into_owned()
    }
    
    pub fn glGetActiveUniformsiv(&self, program: u32, uniformIndices: &[u32], pname: UniformPName) -> Box<[i32]> {
        let mut params = vec![0; uniformIndices.len()].into_boxed_slice();

        unsafe { self.fns.GetActiveUniformsiv(program, uniformIndices.len() as _, uniformIndices.as_ptr() as _, pname, params.as_mut_ptr()) }

        params
    }
    
    pub fn glGetAttachedShaders(&self, program: u32) -> Box<[u32]> {
        let mut shaders = [0u32; 100];
        let mut count = 0i32;

        unsafe { self.fns.GetAttachedShaders(program, 100, &mut count as _, shaders.as_mut_ptr()); }

        let mut shaders = shaders.to_vec();
        shaders.truncate(count as _);

        shaders.into_boxed_slice()
    }
    
    pub fn glGetAttribLocation(&self, program: u32, name: &str) -> i32 {
        let null_str = CString::new(name).unwrap();
        unsafe { self.fns.GetAttribLocation(program, null_str.as_ptr() as _) }
    }
    
    pub unsafe fn glGetBooleani_v(&self, target: BufferTargetARB, index: u32, data: *mut u8) {
        self.fns.GetBooleani_v(target, index, data)
    }
    
    pub unsafe fn glGetBooleanv(&self, pname: GetPName, data: *mut u8) {
        self.fns.GetBooleanv(pname, data)
    }
    
    pub unsafe fn glGetBufferParameteri64v(&self, target: BufferTargetARB, pname: BufferPNameARB, params: *mut i64) {
        self.fns.GetBufferParameteri64v(target, pname, params)
    }
    
    pub unsafe fn glGetBufferParameteriv(&self, target: BufferTargetARB, pname: BufferPNameARB, params: *mut i32) {
        self.fns.GetBufferParameteriv(target, pname, params)
    }
    
    pub unsafe fn glGetBufferPointerv(&self, target: BufferTargetARB, pname: BufferPointerNameARB, params: *mut *mut c_void) {
        self.fns.GetBufferPointerv(target, pname, params)
    }
    
    pub unsafe fn glGetBufferSubData(&self, target: BufferTargetARB, offset: isize, size: isize, data: *mut c_void) {
        self.fns.GetBufferSubData(target, offset, size, data)
    }
    
    pub unsafe fn glGetCompressedTexImage(&self, target: TextureTarget, level: i32, img: *mut c_void) {
        self.fns.GetCompressedTexImage(target, level, img)
    }
    
    pub unsafe fn glGetDoublev(&self, pname: GetPName, data: *mut f64) {
        self.fns.GetDoublev(pname, data)
    }
    
    pub unsafe fn glGetError(&self) -> ErrorCode {
        self.fns.GetError()
    }
    
    pub unsafe fn glGetFloatv(&self, pname: GetPName, data: *mut f32) {
        self.fns.GetFloatv(pname, data)
    }
    
    pub fn glGetFragDataIndex(&self, program: u32, name: &str) -> i32 {
        let null_str = CString::new(name).unwrap();
        unsafe { self.fns.GetFragDataIndex(program, null_str.as_ptr() as _) }
    }
    
    pub fn glGetFragDataLocation(&self, program: u32, name: &str) -> i32 {
        let null_str = CString::new(name).unwrap();
        unsafe { self.fns.GetFragDataLocation(program, null_str.as_ptr() as _) }
    }
    
    pub unsafe fn glGetFramebufferAttachmentParameteriv(&self, target: FramebufferTarget, attachment: FramebufferAttachment, pname: FramebufferAttachmentParameterName, params: *mut i32) {
        self.fns.GetFramebufferAttachmentParameteriv(target, attachment, pname, params)
    }
    
    pub unsafe fn glGetInteger64i_v(&self, target: GetPName, index: u32, data: *mut i64) {
        self.fns.GetInteger64i_v(target, index, data)
    }
    
    pub unsafe fn glGetInteger64v(&self, pname: GetPName, data: *mut i64) {
        self.fns.GetInteger64v(pname, data)
    }
    
    pub unsafe fn glGetIntegeri_v(&self, target: GetPName, index: u32, data: *mut i32) {
        self.fns.GetIntegeri_v(target, index, data)
    }
    
    pub unsafe fn glGetIntegerv(&self, pname: GetPName, data: *mut i32) {
        self.fns.GetIntegerv(pname, data)
    }
    
    pub unsafe fn glGetMultisamplefv(&self, pname: GetMultisamplePNameNV, index: u32, val: *mut f32) {
        self.fns.GetMultisamplefv(pname, index, val)
    }
    
    pub unsafe fn glGetProgramInfoLog(&self, program: u32, bufSize: i32, length: *mut i32, infoLog: *mut u8) {
        self.fns.GetProgramInfoLog(program, bufSize, length, infoLog)
    }
    
    pub unsafe fn glGetProgramiv(&self, program: u32, pname: ProgramPropertyARB, params: *mut i32) {
        self.fns.GetProgramiv(program, pname, params)
    }
    
    pub unsafe fn glGetQueryObjecti64v(&self, id: u32, pname: QueryObjectParameterName, params: *mut i64) {
        self.fns.GetQueryObjecti64v(id, pname, params)
    }
    
    pub unsafe fn glGetQueryObjectiv(&self, id: u32, pname: QueryObjectParameterName, params: *mut i32) {
        self.fns.GetQueryObjectiv(id, pname, params)
    }
    
    pub unsafe fn glGetQueryObjectui64v(&self, id: u32, pname: QueryObjectParameterName, params: *mut u64) {
        self.fns.GetQueryObjectui64v(id, pname, params)
    }
    
    pub unsafe fn glGetQueryObjectuiv(&self, id: u32, pname: QueryObjectParameterName, params: *mut u32) {
        self.fns.GetQueryObjectuiv(id, pname, params)
    }
    
    pub unsafe fn glGetQueryiv(&self, target: QueryTarget, pname: QueryParameterName, params: *mut i32) {
        self.fns.GetQueryiv(target, pname, params)
    }
    
    pub unsafe fn glGetRenderbufferParameteriv(&self, target: RenderbufferTarget, pname: RenderbufferParameterName, params: *mut i32) {
        self.fns.GetRenderbufferParameteriv(target, pname, params)
    }
    
    pub unsafe fn glGetSamplerParameterIiv(&self, sampler: u32, pname: SamplerParameterI, params: *mut i32) {
        self.fns.GetSamplerParameterIiv(sampler, pname, params)
    }
    
    pub unsafe fn glGetSamplerParameterIuiv(&self, sampler: u32, pname: SamplerParameterI, params: *mut u32) {
        self.fns.GetSamplerParameterIuiv(sampler, pname, params)
    }
    
    pub unsafe fn glGetSamplerParameterfv(&self, sampler: u32, pname: SamplerParameterF, params: *mut f32) {
        self.fns.GetSamplerParameterfv(sampler, pname, params)
    }
    
    pub unsafe fn glGetSamplerParameteriv(&self, sampler: u32, pname: SamplerParameterI, params: *mut i32) {
        self.fns.GetSamplerParameteriv(sampler, pname, params)
    }
    
    pub fn glGetShaderInfoLog(&self, shader: u32) -> String {
        let mut length = 0;
        self.glGetShaderiv(shader, GL_INFO_LOG_LENGTH, &mut length);

        let mut buffer = vec![0u8; length as usize];
        unsafe { self.fns.GetShaderInfoLog(shader, length, std::ptr::null_mut(), buffer.as_mut_ptr()) }
        // Trim null terminator and 2 newline u8acters
        buffer.truncate(buffer.len() - 3);

        String::from_utf8(buffer).unwrap()
    }
    
    pub unsafe fn glGetShaderSource(&self, shader: u32, bufSize: i32, length: *mut i32, source: *mut u8) {
        self.fns.GetShaderSource(shader, bufSize, length, source)
    }
    
    pub fn glGetShaderiv(&self, shader: u32, pname: ShaderParameterName, params: &mut i32) {
        unsafe { self.fns.GetShaderiv(shader, pname, params) }
    }
    
    pub unsafe fn glGetString(&self, name: StringName) -> String {
        let raw_str = self.fns.GetString(name);

        unsafe {
            let len = libc::strlen(raw_str as _);
            let slice = std::slice::from_raw_parts(raw_str, len);
            String::from_utf8_lossy(slice).to_string()
        }
    }
    
    pub unsafe fn glGetStringi(&self, name: StringName, index: u32) -> String {
        let raw_str = self.fns.GetStringi(name, index);

        unsafe {
            let len = libc::strlen(raw_str as _);
            let slice = std::slice::from_raw_parts(raw_str, len);
            String::from_utf8_lossy(slice).to_string()
        }
    }
    
    pub unsafe fn glGetSynciv(&self, sync: GLsync, pname: SyncParameterName, count: i32, length: *mut i32, values: *mut i32) {
        self.fns.GetSynciv(sync, pname, count, length, values)
    }
    
    pub unsafe fn glGetTexImage(&self, target: TextureTarget, level: i32, format: PixelFormat, type_: PixelType, pixels: *mut c_void) {
        self.fns.GetTexImage(target, level, format, type_, pixels)
    }
    
    pub unsafe fn glGetTexLevelParameterfv(&self, target: TextureTarget, level: i32, pname: GetTextureParameter, params: *mut f32) {
        self.fns.GetTexLevelParameterfv(target, level, pname, params)
    }
    
    pub unsafe fn glGetTexLevelParameteriv(&self, target: TextureTarget, level: i32, pname: GetTextureParameter, params: *mut i32) {
        self.fns.GetTexLevelParameteriv(target, level, pname, params)
    }
    
    pub unsafe fn glGetTexParameterIiv(&self, target: TextureTarget, pname: GetTextureParameter, params: *mut i32) {
        self.fns.GetTexParameterIiv(target, pname, params)
    }
    
    pub unsafe fn glGetTexParameterIuiv(&self, target: TextureTarget, pname: GetTextureParameter, params: *mut u32) {
        self.fns.GetTexParameterIuiv(target, pname, params)
    }
    
    pub unsafe fn glGetTexParameterfv(&self, target: TextureTarget, pname: GetTextureParameter, params: *mut f32) {
        self.fns.GetTexParameterfv(target, pname, params)
    }
    
    pub unsafe fn glGetTexParameteriv(&self, target: TextureTarget, pname: GetTextureParameter, params: *mut i32) {
        self.fns.GetTexParameteriv(target, pname, params)
    }
    
    pub unsafe fn glGetTransformFeedbackVarying(&self, program: u32, index: u32, bufSize: i32, length: *mut i32, size: *mut i32, type_: *mut AttributeType, name: *mut u8) {
        self.fns.GetTransformFeedbackVarying(program, index, bufSize, length, size, type_, name)
    }
    
    pub fn glGetUniformBlockIndex(&self, program: u32, uniformBlockName: &str) -> u32 {
        let null_str = CString::new(uniformBlockName).unwrap();
        unsafe { self.fns.GetUniformBlockIndex(program, null_str.as_ptr() as _) }
    }
    
    // NEEDS TESTING
    pub fn glGetUniformIndices(&self, program: u32, uniformNames: &[&str], uniformIndices: &mut [u32]) {
        let uniformNames = CStringArray::new(uniformNames);
        unsafe { self.fns.GetUniformIndices(program, uniformNames.len() as _, uniformNames.as_ptr(), uniformIndices.as_mut_ptr()) }
    }
    
    pub fn glGetUniformLocation(&self, program: u32, name: &str) -> i32 {
        let null_str = CString::new(name).unwrap();
        unsafe { self.fns.GetUniformLocation(program, null_str.as_ptr() as _) }
    }
    
    pub unsafe fn glGetUniformfv(&self, program: u32, location: i32, params: *mut f32) {
        self.fns.GetUniformfv(program, location, params)
    }
    
    pub unsafe fn glGetUniformiv(&self, program: u32, location: i32, params: *mut i32) {
        self.fns.GetUniformiv(program, location, params)
    }
    
    pub unsafe fn glGetUniformuiv(&self, program: u32, location: i32, params: *mut u32) {
        self.fns.GetUniformuiv(program, location, params)
    }
    
    pub unsafe fn glGetVertexAttribIiv(&self, index: u32, pname: VertexAttribEnum, params: *mut i32) {
        self.fns.GetVertexAttribIiv(index, pname, params)
    }
    
    pub unsafe fn glGetVertexAttribIuiv(&self, index: u32, pname: VertexAttribEnum, params: *mut u32) {
        self.fns.GetVertexAttribIuiv(index, pname, params)
    }
    
    pub unsafe fn glGetVertexAttribPointerv(&self, index: u32, pname: VertexAttribPointerPropertyARB, pointer: *mut *mut c_void) {
        self.fns.GetVertexAttribPointerv(index, pname, pointer)
    }
    
    pub unsafe fn glGetVertexAttribdv(&self, index: u32, pname: VertexAttribPropertyARB, params: *mut[f64; 4]) {
        self.fns.GetVertexAttribdv(index, pname, params)
    }
    
    pub unsafe fn glGetVertexAttribfv(&self, index: u32, pname: VertexAttribPropertyARB, params: *mut[f32; 4]) {
        self.fns.GetVertexAttribfv(index, pname, params)
    }
    
    pub unsafe fn glGetVertexAttribiv(&self, index: u32, pname: VertexAttribPropertyARB, params: *mut[i32; 4]) {
        self.fns.GetVertexAttribiv(index, pname, params)
    }
    
    pub unsafe fn glHint(&self, target: HintTarget, mode: HintMode) {
        self.fns.Hint(target, mode)
    }
    
    pub unsafe fn glIsBuffer(&self, buffer: u32) -> u8 {
        self.fns.IsBuffer(buffer)
    }
    
    pub unsafe fn glIsEnabled(&self, cap: EnableCap) -> u8 {
        self.fns.IsEnabled(cap)
    }
    
    pub unsafe fn glIsEnabledi(&self, target: EnableCap, index: u32) -> u8 {
        self.fns.IsEnabledi(target, index)
    }
    
    pub unsafe fn glIsFramebuffer(&self, framebuffer: u32) -> u8 {
        self.fns.IsFramebuffer(framebuffer)
    }
    
    pub unsafe fn glIsProgram(&self, program: u32) -> u8 {
        self.fns.IsProgram(program)
    }
    
    pub unsafe fn glIsQuery(&self, id: u32) -> u8 {
        self.fns.IsQuery(id)
    }
    
    pub unsafe fn glIsRenderbuffer(&self, renderbuffer: u32) -> u8 {
        self.fns.IsRenderbuffer(renderbuffer)
    }
    
    pub unsafe fn glIsSampler(&self, sampler: u32) -> u8 {
        self.fns.IsSampler(sampler)
    }
    
    pub unsafe fn glIsShader(&self, shader: u32) -> u8 {
        self.fns.IsShader(shader)
    }
    
    pub unsafe fn glIsSync(&self, sync: GLsync) -> u8 {
        self.fns.IsSync(sync)
    }
    
    pub unsafe fn glIsTexture(&self, texture: u32) -> u8 {
        self.fns.IsTexture(texture)
    }
    
    pub unsafe fn glIsVertexArray(&self, array: u32) -> u8 {
        self.fns.IsVertexArray(array)
    }
    
    pub unsafe fn glLineWidth(&self, width: f32) {
        self.fns.LineWidth(width as _)
    }
    
    pub fn glLinkProgram(&self, program: u32) {
        self.fns.LinkProgram(program)
    }
    
    pub unsafe fn glLogicOp(&self, opcode: LogicOp) {
        self.fns.LogicOp(opcode)
    }
    
    pub unsafe fn glMapBuffer(&self, target: BufferTargetARB, access: BufferAccessARB) -> *mut c_void {
        self.fns.MapBuffer(target, access)
    }
    
    pub unsafe fn glMapBufferRange(&self, target: BufferTargetARB, offset: isize, length: isize, access: GLbitfield) -> *mut c_void {
        self.fns.MapBufferRange(target, offset, length, access)
    }
    
    pub unsafe fn glMultiDrawArrays(&self, mode: PrimitiveType, first: *const i32, count: *const i32, drawcount: i32) {
        self.fns.MultiDrawArrays(mode, first, count, drawcount)
    }
    
    pub unsafe fn glMultiDrawElements(&self, mode: PrimitiveType, count: *const i32, type_: DrawElementsType, indices: *const *const c_void, drawcount: i32) {
        self.fns.MultiDrawElements(mode, count, type_, indices, drawcount)
    }
    
    pub unsafe fn glMultiDrawElementsBaseVertex(&self, mode: PrimitiveType, count: *const i32, type_: DrawElementsType, indices: *const *const c_void, drawcount: i32, basevertex: *const i32) {
        self.fns.MultiDrawElementsBaseVertex(mode, count, type_, indices, drawcount, basevertex)
    }
    
    pub unsafe fn glPixelStoref(&self, pname: PixelStoreParameter, param: f32) {
        self.fns.PixelStoref(pname, param)
    }
    
    pub unsafe fn glPixelStorei(&self, pname: PixelStoreParameter, param: i32) {
        self.fns.PixelStorei(pname, param)
    }
    
    pub unsafe fn glPointParameterf(&self, pname: PointParameterNameARB, param: f32) {
        self.fns.PointParameterf(pname, param)
    }
    
    pub unsafe fn glPointParameterfv(&self, pname: PointParameterNameARB, params: *const f32) {
        self.fns.PointParameterfv(pname, params)
    }
    
    pub unsafe fn glPointParameteri(&self, pname: PointParameterNameARB, param: i32) {
        self.fns.PointParameteri(pname, param)
    }
    
    pub unsafe fn glPointParameteriv(&self, pname: PointParameterNameARB, params: *const i32) {
        self.fns.PointParameteriv(pname, params)
    }
    
    pub fn glPointSize(&self, size: f32) {
        self.fns.PointSize(size)
    }
    
    pub unsafe fn glPolygonMode(&self, face: MaterialFace, mode: PolygonMode) {
        self.fns.PolygonMode(face, mode)
    }
    
    pub unsafe fn glPolygonOffset(&self, factor: f32, units: f32) {
        self.fns.PolygonOffset(factor, units)
    }
    
    pub unsafe fn glPrimitiveRestartIndex(&self, index: u32) {
        self.fns.PrimitiveRestartIndex(index)
    }
    
    pub unsafe fn glProvokingVertex(&self, mode: VertexProvokingMode) {
        self.fns.ProvokingVertex(mode)
    }
    
    pub unsafe fn glQueryCounter(&self, id: u32, target: QueryCounterTarget) {
        self.fns.QueryCounter(id, target)
    }
    
    pub unsafe fn glReadBuffer(&self, src: ReadBufferMode) {
        self.fns.ReadBuffer(src)
    }
    
    pub unsafe fn glReadPixels(&self, x: i32, y: i32, width: u32, height: u32, format: PixelFormat, type_: PixelType, pixels: *mut c_void) {
        self.fns.ReadPixels(x, y, width as _, height as _, format, type_, pixels)
    }
    
    pub unsafe fn glRenderbufferStorage(&self, target: RenderbufferTarget, internalformat: InternalFormat, width: u32, height: u32) {
        self.fns.RenderbufferStorage(target, internalformat, width as _, height as _)
    }
    
    pub unsafe fn glRenderbufferStorageMultisample(&self, target: RenderbufferTarget, samples: i32, internalformat: InternalFormat, width: u32, height: u32) {
        self.fns.RenderbufferStorageMultisample(target, samples, internalformat, width as _, height as _)
    }
    
    pub unsafe fn glSampleCoverage(&self, value: f32, invert: u8) {
        self.fns.SampleCoverage(value, invert)
    }
    
    pub unsafe fn glSampleMaski(&self, maskNumber: u32, mask: GLbitfield) {
        self.fns.SampleMaski(maskNumber, mask)
    }
    
    pub unsafe fn glSamplerParameterIiv(&self, sampler: u32, pname: SamplerParameterI, param: *const i32) {
        self.fns.SamplerParameterIiv(sampler, pname, param)
    }
    
    pub unsafe fn glSamplerParameterIuiv(&self, sampler: u32, pname: SamplerParameterI, param: *const u32) {
        self.fns.SamplerParameterIuiv(sampler, pname, param)
    }
    
    pub unsafe fn glSamplerParameterf(&self, sampler: u32, pname: SamplerParameterF, param: f32) {
        self.fns.SamplerParameterf(sampler, pname, param)
    }
    
    pub unsafe fn glSamplerParameterfv(&self, sampler: u32, pname: SamplerParameterF, param: *const f32) {
        self.fns.SamplerParameterfv(sampler, pname, param)
    }
    
    pub unsafe fn glSamplerParameteri(&self, sampler: u32, pname: SamplerParameterI, param: i32) {
        self.fns.SamplerParameteri(sampler, pname, param)
    }
    
    pub unsafe fn glSamplerParameteriv(&self, sampler: u32, pname: SamplerParameterI, param: *const i32) {
        self.fns.SamplerParameteriv(sampler, pname, param)
    }
    
    pub unsafe fn glScissor(&self, x: i32, y: i32, width: u32, height: u32) {
        self.fns.Scissor(x, y, width as _, height as _)
    }
    
    // NEEDS TESTING
    pub fn glShaderSource(&self, shader: u32, string: &str) {
        let string_pointer = string.as_ptr();
        let len = string.len() as i32;
        
        unsafe { self.fns.ShaderSource(shader, 1, &string_pointer, &len) }
    }
    
    pub unsafe fn glStencilFunc(&self, func: StencilFunction, ref_: i32, mask: u32) {
        self.fns.StencilFunc(func, ref_, mask)
    }
    
    pub unsafe fn glStencilFuncSeparate(&self, face: StencilFaceDirection, func: StencilFunction, ref_: i32, mask: u32) {
        self.fns.StencilFuncSeparate(face, func, ref_, mask)
    }
    
    pub unsafe fn glStencilMask(&self, mask: u32) {
        self.fns.StencilMask(mask)
    }
    
    pub unsafe fn glStencilMaskSeparate(&self, face: StencilFaceDirection, mask: u32) {
        self.fns.StencilMaskSeparate(face, mask)
    }
    
    pub unsafe fn glStencilOp(&self, fail: StencilOp, zfail: StencilOp, zpass: StencilOp) {
        self.fns.StencilOp(fail, zfail, zpass)
    }
    
    pub unsafe fn glStencilOpSeparate(&self, face: StencilFaceDirection, sfail: StencilOp, dpfail: StencilOp, dppass: StencilOp) {
        self.fns.StencilOpSeparate(face, sfail, dpfail, dppass)
    }
    
    pub unsafe fn glTexBuffer(&self, target: TextureTarget, internalformat: InternalFormat, buffer: u32) {
        self.fns.TexBuffer(target, internalformat, buffer)
    }
    
    pub unsafe fn glTexImage1D(&self, target: TextureTarget, level: i32, internalformat: InternalFormat, width: u32, border: i32, format: PixelFormat, type_: PixelType, pixels: *const c_void) {
        self.fns.TexImage1D(target, level, internalformat.0 as _, width as _, border, format, type_, pixels)
    }
    
    pub fn glTexImage2D(&self, target: TextureTarget, level: i32, internalformat: InternalFormat, width: u32, height: u32, border: i32, format: PixelFormat, type_: PixelType, pixels: &[u8]) {
        unsafe { self.fns.TexImage2D(target, level, internalformat.0 as _, width as _, height as _, border, format, type_, pixels.as_ptr() as _) }
    }
    
    pub unsafe fn glTexImage2DMultisample(&self, target: TextureTarget, samples: i32, internalformat: InternalFormat, width: u32, height: u32, fixedsamplelocations: u8) {
        self.fns.TexImage2DMultisample(target, samples, internalformat, width as _, height as _, fixedsamplelocations)
    }
    
    pub unsafe fn glTexImage3D(&self, target: TextureTarget, level: i32, internalformat: InternalFormat, width: u32, height: u32, depth: u32, border: i32, format: PixelFormat, type_: PixelType, pixels: *const c_void) {
        self.fns.TexImage3D(target, level, internalformat.0 as _, width as _, height as _, depth as _, border, format, type_, pixels)
    }
    
    pub unsafe fn glTexImage3DMultisample(&self, target: TextureTarget, samples: i32, internalformat: InternalFormat, width: u32, height: u32, depth: u32, fixedsamplelocations: u8) {
        self.fns.TexImage3DMultisample(target, samples, internalformat, width as _, height as _, depth as _, fixedsamplelocations)
    }
    
    pub unsafe fn glTexParameterIiv(&self, target: TextureTarget, pname: TextureParameterName, params: *const i32) {
        self.fns.TexParameterIiv(target, pname, params)
    }
    
    pub unsafe fn glTexParameterIuiv(&self, target: TextureTarget, pname: TextureParameterName, params: *const u32) {
        self.fns.TexParameterIuiv(target, pname, params)
    }
    
    pub unsafe fn glTexParameterf(&self, target: TextureTarget, pname: TextureParameterName, param: f32) {
        self.fns.TexParameterf(target, pname, param)
    }
    
    pub unsafe fn glTexParameterfv(&self, target: TextureTarget, pname: TextureParameterName, params: *const f32) {
        self.fns.TexParameterfv(target, pname, params)
    }
    
    pub unsafe fn glTexParameteri(&self, target: TextureTarget, pname: TextureParameterName, param: i32) {
        self.fns.TexParameteri(target, pname, param)
    }
    
    pub unsafe fn glTexParameteriv(&self, target: TextureTarget, pname: TextureParameterName, params: *const i32) {
        self.fns.TexParameteriv(target, pname, params)
    }
    
    pub unsafe fn glTexSubImage1D(&self, target: TextureTarget, level: i32, xoffset: i32, width: u32, format: PixelFormat, type_: PixelType, pixels: *const c_void) {
        self.fns.TexSubImage1D(target, level, xoffset, width as _, format, type_, pixels)
    }
    
    pub unsafe fn glTexSubImage2D(&self, target: TextureTarget, level: i32, xoffset: i32, yoffset: i32, width: u32, height: u32, format: PixelFormat, type_: PixelType, pixels: *const c_void) {
        self.fns.TexSubImage2D(target, level, xoffset, yoffset, width as _, height as _, format, type_, pixels)
    }
    
    pub unsafe fn glTexSubImage3D(&self, target: TextureTarget, level: i32, xoffset: i32, yoffset: i32, zoffset: i32, width: u32, height: u32, depth: u32, format: PixelFormat, type_: PixelType, pixels: *const c_void) {
        self.fns.TexSubImage3D(target, level, xoffset, yoffset, zoffset, width as _, height as _, depth as _, format, type_, pixels)
    }
    
    pub fn glTransformFeedbackVaryings(&self, program: u32, varyings: &[&str], bufferMode: TransformFeedbackBufferMode) {
        let varyings = CStringArray::new(varyings);
        unsafe { self.fns.TransformFeedbackVaryings(program, varyings.len() as _, varyings.as_ptr(), bufferMode) }
    }
    
    pub unsafe fn glUniform1f(&self, location: i32, v0: f32) {
        self.fns.Uniform1f(location, v0)
    }
    
    pub unsafe fn glUniform1fv(&self, location: i32, count: i32, value: *const f32) {
        self.fns.Uniform1fv(location, count, value)
    }
    
    pub unsafe fn glUniform1i(&self, location: i32, v0: i32) {
        self.fns.Uniform1i(location, v0)
    }
    
    pub unsafe fn glUniform1iv(&self, location: i32, count: i32, value: *const i32) {
        self.fns.Uniform1iv(location, count, value)
    }
    
    pub unsafe fn glUniform1ui(&self, location: i32, v0: u32) {
        self.fns.Uniform1ui(location, v0)
    }
    
    pub unsafe fn glUniform1uiv(&self, location: i32, count: i32, value: *const u32) {
        self.fns.Uniform1uiv(location, count, value)
    }
    
    pub unsafe fn glUniform2f(&self, location: i32, v0: f32, v1: f32) {
        self.fns.Uniform2f(location, v0, v1)
    }
    
    pub unsafe fn glUniform2fv(&self, location: i32, count: i32, value: *const f32) {
        self.fns.Uniform2fv(location, count, value)
    }
    
    pub unsafe fn glUniform2i(&self, location: i32, v0: i32, v1: i32) {
        self.fns.Uniform2i(location, v0, v1)
    }
    
    pub unsafe fn glUniform2iv(&self, location: i32, count: i32, value: *const i32) {
        self.fns.Uniform2iv(location, count, value)
    }
    
    pub unsafe fn glUniform2ui(&self, location: i32, v0: u32, v1: u32) {
        self.fns.Uniform2ui(location, v0, v1)
    }
    
    pub unsafe fn glUniform2uiv(&self, location: i32, count: i32, value: *const u32) {
        self.fns.Uniform2uiv(location, count, value)
    }
    
    pub unsafe fn glUniform3f(&self, location: i32, v0: f32, v1: f32, v2: f32) {
        self.fns.Uniform3f(location, v0, v1, v2)
    }
    
    pub unsafe fn glUniform3fv(&self, location: i32, count: i32, value: *const f32) {
        self.fns.Uniform3fv(location, count, value)
    }
    
    pub unsafe fn glUniform3i(&self, location: i32, v0: i32, v1: i32, v2: i32) {
        self.fns.Uniform3i(location, v0, v1, v2)
    }
    
    pub unsafe fn glUniform3iv(&self, location: i32, count: i32, value: *const i32) {
        self.fns.Uniform3iv(location, count, value)
    }
    
    pub unsafe fn glUniform3ui(&self, location: i32, v0: u32, v1: u32, v2: u32) {
        self.fns.Uniform3ui(location, v0, v1, v2)
    }
    
    pub unsafe fn glUniform3uiv(&self, location: i32, count: i32, value: *const u32) {
        self.fns.Uniform3uiv(location, count, value)
    }
    
    pub unsafe fn glUniform4f(&self, location: i32, v0: f32, v1: f32, v2: f32, v3: f32) {
        self.fns.Uniform4f(location, v0, v1, v2, v3)
    }
    
    pub unsafe fn glUniform4fv(&self, location: i32, count: i32, value: *const f32) {
        self.fns.Uniform4fv(location, count, value)
    }
    
    pub unsafe fn glUniform4i(&self, location: i32, v0: i32, v1: i32, v2: i32, v3: i32) {
        self.fns.Uniform4i(location, v0, v1, v2, v3)
    }
    
    pub unsafe fn glUniform4iv(&self, location: i32, count: i32, value: *const i32) {
        self.fns.Uniform4iv(location, count, value)
    }
    
    pub unsafe fn glUniform4ui(&self, location: i32, v0: u32, v1: u32, v2: u32, v3: u32) {
        self.fns.Uniform4ui(location, v0, v1, v2, v3)
    }
    
    pub unsafe fn glUniform4uiv(&self, location: i32, count: i32, value: *const u32) {
        self.fns.Uniform4uiv(location, count, value)
    }
    
    pub unsafe fn glUniformBlockBinding(&self, program: u32, uniformBlockIndex: u32, uniformBlockBinding: u32) {
        self.fns.UniformBlockBinding(program, uniformBlockIndex, uniformBlockBinding)
    }
    
    pub unsafe fn glUniformMatrix2fv(&self, location: i32, count: i32, transpose: u8, value: *const f32) {
        self.fns.UniformMatrix2fv(location, count, transpose, value)
    }
    
    pub unsafe fn glUniformMatrix2x3fv(&self, location: i32, count: i32, transpose: u8, value: *const f32) {
        self.fns.UniformMatrix2x3fv(location, count, transpose, value)
    }
    
    pub unsafe fn glUniformMatrix2x4fv(&self, location: i32, count: i32, transpose: u8, value: *const f32) {
        self.fns.UniformMatrix2x4fv(location, count, transpose, value)
    }
    
    pub unsafe fn glUniformMatrix3fv(&self, location: i32, count: i32, transpose: u8, value: *const f32) {
        self.fns.UniformMatrix3fv(location, count, transpose, value)
    }
    
    pub unsafe fn glUniformMatrix3x2fv(&self, location: i32, count: i32, transpose: u8, value: *const f32) {
        self.fns.UniformMatrix3x2fv(location, count, transpose, value)
    }
    
    pub unsafe fn glUniformMatrix3x4fv(&self, location: i32, count: i32, transpose: u8, value: *const f32) {
        self.fns.UniformMatrix3x4fv(location, count, transpose, value)
    }
    
    pub unsafe fn glUniformMatrix4fv(&self, location: i32, count: i32, transpose: u8, value: *const f32) {
        self.fns.UniformMatrix4fv(location, count, transpose, value)
    }
    
    pub unsafe fn glUniformMatrix4x2fv(&self, location: i32, count: i32, transpose: u8, value: *const f32) {
        self.fns.UniformMatrix4x2fv(location, count, transpose, value)
    }
    
    pub unsafe fn glUniformMatrix4x3fv(&self, location: i32, count: i32, transpose: u8, value: *const f32) {
        self.fns.UniformMatrix4x3fv(location, count, transpose, value)
    }
    
    pub unsafe fn glUnmapBuffer(&self, target: BufferTargetARB) -> u8 {
        self.fns.UnmapBuffer(target)
    }
    
    pub fn glUseProgram(&self, program: u32) {
        self.fns.UseProgram(program)
    }
    
    pub unsafe fn glValidateProgram(&self, program: u32) {
        self.fns.ValidateProgram(program)
    }
    
    pub unsafe fn glVertexAttrib1d(&self, index: u32, x: f64) {
        self.fns.VertexAttrib1d(index, x)
    }
    
    pub unsafe fn glVertexAttrib1dv(&self, index: u32, v: *const f64) {
        self.fns.VertexAttrib1dv(index, v)
    }
    
    pub unsafe fn glVertexAttrib1f(&self, index: u32, x: f32) {
        self.fns.VertexAttrib1f(index, x)
    }
    
    pub unsafe fn glVertexAttrib1fv(&self, index: u32, v: *const f32) {
        self.fns.VertexAttrib1fv(index, v)
    }
    
    pub unsafe fn glVertexAttrib1s(&self, index: u32, x: i16) {
        self.fns.VertexAttrib1s(index, x)
    }
    
    pub unsafe fn glVertexAttrib1sv(&self, index: u32, v: *const i16) {
        self.fns.VertexAttrib1sv(index, v)
    }
    
    pub unsafe fn glVertexAttrib2d(&self, index: u32, x: f64, y: f64) {
        self.fns.VertexAttrib2d(index, x, y)
    }
    
    pub unsafe fn glVertexAttrib2dv(&self, index: u32, v: *const[f64; 2]) {
        self.fns.VertexAttrib2dv(index, v)
    }
    
    pub unsafe fn glVertexAttrib2f(&self, index: u32, x: f32, y: f32) {
        self.fns.VertexAttrib2f(index, x, y)
    }
    
    pub unsafe fn glVertexAttrib2fv(&self, index: u32, v: *const[f32; 2]) {
        self.fns.VertexAttrib2fv(index, v)
    }
    
    pub unsafe fn glVertexAttrib2s(&self, index: u32, x: i16, y: i16) {
        self.fns.VertexAttrib2s(index, x, y)
    }
    
    pub unsafe fn glVertexAttrib2sv(&self, index: u32, v: *const[i16; 2]) {
        self.fns.VertexAttrib2sv(index, v)
    }
    
    pub unsafe fn glVertexAttrib3d(&self, index: u32, x: f64, y: f64, z: f64) {
        self.fns.VertexAttrib3d(index, x, y, z)
    }
    
    pub unsafe fn glVertexAttrib3dv(&self, index: u32, v: *const[f64; 3]) {
        self.fns.VertexAttrib3dv(index, v)
    }
    
    pub unsafe fn glVertexAttrib3f(&self, index: u32, x: f32, y: f32, z: f32) {
        self.fns.VertexAttrib3f(index, x, y, z)
    }
    
    pub unsafe fn glVertexAttrib3fv(&self, index: u32, v: *const[f32; 3]) {
        self.fns.VertexAttrib3fv(index, v)
    }
    
    pub unsafe fn glVertexAttrib3s(&self, index: u32, x: i16, y: i16, z: i16) {
        self.fns.VertexAttrib3s(index, x, y, z)
    }
    
    pub unsafe fn glVertexAttrib3sv(&self, index: u32, v: *const[i16; 3]) {
        self.fns.VertexAttrib3sv(index, v)
    }
    
    pub unsafe fn glVertexAttrib4Nbv(&self, index: u32, v: *const[i8; 4]) {
        self.fns.VertexAttrib4Nbv(index, v)
    }
    
    pub unsafe fn glVertexAttrib4Niv(&self, index: u32, v: *const[i32; 4]) {
        self.fns.VertexAttrib4Niv(index, v)
    }
    
    pub unsafe fn glVertexAttrib4Nsv(&self, index: u32, v: *const[i16; 4]) {
        self.fns.VertexAttrib4Nsv(index, v)
    }
    
    pub unsafe fn glVertexAttrib4Nub(&self, index: u32, x: u8, y: u8, z: u8, w: u8) {
        self.fns.VertexAttrib4Nub(index, x, y, z, w)
    }
    
    pub unsafe fn glVertexAttrib4Nubv(&self, index: u32, v: *const[u8; 4]) {
        self.fns.VertexAttrib4Nubv(index, v)
    }
    
    pub unsafe fn glVertexAttrib4Nuiv(&self, index: u32, v: *const[u32; 4]) {
        self.fns.VertexAttrib4Nuiv(index, v)
    }
    
    pub unsafe fn glVertexAttrib4Nusv(&self, index: u32, v: *const[u16; 4]) {
        self.fns.VertexAttrib4Nusv(index, v)
    }
    
    pub unsafe fn glVertexAttrib4bv(&self, index: u32, v: *const[i8; 4]) {
        self.fns.VertexAttrib4bv(index, v)
    }
    
    pub unsafe fn glVertexAttrib4d(&self, index: u32, x: f64, y: f64, z: f64, w: f64) {
        self.fns.VertexAttrib4d(index, x, y, z, w)
    }
    
    pub unsafe fn glVertexAttrib4dv(&self, index: u32, v: *const[f64; 4]) {
        self.fns.VertexAttrib4dv(index, v)
    }
    
    pub unsafe fn glVertexAttrib4f(&self, index: u32, x: f32, y: f32, z: f32, w: f32) {
        self.fns.VertexAttrib4f(index, x, y, z, w)
    }
    
    pub unsafe fn glVertexAttrib4fv(&self, index: u32, v: *const[f32; 4]) {
        self.fns.VertexAttrib4fv(index, v)
    }
    
    pub unsafe fn glVertexAttrib4iv(&self, index: u32, v: *const[i32; 4]) {
        self.fns.VertexAttrib4iv(index, v)
    }
    
    pub unsafe fn glVertexAttrib4s(&self, index: u32, x: i16, y: i16, z: i16, w: i16) {
        self.fns.VertexAttrib4s(index, x, y, z, w)
    }
    
    pub unsafe fn glVertexAttrib4sv(&self, index: u32, v: *const[i16; 4]) {
        self.fns.VertexAttrib4sv(index, v)
    }
    
    pub unsafe fn glVertexAttrib4ubv(&self, index: u32, v: *const[u8; 4]) {
        self.fns.VertexAttrib4ubv(index, v)
    }
    
    pub unsafe fn glVertexAttrib4uiv(&self, index: u32, v: *const[u32; 4]) {
        self.fns.VertexAttrib4uiv(index, v)
    }
    
    pub unsafe fn glVertexAttrib4usv(&self, index: u32, v: *const[u16; 4]) {
        self.fns.VertexAttrib4usv(index, v)
    }
    
    pub unsafe fn glVertexAttribDivisor(&self, index: u32, divisor: u32) {
        self.fns.VertexAttribDivisor(index, divisor)
    }
    
    pub unsafe fn glVertexAttribI1i(&self, index: u32, x: i32) {
        self.fns.VertexAttribI1i(index, x)
    }
    
    pub unsafe fn glVertexAttribI1iv(&self, index: u32, v: *const i32) {
        self.fns.VertexAttribI1iv(index, v)
    }
    
    pub unsafe fn glVertexAttribI1ui(&self, index: u32, x: u32) {
        self.fns.VertexAttribI1ui(index, x)
    }
    
    pub unsafe fn glVertexAttribI1uiv(&self, index: u32, v: *const u32) {
        self.fns.VertexAttribI1uiv(index, v)
    }
    
    pub unsafe fn glVertexAttribI2i(&self, index: u32, x: i32, y: i32) {
        self.fns.VertexAttribI2i(index, x, y)
    }
    
    pub unsafe fn glVertexAttribI2iv(&self, index: u32, v: *const[i32; 2]) {
        self.fns.VertexAttribI2iv(index, v)
    }
    
    pub unsafe fn glVertexAttribI2ui(&self, index: u32, x: u32, y: u32) {
        self.fns.VertexAttribI2ui(index, x, y)
    }
    
    pub unsafe fn glVertexAttribI2uiv(&self, index: u32, v: *const[u32; 2]) {
        self.fns.VertexAttribI2uiv(index, v)
    }
    
    pub unsafe fn glVertexAttribI3i(&self, index: u32, x: i32, y: i32, z: i32) {
        self.fns.VertexAttribI3i(index, x, y, z)
    }
    
    pub unsafe fn glVertexAttribI3iv(&self, index: u32, v: *const[i32; 3]) {
        self.fns.VertexAttribI3iv(index, v)
    }
    
    pub unsafe fn glVertexAttribI3ui(&self, index: u32, x: u32, y: u32, z: u32) {
        self.fns.VertexAttribI3ui(index, x, y, z)
    }
    
    pub unsafe fn glVertexAttribI3uiv(&self, index: u32, v: *const[u32; 3]) {
        self.fns.VertexAttribI3uiv(index, v)
    }
    
    pub unsafe fn glVertexAttribI4bv(&self, index: u32, v: *const[i8; 4]) {
        self.fns.VertexAttribI4bv(index, v)
    }
    
    pub unsafe fn glVertexAttribI4i(&self, index: u32, x: i32, y: i32, z: i32, w: i32) {
        self.fns.VertexAttribI4i(index, x, y, z, w)
    }
    
    pub unsafe fn glVertexAttribI4iv(&self, index: u32, v: *const[i32; 4]) {
        self.fns.VertexAttribI4iv(index, v)
    }
    
    pub unsafe fn glVertexAttribI4sv(&self, index: u32, v: *const[i16; 4]) {
        self.fns.VertexAttribI4sv(index, v)
    }
    
    pub unsafe fn glVertexAttribI4ubv(&self, index: u32, v: *const[u8; 4]) {
        self.fns.VertexAttribI4ubv(index, v)
    }
    
    pub unsafe fn glVertexAttribI4ui(&self, index: u32, x: u32, y: u32, z: u32, w: u32) {
        self.fns.VertexAttribI4ui(index, x, y, z, w)
    }
    
    pub unsafe fn glVertexAttribI4uiv(&self, index: u32, v: *const[u32; 4]) {
        self.fns.VertexAttribI4uiv(index, v)
    }
    
    pub unsafe fn glVertexAttribI4usv(&self, index: u32, v: *const[u16; 4]) {
        self.fns.VertexAttribI4usv(index, v)
    }
    
    pub unsafe fn glVertexAttribIPointer(&self, index: u32, size: i32, type_: VertexAttribIType, stride: i32, pointer: *const c_void) {
        self.fns.VertexAttribIPointer(index, size, type_, stride, pointer)
    }
    
    pub unsafe fn glVertexAttribP1ui(&self, index: u32, type_: VertexAttribPointerType, normalized: u8, value: u32) {
        self.fns.VertexAttribP1ui(index, type_, normalized, value)
    }
    
    pub unsafe fn glVertexAttribP1uiv(&self, index: u32, type_: VertexAttribPointerType, normalized: u8, value: *const u32) {
        self.fns.VertexAttribP1uiv(index, type_, normalized, value)
    }
    
    pub unsafe fn glVertexAttribP2ui(&self, index: u32, type_: VertexAttribPointerType, normalized: u8, value: u32) {
        self.fns.VertexAttribP2ui(index, type_, normalized, value)
    }
    
    pub unsafe fn glVertexAttribP2uiv(&self, index: u32, type_: VertexAttribPointerType, normalized: u8, value: *const u32) {
        self.fns.VertexAttribP2uiv(index, type_, normalized, value)
    }
    
    pub unsafe fn glVertexAttribP3ui(&self, index: u32, type_: VertexAttribPointerType, normalized: u8, value: u32) {
        self.fns.VertexAttribP3ui(index, type_, normalized, value)
    }
    
    pub unsafe fn glVertexAttribP3uiv(&self, index: u32, type_: VertexAttribPointerType, normalized: u8, value: *const u32) {
        self.fns.VertexAttribP3uiv(index, type_, normalized, value)
    }
    
    pub unsafe fn glVertexAttribP4ui(&self, index: u32, type_: VertexAttribPointerType, normalized: u8, value: u32) {
        self.fns.VertexAttribP4ui(index, type_, normalized, value)
    }
    
    pub unsafe fn glVertexAttribP4uiv(&self, index: u32, type_: VertexAttribPointerType, normalized: u8, value: *const u32) {
        self.fns.VertexAttribP4uiv(index, type_, normalized, value)
    }
    
    pub fn glVertexAttribPointer(&self, index: u32, size: i32, type_: VertexAttribPointerType, normalized: bool, stride: i32, offset: u32) {
        unsafe { self.fns.VertexAttribPointer(index, size, type_, normalized as _, stride, offset as _) }
    }
    
    pub unsafe fn glViewport(&self, x: i32, y: i32, width: u32, height: u32) {
        self.fns.Viewport(x, y, width as _, height as _)
    }
    
    pub unsafe fn glWaitSync(&self, sync: GLsync, flags: GLbitfield, timeout: u64) {
        self.fns.WaitSync(sync, flags, timeout)
    }
    
    pub unsafe fn glDebugMessageCallback(&self, callback: GLDEBUGPROC, userParam: *const c_void) {
        self.fns.DebugMessageCallback(callback, userParam)
    }
    
    pub unsafe fn glDebugMessageControl(&self, source: DebugSource, type_: DebugType, severity: DebugSeverity, count: i32, ids: *const u32, enabled: u8) {
        self.fns.DebugMessageControl(source, type_, severity, count, ids, enabled)
    }
    
    pub fn glDebugMessageInsert(&self, source: DebugSource, type_: DebugType, id: u32, severity: DebugSeverity, buf: &str) {
        unsafe { self.fns.DebugMessageInsert(source, type_, id, severity, buf.len() as _, buf.as_ptr()) }
    }
    
    pub unsafe fn glGetDebugMessageLog(&self, count: u32, bufSize: i32, sources: *mut DebugSource, types: *mut DebugType, ids: *mut u32, severities: *mut DebugSeverity, lengths: *mut i32, messageLog: *mut u8) -> u32 {
        self.fns.GetDebugMessageLog(count, bufSize, sources, types, ids, severities, lengths, messageLog)
    }
    
    pub unsafe fn glGetObjectLabel(&self, identifier: ObjectIdentifier, name: u32, bufSize: i32, length: *mut i32, label: *mut u8) {
        self.fns.GetObjectLabel(identifier, name, bufSize, length, label)
    }
    
    pub unsafe fn glGetObjectPtrLabel(&self, ptr: *const c_void, bufSize: i32, length: *mut i32, label: *mut u8) {
        self.fns.GetObjectPtrLabel(ptr, bufSize, length, label)
    }
    
    pub unsafe fn glGetPointerv(&self, pname: GetPointervPName, params: *mut *mut c_void) {
        self.fns.GetPointerv(pname, params)
    }
    
    pub fn glObjectLabel(&self, identifier: ObjectIdentifier, name: u32, label: &str) {
        unsafe { self.fns.ObjectLabel(identifier, name, label.len() as _, label.as_ptr()) }
    }
    
    pub fn glObjectPtrLabel(&self, ptr: *const c_void, label: &str) {
        unsafe { self.fns.ObjectPtrLabel(ptr, label.len() as _, label.as_ptr()) }
    }
    
    pub unsafe fn glPopDebugGroup(&self) {
        self.fns.PopDebugGroup()
    }
    
    pub fn glPushDebugGroup(&self, source: DebugSource, id: u32, message: &str) {
        unsafe { self.fns.PushDebugGroup(source, id, message.len() as _, message.as_ptr()) }
    }
    
    // Functions new to gl46
    pub fn glActiveShaderProgram(&self, pipeline: u32, program: u32) {
        unsafe { self.fns.ActiveShaderProgram(pipeline, program) }
    }
    
    pub fn glBeginQueryIndexed(&self, target: QueryTarget, index: u32, id: u32) {
        unsafe { self.fns.BeginQueryIndexed(target, index, id) }
    }
    
    pub fn glBindBuffersBase(&self, target: BufferTargetARB, index: u32, buffer: u32) {
        unsafe { self.fns.BindBufferBase(target, index, buffer) }
    }
    
    pub fn glBindBuffersRange(&self, target: BufferTargetARB, index: u32, buffer: u32, offset: isize, size: isize) {
        unsafe { self.fns.BindBufferRange(target, index, buffer, offset, size) }
    }
    
    pub fn glBindImageTexture(&self, unit: u32, texture: u32, level: i32, layered: u8, layer: i32, access: BufferAccessARB, format: InternalFormat) {
        unsafe { self.fns.BindImageTexture(unit, texture, level, layered, layer, access, format) }
    }
    
    pub fn glBindImageTextures(&self, first: u32, textures: &[u32]) {
        unsafe { self.fns.BindImageTextures(first, textures.len() as i32, textures.as_ptr()) }
    }
    
    pub fn glBindProgramPipeline(&self, pipeline: u32) {
        unsafe { self.fns.BindProgramPipeline(pipeline) }
    }
    
    pub fn glBindSamplers(&self, first: u32, samplers: &[u32]) {
        unsafe { self.fns.BindSamplers(first, samplers.len() as _, samplers.as_ptr()) }
    }
    
    pub fn glBindTextureUnit(&self, unit: u32, texture: u32) {
        unsafe { self.fns.BindTextureUnit(unit, texture) }
    }
    
    pub fn glBindTextures(&self, first: u32, textures: &[u32]) {
        unsafe { self.fns.BindTextures(first, textures.len() as _, textures.as_ptr()) }
    }
    
    pub fn glBindTransformFeedback(&self, target: BindTransformFeedbackTarget, id: u32) {
        unsafe { self.fns.BindTransformFeedback(target, id) }
    }
    
    pub fn glBindVertexBuffer(&self, bindingindex: u32, buffer: u32, offset: isize, stride: i32) {
        unsafe { self.fns.BindVertexBuffer(bindingindex, buffer, offset, stride) }
    }
    
    pub fn glBindVertexBuffers(&self, first: u32, buffers: &[u32], offsets: &[isize], strides: &[i32]) {
        if buffers.len() != offsets.len() || offsets.len() != strides.len() {
            panic!("buffers, offsets, and strides must all be the same length!");
        }

        unsafe { self.fns.BindVertexBuffers(first, buffers.len() as _, buffers.as_ptr(), offsets.as_ptr(), strides.as_ptr()) }
    }
    
    pub fn glBlendEquationSeparatei(&self, buf: u32, modeRGB: BlendEquationModeEXT, modeAlpha: BlendEquationModeEXT) {
        unsafe { self.fns.BlendEquationSeparatei(buf, modeRGB, modeAlpha) }
    }
    
    pub fn glBlendEquationi(&self, buf: u32, mode: BlendEquationModeEXT) {
        unsafe { self.fns.BlendEquationi(buf, mode) }
    }
    
    pub fn glBlendFuncSeparatei(&self, buf: u32, srcRGB: BlendingFactor, dstRGB: BlendingFactor, srcAlpha: BlendingFactor, dstAlpha: BlendingFactor) {
        unsafe { self.fns.BlendFuncSeparatei(buf, srcRGB, dstRGB, srcAlpha, dstAlpha) }
    }
    
    pub fn glBlendFunci(&self, buf: u32, src: BlendingFactor, dst: BlendingFactor) {
        unsafe { self.fns.BlendFunci(buf, src, dst) }
    }
    
    pub fn glBlitNamedFramebuffer(&self, readFramebuffer: u32, drawFramebuffer: u32, srcX0: i32, srcY0: i32, srcX1: i32, srcY1: i32, dstX0: i32, dstY0: i32, dstX1: i32, dstY1: i32, mask: GLbitfield, filter: BlitFramebufferFilter) {
        unsafe { self.fns.BlitNamedFramebuffer(readFramebuffer, drawFramebuffer, srcX0, srcY0, srcX1, srcY1, dstX0, dstY0, dstX1, dstY1, mask, filter) }
    }
    
    pub fn glBufferStorage<T>(&self, target: BufferStorageTarget, data: &[T], flags: GLbitfield) {
        unsafe { self.fns.BufferStorage(target, (std::mem::size_of::<T>() * data.len()) as _, data.as_ptr() as _, flags) }
    }
    
    pub fn glCheckNamedFramebufferStatus(&self, framebuffer: u32, target: FramebufferTarget) -> FramebufferStatus {
        unsafe { self.fns.CheckNamedFramebufferStatus(framebuffer, target) }
    }
    
    pub fn glClearBufferData<T: GLType>(&self, target: BufferStorageTarget, internalformat: InternalFormat, format: PixelFormat, data: &[T]) {
        let expected_size = match internalformat {
            GL_R8 | GL_R16 | GL_R16F | GL_R32F | GL_R8I | GL_R16I | GL_R32I | GL_R8UI | GL_R16UI | GL_R32UI => 1,
            GL_RG8 | GL_RG16 | GL_RG16F | GL_RG32F | GL_RG8I | GL_RG16I | GL_RG32I | GL_RG8UI | GL_RG16UI | GL_RG32UI => 2,
            GL_RGB32F | GL_RGB32I | GL_RGB32UI => 3,
            GL_RGBA8 | GL_RGBA16 | GL_RGBA16F | GL_RGBA32F | GL_RGBA8I | GL_RGBA16I | GL_RGBA32I | GL_RGBA8UI | GL_RGBA16UI | GL_RGBA32UI => 4,
            _ => -1 
        };

        if data.len() as i32 != expected_size {
            panic!("Incorrect value size!")
        }

        unsafe { self.fns.ClearBufferData(target, internalformat, format, T::gl_type(), data.as_ptr() as _) }
    }
    
    pub fn glClearBufferSubData<T: GLType>(&self, target: BufferStorageTarget, internalformat: InternalFormat, offset: isize, size: isize, format: PixelFormat, data: &[T]) {
        let expected_size = match internalformat {
            GL_R8 | GL_R16 | GL_R16F | GL_R32F | GL_R8I | GL_R16I | GL_R32I | GL_R8UI | GL_R16UI | GL_R32UI => 1,
            GL_RG8 | GL_RG16 | GL_RG16F | GL_RG32F | GL_RG8I | GL_RG16I | GL_RG32I | GL_RG8UI | GL_RG16UI | GL_RG32UI => 2,
            GL_RGB32F | GL_RGB32I | GL_RGB32UI => 3,
            GL_RGBA8 | GL_RGBA16 | GL_RGBA16F | GL_RGBA32F | GL_RGBA8I | GL_RGBA16I | GL_RGBA32I | GL_RGBA8UI | GL_RGBA16UI | GL_RGBA32UI => 4,
            _ => -1 
        };

        if data.len() as i32 != expected_size {
            panic!("Incorrect value size!")
        }

        unsafe { self.fns.ClearBufferSubData(target, internalformat, offset, size, format, T::gl_type(), data.as_ptr() as _) }
    }
    
    pub fn glClearDepthf(&self, d: f32) {
        unsafe { self.fns.ClearDepthf(d) }
    }
    
    pub fn glClearNamedBufferData<T: GLType>(&self, buffer: u32, internalformat: InternalFormat, format: PixelFormat, data: &[T]) {
        let expected_size = match internalformat {
            GL_R8 | GL_R16 | GL_R16F | GL_R32F | GL_R8I | GL_R16I | GL_R32I | GL_R8UI | GL_R16UI | GL_R32UI => 1,
            GL_RG8 | GL_RG16 | GL_RG16F | GL_RG32F | GL_RG8I | GL_RG16I | GL_RG32I | GL_RG8UI | GL_RG16UI | GL_RG32UI => 2,
            GL_RGB32F | GL_RGB32I | GL_RGB32UI => 3,
            GL_RGBA8 | GL_RGBA16 | GL_RGBA16F | GL_RGBA32F | GL_RGBA8I | GL_RGBA16I | GL_RGBA32I | GL_RGBA8UI | GL_RGBA16UI | GL_RGBA32UI => 4,
            _ => -1 
        };

        if data.len() as i32 != expected_size {
            panic!("Incorrect value size!")
        }

        unsafe { self.fns.ClearNamedBufferData(buffer, internalformat, format, T::gl_type(), data.as_ptr() as _) }
    }
    
    pub fn glClearNamedBufferSubData<T: GLType>(&self, buffer: u32, internalformat: InternalFormat, offset: isize, size: isize, format: PixelFormat, data: &[T]) {
        let expected_size = match internalformat {
            GL_R8 | GL_R16 | GL_R16F | GL_R32F | GL_R8I | GL_R16I | GL_R32I | GL_R8UI | GL_R16UI | GL_R32UI => 1,
            GL_RG8 | GL_RG16 | GL_RG16F | GL_RG32F | GL_RG8I | GL_RG16I | GL_RG32I | GL_RG8UI | GL_RG16UI | GL_RG32UI => 2,
            GL_RGB32F | GL_RGB32I | GL_RGB32UI => 3,
            GL_RGBA8 | GL_RGBA16 | GL_RGBA16F | GL_RGBA32F | GL_RGBA8I | GL_RGBA16I | GL_RGBA32I | GL_RGBA8UI | GL_RGBA16UI | GL_RGBA32UI => 4,
            _ => -1 
        };

        if data.len() as i32 != expected_size {
            panic!("Incorrect value size!")
        }

        unsafe { self.fns.ClearNamedBufferSubData(buffer, internalformat, offset, size, format, T::gl_type(), data.as_ptr() as _) }
    }
    
    pub fn glClearNamedFramebufferfi(&self, framebuffer: u32, buffer: Buffer, drawbuffer: i32, depth: f32, stencil: i32) {
        unsafe { self.fns.ClearNamedFramebufferfi(framebuffer, buffer, drawbuffer, depth as _, stencil) }
    }
    
    pub fn glClearNamedFramebufferfv(&self, framebuffer: u32, buffer: Buffer, drawbuffer: i32, value: &[f32]) {
        let expected_size = match buffer {
            GL_COLOR => 4,
            GL_DEPTH => 1,
            GL_STENCIL => 1,
            _ => -1
        };

        if value.len() as i32 != expected_size {
            panic!("Incorrect value size!")
        }

        unsafe { self.fns.ClearNamedFramebufferfv(framebuffer, buffer, drawbuffer, value.as_ptr()) }
    }
    
    pub fn glClearNamedFramebufferiv(&self, framebuffer: u32, buffer: Buffer, drawbuffer: i32, value: &[i32]) {
        let expected_size = match buffer {
            GL_COLOR => 4,
            GL_DEPTH => 1,
            GL_STENCIL => 1,
            _ => -1
        };

        if value.len() as i32 != expected_size {
            panic!("Incorrect value size!")
        }

        unsafe { self.fns.ClearNamedFramebufferiv(framebuffer, buffer, drawbuffer, value.as_ptr() as _) }
    }
    
    pub fn glClearNamedFramebufferuiv(&self, framebuffer: u32, buffer: Buffer, drawbuffer: i32, value: &[u32]) {
        let expected_size = match buffer {
            GL_COLOR => 4,
            GL_DEPTH => 1,
            GL_STENCIL => 1,
            _ => -1
        };

        if value.len() as i32 != expected_size {
            panic!("Incorrect value size!")
        }

        unsafe { self.fns.ClearNamedFramebufferuiv(framebuffer, buffer, drawbuffer, value.as_ptr() as _) }
    }
    
    pub unsafe fn glClearTexImage<T: GLType>(&self, texture: u32, level: i32, format: PixelFormat, type_: PixelType, data: *const c_void ) {
        self.fns.ClearTexImage(texture, level, format, type_, data)
    }
    
    pub unsafe fn glClearTexSubImage(&self, texture: u32, level: i32, xoffset: i32, yoffset: i32, zoffset: i32, width: u32, height: u32, depth: u32, format: PixelFormat, type_: PixelType, data: *const c_void) {
        self.fns.ClearTexSubImage(texture, level, xoffset, yoffset, zoffset, width as _, height as _, depth as _, format, type_, data)
    }
    
    pub fn glClipControl(&self, origin: ClipControlOrigin, depth: ClipControlDepth) {
        unsafe { self.fns.ClipControl(origin, depth) }
    }
    
    pub fn glCompressedTextureSubImage1D<T>(&self, target: TextureTarget, level: i32, internalformat: InternalFormat, width: u32, border: i32, data: &[T]) {
        unsafe { self.fns.CompressedTexImage1D(target, level, internalformat, width as _, border, (std::mem::size_of::<T>() * data.len()) as _, data.as_ptr() as _) }
    }
    
    pub fn glCompressedTextureSubImage2D<T>(&self, target: TextureTarget, level: i32, internalformat: InternalFormat, width: u32, height: u32, border: i32, data: &[T]) {
        unsafe { self.fns.CompressedTexImage2D(target, level, internalformat, width as _, height as _, border, (std::mem::size_of::<T>() * data.len()) as _, data.as_ptr() as _) }
    }
    
    pub fn glCompressedTextureSubImage3D<T>(&self, target: TextureTarget, level: i32, internalformat: InternalFormat, width: u32, height: u32, depth: u32, border: i32, data: &[T]) {
        unsafe { self.fns.CompressedTexImage3D(target, level, internalformat, width as _, height as _, depth as _, border, (std::mem::size_of::<T>() * data.len()) as _, data.as_ptr() as _) }
    }
    
    pub fn glCopyImageSubData(&self, srcName: u32, srcTarget: CopyImageSubDataTarget, srcLevel: i32, srcX: i32, srcY: i32, srcZ: i32, dstName: u32, dstTarget: CopyImageSubDataTarget, dstLevel: i32, dstX: i32, dstY: i32, dstZ: i32, srcWidth: u32, srcHeight: u32, srcDepth: u32) {
        unsafe { self.fns.CopyImageSubData(srcName, srcTarget, srcLevel, srcX, srcY, srcZ, dstName, dstTarget, dstLevel, dstX, dstY, dstZ, srcWidth as _, srcHeight as _, srcDepth as _) }
    }
    
    pub fn glCopyNamedBufferSubData(&self, readBuffer: u32, writeBuffer: u32, readOffset: isize, writeOffset: isize, size: usize) {
        unsafe { self.fns.CopyNamedBufferSubData(readBuffer, writeBuffer, readOffset, writeOffset, size as _) }
    }

    pub fn glCopyTextureSubImage1D(&self, texture: u32, level: i32, xoffset: i32, x: i32, y: i32, width: u32) {
        unsafe { self.fns.CopyTextureSubImage1D(texture, level, xoffset, x, y, width as _) }
    }

    pub fn glCopyTextureSubImage2D(&self, texture: u32, level: i32, xoffset: i32, yoffset: i32, x: i32, y: i32, width: u32, height: u32) {
        unsafe { self.fns.CopyTextureSubImage2D(texture, level, xoffset, yoffset, x, y, width as _, height as _) }
    }

    pub fn glCopyTextureSubImage3D(&self, texture: u32, level: i32, xoffset: i32, yoffset: i32, zoffset: i32, x: i32, y: i32, width: u32, height: u32) {
        unsafe { self.fns.CopyTextureSubImage3D(texture, level, xoffset, yoffset, zoffset, x, y, width as _, height as _) }
    }

    pub fn glCreateBuffers(&self, buffers: &mut [u32]) {
        unsafe { self.fns.CreateBuffers(buffers.len() as _, buffers.as_mut_ptr()) }
    }

    pub fn glCreateFramebuffers(&self, framebuffers: &mut [u32]) {
        unsafe { self.fns.CreateFramebuffers(framebuffers.len() as _, framebuffers.as_mut_ptr()) }
    }

    pub fn glCreateProgramPipelines(&self, pipelines: &mut [u32]) {
        unsafe { self.fns.CreateProgramPipelines(pipelines.len() as _, pipelines.as_mut_ptr()) }
    }

    pub fn glCreateQueries(&self, target: QueryTarget, ids: &mut [u32]) {
        unsafe { self.fns.CreateQueries(target, ids.len() as _, ids.as_mut_ptr()) }
    }

    pub fn glCreateRenderbuffers(&self, renderbuffers: &mut [u32]) {
        unsafe { self.fns.CreateRenderbuffers(renderbuffers.len() as _, renderbuffers.as_mut_ptr()) }
    }

    pub fn glCreateSamplers(&self, samplers: &mut [u32]) {
        unsafe { self.fns.CreateSamplers(samplers.len() as _, samplers.as_mut_ptr()) }
    }

    pub fn glCreateShaderProgramv(&self, type_: ShaderType, strings: &[&str]) -> u32 {
        let strings = CStringArray::new(strings);
        unsafe { self.fns.CreateShaderProgramv(type_, strings.len() as _, strings.as_ptr()) }
    }

    pub fn glCreateTextures(&self, target: TextureTarget, textures: &mut [u32]) {
        unsafe { self.fns.CreateTextures(target, textures.len() as _, textures.as_mut_ptr()) }
    }

    pub fn glCreateTransformFeedbacks(&self, ids: &mut [u32]) {
        unsafe { self.fns.CreateTransformFeedbacks(ids.len() as _, ids.as_mut_ptr()) }
    }

    pub fn glCreateVertexArrays(&self, arrays: &mut [u32]) {
        unsafe { self.fns.CreateVertexArrays(arrays.len() as _, arrays.as_mut_ptr()) }
    }

    pub fn glDeleteProgramPipelines(&self, pipelines: &[u32]) {
        unsafe { self.fns.DeleteProgramPipelines(pipelines.len() as _, pipelines.as_ptr()) }
    }

    pub fn glDeleteTransformFeedbacks(&self, ids: &[u32]) {
        unsafe { self.fns.DeleteTransformFeedbacks(ids.len() as _, ids.as_ptr()) }
    }

    pub fn glDepthRangeArrayv(&self, first: u32, v: &[(f64, f64)]) {
        unsafe { self.fns.DepthRangeArrayv(first, v.len() as _, v.as_ptr() as _) }
    }

    pub fn glDepthRangeIndexed(&self, index: u32, n: f64, f: f64) {
        unsafe { self.fns.DepthRangeIndexed(index, n, f) }
    }

    pub fn glDepthRangef(&self, n: f32, f: f32) {
        unsafe { self.fns.DepthRangef(n, f) }
    }

    pub fn glDisableVertexArrayAttrib(&self, vaobj: u32, index: u32) {
        unsafe { self.fns.DisableVertexArrayAttrib(vaobj, index) }
    }

    pub fn glDispatchCompute(&self, num_groups_x: u32, num_groups_y: u32, num_groups_z: u32) {
        unsafe { self.fns.DispatchCompute(num_groups_x, num_groups_y, num_groups_z) }
    }

    pub fn glDispatchComputeIndirect(&self, indirect: isize) {
        unsafe { self.fns.DispatchComputeIndirect(indirect) }
    }

    pub unsafe fn glDrawArraysIndirect(&self, mode: PrimitiveType, indirect: *const c_void) {
        // Needs Special Attention
        unsafe { self.fns.DrawArraysIndirect(mode, indirect) }
    }

    pub fn glDrawArraysInstancedBaseInstance(&self, mode: PrimitiveType, first: i32, count: u32, instancecount: u32, baseinstance: u32) {
        unsafe { self.fns.DrawArraysInstancedBaseInstance(mode, first, count as _, instancecount as _, baseinstance) }
    }

    pub fn glDrawElementsIndirect(&self, mode: PrimitiveType, type_: DrawElementsType, indirect: *const c_void) {
        // Unreviewed
        unsafe { self.fns.DrawElementsIndirect(mode, type_, indirect) }
    }

    pub fn glDrawElementsInstancedBaseInstance(&self, mode: PrimitiveType, count: u32, type_: PrimitiveType, indices: *const c_void, instancecount: u32, baseinstance: u32) {
        // Unreviewed
        unsafe { self.fns.DrawElementsInstancedBaseInstance(mode, count as _, type_, indices, instancecount as _, baseinstance) }
    }

    pub fn glDrawElementsInstancedBaseVertexBaseInstance(&self, mode: PrimitiveType, count: u32, type_: DrawElementsType, indices: *const c_void, instancecount: u32, basevertex: i32, baseinstance: u32) {
        // Unreviewed
        unsafe { self.fns.DrawElementsInstancedBaseVertexBaseInstance(mode, count as _, type_, indices, instancecount as _, basevertex, baseinstance) }
    }

    pub fn glDrawTransformFeedback(&self, mode: PrimitiveType, id: u32) {
        unsafe { self.fns.DrawTransformFeedback(mode, id) }
    }

    pub fn glDrawTransformFeedbackInstanced(&self, mode: PrimitiveType, id: u32, instancecount: u32) {
        unsafe { self.fns.DrawTransformFeedbackInstanced(mode, id, instancecount as _) }
    }

    pub fn glDrawTransformFeedbackStream(&self, mode: PrimitiveType, id: u32, stream: u32) {
        unsafe { self.fns.DrawTransformFeedbackStream(mode, id, stream) }
    }

    pub fn glDrawTransformFeedbackStreamInstanced(&self, mode: PrimitiveType, id: u32, stream: u32, instancecount: u32) {
        unsafe { self.fns.DrawTransformFeedbackStreamInstanced(mode, id, stream, instancecount as _) }
    }

    pub fn glEnableVertexArrayAttrib(&self, vaobj: u32, index: u32) {
        unsafe { self.fns.EnableVertexArrayAttrib(vaobj, index) }
    }

    pub fn glEndQueryIndexed(&self, target: QueryTarget, index: u32) {
        unsafe { self.fns.EndQueryIndexed(target, index) }
    }

    pub fn glFlushMappedNamedBufferRange(&self, buffer: u32, offset: isize, length: usize) {
        unsafe { self.fns.FlushMappedNamedBufferRange(buffer, offset, length as _) }
    }

    pub fn glFramebufferParameteri(&self, target: FramebufferTarget, pname: FramebufferParameterName, param: i32) {
        unsafe { self.fns.FramebufferParameteri(target, pname, param) }
    }

    pub fn glGenProgramPipelines(&self, n: u32, pipelines: *mut u32) {
        // Unreviewed
        unsafe { self.fns.GenProgramPipelines(n as _, pipelines) }
    }

    pub fn glGenTransformFeedbacks(&self, n: u32, ids: *mut u32) {
        // Unreviewed
        unsafe { self.fns.GenTransformFeedbacks(n as _, ids) }
    }

    pub fn glGenerateTextureMipmap(&self, texture: u32) {
        unsafe { self.fns.GenerateTextureMipmap(texture) }
    }

    pub fn glGetActiveAtomicCounterBufferiv(&self, program: u32, bufferIndex: u32, pname: AtomicCounterBufferPName, params: *mut i32) {
        // Unreviewed
        unsafe { self.fns.GetActiveAtomicCounterBufferiv(program, bufferIndex, pname, params) }
    }

    pub fn glGetActiveSubroutineName(&self, program: u32, shadertype: ShaderType, index: u32, bufSize: u32, length: *mut u32, name: *mut u8) {
        // Unreviewed
        unsafe { self.fns.GetActiveSubroutineName(program, shadertype, index, bufSize as _, length as _, name) }
    }

    pub fn glGetActiveSubroutineUniformName(&self, program: u32, shadertype: ShaderType, index: u32, bufSize: u32, length: *mut u32, name: *mut u8) {
        // Unreviewed
        unsafe { self.fns.GetActiveSubroutineUniformName(program, shadertype, index, bufSize as _, length as _, name) }
    }

    pub fn glGetActiveSubroutineUniformiv(&self, program: u32, shadertype: ShaderType, index: u32, pname: SubroutineParameterName, values: *mut i32) {
        // Unreviewed
        unsafe { self.fns.GetActiveSubroutineUniformiv(program, shadertype, index, pname, values) }
    }

    pub fn glGetCompressedTextureImage(&self, texture: u32, level: i32, bufSize: u32, pixels: *mut c_void) {
        // Unreviewed
        unsafe { self.fns.GetCompressedTextureImage(texture, level, bufSize as _, pixels) }
    }

    pub fn glGetCompressedTextureSubImage(&self, texture: u32, level: i32, xoffset: i32, yoffset: i32, zoffset: i32, width: u32, height: u32, depth: u32, bufSize: u32, pixels: *mut c_void) {
        // Unreviewed
        unsafe { self.fns.GetCompressedTextureSubImage(texture, level, xoffset, yoffset, zoffset, width as _, height as _, depth as _, bufSize as _, pixels) }
    }

    pub fn glGetDoublei_v(&self, target: GetPName, index: u32, data: *mut f64) {
        // Unreviewed
        unsafe { self.fns.GetDoublei_v(target, index, data) }
    }

    pub fn glGetFloati_v(&self, target: GetPName, index: u32, data: *mut f32) {
        // Unreviewed
        unsafe { self.fns.GetFloati_v(target, index, data) }
    }

    pub fn glGetFramebufferParameteriv(&self, target: FramebufferTarget, pname: FramebufferAttachmentParameterName, params: *mut i32) {
        // Unreviewed
        unsafe { self.fns.GetFramebufferParameteriv(target, pname, params) }
    }

    pub fn glGetGraphicsResetStatus(&self) -> GraphicsResetStatus {
        unsafe { self.fns.GetGraphicsResetStatus() }
    }

    pub fn glGetInternalformati64v(&self, target: TextureTarget, internalformat: InternalFormat, pname: InternalFormatPName, count: u32, params: *mut i64) {
        // Unreviewed
        unsafe { self.fns.GetInternalformati64v(target, internalformat, pname, count as _, params) }
    }

    pub fn glGetInternalformativ(&self, target: TextureTarget, internalformat: InternalFormat, pname: InternalFormatPName, count: u32, params: *mut i32) {
        // Unreviewed
        unsafe { self.fns.GetInternalformativ(target, internalformat, pname, count as _, params) }
    }

    pub fn glGetNamedBufferParameteri64v(&self, buffer: u32, pname: BufferPNameARB, params: *mut i64) {
        // Unreviewed
        unsafe { self.fns.GetNamedBufferParameteri64v(buffer, pname, params) }
    }

    pub fn glGetNamedBufferParameteriv(&self, buffer: u32, pname: BufferPNameARB, params: *mut i32) {
        // Unreviewed
        unsafe { self.fns.GetNamedBufferParameteriv(buffer, pname, params) }
    }

    pub fn glGetNamedBufferPointerv(&self, buffer: u32, pname: BufferPointerNameARB, params: *mut *mut c_void) {
        // Unreviewed
        unsafe { self.fns.GetNamedBufferPointerv(buffer, pname, params) }
    }

    pub fn glGetNamedBufferSubData(&self, buffer: u32, offset: isize, size: usize, data: *mut c_void) {
        // Unreviewed
        unsafe { self.fns.GetNamedBufferSubData(buffer, offset, size as _, data) }
    }

    pub fn glGetNamedFramebufferAttachmentParameteriv(&self, framebuffer: u32, attachment: FramebufferAttachment, pname: FramebufferAttachmentParameterName, params: *mut i32) {
        // Unreviewed
        unsafe { self.fns.GetNamedFramebufferAttachmentParameteriv(framebuffer, attachment, pname, params) }
    }

    pub fn glGetNamedFramebufferParameteriv(&self, framebuffer: u32, pname: GetFramebufferParameter, param: *mut i32) {
        // Unreviewed
        unsafe { self.fns.GetNamedFramebufferParameteriv(framebuffer, pname, param) }
    }

    pub fn glGetNamedRenderbufferParameteriv(&self, renderbuffer: u32, pname: RenderbufferParameterName, params: *mut i32) {
        // Unreviewed
        unsafe { self.fns.GetNamedRenderbufferParameteriv(renderbuffer, pname, params) }
    }

    pub fn glGetProgramBinary(&self, program: u32, bufSize: u32, length: *mut u32, binaryFormat: *mut GLenum, binary: *mut c_void) {
        // Unreviewed
        unsafe { self.fns.GetProgramBinary(program, bufSize as _, length as _, binaryFormat, binary) }
    }

    pub fn glGetProgramInterfaceiv(&self, program: u32, programInterface: ProgramInterface, pname: ProgramInterfacePName, params: *mut i32) {
        // Unreviewed
        unsafe { self.fns.GetProgramInterfaceiv(program, programInterface, pname, params) }
    }

    pub fn glGetProgramPipelineInfoLog(&self, pipeline: u32, bufSize: u32, length: *mut u32, infoLog: *mut u8) {
        // Unreviewed
        unsafe { self.fns.GetProgramPipelineInfoLog(pipeline, bufSize as _, length as _, infoLog) }
    }

    pub fn glGetProgramPipelineiv(&self, pipeline: u32, pname: PipelineParameterName, params: *mut i32) {
        // Unreviewed
        unsafe { self.fns.GetProgramPipelineiv(pipeline, pname, params) }
    }

    pub fn glGetProgramResourceIndex(&self, program: u32, programInterface: ProgramInterface, name: *const u8) -> u32 {
        // Unreviewed
        unsafe { self.fns.GetProgramResourceIndex(program, programInterface, name) }
    }

    pub fn glGetProgramResourceLocation(&self, program: u32, programInterface: ProgramInterface, name: *const u8) -> i32 {
        // Unreviewed
        unsafe { self.fns.GetProgramResourceLocation(program, programInterface, name) }
    }

    pub fn glGetProgramResourceLocationIndex(&self, program: u32, programInterface: ProgramInterface, name: *const u8) -> i32 {
        // Unreviewed
        unsafe { self.fns.GetProgramResourceLocationIndex(program, programInterface, name) }
    }

    pub fn glGetProgramResourceName(&self, program: u32, programInterface: ProgramInterface, index: u32, bufSize: u32, length: *mut u32, name: *mut u8) {
        // Unreviewed
        unsafe { self.fns.GetProgramResourceName(program, programInterface, index, bufSize as _, length as _, name) }
    }

    pub fn glGetProgramResourceiv(&self, program: u32, programInterface: ProgramInterface, index: u32, propCount: u32, props: *const ProgramResourceProperty, count: u32, length: *mut u32, params: *mut i32) {
        // Unreviewed
        unsafe { self.fns.GetProgramResourceiv(program, programInterface, index, propCount as _, props, count as _, length as _, params) }
    }

    pub fn glGetProgramStageiv(&self, program: u32, shadertype: ShaderType, pname: ProgramStagePName, values: *mut i32) {
        // Unreviewed
        unsafe { self.fns.GetProgramStageiv(program, shadertype, pname, values) }
    }

    pub fn glGetQueryBufferObjecti64v(&self, id: u32, buffer: u32, pname: QueryObjectParameterName, offset: isize) {
        unsafe { self.fns.GetQueryBufferObjecti64v(id, buffer, pname, offset) }
    }

    pub fn glGetQueryBufferObjectiv(&self, id: u32, buffer: u32, pname: QueryObjectParameterName, offset: isize) {
        unsafe { self.fns.GetQueryBufferObjectiv(id, buffer, pname, offset) }
    }

    pub fn glGetQueryBufferObjectui64v(&self, id: u32, buffer: u32, pname: QueryObjectParameterName, offset: isize) {
        unsafe { self.fns.GetQueryBufferObjectui64v(id, buffer, pname, offset) }
    }

    pub fn glGetQueryBufferObjectuiv(&self, id: u32, buffer: u32, pname: QueryObjectParameterName, offset: isize) {
        unsafe { self.fns.GetQueryBufferObjectuiv(id, buffer, pname, offset) }
    }

    pub fn glGetQueryIndexediv(&self, target: QueryTarget, index: u32, pname: QueryParameterName, params: *mut i32) {
        // Unreviewed
        unsafe { self.fns.GetQueryIndexediv(target, index, pname, params) }
    }

    pub fn glGetShaderPrecisionFormat(&self, shadertype: ShaderType, precisiontype: PrecisionType, range: *mut [i32; 2], precision: *mut i32) {
        // Unreviewed
        unsafe { self.fns.GetShaderPrecisionFormat(shadertype, precisiontype, range, precision) }
    }

    pub fn glGetSubroutineIndex(&self, program: u32, shadertype: ShaderType, name: *const u8) -> u32 {
        // Unreviewed
        unsafe { self.fns.GetSubroutineIndex(program, shadertype, name) }
    }

    pub fn glGetSubroutineUniformLocation(&self, program: u32, shadertype: ShaderType, name: *const u8) -> i32 {
        // Unreviewed
        unsafe { self.fns.GetSubroutineUniformLocation(program, shadertype, name) }
    }

    pub fn glGetTextureImage(&self, texture: u32, level: i32, format: PixelFormat, type_: PixelType, bufSize: u32, pixels: *mut c_void) {
        // Unreviewed
        unsafe { self.fns.GetTextureImage(texture, level, format, type_, bufSize as _, pixels) }
    }

    pub fn glGetTextureLevelParameterfv(&self, texture: u32, level: i32, pname: GetTextureParameter, params: *mut f32) {
        // Unreviewed
        unsafe { self.fns.GetTextureLevelParameterfv(texture, level, pname, params) }
    }

    pub fn glGetTextureLevelParameteriv(&self, texture: u32, level: i32, pname: GetTextureParameter, params: *mut i32) {
        // Unreviewed
        unsafe { self.fns.GetTextureLevelParameteriv(texture, level, pname, params) }
    }

    pub fn glGetTextureParameterIiv(&self, texture: u32, pname: GetTextureParameter, params: *mut i32) {
        // Unreviewed
        unsafe { self.fns.GetTextureParameterIiv(texture, pname, params) }
    }

    pub fn glGetTextureParameterIuiv(&self, texture: u32, pname: GetTextureParameter, params: *mut u32) {
        // Unreviewed
        unsafe { self.fns.GetTextureParameterIuiv(texture, pname, params) }
    }

    pub fn glGetTextureParameterfv(&self, texture: u32, pname: GetTextureParameter, params: *mut f32) {
        // Unreviewed
        unsafe { self.fns.GetTextureParameterfv(texture, pname, params) }
    }

    pub fn glGetTextureParameteriv(&self, texture: u32, pname: GetTextureParameter, params: *mut i32) {
        // Unreviewed
        unsafe { self.fns.GetTextureParameteriv(texture, pname, params) }
    }

    pub fn glGetTextureSubImage(&self, texture: u32, level: i32, xoffset: i32, yoffset: i32, zoffset: i32, width: u32, height: u32, depth: u32, format: PixelFormat, type_: PixelType, bufSize: u32, pixels: *mut c_void) {
        // Unreviewed
        unsafe { self.fns.GetTextureSubImage(texture, level, xoffset, yoffset, zoffset, width as _, height as _, depth as _, format, type_, bufSize as _, pixels) }
    }

    pub fn glGetTransformFeedbacki64_v(&self, xfb: u32, pname: TransformFeedbackPName, index: u32, param: *mut i64) {
        // Unreviewed
        unsafe { self.fns.GetTransformFeedbacki64_v(xfb, pname, index, param) }
    }

    pub fn glGetTransformFeedbacki_v(&self, xfb: u32, pname: TransformFeedbackPName, index: u32, param: *mut i32) {
        // Unreviewed
        unsafe { self.fns.GetTransformFeedbacki_v(xfb, pname, index, param) }
    }

    pub fn glGetTransformFeedbackiv(&self, xfb: u32, pname: TransformFeedbackPName, param: *mut i32) {
        // Unreviewed
        unsafe { self.fns.GetTransformFeedbackiv(xfb, pname, param) }
    }

    pub fn glGetUniformSubroutineuiv(&self, shadertype: ShaderType, location: i32, params: *mut u32) {
        // Unreviewed
        unsafe { self.fns.GetUniformSubroutineuiv(shadertype, location, params) }
    }

    pub fn glGetUniformdv(&self, program: u32, location: i32, params: *mut f64) {
        // Unreviewed
        unsafe { self.fns.GetUniformdv(program, location, params) }
    }

    pub fn glGetVertexArrayIndexed64iv(&self, vaobj: u32, index: u32, pname: VertexArrayPName, param: *mut i64) {
        // Unreviewed
        unsafe { self.fns.GetVertexArrayIndexed64iv(vaobj, index, pname, param) }
    }

    pub fn glGetVertexArrayIndexediv(&self, vaobj: u32, index: u32, pname: VertexArrayPName, param: *mut i32) {
        // Unreviewed
        unsafe { self.fns.GetVertexArrayIndexediv(vaobj, index, pname, param) }
    }

    pub fn glGetVertexArrayiv(&self, vaobj: u32, pname: VertexArrayPName, param: *mut i32) {
        // Unreviewed
        unsafe { self.fns.GetVertexArrayiv(vaobj, pname, param) }
    }

    pub fn glGetVertexAttribLdv(&self, index: u32, pname: VertexAttribEnum, params: *mut f64) {
        // Unreviewed
        unsafe { self.fns.GetVertexAttribLdv(index, pname, params) }
    }

    pub fn glGetnCompressedTexImage(&self, target: TextureTarget, lod: i32, bufSize: u32, pixels: *mut c_void) {
        // Unreviewed
        unsafe { self.fns.GetnCompressedTexImage(target, lod, bufSize as _, pixels) }
    }

    pub fn glGetnTexImage(&self, target: TextureTarget, level: i32, format: PixelFormat, type_: PixelType, bufSize: u32, pixels: *mut c_void) {
        // Unreviewed
        unsafe { self.fns.GetnTexImage(target, level, format, type_, bufSize as _, pixels) }
    }

    pub fn glGetnUniformdv(&self, program: u32, location: i32, bufSize: u32, params: *mut f64) {
        // Unreviewed
        unsafe { self.fns.GetnUniformdv(program, location, bufSize as _, params) }
    }

    pub fn glGetnUniformfv(&self, program: u32, location: i32, bufSize: u32, params: *mut f32) {
        // Unreviewed
        unsafe { self.fns.GetnUniformfv(program, location, bufSize as _, params) }
    }

    pub fn glGetnUniformiv(&self, program: u32, location: i32, bufSize: u32, params: *mut i32) {
        // Unreviewed
        unsafe { self.fns.GetnUniformiv(program, location, bufSize as _, params) }
    }

    pub fn glGetnUniformuiv(&self, program: u32, location: i32, bufSize: u32, params: *mut u32) {
        // Unreviewed
        unsafe { self.fns.GetnUniformuiv(program, location, bufSize as _, params) }
    }

    pub fn glInvalidateBufferData(&self, buffer: u32) {
        unsafe { self.fns.InvalidateBufferData(buffer) }
    }

    pub fn glInvalidateBufferSubData(&self, buffer: u32, offset: isize, length: usize) {
        unsafe { self.fns.InvalidateBufferSubData(buffer, offset, length as _) }
    }

    pub fn glInvalidateFramebuffer(&self, target: FramebufferTarget, numAttachments: u32, attachments: *const InvalidateFramebufferAttachment) {
        // Unreviewed
        unsafe { self.fns.InvalidateFramebuffer(target, numAttachments as _, attachments) }
    }

    pub fn glInvalidateNamedFramebufferData(&self, framebuffer: u32, numAttachments: u32, attachments: *const FramebufferAttachment) {
        // Unreviewed
        unsafe { self.fns.InvalidateNamedFramebufferData(framebuffer, numAttachments as _, attachments) }
    }

    pub fn glInvalidateNamedFramebufferSubData(&self, framebuffer: u32, numAttachments: u32, attachments: *const FramebufferAttachment, x: i32, y: i32, width: u32, height: u32) {
        // Unreviewed
        unsafe { self.fns.InvalidateNamedFramebufferSubData(framebuffer, numAttachments as _, attachments, x, y, width as _, height as _) }
    }

    pub fn glInvalidateSubFramebuffer(&self, target: FramebufferTarget, numAttachments: u32, attachments: *const InvalidateFramebufferAttachment, x: i32, y: i32, width: u32, height: u32) {
        // Unreviewed
        unsafe { self.fns.InvalidateSubFramebuffer(target, numAttachments as _, attachments, x, y, width as _, height as _) }
    }

    pub fn glInvalidateTexImage(&self, texture: u32, level: i32) {
        unsafe { self.fns.InvalidateTexImage(texture, level) }
    }

    pub fn glInvalidateTexSubImage(&self, texture: u32, level: i32, xoffset: i32, yoffset: i32, zoffset: i32, width: u32, height: u32, depth: u32) {
        unsafe { self.fns.InvalidateTexSubImage(texture, level, xoffset, yoffset, zoffset, width as _, height as _, depth as _) }
    }

    pub fn glIsProgramPipeline(&self, pipeline: u32) -> bool {
        unsafe { self.fns.IsProgramPipeline(pipeline) > 0 }
    }

    pub fn glIsTransformFeedback(&self, id: u32) -> bool {
        unsafe { self.fns.IsTransformFeedback(id) > 0 }
    }

    pub fn glMapNamedBuffer(&self, buffer: u32, access: BufferAccessARB) -> *mut c_void {
        // Unreviewed
        unsafe { self.fns.MapNamedBuffer(buffer, access) }
    }

    pub fn glMapNamedBufferRange(&self, buffer: u32, offset: isize, length: usize, access: GLbitfield) -> *mut c_void {
        // Unreviewed
        unsafe { self.fns.MapNamedBufferRange(buffer, offset, length as _, access) }
    }

    pub fn glMemoryBarrier(&self, barriers: GLbitfield) {
        unsafe { self.fns.MemoryBarrier(barriers) }
    }

    pub fn glMemoryBarrierByRegion(&self, barriers: GLbitfield) {
        unsafe { self.fns.MemoryBarrierByRegion(barriers) }
    }

    pub fn glMinSampleShading(&self, value: f32) {
        unsafe { self.fns.MinSampleShading(value) }
    }

    pub fn glMultiDrawArraysIndirect(&self, mode: PrimitiveType, indirect: *const c_void, drawcount: u32, stride: u32) {
        // Unreviewed
        unsafe { self.fns.MultiDrawArraysIndirect(mode, indirect, drawcount as _, stride as _) }
    }

    pub fn glMultiDrawArraysIndirectCount(&self, mode: PrimitiveType, indirect: *const c_void, drawcount: isize, maxdrawcount: u32, stride: u32) {
        // Unreviewed
        unsafe { self.fns.MultiDrawArraysIndirectCount(mode, indirect, drawcount, maxdrawcount as _, stride as _) }
    }

    pub fn glMultiDrawElementsIndirect(&self, mode: PrimitiveType, type_: DrawElementsType, indirect: *const c_void, drawcount: u32, stride: u32) {
        // Unreviewed
        unsafe { self.fns.MultiDrawElementsIndirect(mode, type_, indirect, drawcount as _, stride as _) }
    }

    pub fn glMultiDrawElementsIndirectCount(&self, mode: PrimitiveType, type_: DrawElementsType, indirect: *const c_void, drawcount: isize, maxdrawcount: u32, stride: u32) {
        // Unreviewed
        unsafe { self.fns.MultiDrawElementsIndirectCount(mode, type_, indirect, drawcount, maxdrawcount as _, stride as _) }
    }

    pub fn glNamedBufferData(&self, buffer: u32, size: usize, data: *const c_void, usage: VertexBufferObjectUsage) {
        // Unreviewed
        unsafe { self.fns.NamedBufferData(buffer, size as _, data, usage) }
    }

    pub fn glNamedBufferStorage(&self, buffer: u32, size: usize, data: *const c_void, flags: GLbitfield) {
        // Unreviewed
        unsafe { self.fns.NamedBufferStorage(buffer, size as _, data, flags) }
    }

    pub fn glNamedBufferSubData(&self, buffer: u32, offset: isize, size: usize, data: *const c_void) {
        // Unreviewed
        unsafe { self.fns.NamedBufferSubData(buffer, offset, size as _, data) }
    }

    pub fn glNamedFramebufferDrawBuffer(&self, framebuffer: u32, buf: ColorBuffer) {
        unsafe { self.fns.NamedFramebufferDrawBuffer(framebuffer, buf) }
    }

    pub fn glNamedFramebufferDrawBuffers(&self, framebuffer: u32, n: u32, bufs: *const ColorBuffer) {
        // Unreviewed
        unsafe { self.fns.NamedFramebufferDrawBuffers(framebuffer, n as _, bufs) }
    }

    pub fn glNamedFramebufferParameteri(&self, framebuffer: u32, pname: FramebufferParameterName, param: i32) {
        unsafe { self.fns.NamedFramebufferParameteri(framebuffer, pname, param) }
    }

    pub fn glNamedFramebufferReadBuffer(&self, framebuffer: u32, src: ColorBuffer) {
        unsafe { self.fns.NamedFramebufferReadBuffer(framebuffer, src) }
    }

    pub fn glNamedFramebufferRenderbuffer(&self, framebuffer: u32, attachment: FramebufferAttachment, renderbuffertarget: RenderbufferTarget, renderbuffer: u32) {
        unsafe { self.fns.NamedFramebufferRenderbuffer(framebuffer, attachment, renderbuffertarget, renderbuffer) }
    }

    pub fn glNamedFramebufferTexture(&self, framebuffer: u32, attachment: FramebufferAttachment, texture: u32, level: i32) {
        unsafe { self.fns.NamedFramebufferTexture(framebuffer, attachment, texture, level) }
    }

    pub fn glNamedFramebufferTextureLayer(&self, framebuffer: u32, attachment: FramebufferAttachment, texture: u32, level: i32, layer: i32) {
        unsafe { self.fns.NamedFramebufferTextureLayer(framebuffer, attachment, texture, level, layer) }
    }

    pub fn glNamedRenderbufferStorage(&self, renderbuffer: u32, internalformat: InternalFormat, width: u32, height: u32) {
        unsafe { self.fns.NamedRenderbufferStorage(renderbuffer, internalformat, width as _, height as _) }
    }

    pub fn glNamedRenderbufferStorageMultisample(&self, renderbuffer: u32, samples: u32, internalformat: InternalFormat, width: u32, height: u32) {
        unsafe { self.fns.NamedRenderbufferStorageMultisample(renderbuffer, samples as _, internalformat, width as _, height as _) }
    }

    pub fn glPatchParameterfv(&self, pname: PatchParameterName, values: *const f32) {
        // Unreviewed
        unsafe { self.fns.PatchParameterfv(pname, values) }
    }

    pub fn glPatchParameteri(&self, pname: PatchParameterName, value: i32) {
        unsafe { self.fns.PatchParameteri(pname, value) }
    }

    pub fn glPauseTransformFeedback(&self) {
        unsafe { self.fns.PauseTransformFeedback() }
    }

    pub fn glPolygonOffsetClamp(&self, factor: f32, units: f32, clamp: f32) {
        unsafe { self.fns.PolygonOffsetClamp(factor, units, clamp) }
    }

    pub fn glProgramBinary(&self, program: u32, binaryFormat: GLenum, binary: *const c_void, length: u32) {
        // Unreviewed
        unsafe { self.fns.ProgramBinary(program, binaryFormat, binary, length as _) }
    }

    pub fn glProgramParameteri(&self, program: u32, pname: ProgramParameterPName, value: i32) {
        unsafe { self.fns.ProgramParameteri(program, pname, value) }
    }

    pub fn glProgramUniform1d(&self, program: u32, location: i32, v0: f64) {
        unsafe { self.fns.ProgramUniform1d(program, location, v0) }
    }

    pub fn glProgramUniform1dv(&self, program: u32, location: i32, count: u32, value: *const f64) {
        // Unreviewed
        unsafe { self.fns.ProgramUniform1dv(program, location, count as _, value) }
    }

    pub fn glProgramUniform1f(&self, program: u32, location: i32, v0: f32) {
        unsafe { self.fns.ProgramUniform1f(program, location, v0) }
    }

    pub fn glProgramUniform1fv(&self, program: u32, location: i32, count: u32, value: *const f32) {
        // Unreviewed
        unsafe { self.fns.ProgramUniform1fv(program, location, count as _, value) }
    }

    pub fn glProgramUniform1i(&self, program: u32, location: i32, v0: i32) {
        unsafe { self.fns.ProgramUniform1i(program, location, v0) }
    }

    pub fn glProgramUniform1iv(&self, program: u32, location: i32, count: u32, value: *const i32) {
        // Unreviewed
        unsafe { self.fns.ProgramUniform1iv(program, location, count as _, value) }
    }

    pub fn glProgramUniform1ui(&self, program: u32, location: i32, v0: u32) {
        unsafe { self.fns.ProgramUniform1ui(program, location, v0) }
    }

    pub fn glProgramUniform1uiv(&self, program: u32, location: i32, count: u32, value: *const u32) {
        // Unreviewed
        unsafe { self.fns.ProgramUniform1uiv(program, location, count as _, value) }
    }

    pub fn glProgramUniform2d(&self, program: u32, location: i32, v0: f64, v1: f64) {
        unsafe { self.fns.ProgramUniform2d(program, location, v0, v1) }
    }

    pub fn glProgramUniform2dv(&self, program: u32, location: i32, count: u32, value: *const f64) {
        // Unreviewed
        unsafe { self.fns.ProgramUniform2dv(program, location, count as _, value) }
    }

    pub fn glProgramUniform2f(&self, program: u32, location: i32, v0: f32, v1: f32) {
        unsafe { self.fns.ProgramUniform2f(program, location, v0, v1) }
    }

    pub fn glProgramUniform2fv(&self, program: u32, location: i32, count: u32, value: *const f32) {
        // Unreviewed
        unsafe { self.fns.ProgramUniform2fv(program, location, count as _, value) }
    }

    pub fn glProgramUniform2i(&self, program: u32, location: i32, v0: i32, v1: i32) {
        unsafe { self.fns.ProgramUniform2i(program, location, v0, v1) }
    }

    pub fn glProgramUniform2iv(&self, program: u32, location: i32, count: u32, value: *const i32) {
        // Unreviewed
        unsafe { self.fns.ProgramUniform2iv(program, location, count as _, value) }
    }

    pub fn glProgramUniform2ui(&self, program: u32, location: i32, v0: u32, v1: u32) {
        unsafe { self.fns.ProgramUniform2ui(program, location, v0, v1) }
    }

    pub fn glProgramUniform2uiv(&self, program: u32, location: i32, count: u32, value: *const u32) {
        // Unreviewed
        unsafe { self.fns.ProgramUniform2uiv(program, location, count as _, value) }
    }

    pub fn glProgramUniform3d(&self, program: u32, location: i32, v0: f64, v1: f64, v2: f64) {
        unsafe { self.fns.ProgramUniform3d(program, location, v0, v1, v2) }
    }

    pub fn glProgramUniform3dv(&self, program: u32, location: i32, count: u32, value: *const f64) {
        // Unreviewed
        unsafe { self.fns.ProgramUniform3dv(program, location, count as _, value) }
    }

    pub fn glProgramUniform3f(&self, program: u32, location: i32, v0: f32, v1: f32, v2: f32) {
        unsafe { self.fns.ProgramUniform3f(program, location, v0, v1, v2) }
    }

    pub fn glProgramUniform3fv(&self, program: u32, location: i32, count: u32, value: *const f32) {
        // Unreviewed
        unsafe { self.fns.ProgramUniform3fv(program, location, count as _, value) }
    }

    pub fn glProgramUniform3i(&self, program: u32, location: i32, v0: i32, v1: i32, v2: i32) {
        unsafe { self.fns.ProgramUniform3i(program, location, v0, v1, v2) }
    }

    pub fn glProgramUniform3iv(&self, program: u32, location: i32, count: u32, value: *const i32) {
        // Unreviewed
        unsafe { self.fns.ProgramUniform3iv(program, location, count as _, value) }
    }

    pub fn glProgramUniform3ui(&self, program: u32, location: i32, v0: u32, v1: u32, v2: u32) {
        unsafe { self.fns.ProgramUniform3ui(program, location, v0, v1, v2) }
    }

    pub fn glProgramUniform3uiv(&self, program: u32, location: i32, count: u32, value: *const u32) {
        // Unreviewed
        unsafe { self.fns.ProgramUniform3uiv(program, location, count as _, value) }
    }

    pub fn glProgramUniform4d(&self, program: u32, location: i32, v0: f64, v1: f64, v2: f64, v3: f64) {
        unsafe { self.fns.ProgramUniform4d(program, location, v0, v1, v2, v3) }
    }

    pub fn glProgramUniform4dv(&self, program: u32, location: i32, count: u32, value: *const f64) {
        // Unreviewed
        unsafe { self.fns.ProgramUniform4dv(program, location, count as _, value) }
    }

    pub fn glProgramUniform4f(&self, program: u32, location: i32, v0: f32, v1: f32, v2: f32, v3: f32) {
        unsafe { self.fns.ProgramUniform4f(program, location, v0, v1, v2, v3) }
    }

    pub fn glProgramUniform4fv(&self, program: u32, location: i32, count: u32, value: *const f32) {
        // Unreviewed
        unsafe { self.fns.ProgramUniform4fv(program, location, count as _, value) }
    }

    pub fn glProgramUniform4i(&self, program: u32, location: i32, v0: i32, v1: i32, v2: i32, v3: i32) {
        unsafe { self.fns.ProgramUniform4i(program, location, v0, v1, v2, v3) }
    }

    pub fn glProgramUniform4iv(&self, program: u32, location: i32, count: u32, value: *const i32) {
        // Unreviewed
        unsafe { self.fns.ProgramUniform4iv(program, location, count as _, value) }
    }

    pub fn glProgramUniform4ui(&self, program: u32, location: i32, v0: u32, v1: u32, v2: u32, v3: u32) {
        unsafe { self.fns.ProgramUniform4ui(program, location, v0, v1, v2, v3) }
    }

    pub fn glProgramUniform4uiv(&self, program: u32, location: i32, count: u32, value: *const u32) {
        // Unreviewed
        unsafe { self.fns.ProgramUniform4uiv(program, location, count as _, value) }
    }

    pub fn glProgramUniformMatrix2dv(&self, program: u32, location: i32, count: u32, transpose: bool, value: *const f64) {
        // Unreviewed
        unsafe { self.fns.ProgramUniformMatrix2dv(program, location, count as _, transpose as _, value) }
    }

    pub fn glProgramUniformMatrix2fv(&self, program: u32, location: i32, count: u32, transpose: bool, value: *const f32) {
        // Unreviewed
        unsafe { self.fns.ProgramUniformMatrix2fv(program, location, count as _, transpose as _, value) }
    }

    pub fn glProgramUniformMatrix2x3dv(&self, program: u32, location: i32, count: u32, transpose: bool, value: *const f64) {
        // Unreviewed
        unsafe { self.fns.ProgramUniformMatrix2x3dv(program, location, count as _, transpose as _, value) }
    }

    pub fn glProgramUniformMatrix2x3fv(&self, program: u32, location: i32, count: u32, transpose: bool, value: *const f32) {
        // Unreviewed
        unsafe { self.fns.ProgramUniformMatrix2x3fv(program, location, count as _, transpose as _, value) }
    }

    pub fn glProgramUniformMatrix2x4dv(&self, program: u32, location: i32, count: u32, transpose: bool, value: *const f64) {
        // Unreviewed
        unsafe { self.fns.ProgramUniformMatrix2x4dv(program, location, count as _, transpose as _, value) }
    }

    pub fn glProgramUniformMatrix2x4fv(&self, program: u32, location: i32, count: u32, transpose: bool, value: *const f32) {
        // Unreviewed
        unsafe { self.fns.ProgramUniformMatrix2x4fv(program, location, count as _, transpose as _, value) }
    }

    pub fn glProgramUniformMatrix3dv(&self, program: u32, location: i32, count: u32, transpose: bool, value: *const f64) {
        // Unreviewed
        unsafe { self.fns.ProgramUniformMatrix3dv(program, location, count as _, transpose as _, value) }
    }

    pub fn glProgramUniformMatrix3fv(&self, program: u32, location: i32, count: u32, transpose: bool, value: *const f32) {
        // Unreviewed
        unsafe { self.fns.ProgramUniformMatrix3fv(program, location, count as _, transpose as _, value) }
    }

    pub fn glProgramUniformMatrix3x2dv(&self, program: u32, location: i32, count: u32, transpose: bool, value: *const f64) {
        // Unreviewed
        unsafe { self.fns.ProgramUniformMatrix3x2dv(program, location, count as _, transpose as _, value) }
    }

    pub fn glProgramUniformMatrix3x2fv(&self, program: u32, location: i32, count: u32, transpose: bool, value: *const f32) {
        // Unreviewed
        unsafe { self.fns.ProgramUniformMatrix3x2fv(program, location, count as _, transpose as _, value) }
    }

    pub fn glProgramUniformMatrix3x4dv(&self, program: u32, location: i32, count: u32, transpose: bool, value: *const f64) {
        // Unreviewed
        unsafe { self.fns.ProgramUniformMatrix3x4dv(program, location, count as _, transpose as _, value) }
    }

    pub fn glProgramUniformMatrix3x4fv(&self, program: u32, location: i32, count: u32, transpose: bool, value: *const f32) {
        // Unreviewed
        unsafe { self.fns.ProgramUniformMatrix3x4fv(program, location, count as _, transpose as _, value) }
    }

    pub fn glProgramUniformMatrix4dv(&self, program: u32, location: i32, count: u32, transpose: bool, value: *const f64) {
        // Unreviewed
        unsafe { self.fns.ProgramUniformMatrix4dv(program, location, count as _, transpose as _, value) }
    }

    pub fn glProgramUniformMatrix4fv(&self, program: u32, location: i32, count: u32, transpose: bool, value: *const f32) {
        // Unreviewed
        unsafe { self.fns.ProgramUniformMatrix4fv(program, location, count as _, transpose as _, value) }
    }

    pub fn glProgramUniformMatrix4x2dv(&self, program: u32, location: i32, count: u32, transpose: bool, value: *const f64) {
        // Unreviewed
        unsafe { self.fns.ProgramUniformMatrix4x2dv(program, location, count as _, transpose as _, value) }
    }

    pub fn glProgramUniformMatrix4x2fv(&self, program: u32, location: i32, count: u32, transpose: bool, value: *const f32) {
        // Unreviewed
        unsafe { self.fns.ProgramUniformMatrix4x2fv(program, location, count as _, transpose as _, value) }
    }

    pub fn glProgramUniformMatrix4x3dv(&self, program: u32, location: i32, count: u32, transpose: bool, value: *const f64) {
        // Unreviewed
        unsafe { self.fns.ProgramUniformMatrix4x3dv(program, location, count as _, transpose as _, value) }
    }

    pub fn glProgramUniformMatrix4x3fv(&self, program: u32, location: i32, count: u32, transpose: bool, value: *const f32) {
        // Unreviewed
        unsafe { self.fns.ProgramUniformMatrix4x3fv(program, location, count as _, transpose as _, value) }
    }

    pub fn glReadnPixels(&self, x: i32, y: i32, width: u32, height: u32, format: PixelFormat, type_: PixelType, bufSize: u32, data: *mut c_void) {
        // Unreviewed
        unsafe { self.fns.ReadnPixels(x, y, width as _, height as _, format, type_, bufSize as _, data) }
    }

    pub fn glReleaseShaderCompiler(&self) {
        unsafe { self.fns.ReleaseShaderCompiler() }
    }

    pub fn glResumeTransformFeedback(&self) {
        unsafe { self.fns.ResumeTransformFeedback() }
    }

    pub fn glScissorArrayv(&self, first: u32, count: u32, v: *const i32) {
        // Unreviewed
        unsafe { self.fns.ScissorArrayv(first, count as _, v) }
    }

    pub fn glScissorIndexed(&self, index: u32, left: i32, bottom: i32, width: u32, height: u32) {
        unsafe { self.fns.ScissorIndexed(index, left, bottom, width as _, height as _) }
    }

    pub fn glScissorIndexedv(&self, index: u32, v: *const [i32; 4]) {
        // Unreviewed
        unsafe { self.fns.ScissorIndexedv(index, v) }
    }

    pub fn glShaderBinary(&self, count: u32, shaders: *const u32, binaryFormat: ShaderBinaryFormat, binary: *const c_void, length: u32) {
        // Unreviewed
        unsafe { self.fns.ShaderBinary(count as _, shaders, binaryFormat, binary, length as _) }
    }

    pub fn glShaderStorageBlockBinding(&self, program: u32, storageBlockIndex: u32, storageBlockBinding: u32) {
        unsafe { self.fns.ShaderStorageBlockBinding(program, storageBlockIndex, storageBlockBinding) }
    }

    pub fn glSpecializeShader(&self, shader: u32, pEntryPoint: *const u8, numSpecializationConstants: u32, pConstantIndex: *const u32, pConstantValue: *const u32) {
        // Unreviewed
        unsafe { self.fns.SpecializeShader(shader, pEntryPoint, numSpecializationConstants, pConstantIndex, pConstantValue) }
    }

    pub fn glTexBufferRange(&self, target: TextureTarget, internalformat: InternalFormat, buffer: u32, offset: isize, size: usize) {
        unsafe { self.fns.TexBufferRange(target, internalformat, buffer, offset, size as _) }
    }

    pub fn glTexStorage1D(&self, target: TextureTarget, levels: u32, internalformat: InternalFormat, width: u32) {
        unsafe { self.fns.TexStorage1D(target, levels as _, internalformat, width as _) }
    }

    pub fn glTexStorage2D(&self, target: TextureTarget, levels: u32, internalformat: InternalFormat, width: u32, height: u32) {
        unsafe { self.fns.TexStorage2D(target, levels as _, internalformat, width as _, height as _) }
    }

    pub fn glTexStorage2DMultisample(&self, target: TextureTarget, samples: u32, internalformat: InternalFormat, width: u32, height: u32, fixedsamplelocations: bool) {
        unsafe { self.fns.TexStorage2DMultisample(target, samples as _, internalformat, width as _, height as _, fixedsamplelocations as _) }
    }

    pub fn glTexStorage3D(&self, target: TextureTarget, levels: u32, internalformat: InternalFormat, width: u32, height: u32, depth: u32) {
        unsafe { self.fns.TexStorage3D(target, levels as _, internalformat, width as _, height as _, depth as _) }
    }

    pub fn glTexStorage3DMultisample(&self, target: TextureTarget, samples: u32, internalformat: InternalFormat, width: u32, height: u32, depth: u32, fixedsamplelocations: bool) {
        unsafe { self.fns.TexStorage3DMultisample(target, samples as _, internalformat, width as _, height as _, depth as _, fixedsamplelocations as _) }
    }

    pub fn glTextureBarrier(&self) {
        unsafe { self.fns.TextureBarrier() }
    }

    pub fn glTextureBuffer(&self, texture: u32, internalformat: InternalFormat, buffer: u32) {
        unsafe { self.fns.TextureBuffer(texture, internalformat, buffer) }
    }

    pub fn glTextureBufferRange(&self, texture: u32, internalformat: InternalFormat, buffer: u32, offset: isize, size: usize) {
        unsafe { self.fns.TextureBufferRange(texture, internalformat, buffer, offset, size as _) }
    }

    pub fn glTextureParameterIiv(&self, texture: u32, pname: TextureParameterName, params: *const i32) {
        // Unreviewed
        unsafe { self.fns.TextureParameterIiv(texture, pname, params) }
    }

    pub fn glTextureParameterIuiv(&self, texture: u32, pname: TextureParameterName, params: *const u32) {
        // Unreviewed
        unsafe { self.fns.TextureParameterIuiv(texture, pname, params) }
    }

    pub fn glTextureParameterf(&self, texture: u32, pname: TextureParameterName, param: f32) {
        unsafe { self.fns.TextureParameterf(texture, pname, param) }
    }

    pub fn glTextureParameterfv(&self, texture: u32, pname: TextureParameterName, param: *const f32) {
        // Unreviewed
        unsafe { self.fns.TextureParameterfv(texture, pname, param) }
    }

    pub fn glTextureParameteri(&self, texture: u32, pname: TextureParameterName, param: i32) {
        unsafe { self.fns.TextureParameteri(texture, pname, param) }
    }

    pub fn glTextureParameteriv(&self, texture: u32, pname: TextureParameterName, param: *const i32) {
        // Unreviewed
        unsafe { self.fns.TextureParameteriv(texture, pname, param) }
    }

    pub fn glTextureStorage1D(&self, texture: u32, levels: u32, internalformat: InternalFormat, width: u32) {
        unsafe { self.fns.TextureStorage1D(texture, levels as _, internalformat, width as _) }
    }

    pub fn glTextureStorage2D(&self, texture: u32, levels: u32, internalformat: InternalFormat, width: u32, height: u32) {
        unsafe { self.fns.TextureStorage2D(texture, levels as _, internalformat, width as _, height as _) }
    }

    pub fn glTextureStorage2DMultisample(&self, texture: u32, samples: u32, internalformat: InternalFormat, width: u32, height: u32, fixedsamplelocations: bool) {
        unsafe { self.fns.TextureStorage2DMultisample(texture, samples as _, internalformat, width as _, height as _, fixedsamplelocations as _) }
    }

    pub fn glTextureStorage3D(&self, texture: u32, levels: u32, internalformat: InternalFormat, width: u32, height: u32, depth: u32) {
        unsafe { self.fns.TextureStorage3D(texture, levels as _, internalformat, width as _, height as _, depth as _) }
    }

    pub fn glTextureStorage3DMultisample(&self, texture: u32, samples: u32, internalformat: InternalFormat, width: u32, height: u32, depth: u32, fixedsamplelocations: bool) {
        unsafe { self.fns.TextureStorage3DMultisample(texture, samples as _, internalformat, width as _, height as _, depth as _, fixedsamplelocations as _) }
    }

    pub fn glTextureSubImage1D(&self, texture: u32, level: i32, xoffset: i32, width: u32, format: PixelFormat, type_: PixelType, pixels: *const c_void) {
        // Unreviewed
        unsafe { self.fns.TextureSubImage1D(texture, level, xoffset, width as _, format, type_, pixels) }
    }

    pub fn glTextureSubImage2D(&self, texture: u32, level: i32, xoffset: i32, yoffset: i32, width: u32, height: u32, format: PixelFormat, type_: PixelType, pixels: *const c_void) {
        // Unreviewed
        unsafe { self.fns.TextureSubImage2D(texture, level, xoffset, yoffset, width as _, height as _, format, type_, pixels) }
    }

    pub fn glTextureSubImage3D(&self, texture: u32, level: i32, xoffset: i32, yoffset: i32, zoffset: i32, width: u32, height: u32, depth: u32, format: PixelFormat, type_: PixelType, pixels: *const c_void) {
        // Unreviewed
        unsafe { self.fns.TextureSubImage3D(texture, level, xoffset, yoffset, zoffset, width as _, height as _, depth as _, format, type_, pixels) }
    }

    pub fn glTextureView(&self, texture: u32, target: TextureTarget, origtexture: u32, internalformat: InternalFormat, minlevel: u32, numlevels: u32, minlayer: u32, numlayers: u32) {
        unsafe { self.fns.TextureView(texture, target, origtexture, internalformat, minlevel, numlevels, minlayer, numlayers) }
    }

    pub fn glTransformFeedbackBufferBase(&self, xfb: u32, index: u32, buffer: u32) {
        unsafe { self.fns.TransformFeedbackBufferBase(xfb, index, buffer) }
    }

    pub fn glTransformFeedbackBufferRange(&self, xfb: u32, index: u32, buffer: u32, offset: isize, size: usize) {
        unsafe { self.fns.TransformFeedbackBufferRange(xfb, index, buffer, offset, size as _) }
    }

    pub fn glUniform1d(&self, location: i32, x: f64) {
        unsafe { self.fns.Uniform1d(location, x) }
    }

    pub fn glUniform1dv(&self, location: i32, count: u32, value: *const f64) {
        // Unreviewed
        unsafe { self.fns.Uniform1dv(location, count as _, value) }
    }

    pub fn glUniform2d(&self, location: i32, x: f64, y: f64) {
        unsafe { self.fns.Uniform2d(location, x, y) }
    }

    pub fn glUniform2dv(&self, location: i32, count: u32, value: *const f64) {
        // Unreviewed
        unsafe { self.fns.Uniform2dv(location, count as _, value) }
    }

    pub fn glUniform3d(&self, location: i32, x: f64, y: f64, z: f64) {
        unsafe { self.fns.Uniform3d(location, x, y, z) }
    }

    pub fn glUniform3dv(&self, location: i32, count: u32, value: *const f64) {
        // Unreviewed
        unsafe { self.fns.Uniform3dv(location, count as _, value) }
    }

    pub fn glUniform4d(&self, location: i32, x: f64, y: f64, z: f64, w: f64) {
        unsafe { self.fns.Uniform4d(location, x, y, z, w) }
    }

    pub fn glUniform4dv(&self, location: i32, count: u32, value: *const f64) {
        // Unreviewed
        unsafe { self.fns.Uniform4dv(location, count as _, value) }
    }

    pub fn glUniformMatrix2dv(&self, location: i32, count: u32, transpose: bool, value: *const f64) {
        // Unreviewed
        unsafe { self.fns.UniformMatrix2dv(location, count as _, transpose as _, value) }
    }

    pub fn glUniformMatrix2x3dv(&self, location: i32, count: u32, transpose: bool, value: *const f64) {
        // Unreviewed
        unsafe { self.fns.UniformMatrix2x3dv(location, count as _, transpose as _, value) }
    }

    pub fn glUniformMatrix2x4dv(&self, location: i32, count: u32, transpose: bool, value: *const f64) {
        // Unreviewed
        unsafe { self.fns.UniformMatrix2x4dv(location, count as _, transpose as _, value) }
    }

    pub fn glUniformMatrix3dv(&self, location: i32, count: u32, transpose: bool, value: *const f64) {
        // Unreviewed
        unsafe { self.fns.UniformMatrix3dv(location, count as _, transpose as _, value) }
    }

    pub fn glUniformMatrix3x2dv(&self, location: i32, count: u32, transpose: bool, value: *const f64) {
        // Unreviewed
        unsafe { self.fns.UniformMatrix3x2dv(location, count as _, transpose as _, value) }
    }

    pub fn glUniformMatrix3x4dv(&self, location: i32, count: u32, transpose: bool, value: *const f64) {
        // Unreviewed
        unsafe { self.fns.UniformMatrix3x4dv(location, count as _, transpose as _, value) }
    }

    pub fn glUniformMatrix4dv(&self, location: i32, count: u32, transpose: bool, value: *const f64) {
        // Unreviewed
        unsafe { self.fns.UniformMatrix4dv(location, count as _, transpose as _, value) }
    }

    pub fn glUniformMatrix4x2dv(&self, location: i32, count: u32, transpose: bool, value: *const f64) {
        // Unreviewed
        unsafe { self.fns.UniformMatrix4x2dv(location, count as _, transpose as _, value) }
    }

    pub fn glUniformMatrix4x3dv(&self, location: i32, count: u32, transpose: bool, value: *const f64) {
        // Unreviewed
        unsafe { self.fns.UniformMatrix4x3dv(location, count as _, transpose as _, value) }
    }

    pub fn glUniformSubroutinesuiv(&self, shadertype: ShaderType, count: u32, indices: *const u32) {
        // Unreviewed
        unsafe { self.fns.UniformSubroutinesuiv(shadertype, count as _, indices) }
    }

    pub fn glUnmapNamedBuffer(&self, buffer: u32) -> bool {
        unsafe { self.fns.UnmapNamedBuffer(buffer) > 0 }
    }

    pub fn glUseProgramStages(&self, pipeline: u32, stages: GLbitfield, program: u32) {
        unsafe { self.fns.UseProgramStages(pipeline, stages, program) }
    }

    pub fn glValidateProgramPipeline(&self, pipeline: u32) {
        unsafe { self.fns.ValidateProgramPipeline(pipeline) }
    }

    pub fn glVertexArrayAttribBinding(&self, vaobj: u32, attribindex: u32, bindingindex: u32) {
        unsafe { self.fns.VertexArrayAttribBinding(vaobj, attribindex, bindingindex) }
    }

    pub fn glVertexArrayAttribFormat(&self, vaobj: u32, attribindex: u32, size: i32, type_: VertexAttribType, normalized: bool, relativeoffset: u32) {
        unsafe { self.fns.VertexArrayAttribFormat(vaobj, attribindex, size, type_, normalized as _, relativeoffset) }
    }

    pub fn glVertexArrayAttribIFormat(&self, vaobj: u32, attribindex: u32, size: i32, type_: VertexAttribIType, relativeoffset: u32) {
        unsafe { self.fns.VertexArrayAttribIFormat(vaobj, attribindex, size, type_, relativeoffset) }
    }

    pub fn glVertexArrayAttribLFormat(&self, vaobj: u32, attribindex: u32, size: i32, type_: VertexAttribLType, relativeoffset: u32) {
        unsafe { self.fns.VertexArrayAttribLFormat(vaobj, attribindex, size, type_, relativeoffset) }
    }

    pub fn glVertexArrayBindingDivisor(&self, vaobj: u32, bindingindex: u32, divisor: u32) {
        unsafe { self.fns.VertexArrayBindingDivisor(vaobj, bindingindex, divisor) }
    }

    pub fn glVertexArrayElementBuffer(&self, vaobj: u32, buffer: u32) {
        unsafe { self.fns.VertexArrayElementBuffer(vaobj, buffer) }
    }

    pub fn glVertexArrayVertexBuffer(&self, vaobj: u32, bindingindex: u32, buffer: u32, offset: isize, stride: u32) {
        unsafe { self.fns.VertexArrayVertexBuffer(vaobj, bindingindex, buffer, offset, stride as _) }
    }

    pub fn glVertexArrayVertexBuffers(&self, vaobj: u32, first: u32, count: u32, buffers: *const u32, offsets: *const isize, strides: *const u32) {
        // Unreviewed
        unsafe { self.fns.VertexArrayVertexBuffers(vaobj, first, count as _, buffers, offsets, strides as _) }
    }

    pub fn glVertexAttribBinding(&self, attribindex: u32, bindingindex: u32) {
        unsafe { self.fns.VertexAttribBinding(attribindex, bindingindex) }
    }

    pub fn glVertexAttribFormat(&self, attribindex: u32, size: i32, type_: VertexAttribType, normalized: bool, relativeoffset: u32) {
        unsafe { self.fns.VertexAttribFormat(attribindex, size, type_, normalized as _, relativeoffset) }
    }

    pub fn glVertexAttribIFormat(&self, attribindex: u32, size: i32, type_: VertexAttribIType, relativeoffset: u32) {
        unsafe { self.fns.VertexAttribIFormat(attribindex, size, type_, relativeoffset) }
    }

    pub fn glVertexAttribL1d(&self, index: u32, x: f64) {
        unsafe { self.fns.VertexAttribL1d(index, x) }
    }

    pub fn glVertexAttribL1dv(&self, index: u32, v: *const f64) {
        // Unreviewed
        unsafe { self.fns.VertexAttribL1dv(index, v) }
    }

    pub fn glVertexAttribL2d(&self, index: u32, x: f64, y: f64) {
        unsafe { self.fns.VertexAttribL2d(index, x, y) }
    }

    pub fn glVertexAttribL2dv(&self, index: u32, v: *const [f64; 2]) {
        // Unreviewed
        unsafe { self.fns.VertexAttribL2dv(index, v) }
    }

    pub fn glVertexAttribL3d(&self, index: u32, x: f64, y: f64, z: f64) {
        unsafe { self.fns.VertexAttribL3d(index, x, y, z) }
    }

    pub fn glVertexAttribL3dv(&self, index: u32, v: *const [f64; 3]) {
        // Unreviewed
        unsafe { self.fns.VertexAttribL3dv(index, v) }
    }

    pub fn glVertexAttribL4d(&self, index: u32, x: f64, y: f64, z: f64, w: f64) {
        unsafe { self.fns.VertexAttribL4d(index, x, y, z, w) }
    }

    pub fn glVertexAttribL4dv(&self, index: u32, v: *const [f64; 4]) {
        // Unreviewed
        unsafe { self.fns.VertexAttribL4dv(index, v) }
    }

    pub fn glVertexAttribLFormat(&self, attribindex: u32, size: i32, type_: VertexAttribLType, relativeoffset: u32) {
        unsafe { self.fns.VertexAttribLFormat(attribindex, size, type_, relativeoffset) }
    }

    pub fn glVertexAttribLPointer(&self, index: u32, size: i32, type_: VertexAttribLType, stride: u32, pointer: *const c_void) {
        // Unreviewed
        unsafe { self.fns.VertexAttribLPointer(index, size, type_, stride as _, pointer) }
    }

    pub fn glVertexBindingDivisor(&self, bindingindex: u32, divisor: u32) {
        unsafe { self.fns.VertexBindingDivisor(bindingindex, divisor) }
    }

    pub fn glViewportArrayv(&self, first: u32, count: u32, v: *const f32) {
        // Unreviewed
        unsafe { self.fns.ViewportArrayv(first, count as _, v) }
    }

    pub fn glViewportIndexedf(&self, index: u32, x: f32, y: f32, w: f32, h: f32) {
        unsafe { self.fns.ViewportIndexedf(index, x, y, w, h) }
    }

    pub fn glViewportIndexedfv(&self, index: u32, v: *const [f32; 4]) {
        // Unreviewed
        unsafe { self.fns.ViewportIndexedfv(index, v) }
    }

    pub fn glGetImageHandleARB(&self, texture: u32, level: i32, layered: bool, layer: i32, format: PixelFormat) -> u64 {
        unsafe { self.fns.GetImageHandleARB(texture, level, layered as _, layer, format) }
    }

    pub fn glGetTextureHandleARB(&self, texture: u32) -> u64 {
        // Unreviewed
        unsafe { self.fns.GetTextureHandleARB(texture) }
    }

    pub fn glGetTextureSamplerHandleARB(&self, texture: u32, sampler: u32) -> u64 {
        unsafe { self.fns.GetTextureSamplerHandleARB(texture, sampler) }
    }

    pub fn glGetVertexAttribLui64vARB(&self, index: u32, pname: VertexAttribEnum, params: *mut u64) {
        // Unreviewed
        unsafe { self.fns.GetVertexAttribLui64vARB(index, pname, params) }
    }

    pub fn glIsImageHandleResidentARB(&self, handle: u64) -> bool {
        unsafe { self.fns.IsImageHandleResidentARB(handle) > 0 }
    }

    pub fn glIsTextureHandleResidentARB(&self, handle: u64) -> bool {
        unsafe { self.fns.IsTextureHandleResidentARB(handle) > 0 }
    }

    pub fn glMakeImageHandleNonResidentARB(&self, handle: u64) {
        unsafe { self.fns.MakeImageHandleNonResidentARB(handle) }
    }

    pub fn glMakeImageHandleResidentARB(&self, handle: u64, access: GLenum) {
        unsafe { self.fns.MakeImageHandleResidentARB(handle, access) }
    }

    pub fn glMakeTextureHandleNonResidentARB(&self, handle: u64) {
        unsafe { self.fns.MakeTextureHandleNonResidentARB(handle) }
    }

    pub fn glMakeTextureHandleResidentARB(&self, handle: u64) {
        unsafe { self.fns.MakeTextureHandleResidentARB(handle) }
    }

    pub fn glProgramUniformHandleui64ARB(&self, program: u32, location: i32, value: u64) {
        unsafe { self.fns.ProgramUniformHandleui64ARB(program, location, value) }
    }

    pub fn glProgramUniformHandleui64vARB(&self, program: u32, location: i32, count: u32, values: *const u64) {
        // Unreviewed
        unsafe { self.fns.ProgramUniformHandleui64vARB(program, location, count as _, values) }
    }

    pub fn glTexPageCommitmentARB(&self, target: GLenum, level: i32, xoffset: i32, yoffset: i32, zoffset: i32, width: u32, height: u32, depth: u32, commit: bool) {
        unsafe { self.fns.TexPageCommitmentARB(target, level, xoffset, yoffset, zoffset, width as _, height as _, depth as _, commit as _) }
    }

    pub fn glUniformHandleui64ARB(&self, location: i32, value: u64) {
        unsafe { self.fns.UniformHandleui64ARB(location, value) }
    }

    pub fn glUniformHandleui64vARB(&self, location: i32, count: u32, value: *const u64) {
        // Unreviewed
        unsafe { self.fns.UniformHandleui64vARB(location, count as _, value) }
    }

    pub fn glVertexAttribL1ui64ARB(&self, index: u32, x: u64) {
        unsafe { self.fns.VertexAttribL1ui64ARB(index, x) }
    }

    pub fn glVertexAttribL1ui64vARB(&self, index: u32, v: *const u64) {
        // Unreviewed
        unsafe { self.fns.VertexAttribL1ui64vARB(index, v) }
    }
} 