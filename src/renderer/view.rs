use ::{Vector2};

pub trait View {
    /// Return the translation and scaling respectively
    fn get_transformation(&self) -> (Vector2, Vector2);


    /// Maps a point in the range [-1, 1] to the view's bounds
    fn ndc_to_world(&self, point: Vector2) -> Vector2 {
        let (translation, scale) = self.get_transformation();

        point / scale - translation
    }
}
