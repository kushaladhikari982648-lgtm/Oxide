//! Track total drag distance separately from per-event scroll deltas.
/// Scroll only enough to reveal the selected row, retaining the current offset.
pub(super) fn reveal(scroll: f32, row: [f32; 4], view: [f32; 3]) -> f32 {
    let [top, bottom, maximum] = view;
    let maximum = maximum.max(0.0);
    let adjustment = if row[1] < top {
        row[1] - top
    } else if row[1] + row[3] > bottom {
        // The last rows end past the viewport's end by construction (a list's
        // height is rows x row height, not that less the last row's padding), so
        // asking for the row's frame would leave the list short of its end. The
        // end of the scroll shows the last row whole: go there instead.
        if row[1] + row[3] - bottom >= maximum - scroll {
            maximum - scroll
        } else {
            row[1] + row[3] - bottom
        }
    } else {
        0.0
    };
    (scroll + adjustment).clamp(0.0, maximum)
}
pub(super) struct Drag {
    origin: [f64; 2],
    previous_y: f64,
    pub(super) moved: bool,
}

impl Drag {
    pub(super) fn new(x: f64, y: f64) -> Self {
        Self {
            origin: [x, y],
            previous_y: y,
            moved: false,
        }
    }

    pub(super) fn origin_y(&self) -> f64 {
        self.origin[1]
    }

    pub(super) fn update(&mut self, x: f64, y: f64, density: f64) -> f32 {
        let density = density.max(0.01);
        self.moved |= (x - self.origin[0]).hypot(y - self.origin[1]) > 12.0 * density;
        let delta = if self.moved {
            ((self.previous_y - y) / density) as f32
        } else {
            0.0
        };
        self.previous_y = y;
        delta
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn focus_scroll_reveals_rows_in_both_directions_and_stops_at_bounds() {
        assert_eq!(
            reveal(0.0, [0.0, 350.0, 100.0, 60.0], [100.0, 400.0, 500.0]),
            10.0
        );
        assert_eq!(
            reveal(200.0, [0.0, 80.0, 100.0, 60.0], [100.0, 400.0, 500.0]),
            180.0
        );
        assert_eq!(
            reveal(20.0, [0.0, 150.0, 100.0, 60.0], [100.0, 400.0, 500.0]),
            20.0
        );
        assert_eq!(
            reveal(490.0, [0.0, 700.0, 100.0, 60.0], [100.0, 400.0, 500.0]),
            500.0
        );
    }

    #[test]
    fn the_end_of_the_scroll_reveals_the_last_row_whole() {
        // Ten rows of 70 in a viewport of 300: the content is 700, so the last
        // row's frame ends 90 past the viewport at the maximum scroll of 400.
        // Asking for the frame alone would stop at 390 and leave the list a row
        // short of its end; the end of the scroll shows it whole instead.
        let view = [100.0, 400.0, 400.0];
        let last = [0.0, 100.0 + 9.0 * 70.0, 100.0, 60.0];
        assert_eq!(reveal(0.0, last, view), 390.0);
        // A row above the top still aligns with the top, and the clamp holds.
        assert_eq!(reveal(0.0, [0.0, 20.0, 100.0, 60.0], view), 0.0);
        // A row 20 past the end with 10 left to scroll goes to the end (400),
        // not 410: the clamp is the list's end, not the row's frame.
        assert_eq!(reveal(390.0, [0.0, 760.0, 100.0, 60.0], view), 400.0);
        // The middle of the list is unchanged.
        assert_eq!(reveal(100.0, [0.0, 250.0, 100.0, 60.0], view), 100.0);
    }

    #[test]
    fn slow_vertical_drag_crosses_the_total_threshold() {
        let mut drag = Drag::new(100.0, 300.0);
        for y in [297.0, 294.0, 291.0, 288.0] {
            assert_eq!(drag.update(100.0, y, 1.0), 0.0);
        }
        assert_eq!(drag.update(100.0, 285.0, 1.0), 3.0);
        assert!(drag.moved);
        assert_eq!(drag.origin_y(), 300.0);
        assert_eq!(drag.update(100.0, 290.0, 1.0), -5.0);
    }

    #[test]
    fn density_scales_threshold_and_scroll_together() {
        let mut drag = Drag::new(200.0, 600.0);
        assert_eq!(drag.update(200.0, 580.0, 2.0), 0.0);
        assert_eq!(drag.update(200.0, 570.0, 2.0), 5.0);
        assert!(drag.moved);
    }
}
