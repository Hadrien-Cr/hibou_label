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


use pest::iterators::Pair;

use crate::core::general_context::GeneralContext;
use crate::io::input::error::HibouParsingError;
use crate::io::input::hcf::proc_options::loggers::{parse_graphic_logger, parse_tracegen_logger};
use crate::process::abstract_proc::common::HibouSearchStrategy;
use crate::process::abstract_proc::manager::GenericProcessPriorities;
use crate::process::explo_proc::interface::conf::ExplorationConfig;
use crate::process::explo_proc::interface::filter::ExplorationFilter;
use crate::process::explo_proc::interface::logger::ExplorationLogger;
use crate::process::explo_proc::interface::priorities::ExplorationPriorities;


#[allow(unused_imports)]
use crate::pest::Parser;
#[allow(unused_imports)]
use crate::io::input::hcf::parser::{HcfParser,Rule};


pub struct HibouExploreOptions {
    pub loggers : Vec<Box<dyn ExplorationLogger>>,
    pub strategy : HibouSearchStrategy,
    pub filters : Vec<ExplorationFilter>,
    pub priorities : GenericProcessPriorities<ExplorationConfig>
}



impl HibouExploreOptions {
    pub fn new(loggers : Vec<Box<dyn ExplorationLogger>>,
               strategy : HibouSearchStrategy,
               filters : Vec<ExplorationFilter>,
               priorities : GenericProcessPriorities<ExplorationConfig>) -> HibouExploreOptions {
        return HibouExploreOptions{loggers,strategy,filters,priorities};
    }

    pub fn default() -> HibouExploreOptions {
        return HibouExploreOptions::new(Vec::new(),
            HibouSearchStrategy::BFS,
            vec![ExplorationFilter::MaxLoopInstanciation(1)],
                                        GenericProcessPriorities::Specific(ExplorationPriorities::default()));
    }

}




pub fn parse_explore_options(gen_ctx: &GeneralContext,
                             option_pair : Pair<Rule>,
                             file_name : &str) -> Result<HibouExploreOptions,HibouParsingError> {
    let mut loggers : Vec<Box<dyn ExplorationLogger>> = Vec::new();
    let mut strategy : HibouSearchStrategy = HibouSearchStrategy::BFS;
    let mut filters : Vec<ExplorationFilter> = Vec::new();
    let mut priorities : GenericProcessPriorities<ExplorationConfig> = GenericProcessPriorities::Specific(ExplorationPriorities::default());
    // ***
    for option_decl_pair in option_pair.into_inner() {
        match option_decl_pair.as_rule() {
            Rule::OPTION_LOGGER_DECL => {
                loggers = Vec::new();
                // ***
                let mut logger_id : u32 = 0;
                for logger_kind_pair in option_decl_pair.into_inner() {
                    logger_id += 1;
                    match logger_kind_pair.as_rule() {
                        Rule::OPTION_GRAPHIC_LOGGER => {
                            let glogger = parse_graphic_logger(logger_id,file_name,logger_kind_pair);
                            loggers.push( Box::new(glogger));
                        },
                        Rule::OPTION_TRACEGEN_LOGGER => {
                            match parse_tracegen_logger(logger_id, gen_ctx,file_name,logger_kind_pair) {
                                Err(e) => {
                                    return Err(e);
                                },
                                Ok( tlogger ) => {
                                    loggers.push( Box::new(tlogger));
                                }
                            }
                        }
                        _ => {
                            panic!("what rule then ? : {:?}", logger_kind_pair.as_rule() );
                        }
                    }
                }
            },
            Rule::OPTION_STRATEGY_DECL => {
                let strategy_pair =  option_decl_pair.into_inner().next().unwrap();
                match strategy_pair.as_rule() {
                    Rule::OPTION_STRATEGY_BFS => {
                        strategy = HibouSearchStrategy::BFS;
                    },
                    Rule::OPTION_STRATEGY_DFS => {
                        strategy = HibouSearchStrategy::DFS;
                    },
                    Rule::OPTION_STRATEGY_HCS => {
                        strategy = HibouSearchStrategy::HCS;
                    },
                    _ => {
                        panic!("what rule then ? : {:?}", strategy_pair.as_rule() );
                    }
                }
            },
            Rule::OPTION_FILTERS_DECL => {
                match parse_filters(option_decl_pair) {
                    Ok( got_filters) => {
                        filters = got_filters;
                    },
                    Err(e) => {
                        return Err(e);
                    }
                }
            },
            Rule::OPTION_PRIORITIES_DECL => {
                let inner : Pair<Rule> = option_decl_pair.into_inner().next().unwrap();
                match inner.as_rule() {
                    Rule::OPTION_PRIORITY_SPECIFIC => {
                        match parse_specific_priorities(inner) {
                            Ok( got_priorities) => {
                                priorities = GenericProcessPriorities::Specific(got_priorities);
                            },
                            Err(e) => {
                                return Err(e);
                            }
                        }
                    },
                    Rule::OPTION_PRIORITY_RANDOM => {
                        priorities = GenericProcessPriorities::Random;
                    },
                    _ => {
                        panic!("what rule then ? : {:?}", inner.as_rule() );
                    }
                }
            },
            _ => {
                panic!("what rule then ? : {:?}", option_decl_pair.as_rule() );
            }
        }
    }
    // ***
    let hoptions = HibouExploreOptions{loggers,strategy,filters,priorities};
    return Ok(hoptions);
}



