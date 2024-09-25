use std::{collections::HashMap, rc::Rc};

use rt::{
    models::{
        camera::Camera,
        instances::{rotate::Rotate, translate::Translate},
        materials::{
            dielectric::Dielectric,
            diffuse_light::DiffuseLight,
            material::{Lambertian, Material},
            metal::Metal,
        },
        objects::{
            cylinder::Cylinder,
            global::{Hittable, HittableList},
            quad::{box_shape, Quad},
            sphere::Sphere,
        },
    },
    utils::math::{random, random_float, random_vector},
};

use glam::{DVec3, DVec3 as Point, DVec3 as Color};

fn main() {
/////////////////////////// ADD OTHER REFRACTION INDEXES IF YOU WANT TO ////////////////////////////////
    // Set of materials and their refraction index
    let materials: Vec<&str> = vec!["air", "water", "glass", "diamond"];
    let indexes: Vec<f64> = vec![1.003, 1.33, 1.52, 2.42];

///////////////////////////!!!!!! DO NOT TOUCH THIS !!!!!!!//////////////////////////////////

    let mut refraction_indexes: HashMap<&str, f64> = HashMap::new();
    materials
        .iter()
        .zip(indexes)
        .for_each(|(&material, index)| {
            refraction_indexes.insert(material, index);
        });
///////////////////////////!!!!! DO NOT TOUCH THIS !!!!!!!//////////////////////////////////

    //World settings, adding objects

    let mut world: HittableList = HittableList::new();

    // Camera Scenes
    // let mut camera: Camera =  spheres(&mut world, refraction_indexes);
    let mut camera: Camera = cornell_box(&mut world, refraction_indexes);
    // let mut camera: Camera = light(&mut world, refraction_indexes);
    // let mut camera: Camera = testing(&mut world, refraction_indexes);

    camera.render(&mut world);
}

pub fn spheres(world: &mut HittableList, refraction_indexes: HashMap<&str, f64>) -> Camera {
    let ground: Rc<Lambertian> = Rc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.add(Rc::new(Sphere::new(
        Point::new(0.0, -1000.0, 0.0),
        1000.0,
        ground,
    )));

    for i in -11..11 {
        for j in -11..11 {
            let choose_mat: f64 = random_float();
            let center: Point = Point::new(
                i as f64 + 0.9 * random_float(),
                0.2,
                j as f64 + 0.9 * random_float(),
            );

            if (center - Point::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let material: Rc<dyn Material> = if choose_mat < 0.8 {
                    let albedo: Color = Color::new(random_float(), random_float(), random_float())
                        * Color::new(random_float(), random_float(), random_float());
                    Rc::new(Lambertian::new(albedo))
                } else if choose_mat < 0.95 {
                    let albedo: Color = random_vector(0.5, 1.0);
                    let fuzz: f64 = random(0.0, 0.5);
                    Rc::new(Metal::new(albedo, fuzz))
                } else {
                    Rc::new(Dielectric::new(refraction_indexes["glass"]))
                };
                world.add(Rc::new(Sphere::new(center, 0.2, material)));
            }
        }
    }

    let material_1: Rc<Dielectric> = Rc::new(Dielectric::new(refraction_indexes["glass"]));
    world.add(Rc::new(Sphere::new(
        Point::new(0.0, 1.0, 0.0),
        1.0,
        material_1,
    )));

    let material_2: Rc<Lambertian> = Rc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Rc::new(Sphere::new(
        Point::new(-4.0, 1.0, 0.0),
        1.0,
        material_2,
    )));

    let material_3: Rc<Metal> = Rc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Rc::new(Sphere::new(
        Point::new(4.0, 1.0, 0.0),
        1.0,
        material_3,
    )));

    let image_width: i32 = 800;
    let aspect_ratio: f64 = 16.0 / 9.0;
    let camera_fov: f64 = 50.0;
    let camera_background: Color = Color::new(0.7, 0.8, 1.0);

    // camera position
    let look_from: Point = Point::new(13.0, 2.0, 3.0);
    let look_at: Point = Point::new(0.0, 0.0, 0.0);
    let vup: DVec3 = DVec3::new(0.0, 1.0, 0.0);

    // depth of field
    let defocus_angle: f64 = 0.6;
    let focus_dist: f64 = 10.0;

    let samples_per_pixel: f64 = 50.0;
    let max_depth: f64 = 50.0;
    let brightness: f64 = 1.0;

    let camera: Camera = Camera::new(
        camera_fov,
        image_width,
        look_from,
        look_at,
        vup,
        aspect_ratio,
        samples_per_pixel,
        max_depth,
        defocus_angle,
        focus_dist,
        camera_background,
        brightness,
    );

    camera
}

