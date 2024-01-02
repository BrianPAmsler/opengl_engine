#![allow(non_snake_case)]

use std::ffi::{c_void, c_uint, c_float, c_int, c_double, c_uchar};

use anyhow::{Result, anyhow};

use gl33::*;

pub struct GLWrapper {
    fns: GlFns
}

impl GLWrapper {
    pub fn init_gl<F: Fn(*const u8) -> *const c_void>(f: F) -> Result<GLWrapper> {
        let fns = unsafe { GlFns::load_from(&f).map_err(|e| anyhow!(e))? };

        Ok(GLWrapper { fns })
    }
    pub fn glActiveTexture(&self, texture: TextureUnit) {
        unsafe {self.fns.ActiveTexture(texture)}
    }
    pub fn glAttachShader(&self, program: c_uint, shader: c_uint) {
        self.fns.AttachShader(program, shader)
    }
    pub fn glBeginConditionalRender(&self, id: c_uint, mode: ConditionalRenderMode) {
        unsafe {self.fns.BeginConditionalRender(id, mode)}
    }
    pub fn glBeginQuery(&self, target: QueryTarget, id: c_uint) {
        unsafe {self.fns.BeginQuery(target, id)}
    }
    pub fn glBeginTransformFeedback(&self, primitiveMode: PrimitiveType) {
        unsafe {self.fns.BeginTransformFeedback(primitiveMode)}
    }
    pub fn glBindAttribLocation(&self, program: c_uint, index: c_uint, name: *const u8) {
        unsafe {self.fns.BindAttribLocation(program, index, name)}
    }
    pub fn glBindBuffer(&self, target: BufferTargetARB, buffer: c_uint) {
        unsafe {self.fns.BindBuffer(target, buffer)}
    }
    pub fn glBindBufferBase(&self, target: BufferTargetARB, index: c_uint, buffer: c_uint) {
        unsafe {self.fns.BindBufferBase(target, index, buffer)}
    }
    pub fn glBindBufferRange(&self, target: BufferTargetARB, index: c_uint, buffer: c_uint, offset: isize, size: isize) {
        unsafe {self.fns.BindBufferRange(target, index, buffer, offset, size)}
    }
    pub fn glBindFragDataLocation(&self, program: c_uint, color: c_uint, name: *const u8) {
        unsafe {self.fns.BindFragDataLocation(program, color, name)}
    }
    pub fn glBindFragDataLocationIndexed(&self, program: c_uint, colorNumber: c_uint, index: c_uint, name: *const u8) {
        unsafe {self.fns.BindFragDataLocationIndexed(program, colorNumber, index, name)}
    }
    pub fn glBindFramebuffer(&self, target: FramebufferTarget, framebuffer: c_uint) {
        unsafe {self.fns.BindFramebuffer(target, framebuffer)}
    }
    pub fn glBindRenderbuffer(&self, target: RenderbufferTarget, renderbuffer: c_uint) {
        unsafe {self.fns.BindRenderbuffer(target, renderbuffer)}
    }
    pub fn glBindSampler(&self, unit: c_uint, sampler: c_uint) {
        unsafe {self.fns.BindSampler(unit, sampler)}
    }
    pub fn glBindTexture(&self, target: TextureTarget, texture: c_uint) {
        unsafe {self.fns.BindTexture(target, texture)}
    }
    pub fn glBindVertexArray(&self, array: c_uint) {
        self.fns.BindVertexArray(array)
    }
    pub fn glBlendColor(&self, red: c_float, green: c_float, blue: c_float, alpha: c_float) {
        unsafe {self.fns.BlendColor(red, green, blue, alpha)}
    }
    pub fn glBlendEquation(&self, mode: BlendEquationModeEXT) {
        unsafe {self.fns.BlendEquation(mode)}
    }
    pub fn glBlendEquationSeparate(&self, modeRGB: BlendEquationModeEXT, modeAlpha: BlendEquationModeEXT) {
        unsafe {self.fns.BlendEquationSeparate(modeRGB, modeAlpha)}
    }
    pub fn glBlendFunc(&self, sfactor: BlendingFactor, dfactor: BlendingFactor) {
        unsafe {self.fns.BlendFunc(sfactor, dfactor)}
    }
    pub fn glBlendFuncSeparate(&self, sfactorRGB: BlendingFactor, dfactorRGB: BlendingFactor, sfactorAlpha: BlendingFactor, dfactorAlpha: BlendingFactor) {
        unsafe {self.fns.BlendFuncSeparate(sfactorRGB, dfactorRGB, sfactorAlpha, dfactorAlpha)}
    }
    pub fn glBlitFramebuffer(&self, srcX0: c_int, srcY0: c_int, srcX1: c_int, srcY1: c_int, dstX0: c_int, dstY0: c_int, dstX1: c_int, dstY1: c_int, mask: GLbitfield, filter: BlitFramebufferFilter) {
        unsafe {self.fns.BlitFramebuffer(srcX0, srcY0, srcX1, srcY1, dstX0, dstY0, dstX1, dstY1, mask, filter)}
    }
    pub fn glBufferData(&self, target: BufferTargetARB, size: isize, data: *const c_void, usage: BufferUsageARB) {
        unsafe {self.fns.BufferData(target, size, data, usage)}
    }
    pub fn glBufferSubData(&self, target: BufferTargetARB, offset: isize, size: isize, data: *const c_void) {
        unsafe {self.fns.BufferSubData(target, offset, size, data)}
    }
    pub fn glCheckFramebufferStatus(&self, target: FramebufferTarget) -> FramebufferStatus{
        unsafe {self.fns.CheckFramebufferStatus(target)}
    }
    pub fn glClampColor(&self, target: ClampColorTargetARB, clamp: ClampColorModeARB) {
        unsafe {self.fns.ClampColor(target, clamp)}
    }
    pub fn glClear(&self, mask: GLbitfield) {
        unsafe {self.fns.Clear(mask)}
    }
    pub fn glClearBufferfi(&self, buffer: Buffer, drawbuffer: c_int, depth: c_float, stencil: c_int) {
        unsafe {self.fns.ClearBufferfi(buffer, drawbuffer, depth, stencil)}
    }
    pub fn glClearBufferfv(&self, buffer: Buffer, drawbuffer: c_int, value: *const c_float) {
        unsafe {self.fns.ClearBufferfv(buffer, drawbuffer, value)}
    }
    pub fn glClearBufferiv(&self, buffer: Buffer, drawbuffer: c_int, value: *const c_int) {
        unsafe {self.fns.ClearBufferiv(buffer, drawbuffer, value)}
    }
    pub fn glClearBufferuiv(&self, buffer: Buffer, drawbuffer: c_int, value: *const c_uint) {
        unsafe {self.fns.ClearBufferuiv(buffer, drawbuffer, value)}
    }
    pub fn glClearColor(&self, red: c_float, green: c_float, blue: c_float, alpha: c_float) {
        unsafe {self.fns.ClearColor(red, green, blue, alpha)}
    }
    pub fn glClearDepth(&self, depth: c_double) {
        unsafe {self.fns.ClearDepth(depth)}
    }
    pub fn glClearStencil(&self, s: c_int) {
        unsafe {self.fns.ClearStencil(s)}
    }
    pub fn glClientWaitSync(&self, sync: GLsync, flags: GLbitfield, timeout: u64) -> SyncStatus{
        unsafe {self.fns.ClientWaitSync(sync, flags, timeout)}
    }
    pub fn glColorMask(&self, red: c_uchar, green: c_uchar, blue: c_uchar, alpha: c_uchar) {
        unsafe {self.fns.ColorMask(red, green, blue, alpha)}
    }
    pub fn glColorMaski(&self, index: c_uint, r: c_uchar, g: c_uchar, b: c_uchar, a: c_uchar) {
        unsafe {self.fns.ColorMaski(index, r, g, b, a)}
    }
    pub fn glCompileShader(&self, shader: c_uint) {
        self.fns.CompileShader(shader)
    }
    pub fn glCompressedTexImage1D(&self, target: TextureTarget, level: c_int, internalformat: InternalFormat, width: c_int, border: c_int, imageSize: c_int, data: *const c_void) {
        unsafe {self.fns.CompressedTexImage1D(target, level, internalformat, width, border, imageSize, data)}
    }
    pub fn glCompressedTexImage2D(&self, target: TextureTarget, level: c_int, internalformat: InternalFormat, width: c_int, height: c_int, border: c_int, imageSize: c_int, data: *const c_void) {
        unsafe {self.fns.CompressedTexImage2D(target, level, internalformat, width, height, border, imageSize, data)}
    }
    pub fn glCompressedTexImage3D(&self, target: TextureTarget, level: c_int, internalformat: InternalFormat, width: c_int, height: c_int, depth: c_int, border: c_int, imageSize: c_int, data: *const c_void) {
        unsafe {self.fns.CompressedTexImage3D(target, level, internalformat, width, height, depth, border, imageSize, data)}
    }
    pub fn glCompressedTexSubImage1D(&self, target: TextureTarget, level: c_int, xoffset: c_int, width: c_int, format: PixelFormat, imageSize: c_int, data: *const c_void) {
        unsafe {self.fns.CompressedTexSubImage1D(target, level, xoffset, width, format, imageSize, data)}
    }
    pub fn glCompressedTexSubImage2D(&self, target: TextureTarget, level: c_int, xoffset: c_int, yoffset: c_int, width: c_int, height: c_int, format: PixelFormat, imageSize: c_int, data: *const c_void) {
        unsafe {self.fns.CompressedTexSubImage2D(target, level, xoffset, yoffset, width, height, format, imageSize, data)}
    }
    pub fn glCompressedTexSubImage3D(&self, target: TextureTarget, level: c_int, xoffset: c_int, yoffset: c_int, zoffset: c_int, width: c_int, height: c_int, depth: c_int, format: PixelFormat, imageSize: c_int, data: *const c_void) {
        unsafe {self.fns.CompressedTexSubImage3D(target, level, xoffset, yoffset, zoffset, width, height, depth, format, imageSize, data)}
    }
    pub fn glCopyBufferSubData(&self, readTarget: CopyBufferSubDataTarget, writeTarget: CopyBufferSubDataTarget, readOffset: isize, writeOffset: isize, size: isize) {
        unsafe {self.fns.CopyBufferSubData(readTarget, writeTarget, readOffset, writeOffset, size)}
    }
    pub fn glCopyTexImage1D(&self, target: TextureTarget, level: c_int, internalformat: InternalFormat, x: c_int, y: c_int, width: c_int, border: c_int) {
        unsafe {self.fns.CopyTexImage1D(target, level, internalformat, x, y, width, border)}
    }
    pub fn glCopyTexImage2D(&self, target: TextureTarget, level: c_int, internalformat: InternalFormat, x: c_int, y: c_int, width: c_int, height: c_int, border: c_int) {
        unsafe {self.fns.CopyTexImage2D(target, level, internalformat, x, y, width, height, border)}
    }
    pub fn glCopyTexSubImage1D(&self, target: TextureTarget, level: c_int, xoffset: c_int, x: c_int, y: c_int, width: c_int) {
        unsafe {self.fns.CopyTexSubImage1D(target, level, xoffset, x, y, width)}
    }
    pub fn glCopyTexSubImage2D(&self, target: TextureTarget, level: c_int, xoffset: c_int, yoffset: c_int, x: c_int, y: c_int, width: c_int, height: c_int) {
        unsafe {self.fns.CopyTexSubImage2D(target, level, xoffset, yoffset, x, y, width, height)}
    }
    pub fn glCopyTexSubImage3D(&self, target: TextureTarget, level: c_int, xoffset: c_int, yoffset: c_int, zoffset: c_int, x: c_int, y: c_int, width: c_int, height: c_int) {
        unsafe {self.fns.CopyTexSubImage3D(target, level, xoffset, yoffset, zoffset, x, y, width, height)}
    }
    pub fn glCreateProgram(&self) -> c_uint{
        self.fns.CreateProgram()
    }
    pub fn glCreateShader(&self, type_: ShaderType) -> c_uint{
        self.fns.CreateShader(type_)
    }
    pub fn glCullFace(&self, mode: CullFaceMode) {
        unsafe {self.fns.CullFace(mode)}
    }
    pub fn glDeleteBuffers(&self, n: c_int, buffers: *const c_uint) {
        unsafe {self.fns.DeleteBuffers(n, buffers)}
    }
    pub fn glDeleteFramebuffers(&self, n: c_int, framebuffers: *const c_uint) {
        unsafe {self.fns.DeleteFramebuffers(n, framebuffers)}
    }
    pub fn glDeleteProgram(&self, program: c_uint) {
        self.fns.DeleteProgram(program)
    }
    pub fn glDeleteQueries(&self, n: c_int, ids: *const c_uint) {
        unsafe {self.fns.DeleteQueries(n, ids)}
    }
    pub fn glDeleteRenderbuffers(&self, n: c_int, renderbuffers: *const c_uint) {
        unsafe {self.fns.DeleteRenderbuffers(n, renderbuffers)}
    }
    pub fn glDeleteSamplers(&self, count: c_int, samplers: *const c_uint) {
        unsafe {self.fns.DeleteSamplers(count, samplers)}
    }
    pub fn glDeleteShader(&self, shader: c_uint) {
        self.fns.DeleteShader(shader)
    }
    pub fn glDeleteSync(&self, sync: GLsync) {
        unsafe {self.fns.DeleteSync(sync)}
    }
    pub fn glDeleteTextures(&self, n: c_int, textures: *const c_uint) {
        unsafe {self.fns.DeleteTextures(n, textures)}
    }
    pub fn glDeleteVertexArrays(&self, n: c_int, arrays: *const c_uint) {
        unsafe {self.fns.DeleteVertexArrays(n, arrays)}
    }
    pub fn glDepthFunc(&self, func: DepthFunction) {
        unsafe {self.fns.DepthFunc(func)}
    }
    pub fn glDepthMask(&self, flag: c_uchar) {
        unsafe {self.fns.DepthMask(flag)}
    }
    pub fn glDepthRange(&self, n: c_double, f: c_double) {
        unsafe {self.fns.DepthRange(n, f)}
    }
    pub fn glDetachShader(&self, program: c_uint, shader: c_uint) {
        unsafe {self.fns.DetachShader(program, shader)}
    }
    pub fn glDisable(&self, cap: EnableCap) {
        unsafe {self.fns.Disable(cap)}
    }
    pub fn glDisableVertexAttribArray(&self, index: c_uint) {
        unsafe {self.fns.DisableVertexAttribArray(index)}
    }
    pub fn glDisablei(&self, target: EnableCap, index: c_uint) {
        unsafe {self.fns.Disablei(target, index)}
    }
    pub fn glDrawArrays(&self, mode: PrimitiveType, first: c_int, count: c_int) {
        unsafe {self.fns.DrawArrays(mode, first, count)}
    }
    pub fn glDrawArraysInstanced(&self, mode: PrimitiveType, first: c_int, count: c_int, instancecount: c_int) {
        unsafe {self.fns.DrawArraysInstanced(mode, first, count, instancecount)}
    }
    pub fn glDrawBuffer(&self, buf: DrawBufferMode) {
        unsafe {self.fns.DrawBuffer(buf)}
    }
    pub fn glDrawBuffers(&self, n: c_int, bufs: *const DrawBufferMode) {
        unsafe {self.fns.DrawBuffers(n, bufs)}
    }
    pub fn glDrawElements(&self, mode: PrimitiveType, count: c_int, type_: DrawElementsType, indices: *const c_void) {
        unsafe {self.fns.DrawElements(mode, count, type_, indices)}
    }
    pub fn glDrawElementsBaseVertex(&self, mode: PrimitiveType, count: c_int, type_: DrawElementsType, indices: *const c_void, basevertex: c_int) {
        unsafe {self.fns.DrawElementsBaseVertex(mode, count, type_, indices, basevertex)}
    }
    pub fn glDrawElementsInstanced(&self, mode: PrimitiveType, count: c_int, type_: DrawElementsType, indices: *const c_void, instancecount: c_int) {
        unsafe {self.fns.DrawElementsInstanced(mode, count, type_, indices, instancecount)}
    }
    pub fn glDrawElementsInstancedBaseVertex(&self, mode: PrimitiveType, count: c_int, type_: DrawElementsType, indices: *const c_void, instancecount: c_int, basevertex: c_int) {
        unsafe {self.fns.DrawElementsInstancedBaseVertex(mode, count, type_, indices, instancecount, basevertex)}
    }
    pub fn glDrawRangeElements(&self, mode: PrimitiveType, start: c_uint, end: c_uint, count: c_int, type_: DrawElementsType, indices: *const c_void) {
        unsafe {self.fns.DrawRangeElements(mode, start, end, count, type_, indices)}
    }
    pub fn glDrawRangeElementsBaseVertex(&self, mode: PrimitiveType, start: c_uint, end: c_uint, count: c_int, type_: DrawElementsType, indices: *const c_void, basevertex: c_int) {
        unsafe {self.fns.DrawRangeElementsBaseVertex(mode, start, end, count, type_, indices, basevertex)}
    }
    pub fn glEnable(&self, cap: EnableCap) {
        unsafe {self.fns.Enable(cap)}
    }
    pub fn glEnableVertexAttribArray(&self, index: c_uint) {
        unsafe {self.fns.EnableVertexAttribArray(index)}
    }
    pub fn glEnablei(&self, target: EnableCap, index: c_uint) {
        unsafe {self.fns.Enablei(target, index)}
    }
    pub fn glEndConditionalRender(&self) {
        unsafe {self.fns.EndConditionalRender()}
    }
    pub fn glEndQuery(&self, target: QueryTarget) {
        unsafe {self.fns.EndQuery(target)}
    }
    pub fn glEndTransformFeedback(&self) {
        unsafe {self.fns.EndTransformFeedback()}
    }
    pub fn glFenceSync(&self, condition: SyncCondition, flags: GLbitfield) -> GLsync{
        unsafe {self.fns.FenceSync(condition, flags)}
    }
    pub fn glFinish(&self) {
        unsafe {self.fns.Finish()}
    }
    pub fn glFlush(&self) {
        unsafe {self.fns.Flush()}
    }
    pub fn glFlushMappedBufferRange(&self, target: BufferTargetARB, offset: isize, length: isize) {
        unsafe {self.fns.FlushMappedBufferRange(target, offset, length)}
    }
    pub fn glFramebufferRenderbuffer(&self, target: FramebufferTarget, attachment: FramebufferAttachment, renderbuffertarget: RenderbufferTarget, renderbuffer: c_uint) {
        unsafe {self.fns.FramebufferRenderbuffer(target, attachment, renderbuffertarget, renderbuffer)}
    }
    pub fn glFramebufferTexture(&self, target: FramebufferTarget, attachment: FramebufferAttachment, texture: c_uint, level: c_int) {
        unsafe {self.fns.FramebufferTexture(target, attachment, texture, level)}
    }
    pub fn glFramebufferTexture1D(&self, target: FramebufferTarget, attachment: FramebufferAttachment, textarget: TextureTarget, texture: c_uint, level: c_int) {
        unsafe {self.fns.FramebufferTexture1D(target, attachment, textarget, texture, level)}
    }
    pub fn glFramebufferTexture2D(&self, target: FramebufferTarget, attachment: FramebufferAttachment, textarget: TextureTarget, texture: c_uint, level: c_int) {
        unsafe {self.fns.FramebufferTexture2D(target, attachment, textarget, texture, level)}
    }
    pub fn glFramebufferTexture3D(&self, target: FramebufferTarget, attachment: FramebufferAttachment, textarget: TextureTarget, texture: c_uint, level: c_int, zoffset: c_int) {
        unsafe {self.fns.FramebufferTexture3D(target, attachment, textarget, texture, level, zoffset)}
    }
    pub fn glFramebufferTextureLayer(&self, target: FramebufferTarget, attachment: FramebufferAttachment, texture: c_uint, level: c_int, layer: c_int) {
        unsafe {self.fns.FramebufferTextureLayer(target, attachment, texture, level, layer)}
    }
    pub fn glFrontFace(&self, mode: FrontFaceDirection) {
        unsafe {self.fns.FrontFace(mode)}
    }
    pub fn glGenBuffers(&self, n: c_int, buffers: *mut c_uint) {
        unsafe {self.fns.GenBuffers(n, buffers)}
    }
    pub fn glGenFramebuffers(&self, n: c_int, framebuffers: *mut c_uint) {
        unsafe {self.fns.GenFramebuffers(n, framebuffers)}
    }
    pub fn glGenQueries(&self, n: c_int, ids: *mut c_uint) {
        unsafe {self.fns.GenQueries(n, ids)}
    }
    pub fn glGenRenderbuffers(&self, n: c_int, renderbuffers: *mut c_uint) {
        unsafe {self.fns.GenRenderbuffers(n, renderbuffers)}
    }
    pub fn glGenSamplers(&self, count: c_int, samplers: *mut c_uint) {
        unsafe {self.fns.GenSamplers(count, samplers)}
    }
    pub fn glGenTextures(&self, n: c_int, textures: *mut c_uint) {
        unsafe {self.fns.GenTextures(n, textures)}
    }
    pub fn glGenVertexArrays(&self, n: c_int, arrays: *mut c_uint) {
        unsafe {self.fns.GenVertexArrays(n, arrays)}
    }
    pub fn glGenerateMipmap(&self, target: TextureTarget) {
        unsafe {self.fns.GenerateMipmap(target)}
    }
    pub fn glGetActiveAttrib(&self, program: c_uint, index: c_uint, bufSize: c_int, length: *mut c_int, size: *mut c_int, type_: *mut AttributeType, name: *mut u8) {
        unsafe {self.fns.GetActiveAttrib(program, index, bufSize, length, size, type_, name)}
    }
    pub fn glGetActiveUniform(&self, program: c_uint, index: c_uint, bufSize: c_int, length: *mut c_int, size: *mut c_int, type_: *mut UniformType, name: *mut u8) {
        unsafe {self.fns.GetActiveUniform(program, index, bufSize, length, size, type_, name)}
    }
    pub fn glGetActiveUniformBlockName(&self, program: c_uint, uniformBlockIndex: c_uint, bufSize: c_int, length: *mut c_int, uniformBlockName: *mut u8) {
        unsafe {self.fns.GetActiveUniformBlockName(program, uniformBlockIndex, bufSize, length, uniformBlockName)}
    }
    pub fn glGetActiveUniformBlockiv(&self, program: c_uint, uniformBlockIndex: c_uint, pname: UniformBlockPName, params: *mut c_int) {
        unsafe {self.fns.GetActiveUniformBlockiv(program, uniformBlockIndex, pname, params)}
    }
    pub fn glGetActiveUniformName(&self, program: c_uint, uniformIndex: c_uint, bufSize: c_int, length: *mut c_int, uniformName: *mut u8) {
        unsafe {self.fns.GetActiveUniformName(program, uniformIndex, bufSize, length, uniformName)}
    }
    pub fn glGetActiveUniformsiv(&self, program: c_uint, uniformCount: c_int, uniformIndices: *const c_uint, pname: UniformPName, params: *mut c_int) {
        unsafe {self.fns.GetActiveUniformsiv(program, uniformCount, uniformIndices, pname, params)}
    }
    pub fn glGetAttachedShaders(&self, program: c_uint, maxCount: c_int, count: *mut c_int, shaders: *mut c_uint) {
        unsafe {self.fns.GetAttachedShaders(program, maxCount, count, shaders)}
    }
    pub fn glGetAttribLocation(&self, program: c_uint, name: *const u8) -> c_int{
        unsafe {self.fns.GetAttribLocation(program, name)}
    }
    pub fn glGetBooleani_v(&self, target: BufferTargetARB, index: c_uint, data: *mut c_uchar) {
        unsafe {self.fns.GetBooleani_v(target, index, data)}
    }
    pub fn glGetBooleanv(&self, pname: GetPName, data: *mut c_uchar) {
        unsafe {self.fns.GetBooleanv(pname, data)}
    }
    pub fn glGetBufferParameteri64v(&self, target: BufferTargetARB, pname: BufferPNameARB, params: *mut i64) {
        unsafe {self.fns.GetBufferParameteri64v(target, pname, params)}
    }
    pub fn glGetBufferParameteriv(&self, target: BufferTargetARB, pname: BufferPNameARB, params: *mut c_int) {
        unsafe {self.fns.GetBufferParameteriv(target, pname, params)}
    }
    pub fn glGetBufferPointerv(&self, target: BufferTargetARB, pname: BufferPointerNameARB, params: *mut *mut c_void) {
        unsafe {self.fns.GetBufferPointerv(target, pname, params)}
    }
    pub fn glGetBufferSubData(&self, target: BufferTargetARB, offset: isize, size: isize, data: *mut c_void) {
        unsafe {self.fns.GetBufferSubData(target, offset, size, data)}
    }
    pub fn glGetCompressedTexImage(&self, target: TextureTarget, level: c_int, img: *mut c_void) {
        unsafe {self.fns.GetCompressedTexImage(target, level, img)}
    }
    pub fn glGetDoublev(&self, pname: GetPName, data: *mut c_double) {
        unsafe {self.fns.GetDoublev(pname, data)}
    }
    pub fn glGetError(&self) -> ErrorCode{
        unsafe {self.fns.GetError()}
    }
    pub fn glGetFloatv(&self, pname: GetPName, data: *mut c_float) {
        unsafe {self.fns.GetFloatv(pname, data)}
    }
    pub fn glGetFragDataIndex(&self, program: c_uint, name: *const u8) -> c_int{
        unsafe {self.fns.GetFragDataIndex(program, name)}
    }
    pub fn glGetFragDataLocation(&self, program: c_uint, name: *const u8) -> c_int{
        unsafe {self.fns.GetFragDataLocation(program, name)}
    }
    pub fn glGetFramebufferAttachmentParameteriv(&self, target: FramebufferTarget, attachment: FramebufferAttachment, pname: FramebufferAttachmentParameterName, params: *mut c_int) {
        unsafe {self.fns.GetFramebufferAttachmentParameteriv(target, attachment, pname, params)}
    }
    pub fn glGetInteger64i_v(&self, target: GetPName, index: c_uint, data: *mut i64) {
        unsafe {self.fns.GetInteger64i_v(target, index, data)}
    }
    pub fn glGetInteger64v(&self, pname: GetPName, data: *mut i64) {
        unsafe {self.fns.GetInteger64v(pname, data)}
    }
    pub fn glGetIntegeri_v(&self, target: GetPName, index: c_uint, data: *mut c_int) {
        unsafe {self.fns.GetIntegeri_v(target, index, data)}
    }
    pub fn glGetIntegerv(&self, pname: GetPName, data: *mut c_int) {
        unsafe {self.fns.GetIntegerv(pname, data)}
    }
    pub fn glGetMultisamplefv(&self, pname: GetMultisamplePNameNV, index: c_uint, val: *mut c_float) {
        unsafe {self.fns.GetMultisamplefv(pname, index, val)}
    }
    pub fn glGetProgramInfoLog(&self, program: c_uint, bufSize: c_int, length: *mut c_int, infoLog: *mut u8) {
        unsafe {self.fns.GetProgramInfoLog(program, bufSize, length, infoLog)}
    }
    pub fn glGetProgramiv(&self, program: c_uint, pname: ProgramPropertyARB, params: *mut c_int) {
        unsafe {self.fns.GetProgramiv(program, pname, params)}
    }
    pub fn glGetQueryObjecti64v(&self, id: c_uint, pname: QueryObjectParameterName, params: *mut i64) {
        unsafe {self.fns.GetQueryObjecti64v(id, pname, params)}
    }
    pub fn glGetQueryObjectiv(&self, id: c_uint, pname: QueryObjectParameterName, params: *mut c_int) {
        unsafe {self.fns.GetQueryObjectiv(id, pname, params)}
    }
    pub fn glGetQueryObjectui64v(&self, id: c_uint, pname: QueryObjectParameterName, params: *mut u64) {
        unsafe {self.fns.GetQueryObjectui64v(id, pname, params)}
    }
    pub fn glGetQueryObjectuiv(&self, id: c_uint, pname: QueryObjectParameterName, params: *mut c_uint) {
        unsafe {self.fns.GetQueryObjectuiv(id, pname, params)}
    }
    pub fn glGetQueryiv(&self, target: QueryTarget, pname: QueryParameterName, params: *mut c_int) {
        unsafe {self.fns.GetQueryiv(target, pname, params)}
    }
    pub fn glGetRenderbufferParameteriv(&self, target: RenderbufferTarget, pname: RenderbufferParameterName, params: *mut c_int) {
        unsafe {self.fns.GetRenderbufferParameteriv(target, pname, params)}
    }
    pub fn glGetSamplerParameterIiv(&self, sampler: c_uint, pname: SamplerParameterI, params: *mut c_int) {
        unsafe {self.fns.GetSamplerParameterIiv(sampler, pname, params)}
    }
    pub fn glGetSamplerParameterIuiv(&self, sampler: c_uint, pname: SamplerParameterI, params: *mut c_uint) {
        unsafe {self.fns.GetSamplerParameterIuiv(sampler, pname, params)}
    }
    pub fn glGetSamplerParameterfv(&self, sampler: c_uint, pname: SamplerParameterF, params: *mut c_float) {
        unsafe {self.fns.GetSamplerParameterfv(sampler, pname, params)}
    }
    pub fn glGetSamplerParameteriv(&self, sampler: c_uint, pname: SamplerParameterI, params: *mut c_int) {
        unsafe {self.fns.GetSamplerParameteriv(sampler, pname, params)}
    }
    pub fn glGetShaderInfoLog(&self, shader: c_uint, bufSize: c_int, length: *mut c_int, infoLog: *mut u8) {
        unsafe {self.fns.GetShaderInfoLog(shader, bufSize, length, infoLog)}
    }
    pub fn glGetShaderSource(&self, shader: c_uint, bufSize: c_int, length: *mut c_int, source: *mut u8) {
        unsafe {self.fns.GetShaderSource(shader, bufSize, length, source)}
    }
    pub fn glGetShaderiv(&self, shader: c_uint, pname: ShaderParameterName, params: *mut c_int) {
        unsafe {self.fns.GetShaderiv(shader, pname, params)}
    }
    pub fn glGetString(&self, name: StringName) -> *const u8{
        unsafe {self.fns.GetString(name)}
    }
    pub fn glGetStringi(&self, name: StringName, index: c_uint) -> *const u8{
        unsafe {self.fns.GetStringi(name, index)}
    }
    pub fn glGetSynciv(&self, sync: GLsync, pname: SyncParameterName, count: c_int, length: *mut c_int, values: *mut c_int) {
        unsafe {self.fns.GetSynciv(sync, pname, count, length, values)}
    }
    pub fn glGetTexImage(&self, target: TextureTarget, level: c_int, format: PixelFormat, type_: PixelType, pixels: *mut c_void) {
        unsafe {self.fns.GetTexImage(target, level, format, type_, pixels)}
    }
    pub fn glGetTexLevelParameterfv(&self, target: TextureTarget, level: c_int, pname: GetTextureParameter, params: *mut c_float) {
        unsafe {self.fns.GetTexLevelParameterfv(target, level, pname, params)}
    }
    pub fn glGetTexLevelParameteriv(&self, target: TextureTarget, level: c_int, pname: GetTextureParameter, params: *mut c_int) {
        unsafe {self.fns.GetTexLevelParameteriv(target, level, pname, params)}
    }
    pub fn glGetTexParameterIiv(&self, target: TextureTarget, pname: GetTextureParameter, params: *mut c_int) {
        unsafe {self.fns.GetTexParameterIiv(target, pname, params)}
    }
    pub fn glGetTexParameterIuiv(&self, target: TextureTarget, pname: GetTextureParameter, params: *mut c_uint) {
        unsafe {self.fns.GetTexParameterIuiv(target, pname, params)}
    }
    pub fn glGetTexParameterfv(&self, target: TextureTarget, pname: GetTextureParameter, params: *mut c_float) {
        unsafe {self.fns.GetTexParameterfv(target, pname, params)}
    }
    pub fn glGetTexParameteriv(&self, target: TextureTarget, pname: GetTextureParameter, params: *mut c_int) {
        unsafe {self.fns.GetTexParameteriv(target, pname, params)}
    }
    pub fn glGetTransformFeedbackVarying(&self, program: c_uint, index: c_uint, bufSize: c_int, length: *mut c_int, size: *mut c_int, type_: *mut AttributeType, name: *mut u8) {
        unsafe {self.fns.GetTransformFeedbackVarying(program, index, bufSize, length, size, type_, name)}
    }
    pub fn glGetUniformBlockIndex(&self, program: c_uint, uniformBlockName: *const u8) -> c_uint{
        unsafe {self.fns.GetUniformBlockIndex(program, uniformBlockName)}
    }
    pub fn glGetUniformIndices(&self, program: c_uint, uniformCount: c_int, uniformNames: *const *const u8, uniformIndices: *mut c_uint) {
        unsafe {self.fns.GetUniformIndices(program, uniformCount, uniformNames, uniformIndices)}
    }
    pub fn glGetUniformLocation(&self, program: c_uint, name: *const u8) -> c_int{
        unsafe {self.fns.GetUniformLocation(program, name)}
    }
    pub fn glGetUniformfv(&self, program: c_uint, location: c_int, params: *mut c_float) {
        unsafe {self.fns.GetUniformfv(program, location, params)}
    }
    pub fn glGetUniformiv(&self, program: c_uint, location: c_int, params: *mut c_int) {
        unsafe {self.fns.GetUniformiv(program, location, params)}
    }
    pub fn glGetUniformuiv(&self, program: c_uint, location: c_int, params: *mut c_uint) {
        unsafe {self.fns.GetUniformuiv(program, location, params)}
    }
    pub fn glGetVertexAttribIiv(&self, index: c_uint, pname: VertexAttribEnum, params: *mut c_int) {
        unsafe {self.fns.GetVertexAttribIiv(index, pname, params)}
    }
    pub fn glGetVertexAttribIuiv(&self, index: c_uint, pname: VertexAttribEnum, params: *mut c_uint) {
        unsafe {self.fns.GetVertexAttribIuiv(index, pname, params)}
    }
    pub fn glGetVertexAttribPointerv(&self, index: c_uint, pname: VertexAttribPointerPropertyARB, pointer: *mut *mut c_void) {
        unsafe {self.fns.GetVertexAttribPointerv(index, pname, pointer)}
    }
    pub fn glGetVertexAttribdv(&self, index: c_uint, pname: VertexAttribPropertyARB, params: *mut [ c_double; 4 ]) {
        unsafe {self.fns.GetVertexAttribdv(index, pname, params)}
    }
    pub fn glGetVertexAttribfv(&self, index: c_uint, pname: VertexAttribPropertyARB, params: *mut [ c_float; 4 ]) {
        unsafe {self.fns.GetVertexAttribfv(index, pname, params)}
    }
    pub fn glGetVertexAttribiv(&self, index: c_uint, pname: VertexAttribPropertyARB, params: *mut [ c_int; 4 ]) {
        unsafe {self.fns.GetVertexAttribiv(index, pname, params)}
    }
    pub fn glHint(&self, target: HintTarget, mode: HintMode) {
        unsafe {self.fns.Hint(target, mode)}
    }
    pub fn glIsBuffer(&self, buffer: c_uint) -> c_uchar{
        unsafe {self.fns.IsBuffer(buffer)}
    }
    pub fn glIsEnabled(&self, cap: EnableCap) -> c_uchar{
        unsafe {self.fns.IsEnabled(cap)}
    }
    pub fn glIsEnabledi(&self, target: EnableCap, index: c_uint) -> c_uchar{
        unsafe {self.fns.IsEnabledi(target, index)}
    }
    pub fn glIsFramebuffer(&self, framebuffer: c_uint) -> c_uchar{
        unsafe {self.fns.IsFramebuffer(framebuffer)}
    }
    pub fn glIsProgram(&self, program: c_uint) -> c_uchar{
        unsafe {self.fns.IsProgram(program)}
    }
    pub fn glIsQuery(&self, id: c_uint) -> c_uchar{
        unsafe {self.fns.IsQuery(id)}
    }
    pub fn glIsRenderbuffer(&self, renderbuffer: c_uint) -> c_uchar{
        unsafe {self.fns.IsRenderbuffer(renderbuffer)}
    }
    pub fn glIsSampler(&self, sampler: c_uint) -> c_uchar{
        unsafe {self.fns.IsSampler(sampler)}
    }
    pub fn glIsShader(&self, shader: c_uint) -> c_uchar{
        unsafe {self.fns.IsShader(shader)}
    }
    pub fn glIsSync(&self, sync: GLsync) -> c_uchar{
        unsafe {self.fns.IsSync(sync)}
    }
    pub fn glIsTexture(&self, texture: c_uint) -> c_uchar{
        unsafe {self.fns.IsTexture(texture)}
    }
    pub fn glIsVertexArray(&self, array: c_uint) -> c_uchar{
        unsafe {self.fns.IsVertexArray(array)}
    }
    pub fn glLineWidth(&self, width: c_float) {
        unsafe {self.fns.LineWidth(width)}
    }
    pub fn glLinkProgram(&self, program: c_uint) {
        self.fns.LinkProgram(program)
    }
    pub fn glLogicOp(&self, opcode: LogicOp) {
        unsafe {self.fns.LogicOp(opcode)}
    }
    pub fn glMapBuffer(&self, target: BufferTargetARB, access: BufferAccessARB) -> *mut c_void{
        unsafe {self.fns.MapBuffer(target, access)}
    }
    pub fn glMapBufferRange(&self, target: BufferTargetARB, offset: isize, length: isize, access: GLbitfield) -> *mut c_void{
        unsafe {self.fns.MapBufferRange(target, offset, length, access)}
    }
    pub fn glMultiDrawArrays(&self, mode: PrimitiveType, first: *const c_int, count: *const c_int, drawcount: c_int) {
        unsafe {self.fns.MultiDrawArrays(mode, first, count, drawcount)}
    }
    pub fn glMultiDrawElements(&self, mode: PrimitiveType, count: *const c_int, type_: DrawElementsType, indices: *const *const c_void, drawcount: c_int) {
        unsafe {self.fns.MultiDrawElements(mode, count, type_, indices, drawcount)}
    }
    pub fn glMultiDrawElementsBaseVertex(&self, mode: PrimitiveType, count: *const c_int, type_: DrawElementsType, indices: *const *const c_void, drawcount: c_int, basevertex: *const c_int) {
        unsafe {self.fns.MultiDrawElementsBaseVertex(mode, count, type_, indices, drawcount, basevertex)}
    }
    pub fn glPixelStoref(&self, pname: PixelStoreParameter, param: c_float) {
        unsafe {self.fns.PixelStoref(pname, param)}
    }
    pub fn glPixelStorei(&self, pname: PixelStoreParameter, param: c_int) {
        unsafe {self.fns.PixelStorei(pname, param)}
    }
    pub fn glPointParameterf(&self, pname: PointParameterNameARB, param: c_float) {
        unsafe {self.fns.PointParameterf(pname, param)}
    }
    pub fn glPointParameterfv(&self, pname: PointParameterNameARB, params: *const c_float) {
        unsafe {self.fns.PointParameterfv(pname, params)}
    }
    pub fn glPointParameteri(&self, pname: PointParameterNameARB, param: c_int) {
        unsafe {self.fns.PointParameteri(pname, param)}
    }
    pub fn glPointParameteriv(&self, pname: PointParameterNameARB, params: *const c_int) {
        unsafe {self.fns.PointParameteriv(pname, params)}
    }
    pub fn glPointSize(&self, size: c_float) {
        self.fns.PointSize(size)
    }
    pub fn glPolygonMode(&self, face: MaterialFace, mode: PolygonMode) {
        unsafe {self.fns.PolygonMode(face, mode)}
    }
    pub fn glPolygonOffset(&self, factor: c_float, units: c_float) {
        unsafe {self.fns.PolygonOffset(factor, units)}
    }
    pub fn glPrimitiveRestartIndex(&self, index: c_uint) {
        unsafe {self.fns.PrimitiveRestartIndex(index)}
    }
    pub fn glProvokingVertex(&self, mode: VertexProvokingMode) {
        unsafe {self.fns.ProvokingVertex(mode)}
    }
    pub fn glQueryCounter(&self, id: c_uint, target: QueryCounterTarget) {
        unsafe {self.fns.QueryCounter(id, target)}
    }
    pub fn glReadBuffer(&self, src: ReadBufferMode) {
        unsafe {self.fns.ReadBuffer(src)}
    }
    pub fn glReadPixels(&self, x: c_int, y: c_int, width: c_int, height: c_int, format: PixelFormat, type_: PixelType, pixels: *mut c_void) {
        unsafe {self.fns.ReadPixels(x, y, width, height, format, type_, pixels)}
    }
    pub fn glRenderbufferStorage(&self, target: RenderbufferTarget, internalformat: InternalFormat, width: c_int, height: c_int) {
        unsafe {self.fns.RenderbufferStorage(target, internalformat, width, height)}
    }
    pub fn glRenderbufferStorageMultisample(&self, target: RenderbufferTarget, samples: c_int, internalformat: InternalFormat, width: c_int, height: c_int) {
        unsafe {self.fns.RenderbufferStorageMultisample(target, samples, internalformat, width, height)}
    }
    pub fn glSampleCoverage(&self, value: c_float, invert: c_uchar) {
        unsafe {self.fns.SampleCoverage(value, invert)}
    }
    pub fn glSampleMaski(&self, maskNumber: c_uint, mask: GLbitfield) {
        unsafe {self.fns.SampleMaski(maskNumber, mask)}
    }
    pub fn glSamplerParameterIiv(&self, sampler: c_uint, pname: SamplerParameterI, param: *const c_int) {
        unsafe {self.fns.SamplerParameterIiv(sampler, pname, param)}
    }
    pub fn glSamplerParameterIuiv(&self, sampler: c_uint, pname: SamplerParameterI, param: *const c_uint) {
        unsafe {self.fns.SamplerParameterIuiv(sampler, pname, param)}
    }
    pub fn glSamplerParameterf(&self, sampler: c_uint, pname: SamplerParameterF, param: c_float) {
        unsafe {self.fns.SamplerParameterf(sampler, pname, param)}
    }
    pub fn glSamplerParameterfv(&self, sampler: c_uint, pname: SamplerParameterF, param: *const c_float) {
        unsafe {self.fns.SamplerParameterfv(sampler, pname, param)}
    }
    pub fn glSamplerParameteri(&self, sampler: c_uint, pname: SamplerParameterI, param: c_int) {
        unsafe {self.fns.SamplerParameteri(sampler, pname, param)}
    }
    pub fn glSamplerParameteriv(&self, sampler: c_uint, pname: SamplerParameterI, param: *const c_int) {
        unsafe {self.fns.SamplerParameteriv(sampler, pname, param)}
    }
    pub fn glScissor(&self, x: c_int, y: c_int, width: c_int, height: c_int) {
        unsafe {self.fns.Scissor(x, y, width, height)}
    }
    pub fn glShaderSource(&self, shader: c_uint, count: c_int, string: *const *const u8, length: *const c_int) {
        unsafe {self.fns.ShaderSource(shader, count, string, length)}
    }
    pub fn glStencilFunc(&self, func: StencilFunction, ref_: c_int, mask: c_uint) {
        unsafe {self.fns.StencilFunc(func, ref_, mask)}
    }
    pub fn glStencilFuncSeparate(&self, face: StencilFaceDirection, func: StencilFunction, ref_: c_int, mask: c_uint) {
        unsafe {self.fns.StencilFuncSeparate(face, func, ref_, mask)}
    }
    pub fn glStencilMask(&self, mask: c_uint) {
        unsafe {self.fns.StencilMask(mask)}
    }
    pub fn glStencilMaskSeparate(&self, face: StencilFaceDirection, mask: c_uint) {
        unsafe {self.fns.StencilMaskSeparate(face, mask)}
    }
    pub fn glStencilOp(&self, fail: StencilOp, zfail: StencilOp, zpass: StencilOp) {
        unsafe {self.fns.StencilOp(fail, zfail, zpass)}
    }
    pub fn glStencilOpSeparate(&self, face: StencilFaceDirection, sfail: StencilOp, dpfail: StencilOp, dppass: StencilOp) {
        unsafe {self.fns.StencilOpSeparate(face, sfail, dpfail, dppass)}
    }
    pub fn glTexBuffer(&self, target: TextureTarget, internalformat: InternalFormat, buffer: c_uint) {
        unsafe {self.fns.TexBuffer(target, internalformat, buffer)}
    }
    pub fn glTexImage1D(&self, target: TextureTarget, level: c_int, internalformat: c_int, width: c_int, border: c_int, format: PixelFormat, type_: PixelType, pixels: *const c_void) {
        unsafe {self.fns.TexImage1D(target, level, internalformat, width, border, format, type_, pixels)}
    }
    pub fn glTexImage2D(&self, target: TextureTarget, level: c_int, internalformat: c_int, width: c_int, height: c_int, border: c_int, format: PixelFormat, type_: PixelType, pixels: *const c_void) {
        unsafe {self.fns.TexImage2D(target, level, internalformat, width, height, border, format, type_, pixels)}
    }
    pub fn glTexImage2DMultisample(&self, target: TextureTarget, samples: c_int, internalformat: InternalFormat, width: c_int, height: c_int, fixedsamplelocations: c_uchar) {
        unsafe {self.fns.TexImage2DMultisample(target, samples, internalformat, width, height, fixedsamplelocations)}
    }
    pub fn glTexImage3D(&self, target: TextureTarget, level: c_int, internalformat: c_int, width: c_int, height: c_int, depth: c_int, border: c_int, format: PixelFormat, type_: PixelType, pixels: *const c_void) {
        unsafe {self.fns.TexImage3D(target, level, internalformat, width, height, depth, border, format, type_, pixels)}
    }
    pub fn glTexImage3DMultisample(&self, target: TextureTarget, samples: c_int, internalformat: InternalFormat, width: c_int, height: c_int, depth: c_int, fixedsamplelocations: c_uchar) {
        unsafe {self.fns.TexImage3DMultisample(target, samples, internalformat, width, height, depth, fixedsamplelocations)}
    }
    pub fn glTexParameterIiv(&self, target: TextureTarget, pname: TextureParameterName, params: *const c_int) {
        unsafe {self.fns.TexParameterIiv(target, pname, params)}
    }
    pub fn glTexParameterIuiv(&self, target: TextureTarget, pname: TextureParameterName, params: *const c_uint) {
        unsafe {self.fns.TexParameterIuiv(target, pname, params)}
    }
    pub fn glTexParameterf(&self, target: TextureTarget, pname: TextureParameterName, param: c_float) {
        unsafe {self.fns.TexParameterf(target, pname, param)}
    }
    pub fn glTexParameterfv(&self, target: TextureTarget, pname: TextureParameterName, params: *const c_float) {
        unsafe {self.fns.TexParameterfv(target, pname, params)}
    }
    pub fn glTexParameteri(&self, target: TextureTarget, pname: TextureParameterName, param: c_int) {
        unsafe {self.fns.TexParameteri(target, pname, param)}
    }
    pub fn glTexParameteriv(&self, target: TextureTarget, pname: TextureParameterName, params: *const c_int) {
        unsafe {self.fns.TexParameteriv(target, pname, params)}
    }
    pub fn glTexSubImage1D(&self, target: TextureTarget, level: c_int, xoffset: c_int, width: c_int, format: PixelFormat, type_: PixelType, pixels: *const c_void) {
        unsafe {self.fns.TexSubImage1D(target, level, xoffset, width, format, type_, pixels)}
    }
    pub fn glTexSubImage2D(&self, target: TextureTarget, level: c_int, xoffset: c_int, yoffset: c_int, width: c_int, height: c_int, format: PixelFormat, type_: PixelType, pixels: *const c_void) {
        unsafe {self.fns.TexSubImage2D(target, level, xoffset, yoffset, width, height, format, type_, pixels)}
    }
    pub fn glTexSubImage3D(&self, target: TextureTarget, level: c_int, xoffset: c_int, yoffset: c_int, zoffset: c_int, width: c_int, height: c_int, depth: c_int, format: PixelFormat, type_: PixelType, pixels: *const c_void) {
        unsafe {self.fns.TexSubImage3D(target, level, xoffset, yoffset, zoffset, width, height, depth, format, type_, pixels)}
    }
    pub fn glTransformFeedbackVaryings(&self, program: c_uint, count: c_int, varyings: *const *const u8, bufferMode: TransformFeedbackBufferMode) {
        unsafe {self.fns.TransformFeedbackVaryings(program, count, varyings, bufferMode)}
    }
    pub fn glUniform1f(&self, location: c_int, v0: c_float) {
        unsafe {self.fns.Uniform1f(location, v0)}
    }
    pub fn glUniform1fv(&self, location: c_int, count: c_int, value: *const c_float) {
        unsafe {self.fns.Uniform1fv(location, count, value)}
    }
    pub fn glUniform1i(&self, location: c_int, v0: c_int) {
        unsafe {self.fns.Uniform1i(location, v0)}
    }
    pub fn glUniform1iv(&self, location: c_int, count: c_int, value: *const c_int) {
        unsafe {self.fns.Uniform1iv(location, count, value)}
    }
    pub fn glUniform1ui(&self, location: c_int, v0: c_uint) {
        unsafe {self.fns.Uniform1ui(location, v0)}
    }
    pub fn glUniform1uiv(&self, location: c_int, count: c_int, value: *const c_uint) {
        unsafe {self.fns.Uniform1uiv(location, count, value)}
    }
    pub fn glUniform2f(&self, location: c_int, v0: c_float, v1: c_float) {
        unsafe {self.fns.Uniform2f(location, v0, v1)}
    }
    pub fn glUniform2fv(&self, location: c_int, count: c_int, value: *const c_float) {
        unsafe {self.fns.Uniform2fv(location, count, value)}
    }
    pub fn glUniform2i(&self, location: c_int, v0: c_int, v1: c_int) {
        unsafe {self.fns.Uniform2i(location, v0, v1)}
    }
    pub fn glUniform2iv(&self, location: c_int, count: c_int, value: *const c_int) {
        unsafe {self.fns.Uniform2iv(location, count, value)}
    }
    pub fn glUniform2ui(&self, location: c_int, v0: c_uint, v1: c_uint) {
        unsafe {self.fns.Uniform2ui(location, v0, v1)}
    }
    pub fn glUniform2uiv(&self, location: c_int, count: c_int, value: *const c_uint) {
        unsafe {self.fns.Uniform2uiv(location, count, value)}
    }
    pub fn glUniform3f(&self, location: c_int, v0: c_float, v1: c_float, v2: c_float) {
        unsafe {self.fns.Uniform3f(location, v0, v1, v2)}
    }
    pub fn glUniform3fv(&self, location: c_int, count: c_int, value: *const c_float) {
        unsafe {self.fns.Uniform3fv(location, count, value)}
    }
    pub fn glUniform3i(&self, location: c_int, v0: c_int, v1: c_int, v2: c_int) {
        unsafe {self.fns.Uniform3i(location, v0, v1, v2)}
    }
    pub fn glUniform3iv(&self, location: c_int, count: c_int, value: *const c_int) {
        unsafe {self.fns.Uniform3iv(location, count, value)}
    }
    pub fn glUniform3ui(&self, location: c_int, v0: c_uint, v1: c_uint, v2: c_uint) {
        unsafe {self.fns.Uniform3ui(location, v0, v1, v2)}
    }
    pub fn glUniform3uiv(&self, location: c_int, count: c_int, value: *const c_uint) {
        unsafe {self.fns.Uniform3uiv(location, count, value)}
    }
    pub fn glUniform4f(&self, location: c_int, v0: c_float, v1: c_float, v2: c_float, v3: c_float) {
        unsafe {self.fns.Uniform4f(location, v0, v1, v2, v3)}
    }
    pub fn glUniform4fv(&self, location: c_int, count: c_int, value: *const c_float) {
        unsafe {self.fns.Uniform4fv(location, count, value)}
    }
    pub fn glUniform4i(&self, location: c_int, v0: c_int, v1: c_int, v2: c_int, v3: c_int) {
        unsafe {self.fns.Uniform4i(location, v0, v1, v2, v3)}
    }
    pub fn glUniform4iv(&self, location: c_int, count: c_int, value: *const c_int) {
        unsafe {self.fns.Uniform4iv(location, count, value)}
    }
    pub fn glUniform4ui(&self, location: c_int, v0: c_uint, v1: c_uint, v2: c_uint, v3: c_uint) {
        unsafe {self.fns.Uniform4ui(location, v0, v1, v2, v3)}
    }
    pub fn glUniform4uiv(&self, location: c_int, count: c_int, value: *const c_uint) {
        unsafe {self.fns.Uniform4uiv(location, count, value)}
    }
    pub fn glUniformBlockBinding(&self, program: c_uint, uniformBlockIndex: c_uint, uniformBlockBinding: c_uint) {
        unsafe {self.fns.UniformBlockBinding(program, uniformBlockIndex, uniformBlockBinding)}
    }
    pub fn glUniformMatrix2fv(&self, location: c_int, count: c_int, transpose: c_uchar, value: *const c_float) {
        unsafe {self.fns.UniformMatrix2fv(location, count, transpose, value)}
    }
    pub fn glUniformMatrix2x3fv(&self, location: c_int, count: c_int, transpose: c_uchar, value: *const c_float) {
        unsafe {self.fns.UniformMatrix2x3fv(location, count, transpose, value)}
    }
    pub fn glUniformMatrix2x4fv(&self, location: c_int, count: c_int, transpose: c_uchar, value: *const c_float) {
        unsafe {self.fns.UniformMatrix2x4fv(location, count, transpose, value)}
    }
    pub fn glUniformMatrix3fv(&self, location: c_int, count: c_int, transpose: c_uchar, value: *const c_float) {
        unsafe {self.fns.UniformMatrix3fv(location, count, transpose, value)}
    }
    pub fn glUniformMatrix3x2fv(&self, location: c_int, count: c_int, transpose: c_uchar, value: *const c_float) {
        unsafe {self.fns.UniformMatrix3x2fv(location, count, transpose, value)}
    }
    pub fn glUniformMatrix3x4fv(&self, location: c_int, count: c_int, transpose: c_uchar, value: *const c_float) {
        unsafe {self.fns.UniformMatrix3x4fv(location, count, transpose, value)}
    }
    pub fn glUniformMatrix4fv(&self, location: c_int, count: c_int, transpose: c_uchar, value: *const c_float) {
        unsafe {self.fns.UniformMatrix4fv(location, count, transpose, value)}
    }
    pub fn glUniformMatrix4x2fv(&self, location: c_int, count: c_int, transpose: c_uchar, value: *const c_float) {
        unsafe {self.fns.UniformMatrix4x2fv(location, count, transpose, value)}
    }
    pub fn glUniformMatrix4x3fv(&self, location: c_int, count: c_int, transpose: c_uchar, value: *const c_float) {
        unsafe {self.fns.UniformMatrix4x3fv(location, count, transpose, value)}
    }
    pub fn glUnmapBuffer(&self, target: BufferTargetARB) -> c_uchar{
        unsafe {self.fns.UnmapBuffer(target)}
    }
    pub fn glUseProgram(&self, program: c_uint) {
        self.fns.UseProgram(program)
    }
    pub fn glValidateProgram(&self, program: c_uint) {
        unsafe {self.fns.ValidateProgram(program)}
    }
    pub fn glVertexAttrib1d(&self, index: c_uint, x: c_double) {
        unsafe {self.fns.VertexAttrib1d(index, x)}
    }
    pub fn glVertexAttrib1dv(&self, index: c_uint, v: *const c_double) {
        unsafe {self.fns.VertexAttrib1dv(index, v)}
    }
    pub fn glVertexAttrib1f(&self, index: c_uint, x: c_float) {
        unsafe {self.fns.VertexAttrib1f(index, x)}
    }
    pub fn glVertexAttrib1fv(&self, index: c_uint, v: *const c_float) {
        unsafe {self.fns.VertexAttrib1fv(index, v)}
    }
    pub fn glVertexAttrib1s(&self, index: c_uint, x: i16) {
        unsafe {self.fns.VertexAttrib1s(index, x)}
    }
    pub fn glVertexAttrib1sv(&self, index: c_uint, v: *const i16) {
        unsafe {self.fns.VertexAttrib1sv(index, v)}
    }
    pub fn glVertexAttrib2d(&self, index: c_uint, x: c_double, y: c_double) {
        unsafe {self.fns.VertexAttrib2d(index, x, y)}
    }
    pub fn glVertexAttrib2dv(&self, index: c_uint, v: *const [ c_double; 2 ]) {
        unsafe {self.fns.VertexAttrib2dv(index, v)}
    }
    pub fn glVertexAttrib2f(&self, index: c_uint, x: c_float, y: c_float) {
        unsafe {self.fns.VertexAttrib2f(index, x, y)}
    }
    pub fn glVertexAttrib2fv(&self, index: c_uint, v: *const [ c_float; 2 ]) {
        unsafe {self.fns.VertexAttrib2fv(index, v)}
    }
    pub fn glVertexAttrib2s(&self, index: c_uint, x: i16, y: i16) {
        unsafe {self.fns.VertexAttrib2s(index, x, y)}
    }
    pub fn glVertexAttrib2sv(&self, index: c_uint, v: *const [ i16; 2 ]) {
        unsafe {self.fns.VertexAttrib2sv(index, v)}
    }
    pub fn glVertexAttrib3d(&self, index: c_uint, x: c_double, y: c_double, z: c_double) {
        unsafe {self.fns.VertexAttrib3d(index, x, y, z)}
    }
    pub fn glVertexAttrib3dv(&self, index: c_uint, v: *const [ c_double; 3 ]) {
        unsafe {self.fns.VertexAttrib3dv(index, v)}
    }
    pub fn glVertexAttrib3f(&self, index: c_uint, x: c_float, y: c_float, z: c_float) {
        unsafe {self.fns.VertexAttrib3f(index, x, y, z)}
    }
    pub fn glVertexAttrib3fv(&self, index: c_uint, v: *const [ c_float; 3 ]) {
        unsafe {self.fns.VertexAttrib3fv(index, v)}
    }
    pub fn glVertexAttrib3s(&self, index: c_uint, x: i16, y: i16, z: i16) {
        unsafe {self.fns.VertexAttrib3s(index, x, y, z)}
    }
    pub fn glVertexAttrib3sv(&self, index: c_uint, v: *const [ i16; 3 ]) {
        unsafe {self.fns.VertexAttrib3sv(index, v)}
    }
    pub fn glVertexAttrib4Nbv(&self, index: c_uint, v: *const [ i8; 4 ]) {
        unsafe {self.fns.VertexAttrib4Nbv(index, v)}
    }
    pub fn glVertexAttrib4Niv(&self, index: c_uint, v: *const [ c_int; 4 ]) {
        unsafe {self.fns.VertexAttrib4Niv(index, v)}
    }
    pub fn glVertexAttrib4Nsv(&self, index: c_uint, v: *const [ i16; 4 ]) {
        unsafe {self.fns.VertexAttrib4Nsv(index, v)}
    }
    pub fn glVertexAttrib4Nub(&self, index: c_uint, x: u8, y: u8, z: u8, w: u8) {
        unsafe {self.fns.VertexAttrib4Nub(index, x, y, z, w)}
    }
    pub fn glVertexAttrib4Nubv(&self, index: c_uint, v: *const [ u8; 4 ]) {
        unsafe {self.fns.VertexAttrib4Nubv(index, v)}
    }
    pub fn glVertexAttrib4Nuiv(&self, index: c_uint, v: *const [ c_uint; 4 ]) {
        unsafe {self.fns.VertexAttrib4Nuiv(index, v)}
    }
    pub fn glVertexAttrib4Nusv(&self, index: c_uint, v: *const [ u16; 4 ]) {
        unsafe {self.fns.VertexAttrib4Nusv(index, v)}
    }
    pub fn glVertexAttrib4bv(&self, index: c_uint, v: *const [ i8; 4 ]) {
        unsafe {self.fns.VertexAttrib4bv(index, v)}
    }
    pub fn glVertexAttrib4d(&self, index: c_uint, x: c_double, y: c_double, z: c_double, w: c_double) {
        unsafe {self.fns.VertexAttrib4d(index, x, y, z, w)}
    }
    pub fn glVertexAttrib4dv(&self, index: c_uint, v: *const [ c_double; 4 ]) {
        unsafe {self.fns.VertexAttrib4dv(index, v)}
    }
    pub fn glVertexAttrib4f(&self, index: c_uint, x: c_float, y: c_float, z: c_float, w: c_float) {
        unsafe {self.fns.VertexAttrib4f(index, x, y, z, w)}
    }
    pub fn glVertexAttrib4fv(&self, index: c_uint, v: *const [ c_float; 4 ]) {
        unsafe {self.fns.VertexAttrib4fv(index, v)}
    }
    pub fn glVertexAttrib4iv(&self, index: c_uint, v: *const [ c_int; 4 ]) {
        unsafe {self.fns.VertexAttrib4iv(index, v)}
    }
    pub fn glVertexAttrib4s(&self, index: c_uint, x: i16, y: i16, z: i16, w: i16) {
        unsafe {self.fns.VertexAttrib4s(index, x, y, z, w)}
    }
    pub fn glVertexAttrib4sv(&self, index: c_uint, v: *const [ i16; 4 ]) {
        unsafe {self.fns.VertexAttrib4sv(index, v)}
    }
    pub fn glVertexAttrib4ubv(&self, index: c_uint, v: *const [ u8; 4 ]) {
        unsafe {self.fns.VertexAttrib4ubv(index, v)}
    }
    pub fn glVertexAttrib4uiv(&self, index: c_uint, v: *const [ c_uint; 4 ]) {
        unsafe {self.fns.VertexAttrib4uiv(index, v)}
    }
    pub fn glVertexAttrib4usv(&self, index: c_uint, v: *const [ u16; 4 ]) {
        unsafe {self.fns.VertexAttrib4usv(index, v)}
    }
    pub fn glVertexAttribDivisor(&self, index: c_uint, divisor: c_uint) {
        unsafe {self.fns.VertexAttribDivisor(index, divisor)}
    }
    pub fn glVertexAttribI1i(&self, index: c_uint, x: c_int) {
        unsafe {self.fns.VertexAttribI1i(index, x)}
    }
    pub fn glVertexAttribI1iv(&self, index: c_uint, v: *const c_int) {
        unsafe {self.fns.VertexAttribI1iv(index, v)}
    }
    pub fn glVertexAttribI1ui(&self, index: c_uint, x: c_uint) {
        unsafe {self.fns.VertexAttribI1ui(index, x)}
    }
    pub fn glVertexAttribI1uiv(&self, index: c_uint, v: *const c_uint) {
        unsafe {self.fns.VertexAttribI1uiv(index, v)}
    }
    pub fn glVertexAttribI2i(&self, index: c_uint, x: c_int, y: c_int) {
        unsafe {self.fns.VertexAttribI2i(index, x, y)}
    }
    pub fn glVertexAttribI2iv(&self, index: c_uint, v: *const [ c_int; 2 ]) {
        unsafe {self.fns.VertexAttribI2iv(index, v)}
    }
    pub fn glVertexAttribI2ui(&self, index: c_uint, x: c_uint, y: c_uint) {
        unsafe {self.fns.VertexAttribI2ui(index, x, y)}
    }
    pub fn glVertexAttribI2uiv(&self, index: c_uint, v: *const [ c_uint; 2 ]) {
        unsafe {self.fns.VertexAttribI2uiv(index, v)}
    }
    pub fn glVertexAttribI3i(&self, index: c_uint, x: c_int, y: c_int, z: c_int) {
        unsafe {self.fns.VertexAttribI3i(index, x, y, z)}
    }
    pub fn glVertexAttribI3iv(&self, index: c_uint, v: *const [ c_int; 3 ]) {
        unsafe {self.fns.VertexAttribI3iv(index, v)}
    }
    pub fn glVertexAttribI3ui(&self, index: c_uint, x: c_uint, y: c_uint, z: c_uint) {
        unsafe {self.fns.VertexAttribI3ui(index, x, y, z)}
    }
    pub fn glVertexAttribI3uiv(&self, index: c_uint, v: *const [ c_uint; 3 ]) {
        unsafe {self.fns.VertexAttribI3uiv(index, v)}
    }
    pub fn glVertexAttribI4bv(&self, index: c_uint, v: *const [ i8; 4 ]) {
        unsafe {self.fns.VertexAttribI4bv(index, v)}
    }
    pub fn glVertexAttribI4i(&self, index: c_uint, x: c_int, y: c_int, z: c_int, w: c_int) {
        unsafe {self.fns.VertexAttribI4i(index, x, y, z, w)}
    }
    pub fn glVertexAttribI4iv(&self, index: c_uint, v: *const [ c_int; 4 ]) {
        unsafe {self.fns.VertexAttribI4iv(index, v)}
    }
    pub fn glVertexAttribI4sv(&self, index: c_uint, v: *const [ i16; 4 ]) {
        unsafe {self.fns.VertexAttribI4sv(index, v)}
    }
    pub fn glVertexAttribI4ubv(&self, index: c_uint, v: *const [ u8; 4 ]) {
        unsafe {self.fns.VertexAttribI4ubv(index, v)}
    }
    pub fn glVertexAttribI4ui(&self, index: c_uint, x: c_uint, y: c_uint, z: c_uint, w: c_uint) {
        unsafe {self.fns.VertexAttribI4ui(index, x, y, z, w)}
    }
    pub fn glVertexAttribI4uiv(&self, index: c_uint, v: *const [ c_uint; 4 ]) {
        unsafe {self.fns.VertexAttribI4uiv(index, v)}
    }
    pub fn glVertexAttribI4usv(&self, index: c_uint, v: *const [ u16; 4 ]) {
        unsafe {self.fns.VertexAttribI4usv(index, v)}
    }
    pub fn glVertexAttribIPointer(&self, index: c_uint, size: c_int, type_: VertexAttribIType, stride: c_int, pointer: *const c_void) {
        unsafe {self.fns.VertexAttribIPointer(index, size, type_, stride, pointer)}
    }
    pub fn glVertexAttribP1ui(&self, index: c_uint, type_: VertexAttribPointerType, normalized: c_uchar, value: c_uint) {
        unsafe {self.fns.VertexAttribP1ui(index, type_, normalized, value)}
    }
    pub fn glVertexAttribP1uiv(&self, index: c_uint, type_: VertexAttribPointerType, normalized: c_uchar, value: *const c_uint) {
        unsafe {self.fns.VertexAttribP1uiv(index, type_, normalized, value)}
    }
    pub fn glVertexAttribP2ui(&self, index: c_uint, type_: VertexAttribPointerType, normalized: c_uchar, value: c_uint) {
        unsafe {self.fns.VertexAttribP2ui(index, type_, normalized, value)}
    }
    pub fn glVertexAttribP2uiv(&self, index: c_uint, type_: VertexAttribPointerType, normalized: c_uchar, value: *const c_uint) {
        unsafe {self.fns.VertexAttribP2uiv(index, type_, normalized, value)}
    }
    pub fn glVertexAttribP3ui(&self, index: c_uint, type_: VertexAttribPointerType, normalized: c_uchar, value: c_uint) {
        unsafe {self.fns.VertexAttribP3ui(index, type_, normalized, value)}
    }
    pub fn glVertexAttribP3uiv(&self, index: c_uint, type_: VertexAttribPointerType, normalized: c_uchar, value: *const c_uint) {
        unsafe {self.fns.VertexAttribP3uiv(index, type_, normalized, value)}
    }
    pub fn glVertexAttribP4ui(&self, index: c_uint, type_: VertexAttribPointerType, normalized: c_uchar, value: c_uint) {
        unsafe {self.fns.VertexAttribP4ui(index, type_, normalized, value)}
    }
    pub fn glVertexAttribP4uiv(&self, index: c_uint, type_: VertexAttribPointerType, normalized: c_uchar, value: *const c_uint) {
        unsafe {self.fns.VertexAttribP4uiv(index, type_, normalized, value)}
    }
    pub fn glVertexAttribPointer(&self, index: c_uint, size: c_int, type_: VertexAttribPointerType, normalized: c_uchar, stride: c_int, pointer: *const c_void) {
        unsafe {self.fns.VertexAttribPointer(index, size, type_, normalized, stride, pointer)}
    }
    pub fn glViewport(&self, x: c_int, y: c_int, width: c_int, height: c_int) {
        unsafe {self.fns.Viewport(x, y, width, height)}
    }
    pub fn glWaitSync(&self, sync: GLsync, flags: GLbitfield, timeout: u64) {
        unsafe {self.fns.WaitSync(sync, flags, timeout)}
    }
    pub fn glDebugMessageCallback(&self, callback: GLDEBUGPROC, userParam: *const c_void) {
        unsafe {self.fns.DebugMessageCallback(callback, userParam)}
    }
    pub fn glDebugMessageControl(&self, source: DebugSource, type_: DebugType, severity: DebugSeverity, count: c_int, ids: *const c_uint, enabled: c_uchar) {
        unsafe {self.fns.DebugMessageControl(source, type_, severity, count, ids, enabled)}
    }
    pub fn glDebugMessageInsert(&self, source: DebugSource, type_: DebugType, id: c_uint, severity: DebugSeverity, length: c_int, buf: *const u8) {
        unsafe {self.fns.DebugMessageInsert(source, type_, id, severity, length, buf)}
    }
    pub fn glGetDebugMessageLog(&self, count: c_uint, bufSize: c_int, sources: *mut DebugSource, types: *mut DebugType, ids: *mut c_uint, severities: *mut DebugSeverity, lengths: *mut c_int, messageLog: *mut u8) -> c_uint{
        unsafe {self.fns.GetDebugMessageLog(count, bufSize, sources, types, ids, severities, lengths, messageLog)}
    }
    pub fn glGetObjectLabel(&self, identifier: ObjectIdentifier, name: c_uint, bufSize: c_int, length: *mut c_int, label: *mut u8) {
        unsafe {self.fns.GetObjectLabel(identifier, name, bufSize, length, label)}
    }
    pub fn glGetObjectPtrLabel(&self, ptr: *const c_void, bufSize: c_int, length: *mut c_int, label: *mut u8) {
        unsafe {self.fns.GetObjectPtrLabel(ptr, bufSize, length, label)}
    }
    pub fn glGetPointerv(&self, pname: GetPointervPName, params: *mut *mut c_void) {
        unsafe {self.fns.GetPointerv(pname, params)}
    }
    pub fn glObjectLabel(&self, identifier: ObjectIdentifier, name: c_uint, length: c_int, label: *const u8) {
        unsafe {self.fns.ObjectLabel(identifier, name, length, label)}
    }
    pub fn glObjectPtrLabel(&self, ptr: *const c_void, length: c_int, label: *const u8) {
        unsafe {self.fns.ObjectPtrLabel(ptr, length, label)}
    }
    pub fn glPopDebugGroup(&self) {
        unsafe {self.fns.PopDebugGroup()}
    }
    pub fn glPushDebugGroup(&self, source: DebugSource, id: c_uint, length: c_int, message: *const u8) {
        unsafe {self.fns.PushDebugGroup(source, id, length, message)}
    }
} 