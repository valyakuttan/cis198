use rand;

const MAX_CHEST_VAL: u32 = 100;
const MAX_TRAP_VAL: u32 = 10;
const MAX_FOOD_VAL: u32 = 10;
const NUM_CURIO_VARIANTS: usize = 5;
const NUM_NONRECURSIVE_CURIOS: usize = 3;

#[derive(Clone)]
pub enum Curio {
    Chest(i32),
    SpikeTrap(i32),
    Food(i32),
    IronMaiden(Box<Curio>, i32),
    FallenAdventurer(Box<Curio>),
}

impl Curio {
    pub fn generate_n(n: usize) -> Vec<Curio> {
        let mut acc = Vec::new();
        for _ in 0..n { acc.push(Curio::generate()); }
        acc
    }

    pub fn generate() -> Curio {
        match rand::random::<usize>() % NUM_CURIO_VARIANTS {
            0 => Curio::rand_chest(),
            1 => Curio::rand_spike_trap(),
            2 => Curio::rand_food(),
            3 => Curio::IronMaiden(
                Box::new(Curio::generate_sub_curio()),
                ((rand::random::<u32>() % MAX_TRAP_VAL) + 1) as i32
            ),
            4 => Curio::FallenAdventurer(
                Box::new(Curio::generate_sub_curio())
            ),
            _ => unreachable!(),
        }
    }

    fn rand_chest() -> Curio {
        Curio::Chest(
            ((rand::random::<u32>() % MAX_CHEST_VAL) + 1) as i32
        )
    }

    fn rand_spike_trap() -> Curio {
        Curio::SpikeTrap(
            ((rand::random::<u32>() % MAX_TRAP_VAL) + 1) as i32
        )
    }

    fn rand_food() -> Curio {
        Curio::Food(((rand::random::<u32>() % MAX_FOOD_VAL) + 1) as i32)
    }

    fn generate_sub_curio() -> Curio {
        match rand::random::<usize>() % NUM_NONRECURSIVE_CURIOS {
            0 => Curio::rand_chest(),
            1 => Curio::rand_spike_trap(),
            2 => Curio::rand_food(),
            _ => unreachable!(),
        }
    }
}
