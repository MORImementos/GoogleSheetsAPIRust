use eframe::{egui};
// use eframe::egui::Context;


#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Table {
    striped: bool,
    resizable: bool,
    clickable: bool,
    num_rows: usize,
    scroll_to_row_slider: usize,
    scroll_to_row: Option<usize>,
    selection: std::collections::HashSet<usize>,
    checked: bool,
    data: Vec<Vec<String>>,
}

pub trait View {
    fn ui(&mut self, ui: &mut egui::Ui);
}

pub trait Viewable {
    fn is_enabled(&self, _ctx: &egui::Context) -> bool {
        true
    }

    fn name(&self) -> &'static str;

    fn show(&mut self, ctx: &egui::Context, open: &mut bool);
}


impl Viewable for Table {
    fn name(&self) -> &'static str {
        "Google Sheets Table"
    }

    fn show(&mut self, ctx: &egui::Context, open: &mut bool) {
        egui::Window::new(self.name())
            .default_width(320.0)
            .default_height(480.0)
            .open(open)
            .show(ctx, |ui| {
                use View as _;
                self.ui(ui);
            });
    }
}

impl Default for Table {
    fn default() -> Self {
        Self {
            striped: true,
            resizable: true,
            clickable: true,
            num_rows: 1000,
            scroll_to_row_slider: 0,
            scroll_to_row: None,
            selection: Default::default(),
            checked: false,
            data: Vec::new(),
        }
    }
}

impl View for Table {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                ui.checkbox(&mut self.striped, "Striped");
                ui.checkbox(&mut self.resizable, "Resizable columns");
                ui.checkbox(&mut self.clickable, "Clickable rows");
            });

            let max_rows = self.num_rows;
            ui.add(
                egui::Slider::new(&mut self.num_rows, 0..=100_000)
                    .logarithmic(true)
                    .text("Num rows"),
            );
            let slider_response = ui.add(
                egui::Slider::new(&mut self.scroll_to_row_slider, 0..=max_rows)
                    .logarithmic(true)
                    .text("Row to scroll to"),
            );
            if slider_response.changed() {
                self.scroll_to_row = Some(self.scroll_to_row_slider);
            }
        });

        ui.separator();

        for row in &self.data {
            ui.horizontal(|ui| {
                for cell in row {
                    ui.label(cell);
                }
            });
        }
    }
}

impl Table {
    fn table_ui(&mut self, ui: &mut egui::Ui) {
        use egui_extras::{Column, TableBuilder};

        let text_height = egui::TextStyle::Body
            .resolve(ui.style())
            .size
            .max(ui.spacing().interact_size.y);

        let mut table = TableBuilder::new(ui)
            .striped(self.striped)
            .resizable(self.resizable)
            .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
            .column(Column::auto())
            .column(Column::auto())
            .column(Column::initial(100.0).range(40.0..=300.0))
            .column(Column::initial(100.0).at_least(40.0).clip(true))
            .column(Column::remainder())
            .min_scrolled_height(0.0);

        if self.clickable {
            table = table.sense(egui::Sense::click());
        }

        if let Some(row_index) = self.scroll_to_row.take() {
            table = table.scroll_to_row(row_index, None);
        }

        table
            .header(20.0, |mut header| {
                header.col(|ui| {
                    ui.strong("Row");
                });
                header.col(|ui| {
                    ui.strong("Interaction");
                });
                header.col(|ui| {
                    ui.strong("Expanding content");
                });
                header.col(|ui| {
                    ui.strong("Clipped text");
                });
                header.col(|ui| {
                    ui.strong("Content");
                });
            })
            .body(|body| {
                body.rows(text_height, self.num_rows, |mut row| {
                    let row_index = row.index();
                    row.set_selected(self.selection.contains(&row_index));

                    row.col(|ui| {
                        ui.label(row_index.to_string());
                    });
                    row.col(|ui| {
                        ui.checkbox(&mut self.checked, "Click me");
                    });
                    row.col(|ui| {
                        expanding_content(ui);
                    });
                    row.col(|ui| {
                        ui.label(long_text(row_index));
                    });
                    row.col(|ui| {
                        ui.add(
                            egui::Label::new("Thousands of rows of even height").wrap(false),
                        );
                    });
                    self.toggle_row_selection(row_index, &row.response());
                });
            })
    }

    fn toggle_row_selection(&mut self, row_index: usize, row_response: &egui::Response) {
        if row_response.clicked() {
            if self.selection.contains(&row_index) {
                self.selection.remove(&row_index);
            } else {
                self.selection.insert(row_index);
            }
        }
    }

    pub fn update_data(&mut self, new_data: Vec<Vec<String>>) {
        self.data = new_data;
    }
}
fn expanding_content(ui: &mut egui::Ui) {
    let width = ui.available_width().clamp(20.0, 200.0);
    let height = ui.available_height();
    let (rect, _response) = ui.allocate_exact_size(egui::vec2(width, height), egui::Sense::hover());
    ui.painter().hline(
        rect.x_range(),
        rect.center().y,
        (1.0, ui.visuals().text_color()),
    );
}

fn long_text(row_index: usize) -> String {
    format!("Row {row_index} has some long text that you may want to clip, or it will take up too much horizontal space!")
}

fn thick_row(row_index: usize) -> bool {
    row_index % 6 == 0
}


pub struct Display {
    pub table: Table,
}

impl eframe::App for Display {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.table.show(ctx, &mut true);
    }
}