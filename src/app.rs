use crate::agent::Agent;
use std::rc::Rc;
use yew::prelude::*;
use yew::{Component, Context};
use yew_agent::{Bridge, Bridged};

pub struct App {
    agent: Box<dyn Bridge<Agent>>,
    string_to_display: Option<String>,
}

pub enum Message {
    AgentMsg(String),
    Click,
}

impl Component for App {
    type Message = Message;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let cb = {
            let link = ctx.link().clone();
            move |e| link.send_message(Self::Message::AgentMsg(e))
        };
        let agent = Agent::bridge(Rc::new(cb));

        Self {
            agent,
            string_to_display: None,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Message::AgentMsg(e) => {
                self.string_to_display = Some(e);
                true
            }
            Message::Click => {
                self.agent.send("I clicked !".to_string());
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                <h1>{ "Yew debug" }</h1>
                <button type="button" onclick={ctx.link().callback(|_| Message::Click)}>{ "Click me"}</button>
                {
                    match &self.string_to_display {
                        Some(e) => html!(<p>{ e }</p>),
                        None => html!(),
                    }
                }
            </>
        }
    }
}
