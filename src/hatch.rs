use crate::{Point, Vector, Handle};

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct BoundaryPath {
    pub boundary_type_flags: i32,
    pub edges: Vec<BoundaryPathEdge>,
    pub source_boundary_objects: Vec<Handle>,
}

impl BoundaryPath {
    pub fn new() -> BoundaryPath {
        BoundaryPath {
            boundary_type_flags: 0,
            edges: vec![],
            source_boundary_objects: vec![],
        }
    }

    pub fn is_external(&self) -> bool {
        (self.boundary_type_flags & 1) != 0
    }

    pub fn set_external(&mut self, is_external: bool) {
        if is_external {
            self.boundary_type_flags |= 1;
        } else {
            self.boundary_type_flags &= !1;
        }
    }

    pub fn is_polyline(&self) -> bool {
        (self.boundary_type_flags & 2) != 0
    }

    pub fn set_polyline(&mut self, is_polyline: bool) {
        if is_polyline {
            self.boundary_type_flags |= 2;
        } else {
            self.boundary_type_flags &= !2;
        }
    }

    pub fn add_polyline_vertex(&mut self, vertex: Point) {
        self.edges.push(BoundaryPathEdge::Polyline { vertex });
        self.set_polyline(true);
    }

    pub fn add_line_edge(&mut self, start: Point, end: Point) {
        self.edges.push(BoundaryPathEdge::Line { start, end });
    }

    pub fn add_circular_arc_edge(&mut self, center: Point, radius: f64, start_angle: f64, end_angle: f64, is_counter_clockwise: bool) {
        self.edges.push(BoundaryPathEdge::CircularArc {
            center,
            radius,
            start_angle,
            end_angle,
            is_counter_clockwise,
        });
    }

    pub fn add_elliptical_arc_edge(&mut self, center: Point, major_axis_endpoint: Point, minor_axis_ratio: f64, start_angle: f64, end_angle: f64, is_counter_clockwise: bool) {
        self.edges.push(BoundaryPathEdge::EllipticalArc {
            center,
            major_axis_endpoint,
            minor_axis_ratio,
            start_angle,
            end_angle,
            is_counter_clockwise,
        });
    }

    pub fn add_spline_edge(&mut self, degree: i32, rational: bool, periodic: bool, knots: Vec<f64>, control_points: Vec<Point>, weights: Option<Vec<f64>>) {
        self.edges.push(BoundaryPathEdge::Spline {
            degree,
            rational,
            periodic,
            knots,
            control_points,
            weights,
        });
    }

    /// Create a rectangular boundary path
    pub fn from_rectangle(min_x: f64, min_y: f64, max_x: f64, max_y: f64, is_external: bool) -> BoundaryPath {
        let mut path = BoundaryPath::new();
        path.set_external(is_external);
        
        let corners = vec![
            Point::new(min_x, min_y, 0.0),
            Point::new(max_x, min_y, 0.0),
            Point::new(max_x, max_y, 0.0),
            Point::new(min_x, max_y, 0.0),
        ];
        
        for vertex in corners {
            path.add_polyline_vertex(vertex);
        }
        
        path
    }

    /// Create a circular boundary path
    pub fn from_circle(center: Point, radius: f64, is_external: bool) -> BoundaryPath {
        let mut path = BoundaryPath::new();
        path.set_external(is_external);
        path.add_circular_arc_edge(center, radius, 0.0, 360.0, false);
        path
    }

    /// Create a polygon boundary path from points
    pub fn from_polygon(points: Vec<Point>, is_external: bool) -> BoundaryPath {
        let mut path = BoundaryPath::new();
        path.set_external(is_external);
        
        for point in points {
            path.add_polyline_vertex(point);
        }
        
        path
    }
}

impl Default for BoundaryPath {
    fn default() -> Self {
        BoundaryPath::new()
    }
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub enum BoundaryPathEdge {
    Line {
        start: Point,
        end: Point,
    },
    CircularArc {
        center: Point,
        radius: f64,
        start_angle: f64,
        end_angle: f64,
        is_counter_clockwise: bool,
    },
    EllipticalArc {
        center: Point,
        major_axis_endpoint: Point,
        minor_axis_ratio: f64,
        start_angle: f64,
        end_angle: f64,
        is_counter_clockwise: bool,
    },
    Spline {
        degree: i32,
        rational: bool,
        periodic: bool,
        knots: Vec<f64>,
        control_points: Vec<Point>,
        weights: Option<Vec<f64>>,
    },
    Polyline {
        vertex: Point,
    },
}

impl BoundaryPathEdge {
    pub fn edge_type(&self) -> i32 {
        match self {
            BoundaryPathEdge::Line { .. } => 1,
            BoundaryPathEdge::CircularArc { .. } => 2,
            BoundaryPathEdge::EllipticalArc { .. } => 3,
            BoundaryPathEdge::Spline { .. } => 4,
            BoundaryPathEdge::Polyline { .. } => 0, // Special case for polyline boundary
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct PatternDefinitionLine {
    pub angle: f64,
    pub base_point: Point,
    pub offset: Vector,
    pub dash_lengths: Vec<f64>,
}

impl PatternDefinitionLine {
    pub fn new(angle: f64, base_point: Point, offset: Vector) -> PatternDefinitionLine {
        PatternDefinitionLine {
            angle,
            base_point,
            offset,
            dash_lengths: vec![],
        }
    }
}

impl Default for PatternDefinitionLine {
    fn default() -> Self {
        PatternDefinitionLine::new(0.0, Point::origin(), Vector::new(0.0, 0.0, 0.0))
    }
}