struct PointCharge {
    position: glm::DVec2,
    charge: f64,
}

const K: f64 = 9e9;
const MICRO: f64 = 1e-6;

fn prefix(number: f64) -> String {
    let mut prefix = "";
    let mut number = number;
    if number.abs() >= 1e12 {
        prefix = "T";
        number = number / 1e12;
    } else if number.abs() >= 1e9 {
        prefix = "G";
        number = number / 1e9;
    } else if number.abs() >= 1e6 {
        prefix = "M";
        number = number / 1e6;
    } else if number.abs() >= 1e3 {
        prefix = "k";
        number = number / 1e3;
    } else if number.abs() >= 1.0 {
        prefix = "";
        number = number / 1.0;
    } else if number.abs() >= 1e-3 {
        prefix = "m";
        number = number / 1e-3;
    } else if number.abs() >= 1e-6 {
        prefix = "μ";
        number = number / 1e-6;
    } else if number.abs() >= 1e-9 {
        prefix = "n";
        number = number / 1e-9;
    } else if number.abs() >= 1e-12 {
        prefix = "p";
        number = number / 1e-12;
    } else if number.abs() >= 1e-15 {
        prefix = "f";
        number = number / 1e-15;
    } else if number.abs() >= 1e-18 {
        prefix = "a";
        number = number / 1e-18;
    } else if number.abs() >= 1e-21 {
        prefix = "z";
        number = number / 1e-21;
    } else if number.abs() >= 1e-24 {
        prefix = "y";
        number = number / 1e-24;
    }
    format!("{:.3}{}", number, prefix)
}

fn prefix_dvec2(v: &glm::DVec2) -> String {
    format!("({}, {})", prefix(v.x), prefix(v.y),)
}

fn field_strength_at_point(point: &glm::DVec2, particles: &[PointCharge]) -> glm::DVec2 {
    let mut field = glm::dvec2(0.0, 0.0);

    for particle in particles.iter() {
        let direction = particle.position - *point;
        let distance_squared = glm::length(direction) * glm::length(direction);
        let unit_direction = direction / glm::length(direction);
        let field_magnitude = unit_direction * K * particle.charge / distance_squared;
        field = field + field_magnitude;
    }

    field
}

fn example_19_5() {
    println!("\nExample 19.5: Electric field of two point charges");
    let world = vec![
        PointCharge {
            position: glm::dvec2(-1.0, 0.0),
            charge: 20.0 * MICRO,
        },
        PointCharge {
            position: glm::dvec2(1.0, 0.0),
            charge: -10.0 * MICRO,
        },
    ];

    for particle in world.iter() {
        println!(
            "Charge at {} with charge {}C",
            prefix_dvec2(&particle.position),
            prefix(particle.charge)
        );
    }

    let point = glm::dvec2(2.0, 2.0);
    let field = field_strength_at_point(&point, &world);
    println!(
        "Field at {} is {}N/C",
        prefix_dvec2(&point),
        prefix_dvec2(&field)
    );
}

fn example_19_6() {
    println!("\nExample 19.6: Dipole electric field");

    let d = 3.0;
    let q = 10.0 * MICRO;
    let y = 3.0;

    let world = vec![
        PointCharge {
            position: glm::dvec2(0.0, 0.0),
            charge: q,
        },
        PointCharge {
            position: glm::dvec2(d, 0.0),
            charge: -q,
        },
    ];

    for particle in world.iter() {
        println!(
            "Charge at {} with charge {}C",
            prefix_dvec2(&particle.position),
            prefix(particle.charge)
        );
    }

    let points = vec![
        glm::dvec2(-d / 2.0, 0.0),
        glm::dvec2(d / 2.0, 0.0),
        glm::dvec2(d / 2.0, y),
    ];

    for point in points.iter() {
        let field = field_strength_at_point(&point, &world);
        println!(
            "Field at {} is {}N/C",
            prefix_dvec2(&point),
            prefix_dvec2(&field)
        );
    }
}

fn main() {
    example_19_5();
    example_19_6();
}
