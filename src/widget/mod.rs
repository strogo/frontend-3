use gtk::{
    self,
    BoxExt,
    ContainerExt,
    FrameExt,
    OrientableExt,
    RangeExt,
    SpinButtonSignals,
    ToggleButtonExt,
    WidgetExt,
};
use relm::gtk_ext::BoxExtManual;
use relm_attributes::widget;

#[derive(Msg)]
pub enum Signal {
    Expand,
    Fold,
    Changed(f64),
}

#[widget]
impl ::relm::Widget for PreciseScale {
    fn model() -> () {
    }

    fn update(&mut self, event: Signal, _: &mut Self::Model) {
        match event {
            Signal::Expand => self.spin.set_visible(true),
            Signal::Fold => self.spin.set_visible(false),
            _ => (),
        };
    }

    fn init_view(&self) {
        self.spin.set_visible(false);
        self.scale.add_mark(0.0, ::gtk::PositionType::Top, None);
    }

    view! {
        #[name="frame"]
        gtk::Frame {
            gtk::Box {
                border_width: 10,
                orientation: gtk::Orientation::Vertical,
                gtk::Box {
                    orientation: gtk::Orientation::Horizontal,
                    spacing: 10,
                    #[name="toggle"]
                    gtk::CheckButton {
                        toggled(w) => if w.get_active() {
                            Signal::Expand
                        } else {
                            Signal::Fold
                        }
                    },
                    #[name="scale"]
                    gtk::Scale {
                        packing: {
                            expand: true,
                            fill: true,
                        },
                        change_value(_, _, value) => (Signal::Changed(value), ::gtk::Inhibit(false)),
                    },
                },
                #[name="spin"]
                gtk::SpinButton {
                    no_show_all: true,
                    value_changed(w) => Signal::Changed(w.get_value()),
                },
            },
        },
    }
}

impl PreciseScale {
    pub fn set_adjustment(&self, adjustment: ::gtk::Adjustment) {
        self.scale.set_adjustment(&adjustment);

        adjustment.set_step_increment(
            adjustment.get_step_increment() / 10.0
        );
        adjustment.set_page_increment(
            adjustment.get_page_increment() / 10.0
        );
        self.spin.set_adjustment(&adjustment);
    }

    pub fn set_value(&self, value: f64) {
        self.scale.set_value(value);
    }

    pub fn get_value(&self) -> f64 {
        self.scale.get_value()
    }

    pub fn set_label(&self, label: &str) {
        self.frame.set_label(label);
    }

    pub fn set_visible(&self, visible: bool) {
        self.frame.set_visible(visible);
    }

    pub fn set_digits(&self, digits: u32) {
        self.spin.set_digits(digits);
    }
}
