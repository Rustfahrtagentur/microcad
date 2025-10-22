// The module 'vscode' contains the VS Code extensibility API
// Import the module and reference it with the alias vscode in your code below
import * as vscode from 'vscode';
import {
	LanguageClient,
	LanguageClientOptions,
	ServerOptions,
	StreamInfo,
} from 'vscode-languageclient/node';

import * as net from 'net';

import type { URI as LspURI } from "vscode-languageserver-types";

let client: LanguageClient;



export function showPreview(url: LspURI, component: string): Thenable<unknown> {
	return vscode.commands.executeCommand("microcad/showPreview", url, component);
}

// This method is called when your extension is activated
// Your extension is activated the very first time the command is executed
export function activate(context: vscode.ExtensionContext) {

	//lsp = spawn("microcad-lsp", { stdio: "inherit" });

	//lsp.on("error", (err) => { vscode.window.showErrorMessage(`Failed to start microcad lsp: ${err.message}`); });

	// Use the console to output diagnostic information (console.log) and errors (console.error)
	// This line of code will only be executed once when your extension is activated
	console.log('Congratulations, your extension "microcad" is now active!');

	// The command has been defined in the package.json file
	// Now provide the implementation of the command with registerCommand
	// The commandId parameter must match the command field in package.json
	const disposable = vscode.commands.registerCommand('microcad.helloWorld', () => {
		// The code you place here will be executed every time your command is executed
		// Display a message box to the user
		vscode.window.showInformationMessage('Hello World from microcad-lsp!');
	});
	context.subscriptions.push(
		vscode.commands.registerCommand("microcad.showPreview", async function () {
			const ae = vscode.window.activeTextEditor;
			if (!ae) {
				return;
			}

			await showPreview(ae.document.uri.toString(), "");
		}),
	);
	context.subscriptions.push(disposable);

	const serverOptions: ServerOptions = () => {
		return new Promise<StreamInfo>((resolve, reject) => {
			const port = 5007;
			const socket = net.connect(port, '127.0.0.1', () => {
				console.log(`Connected to language server on port ${port}`);
				resolve({
					reader: socket,
					writer: socket,
				});
			});

			socket.on('error', (err) => {
				console.error('Failed to connect to language server:', err);
				reject(err);
			});
		});
	};

	// Options to control the language client
	const clientOptions: LanguageClientOptions = {
		// Register the server for plain text documents
		documentSelector: [{ scheme: 'file', language: 'microcad' }],
		synchronize: {
			// Notify the server about file changes to '.clientrc files contained in the workspace
			fileEvents: vscode.workspace.createFileSystemWatcher('**/.µcad')
		}
	};

	const path = "/home/micha/Work/mcad/mcad/target/debug/";

	// Create the language client and start the client.
	client = new LanguageClient(
		'microcad-lsp',
		'Microcad LSP',
		serverOptions,
		clientOptions
	);

	// Start the client. This will also launch the server
	client.start();
}

export function deactivate(): Thenable<void> | undefined {
	if (!client) {
		return undefined;
	}
	return client.stop();
}

