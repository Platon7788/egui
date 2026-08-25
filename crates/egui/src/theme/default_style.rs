use emath::Vec2;
use epaint::{Color32, Margin, Shadow, Stroke, text::TextWrapMode};

use crate::{
    Frame, TextStyle,
    theme::StyleProvider,
    widget_style::{
        BaseStyle, ButtonStyle, CheckboxStyle, HasClasses as _, LabelStyle, READ_ONLY_CLASS,
        SELECTED_CLASS, SeparatorStyle, StyleArgs, TextEditStyle, TextVisuals, WidgetState,
    },
};

/// The default [`StyleProvider`], implementing the default egui look based on
/// [`crate::style::WidgetVisuals`].
#[derive(Debug, Clone)]
pub struct DefaultStyle;

impl StyleProvider<BaseStyle> for DefaultStyle {
    fn style(&mut self, modifiers: &StyleArgs<'_>) -> BaseStyle {
        let StyleArgs { style, state, .. } = modifiers;
        let spacing = &style.spacing;
        let widget_visuals = match state {
            WidgetState::Noninteractive => style.visuals.widgets.noninteractive,
            WidgetState::Inactive => style.visuals.widgets.inactive,
            WidgetState::Hovered => style.visuals.widgets.hovered,
            WidgetState::Active => style.visuals.widgets.active,
        };

        BaseStyle {
            frame: Frame {
                fill: widget_visuals.bg_fill,
                stroke: widget_visuals.bg_stroke,
                corner_radius: widget_visuals.corner_radius,
                inner_margin: spacing.button_padding.into(),
                ..Default::default()
            },
            stroke: widget_visuals.fg_stroke,
            text: TextVisuals {
                color: widget_visuals.text_color(),
                font_id: modifiers
                    .style
                    .override_font_id
                    .clone()
                    .unwrap_or_else(|| TextStyle::Body.resolve(style)),
                strikethrough: Stroke::NONE,
                underline: Stroke::NONE,
            },
        }
    }
}

impl StyleProvider<ButtonStyle> for DefaultStyle {
    fn style(&mut self, modifiers: &StyleArgs<'_>) -> ButtonStyle {
        let StyleArgs {
            ctx,
            classes,
            style,
            state,
            ..
        } = modifiers;
        let spacing = &style.spacing;
        let mut widget_visuals = match state {
            WidgetState::Noninteractive => style.visuals.widgets.noninteractive,
            WidgetState::Inactive => style.visuals.widgets.inactive,
            WidgetState::Hovered => style.visuals.widgets.hovered,
            WidgetState::Active => style.visuals.widgets.active,
        };

        let mut ws: BaseStyle = ctx.get_widget_style(modifiers);

        if classes.has(SELECTED_CLASS) {
            let visuals = &style.visuals;
            widget_visuals.weak_bg_fill = visuals.selection.bg_fill;
            widget_visuals.bg_fill = visuals.selection.bg_fill;
            widget_visuals.fg_stroke = visuals.selection.stroke;
            ws.text.color = visuals.selection.stroke.color;
        }

        ButtonStyle {
            frame: Frame {
                fill: widget_visuals.weak_bg_fill,
                stroke: widget_visuals.bg_stroke,
                corner_radius: widget_visuals.corner_radius,
                outer_margin: (-Vec2::splat(widget_visuals.expansion)).into(),
                inner_margin: (spacing.button_padding + Vec2::splat(widget_visuals.expansion)
                    - Vec2::splat(widget_visuals.bg_stroke.width))
                .into(),
                ..Default::default()
            },
            text_style: ws.text,
            // Historically only the height was floored, so that a button is at least as tall as
            // any other interactive widget on the same row.
            min_size: Vec2::new(0.0, spacing.interact_size.y),
            gap: spacing.icon_spacing,
        }
    }
}

