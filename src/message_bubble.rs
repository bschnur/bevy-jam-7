use bevy_vector_shapes::prelude::*;




// use bevy::{
//     prelude::*,
//     reflect::Reflect,
//     // render::render_resource::ShaderType, shader::ShaderRef
// };
// use bevy_vector_shapes::{
//     prelude::*,
//     render::ShapeComponent,
//     // shapes::rectangle::RectData,
// };

// // Enum to allow setting a rectangle's corner radii in multiple ways.
// pub enum CornerRadii {
//     Bespoke {
//         top_left: f32,
//         top_right: f32,
//         bot_left: f32,
//         bot_right: f32,
//     },
//     Uniform(f32),
// }

// /// Component containing the data for drawing a rectangle.
// #[derive(Component, Reflect)]
// pub struct BubbleComponent {
//     pub alignment: Alignment,

//     /// Size of the rectangle on the x and y axis.
//     pub size: Vec2,
//     /// Corner rounding radius for each corner in world units.
//     corner_radii: Vec4,
// }
// impl RectangleComponent {
//     pub fn set_corner_radii(&mut self, value: CornerRadii) {
//         match value {
//             CornerRadii::Bespoke{ top_left: tl, top_right: tr, bot_left: bl, bot_right: br } => {
//                 self.corner_radii = Vec4::new(tl, tr, bl, br);
//             },
//             CornerRadii::Uniform(value) => {
//                 self.corner_radii = Vec4::new(value, value, value, value);
//             },
//         }
//     }
//     pub fn corner_radii(&self) -> CornerRadii {
//         if self.corner_radii.x == self.corner_radii.y
//             && self.corner_radii.x == self.corner_radii.z
//             && self.corner_radii.x == self.corner_radii.w
//         {
//             CornerRadii::Uniform(self.corner_radii.x)
//         }
//         else {
//             CornerRadii::Bespoke {
//                 top_left: self.corner_radii.x,
//                 top_right: self.corner_radii.y,
//                 bot_left: self.corner_radii.z,
//                 bot_right: self.corner_radii.w
//             }
//         }
//     }
// }
// impl ShapeComponent for RectangleComponent {
//     type Data = RectData;

//     fn get_data(&self, tf: &GlobalTransform, fill: &ShapeFill) -> RectData {
//         let mut flags = Flags(0);
//         let thickness = match fill.ty {
//             FillType::Stroke(thickness, thickness_type) => {
//                 flags.set_thickness_type(thickness_type);
//                 flags.set_hollow(1);
//                 thickness
//             }
//             FillType::Fill => 1.0,
//         };
//         flags.set_alignment(self.alignment);

//         RectData {
//             transform: tf.to_matrix().to_cols_array_2d(),

//             color: fill.color.to_linear().to_f32_array(),
//             thickness,
//             flags: flags.0,

//             size: self.size.into(),
//             corner_radii: self.corner_radii.into(),
//         }
//     }
// }