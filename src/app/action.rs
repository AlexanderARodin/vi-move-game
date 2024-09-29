use ratatui::crossterm::event as xEvent;

#[derive(Debug,PartialEq)]
pub enum Action {
    Noop,
    Quit,
    //ProcessRawEventList(Vec<xEvent::Event>),
    TranslateRawEvent(xEvent::Event),
    HandleByEditor(xEvent::Event),
    //Error(String),
}
