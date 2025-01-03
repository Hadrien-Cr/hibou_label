/*
Copyright 2020 Erwan Mahe (github.com/erwanM974)

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
*/


use std::collections::{HashMap, HashSet};
use std::fmt::Formatter;
use std::hash::Hash;
use rand::prelude::{SliceRandom, StdRng};
use rand::Rng;
use crate::core::general_context::GeneralContext;
use crate::core::language::syntax::action::{CommunicationSynchronicity, EmissionAction, ReceptionAction};
use crate::core::language::syntax::interaction::{Interaction, LoopKind};
use crate::core::language::syntax::metrics::SymbolKind;




#[derive(IntoStaticStr, EnumIter, Clone, PartialEq, Eq, PartialOrd, Copy, Ord, Hash, Debug)]
pub enum InteractionGenerationSymbol {
    Empty,
    Action,
    Strict,
    Seq,
    Coreg,
    Par,
    LoopS,
    LoopW,
    LoopP,
    Alt,
    Basic,
    Transmission,
    Broadcast
}


pub struct InteractionSymbolsProbabilities {
    pub ordered_symbols : Vec<InteractionGenerationSymbol>,
    pub ordered_bounds : Vec<f32>
}



#[derive(Clone, PartialEq, Eq, PartialOrd, Copy, Ord, Hash, Debug)]
pub enum InteractionSymbolsProbabilitiesError {
    SymbolProbabilityMustBeBetweenOAnd1,
    SumOfProbabilitiesMustBe1
}


impl InteractionSymbolsProbabilities {

    pub fn from_map(map : HashMap<InteractionGenerationSymbol,f32>) -> Result<Self,InteractionSymbolsProbabilitiesError> {
        let mut ordered_symbols = vec![];
        let mut ordered_bounds = vec![0.0_f32];
        let mut sum = 0.0;
        for (s,p) in map {
            if p < 0.0 - 1e-6 || p > 1.0 + 1e-6 {
                return Err(InteractionSymbolsProbabilitiesError::SymbolProbabilityMustBeBetweenOAnd1);
            }
            ordered_symbols.push(s);
            sum += p;
            ordered_bounds.push(sum);
        }
        if sum < 1.0-1e-6 || sum > 1.0 +1e-6 {
            return Err(InteractionSymbolsProbabilitiesError::SumOfProbabilitiesMustBe1);
        }
        assert!(ordered_bounds.len() == ordered_symbols.len() +1);
        // ***
        Ok(Self{ordered_symbols,ordered_bounds})
    }

    pub fn protocols_with_coreg() -> InteractionSymbolsProbabilities {
        let map = hashmap!{
            InteractionGenerationSymbol::Empty      => 0.025,
            InteractionGenerationSymbol::Transmission     => 0.175,
            // total 0.2
            InteractionGenerationSymbol::Broadcast     => 0.1,
            // total 0.3
            InteractionGenerationSymbol::Seq        => 0.2,
            // total 0.5
            InteractionGenerationSymbol::Coreg        => 0.1,
            // total 0.6
            InteractionGenerationSymbol::Alt        => 0.2,
            // total 0.8
            InteractionGenerationSymbol::LoopW       => 0.15,
            // total 0.95
            InteractionGenerationSymbol::Par => 0.05
            // total 1.0
        };
        Self::from_map(map).unwrap()
    }

    pub fn conservative() -> InteractionSymbolsProbabilities {
        let map = hashmap!{
            InteractionGenerationSymbol::Empty      => 0.025,
            InteractionGenerationSymbol::Basic     => 0.175,
            // total 0.2
            InteractionGenerationSymbol::Action     => 0.15,
            // total 0.35
            InteractionGenerationSymbol::Seq        => 0.3,
            // total 0.65
            InteractionGenerationSymbol::Alt        => 0.15,
            // total 0.8
            InteractionGenerationSymbol::LoopW       => 0.15,
            // total 0.95
            InteractionGenerationSymbol::Par => 0.05
            // total 1.0
        };
        Self::from_map(map).unwrap()
    }

