// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_export::*;

pub struct ExporterRegistry;

/// This is a registry for exporters.
impl ExporterRegistry {
    /// Create a new exporter based on the settings.
    /// This will return an error if the exporter is not found.
    ///
    /// # Arguments
    /// settings - The settings to use for the exporter.
    pub fn create(
        &self,
        settings: &ExportSettings,
    ) -> microcad_core::CoreResult<Box<dyn Exporter>> {
        if let Some(id) = settings.exporter_id()? {
            use microcad_export::*;
            match id.as_str() {
                "svg" => Self::make::<svg::SvgExporter>(settings),
                "stl" => Self::make::<stl::StlExporter>(settings),
                "ply" => Self::make::<ply::PlyExporter>(settings),
                "tree.dump" => Self::make::<tree_dump::TreeDumpExporter>(settings),
                id => Err(microcad_core::CoreError::NoSuitableExporterFound(
                    id.to_string(),
                )),
            }
        } else {
            Err(microcad_core::CoreError::NoFilenameSpecifiedForExport)
        }
    }

    /// Create a new exporter based on the type.
    fn make<T: Exporter + 'static>(
        settings: &ExportSettings,
    ) -> microcad_core::CoreResult<Box<dyn Exporter>> {
        Ok(Box::new(T::from_settings(settings)?))
    }
}

lazy_static::lazy_static! {
    pub static ref EXPORTERS: ExporterRegistry = ExporterRegistry;
}

/// Shortcut to export a node
pub fn export(
    node: microcad_lang::objecttree::ObjectNode,
) -> microcad_core::CoreResult<Vec<std::path::PathBuf>> {
    export_tree(node, |settings| EXPORTERS.create(settings))
}
