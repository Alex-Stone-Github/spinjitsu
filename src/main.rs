use std::{thread, time};
const SCR_SIZE: i32 = 30;
const VERTEX_COUNT: usize = 12;
const SHAPE_SIZE: f32 = 10.0;
const FPS: i32 = 15;
const DT: i32 = 1000 / FPS;
/* Hey, stupid, remeber you are using
 * radians, so don't use treason 
 * units
 */


#[derive(Copy, Clone)]
struct Vertex {
    x: f32,
    y: f32,
    z: f32,
}
impl Vertex {
    pub fn new(x: f32, y: f32, z:f32) -> Self {
        Self {
            x: x,
            y: y,
            z: z
        }
    }
}

fn clear_screen() {
    // or just you know move the cursor to 1;1
    print!("{escape}[2J{escape}[1;1H", escape = 27 as char);
}
fn print_framebuffer(framebuffer: &[char]) {
    for i in 0..SCR_SIZE {
        for j in 0..SCR_SIZE {
            print!("{}", framebuffer[(i * SCR_SIZE + j) as usize]);
            print!("{}", framebuffer[(i * SCR_SIZE + j) as usize]);
        }
        println!();
    }
}
fn plot_pixel(framebuffer: &mut[char], x: i32, y: i32, d: char) {
    if x > -1 && x < SCR_SIZE && y > -1 && y < SCR_SIZE {
        let index: i32 = y * SCR_SIZE + x;
        framebuffer[index as usize] = d;
    }
}
fn plot_rect(framebuffer: &mut[char], x: i32, y: i32, w: i32, h: i32, d: char) {
    for _x in x..x+w {
        for _y in y..y+h {
            plot_pixel(framebuffer, _x, _y, d);
        }
    }
}
fn lerp(x1: f32, y1: f32, x2: f32, y2: f32, t: f32) -> (f32, f32) {
    (x1 + (x2 - x1) * t, y1 + (y2 - y1) * t)
}
fn plot_line(framebuffer: &mut[char], x1: i32, y1: i32, x2: i32, y2: i32, d: char) {
    const LINE_STEPS: i32 = 100;

    for i in 0..LINE_STEPS {
        let (x, y) = lerp(x1 as f32, y1 as f32, x2 as f32, y2 as f32, (i as f32 / LINE_STEPS as f32) as f32);
        plot_pixel(framebuffer, x as i32, y as i32, d);
    }
}
fn plot_triangle(framebuffer: &mut[char], x1: i32, y1: i32, x2: i32, y2: i32, x3: i32, y3: i32, d: char) {
    plot_line(framebuffer, x1, y1, x2, y2, d);
    plot_line(framebuffer, x2, y2, x3, y3, d);
    plot_line(framebuffer, x3, y3, x1, y1, d);
}
fn sleep(ms: i32) {
    let duration = time::Duration::from_millis(ms as u64);
    thread::sleep(duration);
}
fn translate(input: &Vertex, translation: &Vertex) -> Vertex {
    Vertex::new(
        input.x + translation.x,
        input.y + translation.y,
        input.z + translation.z
    )
}
fn rotateX(input: &Vertex, angler: f32) -> Vertex {
    // I worked out the math of a 2d rotation matrix here
    Vertex::new(
        input.x,
        angler.cos() * input.y + -angler.sin() * input.z,
        angler.sin() * input.y + angler.cos() * input.z
    )
}
fn rotateY(input: &Vertex, angler: f32) -> Vertex {
    // I worked out the math of a 2d rotation matrix here
    Vertex::new(
        angler.cos() * input.x + angler.sin() * input.z,
        input.y,
        -angler.sin() * input.x + angler.cos() * input.z
    )
}
fn rotateZ(input: &Vertex, angler: f32) -> Vertex {
    // I worked out the math of a 2d rotation matrix here
    Vertex::new(
        angler.cos() * input.x + -angler.sin() * input.y,
        angler.sin() * input.x + angler.cos() * input.y,
        input.z
    )
}
fn orthographic(input: &Vertex) -> Vertex {
    Vertex::new(
        input.x,
        input.y,
        0.0
    )
}

