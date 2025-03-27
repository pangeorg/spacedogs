use crate::prelude::*;
use std::iter::zip;

#[derive(Clone, Debug, Component)]
pub struct PolyPath {
    vertices: Vec<Vec2>,
    pos: Vec2,
    total_length: f32,
    distance: f32,
}

#[allow(dead_code)]
impl PolyPath {
    pub fn new(vertices: Vec<Vec2>) -> Self {
        let total_length = calculate_total_length(&vertices);

        let mut pos = Vec2::new(0., 0.);
        if vertices.len() >= 1 {
            pos = vertices[0];
        }

        PolyPath {
            vertices: vertices.clone(),
            pos,
            total_length,
            distance: 0.,
        }
    }

    pub fn translate(&mut self, translation: Vec2) -> &mut Self {
        self.vertices = self.vertices.iter().map(|v| v + translation).collect();
        self.reset();
        self
    }

    fn reset(&mut self) -> &mut PolyPath {
        self.total_length = calculate_total_length(&self.vertices);
        _ = self.step(0.);
        self
    }

    pub fn scale(&mut self, scale: f32) -> &mut Self {
        //let center = self.center_of_mass();
        self.vertices = self
            .vertices
            .iter()
            .map(|v| {
                //let dir = (center - v).normalize();
                v * scale // * dir
            })
            .collect();
        self.reset();
        self
    }

    pub fn rotate(&mut self, angle: f32) -> &mut Self {
        let cosa = angle.cos();
        let sina = angle.sin();
        self.vertices = self
            .vertices
            .iter()
            .map(|v| Vec2::new(cosa * v.x - sina * v.y, sina * v.x + cosa * v.y))
            .collect();
        self.reset()
    }

    pub fn center_of_mass(&self) -> Vec2 {
        let sum = self
            .vertices
            .iter()
            .fold(Vec2::new(0., 0.), |acc, v| acc + v);
        sum / self.vertices.len() as f32
    }

    /// Take a step of dx on the path and return the updated position of the path
    pub fn step(&mut self, dx: f32) -> Vec2 {
        if self.vertices.len() == 1 {
            return self.vertices[0];
        }

        let dx = (dx + self.distance) % self.total_length;
        let mut total_length = 0.;

        for i in (0..self.vertices.len()).cycle() {
            let mut j = i + 1;
            if j == self.vertices.len() {
                j = 0;
            }
            let v1 = self.vertices[i];
            let v2 = self.vertices[j];
            let dist = v1.distance(v2);
            if total_length + dist >= dx {
                let direction = (v2 - v1).normalize();
                self.pos = v1 + (dx - total_length) * direction;
                self.distance = dx % self.total_length;
                return self.pos;
            }
            total_length += dist;
        }
        unreachable!();
    }

    pub fn contains(&self, pos: Vec2) -> bool {
        for e in self.edges() {
            if is_between(*e.0, *e.1, pos) {
                return true;
            }
        }
        false
    }

    pub fn edges(&self) -> impl Iterator<Item = (&Vec2, &Vec2)> {
        (0..self.vertices.len())
            .map(|i| {
                let mut j = i + 1;
                if j == self.vertices.len() {
                    j = 0;
                }
                let v1 = &self.vertices[i];
                let v2 = &self.vertices[j];

                (v1, v2)
            })
            .into_iter()
    }

    pub fn directions(&self) -> impl Iterator<Item = Vec2> {
        (0..self.vertices.len())
            .map(|i| {
                let mut j = i + 1;
                if j == self.vertices.len() {
                    j = 0;
                }
                let v1 = self.vertices[i];
                let v2 = self.vertices[j];

                (v2 - v1).normalize()
            })
            .into_iter()
    }

    /// Given a vector pos, return the direction to the closest vertex
    pub fn move_approx(&self, pos: Vec2) -> Vec2 {
        let mut dist = f32::MAX;
        let mut min_vertex = Vec2::default();

        for v in &self.vertices {
            let d = v.distance(pos);
            if d < dist {
                min_vertex = *v;
                dist = d;
            }
        }
        pos - min_vertex
    }
}

/// https://stackoverflow.com/questions/328107/how-can-you-determine-a-point-is-between-two-other-points-on-a-line-segment
fn is_between(a: Vec2, b: Vec2, c: Vec2) -> bool {
    let ac = a.distance(c);
    if ac <= 1. {
        return true;
    }
    let bc = c.distance(b);
    if bc <= 1. {
        return true;
    }
    let d = ac + bc - a.distance(b);
    (-1.0 <= d) && (d <= 1.0)
}

fn calculate_total_length(vertices: &Vec<Vec2>) -> f32 {
    let mut total_length = 0.;
    let n = vertices.len();
    if n == 0 {
        return 0.;
    }

    if vertices.len() >= 1 {
        for (v1, v2) in zip(&vertices[..n], &vertices[1..]) {
            total_length += v1.distance(*v2);
        }

        if vertices.len() > 1 {
            total_length += vertices[n - 1].distance(vertices[0]);
        }
    }

    total_length
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_between() {
        assert_eq!(
            is_between(Vec2::new(0.3, 0.), Vec2::new(1.0, 0.), Vec2::new(0.5, 0.)),
            true,
        );
    }

    #[test]
    fn triangle_path_okay() {
        let vertices = vec![Vec2::new(0.0, 0.0), Vec2::new(1.0, 0.0), Vec2::new(0.5, 1.)];
        let mut tria = PolyPath::new(vertices);
        tria.step(2.);
        assert_eq!(
            tria.pos.x, 0.5527864,
            "Testing correct x coordinate with dx overshoot"
        );
        assert_eq!(
            tria.pos.y, 0.8944272,
            "Testing correct y coordinate with dx overshoot"
        );

        let v = tria.step(2.2);

        assert_eq!(v.x, 1.0, "check if current direction is correct");
        assert_eq!(v.y, 0.0, "check if current direction is correct");
    }
}
