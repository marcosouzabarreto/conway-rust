use raylib::prelude::*;

const ROWS: usize = 48;
const COLUMNS: usize = 64;

const SCREEN_HEIGHT: i32 = 960;
const SCREEN_WIDTH: i32 = 1280;

const CELL_WIDTH: i32 = SCREEN_WIDTH / COLUMNS as i32;
const CELL_HEIGHT: i32 = SCREEN_HEIGHT / ROWS as i32;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Conway's Game of Life")
        .build();

    let mut state = init_state();

    while !rl.window_should_close() {
        rl.set_target_fps(10);
        on_update(&mut rl, &thread, &mut state);
    }
}

fn init_state() -> [[u8; COLUMNS]; ROWS] {
    let mut state = [[0u8; COLUMNS]; ROWS];

    state[0][1] = 1;
    state[1][2] = 1;
    state[2][0] = 1;
    state[2][1] = 1;
    state[2][2] = 1;

    state
}

fn on_update(rl: &mut RaylibHandle, thread: &RaylibThread, state: &mut [[u8; COLUMNS]; ROWS]) {
    let mut d = rl.begin_drawing(&thread);
    print_state(&mut d, &state);
    update_state(state);
}

fn update_state(state: &mut [[u8; COLUMNS]; ROWS]) {
    let mut temp_state = [[0u8; COLUMNS]; ROWS];
    for i in 0..ROWS {
        for j in 0..COLUMNS {
            let neighbors = get_neighbors(j as isize, i as isize, &state);
            temp_state[i][j] = match (state[i][j], neighbors) {
                (1, x) if x < 2 => 0,
                (1, 2) | (1, 3) => 1,
                (_, x) if x > 3 => 0,
                (0, 3) => 1,
                (_, _) => state[i][j],
            }
        }
    }
    *state = temp_state;
}

fn get_neighbors(x: isize, y: isize, state: &[[u8; COLUMNS]; ROWS]) -> usize {
    let mut res = 0;
    let directions: [(isize, isize); 8] = [
        (-1, 1),
        (0, 1),
        (1, 1),
        (1, 0),
        (1, -1),
        (0, -1),
        (-1, -1),
        (-1, 0),
    ];

    for direction in directions {
        let (x_offset, y_offset) = direction;
        let target_x = x + x_offset;
        let target_y = y + y_offset;

        if target_y < 0 || target_y > ROWS as isize - 1 {
            continue;
        }
        if target_x < 0 || target_x > COLUMNS as isize - 1 {
            continue;
        }
        let neighbor = state[target_y as usize][target_x as usize];
        if neighbor == 1 {
            res += 1;
        }
    }
    res
}

fn print_state(d: &mut RaylibDrawHandle, state: &[[u8; COLUMNS]; ROWS]) {
    for i in 0..ROWS {
        for j in 0..COLUMNS {
            let x = j as i32 * CELL_WIDTH;
            let y = i as i32 * CELL_HEIGHT;

            let mut color: Color = Color::DARKGRAY;

            if state[i][j] == 1 {
                color = Color::RED;
            }

            d.draw_rectangle(x, y, CELL_WIDTH, CELL_HEIGHT, color);
            d.draw_rectangle_lines(x, y, CELL_WIDTH, CELL_HEIGHT, Color::GRAY);
        }
    }
}
