use quick_default::quick_default;
use quick_deref::quick_deref;

pub trait Animal {
    fn animal_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn animal_sound(&self) -> &'static str;
    fn yap(&self) {
        println!(
            "{} the {} the says {}",
            self.name(),
            self.animal_name(),
            self.animal_sound()
        )
    }
}

#[quick_default]
pub struct Cat {
    #[default("mittens")]
    pub name: &'static str,
    pub age: u8,
}

impl Animal for Cat {
    fn animal_name(&self) -> &'static str {
        "Cat"
    }
    fn name(&self) -> &'static str {
        self.name
    }
    fn animal_sound(&self) -> &'static str {
        "meow"
    }
}

#[quick_deref]
pub struct AnyAnimal(Box<dyn Animal>);

impl AnyAnimal {
    pub fn new_cat() -> AnyAnimal {
        let inner: Box<dyn Animal> = Box::new(Cat::default());
        AnyAnimal(inner)
    }
}

fn main() {
    let cat = AnyAnimal::new_cat();
    cat.yap();
}
