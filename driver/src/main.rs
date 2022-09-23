#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod make_context;
use env_logger::{filter::Filter, fmt::Color};
use make_context::make_context;

use std::{
    cell::{Cell, RefCell},
    rc::Rc,
    sync::{Arc, RwLock},
};

use eframe::{
    egui::{
        self,
        plot::{Legend, Plot, PlotImage, PlotPoint},
        CollapsingHeader, Direction, Key, Layout, TextureFilter, Ui, WidgetText,
    },
    emath::Align,
    epaint::{Color32, ColorImage, ImageData, TextureHandle, Vec2},
    CreationContext, NativeOptions,
};

use egui_dock::{DockArea, DynamicTabViewer, DynamicTree, NodeIndex, Style, Tab};
use heliochrome::{context::Context, maths::vec2, scene::Scene};

struct StateData {
    pub changed: bool,
    pub context: Context,
}

impl StateData {
    pub fn new(context: Context) -> Self {
        Self {
            changed: false,
            context,
        }
    }
}

type State = Rc<RefCell<StateData>>;

struct ConfigTab {
    bounces: u32,
    samples: u32,
    width: u16,
    height: u16,

    state: State,
}

impl ConfigTab {
    fn new(state: State) -> Self {
        let size = state.borrow().context.get_size();
        Self {
            bounces: 50,
            samples: 100,
            width: size.x as u16,
            height: size.y as u16,
            state,
        }
    }
}

impl Tab for ConfigTab {
    fn ui(&mut self, ui: &mut Ui) {
        egui::Grid::new("bounces/samples").show(ui, |ui| {
            ui.label("bounces: ");
            ui.add(egui::DragValue::new(
                &mut self.state.borrow_mut().context.quality.bounces,
            ));
            ui.end_row();

            ui.label("samples: ");
            ui.add(egui::DragValue::new(
                &mut self.state.borrow_mut().context.quality.samples,
            ));
            ui.end_row();
        });

        ui.label("image size: ");
        egui::Grid::new("size").show(ui, |ui| {
            ui.label("width:");
            let resw = ui.add(egui::DragValue::new(&mut self.width).suffix("px"));
            ui.label("height:");
            let resh = ui.add(egui::DragValue::new(&mut self.height).suffix("px"));
            ui.end_row();
            if resw.changed() || resh.changed() {
                let mut state = self.state.borrow_mut();
                state.context.stop_full_render();
                state
                    .context
                    .resize(vec2::new(self.width as f32, self.height as f32));
            }
        });
    }

    fn title(&mut self) -> WidgetText {
        "Config".into()
    }
}

struct SceneTab {
    state: State,
}

impl SceneTab {
    fn new(state: State) -> Self {
        Self { state }
    }
}

impl Tab for SceneTab {
    fn ui(&mut self, ui: &mut Ui) {
        let mut changed = false;
        CollapsingHeader::new("Camera")
            .default_open(true)
            .show(ui, |ui| {
                egui::Grid::new("bounces/samples").show(ui, |ui| {
                    ui.label("field of view: ");
                    changed = changed
                        || ui
                            .add(egui::DragValue::new(
                                &mut self
                                    .state
                                    .borrow_mut()
                                    .context
                                    .scene
                                    .write()
                                    .unwrap()
                                    .camera
                                    .vfov,
                            ))
                            .changed();
                    ui.end_row();

                    ui.label("aperture diameter: ");
                    changed = changed
                        || ui
                            .add(egui::DragValue::new(
                                &mut self
                                    .state
                                    .borrow_mut()
                                    .context
                                    .scene
                                    .write()
                                    .unwrap()
                                    .camera
                                    .aperture,
                            ))
                            .changed();
                    ui.end_row();

                    let focus_dist = self
                        .state
                        .borrow()
                        .context
                        .scene
                        .read()
                        .unwrap()
                        .camera
                        .focus_dist;
                    let mut custom_focus_dist = focus_dist.is_some();
                    // println!("{custom_focus_dist}");
                    ui.label("focus distance: ");
                    if ui.checkbox(&mut custom_focus_dist, "Custom").changed() {
                        self.state
                            .borrow_mut()
                            .context
                            .scene
                            .write()
                            .unwrap()
                            .camera
                            .focus_dist = if custom_focus_dist {
                            Some(
                                self.state
                                    .borrow()
                                    .context
                                    .scene
                                    .read()
                                    .unwrap()
                                    .camera
                                    .get_default_focus_dist(),
                            )
                        } else {
                            None
                        }
                    }

                    ui.add_enabled_ui(custom_focus_dist, |ui| {
                        let mut focus_dist = focus_dist.unwrap_or(0.0);
                        if ui
                            .add(
                                egui::DragValue::new(&mut focus_dist)
                                    .clamp_range(0.0..=f32::INFINITY)
                                    .speed(0.05),
                            )
                            .changed()
                        {
                            self.state
                                .borrow_mut()
                                .context
                                .scene
                                .write()
                                .unwrap()
                                .camera
                                .focus_dist = Some(focus_dist);
                            changed = true;
                        }
                    });
                    ui.end_row();
                });
            });

        self.state.borrow_mut().changed = changed;
    }

