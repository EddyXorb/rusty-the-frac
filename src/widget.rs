use druid::piet::{ImageFormat, InterpolationMode};
use druid::widget::prelude::*;
use druid::{AppLauncher, Color, LocalizedString, WindowDesc};

use crate::RawImage;

struct MandelbrotWidget;

// If this widget has any child widgets it should call its event, update and layout
// (and lifecycle) methods as well to make sure it works. Some things can be filtered,
// but a general rule is to just pass it through unless you really know you don't want it.
impl Widget<RawImage> for MandelbrotWidget {
    fn event(
        &mut self,
        _ctx: &mut EventCtx<'_, '_>,
        _event: &Event,
        _data: &mut RawImage,
        _env: &Env,
    ) {
    }

    fn lifecycle(
        &mut self,
        _ctx: &mut LifeCycleCtx<'_, '_>,
        _event: &LifeCycle,
        _data: &RawImage,
        _env: &Env,
    ) {
    }

    fn update(
        &mut self,
        _ctx: &mut UpdateCtx<'_, '_>,
        _old_data: &RawImage,
        _data: &RawImage,
        _env: &Env,
    ) {
    }

    fn layout(
        &mut self,
        _layout_ctx: &mut LayoutCtx<'_, '_>,
        bc: &BoxConstraints,
        _data: &RawImage,
        _env: &Env,
    ) -> Size {
        // BoxConstraints are passed by the parent widget.
        // This method can return any Size within those constraints:
        // bc.constrain(my_size)
        //
        // To check if a dimension is infinite or not (e.g. scrolling):
        // bc.is_width_bounded() / bc.is_height_bounded()
        //
        // bx.max() returns the maximum size of the widget. Be careful
        // using this, since always make sure the widget is bounded.
        // If bx.max() is used in a scrolling widget things will probably
        // not work correctly.
        if bc.is_width_bounded() | bc.is_height_bounded() {
            let size = Size::new(100.0, 100.0);
            bc.constrain(size)
        } else {
            bc.max()
        }
    }

    // The paint method gets called last, after an event flow.
    // It goes event -> update -> layout -> paint, and each method can influence the next.
    // Basically, anything that changes the appearance of a widget causes a paint.
    fn paint(&mut self, ctx: &mut PaintCtx<'_, '_, '_>, data: &RawImage, _env: &Env) {
        // Clear the whole widget with the color of your choice
        // (ctx.size() returns the size of the layout rect we're painting in)
        // Note: ctx also has a `clear` method, but that clears the whole context,
        // and we only want to clear this widget's area.
        let size = ctx.size();
        let rect = size.to_rect();
        ctx.fill(rect, &Color::WHITE);

        println!("h{},w{},size{}", data.height(), data.width(),data.rgba().len());
        // Let's burn some CPU to make a (partially transparent) image buffer
        let image = ctx
            .make_image(
                data.width(),
                data.height(),
                data.rgba(),
                ImageFormat::RgbaSeparate,
            )
            .unwrap();
        // The image is automatically scaled to fit the rect you pass to draw_image
        ctx.draw_image(&image, rect, InterpolationMode::Bilinear);
    }

    fn id(&self) -> Option<WidgetId> {
        None
    }

    fn type_name(&self) -> &'static str {
        std::any::type_name::<Self>()
    }
}

pub fn start_widget(start_image: RawImage) {
    //mandelbrot: Vec<u8>, width: usize, height: usize) {
    let window = WindowDesc::new(|| -> MandelbrotWidget { MandelbrotWidget })
        .title(LocalizedString::new("Mandelbrot-Set"));
    AppLauncher::with_window(window)
        .launch(start_image)
        .expect("launch failed");
}
