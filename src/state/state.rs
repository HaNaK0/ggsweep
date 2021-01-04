///The result returned from an event handling function
pub enum EventResult{
	/// Let the event through to the state below this one
	LetThrough,
	/// Block this event from passing to the state below this one
	Block,
}

///Enum returned from the update function can also tell the state stack to push or swap a state
pub enum UpdateResult {
	/// Update the state below this one in the stack
	LetThrough,
	/// Block the state below this one to update
	Block,
	/// Swap this state for the returned state
	Swap(Box<dyn State>),
	/// Push the contained state to the top of the stack
	Push(Box<dyn State>),
}

/// A state in a state stack   
/// Handles the events passed to it   
/// By default it will block unimplemented events from passing through   
pub trait State {
	/// Called called in main_state update
	fn update(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult<UpdateResult>;

	/// Called in main_state draw does not return a result but the MainState uses the let_through_draw to check wether to draw the underlying states
	fn draw(&mut self, _ctx: &mut ggez::Context) -> ggez::GameResult<()>;

	/// Called by mainstate draw to check wether the state blow this one should be drawn
	fn let_through_draw(&mut self) -> bool {
		false
	}
}