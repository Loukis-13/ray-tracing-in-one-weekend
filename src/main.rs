mod camera;
mod color;
mod hittable;
mod material;
mod ray;
mod shape;
mod vec3;

use camera::Camera;
use hittable::{Hittable, HittableList};
use material::{Dielectric, Lambertian, Metal};
use rand::random;
use ray::Ray;
use shape::Sphere;
use vec3::{Color, Point3, Vec3};

fn random_scene() -> HittableList {
    let mut world = HittableList::from(vec![
        Box::new(Sphere::from(
            Point3(0.0, -1000.0, 0.0),
            1000.0,
            Lambertian::from(Color(0.5, 0.5, 0.5)),
        )),
        Box::new(Sphere::from(Point3(0.0, 1.0, 0.0), 1.0, Dielectric::from(1.5))),
        Box::new(Sphere::from(
            Point3(-4.0, 1.0, 0.0),
            1.0,
            Lambertian::from(Color(0.4, 0.2, 0.1)),
        )),
        Box::new(Sphere::from(
            Point3(4.0, 1.0, 0.0),
            1.0,
            Metal::from(Color(0.7, 0.6, 0.5), 0.0),
        )),
    ]);

    let origin = Point3(4.0, 0.2, 0.0);

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random::<f64>();
            let center = Point3(a as f64 + 0.9 * random::<f64>(), 0.2, b as f64 + 0.9 * random::<f64>());

            if (center - origin).length() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    world.add(Sphere::from(
                        center,
                        0.2,
                        Lambertian::from(Color::random(0.0, 1.0) * Color::random(0.0, 1.0)),
                    ));
                } else if choose_mat < 0.95 {
                    // metal
                    world.add(Sphere::from(
                        center,
                        0.2,
                        Metal::from(Color::random(0.5, 1.0), random::<f64>() / 2.0),
                    ));
                } else {
                    // glass
                    world.add(Sphere::from(center, 0.2, Dielectric::from(1.5)));
                }
            }
        }
    }

    world
}

fn ray_color(r: &Ray, world: &dyn Hittable, depth: i32) -> Color {
    // If we've exceeded the ray bounce limit, no more light is gathered.
    if depth <= 0 {
        return Color::default();
    }

    if let Some(rec) = world.hit(r, 0.001, f64::INFINITY) {
        if let Some((scattered, attenuation)) = rec.mat_ptr.scatter(r, &rec) {
            return attenuation * ray_color(&scattered, world, depth - 1);
        }
        return Color::default();
    }

    let unit_direction = vec3::unit_vector(r.direction);
    let t = (unit_direction.1 + 1.0) * 0.5;
    Color(1.0, 1.0, 1.0) * (1.0 - t) + Color(0.5, 0.7, 1.0) * t
}

fn main() {
    // image
    let aspect_ratio = 3.0 / 2.0;
    let image_width = 1200;
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    let samples_per_pixel = 100;
    let max_depth = 50;

    // world
    let world = HittableList::from(vec![
        Box::new(Sphere::from(
            Point3(0.0, -1000.0, 0.0),
            1000.0,
            Lambertian::from(Color(0.5, 0.5, 0.5)),
        )),
        Box::new(Sphere::from(Point3(0.0, 1.0, 0.0), 1.0, Dielectric::from(1.5))),
        Box::new(Sphere::from(
            Point3(-4.0, 1.0, 0.0),
            1.0,
            Lambertian::from(Color(0.4, 0.2, 0.1)),
        )),
        Box::new(Sphere::from(
            Point3(4.0, 1.0, 0.0),
            1.0,
            Metal::from(Color(0.7, 0.6, 0.5), 0.0),
        )),
    ]);

    // camera
    let camera = Camera::new(
        Point3(13.0, 2.0, 3.0),
        Point3(0.0, 0.0, 0.0),
        Vec3(0.0, 1.0, 0.0),
        20.0,
        3.0 / 2.0,
        0.1,
        10.0,
    );

    // render

    println!("P3\n{image_width} {image_height}\n255");

    for i in (0..image_height).rev() {
        eprintln!("Scanlines remaining: {i}");
        for j in 0..image_width {
            let mut pixel_color = Color::new();
            for _ in 0..samples_per_pixel {
                let u = (j as f64 + random::<f64>()) / (image_width - 1) as f64;
                let v = (i as f64 + random::<f64>()) as f64 / (image_height - 1) as f64;
                let r = camera.get_ray(u, v);
                pixel_color += ray_color(&r, &world, max_depth);
            }

            color::write_color(pixel_color, samples_per_pixel);
        }
    }

    eprintln!("Done.");
}
