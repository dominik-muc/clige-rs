/* use std::any::{Any, TypeId};

use crate::object::{entity::Enemy, Entity};

use super::{Item, ItemFactory, Object, ObjectFactory};

pub struct Weapon {
    name: String,
    damage: i32,
}

impl Object for Weapon {
    fn get_name(&self) -> String {
        self.name
    }
}

impl Item for Weapon {
    fn use_on(&mut self, target: Box<dyn Object>) {
        println!(
            "Used {} on {} dealing {} damage.",
            self.name,
            target.get_name(),
            self.damage
        );
        let t: Box<dyn Entity> = target.downcast::<dyn Entity>().unwrap();
    }
}

pub struct WeaponFactory<'a> {
    name: &'a str,
    damage: i32,
}

impl<'a> ObjectFactory<'a> for WeaponFactory<'a> {
    fn new() -> Self {
        WeaponFactory {
            name: "Unnamed item",
            damage: 0,
        }
    }

    fn name(mut self, name: &'a str) -> Self {
        self.name = name;
        self
    }

    fn create_object(&self) -> Weapon {
        Weapon {
            name: self.name,
            damage: self.damage,
        }
    }
}

impl<'a> ItemFactory<'a> for WeaponFactory<'a> {}

impl<'a> WeaponFactory<'a> {
    fn damage(mut self, amount: i32) -> Self {
        self.damage = amount;
        self
    }
}

pub struct Food {}

pub struct FoodFactory {}

pub fn test() {
    /* let apple_factory = FoodFactory::new().name("Apple").health(10);
    let apple1 = apple_factory.create_object();
    let apple2 = apple_factory.create_object(); */

    let sword_factory = WeaponFactory::new().name("Sword").damage(10);

    let some_sword = sword_factory.create_object();
}
 */
