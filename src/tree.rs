use vector::Vector2;

use crate::{Rectangle, Positioned};

const NODE_CAPACITY: usize = 4;

pub struct Quadtree<'a> {
    boundary: Rectangle,

    entries: Vec<&'a dyn Positioned>,

    quadrants: Option<[Box<Quadtree<'a>>; 4]>,
}

impl<'a> Quadtree<'a> {
    pub fn new(boundary: Rectangle) -> Quadtree<'a> {
        Quadtree { 
            boundary, 
            entries: Vec::new(), 
            quadrants: None,
        }
    }

    pub fn insert(&mut self, entry: &'a dyn Positioned) -> Result<(), Box<dyn std::error::Error>> {
        if !self.boundary.contains(entry) {
            return Err("Entry not within bounds")?;
        }

        if self.entries.len() < NODE_CAPACITY && self.quadrants.is_none() {
            self.entries.push(entry);
            return Ok(());
        }

        if self.quadrants.is_none() {
            self.subdivide();
        }

        for quadrant in self.quadrants.as_mut().unwrap() {
            if quadrant.insert(entry).is_ok() {
                return Ok(());
            }
        }

        Err("This should not happen")?
    }

    pub fn query(&self, range: &Rectangle) -> Vec<&'a dyn Positioned> {
        let mut result = Vec::new();

        if !self.boundary.intersects(&range) {
            return result;
        }

        for entry in &self.entries {
            if range.contains(*entry) {
                result.push(*entry);
            }
        }

        if self.quadrants.is_none() {
            return result;
        }

        for quadrant in self.quadrants.as_ref().unwrap() {
            result.append(&mut quadrant.query(range));
        }

        result
    }

    fn subdivide(&mut self) {
        let (px, py) = (self.boundary.center.x, self.boundary.center.y);
        let (hx, hy) = (self.boundary.half_dim.x, self.boundary.half_dim.y);

        let half_dim = Vector2::new(hx / 2.0, hy / 2.0);

        // North-West quadrant
        let nw_center = Vector2::new(px - hx / 2.0, py - hy / 2.0);
        let north_west = Box::new(Quadtree::new(
            Rectangle::new(nw_center, half_dim.clone()))
        );
    
        // North-East quadrant
        let ne_center = Vector2::new(px + hx / 2.0, py - hy / 2.0);
        let north_east = Box::new(Quadtree::new(
            Rectangle::new(ne_center, half_dim.clone()))
        );

        // South-West quadrant
        let sw_center = Vector2::new(px - hx / 2.0, py + hy / 2.0);
        let south_west = Box::new(Quadtree::new(
            Rectangle::new(sw_center, half_dim.clone()))
        );

        // South-East quadrant
        let se_center = Vector2::new(px + hx / 2.0, py + hy / 2.0);
        let south_east = Box::new(Quadtree::new(
            Rectangle::new(se_center, half_dim.clone()))
        );

        self.quadrants = Some([north_west, north_east, south_west, south_east]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use vector::Vector2;
    use crate::Rectangle;
    use crate::Positioned;

    #[test]
    fn test_insert() {
        let mut tree = Quadtree::new(Rectangle::new(Vector2::new(0.0, 0.0), Vector2::new(100.0, 100.0)));
        let entry = Vector2::new(50.0, 50.0);
        assert!(tree.insert(&entry).is_ok());
    }

    #[test]
    fn test_insert_out_of_bounds() {
        let mut tree = Quadtree::new(Rectangle::new(Vector2::new(0.0, 0.0), Vector2::new(100.0, 100.0)));
        let entry = Vector2::new(150.0, 150.0);
        assert!(tree.insert(&entry).is_err());
    }

    #[test]
    fn test_query() {
        let mut tree = Quadtree::new(Rectangle::new(Vector2::new(0.0, 0.0), Vector2::new(100.0, 100.0)));
        let entry = Vector2::new(50.0, 50.0);
        tree.insert(&entry).unwrap();
        let range = Rectangle::new(Vector2::new(0.0, 0.0), Vector2::new(100.0, 100.0));
        let result = tree.query(&range);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].position(), entry.position());
    }

    #[test]
    fn test_query_out_of_bounds() {
        let mut tree = Quadtree::new(Rectangle::new(Vector2::new(0.0, 0.0), Vector2::new(100.0, 100.0)));
        let entry = Vector2::new(150.0, 150.0);
        if tree.insert(&entry).is_err() {}
        let range = Rectangle::new(Vector2::new(0.0, 0.0), Vector2::new(
            100.0, 100.0));
        let result = tree.query(&range);
        assert_eq!(result.len(), 0);
    }
}