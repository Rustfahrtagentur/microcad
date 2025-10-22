// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad language server processor.
//!
//! The processor sets up the µcad language toolchain (parse, resolve, eval).
//! It runs in a separate thread and communication is handled via
//! crossbeam channels with requests and responses.

use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    rc::Rc,
};

use crossbeam::channel::{Receiver, Sender};
use microcad_lang::{
    eval::EvalContext, model::Model, rc::RcMut, render::*, resolve::ResolveContext,
    syntax::SourceFile,
};

/// A processor request.
///
/// Commands that can be passed to the [`Processor`].
#[derive(Clone)]
pub enum ProcessorRequest {
    SetCursorPosition { file: PathBuf, line: u32, col: u32 },
    AddDocument(PathBuf),
    RemoveDocument(PathBuf),
    UpdateDocument(PathBuf),
    RunViewerForDocument(PathBuf),
}

/// An interpreter output.
///
/// Contains the geometry to rendered.
pub enum ProcessorResponse {}

struct Document {
    source_file: Rc<SourceFile>,
    eval_context: EvalContext,
}

impl Document {
    fn new(source_file: Rc<SourceFile>, search_paths: &Vec<PathBuf>) -> anyhow::Result<Self> {
        // resolve the file
        let resolve_context = ResolveContext::create(
            source_file.clone(),
            search_paths,
            Some(microcad_builtin::builtin_module()),
            microcad_lang::diag::DiagHandler::default(),
        )?;

        let eval_context = EvalContext::new(
            resolve_context,
            microcad_lang::eval::Stdout::new(),
            microcad_builtin::builtin_exporters(),
            microcad_builtin::builtin_importers(),
        );

        Ok(Self {
            source_file,
            eval_context,
        })
    }

    fn eval(&mut self) -> anyhow::Result<()> {
        let _ = self
            .eval_context
            .eval()
            .map_err(|err| anyhow::anyhow!("Eval error: {err}"));
        Ok(())
    }
}

pub struct WorkspaceSettings {
    pub search_paths: Vec<PathBuf>,
}

/// The processor  responsable for generating view commands.
///
/// The processor itself runs in a seperate thread and can be controlled
/// via [`ProcessorInterface`] by sending requests and handling the corresponing responses.
pub struct Processor {
    workspace_settings: WorkspaceSettings,
    documents: HashMap<PathBuf, Document>,

    pub request_handler: Receiver<ProcessorRequest>,

    /// Outputs
    pub response_sender: Sender<ProcessorResponse>,
    // pub cursor_position: SourceLocation,
}

pub type ProcessorResult = anyhow::Result<Vec<ProcessorResponse>>;

impl Processor {
    /// Handle processor request.
    pub fn handle_request(&mut self, request: ProcessorRequest) -> ProcessorResult {
        match request {
            ProcessorRequest::SetCursorPosition { file, line, col } => todo!(),
            ProcessorRequest::AddDocument(path) => self.add_document(&path),
            ProcessorRequest::RemoveDocument(path) => self.remove_document(&path),
            ProcessorRequest::UpdateDocument(path) => self.update_document(&path),
            ProcessorRequest::RunViewerForDocument(_) => todo!(),
        }
    }

    /// Process a µcad file (parse, resolve, eval).
    pub fn add_document(&mut self, path: &Path) -> ProcessorResult {
        match self.documents.get(path) {
            Some(_) => {
                log::info!("Document {} already exists.", path.display());
            }
            None => {
                let source_file = SourceFile::load(path)?;
                self.documents.insert(
                    PathBuf::from(path),
                    Document::new(source_file, &self.workspace_settings.search_paths)?,
                );
            }
        }

        self.update_document(path)?;

        Ok(vec![])
    }

    /// Update (re-evaluate) a document.
    pub fn update_document(&mut self, path: &Path) -> anyhow::Result<Vec<ProcessorResponse>> {
        match self.documents.get_mut(path) {
            Some(document) => document.eval()?,
            None => {
                log::warn!("Document {} does not exist!", path.display());
            }
        }

        Ok(vec![])
    }

    /// Remove a document.
    pub fn remove_document(&mut self, path: &Path) -> ProcessorResult {
        self.documents.remove(path);
        Ok(vec![])
    }
}

#[derive(Debug)]
pub struct ProcessorInterface {
    pub request_sender: Sender<ProcessorRequest>,
    pub response_receiver: Receiver<ProcessorResponse>,
}

impl ProcessorInterface {
    /// Send request.
    pub fn send_request(&self, request: ProcessorRequest) -> anyhow::Result<()> {
        Ok(self.request_sender.send(request)?)
    }

    /// Run the processing thread and create interface.
    pub fn run(workspace_settings: WorkspaceSettings) -> Self {
        let (request_sender, request_receiver) = crossbeam::channel::unbounded();
        let (response_sender, response_receiver) = crossbeam::channel::unbounded();

        std::thread::spawn(move || {
            let mut processor = Processor {
                workspace_settings,
                documents: HashMap::default(),
                request_handler: request_receiver,
                response_sender,
            };

            loop {
                if let Ok(request) = processor.request_handler.recv()
                    && let Ok(responses) = processor.handle_request(request)
                {
                    for response in responses {
                        processor.response_sender.send(response).expect("No error");
                    }
                }
            }
        });

        Self {
            request_sender,
            response_receiver,
        }
    }
}
