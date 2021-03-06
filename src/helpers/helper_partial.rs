use helpers::{HelperDef};
use template::{Helper};
use registry::{Registry};
use context::{Context};
use render::{Renderable, RenderContext, RenderError, render_error, EMPTY};

#[derive(Clone, Copy)]
pub struct IncludeHelper;

#[derive(Clone, Copy)]
pub struct BlockHelper;

#[derive(Clone, Copy)]
pub struct PartialHelper;

impl HelperDef for IncludeHelper {
    fn call(&self, c: &Context, h: &Helper, r: &Registry, rc: &mut RenderContext) -> Result<String, RenderError> {
        let param = h.params().get(0);

        if param.is_none() {
            return Err(render_error("Param not found for helper"));
        }

        let template = r.get_template(param.unwrap());

        match template {
            Some(t) => {
                (*t).render(c, r, rc)
            },
            None => {
                Err(render_error("Template not found."))
            }
        }
    }
}

impl HelperDef for BlockHelper {
    fn call(&self, c: &Context, h: &Helper, r: &Registry, rc: &mut RenderContext) -> Result<String, RenderError> {
        let param = h.params().get(0);

        if param.is_none() {
            return Err(render_error("Param not found for helper"));
        }

        let partial_template = rc.get_partial(param.unwrap());

        match partial_template {
            Some(partial_template) => {
                partial_template.render(c, r, rc)
            },
            None => {
                h.template().unwrap().render(c, r, rc)
            }
        }
    }
}

impl HelperDef for PartialHelper {
    fn call(&self, _: &Context, h: &Helper, _: &Registry, rc: &mut RenderContext) -> Result<String, RenderError> {
        let param = h.params().get(0);

        if param.is_none() {
            return Err(render_error("Param not found for helper"));
        }

        rc.set_partial(param.unwrap().clone(), h.template().unwrap().clone());
        Ok(EMPTY.to_string())
    }
}

pub static INCLUDE_HELPER: IncludeHelper = IncludeHelper;
pub static BLOCK_HELPER: BlockHelper = BlockHelper;
pub static PARTIAL_HELPER: PartialHelper = PartialHelper;

#[cfg(test)]
mod test {
    use template::{Template};
    use registry::{Registry};

    #[test]
    fn test() {
        let t0 = Template::compile("<h1>{{#block title}}default{{/block}}</h1>".to_string()).ok().unwrap();
        let t1 = Template::compile("{{#partial title}}{{this}}{{/partial}}{{> t0}}".to_string()).ok().unwrap();
        let t2 = Template::compile("{{> t0}}<p>{{this}}</p>".to_string()).ok().unwrap();

        let mut handlebars = Registry::new();
        handlebars.register_template("t0", t0);
        handlebars.register_template("t1", t1);
        handlebars.register_template("t2", t2);

        let r0 = handlebars.render("t1", &true);
        assert_eq!(r0.ok().unwrap(), "<h1>true</h1>".to_string());

        let r1 = handlebars.render("t2", &true);
        assert_eq!(r1.ok().unwrap(), "<h1>default</h1><p>true</p>".to_string());
    }
}
