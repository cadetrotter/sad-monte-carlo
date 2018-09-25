//! A square well fluid.

use super::*;

use rand::prelude::*;

/// The parameters needed to configure a square well system.
#[derive(Serialize, Deserialize, Debug, ClapMe)]
#[allow(non_snake_case)]
pub struct LatticeGasParams {
    /// Width of the square grid
    L: usize,
}

#[allow(non_snake_case)]
/// A square well fluid.
#[derive(Serialize, Deserialize, Debug)]
pub struct LatticeGas {
    /// The energy of the system
    E: Energy,
    /// The dimensions of the box.
    pub L: usize,
    /// The spins themselves
    N: Vec<i8>,
    /// The last change we made (and might want to undo).
    possible_change: Option<(usize, Energy)>,
    /// Number of atoms
    natoms: usize,
}

impl From<LatticeGasParams> for LatticeGas {
    fn from(params: LatticeGasParams) -> LatticeGas {
        let mut ising = LatticeGas {
            E: Energy::new(0.),
            L: params.L,
            N: vec![0; params.L*params.L],
            natoms: 0,
            possible_change: None,
        };
        assert!(ising.L > 1); // otherwise, we are our own neighbor!
        ising.E = ising.compute_energy();
        ising
    }
}

impl System for LatticeGas {
    fn energy(&self) -> Energy {
        self.E
    }
    fn compute_energy(&self) -> Energy {
        let mut e: Energy = units::EPSILON*0.0;
        for i1 in 0 .. self.L {
            for j1 in 0 .. self.L {
                let j2 = (j1 + 1) % self.L;
                let mut neighbor_tot = self.N[i1 + j2*self.L];
                // let j2 = modulus(j - 1, self.L);
                // neighbor_tot += self.N[i1 + j2*self.L];
                let i2 = (i1 + 1) % self.L;
                neighbor_tot += self.N[i2 + j1*self.L];
                // let i2 = modulus(i - 1, self.L);
                // neighbor_tot += self.N[i1 + j2*self.L];
                e -= neighbor_tot as f64 * Energy::new(self.N[i1+j1*self.L] as f64);
            }
        }
        e
    }
    fn delta_energy(&self) -> Option<Energy> {
        Some(units::EPSILON)
    }
}

impl ConfirmSystem for LatticeGas {
    fn confirm(&mut self) {
        if let Some((i, e)) = self.possible_change {
            self.N[i] ^= 1;
            if self.N[i] == 1 {
                self.natoms += 1;
            } else {
                self.natoms -= 1;
            }
            self.E = e;
        }
    }
}

impl MovableSystem for LatticeGas {
    fn plan_move(&mut self, _: &mut MyRng, _: Length) -> Option<Energy> {
        None
    }
}

impl GrandSystem for LatticeGas {
    fn plan_add(&mut self, rng: &mut MyRng) -> Option<Energy> {
        if self.natoms == self.L*self.L {
            // No room for more!
            self.possible_change = None;
            return None;
        }
        let mut i = rng.gen_range(0, self.L);
        let mut j = rng.gen_range(0, self.L);
        while self.N[i+j*self.L] == 1 {
            i = rng.gen_range(0, self.L);
            j = rng.gen_range(0, self.L);
        }
        let j2 = (j + 1) % self.L;
        let mut neighbor_tot = self.N[i + j2*self.L];
        let j2 = (j + self.L - 1) % self.L;
        neighbor_tot += self.N[i + j2*self.L];
        let i2 = (i + 1) % self.L;
        neighbor_tot += self.N[i2 + j*self.L];
        let i2 = (i + self.L - 1) % self.L;
        neighbor_tot += self.N[i2 + j*self.L];
        let e = self.E - neighbor_tot as f64 * units::EPSILON;
        self.possible_change = Some((i+j*self.L, e));
        Some(e)
    }
    fn plan_remove(&mut self, rng: &mut MyRng) -> Energy {
        if self.natoms == 0 {
            self.possible_change = None;
            return self.E;
        }
        let mut i = rng.gen_range(0, self.L);
        let mut j = rng.gen_range(0, self.L);
        while self.N[i+j*self.L] == 0 {
            i = rng.gen_range(0, self.L);
            j = rng.gen_range(0, self.L);
        }
        let j2 = (j + 1) % self.L;
        let mut neighbor_tot = self.N[i + j2*self.L];
        let j2 = (j + self.L - 1) % self.L;
        neighbor_tot += self.N[i + j2*self.L];
        let i2 = (i + 1) % self.L;
        neighbor_tot += self.N[i2 + j*self.L];
        let i2 = (i + self.L - 1) % self.L;
        neighbor_tot += self.N[i2 + j*self.L];
        let e = self.E + neighbor_tot as f64 * units::EPSILON;
        self.possible_change = Some((i+j*self.L, e));
        e
    }
    fn num_atoms(&self) -> usize {
        self.natoms
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
fn energy_works_with_L(L: usize) {
    let mut ising = LatticeGas::from(LatticeGasParams { L: L });

    println!("starting energy...");
    assert_eq!(ising.energy(), ising.compute_energy());

    let mut rng = ::rng::MyRng::from_u64(10137);
    for _ in 0..10000 {
        if rng.gen::<f64>() < 0.5 {
            ising.plan_add(&mut rng);
        } else {
            ising.plan_remove(&mut rng);
        }
        ising.confirm();
        assert_eq!(ising.energy(), ising.compute_energy());
    }
}

#[test]
fn energy_works() {
    for &n in &[2,3,10,15,137,150] {
        println!("testing with L={}", n);
        energy_works_with_L(n);
    }
}