    fn title(&mut self) -> WidgetText {
        "Scene".into()
    }
}

struct RenderTab {
    texture_handle: TextureHandle,
    state: State,
    rendering: bool,
    paused: bool,
}

const EMPTY_TEXTURE_COLOR: Color32 = Color32::BLACK;

impl RenderTab {
    pub fn new(egui_ctx: &egui::Context, state: State) -> Self {
        let texture_handle = egui_ctx.load_texture(
            "render",
            ImageData::Color(ColorImage::new(
                [
                    state.borrow().context.get_size().x as usize,
                    state.borrow().context.get_size().y as usize,
                ],
                EMPTY_TEXTURE_COLOR,
            )),
            TextureFilter::Nearest,
        );
        Self {
            texture_handle,
            state,
            rendering: false,
            paused: false,
        }
    }
}

impl Tab for RenderTab {
    fn ui(&mut self, ui: &mut Ui) {
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.add_enabled_ui(!self.rendering, |ui| {
                    if ui.button("📷").clicked() {
                        self.state.borrow().context.start_full_render();
                        self.rendering = true;
                        self.paused = false;
                    }
                });
                ui.add_enabled_ui(self.rendering, |ui| {
                    if ui.button(if self.paused { "▶" } else { "⏸" }).clicked() {
                        self.state.borrow().context.toggle_pause_full_render();
                        self.paused = !self.paused;
                    }
                    if ui.button("⏹").clicked() {
                        self.state.borrow_mut().context.stop_full_render();
                        self.rendering = false;
                    }
                });
                ui.end_row();
            });
        });

        {
            let size = self.state.borrow().context.get_size();
            if self.texture_handle.size()[0] != size.x as usize
                || self.texture_handle.size()[1] != size.y as usize
            {
                self.texture_handle.set(
                    ImageData::Color(ColorImage::new(
                        [size.x as usize, size.y as usize],
                        EMPTY_TEXTURE_COLOR,
                    )),
                    TextureFilter::Nearest,
                );
            }
        }

        self.state
            .borrow()
            .context
            .pixel_receiver
            .try_iter()
            .take(1000)
            .for_each(|(c, pos)| {
                self.texture_handle.set_partial(
                    [pos.x as usize, pos.y as usize],
                    ImageData::Color(ColorImage::new([1, 1], Color32::from_rgb(c[0], c[1], c[2]))),
                    TextureFilter::Linear,
                );
            });

        ui.centered_and_justified(|ui| {
            Plot::new("render")
                .legend(Legend::default())
                .show_x(false)
                .show_y(false)
                .data_aspect(1.0)
                .show_axes([false, false])
                .show_background(false)
                .show(ui, |pui| {
                    pui.image(PlotImage::new(
                        self.texture_handle.id(),
                        PlotPoint::new(0, 0),
                        Vec2::new(
                            self.texture_handle.size()[0] as f32,
                            self.texture_handle.size()[1] as f32,
                        ),
                    ));
                });
        });
    }

    fn title(&mut self) -> WidgetText {
        "Render".into()
    }
}

struct PreviewTab {
    texture_handle: TextureHandle,
    state: State,
    rendering: bool,
}

