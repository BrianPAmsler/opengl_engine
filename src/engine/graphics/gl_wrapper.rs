#![allow(non_snake_case)]

use std::{ffi::{c_void, CStr, CString}, ops::{Deref, DerefMut}};

use anyhow::{Result, anyhow};

use gl33::*;
use glfw::GLProc;

pub struct CStringArray {
    strings: Box<[CString]>,
    ptrs: Box<[*const u8]>
}

impl CStringArray {
    pub fn new(strings: &[&str]) -> CStringArray {
        let strings: Box<[CString]> = strings.iter().map(|s| CString::new(*s).unwrap()).collect();
        let ptrs = strings.iter().map(|s| s.as_ptr() as *const u8).collect();

        CStringArray { strings, ptrs }
    }

    pub fn as_ptr(&self) -> *const *const u8 {
        self.ptrs.as_ptr()
    }
}

impl Deref for CStringArray {
    type Target = Box<[CString]>;

    fn deref(&self) -> &Self::Target {
        &self.strings
    }
}

impl DerefMut for CStringArray {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.strings
    }
}

pub struct GLWrapper {
    fns: GlFns
}

impl GLWrapper {
    pub(in crate::engine::graphics) fn init_gl<F: Fn(*const u8) -> GLProc>(f: F) -> Result<GLWrapper> {
        let fns = unsafe { GlFns::load_from(&f).map_err(|e| anyhow!(e))? };

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
        let null_str = CStr::from_bytes_with_nul(name.as_bytes()).unwrap();
        unsafe { self.fns.BindAttribLocation(program, index, null_str.as_ptr() as *const u8) }
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
        let null_str = CStr::from_bytes_with_nul(name.as_bytes()).unwrap();
        unsafe { self.fns.BindFragDataLocation(program, color, null_str.as_ptr() as *const u8) }
    }
    
