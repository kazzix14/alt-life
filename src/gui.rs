use crate::{game::Game, object::creature::Creature};

use azul::{
    prelude::{
        CssProperty, CssPropertyValue, Dom, Layout, LayoutHeight, LayoutInfo, LayoutMarginLeft,
        LayoutMarginTop, LayoutPosition, LayoutWidth, NodeData,
    },
    widgets::{button::Button, label::Label},
};

impl Layout for Game {
    fn layout(&self, info: LayoutInfo<Game>) -> Dom<Game> {
        const PANE_BORDER_WIDTH: f32 = 20.0;
        const FIELD_VIEW_CELL_BORDER: f32 = 1.0;
        const FIELD_VIEW_RECT_MARGIN: f32 = 5.0;

        let field_view_grid_count = self.config.gui_config.field_view_grid_count;
        let dimensions = info.window.state.size.dimensions;

        let left_pane_width = dimensions.height - 2.0 * PANE_BORDER_WIDTH;
        let left_pane_height = dimensions.height - 2.0 * PANE_BORDER_WIDTH;

        let left_pane_margin_left = PANE_BORDER_WIDTH;
        let left_pane_margin_top = PANE_BORDER_WIDTH;

        let right_pane_width =
            dimensions.width - (PANE_BORDER_WIDTH + left_pane_width + PANE_BORDER_WIDTH * 2.0);
        let right_pane_height = left_pane_height;

        let right_pane_margin_left = PANE_BORDER_WIDTH;
        let right_pane_margin_top = left_pane_margin_top;

        let field_view_rect_width = left_pane_width - FIELD_VIEW_RECT_MARGIN * 2.0;
        let field_view_rect_height = field_view_rect_width;

        let field_view_rect_margin_left = FIELD_VIEW_RECT_MARGIN;
        let field_view_rect_margin_top = FIELD_VIEW_RECT_MARGIN;

        let field_view_cell_width = (field_view_rect_width
            - FIELD_VIEW_CELL_BORDER * (field_view_grid_count + 1) as f32)
            / field_view_grid_count as f32;
        let field_view_cell_height = field_view_cell_width;

        let field_view_cell_margin_left = FIELD_VIEW_CELL_BORDER;
        let field_view_cell_margin_top = FIELD_VIEW_CELL_BORDER;

        let label2 = Label::new("label 2").dom().with_class("label");
        let mut left_pane = Dom::div().with_class("pane").with_id("left-pane");

        left_pane.add_css_override(
            "left-pane-margin-left",
            LayoutMarginLeft::px(left_pane_margin_left),
        );

        left_pane.add_css_override(
            "left-pane-margin-top",
            LayoutMarginTop::px(left_pane_margin_top),
        );

        left_pane.add_css_override("left-pane-width", LayoutWidth::px(left_pane_width));

        left_pane.add_css_override("left-pane-height", LayoutHeight::px(left_pane_height));

        let mut right_pane = Dom::div().with_class("pane").with_id("right-pane");

        right_pane.add_css_override(
            "right-pane-margin-left",
            LayoutMarginLeft::px(right_pane_margin_left),
        );

        right_pane.add_css_override(
            "right-pane-margin-top",
            LayoutMarginTop::px(right_pane_margin_top),
        );

        right_pane.add_css_override("right-pane-width", LayoutWidth::px(right_pane_width));

        right_pane.add_css_override("right-pane-height", LayoutHeight::px(right_pane_height));

        let mut field_view_rect = build_field_view_rect(
            &self.field,
            field_view_grid_count,
            field_view_cell_width,
            field_view_cell_height,
            field_view_cell_margin_left,
            field_view_cell_margin_top,
        );

        field_view_rect.add_css_override(
            "field-view-rect-width",
            LayoutWidth::px(field_view_rect_width),
        );
        field_view_rect.add_css_override(
            "field-view-rect-height",
            LayoutHeight::px(field_view_rect_height),
        );
        field_view_rect.add_css_override(
            "field-view-rect-margin-left",
            LayoutMarginLeft::px(field_view_rect_margin_left),
        );
        field_view_rect.add_css_override(
            "field-view-rect-margin-top",
            LayoutMarginTop::px(field_view_rect_margin_top),
        );

        let mut dom = Dom::div()
            .with_child(left_pane.with_child(field_view_rect))
            .with_child(right_pane.with_child(label2))
            .with_id("frame");

        dom
    }
}

fn build_field_view_rect<T: Layout>(
    field: &crate::field::Field,
    field_view_grid_count: u16,
    field_view_cell_width: f32,
    field_view_cell_height: f32,
    field_view_cell_margin_left: f32,
    field_view_cell_margin_top: f32,
) -> Dom<T> {
    use crate::field::Block::*;
    use azul::prelude::{LayoutHeight, LayoutMarginLeft, LayoutMarginTop, LayoutWidth};
    Dom::div()
        .with_child(
            (0..field_view_grid_count)
                .map(|x| {
                    (0..field_view_grid_count)
                        .map(|y| {
                            let mut cell = Dom::div().with_class("field-view-cell").with_class(
                                match field.get(x as i16, y as i16) {
                                    Some(Empty(_)) => "cell-empty",
                                    Some(Wall) => "cell-wall",
                                    None => "cell-none",
                                },
                            );
                            cell.add_css_override(
                                "field-view-cell-width",
                                LayoutWidth::px(field_view_cell_width),
                            );
                            cell.add_css_override(
                                "field-view-cell-height",
                                LayoutHeight::px(field_view_cell_height),
                            );
                            cell.add_css_override(
                                "field-view-cell-margin-left",
                                LayoutMarginLeft::px(field_view_cell_margin_left),
                            );
                            cell.add_css_override(
                                "field-view-cell-margin-top",
                                LayoutMarginTop::px(field_view_cell_margin_top),
                            );
                            cell
                        })
                        .collect::<Dom<T>>()
                        .with_class("row")
                })
                .collect::<Dom<T>>()
                .with_class("column"),
        )
        .with_id("field-view-rect")
}
