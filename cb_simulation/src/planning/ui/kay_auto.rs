//! This is all auto-generated. Do not touch.
#![cfg_attr(rustfmt, rustfmt_skip)]
#[allow(unused_imports)]
use kay::{ActorSystem, TypedID, RawID, Fate, Actor, TraitIDFrom, ActorOrActorTrait};
#[allow(unused_imports)]
use super::*;

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)] #[serde(transparent)]
pub struct PlanningUIID {
    _raw_id: RawID
}

pub struct PlanningUIRepresentative;

impl ActorOrActorTrait for PlanningUIRepresentative {
    type ID = PlanningUIID;
}

impl TypedID for PlanningUIID {
    type Target = PlanningUIRepresentative;

    fn from_raw(id: RawID) -> Self {
        PlanningUIID { _raw_id: id }
    }

    fn as_raw(&self) -> RawID {
        self._raw_id
    }
}

impl<A: Actor + PlanningUI> TraitIDFrom<A> for PlanningUIID {}

impl PlanningUIID {
    pub fn on_plans_update(&self, master_update: PlanHistoryUpdate, proposal_updates: CHashMap < ProposalID , ProposalUpdate >, world: &mut World) {
        world.send(self.as_raw(), MSG_PlanningUI_on_plans_update(master_update, proposal_updates));
    }
    
    pub fn on_proposal_preview_update(&self, proposal_id: ProposalID, effective_history: PlanHistory, result_update: PlanResultUpdate, new_actions: ActionGroups, world: &mut World) {
        world.send(self.as_raw(), MSG_PlanningUI_on_proposal_preview_update(proposal_id, effective_history, result_update, new_actions));
    }

    pub fn register_trait(system: &mut ActorSystem) {
        system.register_trait::<PlanningUIRepresentative>();
        system.register_trait_message::<MSG_PlanningUI_on_plans_update>();
        system.register_trait_message::<MSG_PlanningUI_on_proposal_preview_update>();
    }

    pub fn register_implementor<A: Actor + PlanningUI>(system: &mut ActorSystem) {
        system.register_implementor::<A, PlanningUIRepresentative>();
        system.add_handler::<A, _, _>(
            |&MSG_PlanningUI_on_plans_update(ref master_update, ref proposal_updates), instance, world| {
                instance.on_plans_update(master_update, proposal_updates, world); Fate::Live
            }, false
        );
        
        system.add_handler::<A, _, _>(
            |&MSG_PlanningUI_on_proposal_preview_update(proposal_id, ref effective_history, ref result_update, ref new_actions), instance, world| {
                instance.on_proposal_preview_update(proposal_id, effective_history, result_update, new_actions, world); Fate::Live
            }, false
        );
    }
}

#[derive(Compact, Clone)] #[allow(non_camel_case_types)]
struct MSG_PlanningUI_on_plans_update(pub PlanHistoryUpdate, pub CHashMap < ProposalID , ProposalUpdate >);
#[derive(Compact, Clone)] #[allow(non_camel_case_types)]
struct MSG_PlanningUI_on_proposal_preview_update(pub ProposalID, pub PlanHistory, pub PlanResultUpdate, pub ActionGroups);



#[allow(unused_variables)]
#[allow(unused_mut)]
pub fn auto_setup(system: &mut ActorSystem) {
    PlanningUIID::register_trait(system);
    
}