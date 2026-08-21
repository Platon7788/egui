// This module is only public with the `experimental_theme` feature,
// so without it a lot of it looks unused:
#![cfg_attr(not(feature = "experimental"), allow(dead_code, unused_imports))]

mod classes;

pub use self::classes::{
    ClassName, Classes, HasClasses, MENU_CLASS, READ_ONLY_CLASS, ROOT_CLASS, SELECTED_CLASS,
};

use core::fmt::Debug;

use emath::Vec2;
use epaint::{Color32, FontId, Stroke, text::TextWrapMode};

use crate::{
    Context, Frame, Response, Style, TextStyle, UiStack,
    style::{WidgetVisuals, Widgets},
};

/// Each dedicated style must implement this trait to be used in the theme plugin system
pub trait WidgetStyle: Debug + Clone + Send + Sync + core::any::Any + 'static {}

/// General text style
#[derive(Debug, Clone)]
pub struct TextVisuals {
    /// Font used
    pub font_id: FontId,

    /// Font color
    pub color: Color32,

    /// Text decoration
    pub underline: Stroke,
    pub strikethrough: Stroke,
}

impl TextVisuals {
    /// The undecorated body text of a [`Style`], as a starting point for a widget's own text.
    ///
    /// The color is the style's ordinary text color; a widget that paints its text in a color of
    /// its own overrides it.
    pub fn from_style(style: &Style) -> Self {
        Self {
            color: style.visuals.text_color(),
            font_id: style
                .override_font_id
                .clone()
                .unwrap_or_else(|| TextStyle::Body.resolve(style)),
            underline: Stroke::NONE,
            strikethrough: Stroke::NONE,
        }
    }
}

/// General widget style
#[derive(Debug, Clone)]
pub struct BaseStyle {
    pub frame: Frame,

    pub text: TextVisuals,

    pub stroke: Stroke,
}

impl WidgetStyle for BaseStyle {}

/// How a widget's contents are laid out
///
/// A theme that gives a widget a size decides both together — the height and the gap of a small
/// button are one look, not two — so they travel as one struct, shared by every widget style that
/// lays its contents out with an [`crate::AtomLayout`].
#[derive(Debug, Clone)]
pub struct LayoutStyle {
    /// How small the widget may get, before its contents are taken into account.
    ///
    /// A floor, not a size: a widget is never smaller than what it holds.
    pub min_size: Vec2,

    /// The gap between the widget's atoms, e.g. between an icon and the text beside it.
    pub gap: f32,
}

/// Dedicated button style
#[derive(Debug, Clone)]
pub struct ButtonStyle {
    pub frame: Frame,

    /// How the button's contents are laid out.
    ///
    /// [`LayoutStyle::min_size`] is ignored by a [`crate::Button::small`] button, which sizes
    /// itself purely from its contents and its own [`crate::Button::min_size`];
    /// [`LayoutStyle::gap`] is overridden by [`crate::Button::gap`].
    pub layout: LayoutStyle,

    pub text_style: TextVisuals,
}

impl WidgetStyle for ButtonStyle {}

/// Dedicated style for a [`crate::Popup`], including menus and tooltips
#[derive(Debug, Clone)]
pub struct PopupStyle {
    /// Frame around the popup's contents, including its padding.
    pub frame: Frame,

    /// Spacing between the items inside the popup.
    ///
    /// A menu wants its items flush against each other, while a tooltip wants them spaced out
    /// like any other content.
    pub item_spacing: Vec2,
}

impl WidgetStyle for PopupStyle {}

/// Dedicated text edit style
#[derive(Debug, Clone)]
pub struct TextEditStyle {
    /// Frame around the text, including its padding.
    pub frame: Frame,

    /// How the field's contents are laid out.
    ///
    /// [`LayoutStyle::min_size`] is raised by [`crate::TextEdit::min_size`] and by the rows of
    /// text the field holds, so it only ever sets a floor; [`LayoutStyle::gap`] separates the
    /// text from a [`crate::TextEdit::prefix`] or [`crate::TextEdit::suffix`].
    pub layout: LayoutStyle,

    /// The text being edited.
    pub text: TextVisuals,

    /// The color of the hint text shown while the buffer is empty.
    pub hint_text_color: Color32,
}

impl WidgetStyle for TextEditStyle {}

/// Dedicated checkbox style
#[derive(Debug, Clone)]
pub struct CheckboxStyle {
    /// Frame around
    pub frame: Frame,

    /// Text next to it
    pub text_style: TextVisuals,

    /// Checkbox size
    pub checkbox_size: f32,

    /// Checkmark size
    pub check_size: f32,

    /// Frame of the checkbox itself
    pub checkbox_frame: Frame,

    /// Checkmark stroke
    pub check_stroke: Stroke,
}

impl WidgetStyle for CheckboxStyle {}

/// Dedicated label style
#[derive(Debug, Clone)]
pub struct LabelStyle {
    /// Frame around
    pub frame: Frame,

    /// Text style
    pub text: TextVisuals,

    /// Wrap mode used
    pub wrap_mode: TextWrapMode,
}

impl WidgetStyle for LabelStyle {}

/// Dedicated separator style
#[derive(Debug, Clone)]
pub struct SeparatorStyle {
    /// How much space is allocated in the layout direction
    pub spacing: f32,

    /// How to paint it
    pub stroke: Stroke,
}

impl WidgetStyle for SeparatorStyle {}

/// The different state of a widget can be
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum WidgetState {
    Noninteractive,
    #[default]
    Inactive,
    Hovered,
    Active,
}

impl Widgets {
    /// The widget visuals according to the state
    pub fn state(&self, state: WidgetState) -> &WidgetVisuals {
        match state {
            WidgetState::Noninteractive => &self.noninteractive,
            WidgetState::Inactive => &self.inactive,
            WidgetState::Hovered => &self.hovered,
            WidgetState::Active => &self.active,
        }
    }
}

impl Response {
    pub fn widget_state(&self) -> WidgetState {
        if !self.sense.interactive() {
            WidgetState::Noninteractive
        } else if self.is_pointer_button_down_on() || self.has_focus() || self.clicked() {
            WidgetState::Active
        } else if self.hovered() || self.highlighted() {
            WidgetState::Hovered
        } else {
            WidgetState::Inactive
        }
    }
}

pub struct StyleArgs<'a> {
    pub classes: &'a Classes,
    pub state: WidgetState,
    pub stack: &'a UiStack,
    pub style: &'a Style,
    pub ctx: &'a Context,
}

impl StyleArgs<'_> {
    /// Does the widget, or any [`crate::Ui`] it sits in, carry this class?
    ///
    /// This is the equivalent of a descendant selector (`.parent .child`): it lets a container
    /// style everything inside it without having to touch each widget. Use
    /// [`HasClasses::has`] on [`Self::classes`] instead for a class only ever set on a widget.
    pub fn has_class(&self, class: impl Into<ClassName>) -> bool {
        let class = class.into();
        self.classes.has(class.clone()) || self.stack.has_class(class)
    }

    /// Read a value from the widget's own classes, falling back to the [`crate::Ui`]s it sits in.
    ///
    /// See [`UiStack::inherited`] for how the fallback resolves.
    pub fn inherited<T>(&self, from_classes: impl Fn(&Classes) -> Option<T>) -> Option<T> {
        from_classes(self.classes).or_else(|| self.stack.inherited(from_classes))
    }
}
