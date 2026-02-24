extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(PieceBehavior)]
pub fn derive_behavior_fn(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let expanded = quote! {
        impl crate::piece::Piece for #name {
            fn positions(&self) -> Vec<Vec3> {
                self.positions.clone()
            }

            fn color(&self) -> Color {
                self.color.clone()
            }

            fn rotate(&mut self) {
                let mut new_positions = vec![];

                let s: f32 = std::f32::consts::FRAC_PI_2.sin();
                let c: f32 = std::f32::consts::FRAC_PI_2.cos();

                // We can unwrap as the first position exist
                let first_pos = self.positions.first().unwrap();
                let cx = first_pos.x;
                let cy = first_pos.y;

                for position in self.positions.iter() {
                    let trans_x = position.x - cx;
                    let trans_y = position.y - cy;

                    let xnew = trans_x * c - trans_y * s;
                    let ynew = trans_x * s + trans_y * c;

                    new_positions.push(Vec3::new(xnew + cx, ynew + cy, position.z));
                }

                self.positions = new_positions;
            }
            
            fn move_to_position(&mut self, target_pos: Vec3) {
                let first_pos = self.positions[0];
                let delta = target_pos - first_pos;
                for pos in self.positions.iter_mut() {
                    pos.x += delta.x;
                    pos.y += delta.y;
                    pos.z = target_pos.z;
                }
            }

            fn move_to_cursor(&mut self, cursor: &Res<crate::cursor::Cursor>) {
                let first_pos = self.positions.first_mut().unwrap();

                let delta_x = -first_pos.x + cursor.current_pos.x;
                let delta_y = -first_pos.y + cursor.current_pos.y;

                first_pos.x = cursor.current_pos.x;
                first_pos.y = cursor.current_pos.y;

                for pos in self.positions.iter_mut().skip(1) {
                    pos.x += delta_x;
                    pos.y += delta_y;
                }
            }

            fn snap(&mut self) {
                for position in self.positions.iter_mut() {
                    // Snap to the nearest SQUARE_WIDTH grid
                    position.x = (position.x / crate::SQUARE_WIDTH as f32).round() * crate::SQUARE_WIDTH as f32;
                    position.y = (position.y / crate::SQUARE_WIDTH as f32).round() * crate::SQUARE_WIDTH as f32;
                }
            }

            fn set_moving(&mut self, moving: bool) {
                self.moving = moving;
            }

            fn is_moving(&self) -> bool {
                self.moving
            }

            fn set_positions(&mut self, positions: Vec<Vec3>) {
                self.positions = positions;
            }
        }
    };
    TokenStream::from(expanded)
}