pub fn light(world: &mut HittableList, _refraction_indexes: HashMap<&str, f64>) -> Camera {
    let material_3: Rc<Lambertian> = Rc::new(Lambertian::new(Color::new(0.7, 0.6, 0.5)));
    world.add(Rc::new(Sphere::new(
        Point::new(0.0, -1000.0, 0.0),
        1000.0,
        material_3,
    )));

    let material_3: Rc<Lambertian> = Rc::new(Lambertian::new(Color::new(0.7, 0.6, 0.5)));
    world.add(Rc::new(Sphere::new(
        Point::new(0.0, 1.0, 0.0),
        1.0,
        material_3,
    )));

    let diff_light: Rc<DiffuseLight> = Rc::new(DiffuseLight::new(Color::new(4.0, 4.0, 4.0)));
    world.add(Rc::new(Quad::new(
        Point::new(3.0, 1.0, -2.0),
        DVec3::new(2.0, 0.0, 0.0),
        DVec3::new(0.0, 2.0, 0.0),
        diff_light,
    )));

    let diff_light1: Rc<DiffuseLight> = Rc::new(DiffuseLight::new(Color::new(0.0, 3.0, 0.2)));
    world.add(Rc::new(Sphere::new(
        Point::new(-3.0, 3.0, 2.0),
        1.0,
        diff_light1,
    )));

    let image_width: i32 = 800;
    let aspect_ratio: f64 = 16.0 / 9.0;
    let camera_fov: f64 = 20.0;
    let camera_background: Color = Color::new(0.0, 0.0, 0.0);

    // camera position
    let look_from: Point = Point::new(26.0, 3.0, 6.0);
    let look_at: Point = Point::new(0.0, 2.0, 0.0);
    let vup: DVec3 = DVec3::new(0.0, 1.0, 0.0);

    // depth of field
    let defocus_angle: f64 = 0.0;
    let focus_dist: f64 = 10.0;

    let samples_per_pixel: f64 = 100.0;
    let max_depth: f64 = 50.0;
    let brightness: f64 = 1.0;

    let camera: Camera = Camera::new(
        camera_fov,
        image_width,
        look_from,
        look_at,
        vup,
        aspect_ratio,
        samples_per_pixel,
        max_depth,
        defocus_angle,
        focus_dist,
        camera_background,
        brightness,
    );

    camera
}

pub fn cornell_box(world: &mut HittableList, refraction_indexes: HashMap<&str, f64>) -> Camera {
    let red: Rc<Lambertian> = Rc::new(Lambertian::new(Color::new(0.65, 0.05, 0.05)));
    let white: Rc<Lambertian> = Rc::new(Lambertian::new(Color::new(0.73, 0.73, 0.73)));
    let green: Rc<Lambertian> = Rc::new(Lambertian::new(Color::new(0.12, 0.45, 0.15)));
    let light: Rc<DiffuseLight> = Rc::new(DiffuseLight::new(Color::new(15.0, 15.0, 15.0)));
    // let light: Rc<DiffuseLight> = Rc::new(DiffuseLight::new(Color::new(7.0, 7.0, 7.0)));
    // let metal: Rc<Metal> = Rc::new(Metal::new(Color::new(0.5, 0.5, 0.5), 0.0));

    // Adding walls
    world.add(Rc::new(Quad::new(
        Point::new(555.0, 0.0, 0.0),
        DVec3::new(0.0, 555.0, 0.0),
        DVec3::new(0.0, 0.0, 555.0),
        green,
    )));
    world.add(Rc::new(Quad::new(
        Point::new(0.00, 0.0, 0.0),
        DVec3::new(0.0, 555.0, 0.0),
        DVec3::new(0.0, 0.0, 555.0),
        red,
    )));
    world.add(Rc::new(Quad::new(
        Point::new(0.0, 0.0, 0.0),
        DVec3::new(555.0, 0.0, 0.0),
        DVec3::new(0.0, 0.0, 555.0),
        white.clone(),
    )));
    world.add(Rc::new(Quad::new(
        Point::new(555.0, 555.0, 555.0),
        DVec3::new(-555.0, 0.0, 0.0),
        DVec3::new(0.0, 0.0, -555.0),
        white.clone(),
    )));
    world.add(Rc::new(Quad::new(
        Point::new(0.0, 0.0, 555.0),
        DVec3::new(555.0, 0.0, 0.0),
        DVec3::new(0.0, 555.0, 0.0),
        white.clone(),
    )));

    // Adding light
    world.add(Rc::new(Quad::new(
        Point::new(344.0, 554.0, 332.0),
        DVec3::new(-130.0, 0.0, 0.0),
        DVec3::new(0.0, 0.0, -105.0),
        light,
    )));
    // world.add(Rc::new(Quad::new(
    //     Point::new(113.0, 554.0, 127.0),
    //     DVec3::new(330.0, 0.0, 0.0),
    //     DVec3::new(0.0, 0.0, 305.0),
    //     light,
    // ))); // big light

    // Adding boxes
    let mut box_1: Rc<dyn Hittable> = box_shape(Point::new(0.0, 0.0, 0.0), Point::new(165.0, 330.0, 165.0), white.clone()); // left box
    box_1 = Rc::new(Rotate::new(box_1, 15.0));
    box_1 = Rc::new(Translate::new(box_1, DVec3::new(265.0, 0.0, 295.0)));

    let mut box_2: Rc<dyn Hittable> = box_shape(
        Point::new(0.0, 0.0, 0.0),
        Point::new(165.0, 165.0, 165.0),
        white.clone(),
    ); // right box
    box_2 = Rc::new(Rotate::new(box_2, -18.0));
    box_2 = Rc::new(Translate::new(box_2, DVec3::new(130.0, 0.0, 65.0)));

    // let glass: Rc<Dielectric> = Rc::new(Dielectric::new(refraction_indexes["glass"]));
    // let glass_sphere: Rc<Sphere> =
    //     Rc::new(Sphere::new(Point::new(190.0, 290.0, 190.0), 90.0, glass));

    // let cylinder: Rc<Cylinder> = Rc::new(Cylinder::new(
    //     Point::new(400.0, 0.0, 250.0),
    //     DVec3::new(0.0, 1.0, 0.0),
    //     60.0,
    //     180.0,
    //     metal,
    // ));

    world.add(box_1);
    world.add(box_2);
    // world.add(glass_sphere);
    // world.add(cylinder);

    let image_width: i32 = 800;
    let aspect_ratio: f64 = 1.0;
    let camera_fov: f64 = 40.0;
    let camera_background: Color = Color::new(0.0, 0.0, 0.0);

    // camera position
    let look_from: Point = Point::new(278.0, 278.0, -800.0);
    let look_at: Point = Point::new(278.0, 278.0, 0.0);
    let vup: DVec3 = DVec3::new(0.0, 1.0, 0.0);

    // depth of field
    let defocus_angle: f64 = 0.0;
    let focus_dist: f64 = 10.0;

    let samples_per_pixel: f64 = 100.0;
    let max_depth: f64 = 50.0;
    let brightness: f64 = 1.0;

    let camera: Camera = Camera::new(
        camera_fov,
        image_width,
        look_from,
        look_at,
        vup,
        aspect_ratio,
        samples_per_pixel,
        max_depth,
        defocus_angle,
        focus_dist,
        camera_background,
        brightness,
    );

    camera
}

