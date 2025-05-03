use domain::repositories::AbstractImageRepository;

pub struct ImageRepository;

impl ImageRepository {
    pub fn new() -> Self {
        Self
    }
}

impl AbstractImageRepository for ImageRepository {
    
}