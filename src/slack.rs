use ws::{listen, Handler, Sender, Result, Message, Handshake, CloseCode, Error};

/// A client that encapsulates a websocket connection to the Slack API.
pub struct Client {
    pub out: Sender,
}

impl Handler for Client {
    /// `on_open` will be called only after the WebSocket handshake is successful.
    fn on_open(&mut self, _: Handshake) -> Result<()> {
        Ok(println!("websocket established"))
    }

    /// `on_message`
    fn on_message(&mut self, msg: Message) -> Result<()> {
        //println!("Got message: {}", msg);
        //self.out.send("{\"id\": 1, \"type\": \"message\", 
        Ok(println!("got message: {}", msg))
    }
}
