use yew_agent::{HandlerId, Public, WorkerLink};

pub struct Agent {
    link: WorkerLink<Self>,
}

impl yew_agent::Worker for Agent {
    type Input = String;
    type Message = ();
    type Output = String;
    type Reach = Public<Self>;

    fn create(link: WorkerLink<Self>) -> Self {
        Self { link }
    }

    fn update(&mut self, _msg: Self::Message) {}

    fn handle_input(&mut self, msg: Self::Input, id: HandlerId) {
        self.link.respond(id, msg);
    }

    // fn name_of_resource() -> &'static str {
    //     "agent.js"
    // }
}
