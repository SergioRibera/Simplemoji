use i_slint_core::api::Window;
use i_slint_core::item_tree::ItemRc;
use i_slint_core::items::{FocusScope, TextInput};
use i_slint_core::lengths::LogicalRect;
use i_slint_core::window::WindowInner;

pub trait Inner {
    fn inner(&self) -> &WindowInner;
}

impl Inner for Window {
    fn inner(&self) -> &WindowInner {
        unsafe { std::mem::transmute(self) }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum VisitorResult {
    Continue,
    Skip,
}

pub trait ItemRcExt {
    fn global_rect(&self) -> LogicalRect;
    fn is_focusable(&self) -> bool;
    fn visit_children<F: FnMut(&ItemRc) -> VisitorResult>(&self, visitor: &mut F);
}

impl ItemRcExt for ItemRc {
    fn global_rect(&self) -> LogicalRect {
        let local_rect = self.geometry();
        let global_pos = self.map_to_window(local_rect.origin);

        LogicalRect::new(global_pos, local_rect.size)
    }

    fn is_focusable(&self) -> bool {
        if let Some(ti) = self.downcast::<TextInput>() {
            return ti.as_pin_ref().enabled();
        }

        if let Some(fs) = self.downcast::<FocusScope>() {
            return fs.as_pin_ref().enabled();
        }

        false
    }

    fn visit_children<F: FnMut(&ItemRc) -> VisitorResult>(&self, visitor: &mut F) {
        let mut visited = Vec::new();

        if let Some(child) = self.first_child() {
            visit_depth(child, visitor, &mut visited);

            while let Some(visited_item) = visited.pop() {
                if let Some(sibling) = visited_item.next_sibling() {
                    visit_depth(sibling, visitor, &mut visited);
                }
            }
        }
    }
}

fn visit_depth<F: FnMut(&ItemRc) -> VisitorResult>(
    mut item: ItemRc,
    visitor: &mut F,
    visited: &mut Vec<ItemRc>,
) {
    let mut visit_result = visitor(&item);
    visited.push(item.clone());

    while visit_result == VisitorResult::Continue {
        if let Some(next_child) = item.first_child() {
            item = next_child;

            visit_result = visitor(&item);
            visited.push(item.clone());
        } else {
            break;
        }
    }
}
