use web_sys::WebGlRenderingContext;

use crate::display::gpu::RenderPipeline;

#[allow(dead_code)]

pub struct UniformDescriptor {
    name: &'static str,
    typ: UniformType,
}

#[derive(PartialEq, Eq)]
pub enum UniformType {
    Matrix3fv,
}
pub enum UniformValue<'a> {
    Matrix3fv(&'a [f32]),
}

impl UniformDescriptor {
    pub fn new(name: &'static str, typ: UniformType) -> Self {
        Self { name, typ }
    }
}

impl RenderPipeline {
    pub fn prepare_uniform(&self, gl: &WebGlRenderingContext, name: &str, value: &UniformValue) {
        let uloc = gl.get_uniform_location(self.program(), name);
        match value {
            UniformValue::Matrix3fv(data) => {
                gl.uniform_matrix3fv_with_f32_array(uloc.as_ref(), false, data);
            }
        }
    }
}

impl From<&UniformValue<'_>> for UniformType {
    fn from(val: &UniformValue) -> Self {
        match val {
            UniformValue::Matrix3fv(_) => Self::Matrix3fv,
        }
    }
}
