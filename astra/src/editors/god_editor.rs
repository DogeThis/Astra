use astra_types::{GodBondLevelData, GodBook, GodData, GodLevelData};
use egui::{DragValue, Ui};
use indexmap::IndexMap;

use crate::model::{CacheItem, CachedView, GodDataSheetRetriever};
use crate::widgets::{bitgrid_i32, id_field, keyed_add_modal_content};
use crate::{
    editable_list, editor_tab_strip, model_drop_down, msbt_key_value_multiline,
    msbt_key_value_singleline, EditorState, GodBondLevelDataSheet, GodDataSheet, GodLevelDataSheet,
    GroupEditorContent, ListEditorContent, PropertyGrid,
};

const FLAG_LABELS: &[&str] = &[
    "No Added Exp",
    "Enable Ring List",
    "Unit Icon Darkness",
    "Gauge Darkness",
    "Only Engage Weapon",
    "Armlet",
];

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Tab {
    Main,
    LevelData,
    BondLevelData,
}

pub struct GodEditor {
    tab: Tab,
    god: GodDataSheet,
    cache: CachedView<GodDataSheetRetriever, GodBook, GodData>,
    level_data: GodLevelDataSheet,
    bond_data: GodBondLevelDataSheet,
    main_content: ListEditorContent<IndexMap<String, GodData>, GodData, EditorState>,
    level_data_content: GroupEditorContent,
    bond_data_content: ListEditorContent<Vec<GodBondLevelData>, GodBondLevelData, ()>,
}

impl GodEditor {
    pub fn new(state: &EditorState) -> Self {
        Self {
            tab: Tab::Main,
            god: state.god.clone(),
            cache: CachedView::new(state.god.clone(), state),
            level_data: state.god_level_data.clone(),
            bond_data: state.god_bond_level_data.clone(),
            main_content: ListEditorContent::new("gods")
                .with_add_modal_content(&keyed_add_modal_content),
            level_data_content: GroupEditorContent::new("level_data"),
            bond_data_content: ListEditorContent::new("bond_data"),
        }
    }

    pub fn select(&mut self, index: Option<usize>) {
        self.main_content.select(index);
    }

    pub fn tab_strip(&mut self, ui: &mut Ui) {
        editor_tab_strip(ui, |ui| {
            ui.selectable_value(&mut self.tab, Tab::Main, "Main");
            ui.selectable_value(&mut self.tab, Tab::LevelData, "Level Data");
            ui.selectable_value(&mut self.tab, Tab::BondLevelData, "Bond Level Data");
        });
    }

    pub fn show(&mut self, ctx: &egui::Context, state: &mut EditorState) {
        match self.tab {
            Tab::Main => self.main_content.left_panel(ctx, &self.god, state),
            Tab::LevelData => self
                .level_data_content
                .left_panel(ctx, &self.level_data, state),
            Tab::BondLevelData => self.bond_data_content.left_panel(ctx, &self.bond_data, &()),
        }

        self.cache.refresh(state);

        match self.tab {
            Tab::Main => self.god.write(|data| {
                self.main_content.content(ctx, data, |ui, data| {
                    Self::god_property_grid(self.cache.get(), ui, data, state)
                })
            }),
            Tab::LevelData => self.level_data.write(|data| {
                self.level_data_content.content(ctx, data, |ui, data| {
                    Self::level_data_property_grid(ui, data, state)
                })
            }),
            Tab::BondLevelData => self.bond_data.write(|data| {
                self.bond_data_content.content(ctx, data, |ui, data| {
                    Self::bond_level_data_property_grid(ui, data)
                })
            }),
        }
    }

