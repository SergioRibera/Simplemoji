/// All this code was reused and adapted from library "slint-spatial-focus"
/// thanks to "dngulin"
///
/// Repository: https://github.com/dngulin/slint-spatial-focus
/// Commit: https://github.com/dngulin/slint-spatial-focus/tree/6fd54a60715711a1ef3b5e7b35cc558fcbf7a26d
///
use i_slint_core::api::Window;
use i_slint_core::item_tree::{ItemRc, ParentItemTraversalMode};
use i_slint_core::items::FocusReason;
use i_slint_core::lengths::LogicalRect;
use i_slint_core::Coord;

use crate::{FocusDirection, TOLERANCE};

use self::ext::{Inner, ItemRcExt, VisitorResult};

mod ext;

pub trait MoveFocus {
    /// Tries to move focus in a specified direction.s
    /// Returns `Some(())` on success or `None` if the next item to focus is not found.
    fn move_focus(&self, dir: FocusDirection) -> Option<()>;
}

impl MoveFocus for Window {
    fn move_focus(&self, dir: FocusDirection) -> Option<()> {
        let window = self.inner();

        let mut focus_chain_item = window.focus_item.try_borrow().ok()?.upgrade()?;
        let ctx = FocusMoveCtx::new(focus_chain_item.global_rect(), dir);

        while let Some(parent) =
            focus_chain_item.parent_item(ParentItemTraversalMode::FindAllParents)
        {
            if let Some(target) = find_next_focusable_item(&parent, &focus_chain_item, &ctx) {
                window.set_focus_item(&target, true, FocusReason::Programmatic);
                return Some(());
            }
            focus_chain_item = parent;
        }

        None
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum SpatialAxis {
    Horizontal,
    Vertical,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum SpatialDirection {
    Forward,
    Backward,
}

struct FocusMoveCtx {
    pub axis: SpatialAxis,
    pub dir: SpatialDirection,
    pub focused_rect: LogicalRect,
}

impl FocusMoveCtx {
    pub fn new(focused_rect: LogicalRect, move_dir: FocusDirection) -> Self {
        let (axis, dir) = match move_dir {
            FocusDirection::Up => (SpatialAxis::Vertical, SpatialDirection::Backward),
            FocusDirection::Right => (SpatialAxis::Horizontal, SpatialDirection::Forward),
            FocusDirection::Down => (SpatialAxis::Vertical, SpatialDirection::Forward),
            FocusDirection::Left => (SpatialAxis::Horizontal, SpatialDirection::Backward),
        };

        FocusMoveCtx {
            axis,
            dir,
            focused_rect,
        }
    }
}

fn find_next_focusable_item(
    parent: &ItemRc,
    focus_chain_child: &ItemRc,
    ctx: &FocusMoveCtx,
) -> Option<ItemRc> {
    let mut focusable_items = Vec::new();
    let mut visitor = |item: &ItemRc| {
        if item == focus_chain_child || !item.is_visible() {
            return VisitorResult::Skip;
        }

        if item.is_focusable() {
            focusable_items.push(item.clone());
            return VisitorResult::Skip;
        }

        VisitorResult::Continue
    };
    parent.visit_children(&mut visitor);

    let candidates: Vec<(ItemRc, LogicalRect)> = focusable_items
        .iter()
        .map(|i| (i.clone(), i.global_rect()))
        .filter(|(_, r)| is_focus_target(r, ctx))
        .collect();

    let first = candidates.first()?;

    let mut curr_i = first.0.clone();
    let mut curr_d = distance(&first.1, ctx);
    let mut curr_od = ort_distance(&first.1, ctx);

    for (i, r) in &candidates[1..] {
        let d = distance(r, ctx);
        let od = ort_distance(r, ctx);

        if (d - curr_d).abs() <= TOLERANCE {
            if od < curr_od {
                curr_od = od;
                curr_i = i.clone();
            }
        } else if d < curr_d {
            curr_d = d;
            curr_od = od;
            curr_i = i.clone();
        }
    }

    Some(curr_i)
}

fn is_focus_target(r: &LogicalRect, ctx: &FocusMoveCtx) -> bool {
    let f = ctx.focused_rect;
    match (ctx.axis, ctx.dir) {
        (SpatialAxis::Horizontal, SpatialDirection::Backward) => {
            r.origin.x + r.width() - TOLERANCE <= f.origin.x
        }
        (SpatialAxis::Horizontal, SpatialDirection::Forward) => {
            r.origin.x + TOLERANCE >= f.origin.x + f.width()
        }
        (SpatialAxis::Vertical, SpatialDirection::Backward) => {
            r.origin.y + r.height() - TOLERANCE <= f.origin.y
        }
        (SpatialAxis::Vertical, SpatialDirection::Forward) => {
            r.origin.y + TOLERANCE >= f.origin.y + f.height()
        }
    }
}

fn distance(r: &LogicalRect, ctx: &FocusMoveCtx) -> Coord {
    let f = ctx.focused_rect;
    let d = match (ctx.axis, ctx.dir) {
        (SpatialAxis::Horizontal, SpatialDirection::Backward) => {
            (r.origin.x + r.width()) - f.origin.x
        }
        (SpatialAxis::Horizontal, SpatialDirection::Forward) => {
            r.origin.x - (f.origin.x + f.width())
        }
        (SpatialAxis::Vertical, SpatialDirection::Backward) => {
            (r.origin.y + r.height()) - f.origin.y
        }
        (SpatialAxis::Vertical, SpatialDirection::Forward) => {
            r.origin.y - (f.origin.y + f.height())
        }
    };

    d.abs()
}

fn ort_distance(r: &LogicalRect, ctx: &FocusMoveCtx) -> Coord {
    let f = ctx.focused_rect;
    let (a, b) = match ctx.axis {
        SpatialAxis::Horizontal => {
            let a = (f.origin.y, f.origin.y + f.height());
            let b = (r.origin.y, r.origin.y + r.height());
            (a, b)
        }
        SpatialAxis::Vertical => {
            let a = (f.origin.x, f.origin.x + f.width());
            let b = (r.origin.x, r.origin.x + r.width());
            (a, b)
        }
    };

    if are_intersected(&a, &b) {
        return 0.0;
    }

    let ca = a.0 + (a.1 - a.0) / 2.0;
    let cb = b.0 + (b.1 - b.0) / 2.0;

    (ca - cb).abs()
}

fn are_intersected(a: &(Coord, Coord), b: &(Coord, Coord)) -> bool {
    let p1 = a.0 - b.1; // min(a.0, a.1) - max(b.0, b.1)
    let p2 = a.1 - b.0; // max(a.0, a.1) - min(b.0, b.1)
    p1 < 0.0 && p2 > 0.0 // Origin is inside the Minkowski difference, so segments are intersected
}