fn main() {
    let mut framebuffer: [char;(SCR_SIZE*SCR_SIZE*2) as usize] = ['.';(SCR_SIZE*SCR_SIZE*2) as usize];
    let vertices: [Vertex;VERTEX_COUNT] = [
        Vertex::new(-SHAPE_SIZE,  -SHAPE_SIZE, -SHAPE_SIZE), // front
        Vertex::new(-SHAPE_SIZE,  SHAPE_SIZE,  -SHAPE_SIZE),
        Vertex::new(SHAPE_SIZE,   -SHAPE_SIZE, -SHAPE_SIZE),
        Vertex::new(SHAPE_SIZE,   -SHAPE_SIZE, -SHAPE_SIZE),
        Vertex::new(SHAPE_SIZE,   SHAPE_SIZE,  -SHAPE_SIZE),
        Vertex::new(-SHAPE_SIZE,  SHAPE_SIZE,  -SHAPE_SIZE),
        Vertex::new(-SHAPE_SIZE,  -SHAPE_SIZE, SHAPE_SIZE), // back
        Vertex::new(-SHAPE_SIZE,  SHAPE_SIZE,  SHAPE_SIZE),
        Vertex::new(SHAPE_SIZE,   -SHAPE_SIZE, SHAPE_SIZE),
        Vertex::new(SHAPE_SIZE,   -SHAPE_SIZE, SHAPE_SIZE),
        Vertex::new(SHAPE_SIZE,   SHAPE_SIZE,  SHAPE_SIZE),
        Vertex::new(-SHAPE_SIZE,  SHAPE_SIZE,  SHAPE_SIZE)
    ];
    let mut output_vertices: [Vertex;VERTEX_COUNT] = [Vertex::new(0.0,0.0,0.0);VERTEX_COUNT];
    let mut angle: f32 = 0.0;
    loop {
        angle += 0.1;
        let translation: Vertex = Vertex::new(SCR_SIZE as f32/2.0, SCR_SIZE as f32/2.0, SHAPE_SIZE as f32 * 2.0);
        for i in 0..vertices.len() {
            output_vertices[i] = orthographic(&translate(&rotateZ(&rotateY(&rotateX(&vertices[i], angle), angle), angle), &translation));
        }
        //background
        plot_rect(&mut framebuffer, 0, 0, SCR_SIZE, SCR_SIZE, '.');

        // a square
        plot_triangle(&mut framebuffer, 
                      output_vertices[0].x as i32, output_vertices[0].y as i32,
                      output_vertices[1].x as i32, output_vertices[1].y as i32,
                      output_vertices[2].x as i32, output_vertices[2].y as i32,
                      '5');
        plot_triangle(&mut framebuffer, 
                      output_vertices[3].x as i32, output_vertices[3].y as i32,
                      output_vertices[4].x as i32, output_vertices[4].y as i32,
                      output_vertices[5].x as i32, output_vertices[5].y as i32,
                      '5');
        plot_triangle(&mut framebuffer, 
                      output_vertices[6].x as i32, output_vertices[6].y as i32,
                      output_vertices[7].x as i32, output_vertices[7].y as i32,
                      output_vertices[8].x as i32, output_vertices[8].y as i32,
                      '5');
        plot_triangle(&mut framebuffer, 
                      output_vertices[9].x as i32, output_vertices[9].y as i32,
                      output_vertices[10].x as i32, output_vertices[10].y as i32,
                      output_vertices[11].x as i32, output_vertices[11].y as i32,
                      '5');
        // drawing
        clear_screen();
        print_framebuffer(&framebuffer);
        sleep(DT);
    }
}
/* 
 * If anyone decides to look at this, sorry about your eyes. This is bad code.
 */

