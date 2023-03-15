use stremio_core::models::library_by_type::{
    Catalog, LibraryByType, Selectable, SelectableSort, Selected,
};
use stremio_core::models::library_with_filters::Sort;

use crate::bridge::{FromProtobuf, ToProtobuf};
use crate::protobuf::stremio::core::models;

impl FromProtobuf<Selected> for models::library_by_type::Selected {
    fn from_protobuf(&self) -> Selected {
        Selected {
            sort: models::library_with_filters::Sort::from_i32(self.sort)
                .from_protobuf()
                .unwrap_or(Sort::LastWatched),
        }
    }
}

impl ToProtobuf<models::library_by_type::Selected, ()> for Selected {
    fn to_protobuf(&self, _args: &()) -> models::library_by_type::Selected {
        models::library_by_type::Selected {
            sort: self.sort.to_protobuf(&()) as i32,
        }
    }
}

impl ToProtobuf<models::library_by_type::SelectableSort, ()> for SelectableSort {
    fn to_protobuf(&self, _args: &()) -> models::library_by_type::SelectableSort {
        models::library_by_type::SelectableSort {
            sort: self.sort.to_protobuf(&()) as i32,
            selected: self.selected,
        }
    }
}

impl ToProtobuf<models::library_by_type::Selectable, ()> for Selectable {
    fn to_protobuf(&self, _args: &()) -> models::library_by_type::Selectable {
        models::library_by_type::Selectable {
            sorts: self.sorts.to_protobuf(&()),
        }
    }
}

impl ToProtobuf<models::LibraryCatalog, ()> for Catalog {
    fn to_protobuf(&self, _args: &()) -> models::LibraryCatalog {
        let items = self
            .iter()
            .flatten()
            .map(|item| item.to_protobuf(&()))
            .collect::<Vec<_>>();
        let r#type = items.first().map(|item| item.r#type.to_owned());
        models::LibraryCatalog { r#type, items }
    }
}

impl<F> ToProtobuf<models::LibraryByType, ()> for LibraryByType<F> {
    fn to_protobuf(&self, _args: &()) -> models::LibraryByType {
        models::LibraryByType {
            selected: self.selected.to_protobuf(&()),
            selectable: self.selectable.to_protobuf(&()),
            catalogs: self.catalogs.to_protobuf(&()),
        }
    }
}