fn parse_filters(filters_decl_pair : Pair<Rule>) -> Result<Vec<ExplorationFilter>,HibouParsingError> {
    let mut filters : Vec<ExplorationFilter> = Vec::new();
    for filter_pair in filters_decl_pair.into_inner() {
        match filter_pair.as_rule() {
            Rule::OPTION_FILTER_MAX_DEPTH => {
                let content = filter_pair.into_inner().next().unwrap();
                let content_str : String = content.as_str().chars().filter(|c| !c.is_whitespace()).collect();
                let my_val : u32 = content_str.parse::<u32>().unwrap();
                filters.push(ExplorationFilter::MaxProcessDepth(my_val));
            },
            Rule::OPTION_FILTER_MAX_LOOP_DEPTH  => {
                let content = filter_pair.into_inner().next().unwrap();
                let content_str : String = content.as_str().chars().filter(|c| !c.is_whitespace()).collect();
                let my_val : u32 = content_str.parse::<u32>().unwrap();
                filters.push(ExplorationFilter::MaxLoopInstanciation(my_val));
            },
            Rule::OPTION_FILTER_MAX_NODE_NUMBER  => {
                let content = filter_pair.into_inner().next().unwrap();
                let content_str : String = content.as_str().chars().filter(|c| !c.is_whitespace()).collect();
                let my_val : u32 = content_str.parse::<u32>().unwrap();
                filters.push(ExplorationFilter::MaxNodeNumber(my_val));
            },
            _ => {
                panic!("what rule then ? : {:?}", filter_pair.as_rule() );
            }
        }
    }
    return Ok(filters);
}




fn parse_specific_priorities(priorities_decl_pair : Pair<Rule>) -> Result<ExplorationPriorities,HibouParsingError> {
    let mut emission : i32 = 0;
    let mut reception : i32 = 0;
    let mut multi_rdv : i32 = 0;
    let mut in_loop : i32 = 0;
    // ***
    for priority_pair in priorities_decl_pair.into_inner() {
        let mut priority_contents = priority_pair.into_inner();
        let priority_kind_pair = priority_contents.next().unwrap();
        // ***
        let priority_level_pair = priority_contents.next().unwrap();
        let priority_level_str : String = priority_level_pair.as_str().chars().filter(|c| !c.is_whitespace()).collect();
        let priority_level : i32 = priority_level_str.parse::<i32>().unwrap();
        // ***
        match priority_kind_pair.as_rule() {
            Rule::OPTION_PRIORITY_emission => {
                emission = priority_level;
            },
            Rule::OPTION_PRIORITY_reception => {
                reception = priority_level;
            },
            Rule::OPTION_PRIORITY_multi_rdv => {
                multi_rdv = priority_level;
            },
            Rule::OPTION_PRIORITY_loop => {
                in_loop = priority_level;
            },
            // ***
            Rule::OPTION_PRIORITY_elim => {
                return Err(HibouParsingError::ProcessPriorityError("found elim priority in Exploration".to_string()));
            },
            Rule::OPTION_PRIORITY_simu => {
                return Err(HibouParsingError::ProcessPriorityError("found simu priority in Exploration".to_string()));
            },
            // ***
            Rule::OPTION_PRIORITY_simpl => {
                return Err(HibouParsingError::ProcessPriorityError("found simpl priority in Exploration".to_string()));
            },
            Rule::OPTION_PRIORITY_flush => {
                return Err(HibouParsingError::ProcessPriorityError("found flush priority in Exploration".to_string()));
            },
            Rule::OPTION_PRIORITY_invert => {
                return Err(HibouParsingError::ProcessPriorityError("found invert priority in Exploration".to_string()));
            },
            Rule::OPTION_PRIORITY_deduplicate => {
                return Err(HibouParsingError::ProcessPriorityError("found deduplicate priority in Exploration".to_string()));
            },
            Rule::OPTION_PRIORITY_factorize => {
                return Err(HibouParsingError::ProcessPriorityError("found factorize priority in Exploration".to_string()));
            },
            Rule::OPTION_PRIORITY_defactorize => {
                return Err(HibouParsingError::ProcessPriorityError("found defactorize priority in Exploration".to_string()));
            },
            // ***
            _ => {
                panic!("what rule then ? : {:?}", priority_kind_pair.as_rule() );
            }
        }
    }
    // ***
    let priorities = ExplorationPriorities::new(emission,reception,multi_rdv,in_loop);
    return Ok(priorities);
}