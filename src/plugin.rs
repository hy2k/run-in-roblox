use std::io::Write;

use rbx_xml::EncodeError;

use rbx_dom_weak::{InstanceBuilder, WeakDom};

use rbx_types::Variant;

static PLUGIN_TEMPLATE: &'static str = include_str!("plugin_main_template.lua");

pub struct RunInRbxPlugin<'a> {
    pub port: u16,
    pub server_id: &'a str,
    pub lua_script: &'a str,
}

impl<'a> RunInRbxPlugin<'a> {
    pub fn write<W: Write>(&self, output: W) -> Result<(), EncodeError> {
        let tree = self.build_plugin();
        let root_ref = tree.root_ref();

        rbx_xml::to_writer_default(output, &tree, &[root_ref])
    }

    fn build_plugin(&self) -> WeakDom {
        let complete_source = PLUGIN_TEMPLATE
            .replace("{{PORT}}", &self.port.to_string())
            .replace("{{SERVER_ID}}", self.server_id);

        let plugin_script = InstanceBuilder::new("Script")
            .with_name("run-in-roblox-plugin")
            .with_property("Source", Variant::String(complete_source));

        let main_source = format!("return function()\n{}\nend", self.lua_script);

        let injected_main = InstanceBuilder::new("ModuleScript")
            .with_name("Main")
            .with_property("Source", Variant::String(main_source));
        

        let mut tree = WeakDom::new(plugin_script);

        let root_ref = tree.root_ref();
        tree.insert(root_ref, injected_main);

        tree
    }
}
