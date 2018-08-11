use gtk;
use gtk::prelude::*;
use relm::Widget;
use relm_attributes::widget;

#[derive(Clone)]
pub struct Model {
    pub relm: ::relm::Relm<Request>,
    pub group_name: String,
    pub procedure_name: String,
    pub amount: usize,
}

#[derive(Msg)]
pub enum Msg {
}

#[widget]
impl Widget for Request {
    fn init_view(&mut self) {
        self.group_name.set_text(&self.model.group_name);
        self.procedure_name.set_text(&self.model.procedure_name);
    }

    fn model(relm: &::relm::Relm<Self>, (group_name, procedure_name, amount): (String, String, usize)) -> Model {
        Model {
            relm: relm.clone(),
            group_name: group_name,
            procedure_name: procedure_name,
            amount: amount,
        }
    }

    fn update(&mut self, event: Msg) {
        match event {
            _ => {
            }
        }
    }

    view! {
        gtk::Box {
            orientation: gtk::Orientation::Horizontal,

            #[name="group_name"]
            gtk::Label {},
            #[name="procedure_name"]
            gtk::Label {},
            gtk::Entry {
                input_purpose: gtk::InputPurpose::Digits,
            },
        },
    }
}
