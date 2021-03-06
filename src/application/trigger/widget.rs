use color::Colorable;
use gtk::{
    BoxExt,
    ButtonExt,
    WidgetExt,
};
use relm::ContainerWidget;
use super::Channel;
use super::Edge;
use super::Model;
use super::Mode;
use super::Signal;

#[derive(Clone)]
pub struct Widget {
    page: ::gtk::Box,
    pub single_button: ::gtk::Button,
    stream: ::relm::EventStream<Signal>,
    mode: ::relm::Component<::widget::RadioGroup<Mode>>,
    channel: ::relm::Component<::widget::RadioGroup<Channel>>,
    edge: ::relm::Component<::widget::RadioGroup<Edge>>,
}

impl Widget {
    fn get_source(&self) -> Option<::redpitaya_scpi::trigger::Source> {
        let channel = self.channel.widget().get_current();
        let edge = self.edge.widget().get_current();

        if channel == Some(Channel::CH1) && edge == Some(Edge::Positive) {
            Some(::redpitaya_scpi::trigger::Source::CH1_PE)
        } else if channel == Some(Channel::CH1) && edge == Some(Edge::Negative) {
            Some(::redpitaya_scpi::trigger::Source::CH1_NE)
        } else if channel == Some(Channel::CH2) && edge == Some(Edge::Positive) {
            Some(::redpitaya_scpi::trigger::Source::CH2_PE)
        } else if channel == Some(Channel::CH2) && edge == Some(Edge::Negative) {
            Some(::redpitaya_scpi::trigger::Source::CH2_NE)
        } else if channel == Some(Channel::EXT) && edge == Some(Edge::Positive) {
            Some(::redpitaya_scpi::trigger::Source::EXT_PE)
        } else if channel == Some(Channel::EXT) && edge == Some(Edge::Negative) {
            Some(::redpitaya_scpi::trigger::Source::EXT_NE)
        } else {
            None
        }
    }
}

impl ::relm::Widget for Widget {
    type Model = Model;
    type Msg = Signal;
    type Root = ::gtk::Box;
    type ModelParam = ::redpitaya_scpi::trigger::Trigger;

    fn model(trigger: Self::ModelParam) -> Self::Model {
        Model {
            trigger: trigger,
            mode: Mode::Normal,
        }
    }

    fn root(&self) -> &Self::Root {
        &self.page
    }

    fn update(&mut self, event: Signal, model: &mut Self::Model) {
        match event {
            Signal::InternalTick => {
                match model.mode {
                    Mode::Auto => self.stream.emit(Signal::Auto),
                    Mode::Normal => self.stream.emit(Signal::Normal),
                    Mode::Single => (),
                };
            },
            Signal::Mode(mode) => {
                model.mode = mode;

                match mode {
                    Mode::Auto => self.single_button.set_visible(false),
                    Mode::Normal => self.single_button.set_visible(false),
                    Mode::Single => self.single_button.set_visible(true),
                };
            },
            Signal::Channel(_) | Signal::Edge(_) => {
                if let Some(source) = self.get_source() {
                    self.stream.emit(Signal::Source(source));
                    model.trigger.enable(source);
                }
            },
            _ => (),
        }
    }

    fn view(relm: &::relm::RemoteRelm<Self>, model: &Self::Model) -> Self {
        let page = ::gtk::Box::new(::gtk::Orientation::Vertical, 10);

        let args = ::widget::radio::Model {
            title: String::from("Source"),
            options: vec![Channel::CH1, Channel::CH2, Channel::EXT],
            current: Some(Channel::CH1),
        };
        let channel = page.add_widget::<::widget::RadioGroup<Channel>, _>(&relm, args);
        connect!(
            channel@::widget::radio::Signal::Change(channel),
            relm,
            Signal::Channel(channel)
        );

        let args = ::widget::radio::Model {
            title: String::from("Edge"),
            options: vec![Edge::Positive, Edge::Negative],
            current: Some(Edge::Positive),
        };
        let edge = page.add_widget::<::widget::RadioGroup<Edge>, _>(&relm, args);
        connect!(
            edge@::widget::radio::Signal::Change(edge),
            relm,
            Signal::Edge(edge)
        );

        let args = ::widget::radio::Model {
            title: String::from("Mode"),
            options: vec![Mode::Auto, Mode::Normal, Mode::Single],
            current: Some(model.mode),
        };
        let mode = page.add_widget::<::widget::RadioGroup<Mode>, _>(&relm, args);
        connect!(
            mode@::widget::radio::Signal::Change(mode),
            relm,
            Signal::Mode(mode)
        );

        let single_button = ::gtk::Button::new_with_label("Single");
        page.pack_start(&single_button, false, false, 0);
        connect!(relm, single_button, connect_clicked(_), Signal::Single);

        let stream = relm.stream().clone();
        GLOBAL.with(move |global| {
            *global.borrow_mut() = Some(stream)
        });

        ::gtk::timeout_add(1_000, || {
            GLOBAL.with(|global| {
                if let Some(ref stream) = *global.borrow() {
                    stream.emit(Signal::InternalTick);
                }
            });

            ::glib::Continue(true)
        });

        let stream = relm.stream().clone();

        Widget {
            page,
            single_button,
            stream,
            mode,
            channel,
            edge,
        }
    }
}

impl ::application::Panel for Widget {
    fn draw(&self, context: &::cairo::Context, model: &::application::Model) {
        let mode = self.mode.widget().get_current();

        if mode == Some(Mode::Normal) || mode == Some(Mode::Single) {
            let width = model.scales.get_width();
            let height = model.scales.get_height();
            let delay = model.offset("DELAY");
            let trigger = model.offset("TRIG");

            context.set_color(::color::TRIGGER);

            context.set_line_width(width / 1000.0);
            context.move_to(delay, model.scales.v.0);
            context.line_to(delay, model.scales.v.1);
            context.stroke();

            context.set_line_width(height / 1000.0);
            context.move_to(model.scales.h.0, trigger);
            context.line_to(model.scales.h.1, trigger);
            context.stroke();
        }
    }
}

thread_local!(
    static GLOBAL: ::std::cell::RefCell<Option<::relm::EventStream<Signal>>> = ::std::cell::RefCell::new(None)
);
