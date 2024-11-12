use stremio_core::models::catalog_with_filters::CatalogWithFilters;
use stremio_core::models::ctx::Ctx;
use stremio_core::models::installed_addons_with_filters::InstalledAddonsWithFilters;
use stremio_core::runtime::msg::{Action, ActionLoad, Msg};
use stremio_core::runtime::{Effects, Env, UpdateWithCtx};
use stremio_core::types::addon::Descriptor;
use stremio_core::types::profile::Profile;

#[derive(Default, Clone)]
pub struct AddonsWithFilters {
    pub remote_addons: CatalogWithFilters<Descriptor>,
    pub installed_addons: InstalledAddonsWithFilters,
}

impl AddonsWithFilters {
    pub fn new(profile: &Profile) -> (Self, Effects) {
        let (remote_addons, remote_addons_effects) = CatalogWithFilters::<Descriptor>::new(profile);
        let (installed_addons, installed_addons_effects) = InstalledAddonsWithFilters::new(profile);
        let effects = remote_addons_effects.join(installed_addons_effects);
        (
            Self {
                remote_addons,
                installed_addons,
            },
            effects,
        )
    }
}

impl<E: Env + 'static> UpdateWithCtx<E> for AddonsWithFilters {
    fn update(&mut self, msg: &Msg, ctx: &Ctx) -> Effects {
        match msg {
            Msg::Action(Action::Load(ActionLoad::InstalledAddonsWithFilters(_selected))) => {
                let unload_remote_effects = UpdateWithCtx::<E>::update(
                    &mut self.remote_addons,
                    &Msg::Action(Action::Unload),
                    ctx,
                );
                let installed_addons_effects =
                    UpdateWithCtx::<E>::update(&mut self.installed_addons, msg, ctx);
                unload_remote_effects.join(installed_addons_effects)
            }
            Msg::Action(Action::Load(ActionLoad::CatalogWithFilters(_selected))) => {
                let unload_installed_effects = UpdateWithCtx::<E>::update(
                    &mut self.installed_addons,
                    &Msg::Action(Action::Unload),
                    ctx,
                );
                let remote_effects = UpdateWithCtx::<E>::update(&mut self.remote_addons, msg, ctx);
                unload_installed_effects.join(remote_effects)
            }
            _ => {
                let remote_effects = UpdateWithCtx::<E>::update(&mut self.remote_addons, msg, ctx);
                let installed_effects =
                    UpdateWithCtx::<E>::update(&mut self.installed_addons, msg, ctx);
                remote_effects.join(installed_effects)
            }
        }
    }
}
