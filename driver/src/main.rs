#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::{
    cell::RefCell,
    path::Path,
    rc::Rc,
    time::{Instant, SystemTime, UNIX_EPOCH},
};

use eframe::{
    egui::{
        self,
        plot::{Legend, Plot, PlotImage, PlotPoint},
        CollapsingHeader, ComboBox, Key, TextureFilter, TextureOptions, Ui, WidgetText,
    },
    epaint::{Color32, ColorImage, ImageData, TextureHandle, Vec2},
    CreationContext, NativeOptions,
};
use egui_dock::{DockArea, NodeIndex, Style, TabViewer, Tree};
use heliochrome::{
    color::Color, context::Context, image::Image, maths::vec2, tonemap::ToneMap, util::write_image,
};

mod make_context;
use make_context::make_context;

// NOTE: this is pretty much just a bad copy paste of egui_dock 2.1 code to update it to 5.0, no clue why this changed, i liked it (T_T)
pub trait Tab {
    fn ui(&mut self, ui: &mut Ui);

    fn title(&mut self) -> WidgetText;

    fn on_close(&mut self) -> bool {
        true
    }

    fn force_close(&mut self) -> bool {
        false
    }
}
struct DynamicTabViewer {}

impl TabViewer for DynamicTabViewer {
    type Tab = Box<dyn Tab>;

    fn ui(&mut self, ui: &mut Ui, tab: &mut Self::Tab) {
        tab.ui(ui)
    }

    fn title(&mut self, tab: &mut Self::Tab) -> WidgetText {
        tab.title()
    }

    fn on_close(&mut self, tab: &mut Self::Tab) -> bool {
        tab.on_close()
    }

    fn force_close(&mut self, tab: &mut Self::Tab) -> bool {
        tab.force_close()
    }
}

pub type DynamicTree = Tree<Box<dyn Tab>>;

struct StateData {
    pub changed: bool,
    pub gamma: f32,
    pub image: Image,
    pub context: Context,
}

impl StateData {
    pub fn new(context: Context) -> Self {
        Self {
            changed: false,
            gamma: if matches!(context.tone_map, ToneMap::HejlRichard) {
                1.0
            } else {
                2.2
            },
            image: Image::new(context.get_size()),
            context,
        }
    }

    pub fn save(&self) {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        if let Err(err) = write_image(
            Path::new(&format!("img_{}", now.as_secs())),
            self.context.get_size(),
            self.gamma,
            &self.image,
        ) {
            println!("{err}");
        }
    }
}

type State = Rc<RefCell<StateData>>;
struct ConfigTab {
    width: u16,
    height: u16,

    state: State,
}

impl ConfigTab {
    fn new(state: State) -> Self {
        let size = state.borrow().context.get_size();
        Self {
            width: size.x as u16,
            height: size.y as u16,
            state,
        }
    }
}