pub fn testing(world: &mut HittableList, _refraction_indexes: HashMap<&str, f64>) -> Camera {
    let ground: Rc<Lambertian> = Rc::new(Lambertian::new(Color::new(1.0, 0.0, 0.0)));
    world.add(Rc::new(Sphere::new(
        Point::new(0.0, -1000.0, 0.0),
        1000.0,
        ground,
    )));

    // let material: Rc<Metal> = Rc::new(Metal::new(Color::new(0.5, 0.5, 0.5), 0.0));

    // let box_2: Rc<dyn Hittable> = box_shape(
    //     Point::new(0.0, 4.0, 0.0),
    //     Point::new(3.0, 3.0, 3.0),
    //     material.clone(),
    // ); // right box

    let sphere: Rc<Sphere> = Rc::new(Sphere::new(
        Point::new(2.0, 0.0, 0.0),
        1.0,
        Rc::new(Metal::new(Color::new(1.0, 1.0, 0.3), 0.0)),
    ));

    let sphere2: Rc<Sphere> = Rc::new(Sphere::new(
        Point::new(5.0, 0.0, 0.0),
        1.0,
        Rc::new(Metal::new(Color::new(1.0, 1.0, 0.3), 0.4)),
    ));
    // world.add(mirror);
    // world.add(box_2);
    world.add(sphere);
    world.add(sphere2);

    let image_width: i32 = 800;
    let aspect_ratio: f64 = 16.0 / 9.0;
    let camera_fov: f64 = 90.0;
    let camera_background: Color = Color::new(0.4, 0.6, 0.8);

    // camera position
    let look_from: Point = Point::new(3.0, 3.0, 3.0);
    let look_at: Point = Point::new(3.0, 0.0, 0.0);
    let vup: DVec3 = DVec3::new(0.0, 1.0, 0.0);

    // depth of field
    let defocus_angle: f64 = 3.0;
    let focus_dist: f64 = 3.0;

    let samples_per_pixel: f64 = 50.0;
    let max_depth: f64 = 50.0;
    let brightness: f64 = 1.0;

    let camera: Camera = Camera::new(
        camera_fov,
        image_width,
        look_from,
        look_at,
        vup,
        aspect_ratio,
        samples_per_pixel,
        max_depth,
        defocus_angle,
        focus_dist,
        camera_background,
        brightness,
    );

    camera
}
//TODO https://raytracing.github.io/books/RayTracingTheRestOfYourLife.html#asimplemontecarloprogram/stratifiedsamples(jittering)
