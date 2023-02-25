use web_sys::{WebGlProgram, WebGlRenderingContext, WebGlShader};

use crate::display::gpu::{Gpu, UniformValue, VertexDescriptor};
use crate::{ErrorMessage, PaddleResult};

/// A handle to a registered shader program (essentially a pair of fragment + vertex shaders).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct RenderPipelineHandle {
    index: usize,
}

pub(crate) struct RenderPipelineContainer {
    pipelines: Vec<RenderPipeline>,
}

pub(crate) struct RenderPipeline {
    v_desc: VertexDescriptor,
    program: WebGlProgram,
}

impl Gpu {
    pub fn new_render_pipeline(
        &mut self,
        gl: &WebGlRenderingContext,
        vertex_shader: WebGlShader,
        fragment_shader: WebGlShader,
        vertex_descriptor: VertexDescriptor,
        uniform_values: &[(&'static str, UniformValue)],
    ) -> PaddleResult<RenderPipelineHandle> {
        let current_render_pipeline = self.active_render_pipeline;
        let program = link_program(&gl, &vertex_shader, &fragment_shader)?;
        let pipeline = RenderPipeline::new(program, vertex_descriptor.clone());
        for (name, v) in uniform_values {
            pipeline.prepare_uniform(gl, name, v);
        }
        let handle = self.render_pipelines.store(pipeline);

        self.gpu_buffers.add_vertex_buffer(gl, vertex_descriptor);

        // program "used" after linking
        self.active_render_pipeline = handle;
        self.use_render_pipeline(gl, current_render_pipeline);

        Ok(handle)
    }
    pub fn use_render_pipeline(&mut self, gl: &WebGlRenderingContext, rp: RenderPipelineHandle) {
        if self.active_render_pipeline != rp {
            gl.use_program(Some(&self.render_pipelines[rp].program));
            self.active_render_pipeline = rp;
        }
    }
}

impl RenderPipeline {
    pub(crate) fn new(program: WebGlProgram, v_desc: VertexDescriptor) -> Self {
        Self { v_desc, program }
    }

    pub fn vertex_descriptor(&self) -> &VertexDescriptor {
        &self.v_desc
    }
    pub fn program(&self) -> &WebGlProgram {
        &self.program
    }
}

impl RenderPipelineHandle {
    pub fn num(&self) -> usize {
        self.index
    }
}

impl Default for RenderPipelineHandle {
    fn default() -> Self {
        RenderPipelineHandle { index: 0 }
    }
}

impl RenderPipelineContainer {
    pub(crate) fn new() -> Self {
        Self {
            pipelines: Vec::new(),
        }
    }
    pub(crate) fn store(&mut self, p: RenderPipeline) -> RenderPipelineHandle {
        let handle = RenderPipelineHandle {
            index: self.pipelines.len(),
        };
        self.pipelines.push(p);
        handle
    }
}
impl std::ops::Index<RenderPipelineHandle> for RenderPipelineContainer {
    type Output = RenderPipeline;
    fn index(&self, handle: RenderPipelineHandle) -> &Self::Output {
        &self.pipelines[handle.index]
    }
}

impl RenderPipelineContainer {
    pub(super) fn drop_programs(&mut self, gl: &WebGlRenderingContext) {
        while let Some(mut rp) = self.pipelines.pop() {
            rp.custom_drop(gl);
        }
    }
}
impl RenderPipeline {
    pub(super) fn custom_drop(&mut self, gl: &WebGlRenderingContext) {
        gl.delete_program(Some(&self.program));
    }
}

fn link_program(
    context: &WebGlRenderingContext,
    vert_shader: &WebGlShader,
    frag_shader: &WebGlShader,
) -> PaddleResult<WebGlProgram> {
    let program = context
        .create_program()
        .ok_or_else(|| ErrorMessage::technical("Unable to create shader object".to_owned()))?;

    context.attach_shader(&program, vert_shader);
    context.attach_shader(&program, frag_shader);
    context.link_program(&program);
    context.use_program(Some(&program));

    if context
        .get_program_parameter(&program, WebGlRenderingContext::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(program)
    } else {
        Err(ErrorMessage::technical(
            context
                .get_program_info_log(&program)
                .unwrap_or_else(|| "Unknown error creating program object".to_owned()),
        ))
    }
}