impl PreviewTab {
    pub fn new(egui_ctx: &egui::Context, state: State) -> Self {
        let texture_handle = egui_ctx.load_texture(
            "render",
            ImageData::Color(ColorImage::new(
                [
                    state.borrow().context.get_size().x as usize,
                    state.borrow().context.get_size().y as usize,
                ],
                EMPTY_TEXTURE_COLOR,
            )),
            TextureFilter::Nearest,
        );
        Self {
            texture_handle,
            state,
            rendering: false,
        }
    }
}

impl Tab for PreviewTab {
    fn ui(&mut self, ui: &mut Ui) {
        let size = self.state.borrow().context.get_size();
        let samples = self.state.borrow().context.samples;

        if ui.button(format!("📷 {}", samples)).clicked() {
            self.rendering = !self.rendering;
        }

        let reset = {
            let state = self.state.borrow(); //
            let camera = &mut state.context.scene.write().unwrap().camera;
            let input = ui.input();
            let mut should_update = true;
            let camera_speed = 0.1;
            if input.key_down(Key::A) {
                camera.eye -= (camera.at - camera.eye).cross(camera.up).normalize() * camera_speed;
            } else if input.key_down(Key::D) {
                camera.eye += (camera.at - camera.eye).cross(camera.up).normalize() * camera_speed;
            } else if input.key_down(Key::W) {
                camera.eye += (camera.at - camera.eye).normalize() * camera_speed;
            } else if input.key_down(Key::S) {
                camera.eye -= (camera.at - camera.eye).normalize() * camera_speed;
            } else if input.key_down(Key::Q) {
                camera.eye += camera.up.normalize() * camera_speed;
            } else if input.key_down(Key::E) {
                camera.eye -= camera.up.normalize() * camera_speed;
            } else {
                should_update = false;
            }

            should_update || state.changed
        };
        {
            let mut state = self.state.borrow_mut();
            if reset {
                state.context.reset_samples();
                state.changed = false
            }
        }
        {
            if self.texture_handle.size()[0] != size.x as usize
                || self.texture_handle.size()[1] != size.y as usize
                || reset
            {
                self.texture_handle.set(
                    ImageData::Color(ColorImage::new(
                        [size.x as usize, size.y as usize],
                        EMPTY_TEXTURE_COLOR,
                    )),
                    TextureFilter::Nearest,
                );
            }
        }

        if self.rendering {
            self.texture_handle.set(
                ImageData::Color(ColorImage::from_rgba_unmultiplied(
                    [size.x as usize, size.y as usize],
                    &self
                        .state
                        .borrow_mut()
                        .context
                        .render_sample()
                        .iter()
                        .map(|c| c.to_le_bytes())
                        .flatten()
                        .collect::<Vec<_>>(),
                )),
                TextureFilter::Linear,
            );
        }

        ui.centered_and_justified(|ui| {
            Plot::new("preview")
                .legend(Legend::default())
                .show_x(false)
                .show_y(false)
                .data_aspect(1.0)
                .show_axes([false, false])
                .show_background(false)
                .show(ui, |pui| {
                    pui.image(PlotImage::new(
                        self.texture_handle.id(),
                        PlotPoint::new(0, 0),
                        Vec2::new(
                            self.texture_handle.size()[0] as f32,
                            self.texture_handle.size()[1] as f32,
                        ),
                    ));
                });
        });
    }

    fn title(&mut self) -> WidgetText {
        "Preview".into()
    }
}

struct HeliochromeDriver {
    tree: DynamicTree,
    state: State,
}

impl HeliochromeDriver {
    fn new(cc: &CreationContext) -> Self {
        let state = Rc::new(RefCell::new(StateData::new(make_context())));
        let mut tree = DynamicTree::new(vec![
            Box::new(PreviewTab::new(&cc.egui_ctx, state.clone())),
            Box::new(RenderTab::new(&cc.egui_ctx, state.clone())),
        ]);

        let [a, b] = tree.split_left(
            NodeIndex::root(),
            0.2,
            vec![Box::new(SceneTab::new(state.clone()))],
        );
        let [_, _] = tree.split_below(b, 0.7, vec![Box::new(ConfigTab::new(state.clone()))]);

        Self { tree, state }
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
