use std::rc::Rc;
use web_sys::WebGlRenderingContext;

use crate::display::gpu::RenderPipeline;
use crate::Transform;

/// A list of valued uniforms to associate with primitives when drawn.
#[derive(Clone, PartialEq, Default)]
pub struct UniformList(Option<Rc<[UniformDescriptor]>>);

impl UniformList {
    pub fn new(uniforms: &[UniformDescriptor]) -> Self {
        Self(Some(uniforms.into()))
    }

    pub fn len(&self) -> usize {
        self.0.as_ref().map(|list| list.len()).unwrap_or(0)
    }
}

#[derive(Clone, PartialEq)]
pub struct UniformDescriptor {
    name: &'static str,
    value: UniformValue,
}

#[derive(Clone, PartialEq)]
pub enum UniformValue {
    Matrix3fv([f32; 9]),
    Vec2F32(f32, f32),
    F32(f32),
}

impl UniformDescriptor {
    pub fn new(name: &'static str, value: UniformValue) -> Self {
        Self { name, value }
    }
}

impl RenderPipeline {
    pub fn prepare_uniform(&self, gl: &WebGlRenderingContext, name: &str, value: &UniformValue) {
        let uloc = gl.get_uniform_location(self.program(), name);
        match value {
            UniformValue::Matrix3fv(data) => {
                gl.uniform_matrix3fv_with_f32_array(uloc.as_ref(), false, data);
            }
            UniformValue::F32(data) => gl.uniform1f(uloc.as_ref(), *data),
            UniformValue::Vec2F32(x, y) => gl.uniform2f(uloc.as_ref(), *x, *y),
        }
    }

    pub fn prepare_uniforms(&self, gl: &WebGlRenderingContext, uniforms: &crate::UniformList) {
        if let Some(inner) = uniforms.0.as_ref() {
            for UniformDescriptor { name, value } in inner.iter() {
                self.prepare_uniform(gl, name, &value);
            }
        }
    }
}

impl From<Transform> for UniformValue {
    fn from(value: Transform) -> Self {
        UniformValue::Matrix3fv(value.as_array())
    }
}

impl From<f32> for UniformValue {
    fn from(value: f32) -> Self {
        UniformValue::F32(value)
    }
}
