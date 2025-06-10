use std::collections::HashMap;

use once_cell::sync::Lazy;

#[derive(Debug)]
pub struct Skill {
    pub skill_id: String,
    pub skill_cost: u32,
    pub damage: u32,
    pub healing: u32,
    pub area: SkillArea,
}

#[derive(Debug)]
pub struct SkillArea {
    pub shape: SkillAreaShape,
    pub offset: f32,
}

#[derive(Debug)]
pub enum SkillAreaShape {
    Circle(f32),
    Rectangle(f32, f32),
}

pub static SKILL_REGISTRY: Lazy<HashMap<String, Skill>> = Lazy::new(|| {
    let mut registry = HashMap::new();

    registry.insert(
        "normal_attack_100".to_owned(),
        Skill {
            skill_id: "normal_attack_100".to_owned(),
            skill_cost: 0,
            damage: 100,
            healing: 0,
            area: SkillArea {
                shape: SkillAreaShape::Circle(5.),
                offset: 0.,
            },
        },
    );

    registry
});