    pub fn default_non_regular() -> InteractionSymbolsProbabilities {
        let map = hashmap!{
            InteractionGenerationSymbol::Empty      => 0.025,
            InteractionGenerationSymbol::Basic     => 0.125,
            // total 0.15
            InteractionGenerationSymbol::Action     => 0.05,
            // total 0.2
            InteractionGenerationSymbol::Seq        => 0.3,
            // total 0.5
            InteractionGenerationSymbol::Alt        => 0.15,
            // total 0.65
            InteractionGenerationSymbol::LoopW       => 0.15,
            // total 0.8
            InteractionGenerationSymbol::LoopP       => 0.1,
            // total 0.9
            InteractionGenerationSymbol::Par => 0.1
            // total 1.0
        };
        Self::from_map(map).unwrap()
    }

    pub fn default_regular() -> InteractionSymbolsProbabilities {
        let map = hashmap!{
            InteractionGenerationSymbol::Empty      => 0.025,
            InteractionGenerationSymbol::Action     => 0.175,
            // total 0.2
            InteractionGenerationSymbol::Strict     => 0.1,
            // total 0.3
            InteractionGenerationSymbol::Seq        => 0.3,
            // total 0.6
            InteractionGenerationSymbol::Alt        => 0.15,
            // total 0.75
            InteractionGenerationSymbol::LoopS       => 0.15,
            // total 0.9
            InteractionGenerationSymbol::Par => 0.1
            // total 1.0
        };
        Self::from_map(map).unwrap()
    }

    pub fn default_high_level_regular() -> InteractionSymbolsProbabilities {
        let map = hashmap!{
            InteractionGenerationSymbol::Empty      => 0.05,
            InteractionGenerationSymbol::Action      => 0.15,
            // total 0.2
            InteractionGenerationSymbol::Basic      => 0.3,
            // total 0.5
            InteractionGenerationSymbol::Strict     => 0.15,
            // total 0.65
            InteractionGenerationSymbol::Par        => 0.05,
            // total 0.7
            InteractionGenerationSymbol::LoopS      => 0.1,
            // total 0.8
            InteractionGenerationSymbol::Alt        => 0.2
            // total 1.0
        };
        Self::from_map(map).unwrap()
    }

    // Custom probabilities
    pub fn custom(
        pempty: f32,
        paction: f32,
        pstrict: f32,
        pseq: f32,
        pcoreg: f32,
        ppar: f32,
        ploops: f32,
        ploopw: f32,
        ploopp: f32,
        palt: f32,
        pbasic: f32,
        ptr: f32,
        pbc: f32,
    ) -> InteractionSymbolsProbabilities {
        let map = hashmap!{
            InteractionGenerationSymbol::Empty => pempty,
            InteractionGenerationSymbol::Action => paction,
            InteractionGenerationSymbol::Strict => pstrict,
            InteractionGenerationSymbol::Seq => pseq,
            InteractionGenerationSymbol::Coreg => pcoreg,
            InteractionGenerationSymbol::Par => ppar,
            InteractionGenerationSymbol::LoopS => ploops,
            InteractionGenerationSymbol::LoopW => ploopw,
            InteractionGenerationSymbol::LoopP => ploopp,
            InteractionGenerationSymbol::Alt => palt,
            InteractionGenerationSymbol::Basic => pbasic,
            InteractionGenerationSymbol::Transmission => ptr,
            InteractionGenerationSymbol::Broadcast => pbc,
        };
        Self::from_map(map).unwrap()
    }


    pub fn default_basic() -> InteractionSymbolsProbabilities {
        let map = hashmap!{
            InteractionGenerationSymbol::Action     => 0.4,
            // total 0.4
            InteractionGenerationSymbol::Strict     => 0.4,
            // total 0.8
            InteractionGenerationSymbol::Seq        => 0.2,
            // total 1.0
        };
        Self::from_map(map).unwrap()
    }

    pub fn get_random_symbol(&self, rng : &mut StdRng) -> InteractionGenerationSymbol {
        let got = rng.gen_range(0.0_f32..1.0_f32);
        for (idx,x) in self.ordered_bounds.iter().enumerate() {
            if got <= *x + 1e-6 {
                if idx == 0 {
                    return *self.ordered_symbols.get(0).unwrap();
                } else {
                    return *self.ordered_symbols.get(idx-1).unwrap();
                }
            }
        }
        panic!()
    }
}

impl std::fmt::Display for InteractionSymbolsProbabilities {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {:?}", self.ordered_symbols, self.ordered_bounds)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mutate_remove_test() {
        let probas = InteractionSymbolsProbabilities::default_regular();
        println!("{:}", probas);
    }

}