impl StyleProvider<TextEditStyle> for DefaultStyle {
    fn style(&mut self, modifiers: &StyleArgs<'_>) -> TextEditStyle {
        let StyleArgs {
            ctx,
            classes,
            style,
            state,
            ..
        } = modifiers;

        let widget_visuals = match state {
            WidgetState::Noninteractive => style.visuals.widgets.noninteractive,
            WidgetState::Inactive => style.visuals.widgets.inactive,
            WidgetState::Hovered => style.visuals.widgets.hovered,
            WidgetState::Active => style.visuals.widgets.active,
        };

        // A text edit over an immutable buffer is painted without a background.
        let (fill, stroke) = if classes.has(READ_ONLY_CLASS) {
            let visuals = &style.visuals.widgets.inactive;
            (Color32::TRANSPARENT, visuals.bg_stroke)
        } else if *state == WidgetState::Active {
            // While focused, the frame is outlined in the selection color.
            (
                style.visuals.text_edit_bg_color(),
                style.visuals.selection.stroke,
            )
        } else {
            (style.visuals.text_edit_bg_color(), widget_visuals.bg_stroke)
        };

        let mut ws: BaseStyle = ctx.get_widget_style(modifiers);

        // The text of a text edit doesn't brighten on hover — that would be distracting while
        // typing — so it keeps the inactive color no matter the state.
        ws.text.color = style.visuals.widgets.inactive.text_color();

        TextEditStyle {
            frame: Frame {
                fill,
                stroke,
                corner_radius: widget_visuals.corner_radius,
                // The stroke is painted centered on the frame edge, so half of it eats into the
                // padding; compensate, like the other widgets do.
                inner_margin: Margin::symmetric(4, 2)
                    + Margin::same((widget_visuals.expansion - stroke.width).round() as i8),
                outer_margin: Margin::same(-(widget_visuals.expansion as i8)),
                ..Default::default()
            },
            text: ws.text,
            hint_text_color: style.visuals.weak_text_color(),
        }
    }
}

impl StyleProvider<CheckboxStyle> for DefaultStyle {
    fn style(&mut self, modifiers: &StyleArgs<'_>) -> CheckboxStyle {
        let StyleArgs {
            ctx, style, state, ..
        } = modifiers;
        let spacing = &style.spacing;
        let widget_visuals = match state {
            WidgetState::Noninteractive => style.visuals.widgets.noninteractive,
            WidgetState::Inactive => style.visuals.widgets.inactive,
            WidgetState::Hovered => style.visuals.widgets.hovered,
            WidgetState::Active => style.visuals.widgets.active,
        };

        let ws: BaseStyle = ctx.get_widget_style(modifiers);

        CheckboxStyle {
            frame: Frame::new(),
            checkbox_size: spacing.icon_width,
            check_size: spacing.icon_width_inner,
            checkbox_frame: Frame {
                fill: widget_visuals.bg_fill,
                corner_radius: widget_visuals.corner_radius,
                stroke: widget_visuals.bg_stroke,
                ..Default::default()
            },
            text_style: ws.text,
            check_stroke: ws.stroke,
        }
    }
}

impl StyleProvider<LabelStyle> for DefaultStyle {
    fn style(&mut self, modifiers: &StyleArgs<'_>) -> LabelStyle {
        let StyleArgs { ctx, .. } = modifiers;
        let ws: BaseStyle = ctx.get_widget_style(modifiers);

        LabelStyle {
            frame: Frame {
                fill: ws.frame.fill,
                inner_margin: 0.0.into(),
                outer_margin: 0.0.into(),
                stroke: Stroke::NONE,
                shadow: Shadow::NONE,
                corner_radius: 0.into(),
            },
            text: ws.text,
            wrap_mode: TextWrapMode::Wrap,
        }
    }
}

impl StyleProvider<SeparatorStyle> for DefaultStyle {
    fn style(&mut self, modifiers: &StyleArgs<'_>) -> SeparatorStyle {
        let StyleArgs { style, .. } = modifiers;

        SeparatorStyle {
            spacing: 6.0,
            // A separator is never interactive, so its stroke doesn't depend on the widget state:
            stroke: style.visuals.widgets.noninteractive.bg_stroke,
        }
    }
}
