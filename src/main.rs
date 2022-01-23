#![allow(mixed_script_confusables)]

use nannou::prelude::*;

fn main() {
    Model::run();
}

/// This represents the state of our art-piece
/// It might be more useful to give it a more descriptive name
struct Model {
    moving_rectangle: MovingRectangle,
}

/// A simple type for holding our moving rectangle
struct MovingRectangle {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    θ: f32,
}

struct Point {
    x: f32,
    y: f32,
}

struct DeJongAttractor {
    position: Point,
    a: f32,
    b: f32,
    c: f32,
    d: f32,
}

impl DeJongAttractor {
    fn new(initial_position: Point, a: f32, b: f32, c: f32, d: f32) -> Self {
        Self {
            position: initial_position,
            a,
            b,
            c,
            d,
        }
    }

    fn next(&mut self) -> Point {
        // do the sin/cos business
        self.position.x = (self.a * self.position.y).sin() - (self.b * self.position.x).cos();
        self.position.y = (self.c * self.position.x).sin() - (self.d * self.position.y).cos();

        Point {
            x: self.position.x,
            y: self.position.y,
        }
    }
}

/// I elected to contain all the methods in the model
impl Model {
    /// nannou::app takes a function that, given a nannou::App, returns a Model
    fn setup(app: &App) -> Self {
        println!("LoopMode: {:?}", app.loop_mode());

        Self {
            moving_rectangle: MovingRectangle {
                x: 0.,
                y: 0.,
                w: 30.,
                h: 30.,
                θ: 0.,
            },
        }
    }

    // Called 60 times per second
    fn update(app: &App, model: &mut Self, _update: Update) {
        // update some things about the rectangle

        // for now, just increment the angle
        //model.moving_rectangle.θ += 0.08;

        // We can also use the apps time and pass it to sin.
        let sin = app.time.sin();
        let slowsin = (app.time / 2.).sin();
        let fastsin = (app.time * 2.).sin();

        // Oh, we can use map_range to scale things nicely.
        model.moving_rectangle.w = map_range(sin, -1., 1., 10., 100.);
        model.moving_rectangle.h = map_range(slowsin, -1., 1., 10., 100.);

        // ok more interesting rotations
        model.moving_rectangle.θ = map_range(fastsin, -1., 1., -15.2, 15.2);
    }

    // Draw to frame
    fn view(app: &App, model: &Self, frame: Frame) {
        let draw = app.draw();
        frame.clear(PURPLE);

        // Draw a rect using model state (which is updated in the update function)
        draw.rect()
            .x_y(model.moving_rectangle.x, model.moving_rectangle.y)
            .w_h(model.moving_rectangle.w, model.moving_rectangle.h)
            .rotate(model.moving_rectangle.θ)
            .color(PLUM);

        // Nothing appears until we send it to the frame
        draw.to_frame(app, &frame).unwrap()
    }

    /// A helper function to call nannou with the above methods
    pub fn run() {
        nannou::app(Self::setup)
            // .event(event) // generic event handler
            .update(Self::update)
            .simple_window(Self::view)
            .run()
    }
}
