pub struct GpuConfig {
    pub(crate) depth_test: bool,
}
impl Default for GpuConfig {
    fn default() -> Self {
        Self { depth_test: false }
    }
}
impl GpuConfig {
    #[inline(always)]
    pub fn with_depth_test(mut self) -> Self {
        self.depth_test = true;
        self
    }
    #[inline(always)]
    pub fn without_depth_test(mut self) -> Self {
        self.depth_test = false;
        self
    }
}