impl Tab for ConfigTab {
    fn ui(&mut self, ui: &mut Ui) {
        ui.label("image size: ");
        egui::Grid::new("size").show(ui, |ui| {
            ui.label("width:");
            let resw = ui.add(
                egui::DragValue::new(&mut self.width)
                    .clamp_range(1..=u16::MAX)
                    .suffix("px"),
            );
            ui.label("height:");
            let resh = ui.add(
                egui::DragValue::new(&mut self.height)
                    .clamp_range(1..=u16::MAX)
                    .suffix("px"),
            );
            ui.end_row();
            if resw.changed() || resh.changed() {
                let mut state = self.state.borrow_mut();
                let size = vec2::new(self.width as f32, self.height as f32);
                state.context.stop_full_render();
                state.context.resize(size);
                state.image = Image::new(size);
            }
        });

        egui::Grid::new("settings").show(ui, |ui| {
            ui.label("bounces: ");
            ui.add(
                egui::DragValue::new(&mut self.state.borrow_mut().context.quality.bounces)
                    .clamp_range(0..=u16::MAX),
            );
            ui.end_row();

            ui.label("samples: ");
            ui.add(
                egui::DragValue::new(&mut self.state.borrow_mut().context.quality.samples)
                    .clamp_range(0..=u16::MAX),
            );
            ui.end_row();
            ui.label("gamma: ");
            ui.add(
                egui::DragValue::new(&mut self.state.borrow_mut().gamma)
                    .clamp_range(0.0..=f32::INFINITY)
                    .speed(0.01),
            );
            ui.end_row();

            ui.label("Tone map mode: ");
            let mode = &mut self.state.borrow_mut().context.tone_map;
            ComboBox::from_label("")
                .selected_text(mode.to_string())
                .show_ui(ui, |ui| {
                    for m in &[
                        ToneMap::Clamp,
                        if matches!(mode, ToneMap::Simple(_)) {
                            *mode
                        } else {
                            ToneMap::Simple(1.0)
                        },
                        if matches!(mode, ToneMap::Reinhard(_)) {
                            *mode
                        } else {
                            ToneMap::Reinhard(1.0)
                        },
                        ToneMap::HejlRichard,
                        ToneMap::ACES,
                    ] {
                        ui.selectable_value(mode, *m, m.to_string());
                    }
                });
            ui.end_row();
            match mode {
                ToneMap::Reinhard(white_point) => {
                    ui.label("white point: ");
                    ui.add(egui::DragValue::new(white_point).clamp_range(0.0..=f32::INFINITY));
                    ui.end_row();
                }
                ToneMap::Simple(exposure) => {
                    ui.label("exposure: ");
                    ui.add(
                        egui::DragValue::new(exposure)
                            .clamp_range(0.0..=f32::INFINITY)
                            .speed(0.01),
                    );
                    ui.end_row();
                }

                _ => {}
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
                            .add(
                                egui::DragValue::new(
                                    &mut self
                                        .state
                                        .borrow_mut()
                                        .context
                                        .scene
                                        .write()
                                        .unwrap()
                                        .camera
                                        .vfov,
                                )
                                .clamp_range(0..=180),
                            )
                            .changed();
                    ui.end_row();

                    ui.label("aperture diameter: ");
                    changed = changed
                        || ui
                            .add(
                                egui::DragValue::new(
                                    &mut self
                                        .state
                                        .borrow_mut()
                                        .context
                                        .scene
                                        .write()
                                        .unwrap()
                                        .camera
                                        .aperture,
                                )
                                .clamp_range(0.0..=f32::INFINITY)
                                .speed(0.01),
                            )
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

                let state = self.state.borrow();
                let camera = &state.context.scene.read().unwrap().camera;

                ui.label(format!("eye: {:?}", camera.eye));
                ui.label(format!("at: {:?}", camera.at));
                ui.label(format!("up: {:?}", camera.up));
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
    start_time: Instant,
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
            TextureOptions {
                magnification: TextureFilter::Nearest,
                minification: TextureFilter::Nearest,
            },
        );
        Self {
            texture_handle,
            state,
            rendering: false,
            paused: false,
            start_time: Instant::now(),
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
                        let size = self.state.borrow().context.get_size();

                        self.texture_handle.set(
                            ImageData::Color(ColorImage::new(
                                [size.x as usize, size.y as usize],
                                EMPTY_TEXTURE_COLOR,
                            )),
                            TextureOptions {
                                magnification: TextureFilter::Nearest,
                                minification: TextureFilter::Nearest,
                            },
                        );

                        self.state.borrow_mut().image.buffer.fill(Color::new(
                            EMPTY_TEXTURE_COLOR.r() as f32 / 255.0,
                            EMPTY_TEXTURE_COLOR.g() as f32 / 255.0,
                            EMPTY_TEXTURE_COLOR.b() as f32 / 255.0,
                        ));
                    }
                });
                ui.add_enabled_ui(self.rendering, |ui| {
                    if ui.button(if self.paused { "▶" } else { "⏸" }).clicked() {
                        self.state.borrow().context.toggle_pause_full_render();
                        self.paused = !self.paused;
                        self.start_time = Instant::now();
                    }
                    if ui.button("⏹").clicked() {
                        self.state.borrow_mut().context.stop_full_render();
                        self.rendering = false;
                    }
                });
                ui.end_row();
            });
        });

        if self.rendering {
            let duration = Instant::now().duration_since(self.start_time);
            let seconds = duration.as_secs() % 60;
            let minutes = (duration.as_secs() / 60) % 60;
            let hours = (duration.as_secs() / 60) / 60;

            ui.label(format!(
                "Elapsed Time {:0>2}:{:0>2}:{:0>2}",
                hours, minutes, seconds
            ));

            {
                let size = self.state.borrow().context.get_size();
                if self.texture_handle.size()[0] != size.x as usize
                    || self.texture_handle.size()[1] != size.y as usize
                {
                    self.state.borrow_mut().image.buffer.fill(Color::new(
                        EMPTY_TEXTURE_COLOR.r() as f32 / 255.0,
                        EMPTY_TEXTURE_COLOR.g() as f32 / 255.0,
                        EMPTY_TEXTURE_COLOR.b() as f32 / 255.0,
                    ));

                    self.texture_handle.set(
                        ImageData::Color(ColorImage::new(
                            [size.x as usize, size.y as usize],
                            EMPTY_TEXTURE_COLOR,
                        )),
                        TextureOptions {
                            magnification: TextureFilter::Nearest,
                            minification: TextureFilter::Nearest,
                        },
                    );
                }
            }

            let mut img = self.state.borrow().image.clone();

            self.rendering = false;
            self.state
                .borrow()
                .context
                .pixel_receiver
                .try_iter()
                .take(500)
                .for_each(|(c, pos)| {
                    self.rendering = true;
                    img.set_pixel(&pos, c);
                });

            self.state.borrow_mut().image = img.clone();
            self.state.borrow().context.tone_map.map(&mut img);
            self.texture_handle.set(
                ImageData::Color(ColorImage::from_rgba_unmultiplied(
                    [img.size.x as usize, img.size.y as usize],
                    &img.to_gamma_corrected_rgba8(self.state.borrow().gamma),
                )),
                TextureOptions {
                    magnification: TextureFilter::Linear,
                    minification: TextureFilter::Linear,
                },
            );

            if !self.rendering {
                self.state.borrow_mut().image = img.clone();
            }
        }

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

        if ui.input(|i| i.key_down(Key::S)) && ui.input(|i| i.modifiers.ctrl) {
            self.state.borrow().save();
        }
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
            TextureOptions {
                magnification: TextureFilter::Nearest,
                minification: TextureFilter::Nearest,
            },
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
        let reset = {
            let state = self.state.borrow();
            let camera = &mut state.context.scene.write().unwrap().camera;
            // let input = ui.input(|i| i);
            let mut should_update = true;
            let camera_speed = (camera.at - camera.eye).mag() / 10.0;
            if !ui.input(|i| i.modifiers.ctrl) {
                if ui.input(|i| i.key_down(Key::A)) {
                    camera.eye -=
                        (camera.at - camera.eye).cross(camera.up).normalize() * camera_speed;
                } else if ui.input(|i| i.key_down(Key::D)) {
                    camera.eye +=
                        (camera.at - camera.eye).cross(camera.up).normalize() * camera_speed;
                } else if ui.input(|i| i.key_down(Key::W)) {
                    camera.eye += (camera.at - camera.eye).normalize() * camera_speed;
                } else if ui.input(|i| i.key_down(Key::S)) {
                    camera.eye -= (camera.at - camera.eye).normalize() * camera_speed;
                } else if ui.input(|i| i.key_down(Key::Q)) {
                    camera.eye += camera.up.normalize() * camera_speed;
                } else if ui.input(|i| i.key_down(Key::E)) {
                    camera.eye -= camera.up.normalize() * camera_speed;
                } else {
                    should_update = false;
                }
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
                    TextureOptions {
                        magnification: TextureFilter::Nearest,
                        minification: TextureFilter::Nearest,
                    },
                );
            }
        }
        let start_time = Instant::now();

        egui::Grid::new("size").show(ui, |ui| {
            if ui.button("📷").clicked() || ui.input(|i| i.key_down(Key::Space)) {
                self.rendering = !self.rendering;
            }
            ui.label(format!("samples: {}", samples));

            if self.rendering {
                let gamma = self.state.borrow().gamma;
                let image = self.state.borrow_mut().context.render_sample();
                self.state.borrow_mut().image = image;
                self.texture_handle.set(
                    ImageData::Color(ColorImage::from_rgba_unmultiplied(
                        [size.x as usize, size.y as usize],
                        &self.state.borrow().image.to_gamma_corrected_rgba8(gamma),
                    )),
                    TextureOptions {
                        magnification: TextureFilter::Linear,
                        minification: TextureFilter::Linear,
                    },
                );
            }
        });
        ui.label(if self.rendering {
            format!(
                "last render time: {:?}",
                Instant::now().duration_since(start_time)
            )
        } else {
            "".to_owned()
        });

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

        if ui.input(|i| i.key_down(Key::S)) && ui.input(|i| i.modifiers.ctrl) {
            self.state.borrow().save();
        }
    }

    fn title(&mut self) -> WidgetText {
        "Preview".into()
    }
}

struct HeliochromeDriver {
    tree: DynamicTree,
}

impl HeliochromeDriver {
    fn new(cc: &CreationContext) -> Self {
        let state = Rc::new(RefCell::new(StateData::new(make_context())));
        let mut tree = DynamicTree::new(vec![
            Box::new(PreviewTab::new(&cc.egui_ctx, state.clone())),
            Box::new(RenderTab::new(&cc.egui_ctx, state.clone())),
        ]);

        let [_, b] = tree.split_left(
            NodeIndex::root(),
            0.2,
            vec![Box::new(SceneTab::new(state.clone()))],
        );
        let [_, _] = tree.split_below(b, 0.7, vec![Box::new(ConfigTab::new(state.clone()))]);

        Self { tree }
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
    let options = NativeOptions {
        maximized: true,
        ..NativeOptions::default()
    };

    eframe::run_native(
        "Heliochrome",
        options,
        Box::new(|cc| Box::new(HeliochromeDriver::new(cc))),
    )
    .expect("oof");
}
