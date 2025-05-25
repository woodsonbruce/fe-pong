use macroquad::prelude::*;

const BALL_COLOR: Color = RED;
const WALL_COLOR: Color = BLUE;
const PADDLE_COLOR: Color = WHITE;
const BALL_SIZE: f32 = 12.0;
const WALL_THICKNESS: f32 = 12.0;
const BALL_SPEED: f32 = 400.0;
const PADDLE_HEIGHT: f32 = 50.0;
const PADDLE_THICKNESS: f32 = 12.0;
const PADDLE_SPEED: f32 = 200.0;
const LEFT_BALL_RESET_X: f32 = 0.05;
const RIGHT_BALL_RESET_X: f32 = 0.95;
const BALL_ESTIMATION_X: f32 = 0.2;
const LEFT_PADDLE_FACE_X: f32 = 0.1;
const RIGHT_PADDLE_FACE_X: f32 = 0.9;

struct Ball {
    x: f32,
    y: f32,
    x_step: f32,
    y_step: f32,
    size: f32,
    color: Color,
    projected: bool,
}

impl Ball {
    fn new(x: f32, y: f32, size: f32, color: Color) -> Ball {
        Ball {
            x,
            y,
            x_step: BALL_SPEED * get_frame_time() * rand::gen_range(0.5, 1.0),
            y_step: BALL_SPEED * get_frame_time() * rand::gen_range(-1.0, 1.0),
            size,
            color,
            projected: false,
        }
    }

    fn draw(&self) {
        draw_circle(self.x, self.y, self.size, self.color);
    }

    fn update_position(&mut self) {
        self.x += self.x_step;
        self.y += self.y_step;
    }

    fn reset(&mut self) {
        self.x = screen_width() / 2.0;
        self.y = screen_height() / 2.0;
        self.x_step = BALL_SPEED * get_frame_time() * rand::gen_range(0.5, 1.0);
        self.y_step = BALL_SPEED * get_frame_time() * rand::gen_range(-1.0, 1.0);
        self.projected = false;
    }
}

struct Wall {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
}

impl Wall {
    fn new(x: f32, y: f32, w: f32, h: f32) -> Wall {
        Wall { x, y, w, h }
    }

    fn draw(&self) {
        draw_rectangle(self.x, self.y, self.w, self.h, WALL_COLOR);
    }

    fn intersected_from_below_by(&self, ball: &Ball) -> bool {
        ball.y < (self.y + self.h + BALL_SIZE)
            && ball.x > (self.x - BALL_SIZE)
            && ball.x < (self.x + self.w + BALL_SIZE)
    }

    fn intersected_from_above_by(&self, ball: &Ball) -> bool {
        ball.y > (self.y - BALL_SIZE)
            && ball.x > (self.x - BALL_SIZE)
            && ball.x < (self.x + self.w + BALL_SIZE)
    }
}

struct Paddle {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
}

impl Paddle {
    fn new(x: f32, y: f32, w: f32, h: f32) -> Paddle {
        Paddle { x, y, w, h }
    }

    fn draw(&self) {
        draw_rectangle(self.x, self.y, self.w, self.h, PADDLE_COLOR);
    }

    fn intersected_from_right_by(&self, ball: &Ball) -> bool {
        ball.x < (self.x + self.w + BALL_SIZE)
            && ball.x > (self.x + self.w + BALL_SIZE - PADDLE_THICKNESS) // prevent resonance behind paddle
            && ball.y > (self.y - BALL_SIZE)
            && ball.y < (self.y + self.h + BALL_SIZE)
    }

    fn intersected_from_left_by(&self, ball: &Ball) -> bool {
        ball.x > (self.x - BALL_SIZE)
            && ball.x < (self.x - BALL_SIZE + PADDLE_THICKNESS) // prevent resonance
            && ball.y > (self.y - BALL_SIZE)
            && ball.y < (self.y + self.h + BALL_SIZE)
    }
}

#[macroquad::main("Pong")]
async fn main() {
    rand::srand(miniquad::date::now() as u64);

    let mut ball = Ball::new(
        screen_width() / 2.0,
        screen_height() / 2.0,
        BALL_SIZE,
        BALL_COLOR,
    );
    let mut player_paddle = Paddle::new(
        RIGHT_PADDLE_FACE_X * screen_width(),
        (screen_height() - PADDLE_HEIGHT) / 2.0,
        PADDLE_THICKNESS,
        PADDLE_HEIGHT,
    );
    let mut program_paddle = Paddle::new(
        LEFT_PADDLE_FACE_X * screen_width() - PADDLE_THICKNESS,
        (screen_height() - PADDLE_HEIGHT) / 2.0,
        PADDLE_THICKNESS,
        PADDLE_HEIGHT,
    );
    let top_wall = Wall::new(0.0, 0.0, screen_width(), WALL_THICKNESS);
    let bottom_wall = Wall::new(
        0.0,
        screen_height() - WALL_THICKNESS,
        screen_width(),
        WALL_THICKNESS,
    );

    let mut paddle_increment = 0.0;
    loop {

        let delta_time = get_frame_time();

        top_wall.draw();
        bottom_wall.draw();
        program_paddle.draw();

        if is_key_down(KeyCode::Down)
            && player_paddle.y < (screen_height() - player_paddle.h - WALL_THICKNESS)
        {
            player_paddle.y += PADDLE_SPEED * delta_time;
        }
        if is_key_down(KeyCode::Up) && player_paddle.y > WALL_THICKNESS {
            player_paddle.y -= PADDLE_SPEED * delta_time;
        }

        player_paddle.draw();

        if top_wall.intersected_from_below_by(&mut ball)
            || bottom_wall.intersected_from_above_by(&mut ball)
        {
            ball.y_step = -ball.y_step;
        }

        if ball.x_step > 0.0 && player_paddle.intersected_from_left_by(&mut ball) {
            ball.x_step = -ball.x_step;
        }

        if ball.x_step < 0.0 && ball.x < BALL_ESTIMATION_X * screen_width() && !ball.projected {
            let ball_delta = (LEFT_PADDLE_FACE_X - BALL_ESTIMATION_X) * screen_width() * ball.y_step / ball.x_step;
            let ball_projected_position = ball.y + ball_delta;
            let paddle_delta = ball_projected_position - program_paddle.y - program_paddle.h / 2.0;
            paddle_increment = ball.y_step * paddle_delta / ball_delta;
            ball.projected = true;
        }

        if ball.projected {
            program_paddle.y += paddle_increment;
        }

        if ball.x_step < 0.0 && program_paddle.intersected_from_right_by(&mut ball) {
            ball.x_step = -ball.x_step;
            ball.projected = false;
        }

        if ball.x < LEFT_BALL_RESET_X * screen_width() || ball.x > RIGHT_BALL_RESET_X * screen_width() {
            ball.reset();
            program_paddle.y = (screen_height() - PADDLE_HEIGHT) / 2.0;
        }

        ball.update_position();
        ball.draw();
        next_frame().await;
    }
}
