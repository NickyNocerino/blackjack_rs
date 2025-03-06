pub mod game;
pub mod opt_bj_game;

use pyo3::prelude::*;
use opt_bj_game::OptimizedBlackJackGame;

#[pymodule]
fn blackjack_rs(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<OptimizedBlackJackGame>()?;
    Ok(())
}