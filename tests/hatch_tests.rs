#[cfg(test)]
mod hatch_tests {
    use dxf::{Drawing, Point};
    use dxf::entities::{Entity, EntityType, Hatch};
    use dxf::enums::{HatchStyle, HatchPatternType};
    use dxf::{BoundaryPath, BoundaryPathEdge};

    #[test]
    fn test_create_simple_rectangle_hatch() {
        let hatch = Hatch::new_rectangle_solid_fill(0.0, 0.0, 10.0, 5.0);
        
        assert!(hatch.solid_fill);
        assert_eq!(hatch.hatch_pattern_name, "SOLID");
        assert_eq!(hatch.boundary_paths.len(), 1);
        
        let boundary = &hatch.boundary_paths[0];
        assert!(boundary.is_external());
        assert!(boundary.is_polyline());
        assert_eq!(boundary.edges.len(), 4);
    }

    #[test]
    fn test_create_circular_hatch() {
        let center = Point::new(5.0, 5.0, 0.0);
        let hatch = Hatch::new_circle_solid_fill(center.clone(), 3.0);
        
        assert!(hatch.solid_fill);
        assert_eq!(hatch.boundary_paths.len(), 1);
        
        let boundary = &hatch.boundary_paths[0];
        assert!(boundary.is_external());
        assert_eq!(boundary.edges.len(), 1);
        
        match &boundary.edges[0] {
            BoundaryPathEdge::CircularArc { center: arc_center, radius, .. } => {
                assert_eq!(*arc_center, center);
                assert_eq!(*radius, 3.0);
            }
            _ => panic!("Expected circular arc edge"),
        }
    }

    #[test]
    fn test_create_polygon_hatch() {
        let points = vec![
            Point::new(0.0, 0.0, 0.0),
            Point::new(10.0, 0.0, 0.0),
            Point::new(10.0, 5.0, 0.0),
            Point::new(5.0, 8.0, 0.0),
            Point::new(0.0, 5.0, 0.0),
        ];
        
        let hatch = Hatch::new_polygon_solid_fill(points.clone());
        
        assert!(hatch.solid_fill);
        assert_eq!(hatch.boundary_paths.len(), 1);
        
        let boundary = &hatch.boundary_paths[0];
        assert!(boundary.is_external());
        assert!(boundary.is_polyline());
        assert_eq!(boundary.edges.len(), points.len());
    }

    #[test]
    fn test_create_polygon_with_holes() {
        let outer_points = vec![
            Point::new(0.0, 0.0, 0.0),
            Point::new(20.0, 0.0, 0.0),
            Point::new(20.0, 20.0, 0.0),
            Point::new(0.0, 20.0, 0.0),
        ];
        
        let hole1_points = vec![
            Point::new(5.0, 5.0, 0.0),
            Point::new(8.0, 5.0, 0.0),
            Point::new(8.0, 8.0, 0.0),
            Point::new(5.0, 8.0, 0.0),
        ];
        
        let hole2_points = vec![
            Point::new(12.0, 12.0, 0.0),
            Point::new(15.0, 12.0, 0.0),
            Point::new(15.0, 15.0, 0.0),
            Point::new(12.0, 15.0, 0.0),
        ];
        
        let hatch = Hatch::new_polygon_with_holes_solid_fill(
            outer_points.clone(),
            vec![hole1_points.clone(), hole2_points.clone()]
        );
        
        assert!(hatch.solid_fill);
        assert_eq!(hatch.hatch_style, HatchStyle::Normal);
        assert_eq!(hatch.boundary_paths.len(), 3); // 1 outer + 2 holes
        
        // Check outer boundary
        let outer_boundary = &hatch.boundary_paths[0];
        assert!(outer_boundary.is_external());
        assert_eq!(outer_boundary.edges.len(), outer_points.len());
        
        // Check holes
        let hole1_boundary = &hatch.boundary_paths[1];
        assert!(!hole1_boundary.is_external()); // holes should not be external
        assert_eq!(hole1_boundary.edges.len(), hole1_points.len());
        
        let hole2_boundary = &hatch.boundary_paths[2];
        assert!(!hole2_boundary.is_external()); // holes should not be external
        assert_eq!(hole2_boundary.edges.len(), hole2_points.len());
    }

