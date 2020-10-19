pub fn initialize() {
    println!("entered game_data initialization\n\n");

    //let test_ship = Ship::create("TheTeastShiep Durr.2",34,30,80);
    //println!("{:?}", test_ship); 
}

#[derive(Default, Debug, Clone)]
pub struct Ship{
    name: String,
    hull: i32,
    hull_max: i32,
    armor: i32,
    armor_max: i32,
    armor_regen: i32,
    shield: i32,
    shield_max: i32,
    shield_regen: i32,
}

impl Ship{
    pub fn create(name_: &str, hull_: i32, armor_: i32, shield_: i32) -> Ship{
	Ship{
	    name: name_.to_string(),
	    hull: hull_,
	    hull_max: hull_,
	    armor: armor_,
	    armor_max: armor_,
	    armor_regen: 1,
	    shield: shield_,
	    shield_max: shield_,
	    shield_regen: 5,
	    
	}
    }
    pub fn take_damage(&mut self, damage: i32){
	println!("{} Damage taken!", damage);
    	if self.shield >= damage {
	    self.shield -= damage;
	}
	else if (self.armor + self.shield) >= damage {
	    self.armor -= damage - self.shield;
	    self.shield = 0;
	    println!("Shields are down!");
	}
	else if (self.hull + self.armor + self.shield) > damage{
	    self.hull -= damage - self.shield - self.armor;
	    self.armor = 0;
	    self.shield = 0;
	    println!("Armor is penetrated!");
	}
	else{
	    self.shield = 0;
	    self.armor = 0;
	    self.hull = 0;
	    println!("ship is destroyed!!");
	}
    }
    pub fn print_current(&self){
	println!("Ship \"{0}\", Shields: {1}, Armor: {2}, Hull: {3}",
		 self.name, self.shield, self.armor, self.hull);
    }
    pub fn is_alive(&self) -> bool{
	self.hull > 0 
    }
    pub fn regen(&mut self){
	self.armor += self.armor_regen;
	if self.armor > self.armor_max {
	    self.armor = self.armor_max;
	}
	self.shield += self.shield_regen;
	if self.shield > self.shield_max {
	    self.shield = self.shield_max;
	}
    }
}

