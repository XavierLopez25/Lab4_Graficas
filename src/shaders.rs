use crate::color::Color;
use crate::fragment::Fragment;
use crate::vertex::Vertex;
use crate::Uniforms;
use nalgebra_glm::{mat4_to_mat3, Mat3, Vec3, Vec4};
use rand::rngs::StdRng;
use rand::Rng;
use rand::SeedableRng;

pub fn vertex_shader(vertex: &Vertex, uniforms: &Uniforms) -> Vertex {
    // Transform position
    let position = Vec4::new(vertex.position.x, vertex.position.y, vertex.position.z, 1.0);
    let transformed =
        uniforms.projection_matrix * uniforms.view_matrix * uniforms.model_matrix * position;

    // Perform perspective division
    let w = transformed.w;
    let ndc_position = Vec4::new(transformed.x / w, transformed.y / w, transformed.z / w, 1.0);

    // apply viewport matrix
    let screen_position = uniforms.viewport_matrix * ndc_position;

    // Transform normal
    let model_mat3 = mat4_to_mat3(&uniforms.model_matrix);
    let normal_matrix = model_mat3
        .transpose()
        .try_inverse()
        .unwrap_or(Mat3::identity());

    let transformed_normal = normal_matrix * vertex.normal;

    // Create a new Vertex with transformed attributes
    Vertex {
        position: vertex.position,
        normal: vertex.normal,
        tex_coords: vertex.tex_coords,
        color: vertex.color,
        transformed_position: Vec3::new(screen_position.x, screen_position.y, screen_position.z),
        transformed_normal,
    }
}

pub fn fragment_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    // random_color_shader(fragment, uniforms)
    // black_and_white(fragment, uniforms)
    // dalmata_shader(fragment, uniforms)
    // cloud_shader(fragment, uniforms)
    // cellular_shader(fragment, uniforms)
    // cracked_ground_shader(fragment, uniforms)
    lava_shader(fragment, uniforms)
}

fn random_color_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let seed = uniforms.time as u64;

    let mut rng = StdRng::seed_from_u64(seed);

    let r = rng.gen_range(0..=255);
    let g = rng.gen_range(0..=255);
    let b = rng.gen_range(0..=255);

    let random_color = Color::new(r, g, b);

    random_color * fragment.intensity
}

fn black_and_white(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let seed = uniforms.time as f32 * fragment.vertex_position.y * fragment.vertex_position.x;

    let mut rng = StdRng::seed_from_u64(seed.abs() as u64);

    let random_number = rng.gen_range(0..=100);

    let black_or_white = if random_number < 50 {
        Color::new(0, 0, 0)
    } else {
        Color::new(255, 255, 255)
    };

    black_or_white * fragment.intensity
}

fn dalmata_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let zoom = 100.0;
    let ox = 0.0;
    let oy = 0.0;
    let x = fragment.vertex_position.x;
    let y = fragment.vertex_position.y;

    let noise_value = uniforms.noises[0].get_noise_2d((x + ox) * zoom, (y + oy) * zoom);

    let spot_threshold = 0.5;
    let spot_color = Color::new(255, 255, 255); // White
    let base_color = Color::new(0, 0, 0); // Black

    let noise_color = if noise_value < spot_threshold {
        spot_color
    } else {
        base_color
    };

    noise_color * fragment.intensity
}

fn cloud_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let zoom = 100.0; // to move our values
    let ox = 100.0; // offset x in the noise map
    let oy = 100.0;
    let x = fragment.vertex_position.x;
    let y = fragment.vertex_position.y;
    let t = uniforms.time as f32 * 0.5;

    let noise_value = uniforms.noises[0].get_noise_2d(x * zoom + ox + t, y * zoom + oy);

    // Define cloud threshold and colors
    let cloud_threshold = 0.5; // Adjust this value to change cloud density
    let cloud_color = Color::new(255, 255, 255); // White for clouds
    let sky_color = Color::new(30, 97, 145); // Sky blue

    // Determine if the pixel is part of a cloud or sky
    let noise_color = if noise_value > cloud_threshold {
        cloud_color
    } else {
        sky_color
    };

    noise_color * fragment.intensity
}

fn cellular_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let zoom = 30.0; // Zoom factor to adjust the scale of the cell pattern
    let ox = 50.0; // Offset x in the noise map
    let oy = 50.0; // Offset y in the noise map
    let x = fragment.vertex_position.x;
    let y = fragment.vertex_position.y;

    // Use a cellular noise function to create the plant cell pattern
    let cell_noise_value = uniforms.noises[0]
        .get_noise_2d(x * zoom + ox, y * zoom + oy)
        .abs();

    // Define different shades of green for the plant cells
    let cell_color_1 = Color::new(85, 107, 47); // Dark olive green
    let cell_color_2 = Color::new(124, 252, 0); // Light green
    let cell_color_3 = Color::new(34, 139, 34); // Forest green
    let cell_color_4 = Color::new(173, 255, 47); // Yellow green

    // Use the noise value to assign a different color to each cell
    let final_color = if cell_noise_value < 0.15 {
        cell_color_1
    } else if cell_noise_value < 0.7 {
        cell_color_2
    } else if cell_noise_value < 0.75 {
        cell_color_3
    } else {
        cell_color_4
    };

    // Adjust intensity to simulate lighting effects (optional)
    final_color * fragment.intensity
}

