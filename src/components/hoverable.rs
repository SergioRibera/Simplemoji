use iced::advanced::layout;
use iced::advanced::renderer;
use iced::advanced::widget::tree::{self, Tree};
use iced::advanced::{Clipboard, Layout, Shell, Widget};
use iced::{event, Element, Event, Length, Padding, Point, Rectangle};

#[allow(missing_debug_implementations)]
pub struct Hoverable<'a, Message, Renderer> {
    content: Element<'a, Message, Renderer>,
    hover: Option<Message>,
    unhover: Option<Message>,
    padding: Padding,
}

impl<'a, Message, Renderer> Hoverable<'a, Message, Renderer>
where
    Renderer: renderer::Renderer,
{
    pub fn new(content: Element<'a, Message, Renderer>) -> Self {
        Self {
            content,
            hover: None,
            unhover: None,
            padding: Padding::ZERO,
        }
    }

    pub fn on_hover(mut self, message: Message) -> Self {
        self.hover = Some(message);
        self
    }
    pub fn on_unhover(mut self, message: Message) -> Self {
        self.unhover = Some(message);
        self
    }

    pub fn padding<P>(mut self, padding: P) -> Self
    where
        P: Into<Padding>,
    {
        self.padding = padding.into();
        self
    }
}

impl<'a, Message, Renderer> Widget<Message, Renderer> for Hoverable<'a, Message, Renderer>
where
    Message: 'a + Clone,
    Renderer: renderer::Renderer,
{
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(State::default())
    }

    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(&self.content)]
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(std::slice::from_ref(&self.content));
    }

    fn on_event(
        &mut self,
        tree: &mut Tree,
        event: Event,
        layout: Layout<'_>,
        cursor_position: iced_tiny_skia::core::mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) -> event::Status {
        if let event::Status::Captured = self.content.as_widget_mut().on_event(
            &mut tree.children[0],
            event,
            layout.children().next().unwrap(),
            cursor_position,
            renderer,
            clipboard,
            shell,
            viewport
        ) {
            return event::Status::Captured;
        }

        let mut state = tree.state.downcast_mut::<State>();
        let was_hovered = state.is_hovered;
        let now_hovered = cursor_position.position_in(layout.bounds()).is_some();

        match (was_hovered, now_hovered) {
            (true, true) => {}
            (false, false) => {}
            (true, false) => {
                // exited hover
                state.is_hovered = now_hovered;
                if let Some(on_unhover) = self.unhover.clone() {
                    shell.publish(on_unhover.clone());
                }
            }
            (false, true) => {
                // entered hover
                state.is_hovered = now_hovered;
                if let Some(on_hover) = self.hover.clone() {
                    shell.publish(on_hover.clone());
                }
            }
        }

        event::Status::Ignored
    }

    fn layout(&self, renderer: &Renderer, limits: &layout::Limits) -> layout::Node {
        let limits = limits
            .width(Length::Shrink)
            .height(Length::Shrink)
            .pad(self.padding);

        let mut content_layout = self.content.as_widget().layout(renderer, &limits);
        content_layout.move_to(Point::new(self.padding.left, self.padding.top));

        let size = limits.resolve(content_layout.size()).pad(self.padding);

        layout::Node::with_children(size, vec![content_layout])
    }

    fn width(&self) -> Length {
        Length::Shrink
    }

    fn height(&self) -> Length {
        Length::Shrink
    }

    fn draw(
        &self,
        state: &Tree,
        renderer: &mut Renderer,
        theme: &Renderer::Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor_position: iced::mouse::Cursor,
        _viewport: &Rectangle,
    ) {
        let bounds = layout.bounds();
        let content_layout = layout.children().next().unwrap();

        self.content.as_widget().draw(
            &state.children[0],
            renderer,
            theme,
            style,
            content_layout,
            cursor_position,
            &bounds,
        );
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct State {
    is_hovered: bool,
}

impl<'a, Message, Renderer> From<Hoverable<'a, Message, Renderer>>
    for Element<'a, Message, Renderer>
where
    Message: Clone + 'a,
    Renderer: renderer::Renderer + 'a,
{
    fn from(hoverable: Hoverable<'a, Message, Renderer>) -> Self {
        Self::new(hoverable)
    }
}