    pub fn glBindFragDataLocationIndexed(&self, program: u32, colorNumber: u32, index: u32, name: &str) {
        let null_str = CStr::from_bytes_with_nul(name.as_bytes()).unwrap();
        unsafe { self.fns.BindFragDataLocationIndexed(program, colorNumber, index, null_str.as_ptr() as *const u8) }
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
    
    pub fn glBufferData(&self, target: BufferTargetARB, data: &[u8], usage: BufferUsageARB) {
        unsafe { self.fns.BufferData(target, data.len() as isize, data.as_ptr() as *const c_void, usage) }
    }
    
    pub fn glBufferSubData(&self, target: BufferTargetARB, offset: isize, data: &[u8]) {
        unsafe { self.fns.BufferSubData(target, offset, data.len() as isize, data.as_ptr() as *const c_void) }
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
        unsafe { self.fns.ClearBufferfi(buffer, drawbuffer, depth, stencil) }
    }
    
    pub fn glClearBufferfv(&self, buffer: Buffer, drawbuffer: i32, value: &mut [f32]) {
        let expected_size = match buffer {
            GL_COLOR => 4,
            GL_DEPTH => 1,
            GL_STENCIL => 1,
            _ => 0
        };

        if value.len() != expected_size {
            panic!("Incorrect value size!")
        }

        unsafe { self.fns.ClearBufferfv(buffer, drawbuffer, value.as_ptr()) }
    }
    
    pub fn glClearBufferiv(&self, buffer: Buffer, drawbuffer: i32, value: &mut [i32]) {
        let expected_size = match buffer {
            GL_COLOR => 4,
            GL_DEPTH => 1,
            GL_STENCIL => 1,
            _ => 0
        };

        if value.len() != expected_size {
            panic!("Incorrect value size!")
        }

        unsafe { self.fns.ClearBufferiv(buffer, drawbuffer, value.as_ptr()) }
    }
    
    pub fn glClearBufferuiv(&self, buffer: Buffer, drawbuffer: i32, value: &mut [u32]) {
        let expected_size = match buffer {
            GL_COLOR => 4,
            GL_DEPTH => 1,
            GL_STENCIL => 1,
            _ => 0
        };

        if value.len() != expected_size {
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
    
    pub fn glCompressedTexImage1D(&self, target: TextureTarget, level: i32, internalformat: InternalFormat, width: i32, border: i32, data: &[u8]) {
        unsafe { self.fns.CompressedTexImage1D(target, level, internalformat, width, border, data.len() as i32, data.as_ptr() as *const c_void) }
    }
    
    pub fn glCompressedTexImage2D(&self, target: TextureTarget, level: i32, internalformat: InternalFormat, width: i32, height: i32, border: i32, data: &[u8]) {
        unsafe { self.fns.CompressedTexImage2D(target, level, internalformat, width, height, border, data.len() as i32, data.as_ptr() as *const c_void) }
    }
    
    pub fn glCompressedTexImage3D(&self, target: TextureTarget, level: i32, internalformat: InternalFormat, width: i32, height: i32, depth: i32, border: i32, data: &[u8]) {
        unsafe { self.fns.CompressedTexImage3D(target, level, internalformat, width, height, depth, border, data.len() as i32, data.as_ptr() as *const c_void) }
    }
    
    pub fn glCompressedTexSubImage1D(&self, target: TextureTarget, level: i32, xoffset: i32, width: i32, format: PixelFormat, data: &[u8]) {
        unsafe { self.fns.CompressedTexSubImage1D(target, level, xoffset, width, format, data.len() as i32, data.as_ptr() as *const c_void) }
    }
    
    pub fn glCompressedTexSubImage2D(&self, target: TextureTarget, level: i32, xoffset: i32, yoffset: i32, width: i32, height: i32, format: PixelFormat, data: &[u8]) {
        unsafe { self.fns.CompressedTexSubImage2D(target, level, xoffset, yoffset, width, height, format, data.len() as i32, data.as_ptr() as *const c_void) }
    }
    
    pub fn glCompressedTexSubImage3D(&self, target: TextureTarget, level: i32, xoffset: i32, yoffset: i32, zoffset: i32, width: i32, height: i32, depth: i32, format: PixelFormat, data: &[u8]) {
        unsafe { self.fns.CompressedTexSubImage3D(target, level, xoffset, yoffset, zoffset, width, height, depth, format, data.len() as i32, data.as_ptr() as *const c_void) }
    }
    
    pub fn glCopyBufferSubData(&self, readTarget: CopyBufferSubDataTarget, writeTarget: CopyBufferSubDataTarget, readOffset: isize, writeOffset: isize, size: isize) {
        unsafe { self.fns.CopyBufferSubData(readTarget, writeTarget, readOffset, writeOffset, size) }
    }
    
    pub fn glCopyTexImage1D(&self, target: TextureTarget, level: i32, internalformat: InternalFormat, x: i32, y: i32, width: i32, border: i32) {
        unsafe { self.fns.CopyTexImage1D(target, level, internalformat, x, y, width, border) }
    }
    
    pub unsafe fn glCopyTexImage2D(&self, target: TextureTarget, level: i32, internalformat: InternalFormat, x: i32, y: i32, width: i32, height: i32, border: i32) {
        self.fns.CopyTexImage2D(target, level, internalformat, x, y, width, height, border)
    }
    
    pub unsafe fn glCopyTexSubImage1D(&self, target: TextureTarget, level: i32, xoffset: i32, x: i32, y: i32, width: i32) {
        self.fns.CopyTexSubImage1D(target, level, xoffset, x, y, width)
    }
    
    pub unsafe fn glCopyTexSubImage2D(&self, target: TextureTarget, level: i32, xoffset: i32, yoffset: i32, x: i32, y: i32, width: i32, height: i32) {
        self.fns.CopyTexSubImage2D(target, level, xoffset, yoffset, x, y, width, height)
    }
    
    pub unsafe fn glCopyTexSubImage3D(&self, target: TextureTarget, level: i32, xoffset: i32, yoffset: i32, zoffset: i32, x: i32, y: i32, width: i32, height: i32) {
        self.fns.CopyTexSubImage3D(target, level, xoffset, yoffset, zoffset, x, y, width, height)
    }
    
    pub fn glCreateProgram(&self) -> u32 {
        self.fns.CreateProgram()
    }
    
    pub fn glCreateShader(&self, type_: ShaderType) -> u32 {
        self.fns.CreateShader(type_)
    }
    
    pub unsafe fn glCullFace(&self, mode: CullFaceMode) {
        self.fns.CullFace(mode)
    }
    
    pub unsafe fn glDeleteBuffers(&self, n: i32, buffers: *const u32) {
        self.fns.DeleteBuffers(n, buffers)
    }
    
    pub unsafe fn glDeleteFramebuffers(&self, n: i32, framebuffers: *const u32) {
        self.fns.DeleteFramebuffers(n, framebuffers)
    }
    
    pub fn glDeleteProgram(&self, program: u32) {
        self.fns.DeleteProgram(program)
    }
    
    pub unsafe fn glDeleteQueries(&self, n: i32, ids: *const u32) {
        self.fns.DeleteQueries(n, ids)
    }
    
    pub unsafe fn glDeleteRenderbuffers(&self, n: i32, renderbuffers: *const u32) {
        self.fns.DeleteRenderbuffers(n, renderbuffers)
    }
    
    pub unsafe fn glDeleteSamplers(&self, count: i32, samplers: *const u32) {
        self.fns.DeleteSamplers(count, samplers)
    }
    
    pub fn glDeleteShader(&self, shader: u32) {
        self.fns.DeleteShader(shader)
    }
    
    pub unsafe fn glDeleteSync(&self, sync: GLsync) {
        self.fns.DeleteSync(sync)
    }
    
    pub unsafe fn glDeleteTextures(&self, n: i32, textures: *const u32) {
        self.fns.DeleteTextures(n, textures)
    }
    
    pub unsafe fn glDeleteVertexArrays(&self, n: i32, arrays: *const u32) {
        self.fns.DeleteVertexArrays(n, arrays)
    }
    
    pub unsafe fn glDepthFunc(&self, func: DepthFunction) {
        self.fns.DepthFunc(func)
    }
    
    pub unsafe fn glDepthMask(&self, flag: u8) {
        self.fns.DepthMask(flag)
    }
    
    pub unsafe fn glDepthRange(&self, n: f64, f: f64) {
        self.fns.DepthRange(n, f)
    }
    
    pub unsafe fn glDetachShader(&self, program: u32, shader: u32) {
        self.fns.DetachShader(program, shader)
    }
    
    pub unsafe fn glDisable(&self, cap: EnableCap) {
        self.fns.Disable(cap)
    }
    
    pub unsafe fn glDisableVertexAttribArray(&self, index: u32) {
        self.fns.DisableVertexAttribArray(index)
    }
    
    pub unsafe fn glDisablei(&self, target: EnableCap, index: u32) {
        self.fns.Disablei(target, index)
    }
    
    pub unsafe fn glDrawArrays(&self, mode: PrimitiveType, first: i32, count: i32) {
        self.fns.DrawArrays(mode, first, count)
    }
    
    pub unsafe fn glDrawArraysInstanced(&self, mode: PrimitiveType, first: i32, count: i32, instancecount: i32) {
        self.fns.DrawArraysInstanced(mode, first, count, instancecount)
    }
    
    pub unsafe fn glDrawBuffer(&self, buf: DrawBufferMode) {
        self.fns.DrawBuffer(buf)
    }
    
    pub unsafe fn glDrawBuffers(&self, n: i32, bufs: *const DrawBufferMode) {
        self.fns.DrawBuffers(n, bufs)
    }
    
    pub unsafe fn glDrawElements(&self, mode: PrimitiveType, count: i32, type_: DrawElementsType, indices: *const c_void) {
        self.fns.DrawElements(mode, count, type_, indices)
    }
    
    pub unsafe fn glDrawElementsBaseVertex(&self, mode: PrimitiveType, count: i32, type_: DrawElementsType, indices: *const c_void, basevertex: i32) {
        self.fns.DrawElementsBaseVertex(mode, count, type_, indices, basevertex)
    }
    
    pub unsafe fn glDrawElementsInstanced(&self, mode: PrimitiveType, count: i32, type_: DrawElementsType, indices: *const c_void, instancecount: i32) {
        self.fns.DrawElementsInstanced(mode, count, type_, indices, instancecount)
    }
    
    pub unsafe fn glDrawElementsInstancedBaseVertex(&self, mode: PrimitiveType, count: i32, type_: DrawElementsType, indices: *const c_void, instancecount: i32, basevertex: i32) {
        self.fns.DrawElementsInstancedBaseVertex(mode, count, type_, indices, instancecount, basevertex)
    }
    
    pub unsafe fn glDrawRangeElements(&self, mode: PrimitiveType, start: u32, end: u32, count: i32, type_: DrawElementsType, indices: *const c_void) {
        self.fns.DrawRangeElements(mode, start, end, count, type_, indices)
    }
    
    pub unsafe fn glDrawRangeElementsBaseVertex(&self, mode: PrimitiveType, start: u32, end: u32, count: i32, type_: DrawElementsType, indices: *const c_void, basevertex: i32) {
        self.fns.DrawRangeElementsBaseVertex(mode, start, end, count, type_, indices, basevertex)
    }
    
    pub unsafe fn glEnable(&self, cap: EnableCap) {
        self.fns.Enable(cap)
    }
    
    pub unsafe fn glEnableVertexAttribArray(&self, index: u32) {
        self.fns.EnableVertexAttribArray(index)
    }
    
    pub unsafe fn glEnablei(&self, target: EnableCap, index: u32) {
        self.fns.Enablei(target, index)
    }
    
    pub unsafe fn glEndConditionalRender(&self) {
        self.fns.EndConditionalRender()
    }
    
    pub unsafe fn glEndQuery(&self, target: QueryTarget) {
        self.fns.EndQuery(target)
    }
    
    pub unsafe fn glEndTransformFeedback(&self) {
        self.fns.EndTransformFeedback()
    }
    
    pub unsafe fn glFenceSync(&self, condition: SyncCondition, flags: GLbitfield) -> GLsync {
        self.fns.FenceSync(condition, flags)
    }
    
    pub unsafe fn glFinish(&self) {
        self.fns.Finish()
    }
    
    pub unsafe fn glFlush(&self) {
        self.fns.Flush()
    }
    
    pub unsafe fn glFlushMappedBufferRange(&self, target: BufferTargetARB, offset: isize, length: isize) {
        self.fns.FlushMappedBufferRange(target, offset, length)
    }
    
    pub unsafe fn glFramebufferRenderbuffer(&self, target: FramebufferTarget, attachment: FramebufferAttachment, renderbuffertarget: RenderbufferTarget, renderbuffer: u32) {
        self.fns.FramebufferRenderbuffer(target, attachment, renderbuffertarget, renderbuffer)
    }
    
    pub unsafe fn glFramebufferTexture(&self, target: FramebufferTarget, attachment: FramebufferAttachment, texture: u32, level: i32) {
        self.fns.FramebufferTexture(target, attachment, texture, level)
    }
    
    pub unsafe fn glFramebufferTexture1D(&self, target: FramebufferTarget, attachment: FramebufferAttachment, textarget: TextureTarget, texture: u32, level: i32) {
        self.fns.FramebufferTexture1D(target, attachment, textarget, texture, level)
    }
    
    pub unsafe fn glFramebufferTexture2D(&self, target: FramebufferTarget, attachment: FramebufferAttachment, textarget: TextureTarget, texture: u32, level: i32) {
        self.fns.FramebufferTexture2D(target, attachment, textarget, texture, level)
    }
    
    pub unsafe fn glFramebufferTexture3D(&self, target: FramebufferTarget, attachment: FramebufferAttachment, textarget: TextureTarget, texture: u32, level: i32, zoffset: i32) {
        self.fns.FramebufferTexture3D(target, attachment, textarget, texture, level, zoffset)
    }
    
    pub unsafe fn glFramebufferTextureLayer(&self, target: FramebufferTarget, attachment: FramebufferAttachment, texture: u32, level: i32, layer: i32) {
        self.fns.FramebufferTextureLayer(target, attachment, texture, level, layer)
    }
    
    pub unsafe fn glFrontFace(&self, mode: FrontFaceDirection) {
        self.fns.FrontFace(mode)
    }
    
    pub unsafe fn glGenBuffers(&self, n: i32, buffers: *mut u32) {
        self.fns.GenBuffers(n, buffers)
    }
    
    pub unsafe fn glGenFramebuffers(&self, n: i32, framebuffers: *mut u32) {
        self.fns.GenFramebuffers(n, framebuffers)
    }
    
    pub unsafe fn glGenQueries(&self, n: i32, ids: *mut u32) {
        self.fns.GenQueries(n, ids)
    }
    
    pub unsafe fn glGenRenderbuffers(&self, n: i32, renderbuffers: *mut u32) {
        self.fns.GenRenderbuffers(n, renderbuffers)
    }
    
    pub unsafe fn glGenSamplers(&self, count: i32, samplers: *mut u32) {
        self.fns.GenSamplers(count, samplers)
    }
    
    pub unsafe fn glGenTextures(&self, n: i32, textures: *mut u32) {
        self.fns.GenTextures(n, textures)
    }
    
    pub unsafe fn glGenVertexArrays(&self, n: i32, arrays: *mut u32) {
        self.fns.GenVertexArrays(n, arrays)
    }
    
    pub unsafe fn glGenerateMipmap(&self, target: TextureTarget) {
        self.fns.GenerateMipmap(target)
    }
    
    pub unsafe fn glGetActiveAttrib(&self, program: u32, index: u32, bufSize: i32, length: *mut i32, size: *mut i32, type_: *mut AttributeType, name: *mut u8) {
        self.fns.GetActiveAttrib(program, index, bufSize, length, size, type_, name)
    }
    
    pub unsafe fn glGetActiveUniform(&self, program: u32, index: u32, bufSize: i32, length: *mut i32, size: *mut i32, type_: *mut UniformType, name: *mut u8) {
        self.fns.GetActiveUniform(program, index, bufSize, length, size, type_, name)
    }
    
    pub unsafe fn glGetActiveUniformBlockName(&self, program: u32, uniformBlockIndex: u32, bufSize: i32, length: *mut i32, uniformBlockName: *mut u8) {
        self.fns.GetActiveUniformBlockName(program, uniformBlockIndex, bufSize, length, uniformBlockName)
    }
    
    pub unsafe fn glGetActiveUniformBlockiv(&self, program: u32, uniformBlockIndex: u32, pname: UniformBlockPName, params: *mut i32) {
        self.fns.GetActiveUniformBlockiv(program, uniformBlockIndex, pname, params)
    }
    
    pub unsafe fn glGetActiveUniformName(&self, program: u32, uniformIndex: u32, bufSize: i32, length: *mut i32, uniformName: *mut u8) {
        self.fns.GetActiveUniformName(program, uniformIndex, bufSize, length, uniformName)
    }
    
    pub unsafe fn glGetActiveUniformsiv(&self, program: u32, uniformCount: i32, uniformIndices: *const u32, pname: UniformPName, params: *mut i32) {
        self.fns.GetActiveUniformsiv(program, uniformCount, uniformIndices, pname, params)
    }
    
    pub unsafe fn glGetAttachedShaders(&self, program: u32, maxCount: i32, count: *mut i32, shaders: *mut u32) {
        self.fns.GetAttachedShaders(program, maxCount, count, shaders)
    }
    
    pub fn glGetAttribLocation(&self, program: u32, name: &str) -> i32 {
        let null_str = CStr::from_bytes_with_nul(name.as_bytes()).unwrap();
        unsafe { self.fns.GetAttribLocation(program, null_str.as_ptr() as *const u8) }
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
        let null_str = CStr::from_bytes_with_nul(name.as_bytes()).unwrap();
        unsafe { self.fns.GetFragDataIndex(program, null_str.as_ptr() as *const u8) }
    }
    
    pub fn glGetFragDataLocation(&self, program: u32, name: &str) -> i32 {
        let null_str = CStr::from_bytes_with_nul(name.as_bytes()).unwrap();
        unsafe { self.fns.GetFragDataLocation(program, null_str.as_ptr() as *const u8) }
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
    
    pub unsafe fn glGetShaderInfoLog(&self, shader: u32, bufSize: i32, length: *mut i32, infoLog: *mut u8) {
        self.fns.GetShaderInfoLog(shader, bufSize, length, infoLog)
    }
    
    pub unsafe fn glGetShaderSource(&self, shader: u32, bufSize: i32, length: *mut i32, source: *mut u8) {
        self.fns.GetShaderSource(shader, bufSize, length, source)
    }
    
    pub unsafe fn glGetShaderiv(&self, shader: u32, pname: ShaderParameterName, params: *mut i32) {
        self.fns.GetShaderiv(shader, pname, params)
    }
    
    pub unsafe fn glGetString(&self, name: StringName) -> String {
        let raw_str = self.fns.GetString(name);

        unsafe {
            let len = libc::strlen(raw_str as *const i8);
            let slice = std::slice::from_raw_parts(raw_str, len);
            String::from_utf8_lossy(slice).to_string()
        }
    }
    
    pub unsafe fn glGetStringi(&self, name: StringName, index: u32) -> String {
        let raw_str = self.fns.GetStringi(name, index);

        unsafe {
            let len = libc::strlen(raw_str as *const i8);
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
        let null_str = CStr::from_bytes_with_nul(uniformBlockName.as_bytes()).unwrap();
        unsafe { self.fns.GetUniformBlockIndex(program, null_str.as_ptr() as *const u8) }
    }
    
    // NEEDS TESTING
    pub fn glGetUniformIndices(&self, program: u32, uniformNames: &CStringArray, uniformIndices: &mut [u32]) {
        unsafe { self.fns.GetUniformIndices(program, uniformNames.len() as i32, uniformNames.as_ptr(), uniformIndices.as_mut_ptr()) }
    }
    
    pub fn glGetUniformLocation(&self, program: u32, name: &str) -> i32 {
        let null_str = CStr::from_bytes_with_nul(name.as_bytes()).unwrap();
        unsafe { self.fns.GetUniformLocation(program, null_str.as_ptr() as *const u8) }
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
        self.fns.LineWidth(width)
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
    
    pub unsafe fn glReadPixels(&self, x: i32, y: i32, width: i32, height: i32, format: PixelFormat, type_: PixelType, pixels: *mut c_void) {
        self.fns.ReadPixels(x, y, width, height, format, type_, pixels)
    }
    
    pub unsafe fn glRenderbufferStorage(&self, target: RenderbufferTarget, internalformat: InternalFormat, width: i32, height: i32) {
        self.fns.RenderbufferStorage(target, internalformat, width, height)
    }
    
    pub unsafe fn glRenderbufferStorageMultisample(&self, target: RenderbufferTarget, samples: i32, internalformat: InternalFormat, width: i32, height: i32) {
        self.fns.RenderbufferStorageMultisample(target, samples, internalformat, width, height)
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
    
    pub unsafe fn glScissor(&self, x: i32, y: i32, width: i32, height: i32) {
        self.fns.Scissor(x, y, width, height)
    }
    
    // NEEDS TESTING
    pub fn glShaderSource(&self, shader: u32, string: &CStringArray) {
        unsafe { self.fns.ShaderSource(shader, string.len() as i32, string.as_ptr(), 0 as *const i32) }
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
    
    pub unsafe fn glTexImage1D(&self, target: TextureTarget, level: i32, internalformat: i32, width: i32, border: i32, format: PixelFormat, type_: PixelType, pixels: *const c_void) {
        self.fns.TexImage1D(target, level, internalformat, width, border, format, type_, pixels)
    }
    
    pub unsafe fn glTexImage2D(&self, target: TextureTarget, level: i32, internalformat: i32, width: i32, height: i32, border: i32, format: PixelFormat, type_: PixelType, pixels: *const c_void) {
        self.fns.TexImage2D(target, level, internalformat, width, height, border, format, type_, pixels)
    }
    
    pub unsafe fn glTexImage2DMultisample(&self, target: TextureTarget, samples: i32, internalformat: InternalFormat, width: i32, height: i32, fixedsamplelocations: u8) {
        self.fns.TexImage2DMultisample(target, samples, internalformat, width, height, fixedsamplelocations)
    }
    
    pub unsafe fn glTexImage3D(&self, target: TextureTarget, level: i32, internalformat: i32, width: i32, height: i32, depth: i32, border: i32, format: PixelFormat, type_: PixelType, pixels: *const c_void) {
        self.fns.TexImage3D(target, level, internalformat, width, height, depth, border, format, type_, pixels)
    }
    
    pub unsafe fn glTexImage3DMultisample(&self, target: TextureTarget, samples: i32, internalformat: InternalFormat, width: i32, height: i32, depth: i32, fixedsamplelocations: u8) {
        self.fns.TexImage3DMultisample(target, samples, internalformat, width, height, depth, fixedsamplelocations)
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
    
    pub unsafe fn glTexSubImage1D(&self, target: TextureTarget, level: i32, xoffset: i32, width: i32, format: PixelFormat, type_: PixelType, pixels: *const c_void) {
        self.fns.TexSubImage1D(target, level, xoffset, width, format, type_, pixels)
    }
    
    pub unsafe fn glTexSubImage2D(&self, target: TextureTarget, level: i32, xoffset: i32, yoffset: i32, width: i32, height: i32, format: PixelFormat, type_: PixelType, pixels: *const c_void) {
        self.fns.TexSubImage2D(target, level, xoffset, yoffset, width, height, format, type_, pixels)
    }
    
    pub unsafe fn glTexSubImage3D(&self, target: TextureTarget, level: i32, xoffset: i32, yoffset: i32, zoffset: i32, width: i32, height: i32, depth: i32, format: PixelFormat, type_: PixelType, pixels: *const c_void) {
        self.fns.TexSubImage3D(target, level, xoffset, yoffset, zoffset, width, height, depth, format, type_, pixels)
    }
    
    pub fn glTransformFeedbackVaryings(&self, program: u32, varyings: &CStringArray, bufferMode: TransformFeedbackBufferMode) {
        unsafe { self.fns.TransformFeedbackVaryings(program, varyings.len() as i32, varyings.as_ptr(), bufferMode) }
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
    
    pub unsafe fn glVertexAttribPointer(&self, index: u32, size: i32, type_: VertexAttribPointerType, normalized: u8, stride: i32, pointer: *const c_void) {
        self.fns.VertexAttribPointer(index, size, type_, normalized, stride, pointer)
    }
    
    pub unsafe fn glViewport(&self, x: i32, y: i32, width: i32, height: i32) {
        self.fns.Viewport(x, y, width, height)
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
        unsafe { self.fns.DebugMessageInsert(source, type_, id, severity, buf.len() as i32, buf.as_ptr()) }
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
        unsafe { self.fns.ObjectLabel(identifier, name, label.len() as i32, label.as_ptr()) }
    }
    
    pub fn glObjectPtrLabel(&self, ptr: *const c_void, label: &str) {
        unsafe { self.fns.ObjectPtrLabel(ptr, label.len() as i32, label.as_ptr()) }
    }
    
    pub unsafe fn glPopDebugGroup(&self) {
        self.fns.PopDebugGroup()
    }
    
    pub fn glPushDebugGroup(&self, source: DebugSource, id: u32, message: &str) {
        unsafe { self.fns.PushDebugGroup(source, id, message.len() as i32, message.as_ptr()) }
    }
    
    
} 