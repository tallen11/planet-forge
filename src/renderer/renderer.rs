use renderer::integrator::Integrator;

pub struct Renderer {
    integrator: Box<Integrator>,
}

impl Renderer {
    pub fn new(integrator: Box<Integrator>) -> Renderer {
        Renderer {
            integrator: integrator,
        }
    }
}
