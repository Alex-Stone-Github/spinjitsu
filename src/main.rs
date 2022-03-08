const SCR_SIZE: i32 = 30;

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
    let index: i32 = y * SCR_SIZE + x;
    framebuffer[index as usize] = d;
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

fn main() {
    let mut framebuffer: [char;(SCR_SIZE*SCR_SIZE*2) as usize] = ['.';(SCR_SIZE*SCR_SIZE*2) as usize];


    plot_pixel(&mut framebuffer, 2, 2, '@');
    plot_rect(&mut framebuffer, 3, 3, 3, 3, '%');
    plot_line(&mut framebuffer, 0, 0, 30, 30, '$');


    clear_screen();
    print_framebuffer(&framebuffer);
    println!("Hello, world!");
}
