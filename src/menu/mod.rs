mod completion_menu;
mod history_menu;

use crate::{painter::Painter, Completer, History, LineBuffer, Span};
pub use completion_menu::CompletionMenu;
pub use history_menu::HistoryMenu;
use nu_ansi_term::{Color, Style};

/// Struct to store the menu style
struct MenuTextStyle {
    selected_text_style: Style,
    text_style: Style,
}

impl Default for MenuTextStyle {
    fn default() -> Self {
        Self {
            selected_text_style: Color::Green.bold().reverse(),
            text_style: Color::DarkGray.normal(),
        }
    }
}

/// Defines all possible events that could happen with a menu.
pub enum MenuEvent {
    /// Activation event for the menu. When the bool is true it means that the values
    /// have already being updated. This is true when the option `quick_completions` is true
    Activate(bool),
    /// Deactivation event
    Deactivate,
    /// Line buffer edit event. When the bool is true it means that the values
    /// have already being updated. This is true when the option `quick_completions` is true
    Edit(bool),
    /// Selecting next element in the menu
    NextElement,
    /// Selecting previous element in the menu
    PreviousElement,
    /// Moving up in the menu
    MoveUp,
    /// Moving down in the menu
    MoveDown,
    /// Moving left in the menu
    MoveLeft,
    /// Moving right in the menu
    MoveRight,
    /// Move to next page
    NextPage,
    /// Move to previous page
    PreviousPage,
}

/// Trait that defines how a menu will be printed by the painter
pub trait Menu: Send {
    /// Menu name
    fn name(&self) -> &str;

    /// Menu indicator
    fn indicator(&self) -> &str {
        "% "
    }

    /// Checks if the menu is active
    fn is_active(&self) -> bool;

    /// Selects what type of event happened with the menu
    fn menu_event(&mut self, event: MenuEvent);

    /// Updates the values presented in the menu
    /// This function needs to be defined in the trait because when the menu is
    /// activated or the quick_completion option is true, the len of the values
    /// is calculated to know if there is only one value so it can be selected
    /// immediately
    fn update_values(
        &mut self,
        line_buffer: &mut LineBuffer,
        history: &dyn History,
        completer: &dyn Completer,
    );

    /// The working details of a menu are values that could change based on
    /// the menu conditions before it being printed, such as the number or size
    /// of columns, etc.
    /// In this function should be defined how the menu event is treated since
    /// it is called just before painting the menu
    fn update_working_details(
        &mut self,
        line_buffer: &mut LineBuffer,
        history: &dyn History,
        completer: &dyn Completer,
        painter: &Painter,
    );

    /// Indicates how to replace in the line buffer the selected value from the menu
    fn replace_in_buffer(&self, line_buffer: &mut LineBuffer);

    /// Calculates the real required lines for the menu considering how many lines
    /// wrap the terminal or if entries have multiple lines
    fn menu_required_lines(&self, terminal_columns: u16) -> u16;

    /// Creates the menu representation as a string which will be painted by the painter
    fn menu_string(&self, available_lines: u16, use_ansi_coloring: bool) -> String;

    /// Minimum rows that should be displayed by the menu
    fn min_rows(&self) -> u16;

    /// Gets cached values from menu that will be displayed
    fn get_values(&self) -> &[(Span, String)];
}
