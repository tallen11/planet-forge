use renderer::integrator::Integrator;

pub struct Renderer<'integrator> {
    integrator: &'integrator Integrator,
}

impl<'integrator> Renderer<'integrator> {
    pub fn new(integrator: &'integrator Integrator) -> Renderer<'integrator> {
        Renderer {
            integrator: integrator,
        }
    }
}