    fn god_property_grid(
        cache: &IndexMap<String, CacheItem<GodData>>,
        ui: &mut Ui,
        data: &mut GodData,
        state: &EditorState,
    ) -> bool {
        PropertyGrid::new("gods", data)
            .new_section("")
            .field("GID", |ui, god| ui.add(id_field(&mut god.gid)))
            .field("MID", |ui, god| {
                msbt_key_value_singleline!(ui, state, "gamedata", god.mid)
            })
            .field("Nickname", |ui, god| {
                ui.text_edit_singleline(&mut god.nickname)
            })
            .field("ASCII Name", |ui, god| {
                ui.text_edit_singleline(&mut god.ascii_name)
            })
            .field("Sound ID", |ui, god| {
                ui.text_edit_singleline(&mut god.sound_id)
            })
            .field("Asset ID", |ui, god| {
                ui.text_edit_singleline(&mut god.asset_id)
            })
            .field("Face Icon Name", |ui, god| {
                ui.text_edit_singleline(&mut god.face_icon_name)
            })
            .field("Face Icon (Corrupted)", |ui, god| {
                ui.text_edit_singleline(&mut god.face_icon_name_darkness)
            })
            .field("Ring Name", |ui, god| {
                msbt_key_value_singleline!(ui, state, "gamedata", god.ringname)
            })
            .field("Ring Help", |ui, god| {
                msbt_key_value_multiline!(ui, state, "gamedata", god.ringhelp)
            })
            .field("Unit Icon ID", |ui, god| {
                ui.text_edit_singleline(&mut god.unit_icon_id)
            })
            .field("Changed", |ui, god| {
                ui.add(editable_list(&mut god.change, |_, value, ui| {
                    ui.add(model_drop_down(cache, &(), value))
                }))
            })
            .default_field("Link", |god| &mut god.link)
            .field("Engage Haunt", |ui, god| {
                ui.text_edit_singleline(&mut god.engage_haunt)
            })
            .field("Level", |ui, god| ui.add(DragValue::new(&mut god.level)))
            .field("Force Type", |ui, god| {
                ui.add(DragValue::new(&mut god.force_type))
            })
            .field("Female", |ui, god| ui.add(DragValue::new(&mut god.female)))
            .field("Good Weapon", |ui, god| {
                ui.add(DragValue::new(&mut god.good_weapon))
            })
            .field("Sort", |ui, god| ui.add(DragValue::new(&mut god.sort)))
            .field("Engage Count", |ui, god| {
                ui.add(DragValue::new(&mut god.engage_count))
            })
            .field("Engage Attack", |ui, god| {
                state
                    .skill
                    .read(|data| ui.add(model_drop_down(data, state, &mut god.engage_attack)))
            })
            .field("Engage Attack Rampage", |ui, god| {
                state.skill.read(|data| {
                    ui.add(model_drop_down(data, state, &mut god.engage_attack_rampage))
                })
            })
            .field("Engage Attack Link", |ui, god| {
                state
                    .skill
                    .read(|data| ui.add(model_drop_down(data, state, &mut god.engage_attack_link)))
            })
            .field("Link Emblem", |ui, god| {
                ui.add(model_drop_down(cache, &(), &mut god.link_gid))
            })
            .field("Gbid", |ui, god| ui.text_edit_singleline(&mut god.gbid))
            .field("Grow Table", |ui, god| {
                ui.text_edit_singleline(&mut god.grow_table)
            })
            .field("Level Cap", |ui, god| {
                ui.add(DragValue::new(&mut god.level_cap))
            })
            .field("Unlock Level Cap Var Name", |ui, god| {
                ui.text_edit_singleline(&mut god.unlock_level_cap_var_name)
            })
            .field("Engrave Word", |ui, god| {
                ui.text_edit_singleline(&mut god.engrave_word)
            })
            .field("Engrave Power", |ui, god| {
                ui.add(DragValue::new(&mut god.engrave_power))
            })
            .field("Engrave Weight", |ui, god| {
                ui.add(DragValue::new(&mut god.engrave_weight))
            })
            .field("Engrave Hit", |ui, god| {
                ui.add(DragValue::new(&mut god.engrave_hit))
            })
            .field("Engrave Crit", |ui, god| {
                ui.add(DragValue::new(&mut god.engrave_critical))
            })
            .field("Engrave Avoid", |ui, god| {
                ui.add(DragValue::new(&mut god.engrave_avoid))
            })
            .field("Engrave Dodge", |ui, god| {
                ui.add(DragValue::new(&mut god.engrave_secure))
            })
            .field("Synchro HP Bonus", |ui, god| {
                ui.add(DragValue::new(&mut god.synchro_enhance_hp))
            })
            .field("Synchro STR Bonus", |ui, god| {
                ui.add(DragValue::new(&mut god.synchro_enhance_str))
            })
            .field("Synchro DEF Bonus", |ui, god| {
                ui.add(DragValue::new(&mut god.synchro_enhance_def))
            })
            .field("Synchro SKL Bonus", |ui, god| {
                ui.add(DragValue::new(&mut god.synchro_enhance_tech))
            })
            .field("Synchro SPD Bonus", |ui, god| {
                ui.add(DragValue::new(&mut god.synchro_enhance_quick))
            })
            .field("Synchro LCK Bonus", |ui, god| {
                ui.add(DragValue::new(&mut god.synchro_enhance_luck))
            })
            .field("Synchro MAG Bonus", |ui, god| {
                ui.add(DragValue::new(&mut god.synchro_enhance_magic))
            })
            .field("Synchro RES Bonus", |ui, god| {
                ui.add(DragValue::new(&mut god.synchro_enhance_mdef))
            })
            .field("Synchro CON Bonus", |ui, god| {
                ui.add(DragValue::new(&mut god.synchro_enhance_phys))
            })
            .field("Synchro MOV Bonus", |ui, god| {
                ui.add(DragValue::new(&mut god.synchro_enhance_move))
            })
            .field("Flag", |ui, god| {
                ui.add(bitgrid_i32(FLAG_LABELS, 3, &mut god.flag))
            })
            .field("Net Ranking Index", |ui, god| {
                ui.add(DragValue::new(&mut god.net_ranking_index))
            })
            .field("AI Engage Attack Type", |ui, god| {
                ui.add(DragValue::new(&mut god.ai_engage_attack_type))
            })
            .show(ui)
            .changed()
    }