fn lava_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    // Base colors for the lava effect
    let bright_color = Color::new(255, 240, 0); // Bright orange (lava-like)
    let dark_color = Color::new(130, 20, 0); // Darker red-orange

    // Get fragment position
    let position = Vec3::new(
        fragment.vertex_position.x,
        fragment.vertex_position.y,
        fragment.depth,
    );

    // Base frequency and amplitude for the pulsating effect
    let base_frequency = 0.2;
    let pulsate_amplitude = 0.5;
    let t = uniforms.time as f32 * 0.01;

    // Pulsate on the z-axis to change spot size
    let pulsate = (t * base_frequency).sin() * pulsate_amplitude;

    // Apply noise to coordinates with subtle pulsating on z-axis
    let zoom = 1000.0; // Constant zoom factor
    let noise_value1 = uniforms.noises[0].get_noise_3d(
        position.x * zoom,
        position.y * zoom,
        (position.z + pulsate) * zoom,
    );
    let noise_value2 = uniforms.noises[0].get_noise_3d(
        (position.x + 1000.0) * zoom,
        (position.y + 1000.0) * zoom,
        (position.z + 1000.0 + pulsate) * zoom,
    );
    let noise_value = (noise_value1 + noise_value2) * 0.5; // Averaging noise for smoother transitions

    // Use lerp for color blending based on noise value
    let color = dark_color.lerp(&bright_color, noise_value);

    color * fragment.intensity
}

pub fn shader_earth(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    // Posición y normal del fragmento
    let position = fragment.vertex_position;
    let normal = fragment.normal.normalize();

    // Iluminación
    let light_pos = Vec3::new(0.0, 0.0, 20.0);
    let light_dir = (light_pos - position).normalize();
    let diffuse_intensity = normal.dot(&light_dir).max(0.0);

    // Variable de tiempo para animación
    let time = uniforms.time * 0.0001;

    // Parámetros de umbral
    let land_threshold = 0.5;
    let cloud_threshold = 0.7;

    // Colores base
    let water_color = Color::from_float(0.0, 0.1, 0.4); // Darker, more realistic water color
    let land_color = Color::from_float(0.3, 0.5, 0.2); // More muted, natural land color
    let cloud_color: Color = Color::from_float(0.6, 0.6, 0.6);

    // Velocidades de movimiento
    let land_speed = 0.01;
    let cloud_speed = 0.03;

    // Obtener referencias a los ruidos
    let land_noise = uniforms.noises[0];
    let cloud_noise = uniforms.noises[1];

    // Ruido para la tierra (movimiento lento)
    let land_noise_value = land_noise.get_noise_3d(
        position.x + time * land_speed,
        position.y + time * land_speed,
        position.z + time * land_speed,
    );
    let land_normalized = (land_noise_value + 1.0) * 0.5;

    // Determinar si el fragmento es tierra o agua
    let is_land = land_normalized > land_threshold;

    // Color base según sea tierra o agua
    let mut base_color = if is_land { land_color } else { water_color };

    // Ruido para las nubes (movimiento más lento)
    let cloud_noise_value = cloud_noise.get_noise_3d(
        position.x + time * cloud_speed,
        position.y + time * cloud_speed,
        position.z + time * cloud_speed,
    );
    let cloud_normalized = (cloud_noise_value + 1.5) * 0.5;

    // Opacidad de las nubes
    let cloud_opacity =
        ((cloud_normalized - cloud_threshold) / (1.0 - cloud_threshold)).clamp(0.0, 1.0);

    // Mezclar las nubes con el color base
    base_color = base_color.lerp(&cloud_color, cloud_opacity);

    // Aplicar iluminación
    let lit_color = base_color * diffuse_intensity;
    let ambient_intensity = 0.3;
    let ambient_color = base_color * ambient_intensity;
    let final_color = ambient_color + lit_color;

    final_color.clamp()
}

pub fn shader_jupiter(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    // Posición y normal del fragmento
    let position = fragment.vertex_position;
    let normal = fragment.normal.normalize();

    // Iluminación
    let light_pos = Vec3::new(0.0, 0.0, 20.0);
    let light_dir = (light_pos - position).normalize();
    let diffuse_intensity = normal.dot(&light_dir).max(0.0);

    // Ruido para bandas atmosféricas
    let noise_value = uniforms.noises[0].get_noise_3d(position.x, position.y, position.z);
    let normalized_value = (noise_value + 1.0) * 0.5;

    // Definir colores para las bandas de Júpiter
    let color1 = Color::from_float(0.804, 0.522, 0.247); // Marrón claro
    let color2 = Color::from_float(0.870, 0.721, 0.529); // Beige

    // Usar `lerp` para interpolar entre los colores de las bandas
    let base_color = color1.lerp(&color2, normalized_value);

    // Aplicar iluminación difusa al color base
    let lit_color = base_color * diffuse_intensity;

    // Añadir un término ambiental
    let ambient_intensity = 0.1;
    let ambient_color = base_color * ambient_intensity;

    // Combinar los componentes ambiental y difuso
    let final_color = ambient_color + lit_color;

    // Asegurar que los valores de color estén en el rango válido
    final_color.clamp()
}
