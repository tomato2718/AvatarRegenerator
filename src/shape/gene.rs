#[derive(Clone, Copy)]
pub struct Gene {
    pub boundary: (u32, u32),
    pub max_width: u32,
    pub mex_height: u32,
    pub center: (u32, u32),
    pub width: u32,
    pub height: u32,
    pub color: [u8; 4],
}

impl Gene {
    pub fn mutate(&mut self) {
        self.center = (
            !self.center.0 % self.boundary.0,
            !self.center.1 % self.boundary.1,
        );
        self.width = (!self.width % self.max_width) + 1;
        self.height = (!self.height % self.mex_height) + 1;
        self.color = self.color.map(|c| !c);
    }
}
