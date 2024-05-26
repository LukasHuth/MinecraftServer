/// An element of an end city
#[derive(PartialEq, Debug)]
pub struct EndCityElement {
    /// The bounding box of the element
    pub bounding_box: [i32;6],
    /// The distance to the next start of the end city
    pub generation_difference: i32,
    /// The id of the element
    ///
    /// # Note
    ///
    /// Always "ECP" for end cities
    pub id: String,
    /// The orientation of the element
    ///
    /// # Note
    ///
    /// Always 2 for end city elements
    pub orientation: i32,
    /// Unknown ussage
    ///
    /// # Note
    /// `second_floor`, `second_floor_2`, `second_roof`, `third_floor` and `third_floor_c` are
    /// always 0 everything else is 1
    pub ow: i32,
    /// The x position inside of the template
    pub template_x: i32,
    /// The y position inside of the template
    pub template_y: i32,
    /// The z position inside of the template
    pub template_z: i32,
    /// The roration of the element
    pub rot: EndCityRotation,
    /// The template name of the element
    pub template: String,
}
/// A list of all rotation possibilities of an end city piece
#[derive(PartialEq, Eq, Debug)]
pub enum EndCityRotation {
    /// No rotation
    None,
    /// 90 degree clockwise
    Clockwise90,
    /// 180 degree clockwise
    Clockwise180,
    /// 240 degree clockwise
    Counterclockwise90,
}
