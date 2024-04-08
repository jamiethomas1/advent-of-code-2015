pub struct Present {
    pub length: u32,
    pub width: u32,
    pub height: u32,
    pub surface_area: u32,
    pub wrapping_paper: u32,
    pub smallest_perimeter: u32,
    pub ribbon_length: u32
}

impl Present {
    pub fn new(dimensions: &str) -> Present {
        let mut dimensions: Vec<u32> = dimensions
            .splitn(3, 'x')
            .map(|x| -> u32 {
                x.parse()
                    .expect("Expected an integer")
            })
            .collect();
        dimensions.sort();
        let length = dimensions[0];
        let width = dimensions[1];
        let height = dimensions[2];
        let surface_area = 2*length*width + 2*width*height + 2*height*length;
        let wrapping_paper = surface_area + length*width;
        let smallest_perimeter = 2*length + 2*width;
        let ribbon_length = smallest_perimeter + length*width*height;

        Present { length, width, height, surface_area, wrapping_paper, smallest_perimeter, ribbon_length }
    }
}
