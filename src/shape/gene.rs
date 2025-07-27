use rand::random;

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
        let seed = random::<u32>();
        self.center = (
            self.center.0.wrapping_add(seed & 0xff) % self.boundary.0,
            self.center.1.wrapping_add(seed & 0xff00) % self.boundary.1,
        );
        self.width = (self.width.wrapping_add(seed & 0xff0000) % self.max_width) + 1;
        self.height = (self.height.wrapping_add(seed & 0xff000000) % self.mex_height) + 1;
        self.color = self.color.map(|c| c.wrapping_add(random()));
    }
}
