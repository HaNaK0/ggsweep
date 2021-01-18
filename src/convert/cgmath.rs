use ggez::mint;

use crate::convert::Convert;

impl<T> Convert<mint::Vector2<T>> for cgmath::Vector2<T> where T: Copy{
    fn convert_to(&self) -> mint::Vector2<T> {
        mint::Vector2 {x: self.x, y: self.y}
    }

    fn convert_from(other: mint::Vector2<T>) -> Self {
        cgmath::Vector2::new(other.x, other.y)
    }
} 

impl<T> Convert<mint::Point2<T>> for cgmath::Point2<T> where  T: Copy{
    fn convert_to(&self) -> mint::Point2<T> {
        mint::Point2 {x: self.x, y: self.y}
    }

    fn convert_from(other: mint::Point2<T>) -> Self {
        cgmath::Point2::new(other.x, other.y)
    }
}