extern crate sadmc;

use sadmc::system::lattice_gas::*;

use sadmc::mc::MonteCarlo;
use sadmc::mc::energy_number_no_translation::EnergyNumberMC;

fn main() {
    let mut mc = EnergyNumberMC::<LatticeGas>::from_args::<LatticeGasParams>();
    loop {
        mc.move_once();
    }
}
