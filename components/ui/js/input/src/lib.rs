extern crate capnp;

#[macro_use]
extern crate rustfbp;

use std::thread;

component! {
    ui_js_button, contracts(js_create, js_input)
    inputs(input: any),
    inputs_array(),
    outputs(output: any),
    outputs_array(output: any),
    option(),
    acc(js_input),
    fn run(&mut self) -> Result<()> {
        let mut ip_acc = try!(self.ports.recv("acc"));
        let mut ip_input = try!(self.ports.recv("input"));

        match &ip_input.action[..] {
            "create" => {
                {
                    let mut builder = ip_input.init_root::<js_create::Builder>();
                    let reader: js_input::Reader = try!(ip_acc.get_root());
                    builder.set_name(&self.name);
                    let sender = Box::new(try!(self.ports.get_sender("input")));
                    builder.set_sender(Box::into_raw(sender) as u64);
                    let disabled = if reader.get_disabled() { " disabled" } else { "" };
                    let html = format!("<input type=\"text\" id=\"{}\" style=\"{}\" \
                                        size=\"{}\" maxlength=\"{}\" {}/>"
                                       , self.name
                                       , try!(reader.get_css())
                                       , format!("{}", reader.get_size())
                                       , format!("{}", reader.get_max_size())
                                       , disabled);
                    builder.set_html(&html);
                    builder.set_css(try!(reader.get_block_css()));
                }
                let _ = self.ports.send_action("output", ip_input);
            }
            _ => { let _ = self.ports.send_action("output", ip_input); }
        }

        try!(self.ports.send("acc", ip_acc));

        Ok(())
    }
}
