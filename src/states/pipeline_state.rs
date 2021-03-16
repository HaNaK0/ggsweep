use std::time::Duration;

use ggez::{conf, filesystem, graphics, input::keyboard, timer, GameResult};
use log::{error, info};

use crate::{
    config::{GameConfig, PipelineConfig},
    err_here,
    error::LocatedError,
    state::{EventResult, State, UpdateResult},
};

/// # Gen Progress
/// An enum used to keep track on what the pipeline is currently doing
enum GenProgress {
    Setup,
    Draw,
    Show,
    Save,
    Done,
}

const PAUSE: bool = true;

/// # Pipeline State
/// A state that will go through assets that needs to transformed before it
/// be used in game   
/// For ggsweep it will generate the sprite used for the numbers
pub struct PipelineState {
    game_config: GameConfig,
    font: ggez::graphics::Font,
    text_size: f32,
    target_path: String,
    progress: GenProgress,
    timer: Duration,
    number_render: Option<NumberRender>,
}

/// # Number render
/// A struct that contains the information needed for prerendering text to a canvas
struct NumberRender {
    canvas: graphics::Canvas,
    text: Vec<graphics::Text>,
}

impl PipelineState {
    /// # New
    /// Create a new pipeline state
    pub fn new(
        gen_conf_path: &str,
        ctx: &mut ggez::Context,
    ) -> Result<PipelineState, LocatedError> {
        info!("Pipline load stated");
        let load_start = timer::time_since_start(ctx);

        let rdr = filesystem::open(ctx, gen_conf_path).map_err(err_here!())?;
        let pipe_config: PipelineConfig = ron::de::from_reader(rdr).unwrap();

        let rdr = filesystem::open(ctx, &pipe_config.game_config_path).map_err(err_here!())?;
        let game_config = ron::de::from_reader(rdr).unwrap();

        let font = graphics::Font::new(ctx, pipe_config.font).map_err(err_here!())?;
        let target = pipe_config.target;
        let text_size = pipe_config.size;

        let load_end = timer::time_since_start(ctx);
        let time: Duration = load_end - load_start;
        info!("Loaded pipline, it took {} seconds", time.as_secs());

        Ok(PipelineState {
            game_config,
            font,
            text_size,
            target_path: target,
            progress: GenProgress::Setup,
            timer: Duration::default(),
            number_render: None,
        })
    }

    /// # Setup
    /// Create stuff needed for rendering
    fn setup(&mut self, ctx: &mut ggez::Context) -> GameResult<()> {
        info!("Pipline started");
        self.timer = timer::time_since_start(ctx);

        let canvas_size = self.game_config.square_size * 3.0;
        info!("creating canvas of size {:?}", canvas_size);

        let canvas = graphics::Canvas::new(
            ctx,
            canvas_size as u16,
            canvas_size as u16,
            conf::NumSamples::One,
        )?;

        let mut text = Vec::with_capacity(10);
        for i in 0..10 {
            let fragment = graphics::TextFragment::new(i.to_string())
                .color(graphics::WHITE)
                .font(self.font)
                .scale(graphics::Scale::uniform(self.text_size));

            text.push(graphics::Text::new(fragment));
        }

        self.number_render = Some(NumberRender { canvas, text });

        self.progress = GenProgress::Draw;
        Ok(())
    }

    /// # Save
    /// Save stuff that should be saved to a file
    fn save(&mut self, ctx: &mut ggez::Context) -> GameResult<()> {
        if let Some(number_render) = &self.number_render {
            let image = &number_render.canvas.image();

            info!("Rendering to: {}", self.target_path);
            image.encode(ctx, graphics::ImageFormat::Png, &self.target_path)?;
            self.progress = GenProgress::Done;
        } else {
            error!(
                "{}, {}: No number_render at pipeline save. Can't save nothing",
                file!(),
                line!()
            );
            return Err(ggez::GameError::RenderError(
                "Missing number_render".to_string(),
            ));
        }

        Ok(())
    }
}

impl State for PipelineState {
    /// # Update
    /// Update does different things depending on the gen progress
    /// - If ```progress``` is Setup it runs the ```setup``` method
    /// - If ```progress``` is Draw it does nothing
    /// - If ```progress``` is Save it runs the ```save``` method that saves the result to file
    /// - If ```progress``` is Done it cleans up and then returns pop
    fn update(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult<UpdateResult> {
        match self.progress {
            GenProgress::Setup => {
                self.setup(ctx)?;
                Ok(UpdateResult::Block)
            }
            GenProgress::Draw => Ok(UpdateResult::Block),
            GenProgress::Save => {
                self.save(ctx)?;
                Ok(UpdateResult::Block)
            }
            GenProgress::Show => Ok(UpdateResult::Block),
            GenProgress::Done => {
                let time_elapsed: Duration = timer::time_since_start(ctx) - self.timer;
                info!("Pipeline done, took {} seconds", time_elapsed.as_secs());
                Ok(UpdateResult::Pop)
            }
        }
    }

    /// # Draw
    /// Runs drawing functions that draws to canvas that later gets saved
    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult<()> {
        if let (GenProgress::Draw, Some(n_render)) = (&self.progress, &self.number_render) {
            graphics::set_canvas(ctx, Some(&n_render.canvas));

            for (i, text) in n_render.text.iter().enumerate() {
                let x = (i % 3) as f32 * self.game_config.square_size
                    + self.game_config.square_size / 2.0;
                let y = (i / 3) as f32 * self.game_config.square_size
                    + self.game_config.square_size / 2.0;

                let param = graphics::DrawParam::new()
                    .offset(cgmath::Point2::new(0.5, 0.5))
                    .dest(cgmath::point2(x, y));

                graphics::draw(ctx, text, param)?;
            }

            graphics::set_canvas(ctx, None);

            // temporary way might change to something better later
            if PAUSE {
                self.progress = GenProgress::Show;
            } else {
                self.progress = GenProgress::Save;
            }
        } else if let (GenProgress::Draw, None) = (&self.progress, &self.number_render) {
            error!(
                "{}, {}: Pipeline entered draw without number_render.",
                file!(),
                line!()
            );
            return Err(ggez::GameError::RenderError(
                "Missing number_render".to_string(),
            ));
        } else if let (GenProgress::Show, Some(n_render)) = (&self.progress, &self.number_render) {
            graphics::draw(ctx, &n_render.canvas, graphics::DrawParam::default())?;
        }

        Ok(())
    }

    fn key_up_event(
        &mut self,
        _ctx: &mut ggez::Context,
        keycode: keyboard::KeyCode,
        _keymods: keyboard::KeyMods,
    ) -> ggez::GameResult<EventResult> {

        if let GenProgress::Show = &self.progress {

            if let keyboard::KeyCode::Return = keycode {
                self.progress = GenProgress::Save

            } else if let keyboard::KeyCode::Escape = keycode {
                self.progress = GenProgress::Done

            }
        }

        Ok(EventResult::LetThrough)
    }
}
