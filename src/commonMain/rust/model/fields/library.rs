use std::cmp;
use std::convert::TryFrom;
use std::num::NonZeroUsize;

use stremio_core::models::library_with_filters::{
    LibraryRequest, LibraryRequestPage, LibraryWithFilters, Selectable, SelectablePage,
    SelectableSort, SelectableType, Selected, Sort,
};

use crate::bridge::{FromProtobuf, ToProtobuf};
use crate::protobuf::stremio::core::models;

impl FromProtobuf<Sort> for models::library_with_filters::Sort {
    fn from_protobuf(&self) -> Sort {
        match self {
            models::library_with_filters::Sort::LastWatched => Sort::LastWatched,
            models::library_with_filters::Sort::Name => Sort::Name,
            models::library_with_filters::Sort::TimesWatched => Sort::TimesWatched,
        }
    }
}

impl FromProtobuf<LibraryRequest> for models::library_with_filters::LibraryRequest {
    fn from_protobuf(&self) -> LibraryRequest {
        let page = usize::try_from(cmp::max(self.page, 1)).unwrap_or(usize::MAX);
        let page = LibraryRequestPage(NonZeroUsize::new(page).unwrap());
        LibraryRequest {
            r#type: self.r#type.to_owned(),
            sort: models::library_with_filters::Sort::try_from(self.sort)
                .ok()
                .from_protobuf()
                .unwrap_or(Sort::LastWatched),
            page,
        }
    }
}

impl FromProtobuf<Selected> for models::library_with_filters::Selected {
    fn from_protobuf(&self) -> Selected {
        Selected {
            request: self.request.from_protobuf(),
        }
    }
}

impl ToProtobuf<models::library_with_filters::Sort, ()> for Sort {
    fn to_protobuf(&self, _args: &()) -> models::library_with_filters::Sort {
        match self {
            Sort::LastWatched => models::library_with_filters::Sort::LastWatched,
            Sort::Name => models::library_with_filters::Sort::Name,
            Sort::TimesWatched => models::library_with_filters::Sort::TimesWatched,
        }
    }
}

impl ToProtobuf<models::library_with_filters::LibraryRequest, ()> for LibraryRequest {
    fn to_protobuf(&self, _args: &()) -> models::library_with_filters::LibraryRequest {
        models::library_with_filters::LibraryRequest {
            r#type: self.r#type.clone(),
            sort: self.sort.to_protobuf(&()) as i32,
            page: i64::try_from(self.page.0.get()).unwrap_or(i64::MAX),
        }
    }
}

impl ToProtobuf<models::library_with_filters::Selected, ()> for Selected {
    fn to_protobuf(&self, _args: &()) -> models::library_with_filters::Selected {
        models::library_with_filters::Selected {
            request: self.request.to_protobuf(&()),
        }
    }
}

impl ToProtobuf<models::library_with_filters::SelectableType, ()> for SelectableType {
    fn to_protobuf(&self, _args: &()) -> models::library_with_filters::SelectableType {
        models::library_with_filters::SelectableType {
            r#type: self.r#type.clone(),
            selected: self.selected,
            request: self.request.to_protobuf(&()),
        }
    }
}

impl ToProtobuf<models::library_with_filters::SelectableSort, ()> for SelectableSort {
    fn to_protobuf(&self, _args: &()) -> models::library_with_filters::SelectableSort {
        models::library_with_filters::SelectableSort {
            sort: self.sort.to_protobuf(&()) as i32,
            selected: self.selected,
            request: self.request.to_protobuf(&()),
        }
    }
}

impl ToProtobuf<models::library_with_filters::SelectablePage, ()> for SelectablePage {
    fn to_protobuf(&self, _args: &()) -> models::library_with_filters::SelectablePage {
        models::library_with_filters::SelectablePage {
            request: self.request.to_protobuf(&()),
        }
    }
}

impl ToProtobuf<models::library_with_filters::Selectable, ()> for Selectable {
    fn to_protobuf(&self, _args: &()) -> models::library_with_filters::Selectable {
        models::library_with_filters::Selectable {
            types: self.types.to_protobuf(&()),
            sorts: self.sorts.to_protobuf(&()),
            prev_page: self.prev_page.to_protobuf(&()),
            next_page: self.next_page.to_protobuf(&()),
        }
    }
}

impl<F> ToProtobuf<models::LibraryWithFilters, ()> for LibraryWithFilters<F> {
    fn to_protobuf(&self, _args: &()) -> models::LibraryWithFilters {
        models::LibraryWithFilters {
            selected: self.selected.to_protobuf(&()),
            selectable: self.selectable.to_protobuf(&()),
            catalog: self.catalog.to_protobuf(todo!("Args")),
        }
    }
}