    #[test]
    fn test_add_holes_to_existing_hatch() {
        let mut hatch = Hatch::new_rectangle_solid_fill(0.0, 0.0, 20.0, 20.0);
        
        // Add a circular hole
        hatch.add_circular_hole(Point::new(5.0, 5.0, 0.0), 2.0);
        
        // Add a rectangular hole
        hatch.add_rectangular_hole(10.0, 10.0, 15.0, 15.0);
        
        // Add a polygon hole
        let triangle_hole = vec![
            Point::new(2.0, 15.0, 0.0),
            Point::new(5.0, 18.0, 0.0),
            Point::new(8.0, 15.0, 0.0),
        ];
        hatch.add_hole(triangle_hole);
        
        assert_eq!(hatch.boundary_paths.len(), 4); // 1 outer + 3 holes
        
        // Verify all holes are marked as internal (not external)
        for i in 1..hatch.boundary_paths.len() {
            assert!(!hatch.boundary_paths[i].is_external());
        }
    }

    #[test]
    fn test_boundary_path_edge_types() {
        let mut boundary = BoundaryPath::new();
        
        // Add different types of edges
        boundary.add_line_edge(Point::new(0.0, 0.0, 0.0), Point::new(5.0, 0.0, 0.0));
        boundary.add_circular_arc_edge(Point::new(7.5, 2.5, 0.0), 2.5, 0.0, std::f64::consts::PI, false);
        boundary.add_elliptical_arc_edge(
            Point::new(10.0, 0.0, 0.0),
            Point::new(5.0, 0.0, 0.0),
            0.5,
            0.0,
            std::f64::consts::PI,
            false
        );
        
        assert_eq!(boundary.edges.len(), 3);
        
        match &boundary.edges[0] {
            BoundaryPathEdge::Line { .. } => {
                assert_eq!(boundary.edges[0].edge_type(), 1);
            }
            _ => panic!("Expected line edge"),
        }
        
        match &boundary.edges[1] {
            BoundaryPathEdge::CircularArc { .. } => {
                assert_eq!(boundary.edges[1].edge_type(), 2);
            }
            _ => panic!("Expected circular arc edge"),
        }
        
        match &boundary.edges[2] {
            BoundaryPathEdge::EllipticalArc { .. } => {
                assert_eq!(boundary.edges[2].edge_type(), 3);
            }
            _ => panic!("Expected elliptical arc edge"),
        }
    }

    #[test]
    fn test_hatch_in_drawing() {
        let mut drawing = Drawing::new();
        
        // Create a hatch with a hole
        let outer_points = vec![
            Point::new(0.0, 0.0, 0.0),
            Point::new(10.0, 0.0, 0.0),
            Point::new(10.0, 10.0, 0.0),
            Point::new(0.0, 10.0, 0.0),
        ];
        
        let hole_points = vec![
            Point::new(3.0, 3.0, 0.0),
            Point::new(7.0, 3.0, 0.0),
            Point::new(7.0, 7.0, 0.0),
            Point::new(3.0, 7.0, 0.0),
        ];
        
        let hatch = Hatch::new_polygon_with_holes_solid_fill(
            outer_points,
            vec![hole_points]
        );
        
        let entity = Entity::new(EntityType::Hatch(hatch));
        drawing.add_entity(entity);
        
        assert_eq!(drawing.entities().count(), 1);
        
        let first_entity = drawing.entities().next().unwrap();
        match &first_entity.specific {
            EntityType::Hatch(h) => {
                assert!(h.solid_fill);
                assert_eq!(h.boundary_paths.len(), 2);
            }
            _ => panic!("Expected hatch entity"),
        }
    }

    #[test]
    fn test_hatch_default_values() {
        let hatch = Hatch::default();
        
        assert!(hatch.solid_fill);
        assert!(!hatch.associative);
        assert_eq!(hatch.hatch_style, HatchStyle::Normal);
        assert_eq!(hatch.hatch_pattern_type, HatchPatternType::UserDefined);
        assert_eq!(hatch.hatch_pattern_angle, 0.0);
        assert_eq!(hatch.hatch_pattern_scale, 1.0);
        assert!(!hatch.hatch_pattern_double);
        assert_eq!(hatch.elevation, 0.0);
        assert_eq!(hatch.pixel_size, 0.0);
        assert!(hatch.boundary_paths.is_empty());
        assert!(hatch.pattern_definition_lines.is_empty());
        assert!(hatch.seed_points.is_empty());
    }
}