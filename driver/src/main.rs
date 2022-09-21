#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod make_context;
use env_logger::filter::Filter;
use make_context::make_context;

use std::sync::{Arc, RwLock};

use eframe::{
    egui::{self, TextureFilter, Ui, WidgetText},
    epaint::{Color32, ColorImage, ImageData, TextureHandle, Vec2},
    CreationContext, NativeOptions,
};

use egui_dock::{DockArea, DynamicTabViewer, DynamicTree, NodeIndex, Style, Tab};
use heliochrome::context::{Context, RenderTask};

struct ConfigTab {
    bounces: u32,
    samples: u32,
    context: Arc<RwLock<Context>>,
}

impl ConfigTab {
    fn new(context: Arc<RwLock<Context>>) -> Self {
        Self {
            bounces: 50,
            samples: 100,
            context,
        }
    }
}

impl Tab for ConfigTab {
    fn ui(&mut self, ui: &mut Ui) {
        ui.add(egui::Slider::new(&mut self.bounces, 1..=500));
        ui.add(egui::Slider::new(&mut self.samples, 1..=100000));
    }

    fn title(&mut self) -> WidgetText {
        "Config".into()
    }
}

struct PreviewTab {
    texture_handle: TextureHandle,
    context: Arc<RwLock<Context>>,
    render_task: Option<RenderTask>,
}

impl PreviewTab {
    pub fn new(egui_ctx: &egui::Context, context: Arc<RwLock<Context>>) -> Self {
        let texture_handle = egui_ctx.load_texture(
            "preview",
            ImageData::Color(ColorImage::new(
                [
                    context.read().unwrap().get_size().x as usize,
                    context.read().unwrap().get_size().y as usize,
                ],
                Color32::TRANSPARENT,
            )),
            TextureFilter::Nearest,
        );
        let render_task = Some(RenderTask::new(context.clone()));
        render_task.as_ref().unwrap().render();
        Self {
            texture_handle,
            context,
            render_task,
        }
    }
}

impl Tab for PreviewTab {
    fn ui(&mut self, ui: &mut Ui) {
        if let Some(render_task) = &self.render_task {
            render_task
                .data_receiver
                .try_iter()
                .take(100)
                .for_each(|(c, pos)| {
                    self.texture_handle.set_partial(
                        [pos.x as usize, pos.y as usize],
                        ImageData::Color(ColorImage::new(
                            [1, 1],
                            Color32::from_rgb(c[0], c[1], c[2]),
                        )),
                        TextureFilter::Linear,
                    );
                });
        }

        ui.centered_and_justified(|ui| {
            ui.image(self.texture_handle.id(), self.texture_handle.size_vec2());
        });
    }

    fn title(&mut self) -> WidgetText {
        "Preview".into()
    }
}

struct HeliochromeDriver {
    tree: DynamicTree,
    context: Arc<RwLock<Context>>,
}

impl HeliochromeDriver {
    fn new(cc: &CreationContext) -> Self {
        let context = Arc::new(RwLock::new(make_context()));
        let mut tree = DynamicTree::new(vec![Box::new(PreviewTab::new(
            &cc.egui_ctx,
            context.clone(),
        ))]);

        let [a, b] = tree.split_left(
            NodeIndex::root(),
            0.3,
            vec![Box::new(ConfigTab::new(context.clone()))],
        );
        // let [_, _] = tree.split_below(a, 0.7, vec!["tab4".to_owned()]);
        // let [_, _] = tree.split_below(b, 0.5, vec!["tab5".to_owned()]);

        Self { tree, context }
    }
}

impl eframe::App for HeliochromeDriver {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        DockArea::new(&mut self.tree)
            .style(Style::from_egui(ctx.style().as_ref()))
            .show(ctx, &mut DynamicTabViewer {});
        ctx.request_repaint();
    }
}

fn main() {
    let mut options = NativeOptions {
        maximized: true,
        ..NativeOptions::default()
    };

    eframe::run_native(
        "Heliochrome",
        options,
        Box::new(|cc| Box::new(HeliochromeDriver::new(cc))),
    );
}