    fn level_data_property_grid(ui: &mut Ui, data: &mut GodLevelData, state: &EditorState) -> bool {
        PropertyGrid::new("god_level_data", data)
            .new_section("")
            .field("Level", |ui, data| ui.add(DragValue::new(&mut data.level)))
            .field("Inheritance Skills", |ui, d| {
                state.skill.read(|data| {
                    ui.add(editable_list(&mut d.inheritance_skills, |_, value, ui| {
                        ui.add(model_drop_down(data, state, value))
                    }))
                })
            })
            .field("Synchro Skills", |ui, d| {
                state.skill.read(|data| {
                    ui.add(editable_list(&mut d.synchro_skills, |_, value, ui| {
                        ui.add(model_drop_down(data, state, value))
                    }))
                })
            })
            .field("Engage Skills", |ui, d| {
                state.skill.read(|data| {
                    ui.add(editable_list(&mut d.engage_skills, |_, value, ui| {
                        ui.add(model_drop_down(data, state, value))
                    }))
                })
            })
            .field("Engage Items", |ui, d| {
                state.item.read(|data| {
                    ui.add(editable_list(&mut d.engage_items, |_, value, ui| {
                        ui.add(model_drop_down(data, state, value))
                    }))
                })
            })
            .field("Engage (Infantry)", |ui, d| {
                state.skill.read(|data| {
                    ui.add(editable_list(&mut d.engage_cooperations, |_, value, ui| {
                        ui.add(model_drop_down(data, state, value))
                    }))
                })
            })
            .field("Engage (Cavalry)", |ui, d| {
                state.skill.read(|data| {
                    ui.add(editable_list(&mut d.engage_horses, |_, value, ui| {
                        ui.add(model_drop_down(data, state, value))
                    }))
                })
            })
            .field("Engage (Covert)", |ui, d| {
                state.skill.read(|data| {
                    ui.add(editable_list(&mut d.engage_coverts, |_, value, ui| {
                        ui.add(model_drop_down(data, state, value))
                    }))
                })
            })
            .field("Engage (Armored)", |ui, d| {
                state.skill.read(|data| {
                    ui.add(editable_list(&mut d.engage_heavys, |_, value, ui| {
                        ui.add(model_drop_down(data, state, value))
                    }))
                })
            })
            .field("Engage (Flier)", |ui, d| {
                state.skill.read(|data| {
                    ui.add(editable_list(&mut d.engage_flys, |_, value, ui| {
                        ui.add(model_drop_down(data, state, value))
                    }))
                })
            })
            .field("Engage (Magic)", |ui, d| {
                state.skill.read(|data| {
                    ui.add(editable_list(&mut d.engage_magics, |_, value, ui| {
                        ui.add(model_drop_down(data, state, value))
                    }))
                })
            })
            .field("Engage (Fist)", |ui, d| {
                state.skill.read(|data| {
                    ui.add(editable_list(&mut d.engage_pranas, |_, value, ui| {
                        ui.add(model_drop_down(data, state, value))
                    }))
                })
            })
            .field("Engage (Dragon)", |ui, d| {
                state.skill.read(|data| {
                    ui.add(editable_list(&mut d.engage_dragons, |_, value, ui| {
                        ui.add(model_drop_down(data, state, value))
                    }))
                })
            })
            .field("Aptitude", |ui, data| {
                ui.add(DragValue::new(&mut data.aptitude))
            })
            .field("Aptitude Cost (None)", |ui, data| {
                ui.add(DragValue::new(&mut data.aptitude_cost_none))
            })
            .field("Aptitude Cost (Sword)", |ui, data| {
                ui.add(DragValue::new(&mut data.aptitude_cost_sword))
            })
            .field("Aptitude Cost (Lance)", |ui, data| {
                ui.add(DragValue::new(&mut data.aptitude_cost_lance))
            })
            .field("Aptitude Cost (Axe)", |ui, data| {
                ui.add(DragValue::new(&mut data.aptitude_cost_axe))
            })
            .field("Aptitude Cost (Bow)", |ui, data| {
                ui.add(DragValue::new(&mut data.aptitude_cost_bow))
            })
            .field("Aptitude Cost (Dagger)", |ui, data| {
                ui.add(DragValue::new(&mut data.aptitude_cost_dagger))
            })
            .field("Aptitude Cost (Magic)", |ui, data| {
                ui.add(DragValue::new(&mut data.aptitude_cost_magic))
            })
            .field("Aptitude Cost (Rod)", |ui, data| {
                ui.add(DragValue::new(&mut data.aptitude_cost_rod))
            })
            .field("Aptitude Cost (Fist)", |ui, data| {
                ui.add(DragValue::new(&mut data.aptitude_cost_fist))
            })
            .field("Aptitude Cost (Special)", |ui, data| {
                ui.add(DragValue::new(&mut data.aptitude_cost_special))
            })
            .field("Flags", |ui, data| ui.add(DragValue::new(&mut data.flag)))
            .show(ui)
            .changed()
    }

    fn bond_level_data_property_grid(ui: &mut Ui, data: &mut GodBondLevelData) -> bool {
        PropertyGrid::new("god_bond_level_data", data)
            .new_section("")
            .default_field("Level", |d| &mut d.level)
            .default_field("Support Level", |d| &mut d.reliance_level)
            .default_field("Exp", |d| &mut d.exp)
            .default_field("Cost", |d| &mut d.cost)
            .show(ui)
            .changed()
    }
}
