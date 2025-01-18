fn main() {
    let sphere_radius = 1.0;
    let cylinder_radius = 1.0;
    let cylinder_height = 1.0;
    let cone_radius = 1.0;
    let cone_height = 1.0;

    let sphere_volume = (4.0 / 3.0) * (3.14) * f64::powf(sphere_radius, 3.0);
    let cylinder_volume = 3.14 * f64::powf(cylinder_radius, 2.0) * cylinder_height;
    let cone_volume = (1.0 / 3.0) * 3.14 * f64::powf(cone_radius, 2.0) * cone_height;

    println!("The volume of the sphere is {}.", sphere_volume);

    println!("The volume of the sphere is {}.", cylinder_volume);

    println!("The volume of the sphere is {}.", cone_volume);
}
