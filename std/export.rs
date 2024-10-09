// Copyright © 2024 The µCAD authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_core::{ExportSettings, Exporter};

pub struct ExporterRegistry;

/// This is a registry for exporters.
impl ExporterRegistry {
    /// Create a new exporter based on the settings.
    /// This will return an error if the exporter is not found.
    ///
    /// # Arguments
    /// settings - The settings to use for the exporter.
    pub fn create(&self, settings: &ExportSettings) -> microcad_core::Result<Box<dyn Exporter>> {
        let id = settings.exporter_id();
        if id.as_ref().is_none() {
            return Err(microcad_core::CoreError::NoFilenameSpecifiedForExport);
        }

        use microcad_export::*;
        match id.unwrap().as_str() {
            "svg" => Self::make::<svg::SvgExporter>(settings),
            "stl" => Self::make::<stl::StlExporter>(settings),
            "ply" => Self::make::<ply::PlyExporter>(settings),
            "tree.dump" => Self::make::<tree_dump::TreeDumpExporter>(settings),
            id => Err(microcad_core::CoreError::NoSuitableExporterFound(
                id.to_string(),
            )),
        }
    }

    /// Create a new exporter based on the type.
    fn make<T: Exporter + 'static>(
        settings: &ExportSettings,
    ) -> microcad_core::Result<Box<dyn Exporter>> {
        Ok(Box::new(T::from_settings(settings)?))
    }
}

lazy_static::lazy_static! {
    pub static ref EXPORTERS: ExporterRegistry = ExporterRegistry;
}

/// Shortcut to export a node
pub fn export(node: microcad_render::Node) -> microcad_core::Result<()> {
    microcad_core::export::export_tree(node.clone(), |settings| EXPORTERS.create(settings))
}
